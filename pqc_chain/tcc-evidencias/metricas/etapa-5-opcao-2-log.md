# Etapa 5 - Opcao 2 (Polkadot.js Apps local)

Data: 2026-07-09

## Ambiente
- Node: executando local em modo dev com RPC externo.
- Endpoint WS do node: ws://192.168.210.231:9944

## Comandos executados

1) Instalacao de Node/Yarn no WSL:
- nvm install --lts
- npm install -g yarn

2) Apps local:
- cd ~/projetos
- git clone https://github.com/polkadot-js/apps.git
- cd apps
- yarn install --mode=skip-build
- yarn start

## Resultado
- Webpack compilou com sucesso.
- Servidor local disponivel em:
  - http://localhost:3000/
  - http://192.168.210.231:3000/

## Observacao tecnica
- O install padrao pode falhar em modulos nativos opcionais (node-hid/cpu-features) no WSL.
- Para demo web local, o modo skip-build foi suficiente.
