# Ragnarok Client Modernization

## What This Is

Este projeto evolui o workspace local Ragnarok/Korangar com correcoes e melhorias comprovadas no cliente, mantendo o rAthena local como ambiente de reproducao quando necessario.

O milestone anterior estabilizou o fluxo de respawn. O foco atual passa a ser uma feature opcional de dialogo cinematico com NPCs, inspirada em MMORPGs modernos: camera contextual entre player e NPC, texto com efeito typewriter, som por letra com variacao de pitch e escolhas como baloes empilhados.

## Core Value

Ao falar com NPCs, o jogador pode ativar uma apresentacao moderna e cinematica sem perder o dialogo classico existente como fallback/configuracao.

## Requirements

### Validated

- [x] O workspace possui cliente Korangar em Rust com renderizacao, estado de entidades, UI e networking integrados - existente
- [x] O workspace possui servidor rAthena C++ configurado para o ambiente local Docker/MariaDB - existente
- [x] O cliente e o servidor usam o perfil de protocolo `PACKETVER 20220406` como ponto de compatibilidade - existente
- [x] O bug de respawn esta documentado com reproducao basica em `AI_CONTEXT.md` e `.planning/codebase/CONCERNS.md` - existente
- [x] Ja existem tentativas anteriores em `NetworkEvent::ResurrectPlayer` e `NetworkEvent::ChangeMap`, mas elas nao resolveram o problema - existente
- [x] Reproducao atual confirmada em Phase 1: apos `@kill -> Respawn`, o player permanece morto/deitado; a janela Respawn fecha, mas HP nao restaura e movimento nao funciona - Phase 1

### Active

- [ ] Criar uma opcao global em Interface Settings para ativar/desativar o modo de dialogo cinematico com NPCs
- [ ] Manter o dialogo classico atual disponivel quando a opcao estiver desligada ou quando o modo cinematico precisar de fallback
- [ ] Aplicar o modo cinematico a todos os dialogos de NPC quando a opcao estiver ativada
- [ ] Durante o dialogo cinematico, travar temporariamente movimento e controle manual de camera ate o dialogo fechar ou avancar conforme o fluxo
- [ ] A camera cinematica deve ser dinamica e entrar sem cortes bruscos, com configuracao simples de ligar/desligar usando valores padrao
- [ ] O texto cinematico deve usar efeito typewriter, com clique/tecla para revelar a fala inteira antes de avancar
- [ ] Mouse esquerdo, Enter e Espaco devem revelar/avancar texto no dialogo cinematico
- [ ] O som por letra deve ter toggle simples ligado/desligado, usando volume de efeitos e pitch automatico
- [ ] As opcoes de resposta devem aparecer como baloes empilhados acima da caixa de dialogo, centralizados perto da parte inferior
- [ ] Quando o modo cinematico estiver ativo, a UI cinematica deve substituir visualmente a janela classica de dialogo
- [ ] Implementar a UI cinematica como caminho novo sobre os mesmos eventos de dialogo, preservando a `DialogWindow` classica como fallback
- [ ] Definir design de camera, texto typewriter, som por letra e escolhas em baloes antes de implementar

### Out of Scope

- Alterar `rathena-master/` para esta feature; o dialogo cinematico deve ser comportamento de cliente sobre o protocolo existente
- Remover ou substituir definitivamente o dialogo classico atual
- Criar uma suite E2E completa para NPC/dialogo nesta primeira iteracao
- Reestruturar amplamente `korangar/korangar/src/main.rs` sem necessidade direta da feature
- Transformar o ambiente local em servidor publico ou configuracao de producao

## Context

O mapa do codigo identifica este repositorio como um workspace brownfield com tres partes principais:

- Cliente Korangar em Rust, com entrada em `korangar/korangar/src/main.rs`, estado de entidades em `korangar/korangar/src/world/entity/mod.rs`, networking em `korangar/korangar-networking/src/lib.rs` e pacotes em `korangar/ragnarok-packets/src/lib.rs`
- Servidor rAthena em C++, com fluxo de respawn relevante em `rathena-master/src/map/pc.cpp` e envio de pacotes em `rathena-master/src/map/clif.cpp`
- Infra local com `docker-compose.yml`, `docker/entrypoint.sh`, MariaDB 11 e scripts/configs importados em `docker/import/`

Baseline confirmado na Phase 1 do milestone anterior:

- Ambiente local pronto: Docker/rAthena/MariaDB, portas locais, `play.bat`, executavel Korangar e GRFs.
- Fluxo observado: `@kill -> Respawn` no cliente real.
- Resultado final: player permanece morto/deitado.
- Janela Respawn fecha.
- HP nao restaura.
- Movimento nao funciona.
- Conta/personagem nao foram informados no checkpoint.

Resolucao do milestone de respawn:

- O bug foi resolvido fora do fluxo completo de fases planejado.
- Causa informada pelo usuario: faltava registrar/tratar um pacote desconhecido como evento no client.
- Com o pacote agora reconhecido no cliente, o diagnostico de respawn fica encerrado e o projeto pode seguir para um novo milestone.

Hipoteses antigas do respawn, mantidas apenas como historico:

- O codigo pode estar resetando `entities().first_mut()`, mas o player real pode precisar ser localizado por `this_entity()`
- `set_idle()` pode nao desfazer completamente o estado interno de morte/animacao
- Um pacote posterior de status/HP pode chamar `set_dead()` depois do respawn
- O rAthena pode nao enviar `ZC_RESURRECTION` no respawn por save point, representando o retorno apenas como mudanca de mapa/posicao
- A janela de respawn pode estar sendo reaberta por outro handler de status apos `ChangeMap`

## Constraints

- **Escopo**: construir a feature no cliente Korangar, mantendo o dialogo classico como fallback
- **Validacao**: a feature precisa ser verificavel manualmente conversando com NPCs reais
- **Arquitetura**: `main.rs` e um hub grande de eventos; mudancas devem ser pequenas e bem isoladas quando possivel
- **Compatibilidade**: manter `PACKETVER 20220406` - cliente e servidor locais dependem desse perfil
- **Ambiente**: foco em Windows + Docker local conforme o workspace atual - e onde o bug foi observado

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Diagnostico de respawn encerrado | Usuario informou que a causa real era pacote desconhecido nao registrado como evento no cliente | Closed |
| Iniciar novo milestone de feature | O bug de respawn nao existe mais no estado atual; foco muda para dialogo cinematico de NPC | Accepted |
| Feature deve ser opcional | Preserva comportamento atual e permite fallback seguro | Accepted |
| Aplicar a todos os dialogos de NPC quando ativo | Usuario escolheu abrangencia total para o modo cinematico | Accepted |
| Travar movimento e camera durante dialogo cinematico | Usuario escolheu controle temporario da cena para manter enquadramento moderno | Accepted |
| Camera cinematica dinamica com configuracao simples | Usuario prefere movimento suave sem cortes e escolheu toggle simples em vez de controles granulares | Accepted |
| Typewriter pode ser pulado | Primeiro clique/tecla revela a fala inteira; depois o fluxo avanca normalmente | Accepted |
| Entradas de dialogo cinematico | Mouse esquerdo, Enter e Espaco revelam/avancam texto | Accepted |
| Som de texto configuravel por toggle simples | Usuario escolheu ligar/desligar sem volume separado; usa volume de efeitos e pitch automatico | Accepted |
| Opcoes como baloes empilhados | Usuario escolheu baloes acima da caixa de dialogo, centralizados perto da parte inferior | Accepted |
| UI cinematica substitui janela classica visualmente | Quando ativo, so a UI cinematica aparece; o classico permanece como fallback/configuracao | Accepted |
| UI cinematica isolada | Usuario aprovou criar caminho novo sobre os mesmos eventos/protocolo, sem misturar a janela classica | Accepted |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition**:
1. Requirements invalidated? -> Move to Out of Scope with reason
2. Requirements validated? -> Move to Validated with phase reference
3. New requirements emerged? -> Add to Active
4. Decisions to log? -> Add to Key Decisions
5. "What This Is" still accurate? -> Update if drifted

**After each milestone**:
1. Full review of all sections
2. Core Value check -> still the right priority?
3. Audit Out of Scope -> reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-28 after respawn diagnostic closure and NPC cinematic dialog milestone start*
