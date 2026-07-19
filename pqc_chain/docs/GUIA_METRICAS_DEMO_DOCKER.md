# Guia Unico: Metricas, Demo e Docker Multi-No

## 1) Metricas que precisam ser coletadas

Ja consolidadas:
- Chave publica ML-DSA-65: 1952 bytes
- Assinatura ML-DSA-65: 3309 bytes

Coletar no Explorer para cada extrinsic:
- `dispatchInfo.weight.refTime`
- `dispatchInfo.weight.proofSize`
- hash da extrinsic
- numero do bloco
- eventos emitidos

Extrinsics obrigatorias para a tabela:
- `balances.transferKeepAlive` (baseline classico)
- `pqc.registerKeys`
- `pqc.verifySignature`
- `governance.propose`
- `governance.vote`
- `governance.close`

Calculos:
- fator_pqc = weight_pqc / weight_transfer
- tempo_ms = (refTime / 1_000_000_000_000) * 1000

## 2) Roteiro de demo (5-6 min)

Cena 1 (30s): mostrar rede viva (blocos incrementando).
Cena 2 (1min): `balances.transferKeepAlive` para baseline.
Cena 3 (1.5min): `pqc.registerKeys` e evento `pqc.KeysRegistered`.
Cena 4 (1.5min): `pqc.verifySignature` e evento `pqc.SignatureVerified`.
Cena 5 (1min): `governance.propose` + `vote`; `close` por video/backup.
Cena 6 (30s): mostrar tabela comparativa final.

Frase de fechamento sugerida:
"Nos resultados medidos, o custo de dados e processamento em PQC aumenta, mas entrega resiliencia pos-quantica com validacao on-chain real."

## 3) Docker no WSL (instalacao)

Opcao A (recomendada): Docker Desktop no Windows + integracao WSL2.
1. Instalar Docker Desktop.
2. Em Settings > Resources > WSL Integration, habilitar sua distro.
3. Validar no WSL:
   - `docker --version`
   - `docker compose version`

Opcao B (Engine nativo no WSL):
1. `curl -fsSL https://get.docker.com -o get-docker.sh`
2. `sudo sh get-docker.sh`
3. `sudo usermod -aG docker $USER`
4. `newgrp docker`
5. `docker --version`

## 4) Build das imagens

Usando Dockerfile existente (build completo):
- `docker build -t entangle-node:local .`

Usando build rapido (binario local ja compilado):
- `docker build -f Dockerfile.fast -t entangle-node:local .`

## 5) Subir rede com containers

Arquivos usados:
- `docker-compose.yml`
- `entangle-local-spec.json`

Subir:
- `docker compose up -d`

Logs:
- `docker compose logs -f`
- `docker compose logs -f alice`

Parar:
- `docker compose down`

Reset completo:
- `docker compose down -v`

## 6) Endpoints para o Apps

- Alice: `ws://127.0.0.1:9944`
- Bob: `ws://127.0.0.1:9945`
- Charlie: `ws://127.0.0.1:9946`

## 7) Validacao de conectividade multi-no

Esperado nos logs:
- peers > 0
- blocos importando nos tres nos
- finalizacao avancando

## 8) Teste executado nesta sessao

Executado e validado:
- geracao de `entangle-local-spec.json`
- criacao de `docker-compose.yml`
- criacao de `Dockerfile.fast`
- peer id fixo derivado para node-key da Alice:
  - `12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp`

Nao executado aqui por bloqueio de ambiente:
- comandos `docker` e `docker compose` (Docker nao instalado neste WSL no momento).
