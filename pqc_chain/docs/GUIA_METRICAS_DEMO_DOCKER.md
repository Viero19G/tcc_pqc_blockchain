# Guia Unico: Demo e Criacao de Metricas (MVP e PQC-E2E)

Objetivo: executar uma demo reproduzivel e coletar metricas comparaveis entre a versao estavel e a versao E2E.

## 1) Estrutura de branches usada

Repositorios separados:

1. Chain: `/home/gabriel_viero/projetos/tcc_pqc_blockchain/pqc_chain`
2. Apps: `/home/gabriel_viero/projetos/apps`

Branches:

1. `mvp-demo-tcc`: versao estavel para demo tradicional
2. `pqc-E2E`: versao com caminho de assinatura hibrida E2E

## 2) Passo a passo rapido (pre-demo)

### 2.1 Escolher o cenario da demo

Demo estavel:

```bash
cd /home/gabriel_viero/projetos/tcc_pqc_blockchain/pqc_chain
git switch mvp-demo-tcc

cd /home/gabriel_viero/projetos/apps
git switch mvp-demo-tcc
```

Demo PQC E2E:

```bash
cd /home/gabriel_viero/projetos/tcc_pqc_blockchain/pqc_chain
git switch pqc-E2E

cd /home/gabriel_viero/projetos/apps
git switch pqc-E2E
```

### 2.2 Subir o backend (chain)

No cenario `pqc-E2E`, use force authoring para bloco local sem peers:

```bash
cd /home/gabriel_viero/projetos/tcc_pqc_blockchain/pqc_chain
./target/debug/entangle-node \
  --chain ./entangle-local-spec.json \
  --tmp \
  --alice \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --rpc-external \
  --rpc-cors=all \
  --force-authoring
```

Saida esperada:

1. linha com `Running JSON-RPC server: addr=0.0.0.0:9944`
2. linhas `Imported #N` incrementando

### 2.3 Subir o frontend

```bash
cd /home/gabriel_viero/projetos/apps
yarn start
```

Abrir:

1. `http://localhost:3000`
2. conectar endpoint `ws://127.0.0.1:9944`

## 3) Passo a passo da demo (5 a 6 min)

1. Mostrar rede conectada e blocos subindo em Explorer.
2. Ir em Developer -> Extrinsics.
3. Enviar baseline classico:
  1. `balances.transferKeepAlive`
4. Enviar fluxo PQC:
  1. `pqc.registerKeys`
  2. `pqc.verifySignature`
5. Mostrar eventos emitidos e hash da extrinsic.
6. Encerrar com tabela comparativa e conclusao.

Frase final sugerida:

"O custo computacional e de dados aumenta no fluxo PQC, mas com ganho direto de resiliencia criptografica pos-quantica e validacao on-chain." 

## 4) Passo a passo de criacao de metricas

## 4.1 O que coletar por extrinsic

1. hash da extrinsic
2. numero do bloco
3. evento principal de sucesso
4. `dispatchInfo.weight.refTime`
5. `dispatchInfo.weight.proofSize`

Extrinsics obrigatorias:

1. `balances.transferKeepAlive` (baseline)
2. `pqc.registerKeys`
3. `pqc.verifySignature`
4. `governance.propose`
5. `governance.vote`
6. `governance.close`

## 4.2 Como coletar no Apps

1. Depois de cada envio, abrir Explorer -> bloco da extrinsic.
2. Registrar hash, bloco e eventos.
3. Copiar `refTime` e `proofSize` da chamada.

## 4.3 Como calcular os indicadores

1. fator de custo relativo:
  1. `fator_pqc = weight_pqc / weight_transfer`
2. tempo aproximado em ms:
  1. `tempo_ms = (refTime / 1_000_000_000_000) * 1000`

## 4.4 Onde salvar evidencias

1. Tabela principal:
  1. `tcc-evidencias/metricas/tabela-comparativa.md`
2. Roteiro da parte 9:
  1. `tcc-evidencias/metricas/roteiro-parte-9-metricas.md`
3. Medicao automatica:
  1. `tcc-evidencias/metricas/medicao-automatica-parte-9.md`

## 5) Checklist final para banca

1. Branches corretas em chain e apps.
2. Node respondendo em `ws://127.0.0.1:9944`.
3. Frontend conectado na rede local.
4. Fluxos `registerKeys` e `verifySignature` com sucesso.
5. Tabela de metricas preenchida e revisada.

## 6) Docker multi-no (opcional para demonstracao estendida)

Arquivos:

1. `Dockerfile.fast`
2. `docker-compose.yml`
3. `entangle-local-spec.json`

Comandos:

```bash
docker compose up -d
docker compose logs -f alice
docker compose down
```

Observacao:

1. Neste ambiente WSL, Docker pode nao estar instalado. Se faltar, rode a demo em modo local sem containers (secoes 2 a 5).
