//! Assinatura híbrida: Sr25519 (clássico) + ML-DSA-65 (pós-quântico).
//!
//! Permite transição gradual: contas existentes continuam com Sr25519 enquanto
//! novas contas podem registrar chaves ML-DSA via `pallet-pqc`.

use alloc::vec::Vec;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{crypto::AccountId32, sr25519};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	MultiSignature, MultiSigner,
};

use crate::{
	constants::ML_DSA65_SIGNATURE_BYTES,
	mldsa::{self, MlDsaPublicKey, MlDsaSignature},
};

/// Esquema de assinatura suportado (crypto-agility).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum SignatureScheme {
	/// Ed25519 / Sr25519 via MultiSignature (legado Substrate).
	Classic,
	/// ML-DSA-65 (FIPS 204).
	MlDsa65,
}

/// Assinatura híbrida usada como `Signature` do runtime.
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum HybridSignature {
	/// Assinatura clássica Substrate (Sr25519, Ed25519 ou ECDSA).
	Classic(MultiSignature),
	/// Assinatura ML-DSA-65 (~3.3 KB).
	MlDsa65(MlDsaSignature),
}

/// Chave pública híbrida correspondente.
#[derive(Clone, PartialEq, Eq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum HybridPublic {
	Classic(MultiSigner),
	MlDsa65(MlDsaPublicKey),
}

impl HybridSignature {
	pub fn scheme(&self) -> SignatureScheme {
		match self {
			Self::Classic(_) => SignatureScheme::Classic,
			Self::MlDsa65(_) => SignatureScheme::MlDsa65,
		}
	}
}

impl HybridPublic {
	pub fn scheme(&self) -> SignatureScheme {
		match self {
			Self::Classic(_) => SignatureScheme::Classic,
			Self::MlDsa65(_) => SignatureScheme::MlDsa65,
		}
	}
}

impl Verify for HybridSignature {
	type Signer = HybridPublic;

	fn verify<L: sp_runtime::traits::Lazy<[u8]>>(
		&self,
		msg: L,
		signer: &Self::Signer,
	) -> bool {
		match (self, signer) {
			(Self::Classic(sig), HybridPublic::Classic(signer)) => sig.verify(msg, signer),
			(Self::MlDsa65(sig), HybridPublic::MlDsa65(pk)) => {
				mldsa::verify(msg.get(), sig, pk)
			},
			_ => false,
		}
	}
}

impl IdentifyAccount for HybridPublic {
	type AccountId = AccountId32;

	fn into_account(self) -> AccountId32 {
		match self {
			Self::Classic(signer) => signer.into_account(),
			Self::MlDsa65(pk) => pk.to_account_id(),
		}
	}
}

impl From<sr25519::Pair> for HybridPublic {
	fn from(pair: sr25519::Pair) -> Self {
		Self::Classic(MultiSigner::from(pair.public()))
	}
}

impl From<sr25519::Public> for HybridPublic {
	fn from(public: sr25519::Public) -> Self {
		Self::Classic(MultiSigner::from(public))
	}
}

impl From<MlDsaPublicKey> for HybridPublic {
	fn from(pk: MlDsaPublicKey) -> Self {
		Self::MlDsa65(pk)
	}
}

/// Estimativa de tamanho máximo de extrinsic com assinatura PQ.
pub const MAX_PQ_EXTRINSIC_SIGNATURE_BYTES: usize = ML_DSA65_SIGNATURE_BYTES + 256;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mldsa::MlDsaKeypair;

	#[test]
	fn hybrid_ml_dsa_verify() {
		let kp = MlDsaKeypair::generate();
		let msg = b"hybrid signature test";
		let sig = HybridSignature::MlDsa65(kp.sign(msg));
		let pk = HybridPublic::MlDsa65(kp.public);
		assert!(sig.verify(msg, &pk));
	}

	#[test]
	fn scheme_mismatch_fails() {
		let kp = MlDsaKeypair::generate();
		let msg = b"test";
		let sig = HybridSignature::MlDsa65(kp.sign(msg));
		let wrong_pk = HybridPublic::Classic(MultiSigner::from(sr25519::Pair::generate().public()));
		assert!(!sig.verify(msg, &wrong_pk));
	}
}
