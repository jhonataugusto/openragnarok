# Codebase Structure

**Analysis Date:** 2026-04-26

## Directory Layout

```text
ragnarok/
├── README.md                         # Project intent, local Docker setup, Korangar connection notes
├── AI_CONTEXT.md                     # Current debugging context and key known integration issues
├── GM_COMMANDS.md                    # Useful rAthena GM command notes
├── play.bat                         # Windows launcher for the Korangar client
├── docker-compose.yml                # Local service topology for MariaDB and rAthena containers
├── docker/                           # rAthena build/runtime Docker image, config imports, DB seed scripts
├── korangar/                         # Rust Korangar client workspace
├── rathena-master/                   # Patched rAthena C++ server source and built binaries
├── korangar-rathena-patches/         # Patch files applied from the Korangar rAthena compatibility set
├── korangar-test/                    # Standalone Korangar test binary and archive assets
├── warp/                             # WARP/Nemo-style client patching tool and data
└── .planning/codebase/               # Generated codebase intelligence documents
```

## Directory Purposes

**Root:**
- Purpose: Coordinates the local Ragnarok engineering workspace.
- Contains: Human-readable project notes, runtime launcher, Docker Compose stack, client/server/tool subtrees.
- Key files: `README.md`, `AI_CONTEXT.md`, `GM_COMMANDS.md`, `play.bat`, `docker-compose.yml`

**`.planning/codebase/`:**
- Purpose: Stores generated codebase maps consumed by GSD planning and execution commands.
- Contains: Architecture, structure, stack, integrations, conventions, testing, and concerns documents.
- Key files: `.planning/codebase/ARCHITECTURE.md`, `.planning/codebase/STRUCTURE.md`

**`docker/`:**
- Purpose: Builds and starts the local rAthena server binaries inside containers.
- Contains: Docker image definition, entrypoint script, config overrides, DB initialization SQL.
- Key files: `docker/Dockerfile`, `docker/entrypoint.sh`, `docker/import/inter_conf.txt`, `docker/import/login_conf.txt`, `docker/import/char_conf.txt`, `docker/import/map_conf.txt`, `docker/init-db/zz-admin.sql`

**`korangar/`:**
- Purpose: Holds the Korangar Rust workspace.
- Contains: Cargo workspace metadata, main client crate, reusable `korangar-*` crates, reusable `ragnarok-*` crates, shaders, assets, and build outputs.
- Key files: `korangar/Cargo.toml`, `korangar/Cargo.lock`, `korangar/rust-toolchain.toml`, `korangar/korangar/Cargo.toml`, `korangar/korangar/src/main.rs`

**`korangar/korangar/`:**
- Purpose: Main Korangar executable crate and runtime asset directory.
- Contains: Application code, shaders, theme/config files, archive assets, GRF files, built executable copy.
- Key files: `korangar/korangar/src/main.rs`, `korangar/korangar/Cargo.toml`, `korangar/korangar/build.rs`, `korangar/korangar/shaders/README.md`, `korangar/korangar/client/login_settings.ron`

**`korangar/korangar/src/`:**
- Purpose: Main client application modules.
- Contains: `graphics`, `input`, `interface`, `loaders`, `networking` debug helpers, `renderer`, `settings`, `state`, `system`, and `world`.
- Key files: `korangar/korangar/src/main.rs`, `korangar/korangar/src/state/mod.rs`, `korangar/korangar/src/loaders/mod.rs`, `korangar/korangar/src/world/mod.rs`

**`korangar/korangar/src/graphics/`:**
- Purpose: Low-level wgpu graphics engine and render passes.
- Contains: Buffers, textures, surfaces, samplers, shader compiler, render instructions, render pass implementations, vertices.
- Key files: `korangar/korangar/src/graphics/mod.rs`, `korangar/korangar/src/graphics/engine.rs`, `korangar/korangar/src/graphics/instruction.rs`, `korangar/korangar/src/graphics/passes/mod.rs`

**`korangar/korangar/shaders/`:**
- Purpose: Slang shader source matched to graphics passes.
- Contains: Shared shader modules and pass-specific shaders for forward, shadows, interface, picker, postprocessing, mipmap, SDSM, and screen blit.
- Key files: `korangar/korangar/shaders/modules/forward.slang`, `korangar/korangar/shaders/modules/interface.slang`, `korangar/korangar/shaders/passes/forward/entity.slang`, `korangar/korangar/shaders/passes/interface/rectangle.slang`

**`korangar/korangar/src/input/`:**
- Purpose: Converts winit keyboard, mouse, wheel, and picker state into input reports/events.
- Contains: Input event definitions, key state, mouse modes, `InputSystem`.
- Key files: `korangar/korangar/src/input/mod.rs`, `korangar/korangar/src/input/event.rs`, `korangar/korangar/src/input/key.rs`, `korangar/korangar/src/input/mode.rs`

**`korangar/korangar/src/interface/`:**
- Purpose: Korangar-specific UI windows, cursor state, item/skill components, and window composition macros.
- Contains: Client windows, cursor handling, resource source helpers, reusable item/skill boxes.
- Key files: `korangar/korangar/src/interface/windows/mod.rs`, `korangar/korangar/src/interface/windows/login.rs`, `korangar/korangar/src/interface/windows/character_selection.rs`, `korangar/korangar/src/interface/windows/chat.rs`, `korangar/korangar/src/interface/components/item_box.rs`

**`korangar/korangar/src/loaders/`:**
- Purpose: Loads client assets and server metadata into typed runtime data.
- Contains: Archive, async, game file, map, model, texture, sprite, action, animation, font, effect, server, and video loaders.
- Key files: `korangar/korangar/src/loaders/mod.rs`, `korangar/korangar/src/loaders/gamefile/mod.rs`, `korangar/korangar/src/loaders/server/client_info.rs`, `korangar/korangar/src/loaders/map/mod.rs`, `korangar/korangar/src/loaders/texture/mod.rs`

**`korangar/korangar/src/renderer/`:**
- Purpose: Produces render instructions for game UI, interface, effects, and debug markers.
- Contains: Renderer traits and instruction producers separate from low-level graphics pass execution.
- Key files: `korangar/korangar/src/renderer/mod.rs`, `korangar/korangar/src/renderer/game_interface.rs`, `korangar/korangar/src/renderer/interface.rs`, `korangar/korangar/src/renderer/effect.rs`

**`korangar/korangar/src/settings/`:**
- Purpose: Defines persistent settings files and settings path extensions.
- Contains: Audio, game, graphics, interface, and login settings modules.
- Key files: `korangar/korangar/src/settings/mod.rs`, `korangar/korangar/src/settings/audio.rs`, `korangar/korangar/src/settings/graphic.rs`, `korangar/korangar/src/settings/login.rs`

**`korangar/korangar/src/state/`:**
- Purpose: Stores UI-visible client state and sub-states.
- Contains: `ClientState`, character slots, hotbar, inventory, localization, skills, themes, debug cache statistics.
- Key files: `korangar/korangar/src/state/mod.rs`, `korangar/korangar/src/state/inventory.rs`, `korangar/korangar/src/state/hotbar.rs`, `korangar/korangar/src/state/skills.rs`, `korangar/korangar/src/state/theme/mod.rs`

**`korangar/korangar/src/system/`:**
- Purpose: Holds small system utilities for the client runtime.
- Contains: Game timer re-export.
- Key files: `korangar/korangar/src/system/mod.rs`, `korangar/korangar/src/system/timer.rs`

**`korangar/korangar/src/world/`:**
- Purpose: Represents world entities, maps, cameras, effects, resources, pathing, lights, sounds, and videos.
- Contains: Entity model, map model, world libraries, camera implementations, particles, path finder, resource metadata.
- Key files: `korangar/korangar/src/world/mod.rs`, `korangar/korangar/src/world/entity/mod.rs`, `korangar/korangar/src/world/map/mod.rs`, `korangar/korangar/src/world/pathing.rs`, `korangar/korangar/src/world/library/mod.rs`

**`korangar/korangar-audio/`:**
- Purpose: Provides audio playback, sound effect cache, spatial tracks, background music, and backend abstractions.
- Contains: Audio engine, cpal backend, sound/track/manager code, resampling, tweening.
- Key files: `korangar/korangar-audio/src/lib.rs`, `korangar/korangar-audio/src/backend/cpal/mod.rs`, `korangar/korangar-audio/src/sound.rs`, `korangar/korangar-audio/src/track.rs`

**`korangar/korangar-collision/`:**
- Purpose: Provides geometric structures and collision/spatial-query helpers.
- Contains: AABB, sphere, planes, frustum, KD-tree.
- Key files: `korangar/korangar-collision/src/lib.rs`, `korangar/korangar-collision/src/kdtree.rs`, `korangar/korangar-collision/src/aabb.rs`

**`korangar/korangar-container/`:**
- Purpose: Provides generational/simple slabs and simple caches.
- Contains: Key macros, slabs, cache structures.
- Key files: `korangar/korangar-container/src/lib.rs`, `korangar/korangar-container/src/generational_slab.rs`, `korangar/korangar-container/src/simple_cache.rs`

**`korangar/korangar-debug/`:**
- Purpose: Provides debug-only logging and profiling utilities.
- Contains: Logging/profiling modules and proc macros.
- Key files: `korangar/korangar-debug/src/lib.rs`, `korangar/korangar-debug/src/logging/mod.rs`, `korangar/korangar-debug/src/profiling/mod.rs`, `korangar/korangar-debug/macros/src/lib.rs`

**`korangar/korangar-interface/`:**
- Purpose: Provides the reusable immediate/declarative UI framework used by Korangar.
- Contains: Application trait, components, elements, event queue, layout, theme, window system, component macros.
- Key files: `korangar/korangar-interface/src/lib.rs`, `korangar/korangar-interface/src/application.rs`, `korangar/korangar-interface/src/window/mod.rs`, `korangar/korangar-interface/src/layout/mod.rs`

**`korangar/korangar-networking/`:**
- Purpose: Provides protocol connection management and converts packets to high-level events.
- Contains: `NetworkingSystem`, event enum, server connection state, item/entity DTOs, packet-version handlers.
- Key files: `korangar/korangar-networking/src/lib.rs`, `korangar/korangar-networking/src/event.rs`, `korangar/korangar-networking/src/server.rs`, `korangar/korangar-networking/src/packet_versions/version_20220406.rs`

**`korangar/korangar-video/`:**
- Purpose: Provides video decoding support for client assets.
- Contains: IVF-related parsing and crate-level video helpers.
- Key files: `korangar/korangar-video/src/lib.rs`, `korangar/korangar-video/src/ivf/mod.rs`

**`korangar/ragnarok-bytes/`:**
- Purpose: Provides byte readers/writers and binary conversion helpers for Ragnarok formats and packets.
- Contains: Encoding helpers, byte reader/writer, derive support.
- Key files: `korangar/ragnarok-bytes/src/lib.rs`, `korangar/ragnarok-bytes/src/reader.rs`, `korangar/ragnarok-bytes/src/writer.rs`

**`korangar/ragnarok-formats/`:**
- Purpose: Provides typed parsers/structures for Ragnarok asset formats.
- Contains: Action, archive, color, effect, map, model, signature, sprite, transform, version modules.
- Key files: `korangar/ragnarok-formats/src/lib.rs`, `korangar/ragnarok-formats/src/map/mod.rs`, `korangar/ragnarok-formats/src/model/mod.rs`, `korangar/ragnarok-formats/src/sprite/mod.rs`

**`korangar/ragnarok-macros/`:**
- Purpose: Provides proc macros used by Ragnarok byte/format/packet code.
- Contains: Macro implementation files.
- Key files: `korangar/ragnarok-macros/src/lib.rs`

**`korangar/ragnarok-packets/`:**
- Purpose: Defines Ragnarok packet structs, packet headers, packet handler logic, and position helpers.
- Contains: Large packet definitions file, handler map, position helpers.
- Key files: `korangar/ragnarok-packets/src/lib.rs`, `korangar/ragnarok-packets/src/handler.rs`, `korangar/ragnarok-packets/src/position.rs`

**`rathena-master/`:**
- Purpose: Holds the patched rAthena server source tree, configuration, scripts, SQL, tools, and built server binaries.
- Contains: `src`, `conf`, `db`, `npc`, `sql-files`, `tools`, root build files, generated binaries.
- Key files: `rathena-master/CMakeLists.txt`, `rathena-master/Makefile`, `rathena-master/login-server`, `rathena-master/char-server`, `rathena-master/map-server`, `rathena-master/web-server`

**`rathena-master/src/common/`:**
- Purpose: Shared server runtime code.
- Contains: Core lifecycle, socket loop, timers, SQL wrapper, DB containers, logging, string utilities, packet base definitions.
- Key files: `rathena-master/src/common/core.hpp`, `rathena-master/src/common/core.cpp`, `rathena-master/src/common/socket.cpp`, `rathena-master/src/common/timer.cpp`, `rathena-master/src/common/sql.cpp`

**`rathena-master/src/login/`:**
- Purpose: Login server implementation.
- Contains: Account DB, login client interface, login-char interface, console interface, IP ban, login logging.
- Key files: `rathena-master/src/login/login.cpp`, `rathena-master/src/login/loginclif.cpp`, `rathena-master/src/login/loginchrif.cpp`, `rathena-master/src/login/account.cpp`

**`rathena-master/src/char/`:**
- Purpose: Character server implementation.
- Contains: Character lifecycle, client interface, login interface, map interface, inter-server persistence modules.
- Key files: `rathena-master/src/char/char.cpp`, `rathena-master/src/char/char_clif.cpp`, `rathena-master/src/char/char_logif.cpp`, `rathena-master/src/char/char_mapif.cpp`, `rathena-master/src/char/inter.cpp`

**`rathena-master/src/map/`:**
- Purpose: Map server and gameplay simulation implementation.
- Contains: Client interface, char interface, player/mob/NPC/skill/status/combat/script/game systems.
- Key files: `rathena-master/src/map/map.cpp`, `rathena-master/src/map/clif.cpp`, `rathena-master/src/map/pc.cpp`, `rathena-master/src/map/skill.cpp`, `rathena-master/src/map/status.cpp`, `rathena-master/src/map/script.cpp`

**`rathena-master/src/web/`:**
- Purpose: rAthena web server and controllers.
- Contains: Web entry point, auth, config, emblem, merchant store, party booking, SQL lock, user config controllers.
- Key files: `rathena-master/src/web/web.cpp`, `rathena-master/src/web/http.hpp`, `rathena-master/src/web/auth.cpp`, `rathena-master/src/web/partybooking_controller.cpp`

**`rathena-master/src/tool/`:**
- Purpose: rAthena command-line tools.
- Contains: YAML/SQL conversion, map cache tooling, upgrade tooling.
- Key files: `rathena-master/src/tool/csv2yaml.cpp`, `rathena-master/src/tool/yaml2sql.cpp`, `rathena-master/src/tool/yamlupgrade.cpp`, `rathena-master/src/tool/mapcache.cpp`

**`rathena-master/src/config/`:**
- Purpose: Compile-time server configuration headers.
- Contains: Packet version/obfuscation configuration, core config, renewal/security headers.
- Key files: `rathena-master/src/config/packets.hpp`, `rathena-master/src/config/core.hpp`, `rathena-master/src/config/renewal.hpp`, `rathena-master/src/config/secure.hpp`

**`rathena-master/src/custom/`:**
- Purpose: rAthena customization include hooks.
- Contains: Custom atcommand, script, battle config, and define include files.
- Key files: `rathena-master/src/custom/atcommand.inc`, `rathena-master/src/custom/script.inc`, `rathena-master/src/custom/defines_pre.hpp`, `rathena-master/src/custom/defines_post.hpp`

**`rathena-master/conf/`:**
- Purpose: Runtime server configuration.
- Contains: Main config files, battle config, import and import-template directories, message configs, groups, atcommands.
- Key files: `rathena-master/conf/login_athena.conf`, `rathena-master/conf/char_athena.conf`, `rathena-master/conf/map_athena.conf`, `rathena-master/conf/inter_athena.conf`, `rathena-master/conf/groups.yml`

**`rathena-master/db/`:**
- Purpose: Server game database content.
- Contains: Renewal/pre-renewal DB data, imports, templates.
- Key files: `rathena-master/db/re/`, `rathena-master/db/pre-re/`, `rathena-master/db/import/`

**`rathena-master/npc/`:**
- Purpose: NPC script content.
- Contains: Cities, jobs, quests, events, instances, warps, mobs, Kafras, custom scripts.
- Key files: `rathena-master/npc/custom/`, `rathena-master/npc/warps/`, `rathena-master/npc/quests/`, `rathena-master/npc/cities/`

**`rathena-master/sql-files/`:**
- Purpose: Database schema and seed SQL for rAthena.
- Contains: Main/log/web schemas and game data SQL.
- Key files: `rathena-master/sql-files/main.sql`, `rathena-master/sql-files/logs.sql`, `rathena-master/sql-files/web.sql`

**`korangar-rathena-patches/`:**
- Purpose: Documents and stores Korangar compatibility patches applied to rAthena.
- Contains: Patch files for packet obfuscation, account creation, char deletion, slot movement, rates, area size, and gameplay tuning.
- Key files: `korangar-rathena-patches/packet-obfuscation.patch`, `korangar-rathena-patches/new-account.patch`, `korangar-rathena-patches/disable-pin-code.patch`, `korangar-rathena-patches/area-size.patch`

**`korangar-test/`:**
- Purpose: Standalone client runtime test directory.
- Contains: `korangar.exe` and `archive/data` assets.
- Key files: `korangar-test/korangar.exe`, `korangar-test/archive/data/sclientinfo.xml`, `korangar-test/archive/data/languages/en-US.ron`

**`warp/`:**
- Purpose: Client executable patching tool distribution and patch definitions.
- Contains: WARP binaries, Qt runtime, patch scripts, input specs, patch YAML, languages, styles, tables, image/font assets.
- Key files: `warp/win32/WARP.exe`, `warp/Scripts/Patches/`, `warp/Patches/Client.yml`, `warp/Patches/Network.yml`, `warp/Inputs/NemoMap.yml`

## Key File Locations

**Entry Points:**
- `README.md`: Project setup and high-level workflow.
- `AI_CONTEXT.md`: Debugging briefing and current integration state.
- `play.bat`: Windows client launcher.
- `korangar/korangar/src/main.rs`: Korangar executable entry point and application loop.
- `korangar/korangar-networking/src/lib.rs`: Networking runtime entry point.
- `docker/entrypoint.sh`: rAthena container command dispatcher.
- `rathena-master/src/login/login.cpp`: Login server entry point.
- `rathena-master/src/char/char.cpp`: Character server entry point.
- `rathena-master/src/map/map.cpp`: Map server entry point.
- `rathena-master/src/web/web.cpp`: Web server entry point.

**Configuration:**
- `docker-compose.yml`: Local service topology. Treat as potentially containing runtime credentials; avoid quoting values into docs.
- `docker/import/*.txt`: rAthena config overrides copied into `rathena-master/conf/import/`.
- `docker/init-db/zz-admin.sql`: Local DB seed script. Treat account data as sensitive in docs.
- `korangar/Cargo.toml`: Rust workspace members and dependencies.
- `korangar/korangar/Cargo.toml`: Main client crate dependencies and features.
- `korangar/rust-toolchain.toml`: Rust toolchain pin.
- `korangar/rustfmt.toml`: Rust formatting settings.
- `korangar/korangar/client/*.ron`: Client runtime settings.
- `korangar/korangar/archive/data/sclientinfo.xml`: Client service connection metadata.
- `rathena-master/conf/*.conf`: rAthena runtime config.
- `rathena-master/conf/import/`: Effective rAthena import override destination.
- `rathena-master/src/config/packets.hpp`: Compile-time packet version and obfuscation settings.

**Core Logic:**
- `korangar/korangar/src/main.rs`: Main client coordinator and event handling.
- `korangar/korangar-networking/src/event.rs`: Client-side networking event contract.
- `korangar/korangar-networking/src/packet_versions/version_20220406.rs`: Packet handlers for the active packet version.
- `korangar/ragnarok-packets/src/lib.rs`: Packet type definitions.
- `korangar/ragnarok-packets/src/handler.rs`: Packet dispatch abstraction.
- `korangar/korangar/src/state/mod.rs`: UI-visible client state root.
- `korangar/korangar/src/world/entity/mod.rs`: Entity behavior and animation state transitions.
- `korangar/korangar/src/world/map/mod.rs`: Client map model.
- `korangar/korangar/src/loaders/server/client_info.rs`: `sclientinfo.xml` parsing.
- `rathena-master/src/common/core.cpp`: rAthena common lifecycle and main loop.
- `rathena-master/src/common/socket.cpp`: rAthena socket layer.
- `rathena-master/src/common/sql.cpp`: rAthena SQL wrapper.
- `rathena-master/src/login/loginclif.cpp`: Login client packet flow.
- `rathena-master/src/login/loginchrif.cpp`: Login-to-char auth flow.
- `rathena-master/src/char/char_clif.cpp`: Character server client packet flow.
- `rathena-master/src/map/clif.cpp`: Map server client packet flow.
- `rathena-master/src/map/pc.cpp`: Player character gameplay logic.

**Testing:**
- `korangar/*/src/**/*.rs`: Rust unit tests are expected to live inline or in crate-level test modules when added.
- `korangar/.github/workflows/tests.yml`: Rust workspace test automation.
- `rathena-master/tools/ci/`: rAthena script/SQL CI helpers.
- `korangar-test/`: Manual runtime smoke-test directory.

## Naming Conventions

**Files:**
- Rust modules use snake_case filenames: `korangar/korangar/src/world/ground_item.rs`, `korangar/korangar/src/interface/windows/character_selection.rs`.
- Rust directories use snake_case when they represent modules: `korangar/korangar/src/loaders/gamefile/`, `korangar/korangar/src/world/skill_tree/` through `korangar/korangar/src/interface/windows/skill_tree/`.
- Rust crate directories use kebab-case: `korangar/korangar-networking/`, `korangar/ragnarok-packets/`, `korangar/korangar-interface/`.
- rAthena C++ files use lower snake_case module names or historical compact names: `rathena-master/src/map/buyingstore.cpp`, `rathena-master/src/map/clif.cpp`, `rathena-master/src/login/loginchrif.cpp`.
- rAthena headers mirror implementation filenames: `rathena-master/src/map/pc.cpp` with `rathena-master/src/map/pc.hpp`.
- rAthena config files use descriptive snake_case with `.conf`, `.yml`, `.txt`, `.hpp`, or `.inc`: `rathena-master/conf/login_athena.conf`, `rathena-master/src/custom/atcommand.inc`.
- Patch files use kebab-case: `korangar-rathena-patches/packet-obfuscation.patch`.
- WARP patch scripts use PascalCase or abbreviated command names as supplied by the upstream tool: `warp/Scripts/Patches/NoHardCodedIP.qjs`, `warp/Scripts/Init/Packet.mjs`.

**Directories:**
- Rust workspace crates are top-level directories under `korangar/` matching Cargo package names.
- Rust module trees use `mod.rs` for directory modules: `korangar/korangar/src/world/mod.rs`, `korangar/korangar/src/loaders/mod.rs`.
- rAthena source directories match process or shared subsystem boundaries: `rathena-master/src/common/`, `rathena-master/src/login/`, `rathena-master/src/char/`, `rathena-master/src/map/`, `rathena-master/src/web/`, `rathena-master/src/tool/`.
- rAthena content directories separate config, DB, NPC scripts, and SQL bootstrap data: `rathena-master/conf/`, `rathena-master/db/`, `rathena-master/npc/`, `rathena-master/sql-files/`.

## Where to Add New Code

**New Client Gameplay Handler:**
- Primary code: Add protocol event handling in `korangar/korangar/src/main.rs` when the behavior coordinates state, world, UI, audio, and networking.
- Packet event contract: Add or extend events in `korangar/korangar-networking/src/event.rs`.
- Packet parsing: Add packet definitions in `korangar/ragnarok-packets/src/lib.rs` and register active-version handlers in `korangar/korangar-networking/src/packet_versions/version_20220406.rs`.
- State: Add UI-visible fields to `ClientState` in `korangar/korangar/src/state/mod.rs` or a focused submodule under `korangar/korangar/src/state/`.
- Tests: Add focused Rust tests near the module under `korangar/` where the behavior is implemented.

**New Client UI Window:**
- Primary code: Add a window file under `korangar/korangar/src/interface/windows/`.
- Registration/export: Update `korangar/korangar/src/interface/windows/mod.rs`.
- State: Add persistent or interactive state to `korangar/korangar/src/state/mod.rs` or a window state type in the new window file.
- Reusable widgets: Add Korangar-specific widgets to `korangar/korangar/src/interface/components/`; add generic UI framework widgets to `korangar/korangar-interface/src/components/`.

**New Client Asset Loader:**
- Primary code: Add a focused module under `korangar/korangar/src/loaders/`.
- Format parsing: Add reusable format structures to `korangar/ragnarok-formats/src/` when the parser is useful beyond the executable client.
- Byte-level parsing: Add helpers to `korangar/ragnarok-bytes/src/` only for reusable binary read/write behavior.
- Exports: Update `korangar/korangar/src/loaders/mod.rs` or `korangar/ragnarok-formats/src/lib.rs`.

**New Client World Feature:**
- Primary code: Add domain model code under `korangar/korangar/src/world/`.
- Rendering bridge: Add instruction production under `korangar/korangar/src/renderer/`.
- GPU implementation: Add low-level resources or passes under `korangar/korangar/src/graphics/` and matching shaders under `korangar/korangar/shaders/`.
- State: Keep UI-observable data in `korangar/korangar/src/state/`; keep render/runtime resources on `Client` or world structs.

**New Graphics Pass:**
- Primary code: Add Rust pass implementation under `korangar/korangar/src/graphics/passes/<pass_name>/`.
- Shaders: Add matching Slang files under `korangar/korangar/shaders/passes/<pass_name>/` and shared helpers under `korangar/korangar/shaders/modules/` only when multiple passes need them.
- Exports: Update `korangar/korangar/src/graphics/passes/mod.rs`.
- Renderer bridge: Add instruction producers under `korangar/korangar/src/renderer/` when the pass needs world/UI data.

**New Reusable Rust Support Logic:**
- Client-only logic: Add under `korangar/korangar/src/`.
- Generic UI framework logic: Add under `korangar/korangar-interface/src/`.
- Networking/protocol logic: Add under `korangar/korangar-networking/src/` and `korangar/ragnarok-packets/src/`.
- Format parsing: Add under `korangar/ragnarok-formats/src/`.
- Containers/caches/slabs: Add under `korangar/korangar-container/src/`.
- Audio: Add under `korangar/korangar-audio/src/`.

**New rAthena Login Behavior:**
- Primary code: Add client login packet behavior in `rathena-master/src/login/loginclif.cpp`.
- Account persistence: Add account behavior in `rathena-master/src/login/account.cpp`.
- Login-to-char behavior: Add inter-server behavior in `rathena-master/src/login/loginchrif.cpp`.
- Config: Add defaults/importable config in `rathena-master/conf/login_athena.conf` or `docker/import/login_conf.txt` when local Docker needs an override.

**New rAthena Character Behavior:**
- Primary code: Add client-facing character selection behavior in `rathena-master/src/char/char_clif.cpp`.
- Login interface: Add login-server communication in `rathena-master/src/char/char_logif.cpp`.
- Map interface: Add map-server communication in `rathena-master/src/char/char_mapif.cpp`.
- Persistent inter data: Add focused modules in `rathena-master/src/char/int_*.cpp` or `rathena-master/src/char/inter.cpp`.

**New rAthena Map Gameplay Behavior:**
- Primary code: Use the existing map module matching the domain:
  - Player/session behavior: `rathena-master/src/map/pc.cpp`
  - Client packet parsing/sending: `rathena-master/src/map/clif.cpp`
  - Skills: `rathena-master/src/map/skill.cpp`
  - Status/effects: `rathena-master/src/map/status.cpp`
  - Combat formulas: `rathena-master/src/map/battle.cpp`
  - NPC/script behavior: `rathena-master/src/map/npc.cpp` and `rathena-master/src/map/script.cpp`
  - Movement/unit logic: `rathena-master/src/map/unit.cpp`
- Config/content: Add data to `rathena-master/conf/`, `rathena-master/db/`, or `rathena-master/npc/` according to the subsystem.
- Compile-time customization hooks: Use `rathena-master/src/custom/*.inc` or `rathena-master/src/custom/*.hpp` when matching existing rAthena extension points.

**New Docker Runtime Behavior:**
- Build/runtime commands: Edit `docker/entrypoint.sh`.
- Image dependencies: Edit `docker/Dockerfile`.
- rAthena import overrides: Add or update files under `docker/import/`.
- Database seed logic: Add SQL under `docker/init-db/`; do not quote seeded credentials or secrets into planning docs.
- Compose service topology: Edit `docker-compose.yml` carefully and avoid exposing secret values in generated documentation.

**New Manual Test Fixture:**
- Standalone client fixture: Add under `korangar-test/`.
- Client archive fixture: Add under `korangar-test/archive/data/`.
- rAthena content fixture: Add under `rathena-master/npc/custom/`, `rathena-master/db/import/`, or `rathena-master/conf/import/` depending on the feature.

## Special Directories

**`korangar/target/`:**
- Purpose: Cargo build output.
- Generated: Yes
- Committed: No

**`korangar/korangar/archive/`:**
- Purpose: Extracted/generated client archive data used at runtime.
- Generated: Mixed
- Committed: Mixed; treat as runtime asset data and avoid broad rewrites.

**`korangar/korangar/data.grf` and `korangar/korangar/rdata.grf`:**
- Purpose: Official client GRF assets required by Korangar.
- Generated: No
- Committed: Local binary assets; avoid editing.

**`korangar/korangar/client/`:**
- Purpose: Korangar runtime settings and theme files.
- Generated: Mixed
- Committed: Yes for defaults; runtime files may change locally.

**`korangar/korangar/shaders/`:**
- Purpose: Source shaders compiled/used by the graphics engine.
- Generated: No
- Committed: Yes

**`rathena-master/log/`:**
- Purpose: rAthena runtime log output.
- Generated: Yes
- Committed: No

**`rathena-master/save/`:**
- Purpose: rAthena runtime save/pid files.
- Generated: Yes
- Committed: No

**`rathena-master/conf/import/`:**
- Purpose: Effective rAthena config overrides.
- Generated: Mixed; Docker copies from `docker/import/` at container startup.
- Committed: Usually yes for intentional overrides, but local Docker flow owns copied files.

**`rathena-master/db/import/`:**
- Purpose: rAthena DB override/import data.
- Generated: No
- Committed: Yes when adding intentional custom DB entries.

**`rathena-master/npc/custom/`:**
- Purpose: Custom NPC scripts.
- Generated: No
- Committed: Yes when adding project-specific scripts.

**`rathena-master/src/custom/`:**
- Purpose: rAthena custom C/C++ include extension points.
- Generated: No
- Committed: Yes

**`rathena-master/src/*/obj/`:**
- Purpose: rAthena object/build output directories.
- Generated: Yes
- Committed: No

**`rathena-master/login-server`, `rathena-master/char-server`, `rathena-master/map-server`, `rathena-master/web-server`:**
- Purpose: Built rAthena binaries.
- Generated: Yes
- Committed: Local build artifacts; avoid editing.

**`korangar-rathena-patches/`:**
- Purpose: Compatibility patch archive for rAthena changes.
- Generated: No
- Committed: Yes

**`warp/win32/`:**
- Purpose: Bundled WARP executable and Windows runtime dependencies.
- Generated: No
- Committed: Tool distribution artifacts.

**`korangar-test/`:**
- Purpose: Manual standalone Korangar runtime test fixture.
- Generated: Mixed
- Committed: Local test asset/binary directory.

---

*Structure analysis: 2026-04-26*
