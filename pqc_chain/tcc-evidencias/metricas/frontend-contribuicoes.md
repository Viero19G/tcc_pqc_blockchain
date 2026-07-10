# Contribuicoes de Frontend para o TCC

Arquivo de acompanhamento para registrar adaptacoes feitas no frontend local do demo, especialmente no Apps da Polkadot, quando elas forem parte da solucao tecnica e da reproducibilidade da demonstracao.

## Objetivo

Documentar ajustes de compatibilidade entre o frontend e o runtime/pallets da cadeia PQC, sem alterar a regra de negocio ou enfraquecer o protocolo.

## Contribuicoes ja aplicadas

- Override de tipos para a chain `entangle` no Apps local, incluindo os tipos PQC usados pelos parametros da call.
- Registro dos tipos `MlDsaPublicKey`, `MlDsaSignature` e tipos auxiliares de ML-KEM.
- Reversao do experimento de extrinsic híbrido no Apps e retorno da assinatura de transacao para o fluxo classico compativel com `register_keys`.
- Confirmacao de que o problema real estava no codec da assinatura de transacao, nao no campo `ml_dsa_public`.
- Validação do bundle de tipos do Apps com `yarn build:typesBundle`.

## Proximas contribuicoes possiveis

- Ajustar telas ou mensagens do Apps para orientar o usuario no fluxo PQC.
- Adicionar apoio visual para o call `pqc.registerKeys` e seu retorno.
- Registrar capturas e evidencias do fluxo assinado com a assinatura ML-DSA.
- Melhorar a experiencia de copia e colagem da chave publica e da assinatura para o demo.

## Evidencias relacionadas

- [demo-keygen-output.txt](demo-keygen-output.txt)
- [etapa-5-opcao-2-log.md](etapa-5-opcao-2-log.md)
- [ambiente-build.txt](ambiente-build.txt)
- [node-startup.txt](node-startup.txt)

## Observacoes

- Sempre que houver mudanca em tipos do Apps, o ideal e reiniciar o frontend local e recarregar a pagina para garantir que o registry seja refeito.
- Se o erro persistir, a proxima verificacao e confirmar se o call esta usando a chain correta e se o `specName` carregado e `entangle`.