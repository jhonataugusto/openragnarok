# Technology Stack

**Analysis Date:** 2026-04-26

## Languages

**Primary:**
- Rust nightly `nightly-2026-02-01`, edition 2024 - Korangar client workspace in `korangar/Cargo.toml`, `korangar/rust-toolchain.toml`, and crate manifests such as `korangar/korangar/Cargo.toml`.
- C++17 - rAthena server implementation in `rathena-master/src/`, with C++17 required by `rathena-master/CMakeLists.txt` and `rathena-master/configure.ac`.

**Secondary:**
- Bash - Docker build/runtime orchestration in `docker/entrypoint.sh`.
- Dockerfile / Docker Compose YAML - local rAthena and MariaDB stack in `docker/Dockerfile` and `docker-compose.yml`.
- SQL - database schema and seed data in `rathena-master/sql-files/` plus local admin seed in `docker/init-db/zz-admin.sql`.
- XML - Korangar server discovery configuration in `korangar/korangar/archive/data/sclientinfo.xml`.
- RON - Korangar data/config assets under `korangar/korangar/archive/data/languages/*.ron`.
- TOML - Rust workspace, toolchain, Cargo, rustfmt, and cargo target settings in `korangar/Cargo.toml`, `korangar/rust-toolchain.toml`, `korangar/rustfmt.toml`, and `korangar/.cargo/config.toml`.
- Windows batch - local launcher in `play.bat`.
- Go - Not detected in implementation files or manifests. The project intent in `README.md` mentions a future Go rewrite, but the current executable codebase is Rust + C++.

## Runtime

**Environment:**
- Native desktop Rust executable: `korangar/korangar/src/main.rs` builds the `korangar` binary.
- Rust toolchain: `nightly-2026-02-01` from `korangar/rust-toolchain.toml`.
- Rust target tuning: x86_64 builds use `target_cpu=x86-64-v3`, `+aes`, and Windows MSVC uses `rust-lld.exe` from `korangar/.cargo/config.toml`.
- Docker Compose runtime: `docker-compose.yml` runs MariaDB plus `login`, `char`, `map`, and `builder` rAthena services.
- rAthena container base: `alpine:3.20` in `docker/Dockerfile`.
- Database runtime: `mariadb:11` image in `docker-compose.yml`.
- rAthena packet version: `PACKETVER=20220406` configured in `docker-compose.yml`, passed through `docker/entrypoint.sh`, and reflected in `rathena-master/src/config/packets.hpp`.
- Client working directory requirement: `play.bat` runs `korangar/target/release/korangar.exe` from `korangar/korangar/` so `data.grf`, `rdata.grf`, and `archive/` resolve by relative path.

**Package Manager:**
- Cargo - Rust workspace dependency and build manager via `korangar/Cargo.toml`.
- Lockfile: present at `korangar/Cargo.lock`.
- Alpine `apk` - container build package installer in `docker/Dockerfile`.
- Docker Compose v2 - local stack manager via `docker-compose.yml`.

## Frameworks

**Core:**
- wgpu `29.0.0` - GPU rendering backend for Korangar, declared in `korangar/Cargo.toml` and used by the `korangar` crate with `static-dxc` and `spirv` features.
- winit `0.30.13` - desktop window/event loop layer for Korangar, declared in `korangar/Cargo.toml`.
- Tokio `1.50.0` - async TCP networking runtime for `korangar-networking`, declared in `korangar/korangar-networking/Cargo.toml` and used through `tokio::net::TcpStream` in `korangar/korangar-networking/src/lib.rs`.
- rAthena - C++ MMORPG server package in `rathena-master/`, providing `login-server`, `char-server`, and `map-server`.
- MariaDB 11 - SQL persistence service in `docker-compose.yml`.

**Testing:**
- Cargo test - Rust unit/integration test runner invoked by `korangar/.github/workflows/tests.yml` with `cargo test --all-features`.
- Rust examples as integration tooling - packet capture and bot examples in `korangar/ragnarok-packets/examples/pcap.rs`, `korangar/korangar-networking/examples/rescue-my-character.rs`, and `korangar/korangar-networking/examples/ollama-chat-bot.rs`.
- rAthena upstream CI - GCC, CMake, and MSBuild workflows under `rathena-master/.github/workflows/`.

**Build/Dev:**
- Cargo build - Korangar release/debug builds in `korangar/.github/workflows/build.yml`.
- rustdoc JSON - `cargo rustdoc -p ragnarok-packets -- -Z unstable-options --output-format json` in `korangar/.github/workflows/build.yml`.
- rustfmt - configured by `korangar/rustfmt.toml`.
- Clippy - enforced by `korangar/.github/workflows/lint.yml`.
- Slang shader compiler (`slangc`) - required by `korangar/korangar/build.rs`; CI pins `SLANG_VERSION: v2025.18.2` in `korangar/.github/workflows/build.yml`.
- Vulkan SDK / Slang - Windows build prerequisite documented in `README.md` and checked in `korangar/korangar/build.rs`.
- NASM - needed by the `rav1d` video dependency, documented in `README.md` and installed in Korangar CI workflows.
- Autotools configure + Make - Dockerized rAthena build path in `docker/entrypoint.sh` uses `./configure --enable-packetver="${PACKETVER}"`, `make clean`, and `make server`.
- CMake - supported rAthena build system in `rathena-master/CMakeLists.txt`; installed into the Docker image by `docker/Dockerfile`.
- Nix devShell - Korangar CI validates the dev shell through `nix develop -L` and `nix flake check -L --all-systems` in `korangar/.github/workflows/build.yml` and `korangar/.github/workflows/lint.yml`.

## Key Dependencies

**Critical:**
- `korangar-networking` `0.1.0` - local crate handling login/map TCP sessions and packet dispatch, declared in `korangar/korangar-networking/Cargo.toml`.
- `ragnarok-packets` `0.1.0` - local packet definitions for packet version `20220406`, declared in `korangar/ragnarok-packets/Cargo.toml`.
- `ragnarok-bytes` `0.1.0` - local binary serialization/deserialization support, declared in `korangar/ragnarok-bytes/Cargo.toml`.
- `ragnarok-formats` `0.1.0` - local Ragnarok asset format loaders, declared in `korangar/ragnarok-formats/Cargo.toml`.
- `wgpu` `29.0.0` - renderer foundation for the client, locked in `korangar/Cargo.lock`.
- `winit` `0.30.13` - OS windowing and input event integration, locked in `korangar/Cargo.lock`.
- `tokio` `1.50.0` - async networking foundation for the client, locked in `korangar/Cargo.lock`.
- `mlua` `0.11.6` with `lua51` and `vendored` - Lua scripting/runtime support in the client, declared in `korangar/korangar/Cargo.toml`.
- `quick-xml` `0.38.4` / `0.39.2` - XML parsing used for client info and asset metadata, declared in `korangar/korangar/Cargo.toml` and locked in `korangar/Cargo.lock`.
- `serde` `1.0.228` - serialization/deserialization for configs and data formats, locked in `korangar/Cargo.lock`.
- `image` `0.25.10` - BMP/JPEG/PNG/TGA texture and image decoding, locked in `korangar/Cargo.lock`.
- `cpal` `0.17.3` and `symphonia` `0.5.5` - audio output and decoding through `korangar/korangar-audio/Cargo.toml`.
- `rav1d` `1.1.0` from `https://github.com/memorysafety/rav1d.git` at revision `c8019327ff0aa4c097475fa5f679561ea3abd983` - AV1 video decoding in `korangar/korangar-video/Cargo.toml`.
- `rust-state` `0.1.0` from `https://github.com/vE5li/rust-state` - UI/debug state integration, declared in workspace dependencies in `korangar/Cargo.toml`.

**Infrastructure:**
- Docker Desktop / Docker Engine + Compose v2 - required to run `docker-compose.yml`.
- MariaDB client/development packages - installed in `docker/Dockerfile` as `mariadb-client`, `mariadb-connector-c-dev`, and `mariadb-dev`.
- C/C++ toolchain - installed in `docker/Dockerfile` as `gcc`, `g++`, `make`, `cmake`, `linux-headers`, and related libraries.
- rAthena third-party C/C++ libraries - vendored under `rathena-master/3rdparty/` including `httplib`, `json`, `libconfig`, `mysql`, `pcre`, `rapidyaml`, `yaml-cpp`, and `zlib`.
- `wait-for` `v2.2.4` - downloaded in `docker/Dockerfile` and used by `docker/entrypoint.sh` to wait for MariaDB, login, and char services.
- `reqwest` `0.13.2` - dev dependency for the `ollama-chat-bot` example in `korangar/korangar-networking/examples/ollama-chat-bot.rs`; not part of the main client runtime path.
- `pcap` `2.4.0` and `etherparse` `0.19` - packet analysis tooling in `korangar/ragnarok-packets/Cargo.toml`.
- `sevenz-rust2` `0.20.2` - shader archive generation in `korangar/korangar/build.rs`.

## Configuration

**Environment:**
- Docker services are configured in `docker-compose.yml`; secret values exist there and in rAthena import files, but only variable names should be documented.
- rAthena common env vars: `DB_HOST`, `DB_PORT`, `DB_USER`, `DB_PASS`, `DB_NAME`, `PACKETVER` in `docker-compose.yml` and `docker/entrypoint.sh`.
- MariaDB env vars: `MARIADB_ROOT_PASSWORD`, `MARIADB_DATABASE`, `MARIADB_USER`, `MARIADB_PASSWORD` in `docker-compose.yml`.
- rAthena runtime defaults: `LOGIN_HOST`, `LOGIN_PORT`, `CHAR_HOST`, and `CHAR_PORT` are read by `docker/entrypoint.sh`.
- rAthena SQL connection overrides: `docker/import/inter_conf.txt` maps login, ipban, char, map, web, and log databases to the Docker `mariadb` service.
- rAthena service overrides: `docker/import/login_conf.txt`, `docker/import/char_conf.txt`, and `docker/import/map_conf.txt` adjust auth, pincode, and advertised service IP behavior for local Korangar use.
- Korangar server list: `korangar/korangar/archive/data/sclientinfo.xml` points the client at `127.0.0.1:6900`.
- Korangar asset search path: `korangar/korangar/src/loaders/archive/native/list.rs` defaults to `data.grf`, `rdata.grf`, and `archive/`.
- `.env` files: Not detected at repository root during analysis.

**Build:**
- Rust workspace manifest: `korangar/Cargo.toml`.
- Rust lockfile: `korangar/Cargo.lock`.
- Rust toolchain: `korangar/rust-toolchain.toml`.
- Cargo target config: `korangar/.cargo/config.toml`.
- Rust formatting config: `korangar/rustfmt.toml`.
- Korangar CI: `korangar/.github/workflows/build.yml`, `korangar/.github/workflows/tests.yml`, `korangar/.github/workflows/lint.yml`, `korangar/.github/workflows/formatting.yml`, and `korangar/.github/workflows/release.yml`.
- Docker runtime: `docker-compose.yml`, `docker/Dockerfile`, and `docker/entrypoint.sh`.
- rAthena configure build: `rathena-master/configure.ac`, `rathena-master/Makefile.in`, and generated `rathena-master/configure`.
- rAthena CMake build: `rathena-master/CMakeLists.txt`.
- rAthena packet config: `rathena-master/src/config/packets.hpp`.

## Platform Requirements

**Development:**
- Windows host supported by `play.bat` and documented paths in `README.md`.
- Docker Desktop or Docker Engine + Compose v2 is required for the local rAthena/MariaDB stack in `docker-compose.yml`.
- Rust + rustup are required; `korangar/rust-toolchain.toml` selects `nightly-2026-02-01` and installs `rust-src`, `rust-analyzer`, and `miri`.
- Vulkan SDK or standalone Slang compiler is required because `korangar/korangar/build.rs` invokes `slangc` to compile SPIR-V shaders.
- NASM is required for the `rav1d` dependency used by `korangar/korangar-video/Cargo.toml`.
- Official kRO `data.grf` and `rdata.grf` assets must be present in `korangar/korangar/` for the launcher flow documented in `README.md` and checked in `play.bat`.
- On Linux CI, Korangar installs `libasound2-dev`, `nasm`, and `libpcap-dev` in `.github/workflows/*.yml`.
- rAthena container builds need Docker-mounted source at `./rathena-master:/rathena` and compile tools from `docker/Dockerfile`.

**Production:**
- No production deployment target is defined.
- Current stack is a local development/lab setup: `docker-compose.yml` publishes MariaDB `3306`, login-server `6900`, char-server `6121`, and map-server `5121` on the host.
- CI/CD is GitHub Actions for the upstream Korangar/rAthena repositories; no deployment workflow exists in the root project.

---

*Stack analysis: 2026-04-26*
