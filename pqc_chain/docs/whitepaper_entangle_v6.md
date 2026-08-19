# Entangle

## Uma Layer 1 Soberana com Criptografia Pós-Quântica Nativa

**Whitepaper — TCC, Pós-Graduação em Blockchain, Smart Contracts e Tokenização — Turma 1 (2025) — NearX**

**Autores:**
- Gabriel Viero — gabrielviero22@gmail.com
- Vinicius de Carvalho Viana — vinicius200691@gmail.com
- Filêmon de Castro Santos — filemoncsantos@gmail.com

**Versão:** 1.0
**Data:** 18/08/2026
**Token nativo:** Strand (STR)

---

## Resumo

As blockchains atuais dependem de assinaturas de curvas elípticas. O algoritmo de Shor [1] quebra a base matemática dessas assinaturas em um computador quântico tolerante a falhas. Esse é o problema atacado neste trabalho. A resposta proposta é a Entangle: uma blockchain Layer 1 soberana, escrita em Rust sobre o Substrate (Polkadot SDK), com os padrões pós-quânticos do NIST integrados como primitivas nativas do protocolo — ML-DSA-65 (FIPS 204) [2] para assinaturas e ML-KEM-768 (FIPS 203) [3] para encapsulamento de chaves. O diferencial está no desenho híbrido do protocolo: um tipo de assinatura em enum que acomoda, na mesma cadeia, contas clássicas (Sr25519/Ed25519/ECDSA) e contas pós-quânticas, concebido para migração gradual, sem fork. Nesta fase, a verificação ML-DSA opera on-chain em nível de aplicação, via extrinsic dedicado; a autenticação de origem de transações com ML-DSA integra a fase seguinte do roadmap. A metodologia foi experimental: o protocolo foi implementado em três fases (fundação, primitivas PQC, governança), os custos de ML-DSA frente ao ECDSA foram medidos em duas campanhas de benchmark e o modelo foi validado em dois cenários complementares: uma simulação de blocos com transações pós-quânticas e um MVP de notarização com âncora blockchain. Os resultados mostram que a integração dos padrões FIPS 203/204 a um runtime WASM é viável hoje. O custo dominante do pós-quântico é espaço (assinaturas ~52× maiores), não tempo: a verificação ML-DSA-65 ficou na mesma ordem de grandeza do ECDSA nos ambientes medidos. As mitigações adotadas (blocos de 8 MB e contas protegidas por hash) absorvem esse overhead. O trabalho entrega uma arquitetura de referência aberta para a transição pós-quântica de blockchains.

**Palavras-chave:** blockchain; criptografia pós-quântica; ML-DSA; ML-KEM; Substrate; assinatura híbrida; Layer 1.

---

## 1. Introdução

### 1.1 Contexto

Bitcoin, Ethereum e praticamente todas as redes Web3 autenticam transações com assinaturas de curvas elípticas: ECDSA secp256k1, Ed25519, Sr25519. A segurança desses esquemas depende de um único fato: o problema do logaritmo discreto elíptico é intratável para computadores clássicos. Em 1994, Shor provou que esse fato deixa de valer diante de um computador quântico tolerante a falhas [1]. Em 2024, o NIST respondeu e publicou os primeiros padrões de criptografia pós-quântica (PQC): FIPS 204 (ML-DSA, assinaturas sobre reticulados) [2], FIPS 203 (ML-KEM, encapsulamento de chaves) [3] e FIPS 205 (SLH-DSA, assinaturas baseadas em hash) [4]. A migração da infraestrutura digital global já começou. As blockchains, por serem imutáveis e de longa vida, estão entre os sistemas mais expostos.

### 1.2 Problema

O risco quântico é mais grave para blockchains do que para sistemas convencionais, por três razões.

Primeira: em blockchain, a chave pública é revelada no gasto — e muitas vezes antes, por reutilização de endereço, contratos e chaves de validadores. Uma fração substancial dos ativos em circulação nas principais redes está em endereços com chave pública já exposta. Esses ativos são alvo direto de um futuro adversário quântico.

Segunda: o padrão de ataque *harvest now, decrypt later*. Um adversário coleta hoje o material criptográfico público e ataca depois, quando tiver o hardware. Para registros imutáveis, a ameaça futura vira risco presente.

Terceira: rede descentralizada não tem botão de atualização. Migrar milhões de contas e o consenso de uma cadeia viva leva anos e exige coexistência de esquemas durante todo o processo.

As iniciativas existentes tratam PQC como remendo sobre arquiteturas desenhadas para assinaturas de 64 bytes. Essa é a lacuna endereçada neste trabalho.

### 1.3 Motivação

Só uma cadeia soberana controla o próprio formato de transação, o esquema de assinatura, o consenso e a política de evolução. Parachain e contrato inteligente herdam as premissas criptográficas da camada de baixo. A motivação central deste trabalho é demonstrar, com implementação funcional e números medidos, que é possível desenhar uma Layer 1 nativamente pós-quântica: uma cadeia em que o overhead de assinaturas de ~3,3 KB é premissa de dimensionamento (de blocos, de pesos, do modelo de contas) e em que o caminho de migração híbrido está embutido no próprio sistema de tipos do protocolo.

### 1.4 Objetivos

**Objetivo geral:** projetar e implementar uma blockchain Layer 1 soberana com criptografia pós-quântica nativa, baseada nos padrões NIST FIPS 203 e FIPS 204, avaliando viabilidade técnica e custos de desempenho.

**Objetivos específicos:**

- Integrar ML-DSA-65 e ML-KEM-768 como primitivas `no_std`/WASM de um runtime Substrate, com um tipo de assinatura híbrida (clássica + pós-quântica) projetado para a migração gradual da cadeia;
- Implementar o ciclo de vida on-chain de chaves pós-quânticas (registro, verificação, sessões ML-KEM, revogação) e uma governança ponderada pelo token nativo Strand (STR);
- Medir os custos de tempo e espaço de ML-DSA frente ao ECDSA em campanhas reprodutíveis e discutir as mitigações arquiteturais;
- Validar o modelo em cenários complementares: simulação de blocos com transações pós-quânticas e um MVP de notarização com âncora blockchain e assinatura ML-DSA.

### 1.5 Organização do documento

A Seção 2 cobre o referencial teórico: ameaça quântica, padrões NIST e o estado da arte de PQC em blockchains. A Seção 3 descreve a metodologia. A Seção 4 é o núcleo do whitepaper: a arquitetura da Entangle (camadas, criptografia, pallet PQC, consenso, token Strand e governança). A Seção 5 apresenta e discute os resultados. A Seção 6 traz o roadmap e o impacto esperado. A Seção 7 conclui e lista os trabalhos futuros. Seguem as referências e os anexos (reprodutibilidade, MVP de notarização e simulação de cadeia).

---

## 2. Referencial Teórico

### 2.1 A ameaça quântica à criptografia de curvas elípticas

O algoritmo de Shor [1] resolve o logaritmo discreto e a fatoração em tempo polinomial num computador quântico tolerante a falhas (FTQC). Isso quebra ECDSA, Ed25519, Sr25519, RSA e Diffie-Hellman — a base inteira, não uma parte. Já o algoritmo de Grover só oferece ganho quadrático contra primitivas simétricas e funções hash. Por isso AES-256 e as famílias SHA-2/SHA-3 (e BLAKE2) continuam seguras com margem adequada. Essa assimetria define o desenho de qualquer sistema pós-quântico: troca-se a criptografia assimétrica; mantêm-se hashes e cifras simétricas.

Ninguém sabe a data de um FTQC criptograficamente relevante. Mas a decisão de migrar não depende dessa data: o modelo *harvest now, decrypt later* e o ciclo de migração de infraestrutura crítica, que se mede em anos, exigem começar antes. É a posição do próprio NIST e de agências de segurança nacionais.

### 2.2 Os padrões pós-quânticos do NIST

Os padrões de 2024 se apoiam em problemas para os quais não se conhece vantagem quântica exponencial:

- **ML-DSA (FIPS 204)** [2] — assinaturas sobre reticulados modulares (Module-LWE/SIS), derivadas do CRYSTALS-Dilithium. Os parâmetros ML-DSA-44/65/87 correspondem às categorias de segurança NIST 2, 3 e 5;
- **ML-KEM (FIPS 203)** [3] — encapsulamento de chaves derivado do CRYSTALS-Kyber, nas variantes 512/768/1024;
- **SLH-DSA (FIPS 205)** [4] — assinaturas *stateless* baseadas em hash. Premissas conservadoras, tamanhos grandes; serve como âncora de confiança de longo prazo.

O custo característico dos reticulados é tamanho: chaves e assinaturas na casa dos quilobytes, contra dezenas de bytes no mundo elíptico. Esse é o desafio central da adoção em blockchain — e o que a Entangle dimensiona por desenho.

### 2.3 PQC em blockchains: estado da arte e lacuna

A indústria já se move: CRYSTALS-Dilithium no XRPL [5], a testnet pós-quântica da BTQ [6], propostas de assinaturas Winternitz no ecossistema Solana. Todas essas abordagens têm algo em comum: adaptam PQC a arquiteturas concebidas para assinaturas compactas. Herdam limites de bloco, formatos de transação e modelos de conta do mundo clássico.

O que não existe é uma cadeia desenhada desde a origem para o regime pós-quântico, com coexistência de esquemas embutida no protocolo como mecanismo de migração de primeira classe. A Entangle ocupa essa lacuna.

### 2.4 Substrate e o Polkadot SDK

O Substrate [7] é um framework Rust para construção de blockchains soberanas. Divide-se em cliente (networking libp2p, banco de dados, RPC, consenso) e runtime, a função de transição de estados, compilada para WASM e composta por módulos (*pallets*) via FRAME. Duas propriedades motivaram sua escolha: liberdade total sobre os tipos fundamentais do protocolo, incluindo o tipo de assinatura das transações; e *forkless runtime upgrades* via `frame_system::set_code`, que permitem evoluir a criptografia da cadeia por governança, sem fork. Para uma migração criptográfica que vai durar anos, essa segunda propriedade não é conveniência — é requisito.

---

## 3. Metodologia

### 3.1 Tipo de pesquisa

Pesquisa aplicada, experimental e exploratória, sob a lógica de *design science research*: concepção de um artefato (o protocolo Entangle), implementação em fases incrementais e avaliação com medições reprodutíveis e cenários de validação.

### 3.2 Materiais

- **Protocolo:** Rust estável, Substrate/Polkadot SDK (FRAME, Aura, GRANDPA). Para criptografia pós-quântica, os crates puros em Rust da família RustCrypto (`ml-dsa` 0.1.1 e `ml-kem` 0.3.2 [8]), escolhidos por compatibilidade `no_std`/WASM e ausência de bindings C. Alternativas avaliadas (liboqs, `mldsa-native`, `fips203`/`fips204`) foram descartadas nesta fase: ou complicam a cross-compilação para WASM, ou ficam reservadas para otimização futura;
- **Benchmarks:** runner Node.js com `@noble/post-quantum` e `@noble/secp256k1` [9], mensagem fixa de 32 bytes, resultados versionados em CSV/JSON com metadados de ambiente;
- **Validações complementares:** simulador de cadeia com transações ML-DSA-65 (Node.js) e MVP de notarização (Hardhat/Solidity + React), descritos nos Anexos B e C.

### 3.3 Procedimentos

1. **Fase 0 — Fundação:** node e runtime Substrate operacionais (Aura + GRANDPA, blocos de 6 s), token nativo STR, gênese de desenvolvimento;
2. **Fase 1 — Primitivas PQC:** crate `pqc-crypto` (`no_std`) com ML-DSA-65, ML-KEM-768, tipos de assinatura híbrida e derivação de contas; `pallet-pqc` com registro de chaves, verificação ML-DSA on-chain e âncora de sessões ML-KEM;
3. **Fase 2 — Governança:** `pallet-governance` com propostas, votação ponderada em STR e execução autônoma; registro de chaves pós-quânticas de validadores, preparando o consenso PoS;
4. **Campanhas de medição:** duas campanhas de benchmark ECDSA × ML-DSA-44/65/87 em ambientes distintos (Seção 5.1), com metodologia declarada;
5. **Validação e análise:** simulação de blocos com verificação ML-DSA obrigatória antes da raiz de Merkle; MVP de notarização exercitando o modelo "âncora on-chain + prova pós-quântica"; interpretação dos custos e das mitigações.

### 3.4 Ferramentas utilizadas

Rust, Substrate/Polkadot SDK (FRAME), WASM, crates RustCrypto (`ml-dsa`, `ml-kem`), Node.js, `@noble/post-quantum`, `@noble/secp256k1`, Hardhat, Solidity, TypeScript, React, Git (CI com build, clippy e testes).

---

## 4. A Entangle: Arquitetura do Protocolo

### 4.1 Visão geral em camadas

```
┌────────────────────────────────────────────────────────────┐
│  NODE (cliente Rust)                                       │
│  networking (libp2p) · banco de dados · RPC · consenso     │
│  Aura (produção de blocos) + GRANDPA (finalidade)          │
├────────────────────────────────────────────────────────────┤
│  RUNTIME (WASM — lógica on-chain / STF)                    │
│  System · Balances (STR) · TransactionPayment · Sudo       │
│  ┌──────────────────┐  ┌───────────────────────────────┐   │
│  │  pallet-pqc      │  │  pallet-governance            │   │
│  │  chaves ML-DSA/  │  │  propostas + votação          │   │
│  │  ML-KEM, sessões │  │  ponderada em STR             │   │
│  └──────────────────┘  └───────────────────────────────┘   │
├────────────────────────────────────────────────────────────┤
│  PRIMITIVES (pqc-crypto — no_std / WASM)                   │
│  ML-DSA-65 (FIPS 204) · ML-KEM-768 (FIPS 203)              │
│  HybridSignature · HybridPublic · derivação de AccountId   │
└────────────────────────────────────────────────────────────┘
```

**Identidade do runtime:** `spec_name: "entangle"`, `spec_version: 100`. **Tempo de bloco:** 6 segundos. **Hash:** BLAKE2b-256 para blocos e derivação de contas. **Endereçamento:** `MultiAddress<AccountId32>`, prefixo SS58 42.

A camada de primitivas é um crate independente que concentra todo o material pós-quântico. Runtime e pallets consomem apenas essa abstração. A implementação subjacente dos algoritmos pode ser trocada sem alterar a lógica de negócio — *crypto-agility* em nível de biblioteca.

### 4.2 Desenho criptográfico

#### 4.2.1 Escolha dos algoritmos

| Algoritmo | Padrão | Categoria NIST | Papel na Entangle |
|---|---|---|---|
| **ML-DSA-65** | FIPS 204 [2] | 3 (~AES-192) | Assinatura de transações e provas on-chain |
| **ML-KEM-768** | FIPS 203 [3] | 3 (~AES-192) | Encapsulamento de chaves para sessões seguras |
| Sr25519 / Ed25519 / ECDSA | — | clássico | Compatibilidade legada (contas em migração) |

A categoria 3 foi escolhida, nos dois esquemas, como equilíbrio deliberado. ML-DSA-44 reduziria o overhead, com margem de segurança menor. ML-DSA-87 seria mais conservador, com assinaturas ~40% maiores. Para uma cadeia cujo diferencial é robustez de longo prazo, a categoria 3 (equivalente a AES-192) oferece margem confortável sem inviabilizar o throughput.

#### 4.2.2 Assinatura híbrida: o núcleo do modelo de migração

O modelo de migração da Entangle está expresso no sistema de tipos das primitivas: a `HybridSignature`.

```rust
// primitives/pqc-crypto/src/hybrid.rs
pub enum HybridSignature {
    Classic(MultiSignature),   // Sr25519 / Ed25519 / ECDSA — 64 bytes
    MlDsa65(MlDsaSignature),   // FIPS 204 — 3.309 bytes
}
```

Cada assinatura carrega, no próprio tipo, o esquema que a produziu, e uma assinatura de um esquema nunca valida contra chave de outro: o casamento de variantes do enum retorna falso para pares cruzados. Contas clássicas seguem operando com Sr25519 e todo o ferramental existente (Polkadot.js, subxt), enquanto o material pós-quântico entra na mesma cadeia, sem fork.

O escopo de cada fase é delimitado com precisão. Nesta fase, a **verificação ML-DSA on-chain opera em nível de aplicação**, via extrinsic `verify_signature` do `pallet-pqc`, que busca a chave pública de 1.952 bytes no armazenamento da própria cadeia (Seção 4.3). A **autenticação de origem de transações** permanece pelo caminho clássico, e a razão é estrutural, não circunstancial: a interface padrão de verificação de extrinsics do Substrate (trait `Verify`) entrega ao verificador apenas o identificador de conta de 32 bytes, um hash do qual a chave pública ML-DSA não é recuperável. Autenticar transações com ML-DSA exige, portanto, uma extensão de transação dedicada, que transporte ou referencie a chave registrada. Esse trabalho está programado para a Fase 3, junto com a migração do consenso.

> **Precisão terminológica.** "Híbrida", aqui, significa *coexistência de esquemas* (crypto-agility via enum). Não é *assinatura composta* no sentido criptográfico estrito, em que forjar uma única assinatura exigiria quebrar os dois esquemas ao mesmo tempo. A composição clássica+PQC é uma extensão natural do desenho (o enum comporta mais uma variante) e está no roadmap, alinhada às recomendações de transição do NIST. A distinção é feita no texto porque importa: são garantias diferentes.

#### 4.2.3 Contas pós-quânticas e proteção por hash

A identidade de conta estende o padrão Substrate às chaves ML-DSA:

```
AccountId32 = BLAKE2b-256( chave pública ML-DSA-65 [1.952 bytes] )
```

Uma consequência prática importante: até a exposição voluntária (o registro via `pallet-pqc`), a chave pública ML-DSA não aparece on-chain. Só o hash de 32 bytes, resistente a Grover com margem adequada. É a mesma proteção de endereços não reutilizados do Bitcoin (*hash-shielding*), mas aqui como propriedade padrão do modelo de contas.

As chaves secretas seguem a forma de *seed* dos padrões: 32 bytes (ML-DSA-65) e 64 bytes (ML-KEM-768). O custo de custódia da chave privada não cresce em relação ao mundo clássico.

#### 4.2.4 Tamanhos e overhead

| Métrica | Clássico (Sr25519) | PQC (ML-DSA-65) | Fator |
|---|---|---|---|
| Assinatura | 64 B | 3.309 B | ~52× |
| Chave pública | 32 B | 1.952 B | ~61× |
| Chave secreta (seed) | 32 B | 32 B | 1× |
| Ciphertext ML-KEM-768 | — | 1.088 B | — |

A mitigação é estrutural: o limite de bloco (`RuntimeBlockLength`) foi dimensionado em 8 MB, contra 5 MB do template Substrate, para acomodar extrinsics pós-quânticos, cujo envelope máximo estimado é a assinatura ML-DSA mais 256 bytes. Verificação em lote entra no roadmap como mitigação adicional.

### 4.3 O pallet PQC

O `pallet-pqc` gerencia o ciclo de vida do material pós-quântico on-chain.

**Armazenamento:** `PqcKeys` (conta → bundle com chave ML-DSA obrigatória, chave ML-KEM opcional e esquema ativo); `ValidatorPqcKeys` (chaves PQ de authorities, preparação do PoS); `Sessions` (sessões ML-KEM, armazenando um identificador público da sessão: o hash BLAKE2b-256 do ciphertext; o segredo compartilhado nunca toca o estado da cadeia); `NextSessionId`.

**Extrinsics:**

| Chamada | Função |
|---|---|
| `register_keys(ml_dsa_pk, ml_kem_pk?)` | Registra o bundle PQC; a conta vira pós-quântica. |
| `verify_signature(message, signature)` | Verificação ML-DSA-65 **on-chain** contra a chave registrada — primitiva de prova para aplicações. |
| `establish_session(responder, ciphertext)` | Âncora on-chain do handshake ML-KEM: valida o ciphertext contra a chave registrada do respondedor e grava a sessão; a decapsulação ocorre off-chain. |
| `remove_keys()` | Revogação do bundle. |
| `register_validator_keys(...)` | Registro de chaves PQ de authorities (Fase 3). |

**Fluxo de sessão ML-KEM:**

```
Iniciador                              Respondedor
    │  encapsulate(pk_KEM_respondedor)      │
    │────────── ciphertext (1.088 B) ──────►│
    │                                       │ decapsulate(sk, ct)
    │                                       │ → segredo compartilhado (32 B)
    │◄──────── SessionEstablished ──────────│
```

A decapsulação e a derivação do segredo de 32 bytes ocorrem off-chain, pelo respondedor, com segurança pós-quântica; a cadeia atua como âncora pública e verificável do handshake — registra quem iniciou, contra qual chave e com qual ciphertext. Isso serve de base para canais cifrados off-chain e derivação de chaves de aplicação.

### 4.4 Consenso

A Entangle opera com o par clássico do Substrate: **Aura** (produção determinística de blocos em slots de 6 s) e **GRANDPA** (finalidade). É *Proof of Authority*, adequado para a fase de prova de conceito, e assim declarado explicitamente. A Fase 3 do roadmap prevê a transição para **BABE + PoS** com staking em STR. O `pallet-pqc` já expõe `register_validator_keys` exatamente para isso: as chaves de sessão dos validadores migram para o regime pós-quântico junto com o consenso, eliminando as authorities clássicas como elo fraco quântico da finalidade.

### 4.5 O token Strand (STR)

O Strand é o ativo nativo da Entangle, com três funções: **taxas de transação** (via `pallet-transaction-payment`, com pesos por operação a calibrar formalmente na Fase 5), **governança** (Seção 4.6) e **staking** (Fase 3).

| Unidade | Valor em unidades base |
|---|---|
| 1 STR | 10¹² |
| 1 mSTR | 10⁹ |
| 1 µSTR | 10⁶ |

**Depósito existencial:** 1 mSTR. Contas abaixo desse saldo são removidas do estado: prevenção de *dust accounts*, que pesa mais numa cadeia cujo material de conta é volumoso.

> **Nota de transparência.** Nesta fase, o suprimento vem da gênese de desenvolvimento e a cadeia mantém `pallet-sudo` como chave administrativa. Não existe política monetária definitiva (suprimento máximo, emissão, distribuição). Isso é entregável da fase de tokenomics do roadmap, condicionado ao desenho do staking. Optou-se por não anunciar números de emissão que não existem no protocolo.

### 4.6 Governança on-chain

O `pallet-governance` implementa o ciclo completo de propostas com execução autônoma.

| Parâmetro | Valor no runtime |
|---|---|
| Depósito mínimo de proposta | 100 STR |
| Período de votação | 600 blocos (~1 h a 6 s/bloco) |
| Quórum mínimo (*turnout*) | 10% do suprimento |
| Aprovação mínima | 51% dos votos |

**Ciclo de vida:** `propose` (com depósito e a chamada a executar) → `vote` (aye/nay, peso proporcional ao saldo STR) → `close` (apura quórum e aprovação ao fim do período) → execução automática da chamada com origem Root em caso de aprovação → `cancel` disponível ao proponente antes do fechamento.

A execução com origem Root faz da governança o mecanismo canônico de evolução do protocolo, inclusive das **atualizações de runtime sem fork** (`set_code`). A migração criptográfica futura da própria Entangle pode ser deliberada e aplicada pela comunidade sem parar a cadeia.

> **Limitação reconhecida (Fase 2).** O peso de voto usa o saldo livre no momento do voto, sem lock e sem *conviction voting*. Isso permite reutilizar tokens entre votos em janelas curtas. O endurecimento está no roadmap: trava de saldo durante a votação, *conviction voting* e restrição progressiva da origem Root por *tracks* com limiares diferenciados. Optou-se por declarar a limitação explicitamente.

---

## 5. Resultados

### 5.1 Resultados obtidos

#### 5.1.1 Protocolo funcional (Fases 0–2)

O artefato central está implementado: node Substrate operacional (Aura + GRANDPA, 6 s); primitivas ML-DSA-65/ML-KEM-768 em crate `no_std`, compilável para o alvo WASM do runtime; tipo de assinatura híbrida definido nas primitivas; `pallet-pqc` completo (registro e revogação de chaves, verificação ML-DSA on-chain via `verify_signature`, âncora de sessões ML-KEM, chaves de validador); `pallet-governance` com votação ponderada e execução autônoma. Os dois pallets têm testes unitários. A base de código permanece em refatoração ativa; a consolidação do build da revisão corrente e a ligação da assinatura híbrida à verificação de origem de extrinsics são pendências de integração declaradas na Seção 5.2.

#### 5.1.2 Custos medidos: ECDSA × ML-DSA

**Nota metodológica.** Os números vêm de **duas campanhas em ambientes distintos** e não são comparáveis entre si, apenas *dentro* de cada campanha. Campanha A (jun/2026): Node.js v22, win32/x64, implementações Noble [9], mensagem de 32 bytes, médias por lote, resultados versionados em CSV/JSON. Campanha B (mai/2026): ambiente anterior (Node.js 18, hardware distinto), reportada no artigo do projeto. Medição em JavaScript não substitui benchmark nativo; o valor dela é a comparação relativa entre algoritmos no mesmo runtime. A referência de custo para o runtime Rust é ~3 ms por verificação ML-DSA-65 em CPU moderna, a ser substituída por benchmarks formais com `frame_benchmarking` na Fase 5.

**Campanha A (ms/operação, média):**

| Algoritmo | pk (B) | sig (B) | keygen | sign | verify |
|---|---|---|---|---|---|
| ECDSA secp256k1 | 33 | 64 | 0,29 | 0,31 | 2,34 |
| ML-DSA-44 | 1.312 | 2.420 | 0,85 | 3,75 | 0,88 |
| **ML-DSA-65** | **1.952** | **3.309** | **1,56** | **6,27** | **1,47** |
| ML-DSA-87 | 2.592 | 4.627 | 2,38 | 6,62 | 2,40 |

Na Campanha B, com hardware mais modesto, as proporções se mantêm (ECDSA: keygen 2,56 ms, verify 16,29 ms; ML-DSA proporcionalmente mais custoso em assinatura e tamanhos, competitivo em verificação).

#### 5.1.3 Validações complementares

- **Simulação de blocos pós-quânticos** (Anexo C): cadeia simulada em que cada transação carrega assinatura ML-DSA-65, verificada obrigatoriamente antes do cálculo da raiz de Merkle e do hash do bloco. Overhead típico por transação: 1.952 B de chave pública + 3.309 B de assinatura, coerente com as constantes do protocolo;
- **MVP de notarização com âncora blockchain** (Anexo B): protótipo funcional do padrão "âncora on-chain + prova pós-quântica off-chain", com assinatura ML-DSA-65 sobre o compromisso `keccak256` de documentos e registro imutável em contrato. O MVP exercita, em ambiente EVM, o mesmo modelo de prova que o `verify_signature` da Entangle oferece nativamente. Por contraste, evidencia o valor da verificação nativa.

### 5.2 Discussão

Três leituras saem dos números, e as três validam o desenho da Entangle:

1. **O custo dominante do PQC é espaço, não tempo.** A assinatura ML-DSA-65 é ~52× maior que a clássica; os tempos ficam em poucos milissegundos. Por isso as mitigações são estruturais ao *layout* (blocos de 8 MB, pesos por byte, *hash-shielding* de chaves) e não sacrificam o algoritmo.
2. **A verificação ML-DSA é competitiva.** Nos dois ambientes, o *verify* do ML-DSA-65 ficou na mesma ordem de grandeza do ECDSA — na Campanha A, ficou abaixo. Verificação é a operação que todos os nós executam a cada bloco; é ela que governa o throughput. E o número é favorável.
3. **A assinatura é gargalo do cliente, não da rede.** O *sign* de ~6 ms acontece uma vez, na carteira do usuário. Imperceptível em UX.

**Limitações declaradas.** A regra adotada neste trabalho foi afirmar exatamente o que está implementado, nem um passo além. Declara-se, portanto: (i) a autenticação de origem de transações permanece clássica: a verificação ML-DSA on-chain opera em nível de aplicação (`verify_signature`), e a integração da assinatura híbrida ao caminho de extrinsics exige extensão de transação dedicada, programada para a Fase 3; (ii) consenso e transporte de rede ainda usam criptografia clássica, e a migração das chaves de validador também é a Fase 3; (iii) contas clássicas continuam, por definição, vulneráveis a um adversário quântico futuro — a coexistência é o meio da migração, não o fim; (iv) os pesos de dispatch dos pallets são estimativas fixadas em código e subcalibradas frente ao custo medido (~30× no caso de `verify_signature`: ~0,1 ms declarado contra ~3 ms reais), vetor de subprecificação a corrigir na Fase 5 com `frame_benchmarking`; (v) o registro de sessão ML-KEM ancora o handshake e valida formato; a decapsulação é off-chain por desenho; (vi) a governança não trava saldo durante a votação (Seção 4.6); (vii) os crates de reticulados são recentes, sem auditoria formal completa, e implementação em software está sujeita a canais laterais; (viii) é prova de conceito em refatoração ativa: `pallet-sudo` ativo, gênese de desenvolvimento, sem auditoria de segurança externa.

---

## 6. Roadmap e Impacto Esperado

### 6.1 Roadmap do protocolo

| Fase | Escopo | Status |
|---|---|---|
| **0 — Fundação** | Node + runtime Substrate/FRAME, token STR, Aura + GRANDPA | ✅ Concluída |
| **1 — Primitivas PQC** | ML-DSA-65, ML-KEM-768, tipo de assinatura híbrida, `pallet-pqc` | ✅ Concluída |
| **2 — Governança** | Propostas e votação ponderada em STR; chaves PQ de validadores | ✅ Concluída |
| **3 — Consenso PQ-ready** | BABE + PoS; staking em STR; chaves de validadores ML-DSA; origem de transação ML-DSA (extensão dedicada) | 🔜 |
| **4 — Programabilidade** | Smart contracts ink! (EVM opcional) com acesso às primitivas PQC | 🔜 |
| **5 — Calibração** | Benchmarks formais `frame_benchmarking`; pesos medidos; testnet pública | 🔜 |
| **6 — Otimização** | Batch verification; assinatura composta clássica+PQ; tokenomics final; auditoria | 🔜 |

### 6.2 Impacto esperado

**Científico:** uma arquitetura de referência aberta e reproduzível para integrar os padrões FIPS 203/204 a runtimes WASM de blockchains, com medições e limitações documentadas. **Tecnológico:** a demonstração de que coexistência de esquemas no sistema de tipos do runtime é um mecanismo de migração viável, e superior ao *retrofit*, aplicável por outras cadeias Substrate. **Econômico e social:** contribuição à preservação de longo prazo de ativos e registros digitais frente ao risco quântico, um problema cuja janela de resposta se fecha antes de a ameaça se materializar.

---

## 7. Conclusão e Trabalhos Futuros

### 7.1 Conclusão

A transição pós-quântica das blockchains é um processo de anos, e precisa começar enquanto a criptografia clássica ainda é segura. O trabalho partiu de um problema, a vulnerabilidade quântica das curvas elípticas que sustentam as redes atuais, e de uma lacuna: a ausência de uma arquitetura nativamente pós-quântica com migração híbrida embutida no protocolo. A resposta é a Entangle: uma Layer 1 soberana em que FIPS 203 e FIPS 204 são primitivas de primeira classe, a coexistência clássico/pós-quântico está desenhada no sistema de tipos do protocolo e a evolução criptográfica é deliberável por governança, sem fork.

A metodologia experimental (implementação em fases, duas campanhas de benchmark e validações complementares) sustentou as conclusões centrais: integrar ML-DSA-65 e ML-KEM-768 a um runtime WASM é viável hoje; o custo dominante do pós-quântico é espaço, não tempo; a verificação ML-DSA é competitiva com a clássica; e as mitigações estruturais adotadas absorvem o overhead. O que foi proposto foi entregue: as Fases 0–2 do protocolo, funcionais, medidas e com as limitações declaradas — sem afirmar um passo além do que está implementado.

### 7.2 Trabalhos futuros

- **Consenso e origem de transação pós-quânticos (Fase 3):** BABE + PoS com staking em STR, chaves de sessão de validadores ML-DSA e extensão de transação dedicada para autenticar origens com ML-DSA, estendendo a garantia pós-quântica às transações e à finalidade de blocos;
- **Programabilidade (Fase 4):** contratos ink! com acesso nativo às primitivas PQC via `pallet-pqc`;
- **Calibração de pesos (Fase 5):** benchmarks formais com `frame_benchmarking` em hardware de referência, substituindo as estimativas atuais e fundamentando a precificação de transações;
- **Assinatura composta e batch verification (Fase 6):** variante que exige quebra simultânea dos esquemas clássico e pós-quântico, e verificação em lote no import de blocos;
- **Endurecimento da governança:** trava de saldo, *conviction voting* e origens com *tracks* diferenciados;
- **Tokenomics do STR:** política monetária definitiva, condicionada ao desenho do staking;
- **Comparação NIST × PQMagic:** extensão do benchmark aos algoritmos do ecossistema chinês, com análise técnica e geopolítica;
- **Auditoria e testnet pública:** auditoria de segurança externa das primitivas e dos pallets antes de qualquer implantação pública.

---

## Referências Bibliográficas

[1] P. W. Shor, "Algorithms for quantum computation: discrete logarithms and factoring," in *Proceedings 35th Annual Symposium on Foundations of Computer Science*, Santa Fe, NM, USA, 1994, pp. 124–134.

[2] National Institute of Standards and Technology, *Module-Lattice-Based Digital Signature Standard*, FIPS 204, Gaithersburg, MD, USA, Aug. 2024.

[3] National Institute of Standards and Technology, *Module-Lattice-Based Key-Encapsulation Mechanism Standard*, FIPS 203, Gaithersburg, MD, USA, Aug. 2024.

[4] National Institute of Standards and Technology, *Stateless Hash-Based Digital Signature Standard*, FIPS 205, Gaithersburg, MD, USA, Aug. 2024.

[5] Ripple Labs, "Relatório técnico sobre a implantação de CRYSTALS-Dilithium no XRP Ledger," 2025.

[6] BTQ Technologies, "Documento técnico da testnet de blockchain resistente a ataques quânticos," 2026.

[7] Parity Technologies, "Substrate — Polkadot SDK documentation." [Online]. Disponível em: https://docs.substrate.io

[8] RustCrypto, "Crates `ml-dsa` e `ml-kem` — implementações puras em Rust dos padrões FIPS 204/203." [Online]. Disponível em: https://github.com/RustCrypto

[9] P. Miller et al., "`@noble/post-quantum` e `@noble/secp256k1` — implementações JavaScript auditáveis de primitivas criptográficas." [Online]. Disponível em: https://github.com/paulmillr

[10] S. Nakamoto, "Bitcoin: A Peer-to-Peer Electronic Cash System," 2008.

---

## Anexos

### Anexo A — Reprodutibilidade

Todos os componentes do trabalho são reproduzíveis a partir do código-fonte do projeto.

**Cadeia Entangle:** toolchain Rust fixado em `rust-toolchain.toml`; compilação com `cargo build --release`; suíte de testes dos pallets com `cargo test`; execução do node em modo de desenvolvimento.

**Benchmarks:** `npm install` e `npm run bench` na raiz do projeto, o que gera `benchmarks/results/latest.csv` e `latest.json`, com metadados de ambiente (versão do Node, sistema operacional, modelo de CPU).

**Simulação de cadeia:** `npm run experiment:sim`, que gera `experiments/results/latest-chain.json` com a cadeia completa e o overhead por transação.

### Anexo B — Validação complementar: MVP de notarização com âncora blockchain

Protótipo funcional (Hardhat + Solidity + React) do padrão "âncora on-chain + prova pós-quântica". O fluxo: o usuário associa um documento; o sistema calcula o compromisso `keccak256` dos bytes; produz uma assinatura ML-DSA-65 sobre esse compromisso; e registra na cadeia local a chave pública e a assinatura como bytes opacos (contrato `PQCNotaryRegistry`, com unicidade por hash, evento de notarização e erros tipados).

A verificação criptográfica é deliberadamente **off-chain**, no navegador — verificar reticulados na EVM seria impraticável em gas. Essa restrição evidencia, por contraste, o valor da verificação ML-DSA **nativa** que a Entangle oferece via `verify_signature`. O MVP também expõe o custo prático de calldata volumoso (chave de 1.952 B + assinatura de 3.309 B) em transações reais com carteira MetaMask.

### Anexo C — Validação complementar: simulação de blocos pós-quânticos

Simulador (Node.js) de uma cadeia em que cada transação contém assinatura ML-DSA-65 sobre um hash de documento de 32 bytes, com verificação **obrigatória** antes do cálculo da raiz de Merkle e do hash de bloco. O invariante modelado é o central de uma cadeia pós-quântica: nenhuma transação inválida chega ao encadeamento. A saída registra a cadeia completa (gênese + blocos), os hashes encadeados e o overhead por transação, coerente com as constantes do protocolo Entangle.

### Anexo D — Constantes criptográficas do protocolo

| Constante | Valor |
|---|---|
| Chave pública ML-DSA-65 | 1.952 B |
| Seed secreta ML-DSA-65 | 32 B |
| Assinatura ML-DSA-65 | 3.309 B |
| Chave pública ML-KEM-768 | 1.184 B |
| Seed secreta ML-KEM-768 | 64 B |
| Ciphertext ML-KEM-768 | 1.088 B |
| Segredo compartilhado ML-KEM | 32 B |
| Envelope máximo de extrinsic PQ | assinatura + 256 B |

---

## Aviso Legal

Este documento é um trabalho acadêmico de conclusão de curso e descreve uma prova de conceito em desenvolvimento. Não constitui oferta de venda de tokens, solicitação de investimento nem promessa de funcionalidade futura. O token Strand (STR) descrito neste documento existe apenas em ambiente de teste e não possui valor econômico. Todo o conteúdo é fornecido "no estado em que se encontra", sem garantias de qualquer natureza.
