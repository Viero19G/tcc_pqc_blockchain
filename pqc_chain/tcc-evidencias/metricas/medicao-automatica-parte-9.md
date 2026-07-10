# Medicao Automatica - PARTE 9

Data: 2026-07-10
Metodo: script Node.js com `@polkadot/api` conectado em `ws://127.0.0.1:9944`.

## 1) Weights medidos via paymentInfo

- `pqc.registerKeys`
  - refTime: 537728000
  - proofSize: 12184
  - partialFee: 645887061

- `pqc.verifySignature`
  - refTime: 587745000
  - proofSize: 12184
  - partialFee: 695905469

- `balances.transferKeepAlive` (baseline classico)
  - refTime: 651779000
  - proofSize: 15777
  - partialFee: 759936147

Fatores (refTime):
- register_vs_transfer: 0.8250x
- verify_vs_transfer: 0.9018x

## 2) Tempo de bloco (amostra 20 blocos)

- intervalos medidos: 19
- media: 6.000 s
- minimo: 5.999 s
- maximo: 6.001 s

## 3) Tamanho de bloco (amostra 20 blocos)

- media: 195 bytes
- minimo: 195 bytes
- maximo: 195 bytes

Observacao:
- Essa amostra foi coletada em blocos quase vazios em rede local de desenvolvimento.
- Recomenda-se repetir com blocos contendo extrinsics PQC para capturar impacto em payload real.

## 4) Blocos recentes usados como referencia

- #25 `0x8a8fc6986ba020a2f7d09ef7bd165b58d28c74c2d24c2118a0cfca4c7b442f5c`
- #26 `0xcb43c0d5dc6bc42f7b539a8f75b615a7ed2966b97e2fc8fea12476bd3ce123dc`
- #27 `0x758c9c913bd24314a8ecae4fae85adff4bd945fb032e14553e693aaf701ae191`
- #28 `0x660e9b15ea737b22769e976af2966396c9b1ca8f281b2c6671541724e3c7a86c`
- #29 `0xaa572f0960025608df67f5018d3e2f78ab2c1e4f88cf9276e989c1d2a69efc1b`
