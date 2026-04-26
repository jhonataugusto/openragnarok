# Architecture Research

**Domain:** ciclo morte -> respawn visual no workspace Korangar + rAthena
**Researched:** 2026-04-26
**Confidence:** MEDIUM

## Standard Architecture

### System Overview

```text
+------------------------------------------------------------------+
|                         Cliente Korangar                         |
|                                                                  |
|  Input/UI -> main.rs event loop -> ClientState/world -> renderer  |
|      |              ^                    |                       |
|      |              |                    v                       |
|      +---- networking API          Entity animation state         |
|                                                                  |
+---------------------|--------------------|-----------------------+
                      | TCP packets         |
                      v                     |
+------------------------------------------------------------------+
|                         korangar-networking                      |
|                                                                  |
|  packet_versions/version_20220406.rs -> NetworkEvent             |
|  ragnarok-packets/src/lib.rs -> typed packet structs             |
|                                                                  |
+---------------------|--------------------------------------------+
                      | map-server connection
                      v
+------------------------------------------------------------------+
|                         rAthena map-server                       |
|                                                                  |
|  clif.cpp parses CZ_RESTART -> pc.cpp pc_respawn                 |
|  pc_respawn -> pc_setstand + pc_setrestartvalue + pc_setpos       |
|  clif_resurrection is only sent when save-point pc_setpos fails   |
|                                                                  |
+------------------------------------------------------------------+
```

Este fluxo é um caso brownfield com dois caminhos de respawn concorrentes:

1. **Ressurreição explícita:** rAthena envia `ZC_RESURRECTION` (`0x0148`), Korangar converte para `NetworkEvent::ResurrectPlayer`, fecha a janela de respawn se o `entity_id` for do player local e chama `set_idle()` na entidade ressuscitada.
2. **Respawn por save point:** o usuário clica Respawn, Korangar envia `CZ_RESTART` com tipo respawn, rAthena chama `pc_respawn()`, teleporta o personagem com `pc_setpos()`, e o cliente observa isso como `NetworkEvent::ChangeMap`. Neste caminho, pelo código atual do rAthena, `ZC_RESURRECTION` não é enviado quando o warp funciona.

O bug provável vive na junção entre esses caminhos: o estado lógico do player volta a vivo, mas o estado visual `AnimationState::Die` permanece ou é reaplicado depois do reset.

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| Korangar main event loop | Drena eventos de rede e input por frame, aplica mutações em `ClientState`, entidades, mapa, UI e networking. É o coordenador principal do bug. | `korangar/korangar/src/main.rs`, especialmente `render_frame`, `network_event_buffer.drain()`, handlers `NetworkEvent::RemoveEntity`, `ResurrectPlayer`, `ChangeMap` e `InputEvent::Respawn`. |
| Entity lifecycle | Guarda entidades vivas/mortas, movimento, HP, tipo de entidade e estado de animação. Expõe `set_dead`, `set_idle`, `stop_movement` e `is_dead`. | `korangar/korangar/src/world/entity/mod.rs`; `ClientState.entities` e `ClientState.dead_entities` em `korangar/korangar/src/state/mod.rs`. |
| Animation lifecycle | Define a transição visual real entre `Idle`, `Walk`, `Attack`, `Pickup` e `Die`. `dead()` muda para `AnimationActionType::Die`; `idle()` muda para `AnimationActionType::Idle`. | `korangar/korangar/src/world/animation/mod.rs`. |
| Player identity path | Resolve "o jogador local" hoje como `entities.first()` / `entities.first_mut()`. Isso é uma fronteira frágil porque handlers diferentes usam `this_entity()` ou índice 0. | `korangar/korangar/src/state/mod.rs`, funções `this_player()` e `this_entity()`. |
| Korangar networking | Envia ações de map server e converte pacotes em `NetworkEvent`. `respawn()` envia `RestartPacket::new(RestartType::Respawn)`. | `korangar/korangar-networking/src/lib.rs`, `korangar/korangar-networking/src/event.rs`, `korangar/korangar-networking/src/packet_versions/version_20220406.rs`. |
| Packet definitions | Define `ResurrectionPacket` com header `0x0148` e demais structs usadas pelo parser ativo. | `korangar/ragnarok-packets/src/lib.rs`. |
| rAthena clif | Recebe `CZ_RESTART` do cliente, despacha respawn/logout e envia pacotes de cliente como `ZC_RESURRECTION`. | `rathena-master/src/map/clif.cpp`, funções `clif_parse_Restart` e `clif_resurrection`. |
| rAthena pc | Executa regra de gameplay do respawn: valida morte, põe o personagem de pé, recalcula valores de restart e teleporta para save point. | `rathena-master/src/map/pc.cpp`, função `pc_respawn`. |
| Respawn UI | Mostra botões Respawn/Disconnect e emite `InputEvent::Respawn`. A janela deve abrir só na morte do player local e fechar quando o player deixa o estado morto. | `korangar/korangar/src/interface/windows/respawn.rs`, `korangar/korangar/src/interface/windows/mod.rs`, handlers em `main.rs`. |

## Recommended Project Structure

```text
korangar/
  korangar/src/main.rs
    # Coordenador atual: manter correções mínimas aqui durante a investigação.
  korangar/src/state/mod.rs
    # Player identity: this_entity()/this_player(), hoje baseados em entities[0].
  korangar/src/world/entity/mod.rs
    # API de lifecycle de entidade: set_dead/set_idle/stop_movement/update_health.
  korangar/src/world/animation/mod.rs
    # Estado visual efetivo: AnimationState::dead/idle/is_dead.
  korangar-networking/src/lib.rs
    # Envio de RestartPacket no clique Respawn.
  korangar-networking/src/event.rs
    # Contrato NetworkEvent consumido pelo main loop.
  korangar-networking/src/packet_versions/version_20220406.rs
    # Registro de handlers PACKETVER 20220406.
  ragnarok-packets/src/lib.rs
    # Structs de pacotes, incluindo ResurrectionPacket 0x0148.

rathena-master/
  src/map/clif.cpp
    # CZ_RESTART e ZC_RESURRECTION.
  src/map/pc.cpp
    # pc_respawn, pc_setstand, pc_setrestartvalue, pc_setpos.
```

### Structure Rationale

- **Não criar subsistemas novos no primeiro corte:** `main.rs` já concentra o fluxo e é a menor superfície para instrumentar e corrigir o bug comprovado.
- **Tratar `state/mod.rs` como fronteira de identidade:** qualquer correção que dependa de "o player local" deve passar por uma decisão clara: continuar aceitando `entities[0]` ou introduzir resolução por `entity_id` observado no login/map enter.
- **Separar evidência de correção:** primeiro confirmar a sequência exata de `NetworkEvent` e pacotes; só depois decidir se o fix fica no cliente (`ChangeMap` como respawn implícito), no servidor (`pc_respawn` emitindo resurrection também no sucesso), ou em ambos.

## Architectural Patterns

### Pattern 1: Lifecycle único do player local

**What:** Criar ou localizar uma função/trecho único que aplique "player local saiu do estado morto": fechar `WindowClass::Respawn`, zerar movimento, aplicar `set_idle()`, limpar listas transitórias se necessário e registrar evidência debug.

**When to use:** Quando `ResurrectPlayer`, `ChangeMap` ou outro pacote confirmar respawn/revive do player local.

**Trade-offs:** Reduz divergência entre handlers, mas deve ficar mínimo no primeiro fix para não transformar o bug em refatoração de `main.rs`.

**Example:**

```rust
// Exemplo conceitual para guiar o roadmap; nao aplicar sem validar o fluxo real.
fn mark_local_player_alive_after_respawn(&mut self, client_tick: ClientTick) {
    self.interface.close_window_with_class(WindowClass::Respawn);

    if let Some(player) = self.client_state.try_follow_mut(this_entity()) {
        player.set_idle(client_tick);
        player.stop_movement();
    }
}
```

### Pattern 2: Pacote -> evento -> mutação visual

**What:** Manter a direção de dados explícita: pacote bruto do map-server vira pacote tipado, pacote tipado vira `NetworkEvent`, `NetworkEvent` muda `ClientState`, e o renderer só reflete o estado resultante.

**When to use:** Em toda investigação do respawn para evitar corrigir o renderer quando o problema está no estado de entidade ou no pacote ausente.

**Trade-offs:** Exige logs nos limites certos; logar só a UI ou só o servidor não prova a ordem de sobrescrita.

### Pattern 3: Evidência temporal por frame

**What:** Instrumentar temporariamente a ordem dos eventos no frame em que o player morre e respawna: `RemoveEntity(Died)`, `ResurrectPlayer`, `ChangeMap`, `UpdateEntityHealth`, `PlayerMove`, abertura/fechamento de `RespawnWindow`, `AnimationState`.

**When to use:** Antes da correção mínima, porque o bug pode ser pacote ausente, entidade errada ou `set_dead()` posterior.

**Trade-offs:** Logs temporários são mais baratos que uma suíte E2E agora, mas devem ser removidos ou protegidos por `debug` depois da correção.

## Data Flow

### Death Flow

```text
rAthena map-server
  -> clif sends disappearance/death notification
  -> korangar-networking decodes packet
  -> NetworkEvent::RemoveEntity { reason: Died }
  -> korangar/korangar/src/main.rs
      -> find entity by entity_id in ClientState.entities
      -> if Monster: clone to dead_entities, remove from entities
      -> if Player: entity.set_dead(client_tick)
      -> if local player assumed by entities[0]: open RespawnWindow
  -> world/entity/mod.rs
      -> Entity::set_dead()
  -> world/animation/mod.rs
      -> AnimationState::dead() sets AnimationActionType::Die
  -> renderer draws dead pose
```

Ponto frágil: o handler abre a janela quando `entity_id == self.client_state.follow(client_state().entities())[0].get_entity_id()`. Isso repete a premissa de que o player local é sempre `entities[0]`.

### User Respawn Flow

```text
RespawnWindow button
  -> InputEvent::Respawn
  -> main.rs calls networking_system.respawn()
  -> korangar-networking/src/lib.rs sends RestartPacket(Respawn)
  -> rAthena clif.cpp clif_parse_Restart(type 0)
  -> rAthena pc.cpp pc_respawn(sd, CLR_OUTSIGHT)
      -> pc_isdead guard
      -> pc_setstand(sd, true)
      -> pc_setrestartvalue(sd, 3)
      -> pc_setpos(save_point)
      -> if pc_setpos fails: clif_resurrection(*sd)
      -> if pc_setpos succeeds: no clif_resurrection in current code
```

Implicação: no respawn normal por save point, o evento mais confiável no cliente provavelmente é `ChangeMap`, não `ResurrectPlayer`.

### Explicit Resurrection Flow

```text
rAthena clif_resurrection()
  -> sends ZC_RESURRECTION 0x0148 to AREA
  -> ragnarok-packets ResurrectionPacket
  -> version_20220406.rs maps to NetworkEvent::ResurrectPlayer { entity_id }
  -> main.rs:
      -> if this_entity().entity_id == entity_id: close RespawnWindow
      -> find entity by entity_id in entities
      -> entity.set_idle(client_tick)
      -> entity.stop_movement()
  -> AnimationState::idle() sets AnimationActionType::Idle
```

Este caminho já usa busca por `entity_id` para a entidade ressuscitada, mas ainda usa `this_entity()` para decidir se fecha a UI.

### Save-Point Map Change Respawn Flow

```text
rAthena pc_respawn successful pc_setpos(save_point)
  -> map-change/position packets
  -> korangar-networking maps ChangeMapPacket to NetworkEvent::ChangeMap
  -> main.rs ChangeMap handler:
      -> clears map/effects/lights/audio
      -> truncates ClientState.entities to 1
      -> clears dead_entities and ground_items
      -> closes Dialog
      -> closes RespawnWindow
      -> entities.first_mut().set_idle(client_tick)
      -> entities.first_mut().stop_movement()
      -> request_map_load(map_name, position)
```

Ponto frágil: o reset visual no caminho mais provável do respawn depende de `entities.first_mut()`, não de uma identidade confirmada do player local.

### Health/Status Flow

```text
rAthena status/HP packet
  -> korangar-networking UpdateEntityHealth
  -> main.rs updates entity health only
```

Pelo código observado, `UpdateEntityHealth` apenas chama `entity.update_health()`; ele não chama `set_dead()`. A hipótese de um HP=0 posterior reabrir morte ainda deve ser validada nos pacotes reais, mas o ponto de `set_dead()` confirmado neste recorte é `NetworkEvent::RemoveEntity { reason: Died }`.

## Build Order Implications

1. **Reprodução controlada e baseline**
   - Subir Docker e cliente conforme `AI_CONTEXT.md`.
   - Reproduzir `@kill -> Respawn`.
   - Registrar se o bug ocorre por botão Respawn, `@alive`, ou ambos.
   - Resultado esperado da fase: bug reproduzido com versão e comandos exatos.

2. **Evidência de eventos no cliente**
   - Instrumentar temporariamente `korangar/korangar/src/main.rs` nos handlers `RemoveEntity`, `ResurrectPlayer`, `ChangeMap`, `UpdateEntityHealth` e `InputEvent::Respawn`.
   - Logar `entity_id`, se bate com `this_entity()`, índice na lista `entities`, `is_dead()`, HP, abertura/fechamento de `RespawnWindow` e ordem por `client_tick`.
   - Resultado esperado da fase: saber se `set_idle()` é chamado no player real e se algum evento posterior o desfaz.

3. **Evidência de pacote/protocolo**
   - Usar `packet_callback`/debug ou captura no port do map-server para confirmar se `ZC_RESURRECTION` aparece no respawn normal.
   - Validar `ChangeMapPacket` pós-respawn e qualquer pacote de vanish/death subsequente.
   - Resultado esperado da fase: diferenciar "servidor não manda resurrection" de "cliente ignora/aplica na entidade errada".

4. **Correção mínima no cliente se o fluxo confirmado for ChangeMap**
   - Se `ChangeMap` é o sinal real do respawn por save point, substituir a premissa `entities.first_mut()` por uma resolução explícita do player local via `this_entity()` ou por `entity_id` rastreado.
   - Centralizar o reset de respawn para fechar UI + `set_idle()` + `stop_movement()` no mesmo caminho.
   - Não mexer em renderer nem em animação antes de provar que `AnimationState::idle()` é insuficiente.

5. **Correção mínima no rAthena somente se a evidência exigir**
   - Se o cliente precisa receber `ZC_RESURRECTION` para compatibilidade ou se `ChangeMap` é ambíguo demais, avaliar patch em `rathena-master/src/map/pc.cpp` para chamar `clif_resurrection(*sd)` também após `pc_setpos()` bem-sucedido.
   - Essa mudança tem blast radius maior: afeta semântica de servidor e clientes oficiais/compatíveis, então deve ser posterior à prova de que o cliente não consegue resolver de forma segura.

6. **Verificação manual**
   - Verificar `@kill -> Respawn`: player visualmente em pé/vivo, HP restaurado, movimento funcional, janela Respawn fechada.
   - Verificar `@kill -> @alive` se esse caminho for relevante.
   - Verificar que morte ainda abre RespawnWindow e que monstros mortos continuam no fluxo de `dead_entities`.

## Scaling Considerations

| Scale | Architecture Adjustments |
|-------|--------------------------|
| Correção local única | Manter logs temporários e fix mínimo nos handlers atuais de `main.rs`. |
| Mais bugs de lifecycle | Extrair helper de player lifecycle dentro do cliente antes de mexer em networking ou renderer. |
| Compatibilidade ampla Korangar/rAthena | Criar matriz de pacotes para morte, vanish, resurrection, map move, HP/status e smoke test manual automatizável. |

### Scaling Priorities

1. **Primeiro gargalo:** identidade do player local baseada em `entities[0]`. Corrigir com rastreamento explícito antes de adicionar mais casos especiais.
2. **Segundo gargalo:** `main.rs` como hub gigante. Só extrair depois que a correção mínima estiver comprovada, para não esconder regressão em refatoração.

## Anti-Patterns

### Anti-Pattern 1: Corrigir animação sem provar o evento

**What people do:** Alterar `AnimationState::idle()`/`dead()` ou renderer porque o sintoma é visual.

**Why it's wrong:** O estado visual pode estar correto para a entidade errada, ou pode ser sobrescrito por evento posterior. Mexer na animação pode mascarar morte real de outros players/monstros.

**Do this instead:** Logar pacote -> `NetworkEvent` -> entidade afetada -> `AnimationState` antes/depois.

### Anti-Pattern 2: Assumir que respawn sempre envia `ZC_RESURRECTION`

**What people do:** Depender de `NetworkEvent::ResurrectPlayer` como único ponto de reset.

**Why it's wrong:** `pc_respawn()` em `rathena-master/src/map/pc.cpp` só chama `clif_resurrection()` se `pc_setpos()` falhar; no caminho normal, o sinal é teleport/map change.

**Do this instead:** Tratar `ChangeMap` pós-morte como candidato principal de respawn por save point, mas só depois de confirmar a sequência real.

### Anti-Pattern 3: Usar `entities.first_mut()` em mais lugares

**What people do:** Repetir a premissa atual de que o player local é sempre a primeira entidade.

**Why it's wrong:** O próprio código marca `this_entity()`/`this_player()` com TODO "Select our player better", e o bug atual pode ser exatamente entidade errada.

**Do this instead:** Resolver o player local por uma identidade estável ou ao menos centralizar a premissa em um helper auditável.

### Anti-Pattern 4: Patchar rAthena antes de isolar o cliente

**What people do:** Fazer `pc_respawn()` sempre mandar `clif_resurrection()` porque isso parece fechar o bug.

**Why it's wrong:** Pode criar comportamento duplicado ou divergente do protocolo esperado para map-change respawn, e altera o servidor para compensar estado cliente mal resolvido.

**Do this instead:** Só patchar rAthena se a captura provar que o contrato esperado pelo Korangar exige esse pacote ou se a correção cliente for mais arriscada.

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| rAthena map-server | TCP map-server packets, PACKETVER 20220406 | Respawn por botão usa `CZ_RESTART`; resposta normal pode ser map move em vez de `ZC_RESURRECTION`. |
| MariaDB/Docker | Estado persistente e runtime local | Não é fronteira principal do bug visual; usar apenas para resetar ambiente se necessário. |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| `RespawnWindow` -> `main.rs` | `InputEvent::Respawn` | UI dispara intenção; não deve decidir lifecycle visual. |
| `main.rs` -> `korangar-networking` | chamada direta `networking_system.respawn()` | Envia `RestartPacket(Respawn)` ao map-server. |
| `korangar-networking` -> `main.rs` | `NetworkEventBuffer` drenado por frame | Ordem de eventos dentro do frame é central para a investigação. |
| `main.rs` -> `ClientState.entities` | mutação direta | Local atual de `set_dead`, `set_idle`, truncamento em map change e abertura/fechamento da UI. |
| `Entity` -> `AnimationState` | métodos `set_dead`/`set_idle` | A entidade é a fronteira correta para mudar estado visual; renderer deve permanecer passivo. |
| rAthena `clif.cpp` -> `pc.cpp` | chamada `pc_respawn(sd, CLR_OUTSIGHT)` | `clif` interpreta pacote cliente; `pc` executa regra de respawn. |
| rAthena `pc.cpp` -> `clif.cpp` | chamada condicional `clif_resurrection(*sd)` | Atualmente só no erro de `pc_setpos()` dentro de `pc_respawn`. |

## Roadmap Recommendation

Para o roadmap, a ordem deve ser:

1. **Reprodução e baseline:** confirmar o bug atual sem mudar código.
2. **Instrumentação de lifecycle cliente:** provar entidade, ordem de eventos, janela e animação.
3. **Captura de protocolo/rAthena:** confirmar se `ZC_RESURRECTION` está ausente no respawn normal e quais pacotes substituem esse sinal.
4. **Fix mínimo cliente:** corrigir o reset do player local no caminho comprovado, preferindo identidade correta sobre `entities[0]`.
5. **Fix servidor apenas se necessário:** considerar `pc_respawn`/`clif_resurrection` depois de evidência.
6. **Verificação manual focada:** `@kill -> Respawn`, `@alive`, janela fechada, player em pé, movimento e HP funcionais.

Essa ordem evita duas falhas comuns: patchar rAthena para esconder uma premissa errada no cliente, ou mexer na animação sem saber se o estado morto foi reaplicado por evento posterior.

## Sources

- `.planning/PROJECT.md` - contexto do projeto e requisitos ativos.
- `.planning/codebase/ARCHITECTURE.md` - arquitetura existente do workspace.
- `.planning/codebase/STRUCTURE.md` - localizações-chave.
- `.planning/codebase/CONCERNS.md` - áreas frágeis e bug conhecido.
- `AI_CONTEXT.md` - reprodução, hipóteses e tentativas anteriores.
- `korangar/korangar/src/main.rs` - handlers de `NetworkEvent` e `InputEvent`.
- `korangar/korangar/src/state/mod.rs` - `ClientState`, `this_entity()` e `this_player()`.
- `korangar/korangar/src/world/entity/mod.rs` - lifecycle de entidade.
- `korangar/korangar/src/world/animation/mod.rs` - `AnimationState`.
- `korangar/korangar/src/interface/windows/respawn.rs` - UI de respawn.
- `korangar/korangar-networking/src/lib.rs` - envio de `RestartPacket`.
- `korangar/korangar-networking/src/event.rs` - contrato `NetworkEvent`.
- `korangar/korangar-networking/src/packet_versions/version_20220406.rs` - mapeamento de pacotes.
- `korangar/ragnarok-packets/src/lib.rs` - `ResurrectionPacket`.
- `rathena-master/src/map/clif.cpp` - `clif_parse_Restart` e `clif_resurrection`.
- `rathena-master/src/map/pc.cpp` - `pc_respawn`.

---
*Architecture research for: ciclo morte -> respawn visual no Korangar + rAthena*
*Researched: 2026-04-26*
