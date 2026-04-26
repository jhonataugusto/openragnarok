# Coding Conventions

**Analysis Date:** 2026-04-26

## Naming Patterns

**Files:**
- Use Rust snake_case module filenames under `korangar/**/src`, with directory modules exposed through `mod.rs`, for example `korangar/korangar/src/graphics/picker_target.rs`, `korangar/korangar/src/world/pathing.rs`, and `korangar/korangar/src/interface/windows/mod.rs`.
- Use Rust crate directories that match Cargo package names, for example `korangar/korangar-audio`, `korangar/ragnarok-bytes`, and `korangar/ragnarok-packets`.
- Use rAthena C++ source/header pairs with matching snake_case or subsystem prefixes, for example `rathena-master/src/char/char.cpp` with `rathena-master/src/char/char.hpp`, `rathena-master/src/login/loginclif.cpp` with `rathena-master/src/login/loginclif.hpp`, and `rathena-master/src/map/skills/acolyte/heal.cpp` with `rathena-master/src/map/skills/acolyte/heal.hpp`.
- Treat `rathena-master.bak/` as a backup copy, not the convention source. Use active code under `rathena-master/`.

**Functions:**
- Use Rust `snake_case` for functions and methods, for example `init_tls_rand()` in `korangar/korangar/src/main.rs`, `find_walkable_path()` in `korangar/korangar/src/world/pathing.rs`, and `from_inputs()` in `korangar/korangar-interface/src/element/store.rs`.
- Use rAthena C++ mixed subsystem style by area: subsystem free functions use lower snake_case such as `status_get_sc()` and `skill_calc_heal()` in `rathena-master/src/map/skills/acolyte/heal.cpp`; class methods use lower camelCase such as `castendNoDamageId()` in `rathena-master/src/map/skills/acolyte/heal.cpp`; database helper methods use lower camelCase such as `verifyCompatibility()` and `parseBodyNode()` in `rathena-master/src/common/database.hpp`.
- New rAthena skill classes should follow the `Skill<Name>` class pattern, for example `SkillHeal` in `rathena-master/src/map/skills/acolyte/heal.hpp`.

**Variables:**
- Use Rust `snake_case` locals and fields, for example `open_set`, `closed_set`, `came_from`, and `g_scores` in `korangar/korangar/src/world/pathing.rs`.
- Use Rust uppercase constants for fixed values, for example `CLIENT_NAME`, `DEFAULT_MAP`, `MAX_WALK_PATH_SIZE`, and `MOVE_DIAGONAL_COST` in `korangar/korangar/src/main.rs` and `korangar/korangar/src/world/pathing.rs`.
- Use rAthena fixed-width typedefs from `rathena-master/src/common/cbasetypes.hpp` (`int32`, `uint16`, `uint64`) instead of plain platform-dependent integer types in first-party server code.
- Use rAthena global/subsystem variables in lower snake_case, for example `login_fd`, `char_fd`, `msg_table`, and `charserv_config` in `rathena-master/src/char/char.cpp`.

**Types:**
- Use Rust `UpperCamelCase` for structs, enums, traits, and type aliases, for example `PathFinder`, `Traversable`, `ElementStore`, `PersistentDataProvider`, and `ConversionError` in `korangar/korangar/src/world/pathing.rs`, `korangar/korangar-interface/src/element/store.rs`, and `korangar/ragnarok-bytes/src/lib.rs`.
- Use Rust trait extension names ending in `Ext` when adding convenience behavior, for example `FromBytesExt`, `ToBytesExt`, `ConversionResultExt`, and `ClientStatePathExt` in `korangar/ragnarok-bytes/src/lib.rs` and `korangar/korangar/src/main.rs`.
- Use rAthena C++ class names in `UpperCamelCase`, for example `YamlDatabase`, `TypesafeYamlDatabase`, and `SkillHeal` in `rathena-master/src/common/database.hpp` and `rathena-master/src/map/skills/acolyte/heal.hpp`.
- Use rAthena enum and macro names in upper snake case for constants and preprocessor values, for example `MSG_FATALERROR`, `CL_RED`, and `NULLPO_CHECK` in `rathena-master/src/common/showmsg.hpp` and `rathena-master/src/common/nullpo.hpp`.

## Code Style

**Formatting:**
- Rust is formatted with Rustfmt using `korangar/rustfmt.toml`. Run `cargo fmt --all` from `korangar/` before committing Rust changes.
- Rust formatting settings are non-default and must be respected: edition/style edition `2024`, `max_width = 140`, `group_imports = "StdExternalCrate"`, `imports_granularity = "Module"`, `use_field_init_shorthand = true`, `reorder_impl_items = true`, and wrapped comments.
- Rust CI enforces formatting with `cargo fmt --all --check` in `korangar/.github/workflows/formatting.yml`.
- Nix formatting for `korangar/flake.nix` is checked with `nix fmt flake.nix -- --check` in `korangar/.github/workflows/formatting.yml`.
- rAthena CMake uses spaced command arguments in `rathena-master/CMakeLists.txt`, for example `set( CMAKE_LEGACY_CYGWIN_WIN32 0 )` and `option( ALLOW_SAME_DIRECTORY ... OFF )`.
- rAthena C++ uses tabs for indentation in many active files, compact control-flow spacing such as `if( condition ){`, and brace placement that varies by subsystem. Match the surrounding file rather than applying a new formatter globally.

**Linting:**
- Rust Clippy is enforced in `korangar/.github/workflows/lint.yml` with both `cargo clippy -- -Dwarnings` and `cargo clippy --all-features -- -Dwarnings`.
- Rust crate roots may carry targeted lint allowances when required by current architecture, for example `#![allow(incomplete_features)]` and `#![allow(clippy::too_many_arguments)]` in `korangar/korangar/src/main.rs`. Add new allowances only at the narrowest practical scope.
- The Rust toolchain is pinned in `korangar/rust-toolchain.toml` to `nightly-2026-02-01` with `rust-src`, `rust-analyzer`, and `miri`; code can use nightly features already declared in crate roots.
- rAthena GCC CI treats warnings as errors through `CXXFLAGS='-Werror -Wno-error=builtin-declaration-mismatch'` in `rathena-master/.github/workflows/build_servers_gcc.yml`.
- rAthena is also compiled through CMake and MSBuild workflows under `rathena-master/.github/workflows/`, so C++ changes must stay portable across GCC, Clang, MSVC, Make, and CMake.

## Import Organization

**Order:**
1. Rust standard library imports first, for example `std::cmp::Ordering` and `std::collections::BinaryHeap` in `korangar/korangar/src/world/pathing.rs`.
2. Rust external crate imports next, for example `hashbrown::{HashMap, HashSet}` and `ragnarok_packets::{AttackRange, TilePosition}` in `korangar/korangar/src/world/pathing.rs`.
3. Rust local module imports last, for example `use crate::graphics::*;`, `use crate::input::{InputEvent, InputSystem};`, and `use crate::world::*;` in `korangar/korangar/src/main.rs`.
4. rAthena C++ includes local headers first in several implementation files, for example `#include "heal.hpp"` in `rathena-master/src/map/skills/acolyte/heal.cpp`, followed by related relative project headers.
5. rAthena shared headers use system/third-party headers before project headers, for example `<unordered_map>`, `<vector>`, `<ryml_std.hpp>`, `<ryml.hpp>`, then `<config/core.hpp>` and local common headers in `rathena-master/src/common/database.hpp`.

**Path Aliases:**
- Rust uses crate names and workspace dependencies from `korangar/Cargo.toml`; import internal crates by package name such as `korangar_audio`, `korangar_interface`, `ragnarok_bytes`, and `ragnarok_packets`.
- Rust uses `crate::` for intra-crate imports and `super::` for sibling/private module imports, for example `use crate::world::*;` in `korangar/korangar/src/main.rs` and `use super::id::{ElementId, ElementIdGenerator};` in `korangar/korangar-interface/src/element/store.rs`.
- rAthena C++ uses include paths rooted at `rathena-master/src`, for example `<common/cbasetypes.hpp>` and `<config/core.hpp>` in `rathena-master/src/common/showmsg.hpp` and `rathena-master/src/common/database.hpp`.

## Error Handling

**Patterns:**
- Rust fallible domain APIs return `Option` or crate-specific `Result` aliases when the caller can recover. Examples include `Option<&[TilePosition]>` in `korangar/korangar/src/world/pathing.rs` and `ConversionResult` exports in `korangar/ragnarok-bytes/src/lib.rs`.
- Rust uses `unwrap()` and `expect()` in tests, build scripts, and invariant-heavy code paths. Examples are `T::from_bytes(...).unwrap()` in `korangar/ragnarok-bytes/src/lib.rs`, build-time `expect(...)` calls in `korangar/korangar/build.rs`, and `expect("Tried to get invalid child store")` in `korangar/korangar-interface/src/element/store.rs`.
- Rust runtime graphics errors are routed through a dedicated handler in `korangar/korangar/src/graphics/error.rs`, which logs with `print_debug!` and panics under `debug_assertions`.
- rAthena C++ reports operational errors through `ShowError`, `ShowWarning`, `ShowInfo`, and related console functions declared in `rathena-master/src/common/showmsg.hpp`.
- rAthena C++ uses boolean or integer status returns for recoverable failures, for example `return false` after validation failures in `rathena-master/src/char/char.cpp`.
- rAthena C++ uses null-pointer guard macros from `rathena-master/src/common/nullpo.hpp` (`nullpo_ret`, `nullpo_retv`, `nullpo_retr`, `nullpo_retb`) in code paths where debug null checks are desired.

## Logging

**Framework:** Rust custom debug logging and rAthena console logging

**Patterns:**
- Use `korangar_debug::logging::{print_debug, Colorize}` behind `#[cfg(feature = "debug")]` for Korangar debug output, as shown in `korangar/korangar/src/main.rs` and `korangar/korangar/src/graphics/error.rs`.
- Use `korangar_debug::profile_block` and profiler thread macros behind debug feature gates for timing and profiling in `korangar/korangar/src/main.rs`.
- Use `println!("cargo:...")` only for Cargo build-script directives and warnings in `korangar/korangar/build.rs`.
- Use `eprintln!` in command/build validation paths that report missing tools or invalid versions, as in `korangar/korangar/build.rs`.
- Use rAthena `ShowInfo`, `ShowWarning`, `ShowError`, `ShowDebug`, and `ShowFatalError` rather than direct `printf` for server diagnostics; declarations live in `rathena-master/src/common/showmsg.hpp` and call sites are widespread in `rathena-master/src/char/char.cpp`.

## Comments

**When to Comment:**
- Use Rust `///` doc comments for public traits, structs, constants, and methods where behavior matters, for example `Traversable`, `MAX_WALK_PATH_SIZE`, and `PathFinder` methods in `korangar/korangar/src/world/pathing.rs`.
- Use Rust inline comments to explain non-obvious algorithmic or safety constraints, for example diagonal neighbor checks in `korangar/korangar/src/world/pathing.rs` and `UnsafeCell` storage reasoning in `korangar/korangar-interface/src/element/store.rs`.
- Use rAthena C++ file headers consistently: active source and header files start with GPL copyright comments, for example `rathena-master/src/map/skills/acolyte/heal.cpp` and `rathena-master/src/common/showmsg.hpp`.
- Use rAthena Doxygen-style block comments for API-like helpers and macros, for example null pointer macro documentation in `rathena-master/src/common/nullpo.hpp`.
- Keep TODO/FIXME comments specific and local to the behavior being deferred, for example the configurable point-light TODO in `korangar/korangar/src/main.rs` and FIXME comments in `rathena-master/src/char/char.cpp`.

**JSDoc/TSDoc:**
- Not applicable. No first-party JavaScript or TypeScript application source was detected.

## Function Design

**Size:** Rust code favors small focused helpers for reusable logic, but application orchestration files such as `korangar/korangar/src/main.rs` contain large event and client flow functions. Keep new pure logic in focused modules like `korangar/korangar/src/world/pathing.rs` and avoid expanding monolithic orchestration unless the surrounding pattern requires it.

**Parameters:** Rust uses references and trait bounds for reusable logic, for example `map: &impl Traversable` in `korangar/korangar/src/world/pathing.rs` and generic trait constraints in `korangar/korangar-interface/src/element/store.rs`. rAthena C++ uses pointer-heavy APIs and reference output parameters in server code, for example `block_list *src`, `block_list *bl`, and `int32& flag` in `rathena-master/src/map/skills/acolyte/heal.cpp`.

**Return Values:** Rust returns domain types, `Option`, and `Result` aliases rather than integer error codes. rAthena C++ returns `bool`, pointers, `std::shared_ptr`, or subsystem integer codes depending on the existing API, for example `bool load()` in `rathena-master/src/common/database.hpp` and integer character creation outcomes in `rathena-master/src/char/char.cpp`.

## Module Design

**Exports:** Rust crate roots re-export public APIs explicitly, for example `korangar/ragnarok-bytes/src/lib.rs` exports `ByteReader`, `ByteWriter`, `FromBytes`, `ToBytes`, and conversion error types. Keep exports intentional and avoid wildcard public re-exports except where an existing crate facade already uses them.

**Barrel Files:** Rust uses `mod.rs` files as module barrels under directories such as `korangar/korangar/src/graphics/passes/mod.rs`, `korangar/korangar/src/interface/windows/mod.rs`, and `korangar/korangar/src/world/mod.rs`. rAthena C++ does not use barrel files; headers are included directly by feature/subsystem.

---

*Convention analysis: 2026-04-26*
