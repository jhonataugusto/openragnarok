# AGENTS.md

## Idioma

Responda em portugues do Brasil (pt-BR) em todos os fluxos GSD deste projeto, incluindo perguntas, planos, documentos e resumos finais.

## Projeto

Projeto: Ragnarok Respawn Fix

Valor central: depois de `@kill -> Respawn`, o player deve voltar visualmente vivo, com janela de respawn fechada, sem depender de tentativa cega ou workaround nao comprovado.

## Contexto Tecnico

- Cliente Korangar em Rust: `korangar/korangar/src/main.rs`, `korangar/korangar/src/world/entity/mod.rs`, `korangar/korangar-networking/src/lib.rs`, `korangar/ragnarok-packets/src/lib.rs`
- Servidor rAthena em C++: `rathena-master/src/map/pc.cpp`, `rathena-master/src/map/clif.cpp`
- Infra local: `docker-compose.yml`, `docker/entrypoint.sh`, `docker/import/`
- Compatibilidade: manter `PACKETVER=20220406`

## Regras de Trabalho

- Use `.planning/PROJECT.md`, `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md` e `.planning/STATE.md` como fonte principal de contexto GSD.
- Consulte `.planning/codebase/` antes de mexer em arquitetura, stack, testes ou areas frageis.
- Para este milestone, prefira correcao minima comprovada a refatoracao ampla.
- Nao altere `rathena-master/` preventivamente; mude o servidor somente se a evidencia provar que ele e o ponto causal.
- Nao dependa cegamente de `entities()[0]` para identificar o player local sem comprovar a identidade no fluxo observado.
- Mantenha logs/instrumentacao temporaria removiveis ou claramente isolados.

## Proximo Passo

Fase atual: Phase 1 - Reproducao e Baseline Manual.

Comando recomendado: `$gsd-plan-phase 1`
