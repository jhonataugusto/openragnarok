# Phase 1: Reproducao e Baseline Manual - Research

**Researched:** 2026-04-26 [VERIFIED: system date]
**Domain:** baseline manual de reproducao Korangar + rAthena local [VERIFIED: .planning/ROADMAP.md]
**Confidence:** HIGH para ambiente e roteiro; MEDIUM para comportamento final porque a fase ainda precisa executar a reproducao manual observada [VERIFIED: .planning/ROADMAP.md; .planning/research/SUMMARY.md]

## User Constraints

### Locked Decisions

- Responder em portugues do Brasil em todos os fluxos GSD deste projeto. [VERIFIED: AGENTS.md]
- O valor central e: depois de `@kill -> Respawn`, o player deve voltar visualmente vivo, com janela de respawn fechada, sem depender de tentativa cega ou workaround nao comprovado. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Usar `.planning/PROJECT.md`, `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md` e `.planning/STATE.md` como fontes principais de contexto GSD. [VERIFIED: AGENTS.md]
- Consultar `.planning/codebase/` antes de mexer em arquitetura, stack, testes ou areas frageis. [VERIFIED: AGENTS.md]
- Para este milestone, preferir correcao minima comprovada a refatoracao ampla. [VERIFIED: AGENTS.md; .planning/PROJECT.md]
- Nao alterar `rathena-master/` preventivamente; mudar o servidor somente se a evidencia provar que ele e o ponto causal. [VERIFIED: AGENTS.md; .planning/REQUIREMENTS.md]
- Nao depender cegamente de `entities()[0]` para identificar o player local sem comprovar a identidade no fluxo observado. [VERIFIED: AGENTS.md; .planning/codebase/CONCERNS.md]
- Manter logs/instrumentacao temporaria removiveis ou claramente isolados. [VERIFIED: AGENTS.md]
- Manter `PACKETVER=20220406`. [VERIFIED: AGENTS.md; docker-compose.yml]
- Esta fase nao deve corrigir codigo; deve estabelecer baseline manual antes de alterar comportamento permanente. [VERIFIED: user prompt; .planning/ROADMAP.md]

### Claude's Discretion

- Nao ha `01-CONTEXT.md` para esta fase, entao nao ha decisoes adicionais de discuss-phase a copiar. [VERIFIED: shell `Test-Path .planning/phases/01-reproducao-e-baseline-manual/01-CONTEXT.md`]
- O planner pode escolher o formato do artefato de baseline, desde que registre passos, ambiente, comando usado e resultado visual observado. [VERIFIED: .planning/REQUIREMENTS.md]

### Deferred Ideas (OUT OF SCOPE)

- Corrigir login-server disconnect esta fora do escopo v1 imediato. [VERIFIED: .planning/PROJECT.md]
- Criar suite E2E completa para login, combate, morte e respawn esta diferido para v2. [VERIFIED: .planning/REQUIREMENTS.md; .planning/STATE.md]
- Refatorar `korangar/korangar/src/main.rs` amplamente esta fora de escopo para v1. [VERIFIED: .planning/REQUIREMENTS.md]
- Resolver todos os pacotes nao implementados esta fora de escopo; somente pacotes diretamente necessarios ao respawn pertencem ao milestone. [VERIFIED: .planning/REQUIREMENTS.md]

## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| REPR-01 | O desenvolvedor consegue iniciar o ambiente local necessario para reproduzir o fluxo de respawn com Korangar e rAthena. [VERIFIED: .planning/REQUIREMENTS.md] | Docker Compose, portas, containers, binario Korangar e GRFs foram auditados; o plano deve validar esses pontos antes do teste manual. [VERIFIED: shell `docker compose ps`; shell `Test-NetConnection`; shell `Test-Path`] |
| REPR-02 | O desenvolvedor consegue reproduzir o bug atual usando o fluxo `@kill -> Respawn` antes de alterar comportamento permanente. [VERIFIED: .planning/REQUIREMENTS.md] | O fluxo de input envia `RestartPacket` de respawn e o servidor chama `pc_respawn`, entao a reproducao deve usar o cliente real, login real e botao Respawn real. [VERIFIED: korangar/korangar/src/main.rs:1995; korangar/korangar-networking/src/lib.rs:693; rathena-master/src/map/clif.cpp:11785] |
| REPR-03 | O roteiro de reproducao registra passos, ambiente, comando usado e resultado visual observado. [VERIFIED: .planning/REQUIREMENTS.md] | O artefato de baseline deve registrar comandos, contas/personagem, status dos containers, versao/binario usado e observacao visual final. [VERIFIED: .planning/ROADMAP.md; README.md; AI_CONTEXT.md] |

## Summary

Esta fase deve produzir uma baseline manual, nao uma correcao. [VERIFIED: user prompt; .planning/ROADMAP.md] O plano deve confirmar que o laboratorio local roda o suficiente para login, entrada no mapa, `@kill` e clique em Respawn, usando o Korangar existente e o rAthena Docker existente. [VERIFIED: README.md; AI_CONTEXT.md; docker-compose.yml]

O ambiente atual ja tem `mariadb`, `login`, `char` e `map` em execucao, com portas 3306, 6900, 6121 e 5121 publicadas; as portas 6900, 6121 e 5121 responderam em `127.0.0.1`. [VERIFIED: shell `docker compose ps`; shell `Test-NetConnection`] O binario `korangar/target/release/korangar.exe`, `data.grf`, `rdata.grf` e `sclientinfo.xml` existem no workspace local. [VERIFIED: shell `Test-Path`; shell `Get-Item`]

**Primary recommendation:** Planejar um unico roteiro operacional: verificar pre-requisitos, subir/confirmar servidor, abrir `play.bat`, logar com conta GM local, executar `@kill`, clicar Respawn, registrar o estado visual final e salvar um baseline escrito antes de qualquer alteracao permanente. [VERIFIED: README.md; AI_CONTEXT.md; .planning/REQUIREMENTS.md]

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|--------------|----------------|-----------|
| Subir laboratorio local | Infra local / Docker | Database / rAthena services | Docker Compose define MariaDB, builder, login, char e map; a fase precisa apenas confirmar que estao prontos. [VERIFIED: docker-compose.yml] |
| Rodar cliente jogavel | Browser / Client equivalente desktop | OS / assets locais | Korangar e um executavel desktop Rust e `play.bat` ajusta o CWD para localizar `data.grf`, `rdata.grf` e `archive/`. [VERIFIED: play.bat; .planning/codebase/STACK.md] |
| Executar respawn | Cliente Korangar | API / Backend rAthena map-server | `InputEvent::Respawn` chama `networking_system.respawn()`, que envia `RestartPacket`; rAthena parseia restart e chama `pc_respawn`. [VERIFIED: korangar/korangar/src/main.rs:1995; korangar/korangar-networking/src/lib.rs:693; rathena-master/src/map/clif.cpp:11785] |
| Observar bug visual | Cliente Korangar / UI renderizada | rAthena logs como contexto | O bug definido e visual/interativo: HP e movimento podem voltar, mas a pose morta e a janela Respawn podem permanecer. [VERIFIED: AI_CONTEXT.md; .planning/codebase/CONCERNS.md] |
| Registrar baseline | GSD docs | Shell logs / screenshot manual | REPR-03 exige passos, ambiente, comando usado e resultado visual observado. [VERIFIED: .planning/REQUIREMENTS.md] |

## Standard Stack

### Core

| Library / Tool | Version | Purpose | Why Standard |
|----------------|---------|---------|--------------|
| Korangar client | crate `korangar` 0.1.1; Rust edition 2024 [VERIFIED: korangar/korangar/Cargo.toml] | Cliente desktop usado para login, mapa, morte e respawn. [VERIFIED: .planning/codebase/STACK.md] | E o cliente onde o bug visual foi observado. [VERIFIED: AI_CONTEXT.md] |
| Rust toolchain | `nightly-2026-02-01`; `rustc 1.95.0-nightly` no diretorio `korangar/` [VERIFIED: korangar/rust-toolchain.toml; shell `rustc --version`] | Build do cliente se rebuild for necessario. [VERIFIED: README.md] | O workspace Korangar fixa essa toolchain localmente. [VERIFIED: korangar/rust-toolchain.toml] |
| rAthena | fonte local `rathena-master/`; C++17 [VERIFIED: .planning/codebase/STACK.md] | Servidor login/char/map usado pelo fluxo real de respawn. [VERIFIED: docker-compose.yml] | O bug depende do contrato Korangar + rAthena local. [VERIFIED: .planning/PROJECT.md] |
| Docker Compose | Docker 29.2.1; Compose v5.0.2 [VERIFIED: shell `docker --version`; shell `docker compose version`] | Orquestra MariaDB, builder, login, char e map. [VERIFIED: docker-compose.yml] | E o caminho documentado para subir o rAthena local. [VERIFIED: README.md] |
| MariaDB | image `mariadb:11` [VERIFIED: docker-compose.yml] | Banco local do rAthena. [VERIFIED: docker-compose.yml] | O compose importa schema rAthena e seed de conta admin. [VERIFIED: docker-compose.yml; README.md] |
| `PACKETVER` | `20220406` [VERIFIED: docker-compose.yml] | Compatibilidade binaria cliente/servidor. [VERIFIED: AGENTS.md; .planning/codebase/STACK.md] | Mudar esta versao quebraria comparabilidade com o bug observado. [VERIFIED: .planning/REQUIREMENTS.md] |

### Supporting

| Library / Tool | Version | Purpose | When to Use |
|----------------|---------|---------|-------------|
| `play.bat` | script local [VERIFIED: play.bat] | Inicia Korangar com CWD correto. [VERIFIED: play.bat] | Use no roteiro manual para evitar erro de asset path. [VERIFIED: play.bat] |
| `docker compose logs` | Compose CLI v5.0.2 [VERIFIED: shell `docker compose version`] | Captura evidencia de readiness e login/logout do servidor. [VERIFIED: README.md; shell `docker compose logs --tail=60`] | Use para registrar baseline operacional, nao para diagnostico profundo de pacotes nesta fase. [VERIFIED: .planning/ROADMAP.md] |
| `Test-NetConnection` | PowerShell disponivel [VERIFIED: shell `Get-Command Test-NetConnection`] | Confirma portas locais do login/char/map. [VERIFIED: shell `Test-NetConnection`] | Use antes de abrir o cliente se houver duvida de conectividade. [VERIFIED: README.md] |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `play.bat` | Rodar `korangar.exe` manualmente | Manual exige `cd korangar/korangar` para achar GRFs; `play.bat` ja codifica esse requisito. [VERIFIED: play.bat] |
| Baseline manual | E2E automatizado | E2E completo esta diferido para v2 e nao existe harness atual detectado. [VERIFIED: .planning/REQUIREMENTS.md; .planning/codebase/TESTING.md] |
| rAthena real no Docker | Mock de servidor | O objetivo e reproduzir o bug atual no ambiente local real antes de fix permanente. [VERIFIED: .planning/ROADMAP.md] |

**Installation / startup:**

```powershell
# Fonte: README.md e docker-compose.yml
cd D:\ragnarok
docker compose up -d login char map
docker compose ps
.\play.bat
```

**Version verification:** Para esta fase nao ha pacote npm recomendado; versoes foram verificadas por manifests locais e comandos CLI. [VERIFIED: .planning/codebase/STACK.md; shell `docker --version`; shell `docker compose version`; shell `rustc --version`]

## Architecture Patterns

### System Architecture Diagram

```text
Developer
  |
  | docker compose up / ps / logs
  v
Local Docker stack
  |--> MariaDB seed/account state
  |--> rAthena login :6900
  |--> rAthena char  :6121
  '--> rAthena map   :5121
          ^
          | TCP Ragnarok protocol, PACKETVER 20220406
          |
Korangar desktop client
  |
  | login -> select char -> enter map
  v
Manual reproduction
  |
  | @kill
  v
Dead player + Respawn window
  |
  | click Respawn -> RestartPacket 0x00B2
  v
rAthena clif_parse_Restart -> pc_respawn -> save-point map/position handling
  |
  | map/update packets back to Korangar
  v
Observed final state
  |--> visual alive/dead?
  |--> movement works?
  |--> HP restored?
  '--> Respawn window closed/open?
```

### Recommended Project Structure

```text
.planning/phases/01-reproducao-e-baseline-manual/
├── 01-RESEARCH.md        # esta pesquisa [VERIFIED: user prompt]
└── 01-BASELINE.md        # artefato recomendado para REPR-03 [ASSUMED]
```

### Pattern 1: Baseline Manual Antes de Mudanca

**What:** Registrar o estado atual com comandos, versoes, personagem usado, passos e resultado visual antes de alterar codigo. [VERIFIED: .planning/REQUIREMENTS.md; .planning/ROADMAP.md]

**When to use:** Use sempre que uma fase existe para reproduzir comportamento atual e criar comparabilidade para fases posteriores. [VERIFIED: .planning/ROADMAP.md]

**Example:**

```markdown
<!-- Fonte: REPR-03 em .planning/REQUIREMENTS.md -->
## Baseline manual

- Data/hora:
- Git/status relevante:
- Containers:
- Cliente:
- Conta/personagem:
- Passos:
  1. Abrir .\play.bat
  2. Logar
  3. Entrar no mapa
  4. Executar @kill
  5. Clicar Respawn
- Resultado visual:
- Janela Respawn:
- HP/movimento:
- Evidencia anexada:
```

### Pattern 2: Verificacao Operacional Curta

**What:** Confirmar containers, portas, binario e GRFs antes de tentar reproduzir o bug. [VERIFIED: README.md; play.bat; shell probes]

**When to use:** Use no inicio do plano para separar falha de ambiente de falha de gameplay. [VERIFIED: .planning/ROADMAP.md]

**Example:**

```powershell
# Fonte: README.md, play.bat, shell probes desta pesquisa
cd D:\ragnarok
docker compose ps
Test-NetConnection 127.0.0.1 -Port 6900
Test-NetConnection 127.0.0.1 -Port 6121
Test-NetConnection 127.0.0.1 -Port 5121
Test-Path .\korangar\target\release\korangar.exe
Test-Path .\korangar\korangar\data.grf
Test-Path .\korangar\korangar\rdata.grf
```

### Anti-Patterns to Avoid

- **Corrigir durante a reproducao:** Esta fase deve documentar o bug atual antes de qualquer fix permanente. [VERIFIED: user prompt; .planning/ROADMAP.md]
- **Validar so HP/movimento:** O bug conhecido permite movimento e HP cheio enquanto a pose morta continua. [VERIFIED: AI_CONTEXT.md; .planning/codebase/CONCERNS.md]
- **Assumir que servidor e culpado:** `rathena-master/` so deve mudar se evidencia futura provar causalidade. [VERIFIED: AGENTS.md; .planning/REQUIREMENTS.md]
- **Trocar `PACKETVER`:** A compatibilidade deve permanecer em `20220406`. [VERIFIED: AGENTS.md; docker-compose.yml]

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Reproducao do servidor | Script novo de orquestracao | `docker compose up -d login char map` | Compose ja define servicos, dependencias, portas e env vars. [VERIFIED: docker-compose.yml; README.md] |
| Launcher do cliente | Comando manual com CWD sujeito a erro | `.\play.bat` | O script ja valida `korangar.exe` e `data.grf`, e muda para `korangar/korangar`. [VERIFIED: play.bat] |
| Diagnostico profundo de pacotes | Packet tracer novo na fase 1 | Registro manual + logs basicos | Captura de pacotes pertence a Phase 2, nao a baseline. [VERIFIED: .planning/ROADMAP.md] |
| Automacao E2E | Harness novo de login/morte/respawn | Roteiro manual documentado | E2E completo esta diferido para v2. [VERIFIED: .planning/REQUIREMENTS.md] |

**Key insight:** Esta fase precisa provar que o bug ainda existe no laboratorio atual; criar ferramentas novas agora aumenta variaveis e reduz comparabilidade. [VERIFIED: .planning/research/SUMMARY.md]

## Common Pitfalls

### Pitfall 1: Ambiente parece iniciado, mas mapa nao esta pronto

**What goes wrong:** O cliente abre, mas login/char/map ainda nao estao todos prontos para o fluxo completo. [VERIFIED: docker-compose.yml; README.md]
**Why it happens:** O Compose inicia servicos encadeados, mas readiness real do map-server aparece nos logs como servidor online/listening. [VERIFIED: docker-compose.yml; shell `docker compose logs --tail=60`]
**How to avoid:** Registrar `docker compose ps`, testar portas 6900/6121/5121 e capturar tail curto dos logs. [VERIFIED: shell probes]
**Warning signs:** `map` ausente, porta 5121 fechada ou logs sem "Map Server is now online". [VERIFIED: shell `docker compose logs --tail=60`]

### Pitfall 2: Rodar Korangar do diretorio errado

**What goes wrong:** O cliente pode falhar ao localizar assets se o CWD nao for `korangar/korangar`. [VERIFIED: play.bat; README.md]
**Why it happens:** `play.bat` comenta que o CWD precisa achar `data.grf` e `rdata.grf`. [VERIFIED: play.bat]
**How to avoid:** Usar `.\play.bat` no roteiro. [VERIFIED: play.bat]
**Warning signs:** Erro de `data.grf` ausente ou executavel chamado diretamente de outro diretorio. [VERIFIED: play.bat]

### Pitfall 3: Baseline ambigua

**What goes wrong:** Registrar apenas "respawn funcionou" perde o criterio principal do bug visual. [VERIFIED: AI_CONTEXT.md; .planning/codebase/CONCERNS.md]
**Why it happens:** O player pode mover e ter HP restaurado enquanto continua visualmente morto. [VERIFIED: AI_CONTEXT.md]
**How to avoid:** Registrar separadamente pose visual, janela Respawn, movimento e HP. [VERIFIED: .planning/REQUIREMENTS.md]
**Warning signs:** Evidencia sem screenshot/descricao visual final ou sem mencionar janela Respawn. [VERIFIED: .planning/REQUIREMENTS.md]

### Pitfall 4: Misturar Phase 1 com Phase 2

**What goes wrong:** O plano tenta capturar ordem de pacotes e identidade de entidade antes de ter baseline reproduzida. [VERIFIED: .planning/ROADMAP.md]
**Why it happens:** O problema tem hipoteses tecnicas ja conhecidas, mas a fase atual cobre somente REPR-01 a REPR-03. [VERIFIED: .planning/REQUIREMENTS.md]
**How to avoid:** Limitar Phase 1 a reproducao e registro; deixar pacote/evento/entity_id para Phase 2. [VERIFIED: .planning/ROADMAP.md]
**Warning signs:** Tarefas para editar `main.rs`, adicionar logs temporarios ou usar Wireshark nesta fase. [VERIFIED: .planning/ROADMAP.md; AI_CONTEXT.md]

## Code Examples

### Subir e confirmar servidor

```powershell
# Fonte: README.md, docker-compose.yml
cd D:\ragnarok
docker compose up -d login char map
docker compose ps
docker compose logs --no-color --tail=80 login char map
```

### Confirmar portas locais

```powershell
# Fonte: portas em docker-compose.yml
Test-NetConnection 127.0.0.1 -Port 6900
Test-NetConnection 127.0.0.1 -Port 6121
Test-NetConnection 127.0.0.1 -Port 5121
```

### Abrir cliente pelo launcher existente

```powershell
# Fonte: play.bat
cd D:\ragnarok
.\play.bat
```

### Fluxo manual dentro do jogo

```text
Fonte: AI_CONTEXT.md e GM_COMMANDS.md
1. Logar com uma conta GM local, por exemplo admin/123.
2. Entrar com um personagem no mapa.
3. Enviar @kill no chat.
4. Clicar no botao Respawn.
5. Registrar se o player ficou em pe/vivo ou visualmente morto.
6. Registrar se a janela Respawn fechou ou permaneceu aberta.
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Corrigir diretamente handlers de `ResurrectPlayer`/`ChangeMap` | Reproduzir baseline antes de nova mudanca permanente | Roadmap v1 em 2026-04-26 [VERIFIED: .planning/ROADMAP.md; .planning/research/SUMMARY.md] | Evita repetir workaround nao comprovado. [VERIFIED: AI_CONTEXT.md] |
| Validar respawn por movimento/HP | Validar pose visual, HP/movimento e janela Respawn separadamente | Requirements v1 em 2026-04-26 [VERIFIED: .planning/REQUIREMENTS.md] | Alinha aceite com o bug real. [VERIFIED: .planning/PROJECT.md] |
| Assumir que `ZC_RESURRECTION` sempre chega | Tratar ausencia/presenca de pacotes como tema da Phase 2 | Roadmap v1 em 2026-04-26 [VERIFIED: .planning/ROADMAP.md] | Mantem Phase 1 simples e observavel. [VERIFIED: .planning/ROADMAP.md] |

**Deprecated/outdated:**

- Usar `entities()[0]` como prova de player local sem verificacao esta fora do padrao deste milestone. [VERIFIED: AGENTS.md; .planning/codebase/CONCERNS.md]
- Patchar rAthena preventivamente antes da evidencia esta fora do padrao deste milestone. [VERIFIED: AGENTS.md; .planning/REQUIREMENTS.md]

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | O planner pode criar `01-BASELINE.md` como artefato recomendado de REPR-03. [ASSUMED] | Recommended Project Structure | Baixo: o nome do artefato pode mudar sem afetar a reproducao, desde que REPR-03 seja cumprido. |
| A2 | A pesquisa deve ser considerada valida por 7 dias para ambiente local e 30 dias para constraints/roadmap se nao houver nova discuss-phase. [ASSUMED] | Metadata | Baixo: o planner pode revalidar ambiente com comandos rapidos antes de executar. |

## Open Questions

1. **O bug ainda reproduz exatamente no binario atual?**
   - What we know: Historico e concern registram que o player pode continuar visualmente morto apos Respawn. [VERIFIED: AI_CONTEXT.md; .planning/codebase/CONCERNS.md]
   - What's unclear: A fase ainda precisa executar o fluxo no ambiente atual e registrar o resultado visual. [VERIFIED: .planning/ROADMAP.md]
   - Recommendation: Planejar a reproducao manual como primeira tarefa executavel da fase. [VERIFIED: .planning/ROADMAP.md]

2. **Qual conta/personagem deve ser usado como baseline principal?**
   - What we know: README documenta `admin/123`, e AI_CONTEXT lista `admin/123` e `jonato/jonato`. [VERIFIED: README.md; AI_CONTEXT.md]
   - What's unclear: O personagem exato que o executor vai selecionar pode depender do estado atual do banco. [VERIFIED: AI_CONTEXT.md]
   - Recommendation: Registrar conta e personagem usados no baseline; preferir conta GM local para acesso a `@kill`. [VERIFIED: README.md; GM_COMMANDS.md]

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|-------------|-----------|---------|----------|
| Docker Engine | rAthena/MariaDB local | Sim [VERIFIED: shell `docker --version`] | 29.2.1 [VERIFIED: shell `docker --version`] | Nenhum para reproduzir ambiente Docker real. [VERIFIED: docker-compose.yml] |
| Docker Compose | Orquestracao dos servicos | Sim [VERIFIED: shell `docker compose version`] | v5.0.2 [VERIFIED: shell `docker compose version`] | Nenhum pratico para Phase 1. [VERIFIED: README.md] |
| Containers `mariadb/login/char/map` | Fluxo jogavel | Sim, todos `Up`; MariaDB healthy [VERIFIED: shell `docker compose ps`] | `mariadb:11`, `rathena:local` [VERIFIED: docker-compose.yml; shell `docker compose ps`] | Rodar `docker compose up -d login char map`. [VERIFIED: README.md] |
| Portas 6900/6121/5121 | Login/char/map | Sim [VERIFIED: shell `Test-NetConnection`] | N/A | Verificar logs e reiniciar Compose. [VERIFIED: README.md] |
| Korangar release exe | Cliente manual | Sim [VERIFIED: shell `Get-Item korangar/target/release/korangar.exe`] | binario local de 2026-04-26 18:20:34 [VERIFIED: shell `Get-Item`] | Rebuild com Cargo, mas `nasm` ausente pode bloquear. [VERIFIED: shell `nasm -v`] |
| `data.grf` / `rdata.grf` | Assets do cliente | Sim [VERIFIED: shell `Test-Path`] | arquivos locais existentes [VERIFIED: shell `Get-Item`] | Sem fallback legal/local documentado; precisa dos GRFs. [VERIFIED: README.md] |
| Rust/Cargo Korangar | Rebuild opcional | Sim [VERIFIED: shell `rustc --version`; shell `cargo --version` em `korangar/`] | `rustc 1.95.0-nightly`, `cargo 1.95.0-nightly` [VERIFIED: shell] | Usar binario existente se nenhum rebuild for necessario. [VERIFIED: play.bat] |
| `slangc` | Rebuild opcional dos shaders | Sim [VERIFIED: shell `slangc -version`] | `2026.1-52-gc8ddf20bb` [VERIFIED: shell] | Usar binario existente para Phase 1. [VERIFIED: play.bat] |
| `nasm` | Rebuild opcional por dependencia de video | Nao [VERIFIED: shell `Get-Command nasm`] | N/A | Usar binario existente; instalar NASM somente se rebuild for exigido. [VERIFIED: README.md] |

**Missing dependencies with no fallback:**
- Nenhum bloqueio para executar Phase 1 com o binario existente. [VERIFIED: shell probes; play.bat]

**Missing dependencies with fallback:**
- `nasm` ausente; fallback recomendado para Phase 1 e usar `korangar/target/release/korangar.exe` existente. [VERIFIED: shell `Get-Command nasm`; shell `Get-Item korangar/target/release/korangar.exe`]

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Cargo test para unidades Rust; validacao manual para Phase 1. [VERIFIED: .planning/codebase/TESTING.md; .planning/ROADMAP.md] |
| Config file | `korangar/Cargo.toml`, `korangar/rust-toolchain.toml`; sem harness E2E de cliente detectado. [VERIFIED: .planning/codebase/TESTING.md] |
| Quick run command | `cd D:\ragnarok && docker compose ps` para readiness operacional. [VERIFIED: docker-compose.yml] |
| Full suite command | Nao aplicavel como gate da Phase 1; `cd korangar && cargo test --all-features` existe mas nao comprova respawn visual. [VERIFIED: .planning/codebase/TESTING.md; AI_CONTEXT.md] |

### Phase Requirements -> Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|--------------|
| REPR-01 | Ambiente local inicia o suficiente para reproduzir respawn. [VERIFIED: .planning/REQUIREMENTS.md] | smoke/manual | `docker compose ps` + `Test-NetConnection` [VERIFIED: shell probes] | N/A manual |
| REPR-02 | Bug atual reproduz com `@kill -> Respawn` antes de fix permanente. [VERIFIED: .planning/REQUIREMENTS.md] | manual gameplay | Nao ha comando automatizado; executar pelo cliente. [VERIFIED: .planning/codebase/TESTING.md] | N/A manual |
| REPR-03 | Roteiro registra passos, ambiente, comando e resultado visual. [VERIFIED: .planning/REQUIREMENTS.md] | doc review | Conferir artefato de baseline. [VERIFIED: .planning/REQUIREMENTS.md] | Recomendado criar `01-BASELINE.md`. [ASSUMED] |

### Sampling Rate

- **Per task commit:** Nao ha mudanca de codigo esperada nesta fase; validar `docker compose ps` e existencia do baseline. [VERIFIED: user prompt; .planning/ROADMAP.md]
- **Per wave merge:** Conferir que REPR-01, REPR-02 e REPR-03 foram marcados por evidencia escrita. [VERIFIED: .planning/REQUIREMENTS.md]
- **Phase gate:** Baseline manual salvo antes de qualquer alteracao permanente. [VERIFIED: user prompt; .planning/ROADMAP.md]

### Wave 0 Gaps

- [ ] `01-BASELINE.md` ou artefato equivalente para registrar REPR-03. [ASSUMED]
- [ ] Nenhum novo framework de teste deve ser instalado para Phase 1. [VERIFIED: .planning/REQUIREMENTS.md; .planning/codebase/TESTING.md]

## Security Domain

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|------------------|
| V2 Authentication | Sim, mas somente local/manual | Usar contas locais documentadas; nao publicar credenciais como producao. [VERIFIED: README.md; .planning/codebase/CONCERNS.md] |
| V3 Session Management | Sim, como contexto de login | Nao investigar reconnect nesta fase; bug separado esta fora de escopo. [VERIFIED: .planning/PROJECT.md; AI_CONTEXT.md] |
| V4 Access Control | Sim, dev-only | Conta GM e at-commands sao configuracao de laboratorio local. [VERIFIED: GM_COMMANDS.md; .planning/codebase/CONCERNS.md] |
| V5 Input Validation | Nao como mudanca de codigo nesta fase | Sem parser novo ou endpoint novo. [VERIFIED: user prompt; .planning/ROADMAP.md] |
| V6 Cryptography | Nao como mudanca nesta fase | Packet obfuscation e credenciais dev existem como contexto, nao como alvo da fase. [VERIFIED: .planning/codebase/CONCERNS.md] |

### Known Threat Patterns for Local Ragnarok Lab

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Portas de MariaDB e servidores publicadas no host | Information Disclosure / Elevation | Tratar ambiente como local-only e nao usar como servidor publico. [VERIFIED: docker-compose.yml; .planning/codebase/CONCERNS.md] |
| Credenciais dev hard-coded | Information Disclosure | Nao reutilizar fora do laboratorio; registrar apenas como credenciais locais. [VERIFIED: README.md; docker-compose.yml] |
| GM commands amplos | Elevation | Usar somente para reproducao local; nao transformar em perfil compartilhado/producao. [VERIFIED: GM_COMMANDS.md; .planning/codebase/CONCERNS.md] |

## Sources

### Primary (HIGH confidence)

- `AGENTS.md` - idioma, escopo, regras de trabalho e restricoes do milestone. [VERIFIED: file read]
- `.planning/PROJECT.md` - valor central, contexto, constraints e decisoes. [VERIFIED: file read]
- `.planning/REQUIREMENTS.md` - REPR-01, REPR-02, REPR-03 e fora de escopo. [VERIFIED: file read]
- `.planning/ROADMAP.md` - objetivo, success criteria e fronteiras da Phase 1. [VERIFIED: file read]
- `.planning/STATE.md` - foco atual, decisoes e blockers. [VERIFIED: file read]
- `README.md` - comandos locais Docker/Korangar e contas. [VERIFIED: file read]
- `AI_CONTEXT.md` - repro historica e bug nao resolvido. [VERIFIED: file read]
- `docker-compose.yml`, `docker/entrypoint.sh`, `play.bat` - stack e comandos operacionais. [VERIFIED: file read]
- `korangar/korangar/src/main.rs`, `korangar/korangar-networking/src/lib.rs`, `korangar/ragnarok-packets/src/lib.rs`, `rathena-master/src/map/pc.cpp`, `rathena-master/src/map/clif.cpp` - fluxo Respawn/Restart/pc_respawn. [VERIFIED: code grep and line reads]

### Secondary (MEDIUM confidence)

- `.planning/research/SUMMARY.md` - sintese de pesquisa anterior e riscos. [VERIFIED: file read]
- `.planning/codebase/STACK.md` - stack, versoes e requisitos de plataforma. [VERIFIED: file read]
- `.planning/codebase/CONCERNS.md` - bug conhecido, areas frageis e riscos. [VERIFIED: file read]
- `.planning/codebase/TESTING.md` - ausencia de E2E e comandos de teste existentes. [VERIFIED: file read]

### Tertiary (LOW confidence)

- Nome `01-BASELINE.md` para o artefato de reproducao. [ASSUMED]

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - verificado por manifests, compose, launcher e comandos locais. [VERIFIED: .planning/codebase/STACK.md; shell probes]
- Architecture: HIGH para fluxo operacional Phase 1; MEDIUM para causa tecnica do bug, que pertence a Phase 2. [VERIFIED: .planning/ROADMAP.md; .planning/research/SUMMARY.md]
- Pitfalls: HIGH - derivados de requisitos, contexto historico e concerns locais. [VERIFIED: AI_CONTEXT.md; .planning/codebase/CONCERNS.md]

**Research date:** 2026-04-26 [VERIFIED: system date]
**Valid until:** 2026-05-03 para disponibilidade de ambiente local, porque containers/binarios podem mudar rapidamente; 2026-05-26 para constraints e roadmap se nao houver nova discuss-phase. [ASSUMED]
