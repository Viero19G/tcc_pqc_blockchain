# Criptografia Pós-Quântica — Entangle

## Decisões Técnicas (Fase 1)

### Bibliotecas escolhidas

| Algoritmo | Crate | Padrão | Justificativa |
|---|---|---|---|
| ML-DSA-65 | `ml-dsa` 0.1.1 (RustCrypto) | FIPS 204 | Pure Rust, `no_std`, integração WASM |
| ML-KEM-768 | `ml-kem` 0.3.2 (RustCrypto) | FIPS 203 | Pure Rust, sem bindings C |

**Alternativas consideradas:**
- `mldsa-native` / `mlkem-native` — performance superior, mas bindings C complicam cross-compile WASM
- `fips204` / `fips203` (IntegrityChain) — excelente para embedded, reservado para otimização futura
- liboqs — maduro, porém dependência C pesada para runtime Substrate

### Parâmetros de segurança

- **ML-DSA-65**: categoria NIST 3 (~192-bit), assinatura ~3309 bytes
- **ML-KEM-768**: categoria NIST 3 (~192-bit), ciphertext 1088 bytes

### Assinatura híbrida

```
HybridSignature
├── Classic(MultiSignature)  → Sr25519 / Ed25519 / ECDSA
└── MlDsa65(MlDsaSignature)  → FIPS 204
```

**Por quê híbrido?**
1. Compatibilidade com Polkadot.js, subxt e tooling existente (Sr25519)
2. Migração gradual — contas clássicas continuam funcionando
3. Crypto-agility preparada para Fase 3 (múltiplos schemes)

### AccountId para contas PQ

ML-DSA public key → `Blake2-256(pk)` → `AccountId32`

Mesmo padrão usado por Sr25519 no Substrate.

### Overhead on-chain

| Métrica | Clássico (Sr25519) | PQC (ML-DSA-65) | Fator |
|---|---|---|---|
| Assinatura | 64 bytes | 3309 bytes | ~52× |
| Chave pública | 32 bytes | 1952 bytes | ~61× |
| Chave secreta (seed) | 32 bytes | 32 bytes | 1× |
| Tempo verify | ~50 µs | ~3 ms | ~60× |

**Mitigações implementadas:**
- `RuntimeBlockLength` aumentado para 8 MB (default: 5 MB)
- Weights de `verify_signature` calibrados conservadoramente
- Batch verification planejada para Fase 6

### ML-KEM — Handshakes

Fluxo de sessão (Fase 1):

```
Iniciador                          Respondedor
    │                                   │
    │  encapsulate(responder_kem_pk)    │
    │──────── ciphertext ──────────────►│
    │                                   │ decapsulate(sk, ct)
    │                                   │ → shared_secret
    │◄──── SessionEstablished ──────────│
```

A chave secreta KEM **nunca** é armazenada on-chain. Apenas chaves públicas são registradas via `register_keys`.

### Próximos passos (Fase 2-3)

1. Integrar ML-DSA nos session keys de validadores (BABE + GRANDPA)
2. `pallet-staking` com suporte a chaves PQ
3. Batch verification de assinaturas PQ no block import
4. Benchmarks formais com `frame_benchmarking`

## Referências

- [FIPS 204 — ML-DSA](https://csrc.nist.gov/pubs/fips/204/final)
- [FIPS 203 — ML-KEM](https://csrc.nist.gov/pubs/fips/203/final)
- [Polkadot SDK Docs](https://docs.polkadot.com/)
- [RustCrypto ML-DSA](https://docs.rs/ml-dsa/latest/ml_dsa/)
