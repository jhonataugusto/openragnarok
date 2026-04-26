# Pesquisa de Pitfalls

**Domínio:** correção investigativa do bug de respawn visual no workspace Korangar + rAthena  
**Pesquisado em:** 2026-04-26  
**Confiança:** ALTA para riscos baseados no código local; MÉDIA para hipóteses que ainda dependem de reprodução manual.

## Pitfalls Críticos

### Pitfall 1: Tratar movimento e HP como prova de respawn visual corrigido

**O que dá errado:**  
A correção parece funcionar porque o personagem volta a andar, recebe HP e a janela de respawn fecha, mas a entidade continua renderizada na pose de morto. Isso produz um falso positivo exatamente igual ao bug atual.

**Por que acontece:**  
O estado funcional e o estado visual passam por caminhos diferentes. O rAthena executa `pc_setstand`, `pc_setrestartvalue` e `pc_setpos` em `pc_respawn`, enquanto o Korangar mantém estado próprio de `AnimationState`. No cliente, `UpdateEntityHealth` apenas atualiza HP e não prova que `animation_state` saiu de `dead`.

**Como evitar:**  
A fase de investigação deve registrar, no mesmo fluxo `@kill -> Respawn`, os seguintes fatos: pacote recebido, `entity_id`, entidade selecionada como player, HP, `is_dead`, animação atual e estado da janela `WindowClass::Respawn`. O critério de aceite não pode ser "consegue andar"; deve ser "entidade local não está morta visualmente e a janela não reabre".

**Sinais de alerta:**  
- Correção validada só por movimento, HP ou ausência de erro.
- Nenhum log de `AnimationState` antes/depois de `ChangeMap`, `ResurrectPlayer` e `RemoveEntity`.
- Vídeo/screenshot mostra player deitado mesmo com HP cheio.
- `UpdateEntityHealth` ocorre depois do respawn, mas nenhum evento visual de stand-up/idle confirmado.

**Fase responsável:**  
Fase 1 - Reprodução instrumentada e critérios de evidência.

---

### Pitfall 2: Corrigir a entidade errada por assumir `entities()[0]`

**O que dá errado:**  
O código chama `set_idle()` e `stop_movement()` no primeiro item de `entities()`, mas o player real pode não ser a entidade que precisa ser corrigida no momento do evento. O bug permanece ou uma entidade errada muda de animação.

**Por que acontece:**  
O código atual ainda tem `TODO: Select our player better` em `this_player()` e `this_entity()`, ambos baseados em `state.entities.first()`. Além disso, `RemoveEntity` abre a janela de respawn comparando com `entities()[0]`, e `ChangeMap` usa `truncate(1)` seguido de `first_mut()`. Essa suposição é frágil em um cliente que recebe múltiplas entidades, remoções e recriações em sequência.

**Como evitar:**  
Antes de qualquer correção permanente, registrar o `entity_id` do player local em todos os handlers relevantes e comparar com o `entity_id` recebido em `RemoveEntity`, `ResurrectPlayer`, `PlayerStandUp`, `ChangeMap` e pacotes de spawn após carregar o mapa. A correção deve centralizar seleção do player por identidade estável, não por índice, ou pelo menos provar que o índice 0 é invariável durante o fluxo reproduzido.

**Sinais de alerta:**  
- Logs mostram `ResurrectPlayer.entity_id` diferente do primeiro `entities()[0].entity_id`.
- A janela de respawn abre/fecha com base em índice, mas o player local exibido tem outro ID.
- `truncate(1)` remove entidades esperadas ou preserva uma entidade que não corresponde ao player local.
- A correção melhora um cenário isolado, mas falha após teleport, relogin, outro player na tela ou respawn por `@alive`.

**Fase responsável:**  
Fase 2 - Identificação da entidade local e helper de ciclo de vida do player.

---

### Pitfall 3: Esperar `ZC_RESURRECTION` no fluxo em que rAthena usa mudança de mapa

**O que dá errado:**  
A correção fica concentrada em `NetworkEvent::ResurrectPlayer`, mas o respawn normal por save point não passa por esse evento. O cliente só recebe mudança de mapa/posição e continua com a animação de morto preservada.

**Por que acontece:**  
No rAthena local, `pc_respawn` chama `pc_setstand`, `pc_setrestartvalue` e depois `pc_setpos` para o save point. `clif_resurrection` só é chamado se `pc_setpos` falhar. No Korangar, `ZC_RESURRECTION` é mapeado para `NetworkEvent::ResurrectPlayer`, mas a hipótese documentada é que o caminho normal chega como `NetworkEvent::ChangeMap`.

**Como evitar:**  
Capturar o fluxo real de pacotes no map-server durante `@kill -> Respawn`: confirmar se aparece `0x0148 ZC_RESURRECTION`, `ChangeMapPacket`, `DamageType::StandUp`, remoção por morte ou spawn novo. A decisão cliente vs servidor deve sair dessa captura: se não houver `ZC_RESURRECTION`, a fase de correção deve tratar respawn via `ChangeMap` como transição explícita de morte para vivo, ou alterar rAthena de modo mínimo e validado para emitir o sinal esperado.

**Sinais de alerta:**  
- Só há alterações em `NetworkEvent::ResurrectPlayer`, mas nenhuma evidência de `ResurrectPlayer` durante respawn por botão.
- `pc_respawn` continua sem enviar `clif_resurrection` no caso `SETPOS_OK`.
- `ChangeMap` ocorre após clicar Respawn, mas a correção o trata como teleport genérico.
- `@alive` parece diferente de Respawn pelo botão, sem comparação de pacotes.

**Fase responsável:**  
Fase 1 - Captura de pacotes e classificação do fluxo real; Fase 3 - decisão de correção cliente/servidor.

---

### Pitfall 4: `set_dead` reescrever o estado depois do reset para idle

**O que dá errado:**  
`set_idle()` é chamado corretamente, mas um evento posterior volta a marcar a entidade como morta. O log superficial mostra a correção executada, mas a tela final continua deitada.

**Por que acontece:**  
`RemoveEntity { reason: Died }` chama `set_dead()` para players e abre `RespawnWindow`. Eventos podem chegar em sequência após `ChangeMap` ou durante troca de mapa. Se a investigação só registra o ponto onde `set_idle()` é chamado, ela perde o evento que sobrescreve o estado.

**Como evitar:**  
Instrumentar todos os pontos que podem alterar morte/vida visual do player: `RemoveEntity`, `ResurrectPlayer`, `PlayerStandUp`, `ChangeMap`, spawn do player e qualquer handler que atualize status crítico. O log deve ter ordem temporal com `client_tick`, nome do evento, `entity_id`, `old_is_dead`, `new_is_dead`, HP e janela de respawn. A correção deve ser aplicada no último ponto causal confirmado, não no primeiro lugar onde parece conveniente.

**Sinais de alerta:**  
- Log contém `set_idle` antes da tela final, mas não contém o último `set_dead`.
- `RespawnWindow` fecha em `InputEvent::Respawn` ou `ChangeMap` e reabre logo depois.
- A animação fica correta por um frame e volta a morto.
- A ordem dos eventos muda entre respawn por botão e `@alive`.

**Fase responsável:**  
Fase 1 - Instrumentação de transições; Fase 4 - validação contra regressão de ordem de eventos.

---

### Pitfall 5: Misturar correção de protocolo com workaround visual amplo

**O que dá errado:**  
Uma mudança genérica no cliente força `set_idle()` em todo `ChangeMap`, `UpdateEntityHealth` ou spawn, mascarando a causa real e criando regressões em teleport, morte de outros players, monstros mortos, efeitos de batalha ou transições legítimas.

**Por que acontece:**  
`main.rs` é um hub grande de eventos, e a área de morte/respawn cruza networking, UI, entidades e mapa. Como as tentativas anteriores já adicionaram resets em `ResurrectPlayer` e `ChangeMap`, há tendência de ampliar o workaround sem provar o fluxo.

**Como evitar:**  
Manter a correção mínima e condicionada: só aplicar reset visual quando a entidade é o player local e quando há evidência de transição de morte para respawn. Se a escolha for servidor, limitar a alteração a `pc_respawn`/sinalização do caso save point e documentar impacto no protocolo. Se for cliente, criar helper explícito de "player respawned" em vez de espalhar `set_idle()` por handlers genéricos.

**Sinais de alerta:**  
- Mudança toca muitos handlers sem uma captura de pacote correspondente.
- `set_idle()` passa a rodar em todo `ChangeMap`, inclusive teleports normais, sem condição de morte anterior.
- Correção afeta monstros ou outros players.
- Não há teste manual para teleport normal, morte sem respawn, `@alive` e respawn por botão.

**Fase responsável:**  
Fase 3 - Correção mínima comprovada; Fase 4 - matriz de regressão manual.

---

### Pitfall 6: Editar rAthena ativo, backup ou patch errado

**O que dá errado:**  
A mudança é feita em `rathena-master.bak`, em patch não aplicado, ou em fonte ativo que não recompila. O teste roda binário antigo e a investigação conclui falsamente que a hipótese falhou.

**Por que acontece:**  
O workspace tem checkouts aninhados, backup de rAthena, patches separados em `korangar-rathena-patches/` e build Docker que deixa binários gerados no source tree. O entrypoint pode pular compilação se binários já existem.

**Como evitar:**  
Antes de validar qualquer hipótese no servidor, registrar `git -C rathena-master status --short`, verificar que o arquivo alterado está em `rathena-master/`, forçar rebuild quando a mudança for C++, e confirmar no log do container que o binário novo subiu. Se a correção virar permanente, atualizar também o patch correspondente ou criar patch novo reprodutível.

**Sinais de alerta:**  
- Alteração aparece só em `rathena-master.bak` ou em patch solto.
- Docker sobe rápido demais após mudança C++.
- Logs do map-server não mostram marcador temporário esperado.
- `git -C rathena-master status --short` não reflete a alteração testada.

**Fase responsável:**  
Fase 3 - Aplicação da correção; Fase 4 - verificação de build e patch ativo.

---

### Pitfall 7: Transformar instrumentação temporária em ruído permanente

**O que dá errado:**  
Logs úteis para investigação ficam espalhados em `main.rs`, networking e rAthena, poluindo execução normal, piorando performance e tornando futuras análises menos claras.

**Por que acontece:**  
O bug exige observar eventos em ordem, mas não há harness E2E pronto. A solução rápida é adicionar prints em vários pontos e esquecer de removê-los ou protegê-los por `debug`.

**Como evitar:**  
Adicionar instrumentação temporária atrás de `#[cfg(feature = "debug")]`, `print_debug!` ou marcador facilmente removível. Documentar exatamente quais logs são critério de decisão. Ao final da fase de correção, manter apenas logs que agregam diagnóstico real e remover o restante.

**Sinais de alerta:**  
- Logs de respawn aparecem em build normal sem feature debug.
- Mensagens duplicadas por frame ou por pacote comum.
- Revisão mostra prints ad hoc sem condição e sem plano de remoção.
- O arquivo `main.rs` cresce com comentários de investigação que não explicam comportamento permanente.

**Fase responsável:**  
Fase 1 - Instrumentação controlada; Fase 5 - limpeza pós-validação, se houver.

## Padrões de Dívida Técnica

Atalhos que parecem razoáveis, mas aumentam risco neste bug.

| Atalho | Benefício imediato | Custo de longo prazo | Quando aceitável |
|---|---|---|---|
| Chamar `set_idle()` em mais handlers sem prova do fluxo | Teste rápido e simples | Masca a causa, cria regressão em teleport e outros players | Só como experimento temporário com log e revert planejado |
| Usar `entities()[0]` como player local | Segue o padrão atual mínimo | Mantém o bug frágil e dificulta qualquer correção confiável | Apenas se a fase 1 provar invariância no fluxo testado; ainda deve virar dívida explícita |
| Alterar rAthena sem patch reprodutível | Valida hipótese rápido | Perde compatibilidade ao reclonar/atualizar rAthena | Aceitável só em spike local; não no fechamento da fase |
| Fechar `RespawnWindow` no clique e considerar resolvido | Melhora UX aparente | Janela pode reabrir e visual morto continua | Nunca como critério único de conclusão |
| Validar somente com `@alive` | Fluxo rápido de teste | Pode não representar Respawn por botão/save point | Só como caso adicional, nunca substituto do fluxo principal |

## Gotchas de Integração

| Integração | Erro comum | Abordagem correta |
|---|---|---|
| Korangar `NetworkEvent::ResurrectPlayer` + rAthena `pc_respawn` | Assumir que todo respawn envia `ZC_RESURRECTION` | Capturar pacotes; tratar `ChangeMap` como candidato principal no respawn por save point |
| Korangar state paths | Assumir que `this_entity()` resolve identidade estável | Confirmar `entity_id` local e reduzir uso de índice para lógica de morte/respawn |
| UI `RespawnWindow` + estado de entidade | Fechar janela sem corrigir animação | Validar janela e animação como estados independentes |
| Docker rAthena | Testar binário antigo após editar C++ | Rebuild explícito e log/marker de versão da alteração testada |
| Packet history/debug | Depender de histórico incompleto durante tráfego alto | Capturar o trecho de respawn com escopo curto e timestamps, ou usar captura externa se necessário |

## Armadilhas de Performance

| Armadilha | Sintomas | Prevenção | Quando quebra |
|---|---|---|---|
| Logar todos os pacotes/frames sem filtro | Cliente engasga, logs gigantes, evento causal se perde | Filtrar por `entity_id` local e eventos de morte/respawn/mapa | Já quebra em sessão manual com tráfego de mapa |
| Rebuild Docker completo a cada hipótese pequena | Ciclo de investigação lento e hipóteses abandonadas | Separar hipóteses cliente/servidor; só rebuildar rAthena quando a hipótese exigir | Quebra a produtividade da fase 1/3 |
| Captura longa de packet history no cliente | Histórico limpa/drops e perde trecho importante | Iniciar captura imediatamente antes de `@kill`; exportar evidência logo após respawn | Quebra quando há tráfego alto ou buffer excedido |

## Erros de Segurança e Escopo

| Erro | Risco | Prevenção |
|---|---|---|
| Expandir exposição do Docker para facilitar teste remoto | Credenciais dev, GM commands e packet obfuscation desabilitado ficam expostos | Manter validação local/loopback; não mexer em bind público nesta correção |
| Levar patches de debug/GM para perfil compartilhado | Servidor fica inseguro fora do laboratório local | Marcar ajustes como dev-only e não tocar permissões GM nesta fase |
| Incluir GRFs/binários/logs em commit de correção | Vazamento de assets e diffs enormes | Checar status nos repos aninhados e ignorar artefatos gerados |

## Pitfalls de UX

| Pitfall | Impacto no usuário | Melhor abordagem |
|---|---|---|
| Janela de respawn fecha cedo demais, mas personagem segue morto | Usuário acha que está vivo funcionalmente, mas tela contradiz o estado | Sincronizar fechamento da janela com transição visual confirmada |
| Correção funciona só após teleport/relog | Usuário precisa workaround manual | Validar fluxo direto `@kill -> Respawn` sem relogin |
| Respawn por botão e `@alive` têm comportamento diferente | Debug fica enganoso e UX inconsistente | Testar ambos e registrar diferenças de pacote/evento |
| Reset visual remove feedback legítimo de morte de outros players | Cena fica inconsistente em PvP | Condicionar correção ao player local e ao fluxo de respawn confirmado |

## Checklist de "Parece Pronto, Mas Não Está"

- [ ] **Respawn visual:** verificar que o player local termina com `is_dead == false` e renderiza de pé/idle, não apenas com HP > 0.
- [ ] **Janela de respawn:** verificar que `WindowClass::Respawn` fecha e não reabre após os eventos finais de mapa/status.
- [ ] **Fluxo real:** anexar evidência se o respawn por botão recebeu `ZC_RESURRECTION`, `ChangeMap`, `PlayerStandUp` ou combinação deles.
- [ ] **Identidade do player:** verificar que o `entity_id` corrigido é o player local, não apenas `entities()[0]` por conveniência.
- [ ] **Ordem de eventos:** verificar que nenhum `RemoveEntity(Died)` ou outro handler chama `set_dead()` depois da correção.
- [ ] **Servidor ativo:** se rAthena foi alterado, verificar rebuild e que o container usa o binário novo.
- [ ] **Regressão mínima:** testar teleport normal, `@kill` sem respawn imediato, Respawn por botão e `@alive`.
- [ ] **Escopo:** confirmar que a correção não mexeu no bug separado de login/reconnect nem em protocolo fora do respawn.

## Estratégias de Recuperação

| Pitfall | Custo de recuperação | Passos de recuperação |
|---|---|---|
| Falso positivo por movimento/HP | MÉDIO | Reabrir fase 1, adicionar logs de animação e repetir `@kill -> Respawn` com captura curta |
| Entidade errada corrigida | MÉDIO | Introduzir identificação por `entity_id`, comparar com player local e remover dependência de índice no fluxo afetado |
| Correção no handler errado | MÉDIO | Usar captura de pacotes para mover a correção ao evento causal real ou alterar rAthena de modo mínimo |
| Regressão ampla por workaround visual | ALTO | Reverter resets genéricos, criar helper condicionado ao respawn do player local e repetir matriz manual |
| Mudança em rAthena não reproduzível | ALTO | Reaplicar no source ativo, forçar rebuild, criar patch e documentar commit/arquivo alterado |
| Instrumentação permanente ruidosa | BAIXO | Remover logs temporários ou proteger por feature debug antes de fechar a fase |

## Mapeamento Pitfall-para-Fase

| Pitfall | Fase de prevenção | Verificação |
|---|---|---|
| Falso positivo por movimento/HP | Fase 1 - Reprodução instrumentada | Evidência mostra HP, janela e `AnimationState` final no mesmo fluxo |
| Entidade errada por índice | Fase 2 - Identidade do player e helper de lifecycle | Logs provam `entity_id` local e correção não depende cegamente de `entities()[0]` |
| Esperar `ZC_RESURRECTION` quando há `ChangeMap` | Fase 1 e Fase 3 | Captura confirma pacote/evento real; correção atua no caminho confirmado |
| `set_dead` posterior sobrescrevendo `idle` | Fase 1 e Fase 4 | Log ordenado mostra última transição visual após respawn e nenhum `set_dead` tardio |
| Workaround visual amplo | Fase 3 e Fase 4 | Diff é pequeno, condicionado ao player local/respawn, e matriz manual passa |
| Editar backup/binário antigo | Fase 3 e Fase 4 | Status do repo ativo, rebuild confirmado e patch reprodutível atualizado se necessário |
| Logs temporários virarem dívida | Fase 1 e Fase 5 | Build normal sem ruído; logs restantes ficam atrás de debug ou removidos |

## Fases Recomendadas para o Roadmap

1. **Reprodução instrumentada e captura do fluxo real** - provar pacote, evento, entidade e ordem das transições antes de corrigir.
2. **Identidade do player e lifecycle mínimo** - isolar a seleção da entidade local e criar ponto único para transição morto -> vivo.
3. **Correção mínima comprovada** - aplicar no cliente ou rAthena conforme evidência, sem ampliar escopo.
4. **Validação manual e regressão direcionada** - cobrir Respawn por botão, `@alive`, teleport normal, morte sem respawn e janela.
5. **Limpeza e documentação da decisão** - remover instrumentação temporária, atualizar contexto e patch rAthena se houver mudança servidor.

## Fontes

- `.planning/PROJECT.md` - escopo, requisitos ativos e hipóteses conhecidas.
- `.planning/codebase/CONCERNS.md` - bugs conhecidos, áreas frágeis e lacunas de teste.
- `.planning/codebase/CONVENTIONS.md` - padrões locais de Rust/rAthena, logging e edição.
- `.planning/codebase/TESTING.md` - comandos e lacunas de validação automatizada.
- `AI_CONTEXT.md` - tentativas anteriores e hipóteses do bug de respawn.
- `korangar/korangar/src/main.rs` - handlers `ResurrectPlayer`, `RemoveEntity`, `ChangeMap`, `InputEvent::Respawn` e `UpdateEntityHealth`.
- `korangar/korangar/src/state/mod.rs` - `this_player()` e `this_entity()` atualmente baseados em `entities.first()`.
- `korangar/korangar/src/world/entity/mod.rs` - `set_dead`, `set_idle`, `stop_movement` e atualização de HP.
- `korangar/korangar-networking/src/packet_versions/version_20220406.rs` - mapeamento de `ResurrectionPacket`, `ChangeMapPacket` e `DamageType::StandUp`.
- `rathena-master/src/map/pc.cpp` - `pc_respawn` chama `clif_resurrection` apenas quando `pc_setpos` falha.
- `rathena-master/src/map/clif.cpp` - implementação de `clif_resurrection` / `ZC_RESURRECTION`.

---
*Pesquisa de pitfalls para: correção investigativa do bug de respawn visual Korangar + rAthena*  
*Pesquisado em: 2026-04-26*
