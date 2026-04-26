# Architecture

**Analysis Date:** 2026-04-26

## Pattern Overview

**Overall:** Client-server Ragnarok development workspace with a Rust client, a patched C++ rAthena server, and Docker-based local orchestration.

**Key Characteristics:**
- `korangar/` is a Rust workspace organized into a main executable crate and reusable `korangar-*` / `ragnarok-*` support crates.
- `rathena-master/` is a multi-process C++ MMORPG server split into `login-server`, `char-server`, `map-server`, and `web-server` binaries.
- Root-level runtime glue in `docker/`, `docker-compose.yml`, and `play.bat` connects the patched server and Korangar client for local testing.
- Protocol compatibility is centered on `PACKETVER 20220406`, `korangar/ragnarok-packets/src/lib.rs`, `korangar/korangar-networking/src/packet_versions/version_20220406.rs`, and `rathena-master/src/config/packets.hpp`.

## Layers

**Workspace Orchestration:**
- Purpose: Defines how the local server stack and client are started together.
- Location: `README.md`, `AI_CONTEXT.md`, `play.bat`, `docker/`, `docker-compose.yml`
- Contains: Setup documentation, launcher script, Docker build/runtime image, config import overrides, database seed scripts.
- Depends on: `korangar/`, `rathena-master/`, Docker Compose, MariaDB.
- Used by: Developers running the local Ragnarok lab.

**Rust Client Workspace:**
- Purpose: Implements the Korangar client and reusable Ragnarok client-side libraries.
- Location: `korangar/Cargo.toml`, `korangar/korangar/`, `korangar/korangar-*`, `korangar/ragnarok-*`
- Contains: Main executable, rendering, networking, asset loading, UI framework, audio, collision, binary packet helpers, file format parsers.
- Depends on: `wgpu`, `winit`, `tokio`, `ragnarok-packets`, `ragnarok-formats`, `korangar-interface`, `korangar-networking`, `korangar-audio`.
- Used by: `play.bat`, `korangar/korangar/src/main.rs`, development builds via Cargo.

**Client Application Shell:**
- Purpose: Owns process startup, global resources, event loop integration, frame updates, and high-level gameplay event dispatch.
- Location: `korangar/korangar/src/main.rs`
- Contains: `main`, `Client`, `Client::init`, `Client::render_frame`, winit `ApplicationHandler`, render instruction buffers, loader handles, network buffers, input buffers, `ClientState`.
- Depends on: `korangar/korangar/src/graphics/`, `korangar/korangar/src/input/`, `korangar/korangar/src/loaders/`, `korangar/korangar/src/renderer/`, `korangar/korangar/src/state/`, `korangar/korangar/src/world/`, `korangar/korangar-networking/src/lib.rs`.
- Used by: The Korangar executable started from `korangar/target/release/korangar.exe` or `cargo run`.

**Client State and UI:**
- Purpose: Keeps UI-visible game state, persistent settings, window state, and theme state in a `rust-state` tree.
- Location: `korangar/korangar/src/state/mod.rs`, `korangar/korangar/src/interface/`, `korangar/korangar-interface/src/`
- Contains: `ClientState`, inventory, hotbar, skill tree, character slots, localization, themes, windows, component macros, layout/event/window stores.
- Depends on: `rust-state`, `korangar-interface`, `korangar-networking` data types, `ragnarok-packets` identifiers.
- Used by: `korangar/korangar/src/main.rs`, `korangar/korangar/src/interface/windows/*.rs`, `korangar/korangar/src/renderer/interface.rs`.

**Client Networking and Protocol:**
- Purpose: Converts TCP packet streams from login, character, and map servers into typed `NetworkEvent` values and sends typed client packets.
- Location: `korangar/korangar-networking/src/lib.rs`, `korangar/korangar-networking/src/event.rs`, `korangar/korangar-networking/src/server.rs`, `korangar/korangar-networking/src/packet_versions/version_20220406.rs`, `korangar/ragnarok-packets/src/`
- Contains: `NetworkingSystem`, `NetworkEventBuffer`, server connection state, tokio runtime thread, packet handlers, supported packet versions.
- Depends on: `tokio`, `ragnarok-bytes`, `ragnarok-packets`.
- Used by: `korangar/korangar/src/main.rs` during login, character selection, map entry, gameplay actions, and debug packet inspection.

**Client Asset Loading:**
- Purpose: Resolves Ragnarok assets from folders, GRF/native archives, and generated cache formats into typed client resources.
- Location: `korangar/korangar/src/loaders/`, `korangar/ragnarok-formats/src/`, `korangar/korangar-loaders/src/lib.rs`
- Contains: `GameFileLoader`, archive loaders, map/model/sprite/action/texture/font/effect/video loaders, cache helpers, fallback file constants.
- Depends on: `ragnarok-formats`, `sevenz-rust2`, image codecs, `korangar-container`.
- Used by: `Client::init` in `korangar/korangar/src/main.rs`, `AudioEngine` in `korangar/korangar-audio/src/lib.rs`, world resource construction in `korangar/korangar/src/world/`.

**Client World Model:**
- Purpose: Represents renderable and interactive game-world objects after protocol and asset data are decoded.
- Location: `korangar/korangar/src/world/`
- Contains: Entities, maps, lights, objects, effects, particles, cameras, pathing, sounds, videos, world libraries.
- Depends on: `korangar/korangar/src/loaders/`, `korangar/korangar/src/graphics/`, `korangar-collision`, `ragnarok-formats`, `ragnarok-packets`.
- Used by: Network event handlers in `korangar/korangar/src/main.rs`, renderers in `korangar/korangar/src/renderer/`, graphics passes in `korangar/korangar/src/graphics/passes/`.

**Client Rendering:**
- Purpose: Converts world and UI state into render instructions and executes wgpu render passes.
- Location: `korangar/korangar/src/renderer/`, `korangar/korangar/src/graphics/`, `korangar/korangar/shaders/`
- Contains: Interface/game/effect/debug renderers, `GraphicsEngine`, buffers, textures, surfaces, render passes, Slang shaders.
- Depends on: `wgpu`, `winit`, `bytemuck`, `korangar/korangar/src/world/`, `korangar/korangar/src/loaders/`.
- Used by: `Client::render_frame` in `korangar/korangar/src/main.rs`.

**Client Support Crates:**
- Purpose: Keep domain-specific functionality reusable outside the executable client.
- Location: `korangar/korangar-audio/`, `korangar/korangar-collision/`, `korangar/korangar-container/`, `korangar/korangar-debug/`, `korangar/korangar-interface/`, `korangar/korangar-video/`, `korangar/ragnarok-bytes/`, `korangar/ragnarok-formats/`, `korangar/ragnarok-macros/`, `korangar/ragnarok-packets/`
- Contains: Audio engine, spatial structures, caches/slabs, profiling/logging, UI framework, video decoding, byte readers/writers, format parsers, proc macros, packet structs.
- Depends on: Workspace dependencies declared in `korangar/Cargo.toml`.
- Used by: The main client crate `korangar/korangar/` and sometimes by each other through workspace path dependencies.

**rAthena Server Core:**
- Purpose: Provides the common process lifecycle, socket loop, timers, SQL wrapper, database containers, logging, and platform utilities for all rAthena binaries.
- Location: `rathena-master/src/common/`
- Contains: `Core`, `main_core`, `socket.cpp`, `timer.cpp`, `sql.cpp`, `db.cpp`, `showmsg.cpp`, `mmo.hpp`.
- Depends on: C/C++ runtime, MariaDB client libraries, platform socket APIs.
- Used by: `rathena-master/src/login/login.cpp`, `rathena-master/src/char/char.cpp`, `rathena-master/src/map/map.cpp`, `rathena-master/src/web/web.cpp`, tools in `rathena-master/src/tool/`.

**rAthena Login Server:**
- Purpose: Authenticates accounts, manages auth nodes and online users, accepts client login connections, and communicates with character servers.
- Location: `rathena-master/src/login/`
- Contains: `login.cpp`, `loginclif.cpp`, `loginchrif.cpp`, `account.cpp`, `ipban.cpp`, `loginlog.cpp`.
- Depends on: `rathena-master/src/common/`, SQL schema/config, login config files.
- Used by: Korangar login connection and `char-server` inter-server auth flow.

**rAthena Character Server:**
- Purpose: Handles character selection/creation/deletion, character persistence, guild/party/mail/storage inter-server data, and map-server registration.
- Location: `rathena-master/src/char/`
- Contains: `char.cpp`, `char_clif.cpp`, `char_logif.cpp`, `char_mapif.cpp`, `inter.cpp`, `int_*.cpp` persistence modules.
- Depends on: `rathena-master/src/common/`, MariaDB tables from `rathena-master/sql-files/`, login server, map servers.
- Used by: Korangar character selection flow, login server, map server.

**rAthena Map Server:**
- Purpose: Runs active gameplay simulation, packet parsing, scripts, mobs, skills, combat, movement, NPCs, inventory, status, and map state.
- Location: `rathena-master/src/map/`
- Contains: `map.cpp`, `clif.cpp`, `pc.cpp`, `skill.cpp`, `status.cpp`, `battle.cpp`, `script.cpp`, `npc.cpp`, `mob.cpp`, `unit.cpp`, `itemdb.cpp`.
- Depends on: `rathena-master/src/common/`, `rathena-master/conf/`, `rathena-master/db/`, `rathena-master/npc/`, character server.
- Used by: Korangar map connection and live gameplay.

**rAthena Configuration and Content:**
- Purpose: Defines server runtime configuration, database content, scripts, maps, commands, and SQL bootstrap data.
- Location: `rathena-master/conf/`, `rathena-master/db/`, `rathena-master/npc/`, `rathena-master/sql-files/`, `rathena-master/src/custom/`
- Contains: Server config files, import override targets, YAML/TXT game databases, NPC scripts, SQL schemas, customization include hooks.
- Depends on: rAthena loaders in `rathena-master/src/map/`, `rathena-master/src/char/`, `rathena-master/src/login/`.
- Used by: Docker runtime and all rAthena server binaries.

**Docker Runtime Layer:**
- Purpose: Builds and launches rAthena services in a repeatable local container stack.
- Location: `docker/Dockerfile`, `docker/entrypoint.sh`, `docker/import/*.txt`, `docker/init-db/zz-admin.sql`, `docker-compose.yml`
- Contains: Alpine toolchain image, server build/start commands, config import synchronization, database wait logic, admin seed script.
- Depends on: Mounted `rathena-master/`, MariaDB, Docker Compose service definitions.
- Used by: Root README setup flow and local server startup commands.

**Auxiliary Client Patch Tooling:**
- Purpose: Provides WARP/Nemo-style client patch assets and scripts independent of the Korangar runtime path.
- Location: `warp/`
- Contains: `warp/win32/WARP.exe`, `warp/Scripts/`, `warp/Patches/`, `warp/Inputs/`, `warp/Tables/`, `warp/Styles/`.
- Depends on: Bundled Windows/Qt runtime files in `warp/win32/`.
- Used by: Manual client patch research or tooling; it is not part of the Korangar/rAthena runtime flow.

**Runtime Snapshot:**
- Purpose: Holds a runnable Korangar binary plus extracted archive assets for testing outside the Cargo workspace layout.
- Location: `korangar-test/`
- Contains: `korangar-test/korangar.exe`, `korangar-test/archive/data/`.
- Depends on: Local asset files and the same server endpoints configured by `sclientinfo.xml`.
- Used by: Manual smoke testing.

## Data Flow

**Local Server Startup:**

1. `docker-compose.yml` defines the local MariaDB and rAthena service containers.
2. `docker/entrypoint.sh` copies `docker/import/*.txt` into `rathena-master/conf/import/`.
3. `docker/entrypoint.sh` builds `rathena-master/login-server`, `rathena-master/char-server`, and `rathena-master/map-server` when binaries are missing or on rebuild.
4. `docker/init-db/zz-admin.sql` participates in database initialization through the Compose database import path.
5. `login-server`, `char-server`, and `map-server` start against MariaDB and the copied import configs.

**Client Startup:**

1. `play.bat` changes the working directory to `korangar/korangar/` so relative assets such as `data.grf`, `rdata.grf`, and `archive/` are visible.
2. `korangar/korangar/src/main.rs` initializes random state, Rayon, settings, loaders, audio, networking, wgpu, winit, UI, and `ClientState`.
3. `korangar/korangar/src/main.rs` starts the winit event loop with `event_loop.run_app(&mut client)`.
4. `Client::render_frame` drains input and network event buffers, mutates `ClientState`, updates the world, prepares render instructions, and submits graphics work.

**Login to Gameplay:**

1. `korangar/korangar/src/loaders/server/client_info.rs` loads service information from `sclientinfo.xml`.
2. `korangar/korangar-networking/src/lib.rs` connects to the login server and maps login packets into `NetworkEvent::LoginServerConnected` or failure events.
3. `korangar/korangar/src/main.rs` stores login data, displays server selection, and opens the character server connection.
4. `rathena-master/src/login/loginclif.cpp` and `rathena-master/src/login/loginchrif.cpp` process client auth and login-to-char auth requests.
5. `rathena-master/src/char/char_clif.cpp` handles character selection and returns map login data.
6. `korangar/korangar-networking/src/lib.rs` connects to the map server and maps gameplay packets into `NetworkEvent` variants.
7. `korangar/korangar/src/main.rs` updates `ClientState`, `world` entities, map data, particles, effects, audio, and UI windows from those events.

**Frame Rendering:**

1. `korangar/korangar/src/input/mod.rs` converts winit input state into `InputReport` and `InputEvent` values.
2. `korangar/korangar/src/main.rs` resolves hovered targets using `PickerTarget` from `korangar/korangar/src/graphics/picker_target.rs`.
3. World objects in `korangar/korangar/src/world/` produce model/entity/light/effect/interface render instructions.
4. Render helpers in `korangar/korangar/src/renderer/` convert UI/world concepts into lower-level graphics instructions.
5. `korangar/korangar/src/graphics/` executes passes backed by Slang shaders in `korangar/korangar/shaders/`.

**Asset Loading:**

1. `korangar/korangar/src/loaders/gamefile/mod.rs` resolves game-file requests from folder/archive sources.
2. Specific loaders in `korangar/korangar/src/loaders/action/`, `animation/`, `map/`, `model/`, `sprite/`, `texture/`, `font/`, and `effect/` parse files.
3. Format structs from `korangar/ragnarok-formats/src/` and packet/byte helpers from `korangar/ragnarok-bytes/src/` are reused by loaders.
4. Runtime resources are stored in `korangar/korangar/src/world/` structures and cache/slab helpers from `korangar/korangar-container/src/`.
5. Fallback asset constants in `korangar/korangar/src/loaders/mod.rs` provide missing image/model/sprite/action paths.

**rAthena Server Runtime:**

1. Each binary calls `main_core<T>` from `rathena-master/src/common/core.hpp`.
2. `rathena-master/src/common/core.cpp` initializes common SQL, DB, signal, timer, and socket infrastructure.
3. Each concrete server initializes its own config, interfaces, SQL state, timers, packet parsers, and listening sockets.
4. The common loop calls `do_timer(gettick_nocache())`, then `do_sockets(next)`.
5. Server modules dispatch packet handlers through `loginclif`, `chclif`, `clif`, and inter-server interface modules.

**State Management:**
- Client state that is visible or mutable through the UI belongs in `korangar/korangar/src/state/mod.rs` inside `ClientState`.
- OS, GPU, network, audio, loader, and transient render resources belong to `Client` in `korangar/korangar/src/main.rs`.
- rAthena server state is mostly process-global and module-global across `rathena-master/src/common/`, `rathena-master/src/login/`, `rathena-master/src/char/`, and `rathena-master/src/map/`.
- Persistent server data is stored in MariaDB tables initialized from `rathena-master/sql-files/` and accessed through SQL wrappers such as `rathena-master/src/common/sql.cpp`.

## Key Abstractions

**Client:**
- Purpose: Owns Korangar runtime resources and the frame/event lifecycle.
- Examples: `korangar/korangar/src/main.rs`
- Pattern: Single application coordinator around winit, wgpu, loaders, networking, audio, and state.

**ClientState:**
- Purpose: Stores UI-visible game state and persistent settings.
- Examples: `korangar/korangar/src/state/mod.rs`
- Pattern: `rust-state` root with generated path extension traits and state-window support.

**NetworkEvent:**
- Purpose: Normalizes login, character, and map server packets into client events consumed by the main loop.
- Examples: `korangar/korangar-networking/src/event.rs`, `korangar/korangar/src/main.rs`
- Pattern: Large typed event enum drained from `NetworkEventBuffer` during each frame.

**NetworkingSystem:**
- Purpose: Manages three server connections on a dedicated tokio runtime thread.
- Examples: `korangar/korangar-networking/src/lib.rs`
- Pattern: Command/event channels around login, character, and map `ServerConnection` states.

**PacketHandler:**
- Purpose: Reads packet headers and dispatches payload decoding functions.
- Examples: `korangar/ragnarok-packets/src/handler.rs`, `korangar/korangar-networking/src/packet_versions/version_20220406.rs`
- Pattern: Header-to-handler map with packet callback hooks for debug inspection.

**GameFileLoader and Specific Loaders:**
- Purpose: Resolve asset bytes and parse them into runtime resources.
- Examples: `korangar/korangar/src/loaders/gamefile/mod.rs`, `korangar/korangar/src/loaders/map/mod.rs`, `korangar/korangar/src/loaders/texture/mod.rs`
- Pattern: Shared file source plus typed loaders and cache-aware resource construction.

**World Resources:**
- Purpose: Represent game objects after protocol and asset decoding.
- Examples: `korangar/korangar/src/world/entity/mod.rs`, `korangar/korangar/src/world/map/mod.rs`, `korangar/korangar/src/world/light/mod.rs`
- Pattern: Domain structs consumed by render instruction builders and gameplay handlers.

**Render Instructions and Passes:**
- Purpose: Decouple world/UI preparation from wgpu pass execution.
- Examples: `korangar/korangar/src/renderer/mod.rs`, `korangar/korangar/src/graphics/instruction.rs`, `korangar/korangar/src/graphics/passes/`
- Pattern: Renderer traits and instruction buffers feeding specialized graphics passes.

**Interface Framework:**
- Purpose: Provide declarative components, windows, event queues, layout, and themes.
- Examples: `korangar/korangar-interface/src/lib.rs`, `korangar/korangar/src/interface/windows/mod.rs`
- Pattern: Generic `Interface<'static, ClientState>` with `Application`, `StateWindow`, selectors, and component macros.

**AudioEngine:**
- Purpose: Manage background music, sound effects, spatial audio, cache, and async sound loading.
- Examples: `korangar/korangar-audio/src/lib.rs`
- Pattern: Mutex-protected engine context with channels, slabs, caches, and cpal-backed manager.

**rAthena Core:**
- Purpose: Standardize lifecycle and main loop behavior for all server binaries.
- Examples: `rathena-master/src/common/core.hpp`, `rathena-master/src/common/core.cpp`
- Pattern: Template `main_core<T>` plus virtual `Core::initialize`, `Core::handle_main`, `Core::finalize`, `Core::handle_shutdown`.

**rAthena Interface Modules:**
- Purpose: Split external and inter-server packet handling by process and peer type.
- Examples: `rathena-master/src/login/loginclif.cpp`, `rathena-master/src/login/loginchrif.cpp`, `rathena-master/src/char/char_clif.cpp`, `rathena-master/src/char/char_logif.cpp`, `rathena-master/src/char/char_mapif.cpp`, `rathena-master/src/map/clif.cpp`, `rathena-master/src/map/chrif.cpp`
- Pattern: C-style module functions initialized by each process and registered with common socket parsing.

**rAthena Gameplay Modules:**
- Purpose: Implement game rules and content systems.
- Examples: `rathena-master/src/map/pc.cpp`, `rathena-master/src/map/skill.cpp`, `rathena-master/src/map/status.cpp`, `rathena-master/src/map/battle.cpp`, `rathena-master/src/map/script.cpp`, `rathena-master/src/map/npc.cpp`
- Pattern: Large module-global systems initialized from `map.cpp` and backed by DB/config/script loaders.

## Entry Points

**Root Setup Documentation:**
- Location: `README.md`
- Triggers: Developer opens the repository or follows local setup.
- Responsibilities: Defines project intent, Docker startup, server ports, rAthena/Korangar compatibility notes, and client build steps.

**AI/Debug Briefing:**
- Location: `AI_CONTEXT.md`
- Triggers: Assistant or developer starts a debugging session.
- Responsibilities: Summarizes current runtime state, patched files, known bugs, and investigation entry points.

**Client Launcher:**
- Location: `play.bat`
- Triggers: Developer runs the batch script on Windows.
- Responsibilities: Verifies `korangar/target/release/korangar.exe`, verifies client GRF assets, changes CWD to `korangar/korangar/`, and starts the client.

**Korangar Executable:**
- Location: `korangar/korangar/src/main.rs`
- Triggers: `korangar.exe` or `cargo run`.
- Responsibilities: Initializes systems, corrects working directory when possible, creates `Client`, starts winit event loop, processes frames.

**Korangar Networking Thread:**
- Location: `korangar/korangar-networking/src/lib.rs`
- Triggers: `NetworkingSystem::spawn` or `NetworkingSystem::spawn_with_callback`.
- Responsibilities: Creates a tokio current-thread runtime and handles login, character, and map TCP tasks.

**rAthena Docker Entrypoint:**
- Location: `docker/entrypoint.sh`
- Triggers: Docker service command `build`, `rebuild`, `login`, `char`, `map`, or shell command.
- Responsibilities: Synchronizes config imports, builds rAthena binaries, waits for dependencies, starts requested server binary.

**rAthena Login Server:**
- Location: `rathena-master/src/login/login.cpp`
- Triggers: `rathena-master/login-server`.
- Responsibilities: Loads login config, initializes account DB, login client interface, char-server interface, IP ban system, timers, and listen socket.

**rAthena Character Server:**
- Location: `rathena-master/src/char/char.cpp`
- Triggers: `rathena-master/char-server`.
- Responsibilities: Loads char/inter configs, initializes character SQL, inter-server modules, login/map interfaces, timers, and listen socket.

**rAthena Map Server:**
- Location: `rathena-master/src/map/map.cpp`
- Triggers: `rathena-master/map-server`.
- Responsibilities: Loads map/battle/script/inter/log config, map DBs, GRF data when enabled, maps, NPCs, gameplay modules, timers, and gameplay listen socket.

**rAthena Web Server:**
- Location: `rathena-master/src/web/web.cpp`
- Triggers: `rathena-master/web-server`.
- Responsibilities: Provides rAthena web controller endpoints such as config, emblems, merchant stores, party booking, and user config.

**WARP Tool:**
- Location: `warp/win32/WARP.exe`
- Triggers: Manual execution.
- Responsibilities: Applies client patch workflows using data in `warp/Scripts/`, `warp/Patches/`, `warp/Inputs/`, and `warp/Tables/`.

## Error Handling

**Strategy:** Use typed Rust results/events in Korangar, process-level fatal/status reporting in rAthena, and fail-fast shell behavior in Docker runtime scripts.

**Patterns:**
- Korangar startup returns early from `Client::init` through `Option<Self>` in `korangar/korangar/src/main.rs`.
- Korangar user-facing failures become UI windows such as `ErrorWindow` from `korangar/korangar/src/interface/windows/error.rs`.
- Korangar networking emits explicit failure/disconnect events through `NetworkEvent` variants in `korangar/korangar-networking/src/event.rs`.
- Packet parsing reports `UnhandledPacket`, `PacketCutOff`, or `InternalError` through `korangar/ragnarok-packets/src/handler.rs`.
- rAthena initialization reports fatal errors and returns `false` from server `initialize` methods in `rathena-master/src/login/login.cpp`, `rathena-master/src/char/char.cpp`, and `rathena-master/src/map/map.cpp`.
- rAthena SQL errors use `Sql_ShowDebug` and SQL return codes through `rathena-master/src/common/sql.cpp`.
- Docker runtime uses `set -euo pipefail` in `docker/entrypoint.sh`; missing DB readiness or failed builds stop the container command.

## Cross-Cutting Concerns

**Logging:** Korangar debug logging/profiling is feature-gated through `korangar/korangar-debug/` and `#[cfg(feature = "debug")]` paths in `korangar/korangar/src/main.rs`. rAthena logs through `ShowStatus`, `ShowWarning`, `ShowFatalError`, and related functions from `rathena-master/src/common/showmsg.cpp`.

**Validation:** Korangar validates packet boundaries in `korangar/ragnarok-packets/src/handler.rs`, validates asset availability through loader fallbacks in `korangar/korangar/src/loaders/mod.rs`, and gates debug-only state behind feature flags. rAthena validates configs and DB tables during server initialization in `rathena-master/src/login/login.cpp`, `rathena-master/src/char/char.cpp`, and `rathena-master/src/map/map.cpp`.

**Authentication:** Korangar initiates login through `korangar/korangar-networking/src/lib.rs` and stores login transition data in `korangar/korangar/src/main.rs`. rAthena account authentication and login-to-char auth nodes live in `rathena-master/src/login/account.cpp`, `rathena-master/src/login/loginclif.cpp`, and `rathena-master/src/login/loginchrif.cpp`.

---

*Architecture analysis: 2026-04-26*
