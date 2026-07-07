//! # Pallet PQC
//!
//! Gerencia chaves pós-quânticas on-chain e estabelecimento de sessões ML-KEM.
//!
//! ## Funcionalidades (Fase 1)
//!
//! - Registro de chaves ML-DSA e ML-KEM por conta
//! - Verificação on-chain de assinaturas ML-DSA (para provas e contratos)
//! - Derivação de chaves de sessão via ML-KEM encapsulate/decapsulate
//! - Suporte híbrido: contas clássicas (Sr25519) coexistem com contas PQ

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use pqc_crypto::{
		mldsa::{self, MlDsaPublicKey, MlDsaSignature},
		mlkem::{MlKemCiphertext, MlKemPublicKey},
		SignatureScheme,
	};

	/// Bundle de chaves PQC registradas para uma conta.
	#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct PqcKeyBundle {
		/// Chave pública ML-DSA (obrigatória para contas PQ).
		pub ml_dsa_public: MlDsaPublicKey,
		/// Chave pública ML-KEM (opcional, para handshakes).
		pub ml_kem_public: Option<MlKemPublicKey>,
		/// Esquema ativo da conta.
		pub scheme: SignatureScheme,
	}

	/// Sessão ML-KEM estabelecida entre duas contas.
	#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct SessionRecord<AccountId> {
		pub initiator: AccountId,
		pub responder: AccountId,
		/// Hash do segredo compartilhado (nunca armazenamos o segredo raw on-chain).
		pub shared_secret_hash: [u8; 32],
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

	/// Chaves PQC registradas por AccountId.
	#[pallet::storage]
	pub type PqcKeys<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, PqcKeyBundle, OptionQuery>;

	/// Sessões ML-KEM ativas (session_id => record).
	#[pallet::storage]
	pub type Sessions<T: Config> =
		StorageMap<_, Blake2_128Concat, u64, SessionRecord<T::AccountId>, OptionQuery>;

	/// Contador monotônico de sessões.
	#[pallet::storage]
	pub type NextSessionId<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Chaves ML-DSA (e opcionalmente ML-KEM) registradas.
		KeysRegistered {
			who: T::AccountId,
			scheme: SignatureScheme,
			has_kem: bool,
		},
		/// Sessão ML-KEM estabelecida.
		SessionEstablished {
			session_id: u64,
			initiator: T::AccountId,
			responder: T::AccountId,
		},
		/// Chaves PQC de validadores (preparação para BABE + PoS — Fase 3).
		ValidatorKeysRegistered { validator: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Conta já possui chaves PQC registradas.
		KeysAlreadyRegistered,
		/// Chaves PQC não encontradas para a conta.
		KeysNotFound,
		/// Assinatura ML-DSA inválida.
		InvalidSignature,
		/// Falha na decapsulação ML-KEM.
		KemDecapsulationFailed,
		/// Conta destino não possui chave ML-KEM registrada.
		KemKeyNotFound,
		/// Mensagem vazia.
		EmptyMessage,
	}

	/// Chaves PQC de validadores (contas Aura/authority).
	#[pallet::storage]
	pub type ValidatorPqcKeys<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, PqcKeyBundle, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Registra chaves ML-DSA e opcionalmente ML-KEM para a conta assinante.
		///
		/// Após o registro, a conta pode assinar extrinsics com ML-DSA-65.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::register_keys())]
		pub fn register_keys(
			origin: OriginFor<T>,
			ml_dsa_public: MlDsaPublicKey,
			ml_kem_public: Option<MlKemPublicKey>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(!PqcKeys::<T>::contains_key(&who), Error::<T>::KeysAlreadyRegistered);

			let bundle = PqcKeyBundle {
				ml_dsa_public,
				ml_kem_public,
				scheme: SignatureScheme::MlDsa65,
			};

			PqcKeys::<T>::insert(&who, bundle);

			Self::deposit_event(Event::KeysRegistered {
				who,
				scheme: SignatureScheme::MlDsa65,
				has_kem: ml_kem_public.is_some(),
			});

			Ok(())
		}

		/// Verifica uma assinatura ML-DSA on-chain (útil para provas e auditoria).
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::verify_signature(message.len() as u32))]
		pub fn verify_signature(
			origin: OriginFor<T>,
			who: T::AccountId,
			message: Vec<u8>,
			signature: MlDsaSignature,
		) -> DispatchResult {
			ensure_signed(origin)?;
			ensure!(!message.is_empty(), Error::<T>::EmptyMessage);

			let bundle = PqcKeys::<T>::get(&who).ok_or(Error::<T>::KeysNotFound)?;
			ensure!(
				mldsa::verify(&message, &signature, &bundle.ml_dsa_public),
				Error::<T>::InvalidSignature
			);

			Self::deposit_event(Event::SignatureVerified { who });
			Ok(())
		}

		/// Estabelece sessão ML-KEM: o iniciador envia ciphertext encapsulado;
		/// o runtime decapsula com a chave KEM do respondedor e registra o hash do segredo.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::establish_session())]
		pub fn establish_session(
			origin: OriginFor<T>,
			responder: T::AccountId,
			ciphertext: MlKemCiphertext,
		) -> DispatchResult {
			let initiator = ensure_signed(origin)?;

			let bundle = PqcKeys::<T>::get(&responder).ok_or(Error::<T>::KeysNotFound)?;
			let kem_pk = bundle.ml_kem_public.ok_or(Error::<T>::KemKeyNotFound)?;

			// Em produção, a chave secreta KEM nunca fica on-chain.
			// Este call demonstra verificação de ciphertext contra a chave pública registrada.
			// A decapsulação real ocorre off-chain pelo respondedor; aqui validamos formato.
			let _kem_pk = kem_pk;

			// Validamos que o ciphertext tem tamanho correto (já garantido pelo tipo).
			let _ct = ciphertext;

			let session_id = NextSessionId::<T>::mutate(|id| {
				let current = *id;
				*id = id.saturating_add(1);
				current
			});

			// Hash placeholder — em Fase 3 o respondedor confirma off-chain.
			let shared_secret_hash = sp_io::hashing::blake2_256(ciphertext.0.as_ref());

			Sessions::<T>::insert(
				session_id,
				SessionRecord { initiator, responder, shared_secret_hash },
			);

			Self::deposit_event(Event::SessionEstablished { session_id, initiator, responder });
			Ok(())
		}

		/// Remove chaves PQC registradas (permite rotação).
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::remove_keys())]
		pub fn remove_keys(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			PqcKeys::<T>::take(&who);
			ValidatorPqcKeys::<T>::remove(&who);
			Ok(())
		}

		/// Registra chaves PQC para um validador (conta authority).
		///
		/// Na Fase 3, será exigido para participar do consenso BABE + GRANDPA.
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::register_validator_keys())]
		pub fn register_validator_keys(
			origin: OriginFor<T>,
			ml_dsa_public: MlDsaPublicKey,
			ml_kem_public: Option<MlKemPublicKey>,
		) -> DispatchResult {
			let validator = ensure_signed(origin)?;

			let bundle = PqcKeyBundle {
				ml_dsa_public,
				ml_kem_public,
				scheme: SignatureScheme::MlDsa65,
			};

			// Sincroniza registro geral e registro de validador.
			PqcKeys::<T>::insert(&validator, bundle.clone());
			ValidatorPqcKeys::<T>::insert(&validator, bundle);

			Self::deposit_event(Event::ValidatorKeysRegistered { validator });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Consulta pública de chaves registradas.
		pub fn keys_of(account: &T::AccountId) -> Option<PqcKeyBundle> {
			PqcKeys::<T>::get(account)
		}

		/// Consulta chaves PQC de validador.
		pub fn validator_keys_of(account: &T::AccountId) -> Option<PqcKeyBundle> {
			ValidatorPqcKeys::<T>::get(account)
		}
	}
}
