//! # PQC Crypto Primitives
//!
//! Camada de abstração criptográfica para a Entangle.
//!
//! Implementa os padrões NIST:
//! - **ML-DSA** (FIPS 204) — assinaturas pós-quânticas (parâmetro MlDsa65 recomendado)
//! - **ML-KEM** (FIPS 203) — encapsulamento de chaves (parâmetro MlKem768)
//!
//! Suporta modo **híbrido**: assinaturas clássicas (Sr25519) coexistem com ML-DSA
//! durante a transição, permitindo crypto-agility nas fases seguintes.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod constants;
pub mod hybrid;
pub mod mldsa;
pub mod mlkem;

pub use constants::*;
pub use hybrid::{HybridPublic, HybridSignature, SignatureScheme};
pub use mldsa::{MlDsaKeypair, MlDsaPublicKey, MlDsaSecretKey, MlDsaSignature};
pub use mlkem::{MlKemCiphertext, MlKemKeypair, MlKemPublicKey, MlKemSecretKey, MlKemSharedSecret};
