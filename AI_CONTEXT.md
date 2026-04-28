# Contexto do Projeto Ragnarok (Korangar + rAthena)

> **Para IAs assistentes:** este documento é um briefing sobre o estado atual do projeto e os bugs já investigados. Use como base antes de propor mudanças. Coisas marcadas como **NÃO RESOLVIDO** ainda precisam de trabalho.

---

## Stack

- **Cliente:** [Korangar](https://github.com/ve5li/korangar) (Rust) — reimplementação open-source do client de Ragnarok Online.
  - Fonte local: `D:\ragnarok\korangar\`
  - Binário: `D:\ragnarok\korangar\target\release\korangar.exe`
  - **CWD obrigatório ao rodar:** `D:\ragnarok\korangar\korangar\` (precisa achar `data.grf` e `rdata.grf` por caminho relativo)
  - GRFs: `data.grf` e `rdata.grf` oficiais do kRO devem estar em `korangar\korangar\`
- **Servidor:** [rAthena](https://github.com/rathena/rathena) (C++) rodando em Docker.
  - Fonte local: `D:\ragnarok\rathena-master\` — clonado no commit `b70dd346ffc6cc16c1aeaad522913cdbdd1da42b`, que é o exato pinned pelos devs do Korangar em [korangar-rathena](https://github.com/vE5li/korangar-rathena).
  - 10 patches custom de `korangar-rathena/patches/` aplicados (at-command, attack-speed, character-deletion, disable-pin-code, drop-rates, new-account, packet-obfuscation, unlimited-slot-moves, area-size, experience).
  - Config Docker: `D:\ragnarok\docker-compose.yml`, `D:\ragnarok\docker\entrypoint.sh`, `D:\ragnarok\docker\import\*.txt`
  - Packet version: `PACKETVER 20220406`
  - Serviços: `mariadb`, `login`, `char`, `map`, `builder`
- **Launcher:** `D:\ragnarok\play.bat` (faz `cd` correto e roda o exe)

## Como subir tudo

```powershell
# Servidor
cd D:\ragnarok
docker compose up -d login char map

# Cliente
.\play.bat
```

## Como ver logs do servidor

```powershell
cd D:\ragnarok

# Live tail
docker compose logs -f login char map

# Snapshot completo
docker compose logs --no-color > logs-all.txt

# Filtrado
docker compose logs --no-color login | Select-String "REFUSED|Closed|Request"
```

## Como acessar o banco

Container: `rathena-mariadb`. DB: `ragnarok`. User/pass: `ragnarok` / `ragnarok`.

```powershell
docker compose exec mariadb mariadb -uragnarok -pragnarok ragnarok
```

Tabelas-chave: `login` (contas), `char` (personagens), `inventory`, `mob_db`, etc. Em rAthena moderno, `auth_node` é **em memória apenas**, não em DB — para limpar é só reiniciar o login-server.

## Contas de teste

| account_id | userid | senha | sex |
|---|---|---|---|
| 1 | s1 | p1 | S (servidor) |
| 2000000 | admin | 123 | M |
| 2000001 | jonato | jonato | M |

---

## Bugs e Patches

### 1. RFIFOREST batching (rAthena map-server) — **RESOLVIDO**

**Sintoma:** char-server aceitava login, mas ao clicar em selecionar personagem nada acontecia (som de confirmação + flicker, sem progresso).

**Causa:** `clif_parse_WantToConnection_sub` em `src/map/clif.cpp` validava o tamanho do pacote de conexão com `RFIFOREST(fd) != packet_db[cmd].len`. `RFIFOREST` retorna o **buffer inteiro**, e Korangar envia o packet de conexão (23 bytes) imediatamente seguido de outro pacote no mesmo segmento TCP, então `RFIFOREST` era 29 e a comparação falhava.

**Fix aplicado** em `D:\ragnarok\rathena-master\src\map\clif.cpp` (linhas ~10568-10575): trocou `!=` por `<` (mínimo, não exato). Já está commitado e o builder Docker recompila com o patch.

```cpp
// We only need to ensure the buffer contains AT LEAST the
// expected packet length; the rest stays in the FIFO for the next
// parse iteration.
if( packet_len < packet_db[cmd].len )
```

> Esse patch precisa ser **re-aplicado** se o `rathena-master` for re-clonado.

### 2. Auto-reconnect destruindo auth_node (Korangar) — **RESOLVIDO**

**Sintoma:** após login bem-sucedido, ao clicar no botão "rAthena" na tela de seleção de servidor, o login-server logava `Char-server 'rAthena': authentication of the account 2000000 REFUSED`. O `admin` quebrava sempre, `jonato` às vezes funcionava (timing).

**Causa:** o handler `NetworkEvent::LoginServerDisconnected` em `korangar/korangar/src/main.rs` reconectava transparentemente em qualquer `reason != ClosedByClient`. Por motivo ainda não 100% identificado o socket de login caía pouco depois do `LoginServerLoginSuccess`, disparando o reconnect. Cada reconexão passava pelo branch em `loginclif.cpp` ~linhas 100-107:

```c
else if( data->char_server == -1 ) {
    // wipe previous session
    login_remove_auth_node(sd->account_id);
    login_remove_online_user(sd->account_id);
}
```

Resultado: o `auth_node` válido era apagado, e quando o usuário finalmente clicava no servidor, o char-server pedia auth ao login com `login_id1/login_id2` antigos → `REFUSED`.

**Fix aplicado** em `D:\ragnarok\korangar\korangar\src\main.rs` (`NetworkEvent::LoginServerDisconnected`): em vez de reconectar, fecha as janelas, abre a `LoginWindow` e mostra um `ErrorWindow` "Lost connection to the login server". Usuário re-faz login manualmente — `auth_node` fica intacto.

**Causa subjacente do TCP cair ainda é desconhecida.** Hipóteses:
- Race no `tokio::select!` entre `interval.tick()` (keepalive) e `action_receiver.recv()` na primeira iteração de `handle_server_connection` — primeira tick do `tokio::time::interval` completa imediatamente.
- Algum pacote inesperado pós-`LoginServerLoginSuccess` causa `received_bytes == 0`.
- rAthena fechando por algum motivo de configuração.

Se quiser investigar a fundo: `korangar/korangar-networking/src/lib.rs` linhas ~281-387 (`handle_server_connection`).

### 3. Personagem preso na animação de morto após respawn — **NÃO RESOLVIDO**

**Sintoma:** ao morrer (`@kill` ou em PvP) e respawnar (botão Respawn ou `@alive`), o personagem fica visualmente preso na pose de "morto" no chão, mesmo com HP cheio e podendo se mover. Janela de respawn às vezes também não fecha.

**Tentativas que NÃO resolveram (mas estão commitadas em `main.rs`):**

- `NetworkEvent::ResurrectPlayer`: agora chama `entity.set_idle(client_tick)` + `entity.stop_movement()` na entidade ressuscitada (além de fechar `WindowClass::Respawn`).
- `NetworkEvent::ChangeMap`: agora fecha `WindowClass::Respawn` e chama `set_idle()` + `stop_movement()` no primeiro elemento de `entities()` (assumido como o player).

**Por que provavelmente ainda falha** (hipóteses para o próximo investigador):

1. **O player não é `entities()[0]`.** Pode ser que o player real esteja em outro estado (`this_entity()` resolvido por path, não por índice). Verificar com `try_follow(this_entity())` em vez de `.first_mut()`.
2. **`set_idle` é chamado mas o estado de animação tem precedência diferente.** Olhar `korangar/korangar/src/world/entity/animation.rs` (ou similar) para ver se `dead → idle` é uma transição válida ou se precisa de algo tipo `revive()` explícito.
3. **`set_dead` é chamado depois do `ChangeMap`.** Pode haver outro pacote (status update, HP packet) que vem em sequência e re-marca o player como morto. Verificar `NetworkEvent::UpdateStatus`/handlers de HP em `main.rs` linhas ~1199-1210 (que já chamam `set_dead` quando HP=0).
4. **Janela de Respawn é aberta de novo após `ChangeMap`.** Ver onde `WindowClass::Respawn` é aberta — provavelmente algum handler de status que também precisa ser ajustado.

**Repro:**
1. Entrar no jogo
2. `@kill`
3. Clicar Respawn (ou esperar `@alive` de outro player) → visualmente fica deitado.

**Onde olhar:**

- Animation state machine: `korangar/korangar/src/world/entity/mod.rs` linha 1331 (`set_dead`) e 1336 (`set_idle`). Ver o que `animation_state.dead()` e `animation_state.idle()` fazem internamente.
- Handlers que setam `set_dead`: `main.rs` linhas ~1199, ~1210 (provavelmente em `UpdateStatus` ou `EntityDamage`).
- Possivelmente o problema é no servidor: rAthena pode não estar enviando `ZC_RESURRECTION` (0x0148) ao respawnar via save_point, só `ZC_NPCACK_MAPMOVE` (que vira `ChangeMap`). Olhar `pc_setpos` / `pc_respawn` em `src/map/pc.cpp`.

---

## Comandos GM úteis

Ver `D:\ragnarok\GM_COMMANDS.md` para a lista completa. Por padrão **todos** os usuários têm acesso a `@`-comandos (patch `at-command.patch`).

Mais usados durante debug:
- `@kill` / `@alive` — matar/reviver
- `@warp prontera 156 191` — teleporta
- `@go 0` — Prontera (atalho)
- `@battleignore` — ficar invulnerável
- `@item <id> <qty>` — spawnar item
- `@reloadscript` — recarrega NPCs
- `@dropall` — dropa tudo do inventário

---

## Workflow recomendado para a próxima sessão

1. Reproduzir o bug do respawn com o cliente atual.
2. Habilitar a feature `debug` no Korangar (cargo build com `--features debug`) — abre janelas de inspeção do estado das entidades.
3. Logar `AnimationState` antes/depois do ResurrectPlayer/ChangeMap pra confirmar se `set_idle` está realmente sendo aplicado.
4. Capturar pacotes do servidor pós-respawn — wireshark no port 5121, ou habilitar `packet_callback` do Korangar — pra ver se o servidor envia `ZC_RESURRECTION`.
5. Se rAthena não envia `ZC_RESURRECTION` em respawn-via-save-point (que é o comportamento padrão), corrigir lá em `pc.cpp` ou tratar `ChangeMap` como reset implícito de morte no cliente — incluindo refazer o body completamente, não só `set_idle`.

---

## Arquivos-chave

| Arquivo | O que tem |
|---|---|
| `korangar/korangar/src/main.rs` | Loop principal, handlers de `NetworkEvent` e `InputEvent` |
| `korangar/korangar-networking/src/lib.rs` | Tasks tokio, `handle_server_connection`, reconnect logic |
| `korangar/ragnarok-packets/src/lib.rs` | Definições de pacotes (headers, structs) |
| `korangar/korangar/src/world/entity/mod.rs` | `Entity::set_idle`, `set_dead`, `stop_movement` |
| `rathena-master/src/login/loginclif.cpp` | Login server, `logclif_auth_ok`, wipe de auth_node |
| `rathena-master/src/login/loginchrif.cpp` | Login ↔ char comm, `logchrif_parse_reqauth` |
| `rathena-master/src/char/char_clif.cpp` | Char server connect, request_to_connect |
| `rathena-master/src/map/clif.cpp` | Map server, packet parsing (RFIFOREST patch aqui) |
| `rathena-master/src/map/pc.cpp` | Player char logic — respawn, morte, save point |
| `docker-compose.yml` / `docker/entrypoint.sh` | Orquestração |

