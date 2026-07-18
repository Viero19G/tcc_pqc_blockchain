//! ML-KEM (FIPS 203) — Module-Lattice-Based Key Encapsulation Mechanism.

use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use crate::constants::{
	ML_KEM768_CIPHERTEXT_BYTES, ML_KEM768_PUBLIC_KEY_BYTES, ML_KEM768_SECRET_KEY_BYTES,
	ML_KEM_SHARED_SECRET_BYTES,
};

/// Chave pública ML-KEM-768.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo)]
pub struct MlKemPublicKey(pub [u8; ML_KEM768_PUBLIC_KEY_BYTES]);

/// Seed da chave de decapsulação ML-KEM-768.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo)]
pub struct MlKemSecretKey(pub [u8; ML_KEM768_SECRET_KEY_BYTES]);

/// Ciphertext produzido pelo encapsulador.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo)]
pub struct MlKemCiphertext(pub [u8; ML_KEM768_CIPHERTEXT_BYTES]);

/// Segredo compartilhado derivado (32 bytes).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode, DecodeWithMemTracking, MaxEncodedLen, TypeInfo)]
pub struct MlKemSharedSecret(pub [u8; ML_KEM_SHARED_SECRET_BYTES]);

/// Par de chaves ML-KEM (geração requer RNG — apenas com `std`).
#[cfg(feature = "std")]
pub struct MlKemKeypair {
	pub public: MlKemPublicKey,
	pub secret: MlKemSecretKey,
}

#[cfg(feature = "std")]
impl MlKemKeypair {
	/// Gera par de chaves ML-KEM-768.
	pub fn generate() -> Self {
		use ml_kem::{kem::Kem, KeyExport, MlKem768};

		let (dk, ek) = MlKem768::generate_keypair();

		let mut pk = [0u8; ML_KEM768_PUBLIC_KEY_BYTES];
		let mut sk = [0u8; ML_KEM768_SECRET_KEY_BYTES];
		pk.copy_from_slice(ek.to_bytes().as_ref());
		sk.copy_from_slice(dk.to_bytes().as_ref());

		Self { public: MlKemPublicKey(pk), secret: MlKemSecretKey(sk) }
	}
}

#[cfg(feature = "std")]
pub fn encapsulate(public_key: &MlKemPublicKey) -> (MlKemCiphertext, MlKemSharedSecret) {
	use ml_kem::{array::Array, kem::Encapsulate, EncapsulationKey768, KeySizeUser};

	let key: Array<u8, <EncapsulationKey768 as KeySizeUser>::KeySize> =
		Array::try_from(public_key.0.as_slice()).expect("chave pública com tamanho correto");
	let ek = EncapsulationKey768::new(&key).expect("chave pública válida");
	let (ct, ss) = ek.encapsulate();

	let mut ct_bytes = [0u8; ML_KEM768_CIPHERTEXT_BYTES];
	let mut ss_bytes = [0u8; ML_KEM_SHARED_SECRET_BYTES];
	ct_bytes.copy_from_slice(ct.as_ref());
	ss_bytes.copy_from_slice(ss.as_ref());

	(MlKemCiphertext(ct_bytes), MlKemSharedSecret(ss_bytes))
}

pub fn decapsulate(
	secret_key: &MlKemSecretKey,
	ciphertext: &MlKemCiphertext,
) -> Option<MlKemSharedSecret> {
	use ml_kem::{kem::Decapsulate, ml_kem_768, DecapsulationKey768, Seed};

	let seed = Seed::try_from(secret_key.0.as_slice()).ok()?;
	let dk = DecapsulationKey768::from_seed(seed);
	let ct = ml_kem_768::Ciphertext::try_from(ciphertext.0.as_slice()).ok()?;
	let ss = dk.decapsulate(&ct);

	let mut ss_bytes = [0u8; ML_KEM_SHARED_SECRET_BYTES];
	ss_bytes.copy_from_slice(ss.as_ref());
	Some(MlKemSharedSecret(ss_bytes))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ml_kem_encap_decap_roundtrip() {
		let kp = MlKemKeypair::generate();
		let (ct, ss_enc) = encapsulate(&kp.public);
		let ss_dec = decapsulate(&kp.secret, &ct).expect("decapsulation");
		assert_eq!(ss_enc, ss_dec);
	}
}
