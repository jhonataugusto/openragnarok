# Requirements: Ragnarok Client Modernization

**Defined:** 2026-04-26
**Core Value:** Evoluir o cliente Korangar com melhorias comprovadas, preservando compatibilidade e comportamento existente quando necessario.

## Closed Respawn Diagnostic

O milestone de respawn foi encerrado em 2026-04-28 por informacao do usuario: a causa era falta de registro/tratamento de um pacote desconhecido como evento no cliente. As fases restantes do roadmap antigo foram superseded por essa descoberta e nao devem guiar trabalho novo.

## Active Milestone: NPC Cinematic Dialog

Requirements serao definidos apos a spec de design. Decisoes ja aprovadas:

- [x] A feature deve ser ativada por uma configuracao global em Interface Settings.
- [x] O dialogo classico existente deve permanecer disponivel quando a opcao estiver desligada ou como fallback.
- [x] Quando ativa, a feature deve aplicar o modo cinematico a todos os dialogos de NPC.
- [x] Durante o dialogo cinematico, movimento e controle manual de camera devem ficar temporariamente travados.
- [x] A camera cinematica deve ser dinamica, entrar sem cortes bruscos e ter configuracao simples de ligar/desligar.
- [x] O texto cinematico deve usar typewriter, com clique/tecla para revelar a fala inteira antes de avancar.
- [x] Mouse esquerdo, Enter e Espaco devem revelar/avancar texto no dialogo cinematico.
- [x] O som por letra deve ter toggle simples ligado/desligado, usando volume de efeitos e pitch automatico.
- [x] As opcoes de resposta devem aparecer como baloes empilhados acima da caixa de dialogo, centralizados perto da parte inferior.
- [x] Quando o modo cinematico estiver ativo, a UI cinematica deve substituir visualmente a janela classica de dialogo.
- [x] A UI cinematica deve ser um caminho novo sobre os mesmos eventos/protocolo, preservando a `DialogWindow` classica como fallback.
- [x] O typewriter deve iniciar em cerca de 35 caracteres por segundo, com som apenas para caracteres visiveis nao-espaco.
- [x] O pitch alvo do som por letra deve variar aproximadamente entre 0.94x e 1.06x.
- [x] O dialogo classico deve ser fallback quando player/NPC ou estado cinematico nao puderem ser resolvidos com seguranca.

## v1 Requirements

Historical requirements for the initial respawn fix milestone. Preserved for context; the milestone is closed by external diagnostic resolution.

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
