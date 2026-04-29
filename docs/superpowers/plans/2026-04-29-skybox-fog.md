# Skybox Fog Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a real client-side skybox and lightweight distance fog to Korangar, with fog color controlled by the skybox preset.

**Architecture:** The skybox is drawn at the start of the existing forward pass using a dedicated fullscreen shader/drawer. Fog parameters are uploaded through global uniforms and applied in the existing forward model, entity, and water shaders after lighting is computed.

**Tech Stack:** Rust 2024, wgpu 29, Slang shaders, Korangar render instruction/drawer architecture.

---

### Task 1: Add Skybox/Fog Data Model

**Files:**
- Modify: `korangar/korangar/src/graphics/instruction.rs`
- Modify: `korangar/korangar/src/graphics/mod.rs`

- [x] Add `FogInstruction` and `SkyboxInstruction` structs with conservative defaults.
- [x] Add unit tests for fog factor behavior in Rust helper logic.
- [x] Add fields to `Uniforms`/`RenderInstruction` without changing runtime rendering yet.

### Task 2: Upload Fog Uniforms

**Files:**
- Modify: `korangar/korangar/src/graphics/mod.rs`
- Modify: `korangar/korangar/shaders/modules/globals.slang`

- [x] Add fog fields to Rust `GlobalUniforms`.
- [x] Add matching fields to Slang `GlobalUniforms`.
- [x] Keep layout aligned by using `f32`/`u32` groups that match Rust order.

### Task 3: Add Shared Fog Shader Function

**Files:**
- Modify: `korangar/korangar/shaders/modules/forward.slang`
- Modify: `korangar/korangar/shaders/passes/forward/model.slang`
- Modify: `korangar/korangar/shaders/passes/forward/entity.slang`
- Modify: `korangar/korangar/shaders/passes/forward/wave.slang`

- [x] Add `apply_fog` helper.
- [x] Apply fog after lighting in model, entity, and water fragments.
- [x] Preserve alpha values and transparent pass outputs.

### Task 4: Add Skybox Shader and Drawer

**Files:**
- Create: `korangar/korangar/shaders/passes/forward/skybox.slang`
- Create: `korangar/korangar/src/graphics/passes/forward/skybox.rs`
- Modify: `korangar/korangar/src/graphics/passes/forward/mod.rs`
- Modify: `korangar/korangar/src/graphics/engine.rs`

- [x] Implement fullscreen skybox shader using `screen_space.slang`.
- [x] Implement `ForwardSkyboxDrawer` following existing drawer patterns.
- [x] Draw skybox before models in the forward pass.

### Task 5: Provide Initial Preset Texture

**Files:**
- Modify: `korangar/korangar/src/graphics/mod.rs`
- Modify: `korangar/korangar/src/graphics/engine.rs`

- [x] Use a generated 1x2 gradient texture as the default skybox source.
- [x] Bind the texture through the drawer, avoiding asset/runtime file dependencies for the first version.
- [x] Keep later cubemap/panorama asset loading as a future extension.

### Task 6: Wire Runtime Defaults

**Files:**
- Modify: `korangar/korangar/src/main.rs`

- [x] Emit enabled skybox/fog instructions during frame rendering.
- [x] Use skybox-derived fog color, distance start/end, and density values.
- [x] Keep UI and picker passes unaffected.

### Task 7: Verify

**Commands:**
- `cd korangar && cargo fmt --all --check`
- `cd korangar && cargo test -p korangar --all-features`
- `cd korangar && cargo check -p korangar --all-features`

- [x] Run all verification commands with `C:\Program Files\NASM` available in `PATH`.
- [x] Record any environment-only caveats in the final summary.
