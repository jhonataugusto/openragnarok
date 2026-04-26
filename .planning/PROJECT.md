# Ragnarok Respawn Fix

## What This Is

Este projeto estabiliza um bug especifico no workspace local Ragnarok/Korangar: depois de morrer e respawnar, o personagem volta com HP e movimento, mas continua visualmente preso na animacao de morto. O foco e investigar a causa real no fluxo Korangar + rAthena e aplicar uma correcao minima comprovada.

O workspace atual combina o cliente Korangar em Rust, o servidor rAthena em C++, MariaDB e Docker Compose para reproduzir o loop local de login, entrada no mapa, morte e respawn.

## Core Value

Depois de `@kill -> Respawn`, o player deve voltar visualmente vivo, com janela de respawn fechada, sem depender de tentativa cega ou workaround nao comprovado.

## Requirements

### Validated

- ✓ O workspace possui cliente Korangar em Rust com renderizacao, estado de entidades, UI e networking integrados - existente
- ✓ O workspace possui servidor rAthena C++ configurado para o ambiente local Docker/MariaDB - existente
- ✓ O cliente e o servidor usam o perfil de protocolo `PACKETVER 20220406` como ponto de compatibilidade - existente
- ✓ O bug de respawn esta documentado com reproducao basica em `AI_CONTEXT.md` e `.planning/codebase/CONCERNS.md` - existente
- ✓ Ja existem tentativas anteriores em `NetworkEvent::ResurrectPlayer` e `NetworkEvent::ChangeMap`, mas elas nao resolveram o problema - existente

### Active

- [ ] Reproduzir o bug atual com o cliente e servidor locais
- [ ] Registrar evidencias suficientes para diferenciar entre pacote ausente, entidade errada e estado de animacao sobrescrito
- [ ] Identificar se a causa principal esta no cliente Korangar, no fluxo rAthena de respawn, ou na integracao entre os dois
- [ ] Aplicar a menor correcao segura para o fluxo confirmado
- [ ] Verificar manualmente que `@kill -> Respawn` retorna o player para estado visual vivo e fecha a janela de respawn

### Out of Scope

- Corrigir o bug separado de desconexao/login-server - e relevante, mas nao bloqueia a investigacao minima do respawn
- Criar uma suite E2E completa para login, combate, morte e respawn - valioso depois, mas maior que a correcao minima comprovada
- Reestruturar `korangar/korangar/src/main.rs` por dominio - desejavel, mas arriscado como parte da primeira correcao
- Transformar o ambiente local em servidor publico ou configuracao de producao - o foco e o laboratorio local
- Resolver todos os problemas de protocolo ou pacotes nao implementados - apenas os que forem diretamente necessarios para respawn

## Context

O mapa do codigo identifica este repositorio como um workspace brownfield com tres partes principais:

- Cliente Korangar em Rust, com entrada em `korangar/korangar/src/main.rs`, estado de entidades em `korangar/korangar/src/world/entity/mod.rs`, networking em `korangar/korangar-networking/src/lib.rs` e pacotes em `korangar/ragnarok-packets/src/lib.rs`
- Servidor rAthena em C++, com fluxo de respawn relevante em `rathena-master/src/map/pc.cpp` e envio de pacotes em `rathena-master/src/map/clif.cpp`
- Infra local com `docker-compose.yml`, `docker/entrypoint.sh`, MariaDB 11 e scripts/configs importados em `docker/import/`

O bug conhecido: ao morrer com `@kill` ou PvP e respawnar pelo botao Respawn ou por `@alive`, o personagem pode se mover e recuperar HP, mas permanece renderizado deitado/morto. A janela de respawn tambem pode continuar aberta.

Hipoteses ja registradas:

- O codigo pode estar resetando `entities().first_mut()`, mas o player real pode precisar ser localizado por `this_entity()`
- `set_idle()` pode nao desfazer completamente o estado interno de morte/animação
- Um pacote posterior de status/HP pode chamar `set_dead()` depois do respawn
- O rAthena pode nao enviar `ZC_RESURRECTION` no respawn por save point, representando o retorno apenas como mudanca de mapa/posicao
- A janela de respawn pode estar sendo reaberta por outro handler de status apos `ChangeMap`

## Constraints

- **Escopo**: corrigir o bug de respawn, nao redesenhar o ciclo inteiro de gameplay - reduz risco em uma area fragil
- **Validacao**: a correcao precisa ser comprovada com reproducao manual `@kill -> Respawn` - o comportamento visual e o criterio principal
- **Arquitetura**: `main.rs` e um hub grande de eventos; mudancas devem ser pequenas e localizadas - evita regressao ampla
- **Compatibilidade**: manter `PACKETVER 20220406` - cliente e servidor locais dependem desse perfil
- **Ambiente**: foco em Windows + Docker local conforme o workspace atual - e onde o bug foi observado

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Investigar antes de corrigir | Tentativas anteriores em `ResurrectPlayer` e `ChangeMap` nao resolveram; precisamos provar a causa | - Pending |
| v1 sera correcao minima comprovada | O usuario escolheu resolver o bug com verificacao manual, sem expandir para instrumentacao permanente ou suite E2E | - Pending |
| Manter cliente e servidor como candidatos ate haver evidencia | O fluxo pode falhar por pacote ausente, entidade incorreta ou state machine de animacao | - Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `$gsd-transition`):
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check -> still the right priority?
3. Audit Out of Scope -> reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-26 after initialization*
