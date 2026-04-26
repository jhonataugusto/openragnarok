# Stack Research

**Domain:** correcao investigativa do bug de respawn visual no workspace Korangar + rAthena
**Researched:** 2026-04-26
**Confidence:** HIGH

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| Korangar client | Rust edition 2024, toolchain `nightly-2026-02-01` | Cliente desktop que renderiza entidades, processa pacotes e controla UI de morte/respawn | E o alvo real do bug visual; a correcao minima provavelmente passa por estado de entidade, handlers de `NetworkEvent` ou instrumentacao de pacotes no cliente. |
| rAthena server | C++17, commit local documentado `b70dd346ffc6cc16c1aeaad522913cdbdd1da42b` | Servidor `login`, `char` e `map` que decide morte, respawn e envio de pacotes | O fluxo de Respawn do servidor define se o cliente recebe `ZC_RESURRECTION` ou apenas uma mudanca de mapa. Nao alterar sem rebuild limpo. |
| Docker Compose + MariaDB | Docker Compose v2, MariaDB 11 | Laboratorio local reproduzivel para login, mapa, morte e respawn | A verificacao precisa acontecer nesse ambiente porque os pacotes e configs locais sao parte da compatibilidade Korangar + rAthena. |
| Ragnarok packet profile | `PACKETVER=20220406` | Contrato binario entre client e server | Deve permanecer fixo; trocar a versao invalida offsets, headers e handlers em `ragnarok-packets` e no build do rAthena. |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `korangar-networking` | local `0.1.0` | Conexao TCP Tokio, dispatch de pacotes e `NetworkEvent` | Use para instrumentar entrada/saida de pacotes e confirmar se `0x0148` chega apos Respawn. |
| `ragnarok-packets` | local `0.1.0` | Definicoes de pacotes, headers e conversao binaria | Use para validar `RestartPacket` `0x00B2`, `ChangeMapPacket` `0x0091` e `ResurrectionPacket` `0x0148`. |
| Tokio | `1.50.0` | Runtime async de networking do Korangar | Importante para logs de ordem dos eventos; nao e necessario trocar runtime para corrigir este bug. |
| wgpu | `29.0.0` | Renderizacao do cliente | So entra como restricao de build/runtime; o bug atual e de estado de animacao, nao de backend grafico, salvo evidencia contraria. |
| winit | `0.30.13` | Janela e event loop desktop | Mantem o loop principal onde `main.rs` processa eventos; nao e candidato primario para a correcao. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| `docker compose` | Subir, reiniciar e rebuildar rAthena/MariaDB | Rodar a partir de `D:\ragnarok`; `docker/entrypoint.sh` pula build se os binarios ja existem, entao mudancas em C++ podem exigir `docker compose run --rm builder rebuild`. |
| Cargo | Build/test do cliente Korangar | Rodar a partir de `D:\ragnarok\korangar`; para investigacao visual usar `cargo build --release --features debug` quando precisar das janelas de inspecao. |
| `play.bat` | Launcher local do cliente | Garante CWD `D:\ragnarok\korangar\korangar`, necessario para `data.grf`, `rdata.grf` e `archive/`. |
| `docker compose logs` | Evidencia de servidor | Usar principalmente `docker compose logs -f login char map` e logs filtrados do `map` durante `@kill -> Respawn`. |
| Wireshark ou `packet_callback` do Korangar | Captura de pacotes do map-server | Necessario se a fase precisar provar presenca/ausencia de `ZC_RESURRECTION` `0x0148` no port `5121`. |

## Installation

```powershell
# Servidor local
cd D:\ragnarok
docker compose up -d login char map

# Rebuild limpo do rAthena quando alterar C++ ou PACKETVER
docker compose stop login char map
docker compose run --rm builder rebuild
docker compose up -d login char map

# Cliente Korangar release
cd D:\ragnarok\korangar
cargo build --release

# Cliente com ferramentas de debug
cargo build --release --features debug

# Rodar cliente com CWD correto e assets locais
cd D:\ragnarok
.\play.bat

# Logs uteis durante reproducao
docker compose logs -f login char map
docker compose logs --no-color map
```

Pre-requisitos que afetam verificacao:

- `data.grf` e `rdata.grf` oficiais precisam existir em `D:\ragnarok\korangar\korangar`.
- `slangc.exe` precisa estar no `PATH`; `korangar/korangar/build.rs` falha sem ele.
- `nasm.exe` precisa estar no `PATH` para a dependencia `rav1d`.
- Docker precisa publicar `6900`, `6121` e `5121` no host conforme `docker-compose.yml`.
- A base MariaDB persistida em `rathena-db-data` pode carregar estado antigo; use `docker compose down -v` apenas quando a investigacao exigir reset de DB.

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| Corrigir primeiro no cliente, se o fluxo normal for `ChangeMap` sem `ZC_RESURRECTION` | Forcar `clif_resurrection(*sd)` em `pc_respawn` mesmo quando `pc_setpos` tem sucesso | Usar apenas se a captura provar que o cliente depende semanticamente de `ZC_RESURRECTION` e que enviar o pacote extra nao causa regressao em respawn normal. |
| Instrumentar `NetworkEvent` e estado da entidade no Korangar | Captura externa somente por Wireshark | Wireshark e util para prova de protocolo, mas nao mostra se `set_dead` sobrescreve `set_idle` depois do pacote. |
| Manter `PACKETVER=20220406` | Testar outra packet version | So considerar em pesquisa separada; mudar packet version muda o problema e quebra comparabilidade com o ambiente atual. |
| Rebuild limpo via `builder rebuild` | Reiniciar containers sem rebuild | Reinicio simples serve para config/import e logs, mas nao prova alteracao em C++ se os binarios antigos continuarem montados em `rathena-master/`. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| Editar `rathena-master.bak/` | E arvore duplicada/backup, nao o servidor ativo do Docker | Editar `rathena-master/`. |
| Assumir que `entities()[0]` e sempre o player | `main.rs` ainda tem logica sensivel a indice em morte/respawn; isso pode mascarar entidade errada | Usar `this_entity()` quando a intencao for o player local. |
| Trocar packet version para testar | Pode mudar headers, tamanhos e ordem de pacotes | Fixar `20220406` e investigar os pacotes dessa versao. |
| Corrigir por delay/timer visual | Pode esconder ordem errada de eventos e deixar regressao intermitente | Provar sequencia de `RemoveEntity`, `ResurrectPlayer`, `ChangeMap` e updates de HP/estado. |
| Rebuild parcial do rAthena apos C++ | `docker/entrypoint.sh` pula build quando `login-server`, `char-server` e `map-server` existem | Usar `docker compose run --rm builder rebuild`. |
| Rodar `korangar.exe` de outro diretorio | O cliente depende de paths relativos para GRFs e `archive/` | Usar `play.bat` ou CWD `D:\ragnarok\korangar\korangar`. |

## Stack Patterns by Variant

**Se a captura mostrar `ZC_RESURRECTION` `0x0148` chegando:**

- Corrigir no cliente em `korangar/korangar/src/main.rs`, no handler `NetworkEvent::ResurrectPlayer`.
- Confirmar que o `entity_id` do pacote bate com `this_entity()` e que nenhuma atualizacao posterior chama `set_dead`.
- Arquivos envolvidos: `korangar/korangar-networking/src/packet_versions/version_20220406.rs`, `korangar/ragnarok-packets/src/lib.rs`, `korangar/korangar/src/world/entity/mod.rs`.

**Se o Respawn normal vier apenas como `ChangeMap` `0x0091`:**

- Tratar `NetworkEvent::ChangeMap` como reset implicito de morte para o player local.
- Preferir localizar o player por `this_entity()` antes/depois de truncar entidades, ou provar que a truncagem preserva o player correto.
- Arquivos envolvidos: `korangar/korangar/src/main.rs`, `korangar/korangar/src/world/entity/mod.rs`, `rathena-master/src/map/pc.cpp`.

**Se `set_dead` acontecer depois de `set_idle`:**

- A causa esta na ordem dos eventos ou em status/HP posterior, nao no pacote de respawn em si.
- Instrumentar `NetworkEvent::RemoveEntity`, handlers de HP/status e abertura de `RespawnWindow`.
- Arquivos envolvidos: `korangar/korangar/src/main.rs` linhas onde `set_dead`, `set_idle`, `WindowClass::Respawn` e `RespawnWindow` aparecem.

**Se a janela de Respawn continuar aberta mas a animacao voltar ao normal:**

- Separar bug de UI de bug de entidade.
- Corrigir fechamento/reabertura da janela no cliente sem alterar rAthena.
- Arquivos envolvidos: `korangar/korangar/src/main.rs`, especialmente `InputEvent::Respawn`, `NetworkEvent::ResurrectPlayer`, `NetworkEvent::ChangeMap` e handlers que abrem `RespawnWindow`.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| rAthena Docker `PACKETVER=20220406` | Korangar `SupportedPacketVersion::_20220406` | `docker-compose.yml` define `PACKETVER: "20220406"` e `korangar-networking` registra handlers dessa versao. |
| rAthena `pc_respawn` | Korangar `ChangeMapPacket` `0x0091` | Em `rathena-master/src/map/pc.cpp`, `pc_respawn` chama `pc_setpos`; se o warp ao save point tem sucesso, nao envia `clif_resurrection`. |
| rAthena `clif_resurrection` | Korangar `ResurrectionPacket` `0x0148` | `clif_resurrection` monta `ZC_RESURRECTION`, mas no respawn normal so aparece no fallback de falha do warp. |
| Korangar `NetworkingSystem::respawn()` | rAthena `clif_parse_Restart` | O cliente envia `RestartPacket` `0x00B2` com `RestartType::Respawn`; o servidor chama `pc_respawn(sd, CLR_OUTSIGHT)`. |
| Rust `nightly-2026-02-01` | Korangar workspace edition 2024 | Usar a toolchain pinada por `korangar/rust-toolchain.toml`; build.rs valida nightly e `slangc`. |
| Docker bind mount `./rathena-master:/rathena` | Binarios `login-server`, `char-server`, `map-server` no host | Fonte e binarios compartilham diretorio; stale binary pode quebrar verificacao se nao houver rebuild limpo. |

## Protocol and Packet Constraints

- Perfil obrigatorio: `PACKETVER=20220406`. O Docker passa esse valor para `./configure --enable-packetver="${PACKETVER}"`; o fallback de `rathena-master/src/config/packets.hpp` e `20211103`, entao confiar no fallback e erro.
- Pacote de request de respawn: `RestartPacket` `0x00B2` em `korangar/ragnarok-packets/src/lib.rs`; enviado por `NetworkingSystem::respawn()` em `korangar/korangar-networking/src/lib.rs`.
- Resposta do menu de morte no servidor: `clif_parse_Restart` em `rathena-master/src/map/clif.cpp` chama `pc_respawn(sd, CLR_OUTSIGHT)` para tipo `0`.
- Pacote de ressurreicao: `ResurrectionPacket` `0x0148`, registrado para virar `NetworkEvent::ResurrectPlayer` em `korangar/korangar-networking/src/packet_versions/version_20220406.rs`.
- Mudanca de mapa: `ChangeMapPacket` `0x0091`, registrado para virar `NetworkEvent::ChangeMap`; esse e o caminho mais provavel do respawn via save point.
- Restricao critica: em `rathena-master/src/map/pc.cpp`, `pc_respawn` chama `clif_resurrection(*sd)` apenas quando `pc_setpos(...) != SETPOS_OK`. Portanto, ausencia de `0x0148` no respawn normal pode ser comportamento esperado do servidor, nao perda de pacote.

## Client/Server Files Likely Involved

| File | Why It Matters |
|------|----------------|
| `korangar/korangar/src/main.rs` | Hub dos handlers `NetworkEvent::ResurrectPlayer`, `NetworkEvent::RemoveEntity`, `NetworkEvent::ChangeMap` e `InputEvent::Respawn`; principal local de correcao minima no cliente. |
| `korangar/korangar/src/world/entity/mod.rs` | Implementa `Entity::set_dead`, `Entity::set_idle` e `Entity::stop_movement`; confirma se idle realmente muda `animation_state`. |
| `korangar/korangar-networking/src/lib.rs` | Envia `RestartPacket` no `respawn()` e possui `packet_callback`; ponto bom para instrumentar pacotes de map-server. |
| `korangar/korangar-networking/src/packet_versions/version_20220406.rs` | Mapeia `ChangeMapPacket` e `ResurrectionPacket` para eventos do cliente no perfil de protocolo atual. |
| `korangar/ragnarok-packets/src/lib.rs` | Fonte dos headers e structs: `ChangeMapPacket` `0x0091`, `ResurrectionPacket` `0x0148`, `RestartPacket` `0x00B2`. |
| `rathena-master/src/map/pc.cpp` | Implementa `pc_respawn`; decide quando chamar `pc_setstand`, `pc_setrestartvalue`, `pc_setpos` e `clif_resurrection`. |
| `rathena-master/src/map/clif.cpp` | Implementa `clif_resurrection` e `clif_parse_Restart`; liga o pacote do cliente ao fluxo de respawn do servidor. |
| `docker-compose.yml` | Define servicos, portas e `PACKETVER=20220406`. |
| `docker/entrypoint.sh` | Copia configs, compila rAthena e pode pular build se binarios ja existirem. |
| `korangar/korangar/archive/data/sclientinfo.xml` | Aponta o cliente para `127.0.0.1:6900`; mudancas aqui podem quebrar reproducao. |
| `play.bat` | Garante CWD correto para assets e executavel release. |

## Verification Environment Assumptions

- Host principal: Windows em `D:\ragnarok`, com PowerShell.
- Servidor local: containers `rathena-db`, `rathena-login`, `rathena-char`, `rathena-map`, `rathena-builder`.
- Portas esperadas: MariaDB `3306`, login `6900`, char `6121`, map `5121`.
- Conta de teste disponivel no seed local: `admin` / `123` ou `jonato` / `jonato`.
- Repro minimo: entrar no jogo, usar `@kill`, clicar Respawn, observar se HP/movimento voltam e se sprite sai da pose de morto.
- Comandos GM estao habilitados no ambiente local por patch; isso e uma premissa de laboratorio, nao uma configuracao segura.
- Antes de editar, verificar dirty state nos repositorios aninhados com `git -C korangar status --short` e `git -C rathena-master status --short`; ja existem modificacoes locais e artefatos gerados.
- A verificacao visual e manual por enquanto; nao ha teste E2E automatizado para morte/respawn.

## Sources

- `.planning/PROJECT.md` - escopo, requisitos ativos, constraints e decisoes do milestone.
- `.planning/codebase/STACK.md` - stack existente, versoes e comandos de build/runtime.
- `.planning/codebase/INTEGRATIONS.md` - topologia Docker, portas, DB, auth e integracoes.
- `.planning/codebase/CONCERNS.md` - bugs conhecidos, areas frageis e gaps de teste.
- `AI_CONTEXT.md` - contexto operacional, reproducao, tentativas anteriores e comandos.
- `docker-compose.yml` - `PACKETVER`, servicos, portas e volumes.
- `docker/entrypoint.sh` - comportamento de build/rebuild e configuracao efetiva.
- `korangar/korangar/src/main.rs` - handlers de morte, respawn, mudanca de mapa e UI.
- `korangar/korangar/src/world/entity/mod.rs` - API de estado de animacao e movimento.
- `korangar/korangar-networking/src/lib.rs` - envio de respawn e conexao map-server.
- `korangar/korangar-networking/src/packet_versions/version_20220406.rs` - registro de pacotes para `20220406`.
- `korangar/ragnarok-packets/src/lib.rs` - headers e structs de pacotes.
- `rathena-master/src/map/pc.cpp` - fluxo `pc_respawn`.
- `rathena-master/src/map/clif.cpp` - `clif_parse_Restart` e `clif_resurrection`.

---
*Stack research for: correcao investigativa do bug de respawn visual Korangar + rAthena*
*Researched: 2026-04-26*
