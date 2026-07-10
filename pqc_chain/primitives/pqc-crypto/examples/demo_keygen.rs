//! Utilitario de demonstracao: gera um par de chaves ML-DSA-65 e uma
//! assinatura de exemplo, imprimindo tudo em hexadecimal para uso manual
//! na interface do Polkadot.js Apps durante a demo do TCC.
//!
//! Este arquivo NAO faz parte do protocolo (nao e pallet, nao e runtime).
//! E apenas uma ferramenta de apoio para a apresentacao.

use pqc_crypto::mldsa::MlDsaKeypair;

fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(2 + bytes.len() * 2);
    out.push_str("0x");

    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }

    out
}

fn main() {
    let message = b"entangle-tcc-demo";

    let keypair = MlDsaKeypair::generate();
    let signature = keypair.sign(message);

    println!("=== Chaves ML-DSA-65 geradas para demonstracao ===");
    println!();
    println!("Chave publica (cole no campo ml_dsa_public do register_keys):");
    println!("{}", to_hex(&keypair.public.0));
    println!();
    println!("Tamanho da chave publica: {} bytes", keypair.public.0.len());
    println!();
    println!("Mensagem assinada: {:?}", String::from_utf8_lossy(message));
    println!("Assinatura (cole no campo signature do verify_signature):");
    println!("{}", to_hex(&signature.0));
    println!();
    println!("Tamanho da assinatura: {} bytes", signature.0.len());
}
