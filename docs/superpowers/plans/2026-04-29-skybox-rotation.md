# Skybox Rotation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a slow, optional horizontal rotation to the existing skydome skybox.

**Architecture:** Persist a new `GraphicsSettings::skybox_rotation` boolean, expose it in the existing Graphics Settings window, compute a normalized UV offset from the frame timer, and send that offset to the skybox shader through global uniforms. The shader wraps `uv.x + offset` before sampling the panorama.

**Tech Stack:** Rust, serde/RON, rust-state UI paths, wgpu global uniform buffer, Slang shader.

---

### Task 1: Persisted Graphics Setting

**Files:**
- Modify: `korangar/korangar/src/settings/graphic.rs`
- Modify: `korangar/korangar/src/interface/windows/graphics_settings.rs`

- [ ] Add `skybox_rotation: bool` to `GraphicsSettings` with `#[serde(default = "default_true")]`.
- [ ] Set the default to `true`.
- [ ] Add a unit test proving old RON files without the field load with rotation enabled.
- [ ] Add a `state_button!` labeled `Skybox rotation` in Graphics Settings.

### Task 2: Render Instruction Offset

**Files:**
- Modify: `korangar/korangar/src/graphics/instruction.rs`
- Modify: `korangar/korangar/src/main.rs`

- [ ] Add `rotation_offset: f32` to `SkyboxInstruction`.
- [ ] Add `SkyboxInstruction::new(enabled, rotation_enabled, animation_timer_ms)`.
- [ ] Compute one full rotation per 1,200,000 ms.
- [ ] Unit-test disabled offset, initial offset, and wrapped offset.
- [ ] Build the skybox instruction from `graphics_settings.skybox_rotation`.

### Task 3: Shader Sampling

**Files:**
- Modify: `korangar/korangar/src/graphics/mod.rs`
- Modify: `korangar/korangar/shaders/modules/globals.slang`
- Modify: `korangar/korangar/shaders/passes/forward/skybox.slang`

- [ ] Add `skybox_rotation_offset` and padding to the Rust global uniforms.
- [ ] Add matching fields to Slang `GlobalUniforms`.
- [ ] Populate the uniform from `instruction.skybox.rotation_offset`.
- [ ] Sample `skybox_texture` with `float2(frac(input.uv.x + global_uniforms.skybox_rotation_offset), input.uv.y)`.
- [ ] Update the existing shader string test to assert the rotated UV sampling.

### Task 4: Verification

**Files:**
- Verify only.

- [ ] Run `cargo test -p korangar settings::graphic::tests::old_graphics_settings_files_enable_skybox_rotation_by_default`.
- [ ] Run `cargo test -p korangar graphics::instruction::tests::skybox_rotation_offset_is_zero_when_disabled`.
- [ ] Run `cargo test -p korangar graphics::tests::skybox_shader_renders_physical_skydome_mesh`.
- [ ] Run `cargo fmt --all --check`.
