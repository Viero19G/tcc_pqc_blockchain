//! ML-DSA (FIPS 204) — Module-Lattice-Based Digital Signature Standard.

use alloc::vec::Vec;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use crate::constants::{
	ML_DSA65_PUBLIC_KEY_BYTES, ML_DSA65_SECRET_KEY_BYTES, ML_DSA65_SIGNATURE_BYTES,
};

/// Chave pública ML-DSA-65 serializada.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct MlDsaPublicKey(pub [u8; ML_DSA65_PUBLIC_KEY_BYTES]);

/// Seed da chave de assinatura ML-DSA-65 (somente off-chain / genesis).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct MlDsaSecretKey(pub [u8; ML_DSA65_SECRET_KEY_BYTES]);

/// Assinatura ML-DSA-65.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub struct MlDsaSignature(pub [u8; ML_DSA65_SIGNATURE_BYTES]);

/// Par de chaves ML-DSA (geração requer RNG — apenas com `std`).
#[cfg(feature = "std")]
pub struct MlDsaKeypair {
	pub public: MlDsaPublicKey,
	pub secret: MlDsaSecretKey,
}

#[cfg(feature = "std")]
impl MlDsaKeypair {
	/// Gera um par de chaves ML-DSA-65.
	pub fn generate() -> Self {
		use ml_dsa::{Generate, KeyExport, Keypair, MlDsa65, SigningKey};

		let signing_key = SigningKey::<MlDsa65>::generate();
		let verifying_key = signing_key.verifying_key();

		let mut pk = [0u8; ML_DSA65_PUBLIC_KEY_BYTES];
		let mut sk = [0u8; ML_DSA65_SECRET_KEY_BYTES];
		pk.copy_from_slice(verifying_key.to_bytes().as_ref());
		sk.copy_from_slice(signing_key.to_bytes().as_ref());

		Self { public: MlDsaPublicKey(pk), secret: MlDsaSecretKey(sk) }
	}

	/// Assina uma mensagem (raw bytes).
	pub fn sign(&self, message: &[u8]) -> MlDsaSignature {
		use ml_dsa::{KeyInit, MlDsa65, Signer, SigningKey};

		let signing_key =
			SigningKey::<MlDsa65>::from_bytes(&self.secret.0).expect("valid secret key seed");
		let sig = signing_key.sign(message);
		let mut out = [0u8; ML_DSA65_SIGNATURE_BYTES];
		out.copy_from_slice(sig.to_bytes().as_ref());
		MlDsaSignature(out)
	}
}

/// Verifica assinatura ML-DSA-65 sobre mensagem raw.
pub fn verify(message: &[u8], signature: &MlDsaSignature, public_key: &MlDsaPublicKey) -> bool {
	use ml_dsa::{KeyInit, MlDsa65, Signature, Verifier, VerifyingKey};

	let Ok(verifying_key) = VerifyingKey::<MlDsa65>::from_bytes(&public_key.0) else {
		return false;
	};
	let Ok(sig) = Signature::<MlDsa65>::from_bytes(&signature.0) else {
		return false;
	};
	verifying_key.verify(message, &sig).is_ok()
}

impl MlDsaPublicKey {
	/// Deriva AccountId32 via Blake2-256 (padrão Substrate).
	pub fn to_account_id(&self) -> sp_core::crypto::AccountId32 {
		use sp_core::crypto::AccountId32;
		use sp_runtime::traits::Hash;
		let hash = sp_runtime::traits::BlakeTwo256::hash(self.0.as_ref());
		AccountId32::from(hash.0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ml_dsa_sign_verify_roundtrip() {
		let kp = MlDsaKeypair::generate();
		let msg = b"entangle ml-dsa test";
		let sig = kp.sign(msg);
		assert!(verify(msg, &sig, &kp.public));
		assert!(!verify(b"wrong", &sig, &kp.public));
	}
}
