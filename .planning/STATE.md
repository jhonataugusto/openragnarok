---
gsd_state_version: 1.0
milestone: v1.0
milestone_name: NPC Cinematic Dialog
status: discovery
stopped_at: Respawn diagnostic closed; brainstorming NPC cinematic dialog milestone.
last_updated: "2026-04-28T00:00:00-03:00"
last_activity: 2026-04-28 - Respawn bug closed by client packet/event registration discovery; new feature milestone selected.
progress:
  total_phases: 0
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
  percent: 0
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-28)

**Core value:** Dialogos com NPCs podem ganhar apresentacao cinematica moderna sem remover o dialogo classico atual.
**Current focus:** Brainstorming/spec do milestone NPC Cinematic Dialog

## Current Position

Phase: TBD
Plan: TBD
Status: Discovery
Last activity: 2026-04-28 - Usuario encerrou diagnostico do respawn e escolheu iniciar novo milestone de feature.

Progress: [----------] 0%

## Performance Metrics

**Velocity:**

- Total plans completed: 1
- Average duration: 3h 22m
- Total execution time: 3h 22m

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Reproducao e Baseline Manual | 1 | 1 | 3h 22m |

**Recent Trend:**

- Last 5 plans: 1 complete
- Trend: N/A

*Updated after each plan completion*

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- Respawn diagnostic closed: causa informada foi pacote desconhecido nao registrado como evento no cliente.
- Novo milestone selecionado: dialogo cinematico de NPC no cliente Korangar.
- A feature deve ser opcional via Interface Settings.
- Quando ativa, a feature deve se aplicar a todos os dialogos de NPC.
- Durante dialogo cinematico, movimento e controle manual de camera devem ficar temporariamente travados.
- A camera cinematica deve ser dinamica, sem cortes, e configuravel por toggle simples.
- O texto deve aparecer em typewriter, com acao para revelar tudo antes de avancar.
- Mouse esquerdo, Enter e Espaco devem revelar/avancar o dialogo cinematico.
- Som por letra deve ter toggle simples ligado/desligado, usar volume de efeitos e variar pitch automaticamente.
- Opcoes de resposta devem aparecer como baloes empilhados acima da caixa de dialogo, centralizados perto da parte inferior.
- UI cinematica deve substituir visualmente a janela classica quando o modo estiver ativo.
- Abordagem aprovada: UI cinematica nova sobre os mesmos eventos de dialogo, preservando a janela classica intacta como fallback.

### Pending Todos

None yet.

### Blockers/Concerns

- A feature envolve UI, camera, input e audio; precisa de design antes de implementacao.
- Deve preservar o dialogo classico atual como fallback.
- Pitch por letra pode exigir extensao do `korangar-audio`, que hoje expoe `play_sound_effect` sem parametro publico de pitch.

## Deferred Items

| Category | Item | Status | Deferred At |
|----------|------|--------|-------------|
| Testes | Smoke/E2E automatizado de login, morte e respawn | Future | Roadmap v1 |
| Observabilidade | Tracing permanente ou replay de pacotes | Future | Roadmap v1 |
| Arquitetura | Refatorar lifecycle/handlers fora de `main.rs` | Future | Roadmap v1 |

## Session Continuity

Last session: 2026-04-28
Stopped at: NPC cinematic dialog brainstorming.
Resume file: None
