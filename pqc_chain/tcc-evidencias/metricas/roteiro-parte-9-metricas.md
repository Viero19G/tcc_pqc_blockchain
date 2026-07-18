# Roteiro de Coleta e Interpretacao - PARTE 9

Objetivo: voce conseguir explicar de onde vem cada metrica, como foi coletada e por que ela importa.

## 1) Historia curta para abrir a apresentacao

1. "Primeiro provamos funcionalidade" (registerKeys e verifySignature com sucesso on-chain).
2. "Depois medimos custo real" (tamanho, weight, tempo de bloco, tamanho de bloco).
3. "Por fim comparamos com redes classicas" (Bitcoin, Ethereum, Solana, Substrate classico).

## 2) De onde cada metrica surge

### 2.1 Tamanho de chave e assinatura PQC

- Origem: saida do utilitario demo-keygen.
- Arquivo: tcc-evidencias/metricas/demo-keygen-output.txt.
- Valores usados:
  - chave publica ML-DSA-65 = 1952 bytes
  - assinatura ML-DSA-65 = 3309 bytes
- Importancia: mostra custo de payload para transacao e armazenamento.

### 2.2 Referencia classica (Sr25519)

- Origem: docs/PQC.md do projeto.
- Valores usados:
  - chave publica Sr25519 = 32 bytes
  - assinatura Sr25519 = 64 bytes
- Importancia: baseline para fator de crescimento.

### 2.3 Weight de extrinsics

- Origem: Explorer no Apps apos envio de transacao.
- Coleta:
  1. enviar `pqc.registerKeys`
  2. abrir detalhes da extrinsic no Explorer
  3. anotar campo weight
  4. repetir para `pqc.verifySignature`
  5. repetir para `balances.transfer` (baseline classico)
- Importancia: custo computacional/execucao no runtime.

### 2.4 Tempo e tamanho de bloco

- Origem: Explorer e logs do node.
- Coleta tempo de bloco:
  1. pegar 20 blocos consecutivos
  2. calcular diferenca de timestamp entre bloco N e N+1
  3. media, minimo e maximo
- Coleta tamanho de bloco:
  1. abrir bloco no Explorer
  2. anotar tamanho em bytes
  3. repetir para pelo menos 10 blocos
- Importancia: impacto operacional no throughput e latencia.

## 3) Calculos prontos para usar

- Fator chave publica: 1952 / 32 = 61.00x
- Fator assinatura: 3309 / 64 = 51.70x
- Fator de weight (medido em rede local via paymentInfo):
  - registerKeys refTime = 537728000
  - verifySignature refTime = 587745000
  - balances.transferKeepAlive refTime = 651779000
  - fator register = 537728000 / 651779000 = 0.83x
  - fator verify = 587745000 / 651779000 = 0.90x
- Tempo de bloco medido (20 blocos): media 6.0 s (min 5.999 s, max 6.001 s)
- Tamanho de bloco medido (amostra de blocos vazios): 195 bytes

## 4) Frase pronta para fechar a analise tecnica

"Os resultados mostram um trade-off objetivo: o uso de ML-DSA-65 eleva significativamente o tamanho de chave e assinatura (61x e 51.7x), com reflexo esperado em custo de execucao e dados por transacao, mas entrega compatibilidade com um modelo de seguranca pos-quantico, diferenciado frente aos esquemas classicos usados nas principais redes atuais."

## 5) Roteiro de fala (2 a 3 minutos)

1. "Nosso foco nao foi apenas demonstrar funcionando, mas medir custo real."
2. "Medimos localmente com dados on-chain e Explorer, e nao por estimativa abstrata."
3. "No Entangle, assinatura e chave PQC cresceram para 3309 e 1952 bytes."
4. "Comparando com Sr25519, isso representa 51.7x e 61x."
5. "Tambem coletamos weight de register/verify e comparamos com transfer classica."
6. "Por fim, contextualizamos com redes conhecidas que ainda usam criptografia classica."
7. "Conclusao: maior custo hoje, maior resiliencia criptografica para longo prazo."

## 6) Checklist final de evidencias

- [ ] Print do registerKeys inblock + eventos
- [ ] Print do verifySignature inblock + eventos
- [ ] Hash e bloco das duas extrinsics
- [ ] Weight de registerKeys
- [ ] Weight de verifySignature
- [ ] Weight de balances.transfer
- [ ] Amostra de tempo de bloco (20 blocos)
- [ ] Amostra de tamanho de bloco (10 blocos)
- [ ] Tabela final preenchida em tabela-comparativa.md

## 7) Versionar o Apps junto com o TCC

Objetivo: registrar no repositorio do TCC que o frontend precisou de ajustes para interagir com a chain.

Opcao recomendada (historia preservada): `git subtree`

1. No repo do TCC, adicionar remoto do Apps:
  - `git remote add apps-local /caminho/para/apps`
2. Buscar historico:
  - `git fetch apps-local`
3. Importar em subpasta do TCC:
  - `git subtree add --prefix=frontend/apps apps-local master --squash`
4. Registrar no README do TCC quais arquivos foram alterados para compatibilidade com Entangle.

Opcao simples (sem historico): copiar pasta e commit unico.

1. Copiar codigo para `frontend/apps`.
2. Commit com mensagem explicita: `feat(frontend): adapta apps para chain entangle`.

Observacao:
- Se o orientador valoriza rastreabilidade, prefira subtree.
