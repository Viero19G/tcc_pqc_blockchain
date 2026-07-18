# Tabela Comparativa de Metricas (PQC vs Classico)

Objetivo: consolidar evidencias da PARTE 9 para o TCC com fonte, valor e metodo de coleta.

## 1) Metricas medidas no projeto Entangle

| Metrica | Classico (Sr25519) | PQC (ML-DSA-65) | Fator | Fonte | Status |
|---|---:|---:|---:|---|---|
| Tamanho da chave publica | 32 bytes | 1952 bytes | 61.00x | demo-keygen + docs/PQC.md | Medido |
| Tamanho da assinatura | 64 bytes | 3309 bytes | 51.70x | demo-keygen + docs/PQC.md | Medido |
| Tempo de verificacao (teorico) | ~50 us | ~3 ms | ~60x | docs/PQC.md | Documentado |
| Weight registerKeys (refTime) | 651779000 (transfer baseline) | 537728000 | 0.83x | medido via `paymentInfo` (@polkadot/api) | Medido |
| Weight verifySignature (refTime) | 651779000 (transfer baseline) | 587745000 | 0.90x | medido via `paymentInfo` (@polkadot/api) | Medido |
| Weight de referencia classica (balances.transferKeepAlive) | 651779000 | n/a | baseline | medido via `paymentInfo` (@polkadot/api) | Medido |
| Tempo medio de bloco (rede local, 20 blocos) | ~6.0 s | ~6.0 s | n/a | medido por timestamps on-chain | Medido |
| Tamanho de bloco (bytes, amostra vazia) | 195 | 195 | n/a | medido por `block.toU8a().length` | Medido |

Notas de medicao:
- Weights coletados em rede local via `api.tx.*.paymentInfo(alice)` (estimativa de custo da extrinsic sem envio).
- Em cenarios com bloco cheio e mais leitura/escrita de storage, o custo relativo pode mudar.
- O tamanho de bloco acima e de blocos quase vazios; para analise final, capture tambem blocos contendo as extrinsics PQC.

## 2) Metricas tipicas de blockchains conhecidas (referencia externa)

Observacao: tabela de contexto para apresentacao. Sao valores tipicos de protocolo/rede e podem variar conforme versao e configuracao.

| Rede | Assinatura tipica | Chave publica tipica | Tempo de bloco/slot | Observacao |
|---|---:|---:|---|---|
| Bitcoin (secp256k1) | ~71-73 bytes (DER ECDSA) | 33 bytes (compressed pubkey) | ~10 min | Foco em seguranca e descentralizacao |
| Ethereum (secp256k1) | 65 bytes (r,s,v) | 64 bytes (pubkey sem prefixo; endereco 20 bytes) | ~12 s | Assinatura em transacoes EOA |
| Solana (Ed25519) | 64 bytes | 32 bytes | ~400 ms slot | Alta vazao com menor latencia |
| Polkadot/Substrate (Sr25519/Ed25519) | 64 bytes | 32 bytes | ~6 s (comum em dev chain) | Referencia classica pro ecossistema Substrate |
| Entangle PQC (ML-DSA-65) | 3309 bytes | 1952 bytes | depende do chain spec | Overhead maior, resiliencia pos-quantica |

## 3) Leitura de resultado (texto pronto para o TCC)

Resumo tecnico sugerido:

"Os resultados experimentais mostram que a adocao de ML-DSA-65 aumenta substancialmente o tamanho de artefatos criptograficos em relacao ao padrao classico Sr25519. No Entangle, a chave publica cresce de 32 para 1952 bytes (61x) e a assinatura de 64 para 3309 bytes (51.7x). Em contrapartida, essa elevacao de custo de armazenamento e transmissao traz compatibilidade com cenarios de ameaca pos-quantica, caracterizando um trade-off claro entre eficiencia e robustez criptografica de longo prazo."

## 4) Coleta complementar recomendada (rapido)

1. Enviar `pqc.registerKeys` no Apps.
2. Abrir Explorer e registrar `weight`, hash, bloco, eventos.
3. Enviar `pqc.verifySignature` e registrar os mesmos campos.
4. Enviar `balances.transfer` (valor minimo) como baseline classico.
5. Calcular fator de weight: `weight_pqc / weight_classico`.
6. Registrar tempo de bloco por amostragem de 20 blocos consecutivos.
7. Salvar evidencias (print + hash extrinsic + bloco + timestamp).

## 5) Fontes

- Interna (projeto): docs/PQC.md
- Interna (projeto): tcc-evidencias/metricas/demo-keygen-output.txt
- Externa (contexto):
  - NIST FIPS 203 e FIPS 204
  - Bitcoin Developer Reference
  - Ethereum docs (transaction signature / secp256k1)
  - Solana docs (Ed25519 / slot time)
  - Polkadot docs (Substrate signatures and block production)
