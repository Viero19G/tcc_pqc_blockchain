# Entangle: entrega do TCC

Este repositorio contem a chain Entangle e a versao integrada do Polkadot.js Apps usada na demonstracao.

## Versoes publicas

- `main`: versao estavel de entrega baseada em `mvp-demo-tcc`.
- `mvp-demo-tcc`: demo funcional e reproducivel.
- `pqc-E2E`: variante experimental com suporte ao tipo de assinatura ML-DSA no runtime.

O frontend tambem possui um repositorio proprio em https://github.com/Viero19G/apps.

## Reproducao rapida da versao estavel

### 1. Chain

```bash
git clone https://github.com/Viero19G/tcc_pqc_blockchain.git
cd tcc_pqc_blockchain/pqc_chain
git switch main
rustup target add wasm32-unknown-unknown
cargo build -p entangle-node
./target/debug/entangle-node \
  --chain ./entangle-local-spec.json \
  --tmp --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --rpc-external --rpc-cors=all --force-authoring
```

O endpoint RPC e `ws://127.0.0.1:9944`.

### 2. Apps do fork

Em outro terminal:

```bash
git clone https://github.com/Viero19G/apps.git
cd apps
git switch main
yarn install
yarn start
```

Abra `http://localhost:3000`, conecte no endpoint `ws://127.0.0.1:9944` e acesse `Developer -> Extrinsics`.

Fluxo demonstrado:

1. `pqc.registerKeys`
2. `pqc.verifySignature`
3. Confirmacao do evento e do bloco no Explorer

## Reproducao da variante E2E

Para testar a variante experimental, use `git switch pqc-E2E` nos dois repositorios. Essa branch deve ser tratada como experimental: o runtime possui suporte ao tipo de assinatura ML-DSA, mas a demonstracao visual usa o fluxo classico de conta do Apps.

## Testes da chain

```bash
cargo check -p entangle-node
cargo test -p pqc-crypto
cargo test -p pallet-pqc
cargo test -p pallet-governance
```

## Evidencias e metricas

- Guia de reproducibilidade: `pqc_chain/docs/GUIA_REPRODUCIBILIDADE_EXPERIMENTO.md`
- Guia de demo e metricas: `pqc_chain/docs/GUIA_METRICAS_DEMO_DOCKER.md`
- Tabela comparativa: `pqc_chain/tcc-evidencias/metricas/tabela-comparativa.md`
- Evidencias: `pqc_chain/tcc-evidencias/metricas/`
- Whitepaper: `pqc_chain/docs/whitepaper_entangle_v6.md`

## Sincronizacao do frontend integrado

A chain tambem contem o Apps em `pqc_chain/frontend/apps`, importado do fork por `git subtree`. O repositorio independente continua sendo a fonte de atualizacao do frontend.

## Observacao

O token STR e a rede descritos aqui pertencem a um ambiente experimental local e nao representam ativo financeiro.
