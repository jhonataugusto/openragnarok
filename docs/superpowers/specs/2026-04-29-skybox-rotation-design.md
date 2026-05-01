# Skybox Rotation Design

## Goal

Add an optional, subtle horizontal skybox rotation so the sky feels like a slowly turning planet.

## User-Facing Behavior

- The skybox rotation is enabled by default.
- A toggle in Graphics Settings lets the user disable the rotation.
- When disabled, the skybox renders with the same horizontal alignment as before.
- The rotation is intentionally slow, targeting roughly one complete panorama loop every twenty minutes.

## Architecture

- Store the new preference in `GraphicsSettings` as a persisted boolean with a serde default so older `graphics_settings.ron` files keep loading.
- Pass the per-frame rotation offset through `SkyboxInstruction` into the existing global graphics uniforms.
- Apply the offset in the skybox shader by wrapping the horizontal panorama coordinate before sampling `skybox\day.png`.

## Constraints

- Keep the change client-only.
- Do not alter `rathena-master/`.
- Keep the skybox texture format and `PACKETVER=20220406` untouched.
- Preserve current behavior when the setting is off.

## Testing

- Unit-test old graphics settings deserialization so the new field defaults to enabled.
- Unit-test the skybox rotation offset calculation.
- Unit-test that the shader samples using a wrapped rotated UV.
