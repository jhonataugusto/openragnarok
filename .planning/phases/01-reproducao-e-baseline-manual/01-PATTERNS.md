# Phase 01: Reproducao e Baseline Manual - Pattern Map

**Mapped:** 2026-04-26
**Files analyzed:** 1 artefato novo esperado
**Analogs found:** 3 / 3 padroes documentais principais

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `.planning/phases/01-reproducao-e-baseline-manual/01-BASELINE.md` | documentacao | manual/request-response | `AI_CONTEXT.md` + `README.md` + `GM_COMMANDS.md` | role-match |
| `.planning/phases/01-reproducao-e-baseline-manual/01-PATTERNS.md` | documentacao | transform | `01-RESEARCH.md` + `01-VALIDATION.md` | exact |

## Pattern Assignments

### `.planning/phases/01-reproducao-e-baseline-manual/01-BASELINE.md` (documentacao, manual/request-response)

**Analogs:** `AI_CONTEXT.md`, `README.md`, `GM_COMMANDS.md`, `play.bat`, `docker-compose.yml`

**Documento deve espelhar:**

- `AI_CONTEXT.md`: briefing objetivo de stack, caminho de execucao, contas de teste, bug conhecido, repro e arquivos-chave.
- `README.md`: comandos de Docker, conta padrao, portas publicadas e fluxo de conexao com Korangar.
- `GM_COMMANDS.md`: referencia curta para `@kill`, `@alive`, `@heal`, `@warp` e comandos auxiliares.
- `01-VALIDATION.md`: contrato de validacao manual e criterio de existencia de `01-BASELINE.md`.

**Stack e launcher pattern** (`AI_CONTEXT.md` linhas 7-27):

```markdown
- Cliente: Korangar em Rust
  - Fonte local: `D:\ragnarok\korangar\`
  - Binario: `D:\ragnarok\korangar\target\release\korangar.exe`
  - CWD obrigatorio: `D:\ragnarok\korangar\korangar\`
- Servidor: rAthena em Docker
  - Config Docker: `docker-compose.yml`, `docker/entrypoint.sh`, `docker/import/*.txt`
  - Packet version: `PACKETVER 20220406`
  - Servicos: `mariadb`, `login`, `char`, `map`, `builder`
```

**Startup command pattern** (`AI_CONTEXT.md` linhas 29-38; `README.md` linhas 17-27):

```powershell
cd D:\ragnarok
docker compose up -d login char map
docker compose ps
.\play.bat
```

**Docker service pattern** (`docker-compose.yml` linhas 28-35, 73-113):

```yaml
environment:
  DB_HOST: mariadb
  DB_PORT: "3306"
  DB_USER: ragnarok
  DB_PASS: ragnarok
  DB_NAME: ragnarok
  PACKETVER: "20220406"

login:
  ports:
    - "6900:6900"
char:
  ports:
    - "6121:6121"
map:
  ports:
    - "5121:5121"
```

**Launcher CWD pattern** (`play.bat` linhas 5-8, 17-25):

```bat
set "ROOT=%~dp0"
set "CWD=%ROOT%korangar\korangar"
set "EXE=%ROOT%korangar\target\release\korangar.exe"

if not exist "%CWD%\data.grf" (
    echo [erro] data.grf nao encontrado em "%CWD%"
    echo Coloque data.grf e rdata.grf nessa pasta antes de rodar.
    pause
    exit /b 1
)

cd /d "%CWD%"
"%EXE%" %*
```

**Conta e portas pattern** (`README.md` linhas 31-43; `AI_CONTEXT.md` linhas 61-69):

```markdown
Conta local principal:
- usuario: `admin`
- senha: `123`
- group_id: `99`

Portas locais:
- MariaDB: `3306`
- login-server: `6900`
- char-server: `6121`
- map-server: `5121`
```

**GM command pattern** (`GM_COMMANDS.md` linhas 12-18, 32-36):

```markdown
| `@kill` | Mata voce na hora |
| `@alive` | Ressuscita |
| `@heal` | Restaura HP/SP totalmente |
| `@warp <mapa> <x> <y>` | Teleporta para coordenada |
| `@go <id|nome>` | Atalho pra cidades |
```

**Bug/repro pattern** (`AI_CONTEXT.md` linhas 117-150):

```markdown
Sintoma: ao morrer (`@kill` ou PvP) e respawnar (botao Respawn ou `@alive`),
o personagem fica visualmente preso na pose de morto, mesmo com HP cheio e
podendo se mover. Janela de respawn as vezes tambem nao fecha.

Repro:
1. Entrar no jogo
2. `@kill`
3. Clicar Respawn
4. Registrar se fica visualmente deitado/morto
```

## Suggested `01-BASELINE.md` Output Pattern

Planner deve criar um unico artefato manual em:

`.planning/phases/01-reproducao-e-baseline-manual/01-BASELINE.md`

Formato recomendado:

````markdown
# Phase 01 - Baseline Manual de Respawn

**Data/hora:**
**Executor:**
**Git/status relevante:**
**Objetivo:** reproduzir o estado atual de `@kill -> Respawn` antes de qualquer fix permanente.

## Ambiente

| Item | Valor observado |
|------|-----------------|
| Windows cwd | `D:\ragnarok` |
| Docker services | resultado de `docker compose ps` |
| PACKETVER | `20220406` |
| Cliente | `korangar\target\release\korangar.exe` |
| Launcher | `.\play.bat` |
| Conta/personagem | `admin` / personagem selecionado |

## Comandos Executados

```powershell
cd D:\ragnarok
docker compose up -d login char map
docker compose ps
.\play.bat
```

## Fluxo Manual

1. Logar com a conta local.
2. Selecionar personagem.
3. Confirmar entrada no mapa.
4. Enviar `@kill` no chat.
5. Confirmar janela de Respawn.
6. Clicar Respawn.
7. Registrar estado final.

## Resultado Observado

| Criterio | Observacao |
|----------|------------|
| Player visualmente vivo/em pe | sim/nao + detalhe |
| Janela Respawn fechada | sim/nao + detalhe |
| HP restaurado | sim/nao + detalhe |
| Movimento funcional | sim/nao + detalhe |
| Bug reproduzido | sim/nao |

## Evidencias

- Logs relevantes:
- Screenshot/video manual, se houver:
- Observacoes livres:

## Conclusao

Baseline aceito para REPR-01/REPR-02/REPR-03: sim/nao.
````

## Runtime Commands and Launcher Patterns

Use comandos curtos, executados da raiz `D:\ragnarok`:

```powershell
cd D:\ragnarok
docker compose up -d login char map
docker compose ps
docker compose logs --no-color --tail=80 login char map
.\play.bat
```

Se houver duvida de portas:

```powershell
Test-NetConnection 127.0.0.1 -Port 6900
Test-NetConnection 127.0.0.1 -Port 6121
Test-NetConnection 127.0.0.1 -Port 5121
```

Se o cliente nao abrir, conferir os mesmos pre-requisitos codificados em `play.bat`:

```powershell
Test-Path .\korangar\target\release\korangar.exe
Test-Path .\korangar\korangar\data.grf
Test-Path .\korangar\korangar\rdata.grf
```

## Shared Patterns

### Documentacao operacional

**Source:** `README.md`, `AI_CONTEXT.md`
**Apply to:** `01-BASELINE.md`

Copiar o estilo de instrucoes verificaveis: caminho, comando, resultado esperado e observacao final. Evitar diagnostico especulativo nesta fase.

### Validacao manual

**Source:** `01-VALIDATION.md`
**Apply to:** `01-BASELINE.md`

Critico para REPR-03: o baseline deve registrar ambiente, passos e resultado visual. A fase nao exige suite automatizada nem mudanca de codigo.

### Compatibilidade de protocolo

**Source:** `docker-compose.yml`, `AI_CONTEXT.md`, `.planning/REQUIREMENTS.md`
**Apply to:** qualquer plano operacional da Phase 1

Manter `PACKETVER=20220406`. Nao alterar compose, rAthena ou Korangar para "fazer funcionar" durante a baseline.

## Files the Execution Plan Should Avoid Modifying

Phase 1 e documental/manual. O plano deve evitar modificar:

| File/Directory | Reason |
|----------------|--------|
| `korangar/korangar/src/main.rs` | Phase 1 nao corrige codigo; `main.rs` e area fragil de handlers. |
| `korangar/korangar/src/world/entity/mod.rs` | Estado visual/animacao pertence a fases de evidencia/fix, nao baseline. |
| `korangar/korangar-networking/src/lib.rs` | Captura/alteracao de networking pertence a Phase 2+. |
| `korangar/ragnarok-packets/src/lib.rs` | Pacotes nao devem mudar antes de evidencia. |
| `rathena-master/**` | Regra do projeto: nao alterar servidor preventivamente. |
| `docker-compose.yml` | Manter `PACKETVER=20220406` e stack comparavel ao bug observado. |
| `docker/entrypoint.sh` | Build/runtime do rAthena nao deve mudar nesta fase. |
| `docker/import/**` | Config local deve permanecer estavel durante baseline. |
| `play.bat` | Launcher existente ja codifica CWD correto; nao trocar por script novo. |
| `README.md`, `AI_CONTEXT.md`, `GM_COMMANDS.md` | Sao fontes/analogs, nao alvo da fase. |

Arquivos aceitaveis para a execucao planejar:

- `.planning/phases/01-reproducao-e-baseline-manual/01-BASELINE.md`
- Possivelmente logs/snapshots temporarios dentro da pasta da fase, se o executor precisar anexar evidencia manual.

## Windows Path and Working Directory Pitfalls

- Rodar o cliente direto de `D:\ragnarok` pode falhar em assets; `play.bat` muda para `D:\ragnarok\korangar\korangar`.
- Usar caminhos com `\` em PowerShell e evitar assumir shell Unix.
- Preferir `cd D:\ragnarok` antes de comandos Docker para garantir que o Compose use o arquivo correto.
- `docker compose` e `docker-compose` nao sao a mesma invocacao; os docs locais usam `docker compose`.
- `play.bat` usa `%~dp0`, entao ele deve ser chamado pelo arquivo existente na raiz, nao copiado para outro diretorio.
- Nao executar comandos contra `rathena-master.bak/`; o diretorio ativo e `rathena-master/`.
- Nao depender de rebuild Rust nesta fase; o launcher usa o binario existente e a fase e manual.

## No Analog Found

Nenhum arquivo sem analog relevante. A fase e coberta por documentos operacionais existentes e pelo launcher/Compose atual.

## Metadata

**Analog search scope:** documentos raiz, fase 01, `.planning/codebase/`, launcher e Docker Compose.
**Files scanned:** 12 fontes documentais/operacionais.
**Pattern extraction date:** 2026-04-26
