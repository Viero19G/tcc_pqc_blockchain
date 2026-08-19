# Entangle

**Layer 1 soberana** construída com Rust, Substrate (Polkadot SDK) e criptografia pós-quântica (PQC).

Token nativo: **Strand** (`STR`)

## Características

| Componente | Status | Detalhes |
|---|---|---|
| Framework | ✅ Fase 0 | Substrate + FRAME (Polkadot SDK) |
| Consenso | 🔜 Fase 3 | Aura + GRANDPA (transição para BABE + PoS) |
| ML-DSA (FIPS 204) | ✅ Fase 1 | Assinaturas híbridas Sr25519 + ML-DSA-65 |
| ML-KEM (FIPS 203) | ✅ Fase 1 | Encapsulamento de chaves de sessão |
| Governança | ✅ Fase 2 | Propostas + votação ponderada em STR |
| Validadores PQC | ✅ Fase 2 | Registro de chaves PQ para authorities |
| Smart Contracts | 🔜 Fase 4 | Ink! + EVM opcional |
| Runtime Upgrades | ✅ | Forkless via `frame_system::set_code` |

## Estrutura do Projeto

```
entangle/
├── node/                    # Cliente Substrate (networking, DB, RPC)
├── runtime/                 # Lógica on-chain (STF, pallets)
├── pallets/
│   ├── governance/          # Propostas e votação STR
│   ├── pqc/                 # Gerenciamento de chaves PQC
│   └── template/            # Pallet de referência
├── primitives/
│   └── pqc-crypto/          # ML-DSA, ML-KEM, assinatura híbrida
├── docs/
│   └── PQC.md               # Documentação técnica PQC
├── scripts/
│   └── setup.ps1            # Setup Windows
└── .github/workflows/       # CI (build, clippy, test, node)
```

## Pré-requisitos

- Rust stable (ver `rust-toolchain.toml`)
- Target WASM: `rustup target add wasm32-unknown-unknown`
- Protobuf compiler (Windows: `choco install protoc`)

## Setup Rápido (Windows)

```powershell
# Instalar Rust
winget install Rustlang.Rustup

# Adicionar target WASM
rustup target add wasm32-unknown-unknown

# Build
cargo build --release

# Executar nó de desenvolvimento
./target/release/entangle-node --dev
```

## Desenvolvimento

```bash
# Build (sem WASM, mais rápido)
SKIP_WASM_BUILD=1 cargo build

# Testes
SKIP_WASM_BUILD=1 cargo test

# Formatação
cargo fmt --all

# Lint
SKIP_WASM_BUILD=1 cargo clippy --all-targets --workspace
```

## Reproducibilidade do Experimento

- Guia completo (clone, dependencias, build, execucao da chain, execucao do app e validacao fim a fim):
    - `docs/GUIA_REPRODUCIBILIDADE_EXPERIMENTO.md`

## Reproducao publica: chain + Apps

A versao funcional para a entrega esta na branch `mvp-demo-tcc` da chain e do
Apps. A branch `pqc-E2E` contem a variante experimental com suporte ao tipo de
assinatura ML-DSA no runtime.

### 1. Clonar a chain

```bash
git clone https://github.com/Viero19G/tcc_pqc_blockchain.git
cd tcc_pqc_blockchain/pqc_chain
git switch mvp-demo-tcc
```

### 2. Compilar e executar a chain

```bash
rustup target add wasm32-unknown-unknown
cargo build -p entangle-node
./target/debug/entangle-node \
    --chain ./entangle-local-spec.json \
    --tmp --alice \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    --rpc-external --rpc-cors=all --force-authoring
```

O endpoint RPC fica em `ws://127.0.0.1:9944`.

### 3. Clonar e executar o Apps do fork

Em outro terminal:

```bash
git clone https://github.com/Viero19G/apps.git
cd apps
git switch main
yarn install
yarn start
```

Abra `http://localhost:3000`, conecte em `ws://127.0.0.1:9944` e acesse
Developer -> Extrinsics. Para reproduzir o fluxo PQC, use `pqc.registerKeys` e
`pqc.verifySignature`. Para a variante E2E experimental, troque `mvp-demo-tcc`
por `pqc-E2E` nos dois repositorios.

### 4. Validar a entrega

```bash
cargo check -p entangle-node
cargo test -p pqc-crypto
cargo test -p pallet-pqc
cargo test -p pallet-governance
```

Os registros de metricas e evidencias ficam em `tcc-evidencias/metricas/`.

## PQC — Uso Básico

### Registrar chaves ML-DSA

```rust
// Via extrinsic pallet-pqc
Pqc::register_keys(
    origin,
    ml_dsa_public_key,   // 1952 bytes
    Some(ml_kem_public), // 1184 bytes (opcional)
);
```

### Assinatura híbrida

O runtime usa `HybridSignature`:
- `Classic(Sr25519)` — compatibilidade com tooling Substrate existente
- `MlDsa65 { signature, public }` — assinatura pós-quântica (~3309 bytes) com chave pública para verificação

## Governança — Uso Básico

```rust
// Submeter proposta (100 STR de depósito mínimo)
Governance::propose(origin, call, deposit);

// Votar (peso = saldo livre de STR)
Governance::vote(origin, proposal_id, aye);

// Fechar após período de votação (~1 hora)
Governance::close(origin, proposal_id);
```

## Roadmap

- [x] **Fase 0** — Setup, CI, formatação
- [x] **Fase 1** — Integração PQC (ML-DSA + ML-KEM + pallet-pqc)
- [x] **Fase 2** — Governança STR, chaves PQ de validadores, limites de bloco
- [ ] **Fase 3** — BABE + GRANDPA com chaves PQ
- [ ] **Fase 4** — Ink! + EVM
- [ ] **Fase 5** — Testes de rede e performance
- [ ] **Fase 6** — Produção e auditoria

## Licença

MIT-0
