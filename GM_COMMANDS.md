# Comandos GM — rAthena

Referência rápida dos comandos GM mais usados no rAthena.

- Todos com prefixo `@` (executados no chat do jogo).
- Para mirar em outro jogador, use `#comando "Nome do Personagem"` em vez de `@`.
- Permissões ficam em `rathena-master/conf/groups.yml`. Grupo 99 normalmente tem tudo liberado.
- Lista todos os comandos disponíveis pro seu nível: `@commands`
- Ajuda de um comando específico: `@help <comando>`

---

## Personagem (você)

| Comando | Descrição |
|---|---|
| `@kill` | Mata você na hora |
| `@alive` | Ressuscita |
| `@heal` | Restaura HP/SP totalmente |
| `@jobchange <id\|nome>` | Troca de classe (ex: `@jobchange 4060` = Lord Knight) |
| `@blvl <n>` | Adiciona base level (negativo subtrai) |
| `@jlvl <n>` | Adiciona job level |
| `@stpoint <n>` | Ganha pontos de status |
| `@skpoint <n>` | Ganha pontos de skill |
| `@allskill` | Aprende todas as skills da classe atual |
| `@reset` | Zera stats e skills |
| `@option <o1> <o2> <o3>` | Efeitos visuais (asas, montaria etc.) |
| `@speed <0-1000>` | Velocidade de movimento (`0` = instantâneo, `-1` reseta) |
| `@hide` / `@invisible` | Invisível pra mobs |
| `@battleignore` | Mobs param de te atacar |

## Movimento / mapa

| Comando | Descrição |
|---|---|
| `@warp <mapa> <x> <y>` | Teleporta para coordenada (ex: `@warp prontera 156 191`) |
| `@go <id\|nome>` | Atalho pra cidades (ex: `@go prontera`, `@go 0`) |
| `@jumpto <nome>` | Vai até outro player |
| `@recall <nome>` | Traz outro player até você |
| `@mapinfo` | Info do mapa atual |
| `@where <nome>` | Descobre onde alguém está |
| `@memo` | Salva ponto de retorno (pra Butterfly Wing) |
| `@load` / `@return` | Volta para o save point |

## Itens

| Comando | Descrição |
|---|---|
| `@item <id\|nome> [qtd]` | Cria item (ex: `@item 501 50`) |
| `@itemreset` | Apaga inventário |
| `@storage` | Abre armazém de qualquer lugar |
| `@guildstorage` | Abre armazém da guild |
| `@cart <0-5>` | Pega/troca carroça |
| `@refine <slot> <+>` | Refina equipamento equipado |
| `@produce <id> [base+] [card]` | Fabrica item específico |

## Monstros

| Comando | Descrição |
|---|---|
| `@monster <id\|nome> [qtd]` | Spawna mob (ex: `@monster poring 10`) |
| `@summon <id\|nome>` | Spawna mob que te segue/serve |
| `@killmonster` | Mata todos os mobs do mapa (sem drop) |
| `@killmonster2` | Mata todos os mobs do mapa (com drop) |
| `@spawn <id> [qtd]` | Alias de `@monster` |

## Pet / Homunculus / Mercenário

| Comando | Descrição |
|---|---|
| `@makeegg <id>` | Cria ovo de pet |
| `@hatch` | Choca o ovo |
| `@petfriendly <0-1000>` | Ajusta amizade do pet |
| `@pethungry <0-100>` | Ajusta fome do pet |
| `@homlevel <n>` | Sobe nível do homunculus |
| `@homevolution` | Evolui homunculus |
| `@homshuffle` | Re-rola stats do homunculus |
| `@mercenary <id> <duracao>` | Invoca mercenário |

## Servidor / admin

| Comando | Descrição |
|---|---|
| `@kick <nome>` | Desconecta um player |
| `@kickall` | Desconecta todo mundo |
| `@ban <tempo> <nome>` | Bane (ex: `@ban +1d nome`) |
| `@unban <nome>` | Remove ban |
| `@block <nome>` / `@unblock <nome>` | Bloqueia conta permanentemente |
| `@mute <min> <nome>` | Silencia |
| `@reloaditemdb` | Recarrega item_db |
| `@reloadmobdb` | Recarrega mob_db |
| `@reloadskilldb` | Recarrega skill_db |
| `@reloadscript` | Recarrega scripts/NPCs |
| `@broadcast <texto>` / `@kami <texto>` | Anúncio global amarelo |
| `@kamib <texto>` | Anúncio global azul |
| `@disguise <id_mob>` | Vira monstro |
| `@undisguise` | Volta ao normal |
| `@time` | Hora do servidor |
| `@who` / `@who2` / `@who3` | Lista quem está online |

## Eventos / debug

| Comando | Descrição |
|---|---|
| `@event <NPC::OnEvento>` | Dispara label de NPC |
| `@npctalk <NPC> <texto>` | Força NPC a falar |
| `@dropall` | Derruba tudo do inventário |
| `@autoloot <%>` | Pega drops automaticamente |
| `@duel` | Inicia duelo |
| `@accept` / `@reject` | Aceita/recusa duelo |
| `@leave` | Sai do duelo |

---

## Atalhos úteis

- `@commands` — lista todos os comandos disponíveis pro seu grupo
- `@help <comando>` — descrição do comando
- `@where` — mostra suas coordenadas (útil pra `@warp` depois)

## IDs de cidades para `@go`

| ID | Cidade |
|---|---|
| 0 | Prontera |
| 1 | Morroc |
| 2 | Geffen |
| 3 | Payon |
| 4 | Alberta |
| 5 | Izlude |
| 6 | Al de Baran |
| 7 | Lutie |
| 8 | Comodo |
| 9 | Yuno |
| 10 | Amatsu |
| 11 | Gonryun |
| 12 | Umbala |
| 13 | Niflheim |
| 14 | Louyang |
| 15 | Save point |
| 16 | Novice grounds |
| 17 | Prontera prison |
| 18 | Jawaii |
| 19 | Ayothaya |
| 20 | Einbroch |
| 21 | Lighthalzen |
| 22 | Einbech |
| 23 | Hugel |
| 24 | Rachel |
| 25 | Veins |
| 26 | Moscovia |
| 27 | Midgard Camp (Battlegrounds) |
| 28 | Manuk |
| 29 | Splendide |
| 30 | Brasilis |
| 31 | El Dicastes |
| 32 | Mora |
| 33 | Dewata |
| 34 | Malangdo |
| 35 | Malaya |
| 36 | Eclage |
