# Requirements: Ragnarok Respawn Fix

**Defined:** 2026-04-26
**Core Value:** Depois de `@kill -> Respawn`, o player deve voltar visualmente vivo, com janela de respawn fechada, sem depender de tentativa cega ou workaround nao comprovado.

## v1 Requirements

Requirements for the initial fix milestone. Each maps to exactly one roadmap phase.

### Reproducao

- [x] **REPR-01**: O desenvolvedor consegue iniciar o ambiente local necessario para reproduzir o fluxo de respawn com Korangar e rAthena.
- [x] **REPR-02**: O desenvolvedor consegue reproduzir o bug atual usando o fluxo `@kill -> Respawn` antes de alterar comportamento permanente.
- [x] **REPR-03**: O roteiro de reproducao registra passos, ambiente, comando usado e resultado visual observado.

### Evidencia

- [ ] **EVID-01**: O desenvolvedor consegue registrar a sequencia relevante de eventos/pacotes durante `@kill -> Respawn`, incluindo se ocorre `ResurrectPlayer`, `ChangeMap`, `RemoveEntity`, status/HP ou combinacao deles.
- [ ] **EVID-02**: O desenvolvedor consegue identificar qual `entity_id` corresponde ao player local durante morte e respawn.
- [ ] **EVID-03**: O desenvolvedor consegue registrar se a entidade local termina o fluxo com estado visual morto, estado de animacao morto ou janela `Respawn` aberta.
- [ ] **EVID-04**: A causa provavel do bug fica classificada como cliente Korangar, servidor rAthena ou integracao/protocolo, com evidencia suficiente para orientar a correcao.

### Correcao

- [ ] **FIX-01**: A correcao altera o menor ponto causal comprovado, evitando refatoracao ampla de `korangar/korangar/src/main.rs`.
- [ ] **FIX-02**: A correcao atualiza somente a entidade do player local no fluxo de respawn, sem depender cegamente de uma entidade errada.
- [ ] **FIX-03**: A correcao garante que o fluxo confirmado de respawn tira o player local do estado visual morto.
- [ ] **FIX-04**: A correcao garante que a janela `Respawn` fecha e nao reabre indevidamente depois do respawn confirmado.

### Verificacao Manual

- [ ] **VERI-01**: Depois da correcao, o fluxo `@kill -> Respawn` termina com o player visualmente em pe/vivo.
- [ ] **VERI-02**: Depois da correcao, o player respawnado conserva movimento funcional e HP restaurado.
- [ ] **VERI-03**: Depois da correcao, a janela `Respawn` nao permanece aberta no estado final.
- [ ] **VERI-04**: A verificacao final registra o resultado antes/depois e os arquivos alterados.

### Registro

- [ ] **REG-01**: Logs ou instrumentacao temporaria usados na investigacao sao removidos ou deixam de gerar ruido permanente.
- [ ] **REG-02**: A causa provavel, a decisao cliente/servidor e o criterio de aceite final ficam documentados para a proxima sessao.

## v2 Requirements

Deferred to future release. Tracked but not in the current roadmap.

### Testes e Observabilidade

- **TEST-01**: Criar smoke test automatizado para login, entrada no mapa, morte e respawn.
- **TEST-02**: Criar tracing permanente ou replay de pacotes para investigacoes futuras.
- **TEST-03**: Criar teste unitario de lifecycle se a correcao extrair helper puro e estavel.

### Arquitetura

- **ARCH-01**: Refatorar handlers de morte, respawn e mudanca de mapa para fora do hub principal de `main.rs`.
- **ARCH-02**: Criar identidade de player local mais robusta do que a suposicao de `entities()[0]`.

## Out of Scope

Explicitly excluded from v1.

| Feature | Reason |
|---------|--------|
| Corrigir login-server disconnect | Bug separado e nao necessario para a prova minima do respawn. |
| Criar suite E2E completa | Alto custo e sem harness atual; v1 exige verificacao manual objetiva. |
| Refatorar `main.rs` amplamente | Alto risco de regressao em uma area fragil; v1 deve ser localizado. |
| Resolver todos os pacotes nao implementados | Escopo amplo; corrigir apenas o que for causal para respawn. |
| Alterar rAthena preventivamente | Servidor so deve mudar se a evidencia provar que esse e o ponto causal. |
| Mudar `PACKETVER=20220406` | Quebraria a comparabilidade com o ambiente onde o bug foi observado. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| REPR-01 | Phase 1 | Complete |
| REPR-02 | Phase 1 | Complete |
| REPR-03 | Phase 1 | Complete |
| EVID-01 | Phase 2 | Pending |
| EVID-02 | Phase 2 | Pending |
| EVID-03 | Phase 2 | Pending |
| EVID-04 | Phase 2 | Pending |
| FIX-01 | Phase 3 | Pending |
| FIX-02 | Phase 3 | Pending |
| FIX-03 | Phase 3 | Pending |
| FIX-04 | Phase 3 | Pending |
| VERI-01 | Phase 4 | Pending |
| VERI-02 | Phase 4 | Pending |
| VERI-03 | Phase 4 | Pending |
| VERI-04 | Phase 4 | Pending |
| REG-01 | Phase 5 | Pending |
| REG-02 | Phase 5 | Pending |

**Coverage:**
- v1 requirements: 17 total
- Mapped to phases: 17
- Unmapped: 0

---
*Requirements defined: 2026-04-26*
*Last updated: 2026-04-27 after Phase 1 execution*
