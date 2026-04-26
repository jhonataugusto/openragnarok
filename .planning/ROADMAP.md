# Roadmap: Ragnarok Respawn Fix

## Overview

Este roadmap entrega uma correcao minima comprovada para o bug de respawn visual no workspace Korangar + rAthena. O caminho segue a ordem causal do problema: reproduzir o bug atual, coletar evidencia de pacotes/eventos/entidade, aplicar o menor fix no ponto comprovado, validar manualmente o fluxo `@kill -> Respawn` e registrar a causa para continuidade.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Reproducao e Baseline Manual** - Ambiente local e bug atual confirmados antes de qualquer mudanca permanente.
- [ ] **Phase 2: Evidencia de Pacotes, Eventos e Entidade** - Fluxo real de respawn capturado e causa provavel classificada.
- [ ] **Phase 3: Fix Minimo no Ponto Causal** - Menor correcao segura aplicada ao cliente, servidor ou integracao conforme evidencia.
- [ ] **Phase 4: Verificacao Manual Direcionada** - Fluxo corrigido validado manualmente com estado visual vivo, movimento, HP e janela fechada.
- [ ] **Phase 5: Limpeza e Registro Final** - Instrumentacao temporaria removida e decisao tecnica registrada para proximas sessoes.

## Phase Details

### Phase 1: Reproducao e Baseline Manual
**Goal**: O desenvolvedor consegue reproduzir e documentar o bug atual no ambiente local antes de alterar comportamento permanente.
**Depends on**: Nothing (first phase)
**Requirements**: REPR-01, REPR-02, REPR-03
**Success Criteria** (what must be TRUE):
  1. O ambiente local necessario para Korangar, rAthena, Docker e MariaDB inicia o suficiente para executar o fluxo de respawn.
  2. O bug atual e reproduzido com `@kill -> Respawn` antes de qualquer fix permanente.
  3. Um roteiro de reproducao registra passos, ambiente, comando usado e resultado visual observado.
**Plans**: TBD

### Phase 2: Evidencia de Pacotes, Eventos e Entidade
**Goal**: A causa provavel do bug fica apoiada por evidencia objetiva do fluxo de pacotes, eventos, entidade local e estado visual final.
**Depends on**: Phase 1
**Requirements**: EVID-01, EVID-02, EVID-03, EVID-04
**Success Criteria** (what must be TRUE):
  1. A sequencia relevante de `@kill -> Respawn` mostra se ocorreram `ResurrectPlayer`, `ChangeMap`, `RemoveEntity`, status/HP ou combinacao deles.
  2. O `entity_id` do player local fica identificado durante morte e respawn.
  3. O estado final da entidade local fica registrado, incluindo estado visual morto, animacao morta e janela `Respawn` aberta ou fechada.
  4. A causa provavel fica classificada como cliente Korangar, servidor rAthena ou integracao/protocolo, com evidencia suficiente para orientar o fix.
**Plans**: TBD
**UI hint**: yes

### Phase 3: Fix Minimo no Ponto Causal
**Goal**: O fluxo de respawn confirmado tira o player local do estado visual morto e fecha a janela de respawn usando a menor alteracao segura.
**Depends on**: Phase 2
**Requirements**: FIX-01, FIX-02, FIX-03, FIX-04
**Success Criteria** (what must be TRUE):
  1. A mudanca permanente fica limitada ao menor ponto causal comprovado, sem refatoracao ampla de `korangar/korangar/src/main.rs`.
  2. O fluxo corrigido atualiza a entidade do player local por identidade confirmada, sem depender cegamente da primeira entidade da lista.
  3. O respawn confirmado remove o estado visual morto do player local.
  4. A janela `Respawn` fecha e nao reabre indevidamente depois do respawn confirmado.
**Plans**: TBD
**UI hint**: yes

### Phase 4: Verificacao Manual Direcionada
**Goal**: A correcao e comprovada no jogo real pelo fluxo manual `@kill -> Respawn`, nao apenas por logs ou HP/movimento.
**Depends on**: Phase 3
**Requirements**: VERI-01, VERI-02, VERI-03, VERI-04
**Success Criteria** (what must be TRUE):
  1. Depois do fix, `@kill -> Respawn` termina com o player visualmente em pe/vivo.
  2. Depois do fix, o player respawnado consegue se mover e aparece com HP restaurado.
  3. Depois do fix, a janela `Respawn` nao permanece aberta no estado final.
  4. O resultado antes/depois e os arquivos alterados ficam registrados em artefato de verificacao.
**Plans**: TBD
**UI hint**: yes

### Phase 5: Limpeza e Registro Final
**Goal**: A investigacao fecha sem ruido temporario permanente e com a causa/decisao documentada para retomada futura.
**Depends on**: Phase 4
**Requirements**: REG-01, REG-02
**Success Criteria** (what must be TRUE):
  1. Logs e instrumentacao temporaria usados na investigacao sao removidos ou deixam de gerar ruido permanente.
  2. A causa provavel, a decisao cliente/servidor e o criterio de aceite final ficam documentados para a proxima sessao.
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Reproducao e Baseline Manual | 0/TBD | Not started | - |
| 2. Evidencia de Pacotes, Eventos e Entidade | 0/TBD | Not started | - |
| 3. Fix Minimo no Ponto Causal | 0/TBD | Not started | - |
| 4. Verificacao Manual Direcionada | 0/TBD | Not started | - |
| 5. Limpeza e Registro Final | 0/TBD | Not started | - |

