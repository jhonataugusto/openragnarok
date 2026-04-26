---
phase: 01
slug: reproducao-e-baseline-manual
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-04-26
---

# Phase 01 - Validation Strategy

> Contrato de validacao para a fase de reproducao e baseline manual.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Validacao manual + smoke operacional via Docker |
| **Config file** | `docker-compose.yml`, `play.bat` |
| **Quick run command** | `docker compose ps` |
| **Full suite command** | N/A - fase manual, sem mudanca de codigo |
| **Estimated runtime** | 5-15 minutos, dependendo de login e reproducao |

---

## Sampling Rate

- **After every task commit:** Conferir artefato criado e, quando aplicavel, `docker compose ps`
- **After every plan wave:** Confirmar que REPR-01, REPR-02 e REPR-03 possuem evidencia escrita
- **Before `$gsd-verify-work`:** `01-BASELINE.md` deve existir e registrar ambiente, passos e resultado visual
- **Max feedback latency:** Manual/imediato durante a sessao

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 01-01-01 | 01 | 1 | REPR-01 | T-01-01 | Ambiente local tratado como dev-only | smoke/manual | `docker compose ps` | sim | pending |
| 01-01-02 | 01 | 1 | REPR-02 | - | N/A | manual gameplay | N/A | N/A manual | pending |
| 01-01-03 | 01 | 1 | REPR-03 | - | N/A | doc review | N/A | `01-BASELINE.md` | pending |

---

## Wave 0 Requirements

Existing infrastructure covers the phase requirements. Nenhum framework novo deve ser instalado para esta fase.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Abrir o cliente e reproduzir `@kill -> Respawn` | REPR-02 | O bug e visual/interativo e nao ha harness E2E | Usar `play.bat`, entrar com conta local, executar `@kill`, clicar Respawn e registrar pose, janela, HP e movimento |
| Registrar baseline de reproducao | REPR-03 | O resultado depende de observacao visual humana | Criar `01-BASELINE.md` com ambiente, conta/personagem, passos e resultado observado |

---

## Validation Sign-Off

- [x] All tasks have manual verify or existing infrastructure
- [x] Sampling continuity: no code-task sequence applies in this phase
- [x] Wave 0 covers all missing references
- [x] No watch-mode flags
- [x] Feedback latency acceptable for manual baseline
- [x] `nyquist_compliant: true` set in frontmatter

**Approval:** approved 2026-04-26
