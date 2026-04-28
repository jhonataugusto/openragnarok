# Upstreams e Adaptacoes

Este projeto usa um monorepo direto. `korangar/` e `rathena-master/` sao
pastas normais do repositorio raiz, nao submodulos e nao um fluxo baseado em
patches.

## Origem do Codigo

| Pasta | Upstream | Revisao local registrada |
| --- | --- | --- |
| `korangar/` | `https://github.com/vE5li/korangar.git` | `c78445cbb3f766bf0228561806a552a34b216104` |
| `rathena-master/` | `https://github.com/rathena/rathena.git` | `b70dd346ffc6cc16c1aeaad522913cdbdd1da42b` |

## Politica do Monorepo

- O desenvolvimento normal acontece direto neste repositorio.
- Alteracoes locais devem ser documentadas no README, em docs especificos ou em
  commits claros.
- Nao e necessario aplicar patches para deixar o workspace funcional.
- Se precisarmos atualizar Korangar ou rAthena a partir dos upstreams, fazemos
  isso como uma tarefa explicita, revisando conflitos e registrando a decisao.

## Adaptacoes Conhecidas

- O ambiente local usa `PACKETVER=20220406` para manter compatibilidade entre
  Korangar e rAthena.
- A ofuscacao de pacotes do rAthena fica desativada para o cliente Korangar.
- A stack Docker sobe MariaDB, login-server, char-server e map-server para uso
  local.
- `docker/import/` contem overrides de configuracao aplicados no rAthena dentro
  dos containers.
- Assets oficiais como `data.grf` e `rdata.grf` nao fazem parte do repositorio.
  Cada pessoa precisa fornecer seus proprios arquivos localmente.

## Estado Antes da Conversao

Antes de converter para monorepo direto, as pastas abaixo eram checkouts Git
aninhados:

- `korangar/`: branch `main`, `ahead 13` em relacao a `origin/main`, com
  alteracoes locais em networking, pacotes, estado, input, UI e entidades.
- `rathena-master/`: checkout detached HEAD, com alteracoes locais em configs,
  char server, packet config, atcommands e `src/map/clif.cpp`.

Esses metadados Git aninhados foram removidos apenas para que o repositorio raiz
possa versionar o codigo como um monorepo unico.
