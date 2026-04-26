# Codebase Concerns

**Analysis Date:** 2026-04-26

## Tech Debt

**Root repository contains nested source checkouts and backup trees:**
- Issue: The root git repository sees the main implementation directories as untracked nested projects, while `korangar/` and `rathena-master/` each have their own git state. `rathena-master.bak/` duplicates a full rAthena tree alongside the active `rathena-master/`.
- Files: `korangar/`, `rathena-master/`, `rathena-master.bak/`, `.git/`
- Impact: Changes can be made in the wrong repository, backup code can be mistaken for active code, searches and tooling traverse duplicate files, and root-level commits do not clearly capture the actual implementation state.
- Fix approach: Treat `korangar/` and `rathena-master/` as explicit submodules/subtrees or vendor snapshots, remove `rathena-master.bak/` from the working tree, and add a root `.gitignore` that excludes generated artifacts and local assets.

**rAthena customization is direct source mutation plus separate patch files:**
- Issue: Korangar compatibility changes are applied directly to `rathena-master/` and also stored as patch files in `korangar-rathena-patches/`, but there is no root script that verifies or reapplies the patch set.
- Files: `korangar-rathena-patches/*.patch`, `rathena-master/src/map/clif.cpp`, `rathena-master/src/config/packets.hpp`, `rathena-master/conf/battle/client.conf`, `rathena-master/conf/battle/drops.conf`, `rathena-master/conf/battle/exp.conf`, `rathena-master/conf/battle/player.conf`, `rathena-master/conf/char_athena.conf`, `rathena-master/conf/login_athena.conf`
- Impact: Re-cloning or updating rAthena can silently remove required compatibility changes such as packet obfuscation disablement, packet batching tolerance, larger area/walk limits, and debug economy rates.
- Fix approach: Add an idempotent patch-application script at the root, pin the source commit in a manifest, and run a verification command that checks critical lines before Docker build.

**Docker build mutates the rAthena source directory:**
- Issue: `docker/entrypoint.sh` builds in `/rathena`, which is bind-mounted from `./rathena-master`, and leaves generated files in the source tree.
- Files: `docker-compose.yml`, `docker/entrypoint.sh`, `rathena-master/login-server`, `rathena-master/char-server`, `rathena-master/map-server`, `rathena-master/Makefile`, `rathena-master/config.log`, `rathena-master/config.status`
- Impact: The source tree contains build outputs and configure artifacts, dirty nested git status becomes normal, and stale binaries can mask source changes because `build_servers()` skips compilation when binaries exist.
- Fix approach: Build into a Docker volume or out-of-tree build directory, keep generated artifacts ignored, and make rebuild behavior explicit in the developer workflow.

**Client entry point concentrates gameplay and network event behavior:**
- Issue: `korangar/korangar/src/main.rs` is a 3,398-line event hub containing login flow, map loading, entity lifecycle, combat, inventory, UI windows, and local workaround logic.
- Files: `korangar/korangar/src/main.rs`
- Impact: Small gameplay changes can affect unrelated systems, ordering bugs are hard to reason about, and targeted regression tests are difficult to write around one large event handler.
- Fix approach: Extract event handlers by domain, especially `NetworkEvent::ResurrectPlayer`, `NetworkEvent::RemoveEntity`, `NetworkEvent::ChangeMap`, inventory/sell windows, and login disconnect handling.

**Protocol parsing has explicit unimplemented paths:**
- Issue: Packet and debug packet types still contain `todo!()` and `unimplemented!()` branches.
- Files: `korangar/ragnarok-packets/src/lib.rs`, `korangar/korangar/src/networking/mod.rs`
- Impact: Receiving or inspecting a packet that reaches these branches can panic the client instead of producing a recoverable protocol error.
- Fix approach: Replace `todo!()` and `unimplemented!()` with typed `ConversionError` results or lossless unknown-packet storage, then add packet-level tests for the affected headers.

**Debug packet history clears data to avoid path lookup crashes:**
- Issue: `PacketHistory::update()` clears all retained entries once the base buffer is exceeded because partial draining breaks path lookups.
- Files: `korangar/korangar/src/networking/mod.rs`
- Impact: Packet inspection loses useful history during high traffic, which weakens debugging for exactly the network issues this project is investigating.
- Fix approach: Replace the `Vec` storage with a stable ring buffer or stable IDs so old entries can be evicted without invalidating interface paths.

**Generated and local runtime artifacts are present in implementation directories:**
- Issue: Large assets and binaries live under project folders, including GRF files, executable builds, logs, and third-party Windows runtime files.
- Files: `korangar/korangar/data.grf`, `korangar/korangar/rdata.grf`, `korangar/korangar/korangar.exe`, `korangar/korangar/logs-all.txt`, `korangar-test/korangar.exe`, `warp/win32/`
- Impact: Backups, indexing, diffs, and root-level status become noisy; accidental commits can include multi-gigabyte or licensed game assets.
- Fix approach: Keep official game assets outside the source tree or under an ignored local asset directory, add root-level ignore rules, and document expected asset paths in `README.md`.

## Known Bugs

**Player remains visually dead after respawn:**
- Symptoms: After death and respawn, the player can move with full HP but remains rendered in the dead animation; the respawn window can also remain visible.
- Files: `AI_CONTEXT.md`, `korangar/korangar/src/main.rs`, `korangar/korangar/src/world/entity/mod.rs`, `rathena-master/src/map/pc.cpp`
- Trigger: Enter the game, kill the current player, then respawn through the respawn button or server-side revive flow.
- Workaround: None confirmed. The current client attempts to call `set_idle()` and `stop_movement()` in `NetworkEvent::ResurrectPlayer` and `NetworkEvent::ChangeMap`, but the project notes the bug as unresolved.

**Respawn relies on map-change semantics instead of a guaranteed resurrection packet:**
- Symptoms: `pc_respawn()` sends `clif_resurrection()` only when `pc_setpos()` fails, so successful save-point respawn is represented as a position/map change.
- Files: `rathena-master/src/map/pc.cpp`, `korangar/korangar/src/main.rs`
- Trigger: Normal respawn through rAthena save-point teleport after player death.
- Workaround: Client-side `ChangeMap` handling closes `WindowClass::Respawn` and resets the first retained entity to idle, but this assumes the retained first entity is always the player.

**Unknown login-server disconnect root cause remains unresolved:**
- Symptoms: The login socket can close after login success; automatic reconnect invalidates the login auth node, causing character-server authentication refusal.
- Files: `AI_CONTEXT.md`, `korangar/korangar-networking/src/lib.rs`, `korangar/korangar/src/main.rs`, `rathena-master/src/login/loginclif.cpp`
- Trigger: Login flow that reaches server selection after the login socket has dropped.
- Workaround: The client currently shows a login error instead of auto-reconnecting, so users must log in again manually.

## Security Considerations

**Dev stack exposes database and game ports on the host:**
- Risk: MariaDB, login, char, and map ports are published directly from Docker, and the stack uses hard-coded development database credentials.
- Files: `docker-compose.yml`, `docker/import/inter_conf.txt`, `README.md`, `AI_CONTEXT.md`
- Current mitigation: The advertised game IPs in `docker/import/char_conf.txt` and `docker/import/map_conf.txt` point at loopback for the local client workflow.
- Recommendations: Bind development ports to `127.0.0.1`, move credentials to ignored environment files, and keep production-like deployment settings separate from this local lab stack.

**Authentication is intentionally weakened for local testing:**
- Risk: Account creation is enabled, pin code is disabled, minimum credential length is reduced, and password hashing is disabled for the local rAthena setup.
- Files: `docker/import/login_conf.txt`, `docker/import/char_conf.txt`, `rathena-master/conf/login_athena.conf`, `rathena-master/conf/char_athena.conf`
- Current mitigation: The configuration is documented as a local Docker development stack.
- Recommendations: Label these settings as dev-only in a machine-readable config profile, and add a startup guard that refuses to run this profile when binding to non-loopback interfaces.

**GM/admin capabilities are broadly available in the debug server:**
- Risk: The at-command patch bypasses normal command group checks, and a seeded full-GM account exists for local development.
- Files: `korangar-rathena-patches/at-command.patch`, `rathena-master/src/map/atcommand.cpp`, `docker/init-db/zz-admin.sql`, `GM_COMMANDS.md`
- Current mitigation: The setup is documented as a debug and learning environment.
- Recommendations: Keep GM command bypasses out of any shared server profile, and make the patch application conditional on an explicit `DEV_CHEATS=1`-style flag.

**Packet obfuscation is disabled for client compatibility:**
- Risk: rAthena packet obfuscation is disabled so Korangar can connect, which removes one compatibility/security layer expected by official clients.
- Files: `rathena-master/src/config/packets.hpp`, `korangar-rathena-patches/packet-obfuscation.patch`, `docker-compose.yml`
- Current mitigation: The setting is required for the current open-source client compatibility path.
- Recommendations: Treat the server as local-only until the client implements the required packet obfuscation or a safer compatibility mode is designed.

**Unsafe Rust is present in UI state and AV1 video bindings:**
- Risk: `UnsafeCell`, unchecked downcasts, raw pointer access, and manual `Send`/`Sync` impls can introduce undefined behavior if UI lifetimes or decoder threading assumptions change.
- Files: `korangar/korangar/src/interface/windows/skill_tree/state.rs`, `korangar/korangar-interface/src/element/store.rs`, `korangar/korangar-video/src/lib.rs`
- Current mitigation: Some unsafe blocks include local safety comments, and `korangar-video/src/lib.rs` explains that the unsafe code wraps the `rav1d` C API.
- Recommendations: Keep unsafe code isolated, add focused tests around persistent UI state reuse, and review `unsafe impl Send/Sync` whenever decoder ownership changes.

## Performance Bottlenecks

**Docker build is serial and source-mounted:**
- Problem: `docker/entrypoint.sh` uses serial `make clean` and `make server`, and the build runs against a bind mount from the Windows host.
- Files: `docker/entrypoint.sh`, `docker-compose.yml`, `rathena-master/Makefile`
- Cause: The script avoids rAthena dependency races but pays full clean-build cost, and Windows bind mounts add filesystem overhead.
- Improvement path: Cache dependencies and build outputs in Docker volumes, separate clean rebuild from normal incremental build, and keep source edits independent from generated binaries.

**Large active source files dominate rAthena modification risk:**
- Problem: Key rAthena behavior lives in very large C++ files.
- Files: `rathena-master/src/map/script.cpp`, `rathena-master/src/map/skill.cpp`, `rathena-master/src/map/clif.cpp`, `rathena-master/src/map/status.cpp`, `rathena-master/src/map/pc.cpp`, `rathena-master/src/map/battle.cpp`
- Cause: rAthena is a mature monolithic C++ server with broad map-server modules.
- Improvement path: Keep local changes as small patches, add comments near compatibility deltas, and prefer config/import overrides over editing large upstream files.

**Client asset and media loading contains many panic-style assumptions:**
- Problem: Rendering, loaders, and media code use `unwrap()`, `expect()`, and `panic!()` across graphics, archive, font, texture, video, and audio paths.
- Files: `korangar/korangar/src/graphics/engine.rs`, `korangar/korangar/src/loaders/archive/native/mod.rs`, `korangar/korangar/src/loaders/texture/mod.rs`, `korangar/korangar/src/loaders/font/mod.rs`, `korangar/korangar-video/src/lib.rs`, `korangar/korangar-audio/src/lib.rs`
- Cause: The client assumes valid local assets, GPU capabilities, shader compiler availability, and supported media formats.
- Improvement path: Convert user-controlled asset and device failures into recoverable errors with visible diagnostics, especially around GRF loading, shader compilation, and audio/video initialization.

**Packet cutoff handling drops oversized or misparsed data:**
- Problem: When packet parsing reports a cutoff at offset zero, the networking loop resets `cut_off_buffer_base` and drops the data instead of reporting a structured packet error.
- Files: `korangar/korangar-networking/src/lib.rs`
- Cause: The loop limits packet size to the read buffer and avoids getting stuck on incorrect packet lengths.
- Improvement path: Emit packet callback/error events for dropped cutoffs, include the raw header and buffer length, and add regression tests for batched and partial packet streams.

## Fragile Areas

**Death, resurrection, and map-change state transitions:**
- Files: `korangar/korangar/src/main.rs`, `korangar/korangar/src/world/entity/mod.rs`, `rathena-master/src/map/pc.cpp`
- Why fragile: Client death state is controlled by multiple packet handlers, while respawn can arrive as resurrection or as map teleport. `ChangeMap` currently truncates entities to one and resets `entities().first_mut()`.
- Safe modification: Route all player death-state changes through a dedicated player lifecycle helper keyed by `this_entity()`, and verify both resurrection packet and save-point map-change flows.
- Test coverage: No automated integration test covers the death-to-respawn visual state across Korangar and rAthena.

**Login, reconnect, and auth-node lifecycle:**
- Files: `korangar/korangar-networking/src/lib.rs`, `korangar/korangar/src/main.rs`, `rathena-master/src/login/loginclif.cpp`, `rathena-master/src/login/loginchrif.cpp`, `rathena-master/src/char/char_clif.cpp`
- Why fragile: Network disconnection events, keepalive timing, and rAthena auth-node cleanup interact across three server processes and the client UI state.
- Safe modification: Add structured logs for login success, disconnect reason, reconnect attempts, and char-server auth requests before changing reconnect behavior.
- Test coverage: Existing Rust unit tests cover lower-level utilities, but there is no automated login/char/map handshake test against the Docker stack.

**Packet version and protocol compatibility:**
- Files: `docker-compose.yml`, `docker/entrypoint.sh`, `rathena-master/src/config/packets.hpp`, `korangar/ragnarok-packets/src/lib.rs`, `korangar/korangar-networking/src/lib.rs`
- Why fragile: Docker passes packet version through configure, source defaults contain a different fallback packet version, and unsupported packets can still panic.
- Safe modification: Keep packet version in one manifest, generate both server build args and client constants from it, and require packet-parser tests for every new header used by the client.
- Test coverage: Packet serialization has unit coverage in some crates, but server-client compatibility is not exercised end to end.

**Local Docker import configs overwrite runtime server configuration:**
- Files: `docker/entrypoint.sh`, `docker/import/login_conf.txt`, `docker/import/char_conf.txt`, `docker/import/inter_conf.txt`, `docker/import/map_conf.txt`, `rathena-master/conf/import/`
- Why fragile: The entrypoint copies import files on every container start, so manual edits under `rathena-master/conf/import/` can disappear.
- Safe modification: Treat `docker/import/*.txt` as the only editable source for Docker overrides, and make the entrypoint log copied file names.
- Test coverage: No script verifies that the effective imported configuration matches the expected Docker profile.

**Nested upstream repositories have separate dirty states:**
- Files: `korangar/`, `rathena-master/`, `README.md`, `AI_CONTEXT.md`
- Why fragile: `korangar/` has modified client files and untracked executable/log artifacts; `rathena-master/` has modified config/source files and generated binaries. The root git status only reports large untracked directories.
- Safe modification: Always check `git -C korangar status --short` and `git -C rathena-master status --short` before editing or committing source changes.
- Test coverage: Not applicable.

## Scaling Limits

**Local-only Docker topology:**
- Current capacity: One local MariaDB container plus one login, char, and map container bound for a single developer machine.
- Limit: Published host ports, loopback advertised IPs, hard-coded service names, and dev credentials are not suitable for multi-user or remote-host deployment.
- Scaling path: Split dev and server profiles, externalize secrets, bind public-facing services intentionally, and document network topology separately from local play instructions.

**Korangar/rAthena compatibility depends on a single pinned protocol profile:**
- Current capacity: rAthena built with packet version `20220406` for Korangar compatibility in the Docker path.
- Limit: Supporting another client packet version requires coordinated changes across rAthena build flags, packet obfuscation, packet definitions, and client parsing.
- Scaling path: Introduce a protocol compatibility matrix and automated smoke tests for login, character select, map enter, movement, death, respawn, inventory, and NPC dialog.

**Repository size and search cost increase with local assets and backups:**
- Current capacity: Working tree includes multi-gigabyte GRF files, compiled executables, a rAthena backup tree, and Windows runtime assets.
- Limit: Code search, backups, sync, and review become slower and noisier as more local artifacts accumulate.
- Scaling path: Store game assets and generated binaries outside tracked source directories, remove duplicate source snapshots, and keep a small reproducible checkout.

## Dependencies at Risk

**Pinned third-party Rust git dependencies:**
- Risk: The Rust workspace depends on external git sources, including `rav1d` and `rust-state`, outside the crates.io versioning flow.
- Impact: Builds depend on repository availability and pinned commit compatibility with the nightly toolchain and platform dependencies.
- Migration plan: Mirror or vendor critical git dependencies, track upstream releases, and document the tested commit set in `korangar/Cargo.lock`.

**Nightly Rust and native toolchain requirements:**
- Risk: Korangar uses `rust-toolchain.toml`, Vulkan/Slang shader tooling, NASM, audio libraries, and platform-specific graphics/audio stacks.
- Impact: New developer setup and CI can fail due to toolchain drift or missing native packages.
- Migration plan: Keep Nix flake and CI dependency lists authoritative, add a Windows setup verification script, and pin Slang/NASM expectations in project docs.

**Downloaded build helper in Docker image:**
- Risk: `docker/Dockerfile` downloads `wait-for` from GitHub during image build.
- Impact: Docker builds can fail or change behavior if the remote URL is unavailable or content changes.
- Migration plan: Pin by checksum or vendor the helper script under `docker/`.

## Missing Critical Features

**No automated end-to-end smoke test for the playable loop:**
- Problem: The critical workflow spans Docker rAthena, MariaDB seed data, Korangar login, character selection, map entry, movement, combat/death, and respawn.
- Blocks: Safe refactoring of networking, packet parsing, death/respawn handling, and server patches.

**No machine-readable local/server configuration profiles:**
- Problem: Dev-only settings are spread across `docker-compose.yml`, `docker/import/*.txt`, rAthena source/config patches, and prose docs.
- Blocks: Clean separation between local reverse-engineering setup and any future shared server configuration.

**No reproducible patch verification for rAthena compatibility:**
- Problem: Required rAthena changes exist as direct edits and patch files, but no command proves that the active source tree contains the exact expected compatibility deltas.
- Blocks: Safe rAthena updates, reclones, and collaboration across machines.

## Test Coverage Gaps

**Korangar/rAthena handshake and map-entry flow:**
- What's not tested: Login server success, char-server authentication, character selection, map-server connection, and batched map connection packets.
- Files: `korangar/korangar-networking/src/lib.rs`, `korangar/korangar/src/main.rs`, `rathena-master/src/map/clif.cpp`, `rathena-master/src/login/loginclif.cpp`, `rathena-master/src/char/char_clif.cpp`
- Risk: Regressions can reintroduce auth refusal or map-entry stalls without failing unit tests.
- Priority: High

**Death and respawn visual state:**
- What's not tested: Transition from alive to dead to respawned player across `RemoveEntity`, `ResurrectPlayer`, and `ChangeMap`.
- Files: `korangar/korangar/src/main.rs`, `korangar/korangar/src/world/entity/mod.rs`, `rathena-master/src/map/pc.cpp`
- Risk: The known dead-pose bug can persist or regress while HP and movement appear functional.
- Priority: High

**Protocol parser completeness for gameplay packets:**
- What's not tested: Unsupported packet paths that reach `todo!()` or `unimplemented!()`, unknown packet inspection, packet cutoff behavior, and partial/batched TCP reads.
- Files: `korangar/ragnarok-packets/src/lib.rs`, `korangar/korangar/src/networking/mod.rs`, `korangar/korangar-networking/src/lib.rs`
- Risk: Server behavior changes can crash the client or silently drop packet data.
- Priority: High

**Docker profile and rAthena patch application:**
- What's not tested: Effective config after `docker/entrypoint.sh` copies imports, packet version used by compiled binaries, and presence of all Korangar compatibility patches.
- Files: `docker-compose.yml`, `docker/entrypoint.sh`, `docker/import/*.txt`, `korangar-rathena-patches/*.patch`, `rathena-master/src/config/packets.hpp`
- Risk: Local servers can start with stale binaries or mismatched config while appearing healthy.
- Priority: Medium

**Asset failure paths in client loaders:**
- What's not tested: Missing/corrupt GRF files, missing shaders, unsupported images, missing fonts, and absent audio/video dependencies.
- Files: `korangar/korangar/src/loaders/archive/native/mod.rs`, `korangar/korangar/src/loaders/texture/mod.rs`, `korangar/korangar/src/loaders/font/mod.rs`, `korangar/korangar/src/graphics/shader_compiler.rs`, `korangar/korangar-audio/src/lib.rs`, `korangar/korangar-video/src/lib.rs`
- Risk: New users hit panics or unclear failures instead of actionable diagnostics.
- Priority: Medium

---

*Concerns audit: 2026-04-26*
