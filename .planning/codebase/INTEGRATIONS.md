# External Integrations

**Analysis Date:** 2026-04-26

## APIs & External Services

**Game Server Protocols:**
- rAthena login-server - Korangar connects to the local login service configured by `korangar/korangar/archive/data/sclientinfo.xml`.
  - SDK/Client: custom TCP client in `korangar/korangar-networking/src/lib.rs` using `tokio::net::TcpStream`.
  - Auth: username/password packets built by `korangar/korangar-networking/src/lib.rs`; credentials are validated by rAthena against MariaDB.
  - Host/port: `127.0.0.1:6900` in `korangar/korangar/archive/data/sclientinfo.xml`.
- rAthena char-server - login-server returns character-server data, and Docker publishes char-server from `docker-compose.yml`.
  - SDK/Client: custom packet structs from `korangar/ragnarok-packets/src/lib.rs` and packet-version handlers in `korangar/korangar-networking/src/packet_versions/version_20220406.rs`.
  - Auth: rAthena session identifiers from login success packets.
  - Host/port: Docker service `char`, host port `6121`, configured by `docker/import/char_conf.txt` and `docker-compose.yml`.
- rAthena map-server - game world connection after character selection.
  - SDK/Client: custom packet sender/receiver in `korangar/korangar-networking/src/lib.rs`.
  - Auth: map login packet uses data from login and character selection flow in `korangar/korangar-networking/src/lib.rs`.
  - Host/port: Docker service `map`, host port `5121`, configured by `docker/import/map_conf.txt` and `docker-compose.yml`.

**Local AI Example:**
- Ollama chat API - example-only integration for a chat bot.
  - SDK/Client: `reqwest` dev dependency in `korangar/korangar-networking/Cargo.toml`.
  - Auth: Not detected.
  - Endpoint: `http://127.0.0.1:11434/api/chat` in `korangar/korangar-networking/examples/ollama-chat-bot.rs`.
  - Runtime status: example binary only, not part of the main `korangar` client target.

**Build-Time Downloads:**
- Slang compiler release artifact - CI downloads `slangc` from GitHub releases.
  - SDK/Client: `curl` in `korangar/.github/workflows/build.yml`, `korangar/.github/workflows/tests.yml`, and `korangar/.github/workflows/lint.yml`.
  - Auth: Not detected.
- `wait-for` utility - Docker image downloads `https://raw.githubusercontent.com/eficode/wait-for/v2.2.4/wait-for`.
  - SDK/Client: `curl` in `docker/Dockerfile`.
  - Auth: Not detected.

**Package Registries / Source Repos:**
- crates.io - Rust dependencies resolved by `korangar/Cargo.toml` and `korangar/Cargo.lock`.
  - SDK/Client: Cargo.
  - Auth: Not detected.
- Git dependencies - `rust-state` from `https://github.com/vE5li/rust-state` and `rav1d` from `https://github.com/memorysafety/rav1d.git`, declared in `korangar/Cargo.toml`.
  - SDK/Client: Cargo git dependency resolver.
  - Auth: Not detected.
- Docker Hub - `mariadb:11` and `alpine:3.20` images referenced by `docker-compose.yml` and `docker/Dockerfile`.
  - SDK/Client: Docker Engine / Docker Compose.
  - Auth: Not detected.

## Data Storage

**Databases:**
- MariaDB 11
  - Connection: rAthena reads `DB_HOST`, `DB_PORT`, `DB_USER`, `DB_PASS`, and `DB_NAME` from `docker-compose.yml` / `docker/entrypoint.sh`.
  - Client: rAthena C++ SQL layer built against MariaDB/MySQL libraries from `docker/Dockerfile` and rAthena third-party code under `rathena-master/3rdparty/mysql`.
  - Docker service: `mariadb` in `docker-compose.yml`.
  - Container: `rathena-db` in `docker-compose.yml`.
  - Published port: `3306:3306` in `docker-compose.yml`.
  - Schema seed: `rathena-master/sql-files/` mounted into `/docker-entrypoint-initdb.d` by `docker-compose.yml`.
  - Local admin/test seed: `docker/init-db/zz-admin.sql` mounted into `/docker-entrypoint-initdb.d/zz-admin.sql` by `docker-compose.yml`.
  - Persistent volume: `rathena-db-data` in `docker-compose.yml`.
- rAthena logical databases
  - Connection: `login_server_*`, `ipban_db_*`, `char_server_*`, `map_server_*`, `web_server_*`, and `log_db_*` settings in `docker/import/inter_conf.txt`.
  - Client: rAthena login, char, map, web, and log SQL subsystems in `rathena-master/src/`.
  - Secrets: password fields exist in config files; document only setting names, never values.

**File Storage:**
- Local filesystem only.
- Korangar game assets: `data.grf`, `rdata.grf`, and `archive/` are loaded from `korangar/korangar/` according to `play.bat` and `korangar/korangar/src/loaders/archive/native/list.rs`.
- Korangar server config: `korangar/korangar/archive/data/sclientinfo.xml`.
- rAthena source/build artifacts: `docker-compose.yml` bind-mounts `./rathena-master:/rathena`, so compiled `login-server`, `char-server`, and `map-server` persist in the host source tree.
- rAthena runtime dirs: `docker/entrypoint.sh` ensures `rathena-master/log/` and `rathena-master/save/` exist inside the bind mount.

**Caching:**
- Docker named volume cache/persistence: `rathena-db-data` for MariaDB data in `docker-compose.yml`.
- GitHub Actions cache: Korangar caches `~/slangc` in `korangar/.github/workflows/build.yml`, `korangar/.github/workflows/tests.yml`, and `korangar/.github/workflows/lint.yml`.
- Application-level cache service: None detected.

## Authentication & Identity

**Auth Provider:**
- Custom rAthena username/password authentication.
  - Implementation: Korangar sends login packets in `korangar/korangar-networking/src/lib.rs`; rAthena validates through `rathena-master/src/login/loginclif.cpp` and related SQL tables.
  - Storage: account data in MariaDB `login` table from rAthena schema under `rathena-master/sql-files/`.
  - Local seed: `docker/init-db/zz-admin.sql` creates a development account.
  - Config: `docker/import/login_conf.txt` adjusts minimum credential length, password hash mode, and web auth token behavior for the local stack.
  - OAuth/SAML/OpenID: Not detected.
  - Web auth provider: Not detected; rAthena web auth tokens are disabled by local config in `docker/import/login_conf.txt`.

## Monitoring & Observability

**Error Tracking:**
- None detected.
- No Sentry, OpenTelemetry collector, Prometheus, Datadog, or external error tracking SDKs were found in `korangar/Cargo.toml`, `docker-compose.yml`, or rAthena local config.

**Logs:**
- Docker logs through `docker compose logs` for `mariadb`, `login`, `char`, and `map`, documented in `README.md` and `AI_CONTEXT.md`.
- rAthena server logs are emitted by the C++ services under `rathena-master/src/` and runtime directories created by `docker/entrypoint.sh`.
- Korangar debug/logging support exists through local crate `korangar-debug` in `korangar/korangar-debug/Cargo.toml`; the main `debug` feature is declared in `korangar/korangar/Cargo.toml`.
- Captured local logs are present in `korangar/korangar/logs-all.txt`; treat this as generated runtime output, not source configuration.

## CI/CD & Deployment

**Hosting:**
- Local Docker Compose only for this workspace.
- No cloud hosting platform, Kubernetes manifests, Terraform, or production deployment target detected.

**CI Pipeline:**
- Korangar GitHub Actions:
  - Build: `korangar/.github/workflows/build.yml`.
  - Tests: `korangar/.github/workflows/tests.yml`.
  - Lint: `korangar/.github/workflows/lint.yml`.
  - Formatting: `korangar/.github/workflows/formatting.yml`.
  - Release: `korangar/.github/workflows/release.yml`.
- rAthena GitHub Actions:
  - GCC build workflows in `rathena-master/.github/workflows/build_servers_gcc.yml`.
  - CMake build workflows in `rathena-master/.github/workflows/build_servers_cmake.yml`.
  - MSBuild workflows in `rathena-master/.github/workflows/build_servers_msbuild.yml`.
  - Additional packet/version/build workflows under `rathena-master/.github/workflows/`.
- Root project CI: Not detected.

## Environment Configuration

**Required env vars:**
- `DB_HOST` - rAthena container database host, read by `docker/entrypoint.sh`.
- `DB_PORT` - rAthena container database port, read by `docker/entrypoint.sh`.
- `DB_USER` - rAthena database user, read by `docker/entrypoint.sh`.
- `DB_PASS` - rAthena database password, read by `docker/entrypoint.sh`.
- `DB_NAME` - rAthena database name, read by `docker/entrypoint.sh`.
- `PACKETVER` - rAthena compile-time packet version passed to `./configure` by `docker/entrypoint.sh`.
- `MARIADB_ROOT_PASSWORD` - MariaDB container root password, set in `docker-compose.yml`.
- `MARIADB_DATABASE` - initial MariaDB database, set in `docker-compose.yml`.
- `MARIADB_USER` - initial MariaDB application user, set in `docker-compose.yml`.
- `MARIADB_PASSWORD` - initial MariaDB application password, set in `docker-compose.yml`.
- `LOGIN_HOST` - optional char-server wait target override in `docker/entrypoint.sh`.
- `LOGIN_PORT` - optional char-server wait target override in `docker/entrypoint.sh`.
- `CHAR_HOST` - optional map-server wait target override in `docker/entrypoint.sh`.
- `CHAR_PORT` - optional map-server wait target override in `docker/entrypoint.sh`.
- `CARGO_TERM_COLOR` - CI output setting in Korangar GitHub Actions workflows.
- `SLANG_VERSION` - CI Slang compiler version in Korangar GitHub Actions workflows.

**Secrets location:**
- Docker Compose and rAthena import files contain local development credential settings in `docker-compose.yml` and `docker/import/inter_conf.txt`; do not quote values in generated docs or logs.
- No `.env` file was detected at repository root.
- No external secret manager integration detected.

## Webhooks & Callbacks

**Incoming:**
- None detected.
- The stack exposes raw TCP services for MariaDB (`3306`), rAthena login (`6900`), char (`6121`), and map (`5121`) through `docker-compose.yml`, but no HTTP webhook endpoints were found.

**Outgoing:**
- Main runtime: no outgoing third-party HTTP API calls detected in the `korangar` client or local rAthena runtime path.
- Example runtime: `korangar/korangar-networking/examples/ollama-chat-bot.rs` calls local Ollama at `http://127.0.0.1:11434/api/chat`.
- Build/CI: `docker/Dockerfile` downloads `wait-for`; Korangar CI downloads `slangc`; Cargo resolves crates.io and Git dependencies from `korangar/Cargo.toml`.

---

*Integration audit: 2026-04-26*
