# Guia de Reproducibilidade do Experimento Entangle

Objetivo: permitir que qualquer pessoa baixe o projeto, instale dependencias, rode a chain, rode o app adaptado e valide o funcionamento passo a passo.

## 1) O que deve existir no repositorio remoto

1. Codigo da chain (este repositorio).
2. Codigo do app adaptado para Entangle dentro de `frontend/apps`.
3. Evidencias em `tcc-evidencias/metricas`.
4. Este guia de reproducibilidade.

## 2) Como incluir o app no repositorio do TCC

Opcao recomendada (preserva rastreabilidade): git subtree.

No repo do TCC:

1. Adicionar remoto local do app:
   git remote add apps-local /CAMINHO/PARA/apps
2. Buscar historico:
   git fetch apps-local
3. Importar app em subpasta:
   git subtree add --prefix=frontend/apps apps-local master --squash

Se o branch principal do app for main, substitua master por main.

## 3) Dependencias minimas

Linux/WSL:

1. Rust via rustup
2. Target wasm:
   rustup target add wasm32-unknown-unknown
3. Node.js (LTS) e Yarn
4. Protobuf compiler (protoc)

## 4) Passo a passo de execucao

### 4.1 Chain

1. Entrar na pasta do projeto:
   cd /CAMINHO/pqc_chain
2. Build:
   cargo build --release
3. Rodar node local:
   ./target/release/entangle-node --dev --rpc-cors all --rpc-methods unsafe --rpc-external

### 4.2 App adaptado

1. Entrar na pasta do app importado:
   cd /CAMINHO/pqc_chain/frontend/apps
2. Instalar dependencias:
   yarn install
3. Rodar app:
   yarn start
4. Abrir no navegador e conectar em:
   ws://127.0.0.1:9944

## 5) Validacao funcional (checklist)

1. Enviar `pqc.registerKeys` com chave ML-DSA.
2. Confirmar `inblock` e evento `pqc.KeysRegistered`.
3. Enviar `pqc.verifySignature` com mensagem `entangle-tcc-demo` e assinatura ML-DSA.
4. Confirmar `inblock` e evento `pqc.SignatureVerified`.
5. Registrar hash da extrinsic e numero do bloco.

## 6) Validacao de metricas (PARTE 9)

1. Conferir tamanhos em `tcc-evidencias/metricas/demo-keygen-output.txt`.
2. Conferir tabela consolidada em `tcc-evidencias/metricas/tabela-comparativa.md`.
3. Conferir coleta automatica em `tcc-evidencias/metricas/medicao-automatica-parte-9.md`.

## 7) Comandos Git para subir tudo ao remoto

No repo do TCC:

1. Ver estado:
   git status
2. Adicionar arquivos:
   git add .
3. Commit:
   git commit -m "feat: chain + app adaptado + guia de reproducibilidade + metricas"
4. Push:
   git push origin BRANCH

## 8) Resultado esperado

Ao final, qualquer avaliador deve conseguir:

1. Clonar o repositorio.
2. Instalar dependencias.
3. Subir chain e app.
4. Executar os fluxos de registro e verificacao PQC.
5. Confirmar evidencias e metricas documentadas.
