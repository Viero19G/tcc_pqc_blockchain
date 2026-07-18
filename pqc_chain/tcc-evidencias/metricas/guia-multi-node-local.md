# Guia Multi-Node Local (2 ou 3 nos)

Objetivo: subir uma rede local com nos conectados entre si na mesma maquina, usando o binario `entangle-node`.

## 0) Preparo (uma vez)

```bash
cd /home/gabriel_viero/projetos/tcc_pqc_blockchain/pqc_chain
./target/release/entangle-node build-spec --chain local --disable-default-bootnode > /tmp/entangle-local-spec.json
```

Opcional: limpar execucoes antigas.

```bash
pkill -f entangle-node || true
rm -rf /tmp/entangle-nodes
```

## 1) Rede com 2 nos validadores (Alice + Bob)

Abra 2 terminais.

Terminal 1 (Alice):

```bash
cd /home/gabriel_viero/projetos/tcc_pqc_blockchain/pqc_chain
./target/release/entangle-node \
  --base-path /tmp/entangle-nodes/alice \
  --chain /tmp/entangle-local-spec.json \
  --alice \
  --validator \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-methods unsafe \
  --rpc-cors all
```

No log da Alice, copie o valor de `Local node identity`, exemplo:

`12D3KooW...`

Monte o bootnode com esse valor:

`/ip4/127.0.0.1/tcp/30333/p2p/<PEER_ID_ALICE>`

Terminal 2 (Bob):

```bash
cd /home/gabriel_viero/projetos/tcc_pqc_blockchain/pqc_chain
./target/release/entangle-node \
  --base-path /tmp/entangle-nodes/bob \
  --chain /tmp/entangle-local-spec.json \
  --bob \
  --validator \
  --port 30334 \
  --rpc-port 9945 \
  --rpc-methods unsafe \
  --rpc-cors all \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<PEER_ID_ALICE>
```

## 2) Opcao com 3 nos (2 validadores + 1 observador)

Abra um terceiro terminal para o no observador (sem validar):

```bash
cd /home/gabriel_viero/projetos/tcc_pqc_blockchain/pqc_chain
./target/release/entangle-node \
  --base-path /tmp/entangle-nodes/charlie \
  --chain /tmp/entangle-local-spec.json \
  --name Charlie \
  --port 30335 \
  --rpc-port 9946 \
  --rpc-methods unsafe \
  --rpc-cors all \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<PEER_ID_ALICE>
```

## 3) Como validar que eles estao se comunicando

Nos logs de cada no, procure por:

- `Idle (1 peers)` ou mais
- blocos sendo importados continuamente
- melhor bloco (`best`) aumentando em todos os nos

## 4) Comandos de teste rapido RPC

No 2o no (porta 9945):

```bash
curl -s -H 'Content-Type: application/json' \
  -d '{"id":1,"jsonrpc":"2.0","method":"system_health","params":[]}' \
  http://127.0.0.1:9945
```

No 3o no (porta 9946), se iniciado:

```bash
curl -s -H 'Content-Type: application/json' \
  -d '{"id":1,"jsonrpc":"2.0","method":"system_health","params":[]}' \
  http://127.0.0.1:9946
```

## 5) Encerrar tudo

```bash
pkill -f entangle-node
```

## Observacoes

- O `--chain local` e o mais indicado para multi-no em ambiente de laboratorio.
- Em apresentacao, mantenha 2 validadores (Alice/Bob) e 1 observador opcional para mostrar conectividade.
- Para evitar conflito de portas, mantenha os pares:
  - Alice: p2p 30333 / rpc 9944
  - Bob: p2p 30334 / rpc 9945
  - Charlie: p2p 30335 / rpc 9946
