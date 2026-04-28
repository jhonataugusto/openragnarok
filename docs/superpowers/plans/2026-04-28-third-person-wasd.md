# Third-Person WASD Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build an optional non-isometric third-person WASD movement mode in Korangar while preserving the current click-to-move gameplay when disabled.

**Architecture:** Add a client-only settings toggle, a focused `ThirdPersonCamera`, and a small input helper that converts `W/A/S/D` plus camera direction into throttled tile movement. Keep server protocol unchanged by reusing `InputEvent::PlayerMove` and the existing `RequestPlayerMovePacket` path.

**Tech Stack:** Rust 2024/nightly, Korangar client, `cgmath`, existing `rust_state` settings/UI, existing packet/networking layer.

---

### Task 1: Testable WASD Direction Helper

**Files:**
- Modify: `korangar/korangar/src/input/mod.rs`

- [ ] Add failing tests for camera-relative forward and diagonal movement.
- [ ] Implement `MovementKeyState` and `tile_offset_for_camera_movement`.
- [ ] Run the focused input tests.

### Task 2: Settings Toggle

**Files:**
- Modify: `korangar/korangar/src/settings/interface.rs`
- Modify: `korangar/korangar/src/interface/windows/interface_settings.rs`

- [ ] Add failing test that old settings load with `third_person_movement_enabled = false`.
- [ ] Add persisted setting with default false.
- [ ] Add Interface Settings toggle.
- [ ] Run the focused settings test.

### Task 3: Third-Person Camera

**Files:**
- Create: `korangar/korangar/src/world/cameras/third_person.rs`
- Modify: `korangar/korangar/src/world/cameras/mod.rs`
- Modify: `korangar/korangar/src/main.rs`

- [ ] Add `ThirdPersonCamera` with smoothed focus, smoothed distance clamp, DebugCamera-style quaternion orientation/FOV, and view/projection generation.
- [ ] Export it through `world::cameras`.
- [ ] Add it to `Client` initialization and render camera selection.

### Task 4: Movement Integration

**Files:**
- Modify: `korangar/korangar/src/input/event.rs`
- Modify: `korangar/korangar/src/input/mod.rs`
- Modify: `korangar/korangar/src/main.rs`

- [ ] Add camera rotation input for third-person yaw/pitch while right mouse is held.
- [ ] Generate WASD movement immediately, then adaptively every 40ms or more while held when the destination changes.
- [ ] Stop generating destinations when movement keys are released.
- [ ] Block only click-to-move on `PickerTarget::Tile` while third-person mode is active.

### Task 5: Verification

**Files:**
- No source changes expected.

- [ ] Run `cargo fmt --all` from `korangar/`.
- [ ] Run focused tests for input/settings.
- [ ] Run `cargo check -p korangar`.
- [ ] Review git diff.
- [ ] Commit implementation.
