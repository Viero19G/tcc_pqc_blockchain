# Guia de Branches: Chain e Frontend

Objetivo: manter uma versao do frontend alinhada para cada branch da chain.

## Estrategia recomendada

1. Branch da chain:
- `mvp-demo-tcc`: versao estavel de demo
- `pqc-E2E`: versao experimental ponta a ponta

2. Branch do frontend (repo separado):
- `mvp-demo-tcc`
- `pqc-E2E`

3. Regra:
- sempre que abrir/atualizar branch da chain, criar branch homonima no frontend.

## Fluxo com subtree (repo do TCC como agregador)

No repo do TCC/chain:

1. atualizar remoto do frontend:
- `git remote add apps-local /CAMINHO/apps` (uma vez)
- `git fetch apps-local`

2. importar frontend para a branch atual da chain:
- `git subtree pull --prefix=frontend/apps apps-local pqc-E2E --squash`

3. publicar mudancas do frontend de volta para o repo apps:
- `git subtree push --prefix=frontend/apps apps-local pqc-E2E`

## Fluxo sem subtree (simples)

1. manter repos separados e versionar branch com mesmo nome nos dois.
2. no README da branch da chain, fixar o commit/tag do frontend correspondente.

## Checklist rapido

- branch da chain criada
- branch do frontend criada com mesmo nome
- frontend atualizado para suportar tipos/assinatura da branch
- guia de reproducibilidade atualizado
- commit com hash de referencia entre repos
