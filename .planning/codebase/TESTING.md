# Testing Patterns

**Analysis Date:** 2026-04-26

## Test Framework

**Runner:**
- Rust built-in test harness via Cargo.
- Config: `korangar/Cargo.toml`, `korangar/rust-toolchain.toml`, and `korangar/.github/workflows/tests.yml`.
- rAthena does not expose a C++ unit-test framework in `rathena-master/src`; validation is performed through build matrices and NPC/database smoke checks in GitHub Actions.
- rAthena NPC/script validation is configured in `rathena-master/.github/workflows/npc_db_validation.yml`, `rathena-master/npc/scripts_test.conf`, and `rathena-master/npc/test/**`.

**Assertion Library:**
- Rust standard assertions: `assert_eq!`, `assert!`, `unwrap()`, and `is_none()` in inline unit tests such as `korangar/korangar/src/world/pathing.rs`, `korangar/korangar/src/loaders/font/color_span_iterator.rs`, and `korangar/ragnarok-bytes/src/lib.rs`.
- rAthena NPC tests use script-level assertion helpers `AssertTrue` and `AssertEquals` in `rathena-master/npc/test/ci/0000_funcs.txt`, plus `errormes` for failure reporting.

**Run Commands:**
```bash
cd korangar && cargo test --all-features              # Run all Korangar Rust tests with all feature-gated code enabled
cd korangar && cargo test                             # Run default-feature Korangar Rust tests
cd korangar && cargo test -p korangar-collision       # Run one workspace package's Rust tests
cd korangar && cargo fmt --all --check                # Verify Rust formatting
cd korangar && cargo clippy -- -Dwarnings             # Lint default-feature Rust build
cd korangar && cargo clippy --all-features -- -Dwarnings # Lint all-feature Rust build
cd rathena-master && ./configure --enable-buildbot=yes && make clean && make all # rAthena GCC-style build validation
cd rathena-master && mkdir cbuild && cd cbuild && cmake -G "Unix Makefiles" .. && make # rAthena CMake build validation
cd rathena-master && ./map-server --run-once          # rAthena NPC/database startup validation after CI setup
```

## Test File Organization

**Location:**
- Rust tests are co-located with implementation files inside `#[cfg(test)]` modules. Detected examples include `korangar/korangar/src/world/pathing.rs`, `korangar/korangar/src/loaders/font/color_span_iterator.rs`, `korangar/korangar/src/graphics/picker_target.rs`, `korangar/korangar-audio/src/lib.rs`, `korangar/korangar-collision/src/aabb.rs`, and `korangar/ragnarok-bytes/src/lib.rs`.
- Rust workspace contains many inline unit tests, with 217 `#[test]` functions and 42 `#[cfg(test)]` blocks detected under `korangar/` excluding `target/`.
- rAthena regression scripts are kept under `rathena-master/npc/test/` and enabled through `rathena-master/npc/scripts_test.conf`.
- rAthena sample test-like scripts live under `rathena-master/doc/sample/`, for example `rathena-master/doc/sample/npc_shop_test.txt`, but these are documentation samples rather than automated unit tests.

**Naming:**
- Rust test functions use `test_*` names, for example `test_straight_path`, `test_diagonal_path`, and `test_no_path_possible` in `korangar/korangar/src/world/pathing.rs`.
- Rust helper structs in tests use descriptive `UpperCamelCase`, for example `TestMap` in `korangar/korangar/src/world/pathing.rs`.
- rAthena NPC issue regression files may be named by issue number, for example `rathena-master/npc/test/ci/5573.txt`, `rathena-master/npc/test/ci/7291.txt`, and `rathena-master/npc/test/ci/8886.txt`.

**Structure:**
```text
korangar/<crate>/src/<module>.rs
  implementation
  #[cfg(test)]
  mod tests { ... }

rathena-master/npc/scripts_test.conf
  npc: npc/test/<script>.txt

rathena-master/npc/test/
  *.txt
  ci/*.txt
```

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    struct TestMap {
        width: u16,
        height: u16,
        not_walkable: HashSet<TilePosition>,
        not_snipable: HashSet<TilePosition>,
    }

    #[test]
    fn test_no_path_possible() {
        let mut map = TestMap::new(5, 5);
        map.set_unwalkable(&[TilePosition { x: 1, y: 0 }]);

        let mut pathfinder = PathFinder::default();
        let start = TilePosition { x: 0, y: 2 };
        let goal = TilePosition { x: 2, y: 2 };

        assert!(pathfinder.find_walkable_path(&map, start, goal).is_none());
    }
}
```

**Patterns:**
- Put `#[cfg(test)] mod tests` at the bottom of the Rust file after implementation, as in `korangar/korangar/src/world/pathing.rs` and `korangar/korangar/src/loaders/font/color_span_iterator.rs`.
- Import implementation items with `use super::*;` inside Rust test modules.
- Build small local fixtures inside test modules instead of global test fixture directories, for example `TestMap` in `korangar/korangar/src/world/pathing.rs`.
- Test deterministic pure logic through explicit input/output assertions, for example color span parsing in `korangar/korangar/src/loaders/font/color_span_iterator.rs` and byte encode/decode round trips in `korangar/ragnarok-bytes/src/lib.rs`.
- rAthena NPC tests use script files with `OnInit` blocks and failure messages via `errormes`, for example `rathena-master/npc/test/ci/5573.txt`.

## Mocking

**Framework:** No dedicated mocking framework detected.

**Patterns:**
```rust
struct TestMap {
    width: u16,
    height: u16,
    not_walkable: HashSet<TilePosition>,
    not_snipable: HashSet<TilePosition>,
}

impl Traversable for TestMap {
    fn is_walkable(&self, position: TilePosition) -> bool {
        position.x < self.width && position.y < self.height && !self.not_walkable.contains(&position)
    }

    fn is_snipeable(&self, position: TilePosition) -> bool {
        position.x < self.width && position.y < self.height && !self.not_snipable.contains(&position)
    }
}
```

**What to Mock:**
- Mock trait boundaries with simple in-memory test structs when testing Rust algorithms, as shown by `TestMap` implementing `Traversable` in `korangar/korangar/src/world/pathing.rs`.
- Use literal byte arrays and in-memory readers/writers for serialization tests, as shown by `encode_decode<T>()` in `korangar/ragnarok-bytes/src/lib.rs`.
- Use NPC script assertion helpers from `rathena-master/npc/test/ci/0000_funcs.txt` for rAthena script behavior checks.

**What NOT to Mock:**
- Do not introduce a mocking crate for simple Rust trait tests; the current convention is local fixtures and standard assertions.
- Do not mock rAthena startup validation in CI paths that are intended to exercise actual database import, NPC enabling, `make map`, and `./map-server --run-once` from `rathena-master/.github/workflows/npc_db_validation.yml`.

## Fixtures and Factories

**Test Data:**
```rust
fn encode_decode<T: FromBytes + ToBytes>(input: &[u8]) {
    let mut byte_reader = ByteReader::without_metadata(input);
    let data = T::from_bytes(&mut byte_reader).unwrap();

    let mut byte_writer = ByteWriter::new();
    data.to_bytes(&mut byte_writer).unwrap();
    let bytes = byte_writer.into_inner();

    assert_eq!(input, bytes.as_slice());
}
```

**Location:**
- Rust fixtures live inline in the test module that uses them, for example `TestMap` in `korangar/korangar/src/world/pathing.rs` and `encode_decode<T>()` in `korangar/ragnarok-bytes/src/lib.rs`.
- rAthena NPC regression fixture scripts live under `rathena-master/npc/test/` and `rathena-master/npc/test/ci/`.
- rAthena test script registration lives in `rathena-master/npc/scripts_test.conf`.

## Coverage

**Requirements:** No coverage threshold or coverage tool configuration detected.

**View Coverage:**
```bash
# Not configured. Add cargo-tarpaulin, llvm-cov, gcov, or another coverage tool before relying on coverage reports.
```

## Test Types

**Unit Tests:**
- Rust unit tests are active and co-located. Use them for pure logic, parsers, math helpers, serialization, collision, pathfinding, timers, and deterministic state transformations.
- Examples: `korangar/korangar/src/world/pathing.rs`, `korangar/korangar/src/loaders/font/color_span_iterator.rs`, `korangar/korangar-collision/src/aabb.rs`, `korangar/korangar-audio/src/lib.rs`, and `korangar/ragnarok-bytes/src/lib.rs`.

**Integration Tests:**
- Rust `tests/` integration test directories were not detected under `korangar/`; use package-level `tests/` only when behavior crosses public crate boundaries and cannot be tested cleanly with inline unit tests.
- rAthena integration validation is CI-driven: build the server, enable NPC/test scripts, import database tables, compile the map server, and start `./map-server --run-once` as defined in `rathena-master/.github/workflows/npc_db_validation.yml`.

**E2E Tests:**
- Browser or UI E2E tests are not detected.
- Runtime smoke testing for rAthena uses `./map-server --run-once`.
- Korangar has no detected automated client E2E harness; CI runs `cargo test --all-features` in `korangar/.github/workflows/tests.yml`.

## Common Patterns

**Async Testing:**
```rust
// Not detected in current Rust tests. Async dependencies exist in korangar/korangar-networking/Cargo.toml,
// but observed tests use synchronous #[test] functions and deterministic in-memory fixtures.
```

**Error Testing:**
```rust
#[test]
fn test_no_path_possible() {
    let mut map = TestMap::new(5, 5);
    map.set_unwalkable(&[
        TilePosition { x: 1, y: 0 },
        TilePosition { x: 1, y: 1 },
        TilePosition { x: 1, y: 2 },
        TilePosition { x: 1, y: 3 },
        TilePosition { x: 1, y: 4 },
    ]);

    let mut pathfinder = PathFinder::default();
    assert!(pathfinder.find_walkable_path(&map, TilePosition { x: 0, y: 2 }, TilePosition { x: 2, y: 2 }).is_none());
}
```

---

*Testing analysis: 2026-04-26*
