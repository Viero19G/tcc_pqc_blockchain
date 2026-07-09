//! ML-DSA (FIPS 204) — Module-Lattice-Based Digital Signature Standard.

use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use crate::constants::{
	ML_DSA65_PUBLIC_KEY_BYTES, ML_DSA65_SECRET_KEY_BYTES, ML_DSA65_SIGNATURE_BYTES,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo)]
pub struct MlDsaPublicKey(pub [u8; ML_DSA65_PUBLIC_KEY_BYTES]);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo)]
pub struct MlDsaSecretKey(pub [u8; ML_DSA65_SECRET_KEY_BYTES]);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo)]
pub struct MlDsaSignature(pub [u8; ML_DSA65_SIGNATURE_BYTES]);

#[cfg(feature = "std")]
pub struct MlDsaKeypair {
	pub public: MlDsaPublicKey,
	pub secret: MlDsaSecretKey,
}

#[cfg(feature = "std")]
impl MlDsaKeypair {
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

	pub fn sign(&self, message: &[u8]) -> MlDsaSignature {
		use ml_dsa::{MlDsa65, Seed, SignatureEncoding, Signer, SigningKey};

		let seed = Seed::try_from(self.secret.0.as_slice()).expect("seed tem 32 bytes");
		let signing_key = SigningKey::<MlDsa65>::from_seed(&seed);

		let sig = signing_key.sign(message);
		let mut out = [0u8; ML_DSA65_SIGNATURE_BYTES];
		out.copy_from_slice(sig.to_bytes().as_ref());
		MlDsaSignature(out)
	}
}

pub fn verify(message: &[u8], signature: &MlDsaSignature, public_key: &MlDsaPublicKey) -> bool {
	use ml_dsa::{EncodedSignature, EncodedVerifyingKey, MlDsa65, Signature, Verifier, VerifyingKey};

	let Ok(enc_vk) = EncodedVerifyingKey::<MlDsa65>::try_from(public_key.0.as_slice()) else {
		return false;
	};
	let verifying_key = VerifyingKey::<MlDsa65>::decode(&enc_vk);

	let Ok(enc_sig) = EncodedSignature::<MlDsa65>::try_from(signature.0.as_slice()) else {
		return false;
	};
	let Some(sig) = Signature::<MlDsa65>::decode(&enc_sig) else {
		return false;
	};

	verifying_key.verify(message, &sig).is_ok()
}

impl MlDsaPublicKey {
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