---
phase: 01-reproducao-e-baseline-manual
status: passed
verified: 2026-04-27
source:
  - .planning/phases/01-reproducao-e-baseline-manual/01-BASELINE.md
  - .planning/phases/01-reproducao-e-baseline-manual/01-01-SUMMARY.md
---

# Phase 01 Verification

## Verdict

PASSED

## Goal

O desenvolvedor consegue reproduzir e documentar o bug atual no ambiente local antes de alterar comportamento permanente.

## Requirement Coverage

| Requirement | Status | Evidence |
|-------------|--------|----------|
| REPR-01 | Passed | Docker/rAthena/MariaDB, portas locais, `play.bat`, executavel Korangar e GRFs registrados em `01-BASELINE.md`. |
| REPR-02 | Passed | Checkpoint humano confirmou que apos `@kill -> Respawn` o player permanece morto/deitado. |
| REPR-03 | Passed | `01-BASELINE.md` registra ambiente, comandos, fluxo manual e resultado observado. |

## Observed Baseline

- Janela Respawn fechou.
- Player permaneceu morto/deitado.
- HP nao restaurou.
- Movimento nao funcionou.
- Conta/personagem nao foram informados no checkpoint.

## Notes for Phase 2

Phase 2 deve capturar evidencia objetiva de pacotes/eventos/entidade para explicar por que o estado final permanece morto, incluindo a diferenca entre janela fechada, HP nao restaurado e movimento indisponivel.
