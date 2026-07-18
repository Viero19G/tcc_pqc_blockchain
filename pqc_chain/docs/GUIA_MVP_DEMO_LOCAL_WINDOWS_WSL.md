# Guia MVP + Demo Local (Windows + WSL) - Entangle

Este guia foi escrito para repetir, do zero, o caminho que funcionou no projeto.
Foco: rodar tudo local, com `entangle-node` + Polkadot.js Apps local (Opcao 2), pronto para demo e pitch.

## 1. Escopo da solucao

Objetivo final:
- Compilar o projeto sem erro.
- Subir o node local.
- Rodar frontend local do Polkadot.js Apps (mais novo que o site publico).
- Conectar no node e executar fluxo de demo.

Por que a Opcao 2:
- O site `polkadot.js.org/apps` pode falhar com tipos grandes (exemplo: arrays fixos > 2048 em alguns cenarios).
- Rodar Apps local reduz variaveis de extensao, cache e versao de biblioteca.
- Para demo de TCC, local e mais previsivel.

---

## 2. Pre-requisitos no WSL

## 2.1 Pacotes de sistema usados no build

```bash
sudo apt update && sudo apt upgrade -y
sudo apt install -y \
  build-essential clang llvm \
  protobuf-compiler \
  libssl-dev libudev-dev pkg-config cmake \
  curl git
```

Como foi compilado (resumo tecnico):
- `build-essential`, `clang`, `llvm`: toolchain C/C++ para dependencias low-level (ex: RocksDB).
- `protobuf-compiler`: geracao de codigo protobuf/gRPC usada por partes do ecossistema Substrate.
- `libssl-dev`, `libudev-dev`, `pkg-config`, `cmake`: libs e ferramentas de link/build para crates de rede/hardware.

## 2.2 Rust (ponto critico)

Este projeto usa versao **especifica** de Rust. Se fugir da versao, pode quebrar build WASM/runtime.

No estado atual do repo, o arquivo `rust-toolchain.toml` fixa a versao.

Conferir:
```bash
cat rust-toolchain.toml
rustc --version
cargo --version
```

Nota importante (crypto-agility de tooling):
- Voce **nao precisa** instalar manualmente `wasm32-unknown-unknown` nem componentes extras para este repo.
- O proprio `rust-toolchain.toml` instrui o `rustup` e faz a instalacao automatica quando voce roda `cargo` na pasta do projeto.

---

## 3. Build do projeto

Na pasta `pqc_chain`:

```bash
# check rapido (sem wasm)
SKIP_WASM_BUILD=1 cargo check

# build final
cargo build --release
```

Validacao esperada:
- `Finished dev profile` no check.
- `Finished release profile` no build.

---

## 4. Subir o node (modo recomendado para WSL + Windows)

Para evitar problema de bind entre WSL e navegador Windows, use RPC externo em ambiente local de demo:

```bash
./target/release/entangle-node --dev --rpc-cors all --rpc-methods unsafe --rpc-external
```

Logs esperados:
- `Running JSON-RPC server: addr=0.0.0.0:9944` (ou equivalente com IPv6)
- Blocos sendo importados (`Imported #...`)

Descobrir IP atual do WSL:

```bash
hostname -I
```

Guarde o IP (exemplo: `192.168.x.x`).

---

## 5. Opcao 2: Polkadot.js Apps local (frontend local)

## 5.1 Instalar Node/Yarn (se necessario)

Checar:
```bash
node --version
yarn --version
```

Se faltar, instalar via `nvm` (recomendado):
```bash
curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
source ~/.bashrc
nvm install --lts
npm install -g yarn
```

## 5.2 Clonar e subir o Apps local

```bash
cd ~/projetos
git clone https://github.com/polkadot-js/apps.git
cd apps
yarn install --mode=skip-build
yarn start
```

Observacao importante desta execucao real:
- Em WSL, o `yarn install` padrao pode falhar em modulos nativos opcionais (ex.: `node-hid`, `cpu-features`).
- Para demo web local, `yarn install --mode=skip-build` foi suficiente e o servidor iniciou com sucesso.

Abrir no Windows:
- `http://localhost:3000`

Tambem disponivel na rede local:
- `http://<IP_DO_WSL>:3000`

## 5.3 Conectar no endpoint do node

No Apps local, use endpoint custom:
- `ws://<IP_DO_WSL>:9944`

Exemplo:
- `ws://192.168.210.231:9944`

Se estiver em modo mirrored e funcionando, `ws://127.0.0.1:9944` tambem pode funcionar.

---

## 6. Regra RPC (Windows Firewall)

Se houver erro de conexao WS (1006), adicionar regra no PowerShell como Administrador:

```powershell
netsh advfirewall firewall add rule name="WSL Substrate RPC" dir=in action=allow protocol=TCP localport=9944
```

Validar regra:

```powershell
netsh advfirewall firewall show rule name="WSL Substrate RPC"
```

---

## 7. Checklist de diagnostico rapido (problemas reais que ocorreram)

1) Build passa com `SKIP_WASM_BUILD=1`, mas falha no runtime WASM
- Causa comum: versao Rust fora da versao suportada do repo.
- Acao: respeitar `rust-toolchain.toml`.

2) `WebSocket 1006 Abnormal Closure`
- Causa comum: endpoint errado (`127.0.0.1` vs IP WSL), firewall, ou bind restrito.
- Acao:
  - subir node com `--rpc-external`
  - usar `ws://<IP_WSL>:9944`
  - aplicar regra de firewall.

2.1) Apps conecta, mas falha com erro de metadata `Only support ... length <= 2048`
- Causa: o frontend ainda encontra tipo de array fixo grande no metadata (`[u8; 3309]`).
- Acao aplicada neste projeto:
  - `MlDsaSignature` migrou de array fixo para `Vec<u8>` em `primitives/pqc-crypto/src/mldsa.rs`.
  - `HybridSignature` removeu `MaxEncodedLen` em `primitives/pqc-crypto/src/hybrid.rs` para compatibilidade com tipo variavel.
- Validacao: `cargo check -p pqc-crypto` e `SKIP_WASM_BUILD=1 cargo check` concluindo com sucesso.

3) Porta ocupada / processo antigo
- Sintoma: comportamento inconsistente de bind.
- Acao:
```bash
pkill -9 entangle-node
ss -tlnp | grep 9944
```

4) Warnings de ciclos (`strongly connected components`)
- Isso apareceu no build e **nao bloqueia**.
- Se `Finished ...` apareceu, o build terminou com sucesso.

---

## 8. Roteiro minimo de demo local (MVP)

1. Terminal A: subir node.
2. Terminal B: subir Apps local (`yarn start`).
3. Browser: abrir `http://localhost:3000`.
4. Conectar em `ws://<IP_WSL>:9944`.
5. Em `Developer -> Extrinsics`, executar os fluxos planejados (`pqc` e `governance`).
6. Coletar prints/videos para evidencia.

---

## 9. Comandos finais (copiar e colar)

```bash
# 1) projeto
cd ~/projetos/tcc_pqc_blockchain/pqc_chain
SKIP_WASM_BUILD=1 cargo check
cargo build --release

# 2) node
./target/release/entangle-node --dev --rpc-cors all --rpc-methods unsafe --rpc-external

# 3) apps local (outro terminal)
cd ~/projetos/apps
yarn install --mode=skip-build
yarn start
```

No browser, conectar em:
- `ws://<IP_DO_WSL>:9944`

Pronto: fluxo local completo para demo e pitch.

---

## 10. Registro da execucao da Etapa 5 (Opcao 2)

Status: concluida

Evidencias tecnicas obtidas:
- Clone do repositorio `polkadot-js/apps` concluido.
- Install com `yarn install --mode=skip-build` concluido.
- `yarn start` compilou e subiu o servidor local com sucesso.
- Endpoints exibidos no console:
  - `http://localhost:3000/`
  - `http://192.168.210.231:3000/`

Proximo passo de demo:
- No Apps local, configurar endpoint custom para `ws://192.168.210.231:9944` (com node em `--rpc-external`).

---

## 11. Continuidade apos Etapa 5: Etapa 6 (geracao de chave ML-DSA)

Status: concluida

Arquivo criado:
- `primitives/pqc-crypto/examples/demo_keygen.rs`

Comando executado:

```bash
cargo run --release --example demo_keygen -p pqc-crypto
```

Resultado medido:
- Tamanho da chave publica: 1952 bytes
- Tamanho da assinatura: 3309 bytes

Evidencia salva em:
- `tcc-evidencias/metricas/demo-keygen-output.txt`

Uso na demo:
- Copiar a chave publica gerada para `pqc.registerKeys`.
- Copiar a assinatura gerada para `pqc.verifySignature`.
