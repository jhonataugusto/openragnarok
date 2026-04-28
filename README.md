# OpenRagnarok

Workspace de estudo e desenvolvimento para evoluir um ambiente Ragnarok local
com cliente Korangar em Rust, servidor rAthena em C++ e infraestrutura Docker.

O objetivo pratico deste repositorio e permitir que outras pessoas clonem o
projeto, subam o servidor local, compilem o cliente e colaborem sem depender de
passos escondidos na maquina original.

## O Que Tem Aqui

```text
.
|-- korangar/                 # Cliente Korangar em Rust
|-- rathena-master/           # Servidor rAthena adaptado para o ambiente local
|-- docker/                   # Dockerfile, entrypoint e configs importadas
|-- docker-compose.yml        # MariaDB + login/char/map server
|-- docs/                     # Documentacao do projeto e decisoes
|-- .planning/                # Contexto GSD do projeto
|-- play.bat                  # Atalho Windows para abrir o cliente
|-- GM_COMMANDS.md            # Comandos uteis de GM
|-- AI_CONTEXT.md             # Historico tecnico e contexto de debug
```

Este e um monorepo direto. `korangar/` e `rathena-master/` sao pastas normais do
repositorio, nao submodulos. A origem e as adaptacoes conhecidas ficam
documentadas em `docs/UPSTREAMS.md`.

## Requisitos

### Obrigatorios

- Windows 10/11
- Git
- Docker Desktop com Docker Compose v2
- Rust via rustup
- Vulkan SDK ou `slangc` disponivel no `PATH`
- NASM disponivel no `PATH`
- Assets oficiais do Ragnarok: `data.grf` e `rdata.grf`

### Instalacao Rapida No Windows

```powershell
winget install --id Git.Git -e --source winget
winget install --id Docker.DockerDesktop -e --source winget
winget install --id Rustlang.Rustup -e --source winget
winget install --id KhronosGroup.VulkanSDK -e --source winget
```

O NASM pode ser instalado manualmente por https://www.nasm.us/. Depois de
instalar, confirme:

```powershell
git --version
docker compose version
rustup --version
slangc --version
nasm -v
```

## Primeiro Setup

Clone o repositorio:

```powershell
git clone https://github.com/jhonataugusto/openragnarok.git D:\ragnarok
cd D:\ragnarok
```

Coloque os assets oficiais do cliente em:

```text
D:\ragnarok\korangar\korangar\data.grf
D:\ragnarok\korangar\korangar\rdata.grf
```

Esses arquivos nao sao versionados. Eles sao grandes e pertencem ao cliente
oficial.

## Subir O Servidor Local

Na raiz do projeto:

```powershell
docker compose up -d
```

Na primeira execucao, o Docker vai:

- subir MariaDB 11;
- importar os SQLs do rAthena;
- compilar `login-server`, `char-server` e `map-server`;
- iniciar os tres servidores;
- criar a conta local de GM.

Conta padrao local:

| Campo | Valor |
| --- | --- |
| usuario | `admin` |
| senha | `123` |
| group_id | `99` |

Portas locais:

| Servico | Porta |
| --- | --- |
| MariaDB | `3306` |
| login-server | `6900` |
| char-server | `6121` |
| map-server | `5121` |

Comandos uteis:

```powershell
docker compose ps
docker compose logs -f
docker compose logs -f map
docker compose restart map
docker compose down
docker compose down -v
```

Para forcar rebuild do rAthena:

```powershell
docker compose stop login char map
docker compose run --rm builder rebuild
docker compose up -d
```

## Configurar E Rodar O Cliente

O Korangar usa o arquivo:

```text
korangar\korangar\archive\data\sclientinfo.xml
```

Para o ambiente local, ele deve apontar para:

```xml
<address>127.0.0.1</address>
<port>6900</port>
```

Compile o cliente:

```powershell
cd D:\ragnarok\korangar
cargo build --release
```

Rode pelo Cargo:

```powershell
cd D:\ragnarok\korangar
cargo run --release
```

Ou rode pelo atalho da raiz:

```powershell
cd D:\ragnarok
.\play.bat
```

## Fluxo De Desenvolvimento

Antes de mexer:

```powershell
git status --short --branch
```

Para trabalhar no cliente:

```powershell
cd D:\ragnarok\korangar
cargo fmt --all
cargo test --all-features
cargo clippy --all-features -- -Dwarnings
```

Para trabalhar no servidor:

```powershell
cd D:\ragnarok
docker compose run --rm builder rebuild
docker compose up -d
```

Para acompanhar comportamento no servidor:

```powershell
docker compose logs -f map
```

## O Que Nao Deve Ser Commitado

Nao commite:

- `data.grf` e `rdata.grf`;
- `korangar/target/`;
- executaveis e DLLs gerados;
- logs e traces locais;
- `.vs/`, `.idea/`, `.vscode/`;
- backups locais como `rathena-master.bak/`;
- fixtures runtime locais como `korangar-test/`.

O `.gitignore` da raiz foi feito para bloquear esses arquivos.

## Compatibilidade Importante

Este workspace mantem `PACKETVER=20220406`. Nao altere esse valor sem uma tarefa
explicita, porque cliente, servidor e handlers de pacotes dependem dele para
continuar comparaveis.

Tambem evite alterar `rathena-master/` preventivamente. Para bugs de cliente,
primeiro prove que o servidor e o ponto causal antes de mudar o rAthena.

## Documentacao Util

- `docs/UPSTREAMS.md`: origem do Korangar/rAthena e politica do monorepo.
- `GM_COMMANDS.md`: comandos de GM para testar o ambiente.
- `AI_CONTEXT.md`: historico tecnico e observacoes de debugging.
- `.planning/PROJECT.md`: contexto GSD principal.
- `.planning/REQUIREMENTS.md`: requisitos ativos e historicos.
- `.planning/ROADMAP.md`: roadmap e fases.
- `.planning/STATE.md`: estado atual do projeto.

## Problemas Comuns

### `slangc` nao encontrado

Instale o Vulkan SDK e reabra o terminal. Confirme com:

```powershell
slangc --version
```

### `nasm` nao encontrado

Instale o NASM e adicione a pasta do executavel ao `PATH`. Confirme com:

```powershell
nasm -v
```

### Cliente abre mas nao encontra assets

Confirme que `data.grf` e `rdata.grf` estao em:

```text
korangar\korangar\
```

### Login nao conecta

Confirme que os containers estao de pe:

```powershell
docker compose ps
docker compose logs -f login
```
