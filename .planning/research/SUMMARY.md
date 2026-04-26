# Project Research Summary

**Project:** Ragnarok Respawn Fix
**Domain:** correcao investigativa brownfield do respawn visual Korangar + rAthena
**Researched:** 2026-04-26
**Confidence:** MEDIUM

## Executive Summary

Este projeto nao e um produto novo: e uma correcao investigativa em um workspace brownfield que integra o cliente Korangar em Rust, o servidor rAthena em C++, Docker Compose, MariaDB e o perfil de protocolo `PACKETVER=20220406`. O objetivo e resolver um bug especifico: depois de `@kill -> Respawn`, o personagem recupera HP e movimento, mas pode continuar visualmente morto e/ou manter a janela de respawn aberta.

A abordagem recomendada e o caminho minimo comprovado: reproduzir o bug no ambiente local, coletar evidencia curta e objetiva do fluxo de pacotes/eventos/entidade, identificar se a causa esta no cliente, no servidor ou na integracao, aplicar a menor correcao segura e validar manualmente `@kill -> Respawn`. As pesquisas convergem que o fluxo normal de respawn por save point no rAthena provavelmente chega ao cliente como `ChangeMap`, nao necessariamente como `ZC_RESURRECTION`, portanto corrigir apenas `NetworkEvent::ResurrectPlayer` tende a ser insuficiente.

Os principais riscos sao falso positivo por validar apenas HP/movimento, corrigir a entidade errada por depender de `entities()[0]`, patchar rAthena antes de provar a causa e espalhar workarounds visuais em handlers genericos. A mitigacao e tratar evidencia como requisito: registrar pacote/evento, `entity_id`, entidade local, `AnimationState`/`is_dead`, HP e estado da `WindowClass::Respawn` antes de alterar comportamento permanente.

## Key Findings

### Recommended Stack

O stack existente deve ser preservado. O bug vive no contrato entre Korangar, `korangar-networking`, `ragnarok-packets` e rAthena; trocar packet version, runtime, renderer ou infraestrutura ampliaria o problema e destruiria comparabilidade com o ambiente onde o bug foi observado.

**Core technologies:**
- Korangar client: cliente Rust edition 2024 com toolchain `nightly-2026-02-01` - alvo principal para estado visual, handlers de `NetworkEvent` e UI de respawn.
- rAthena server: servidor C++17 no commit local documentado - decide morte, restart, `pc_respawn`, `pc_setpos` e envio condicional de `clif_resurrection`.
- Docker Compose + MariaDB 11: laboratorio local reproduzivel - deve ser usado para login, mapa, `@kill` e Respawn reais.
- `PACKETVER=20220406`: contrato binario cliente/servidor - deve permanecer fixo durante a correcao.
- `korangar-networking` + `ragnarok-packets`: fronteira de protocolo - ponto certo para confirmar `RestartPacket 0x00B2`, `ChangeMapPacket 0x0091` e `ResurrectionPacket 0x0148`.

### Expected Features

O MVP deve ser tratado como um fluxo de investigacao e correcao, nao como uma entrega ampla de ferramentas. Tudo que nao ajuda diretamente a provar e corrigir `@kill -> Respawn` deve ser diferido.

**Must have (table stakes):**
- Reproducao controlada - subir rAthena local, abrir Korangar corretamente e reproduzir o bug antes de mudar codigo.
- Roteiro de reproducao - registrar passos, conta/personagem se relevante, comando `@kill`, clique Respawn e resultado visual.
- Evidencia do fluxo real - saber se o cliente recebeu `ResurrectPlayer`, `ChangeMap`, status/HP, `RemoveEntity(Died)` ou combinacao.
- Evidencia da entidade local - confirmar que a entidade alterada e a renderizada como player local, sem depender cegamente de `entities()[0]`.
- Evidencia da transicao visual - provar que o estado final nao e morto visualmente, nao apenas que HP/movimento voltaram.
- Correcao minima localizada - alterar o ponto causal comprovado no cliente ou servidor.
- UI de respawn consistente - janela fecha e nao reabre depois do respawn.
- Verificacao manual final - `@kill -> Respawn` termina com player em pe/vivo, HP restaurado, movimento funcional e janela fechada.

**Should have (pragmatic differentiators):**
- Instrumentacao temporaria com prefixos claros - acelera diagnostico sem virar ferramenta permanente.
- Teste unitario pequeno - somente se a correcao extrair helper puro de lifecycle.
- Comparacao com `@alive` - util para diferenciar resurrection explicita de respawn por save point.
- Evidencia antes/depois em texto ou screenshot curta - facilita revisao humana.

**Defer (v2+):**
- Suite E2E completa de login, mapa, morte e respawn - alto custo e sem harness pronto.
- Packet tracing permanente - util no futuro, mas maior que a correcao minima.
- Refatoracao ampla de `korangar/korangar/src/main.rs` - desejavel depois, arriscada agora.
- Matriz ampla de compatibilidade de packet version - fora do escopo do bug pontual.
- Resolver todos os pacotes nao implementados - corrigir apenas o que for causal para respawn.

### Architecture Approach

A arquitetura relevante e um pipeline simples que precisa ser observado em ordem: pacote bruto do rAthena vira pacote tipado em `ragnarok-packets`, vira `NetworkEvent` em `korangar-networking`, muda `ClientState`/entidades/UI em `main.rs`, e o renderer apenas reflete o estado final. A correcao deve ficar perto da fronteira causal comprovada: normalmente `main.rs` e lifecycle de entidade no cliente; rAthena apenas se a captura provar que o servidor precisa emitir sinal adicional.

**Major components:**
1. Korangar `main.rs` - coordena input, eventos de rede, mapa, entidades, janela Respawn e chamadas de networking.
2. Entity/animation lifecycle - `set_dead`, `set_idle`, `stop_movement`, HP e `AnimationState`.
3. Player identity path - `this_entity()`/`this_player()` e a premissa fragil de `entities.first()`.
4. Korangar networking + packet definitions - enviam `RestartPacket` e convertem `ChangeMap`/`Resurrection` em eventos.
5. rAthena `clif.cpp`/`pc.cpp` - recebe `CZ_RESTART`, executa `pc_respawn` e so chama `clif_resurrection` quando `pc_setpos` falha.
6. Respawn UI - exibe a janela, emite `InputEvent::Respawn` e deve ser fechada quando o player realmente deixa o estado morto.

### Critical Pitfalls

1. **Validar apenas HP ou movimento** - evitar exigindo evidencia de `AnimationState`/`is_dead` e janela Respawn no estado final.
2. **Corrigir a entidade errada por `entities()[0]`** - evitar registrando `entity_id` local e centralizando a selecao do player.
3. **Esperar sempre `ZC_RESURRECTION`** - evitar capturando o fluxo real; respawn por save point pode chegar como `ChangeMap`.
4. **`set_dead` posterior sobrescrever `set_idle`** - evitar logando ordem temporal de `RemoveEntity`, `ChangeMap`, `ResurrectPlayer`, HP/status e UI.
5. **Workaround visual amplo** - evitar condicionando o fix ao player local e ao fluxo de respawn comprovado.
6. **Alterar rAthena errado ou testar binario antigo** - evitar editando `rathena-master/`, fazendo rebuild limpo e confirmando logs/marker quando C++ mudar.

## Implications for Roadmap

Based on research, suggested phase structure:

### Phase 1: Reproducao e Baseline Manual
**Rationale:** Nao ha correcao confiavel sem confirmar que o workspace atual ainda reproduz o bug real.
**Delivers:** roteiro de reproducao, ambiente validado, resultado antes da correcao e criterio visual de aceite.
**Addresses:** reproducao controlada, roteiro explicito, validacao visual inicial.
**Avoids:** falso positivo por assumir que o bug esta igual ao relato antigo.

### Phase 2: Evidencia de Pacotes, Eventos e Entidade
**Rationale:** As tentativas anteriores em `ResurrectPlayer` e `ChangeMap` nao resolveram; a proxima mudanca precisa saber qual evento e qual entidade causam o estado final.
**Delivers:** logs/captura curta com `RestartPacket`, `ChangeMap`, `ResurrectPlayer`, `RemoveEntity(Died)`, HP/status, `entity_id`, `this_entity()`, `is_dead` e estado da janela.
**Uses:** `korangar-networking`, `ragnarok-packets`, `main.rs`, possivelmente captura no port `5121`.
**Implements:** padrao pacote -> evento -> mutacao visual.
**Avoids:** corrigir entidade errada, depender de `ZC_RESURRECTION` sem prova e perder sobrescrita tardia por `set_dead`.

### Phase 3: Decisao Cliente vs Servidor e Fix Minimo
**Rationale:** A correcao deve seguir a evidencia: se o sinal real e `ChangeMap`, preferir fix cliente condicionado ao respawn do player local; se o contrato exige resurrection ausente, avaliar patch minimo em rAthena.
**Delivers:** alteracao pequena no ponto causal comprovado, com instrumentacao temporaria removida ou protegida.
**Implements:** helper minimo de lifecycle do player local, ajuste em `ChangeMap`/`ResurrectPlayer`, ou patch restrito em `pc_respawn` se inevitavel.
**Avoids:** workaround visual amplo, refatoracao de `main.rs`, mudanca preventiva de protocolo e patch servidor sem rebuild comprovado.

### Phase 4: Verificacao Manual Direcionada
**Rationale:** O bug e visual/interativo; testes unitarios isolados nao provam o fluxo real.
**Delivers:** validacao `@kill -> Respawn` com player em pe/vivo, HP restaurado, movimento funcional e janela fechada; regressao minima para `@alive`, teleport normal e morte sem respawn imediato se aplicavel.
**Addresses:** verificacao manual principal, UI de respawn consistente e evidencia pos-correcao.
**Avoids:** concluir com base apenas em logs ou movimento.

### Phase 5: Limpeza e Registro da Causa
**Rationale:** A pesquisa mostra alto risco de logs temporarios, patches soltos e conclusoes perdidas.
**Delivers:** remocao/reducao de instrumentacao, registro da causa provavel, arquivos alterados, evidencia usada e notas sobre divida restante.
**Addresses:** registro do resultado e causa provavel, documentacao operacional.
**Avoids:** ruido permanente, patch rAthena nao reproduzivel e perda de contexto para fases futuras.

### Phase Ordering Rationale

- Reproducao vem primeiro porque o criterio principal e visual e manual.
- Evidencia vem antes da correcao porque o bug pode ser pacote ausente, entidade errada ou ordem de eventos.
- Cliente deve ser preferido para fix inicial quando `ChangeMap` for o fluxo normal comprovado; servidor so entra se o contrato de protocolo realmente exigir.
- Verificacao manual vem depois do fix porque HP/movimento nao provam a animacao final.
- Limpeza fecha o ciclo para impedir que a investigacao temporaria vire divida permanente.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2:** pesquisar durante o plano se a captura interna do Korangar for insuficiente; pode exigir Wireshark ou uso cuidadoso de `packet_callback`.
- **Phase 3:** pesquisar durante o plano se a evidencia apontar para patch em rAthena; precisa confirmar semantica de `pc_respawn`, rebuild Docker e impacto de `clif_resurrection`.

Phases with standard patterns (skip research-phase):
- **Phase 1:** reproducao manual local ja esta bem documentada; planejar comandos e criterio de aceite basta.
- **Phase 4:** verificacao manual focada segue criterios claros dos relatorios.
- **Phase 5:** limpeza/documentacao e padrao GSD normal; nao precisa pesquisa adicional.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Baseado em arquivos locais, comandos reais, `PACKETVER=20220406`, Docker Compose e estrutura Korangar/rAthena. |
| Features | HIGH | O escopo e estreito e bem definido: reproduzir, evidenciar, corrigir minimo e validar manualmente. |
| Architecture | MEDIUM | O fluxo de codigo esta bem mapeado, mas a causa final ainda depende de captura/reproducao manual. |
| Pitfalls | HIGH | Riscos foram derivados de codigo local, tentativas anteriores e areas frageis conhecidas. |

**Overall confidence:** MEDIUM

### Gaps to Address

- Causa final ainda nao comprovada: resolver na Phase 2 com captura ordenada de pacote/evento/entidade.
- Identidade estavel do player local ainda e fragil: validar `entity_id` e reduzir dependencia de `entities()[0]` no fluxo de respawn.
- Presenca ou ausencia de `ZC_RESURRECTION` no respawn por botao precisa ser confirmada no ambiente atual.
- Se rAthena for alterado, e obrigatorio provar rebuild limpo e evitar editar backup ou patch solto.
- Nao ha E2E automatizado para o fluxo; a aceitacao v1 deve permanecer manual e objetiva.

## Sources

### Primary (HIGH confidence)
- `.planning/PROJECT.md` - escopo, valor central, requisitos ativos e fora de escopo.
- `.planning/research/STACK.md` - stack, versoes, comandos, compatibilidade e arquivos envolvidos.
- `.planning/research/FEATURES.md` - capacidades MVP, diferenciadores e anti-features.
- `.planning/research/PITFALLS.md` - riscos criticos, sinais de alerta e mitigacoes por fase.
- `docker-compose.yml` - servicos, portas e `PACKETVER=20220406`.
- `korangar/korangar/src/main.rs` - handlers de input, rede, morte, respawn e mudanca de mapa.
- `korangar/korangar/src/world/entity/mod.rs` - lifecycle de entidade.
- `korangar/korangar-networking/src/lib.rs` - envio de `RestartPacket`.
- `korangar/ragnarok-packets/src/lib.rs` - headers e structs de pacotes.
- `rathena-master/src/map/pc.cpp` - fluxo `pc_respawn`.
- `rathena-master/src/map/clif.cpp` - `clif_parse_Restart` e `clif_resurrection`.

### Secondary (MEDIUM confidence)
- `.planning/research/ARCHITECTURE.md` - modelo arquitetural e fluxo provavel cliente/servidor, ainda dependente da captura final.
- `.planning/codebase/CONCERNS.md` - areas frageis e lacunas de teste.
- `.planning/codebase/TESTING.md` - ausencia de E2E e padroes atuais de verificacao.
- `AI_CONTEXT.md` - reproducao historica, hipoteses e tentativas anteriores.

### Tertiary (LOW confidence)
- Hipoteses de ordem de eventos, reabertura da janela e sobrescrita tardia por `set_dead` - devem ser comprovadas na Phase 2 antes de orientar o fix final.

---
*Research completed: 2026-04-26*
*Ready for roadmap: yes*
