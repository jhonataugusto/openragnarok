---
phase: 01-reproducao-e-baseline-manual
plan: 01
subsystem: testing
tags: [ragnarok, korangar, rathena, respawn, baseline, manual]

requires: []
provides:
  - Baseline manual do bug de respawn atual
  - Evidencia de ambiente local pronto
  - Resultado visual observado para `@kill -> Respawn`
affects:
  - Phase 2: Evidencia de Pacotes, Eventos e Entidade

tech-stack:
  added: []
  patterns:
    - Baseline manual em `.planning/phases/*/01-BASELINE.md`
    - Smoke operacional via Docker Compose e portas locais

key-files:
  created:
    - .planning/phases/01-reproducao-e-baseline-manual/01-BASELINE.md
  modified: []

key-decisions:
  - "Fase 1 confirma o bug atual como reproduzido: apos Respawn, o player continua morto/deitado."
  - "A janela Respawn fecha, mas HP nao restaura e movimento nao funciona no estado observado."
  - "Nenhum codigo, configuracao Docker, assets, launcher ou servidor foi alterado nesta fase."

patterns-established:
  - "Registrar resultado visual separado de janela, HP e movimento para evitar falso positivo."

requirements-completed:
  - REPR-01
  - REPR-02
  - REPR-03

duration: 3h 22m
completed: 2026-04-27
---

# Phase 01 Plan 01: Reproducao e Baseline Manual Summary

**Baseline manual confirmou que o bug de respawn ainda reproduz no cliente atual: apos Respawn, o player permanece morto/deitado, sem HP restaurado e sem movimento.**

## Performance

- **Duration:** 3h 22m
- **Started:** 2026-04-26T22:29:41-03:00
- **Completed:** 2026-04-27T01:52:02-03:00
- **Tasks:** 3
- **Files modified:** 2

## Accomplishments

- Confirmado que Docker, rAthena, MariaDB, portas locais, `play.bat`, executavel Korangar e GRFs estavam prontos para o fluxo local.
- Registrado o fluxo manual `@kill -> Respawn` no cliente real antes de qualquer fix permanente.
- Confirmado por checkpoint humano que o bug nao foi resolvido: player fica morto/deitado apos Respawn, janela fecha, HP nao restaura e movimento nao funciona.
- Criado baseline em `.planning/phases/01-reproducao-e-baseline-manual/01-BASELINE.md`.

## Task Commits

1. **Task 1: Confirmar ambiente local reproduzivel** - `8f34431` (`docs(01-01): record respawn environment baseline`)
2. **Task 2: Reproduzir `@kill -> Respawn` no cliente real** - `efd596d` (`docs(01-01): record respawn reproduction result`)
3. **Task 3: Fechar baseline manual e checar escopo** - included in plan metadata commit

## Files Created/Modified

- `.planning/phases/01-reproducao-e-baseline-manual/01-BASELINE.md` - baseline manual com ambiente, comandos, smoke operacional, fluxo manual e resultado observado.
- `.planning/phases/01-reproducao-e-baseline-manual/01-01-SUMMARY.md` - resumo GSD da execucao da fase.

## Decisions Made

- Conta/personagem nao foi informado no checkpoint; o baseline registra isso explicitamente em vez de inferir.
- A observacao humana e suficiente para REPR-02 porque o bug atual foi observado no cliente real.
- O estado final observado e mais severo que parte do relato historico: alem da pose morta/deitada, HP nao restaurou e movimento nao funcionou.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- O usuario informou duas vezes "nao foi resolvido" antes de detalhar os criterios visuais. Isso foi tratado como checkpoint humano incompleto ate os campos essenciais serem fornecidos.
- Conta/personagem usado nao foi informado; registrado como "Nao informado" no baseline.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

Phase 2 esta pronta para investigar evidencia de pacotes, eventos e entidade. A baseline confirma:

- Ambiente local operacional.
- Bug ainda reproduz.
- Janela Respawn fecha.
- Player permanece morto/deitado.
- HP nao restaura.
- Movimento nao funciona.

---
*Phase: 01-reproducao-e-baseline-manual*
*Completed: 2026-04-27*
