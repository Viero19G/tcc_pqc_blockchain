//! Tamanhos fixos dos parâmetros criptográficos selecionados.
//!
//! Escolhemos ML-DSA-65 (categoria 3 / ~192-bit) e ML-KEM-768 (categoria 3)
//! como equilíbrio entre segurança e overhead on-chain.

/// Chave pública ML-DSA-65 (FIPS 204).
pub const ML_DSA65_PUBLIC_KEY_BYTES: usize = 1952;
/// Seed da chave de assinatura ML-DSA-65 (32 bytes, forma preferida).
pub const ML_DSA65_SECRET_KEY_BYTES: usize = 32;
/// Assinatura ML-DSA-65.
pub const ML_DSA65_SIGNATURE_BYTES: usize = 3309;

/// Chave pública ML-KEM-768 (FIPS 203).
pub const ML_KEM768_PUBLIC_KEY_BYTES: usize = 1184;
/// Seed da chave de decapsulação ML-KEM-768 (64 bytes, forma preferida).
pub const ML_KEM768_SECRET_KEY_BYTES: usize = 64;
/// Ciphertext ML-KEM-768.
pub const ML_KEM768_CIPHERTEXT_BYTES: usize = 1088;
/// Segredo compartilhado ML-KEM (32 bytes para todos os parâmetros).
pub const ML_KEM_SHARED_SECRET_BYTES: usize = 32;

/// Prefixo SS58 customizado para contas PQC (Substrate default = 42).
pub const PQC_SS58_PREFIX: u16 = 42;
