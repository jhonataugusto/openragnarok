# Circular Minimap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implementar um minimapa circular estilo radar no canto superior direito do HUD, com jogador fixo no centro, zoom por scroll e rotacao apenas no modo terceira pessoa.

**Architecture:** A feature fica isolada em um modulo `interface::minimap` para calculos e estado de runtime, com configuracoes persistentes em `InterfaceSettings`. A renderizacao usa uma instrucao HUD propria para o disco do minimapa e uma extensao pequena no renderer de interface para texto rotacionado das direcoes.

**Tech Stack:** Rust 2024, Korangar client, wgpu, Slang shaders, `rust-state`, `serde`/RON, `cgmath`.

---

## Estado Inicial E Cuidados

O workspace ja tem mudancas locais em arquivos do cliente/networking antes deste plano. Nao reverta essas mudancas. Ao implementar, confira `git status --short` antes de cada commit e stageie somente os arquivos da tarefa atual.

Spec aprovada: `docs/superpowers/specs/2026-05-01-circular-minimap-design.md`

## Estrutura De Arquivos

- Criar `korangar/korangar/src/interface/minimap.rs`
  - Responsavel por estado de zoom, caminho da textura do mapa, geometria do HUD, conversao tile->UV, regra de rotacao e testes unitarios puros.
- Modificar `korangar/korangar/src/interface/mod.rs`
  - Exportar o modulo `minimap`.
- Modificar `korangar/korangar/src/settings/interface.rs`
  - Persistir `circular_minimap_enabled` e `circular_minimap_rotate_in_third_person` com defaults compativeis.
- Modificar `korangar/korangar/src/interface/windows/interface_settings.rs`
  - Adicionar os dois toggles em Interface Settings.
- Modificar `korangar/korangar/src/input/event.rs`
  - Adicionar evento `ZoomCircularMinimap { scroll_delta: f32 }`.
- Modificar `korangar/korangar/src/graphics/instruction.rs`
  - Adicionar `MinimapInstruction` e rotacao opcional em instrucoes de texto de interface.
- Modificar `korangar/korangar/src/renderer/interface.rs`
  - Adicionar helper para texto rotacionado usado pelas labels `NORTH`, `SOUTH`, `EAST`, `WEST`.
- Criar `korangar/korangar/src/graphics/passes/interface/minimap.rs`
  - Drawer pequeno para desenhar o disco do minimapa com UV offset, zoom, rotacao e mascara circular.
- Modificar `korangar/korangar/src/graphics/passes/interface/mod.rs`
  - Exportar `InterfaceMinimapDrawer`.
- Criar `korangar/korangar/shaders/passes/interface/minimap.slang`
  - Shader dedicado para o disco do minimapa.
- Modificar `korangar/korangar/shaders/modules/interface.slang`
  - Suportar rotacao opcional nos vertices de texto/retangulo de interface.
- Modificar `korangar/korangar/shaders/passes/interface/rectangle.slang`
  - Usar a rotacao opcional vinda do modulo de interface.
- Modificar `korangar/korangar/src/graphics/passes/interface/rectangle.rs`
  - Preencher dados de rotacao para texto normal e rotacionado.
- Modificar `korangar/korangar/src/graphics/engine.rs`
  - Criar, preparar, fazer upload e desenhar `InterfaceMinimapDrawer` no pass de interface.
- Modificar `korangar/korangar/src/main.rs`
  - Adicionar estado de minimapa no `Client`, carregar textura por mapa, consumir scroll no circulo, criar `MinimapInstruction` e renderizar labels.

---

### Task 1: Calculos Puros Do Minimap

**Files:**
- Create: `korangar/korangar/src/interface/minimap.rs`
- Modify: `korangar/korangar/src/interface/mod.rs`
- Test: `korangar/korangar/src/interface/minimap.rs`

- [ ] **Step 1: Criar testes falhando para geometria, zoom, UV, rotacao e path**

Adicionar `korangar/korangar/src/interface/minimap.rs` com este conteudo inicial:

```rust
use cgmath::{InnerSpace, Rad, Vector2, Vector3};
use ragnarok_packets::TilePosition;

use crate::graphics::{ScreenPosition, ScreenSize};

pub const DEFAULT_MINIMAP_DIAMETER: f32 = 180.0;
pub const DEFAULT_MINIMAP_MARGIN: f32 = 16.0;
pub const DEFAULT_ZOOM_TILES: f32 = 48.0;
pub const MIN_ZOOM_TILES: f32 = 18.0;
pub const MAX_ZOOM_TILES: f32 = 96.0;
pub const SCROLL_ZOOM_FACTOR: f32 = 0.003;
const UI_MAP_TEXTURE_PREFIX: &str = "\u{c720}\u{c800}\u{c778}\u{d130}\u{d398}\u{c774}\u{c2a4}\\map";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MinimapArea {
    pub position: ScreenPosition,
    pub size: ScreenSize,
}

impl MinimapArea {
    pub fn center(self) -> ScreenPosition {
        ScreenPosition {
            left: self.position.left + self.size.width * 0.5,
            top: self.position.top + self.size.height * 0.5,
        }
    }

    pub fn radius(self) -> f32 {
        self.size.width.min(self.size.height) * 0.5
    }
}

#[derive(Clone, Debug)]
pub struct CircularMinimapState {
    zoom_tiles: f32,
    map_texture_path: Option<String>,
}

impl Default for CircularMinimapState {
    fn default() -> Self {
        Self {
            zoom_tiles: DEFAULT_ZOOM_TILES,
            map_texture_path: None,
        }
    }
}

impl CircularMinimapState {
    pub fn zoom_tiles(&self) -> f32 {
        self.zoom_tiles
    }

    pub fn apply_scroll(&mut self, scroll_delta: f32) {
        self.zoom_tiles = clamp_zoom(self.zoom_tiles * (1.0 - scroll_delta * SCROLL_ZOOM_FACTOR));
    }

    pub fn set_map_name(&mut self, map_name: &str) {
        self.map_texture_path = Some(map_texture_path(map_name));
    }

    pub fn map_texture_path(&self) -> Option<&str> {
        self.map_texture_path.as_deref()
    }
}

pub fn minimap_area(window_size: ScreenSize, interface_scaling: f32) -> MinimapArea {
    let diameter = DEFAULT_MINIMAP_DIAMETER * interface_scaling;
    let margin = DEFAULT_MINIMAP_MARGIN * interface_scaling;

    MinimapArea {
        position: ScreenPosition {
            left: (window_size.width - diameter - margin).max(margin),
            top: margin,
        },
        size: ScreenSize {
            width: diameter,
            height: diameter,
        },
    }
}

pub fn is_inside_minimap(area: MinimapArea, point: ScreenPosition) -> bool {
    let center = area.center();
    let delta = Vector2::new(point.left - center.left, point.top - center.top);
    delta.magnitude2() <= area.radius() * area.radius()
}

pub fn clamp_zoom(zoom_tiles: f32) -> f32 {
    zoom_tiles.clamp(MIN_ZOOM_TILES, MAX_ZOOM_TILES)
}

pub fn player_uv(tile_position: TilePosition, map_width: u16, map_height: u16) -> Vector2<f32> {
    let width = map_width.max(1) as f32;
    let height = map_height.max(1) as f32;
    Vector2::new(tile_position.x as f32 / width, 1.0 - tile_position.y as f32 / height)
}

pub fn should_rotate_minimap(rotation_enabled: bool, third_person_active: bool) -> bool {
    rotation_enabled && third_person_active
}

pub fn minimap_rotation(view_direction: Vector3<f32>) -> Rad<f32> {
    let horizontal = Vector2::new(view_direction.x, view_direction.z);

    if horizontal.magnitude2() <= f32::EPSILON {
        return Rad(0.0);
    }

    let horizontal = horizontal.normalize();
    Rad(f32::atan2(horizontal.x, horizontal.y))
}

pub fn map_texture_path(map_name: &str) -> String {
    let base_name = map_name
        .rsplit_once('\\')
        .map(|(_, file_name)| file_name)
        .unwrap_or(map_name)
        .trim_end_matches(".gat")
        .trim_end_matches(".rsw")
        .trim_end_matches(".gnd");

    format!("{UI_MAP_TEXTURE_PREFIX}\\{base_name}.bmp")
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use cgmath::{Vector3, assert_relative_eq};
    use ragnarok_packets::TilePosition;

    use super::*;

    #[test]
    fn minimap_area_is_top_right() {
        let area = minimap_area(
            ScreenSize {
                width: 1280.0,
                height: 720.0,
            },
            1.0,
        );

        assert_eq!(area.position.left, 1084.0);
        assert_eq!(area.position.top, 16.0);
        assert_eq!(area.size.width, 180.0);
        assert_eq!(area.size.height, 180.0);
    }

    #[test]
    fn point_inside_circle_is_detected() {
        let area = MinimapArea {
            position: ScreenPosition { left: 100.0, top: 20.0 },
            size: ScreenSize {
                width: 180.0,
                height: 180.0,
            },
        };

        assert!(is_inside_minimap(area, area.center()));
        assert!(is_inside_minimap(area, ScreenPosition { left: 100.0, top: 110.0 }));
        assert!(!is_inside_minimap(area, ScreenPosition { left: 99.0, top: 110.0 }));
    }

    #[test]
    fn zoom_is_clamped() {
        assert_eq!(clamp_zoom(1.0), MIN_ZOOM_TILES);
        assert_eq!(clamp_zoom(200.0), MAX_ZOOM_TILES);
        assert_eq!(clamp_zoom(48.0), 48.0);
    }

    #[test]
    fn scroll_changes_zoom_with_limits() {
        let mut state = CircularMinimapState::default();

        state.apply_scroll(30.0);
        assert!(state.zoom_tiles() < DEFAULT_ZOOM_TILES);

        state.apply_scroll(-100000.0);
        assert_eq!(state.zoom_tiles(), MAX_ZOOM_TILES);
    }

    #[test]
    fn player_tile_maps_to_uv_with_inverted_y_axis() {
        let uv = player_uv(TilePosition { x: 50, y: 25 }, 200, 100);

        assert_relative_eq!(uv.x, 0.25);
        assert_relative_eq!(uv.y, 0.75);
    }

    #[test]
    fn rotation_requires_config_and_third_person() {
        assert!(should_rotate_minimap(true, true));
        assert!(!should_rotate_minimap(true, false));
        assert!(!should_rotate_minimap(false, true));
    }

    #[test]
    fn camera_direction_maps_to_rotation_angle() {
        assert_relative_eq!(minimap_rotation(Vector3::new(0.0, -0.4, 1.0)).0, 0.0, epsilon = 0.0001);
        assert_relative_eq!(minimap_rotation(Vector3::new(1.0, -0.4, 0.0)).0, FRAC_PI_2, epsilon = 0.0001);
    }

    #[test]
    fn map_texture_path_uses_map_base_name() {
        let path = map_texture_path("prontera.gat");

        assert!(path.ends_with("\\prontera.bmp"));
        assert!(path.contains("\\map\\"));
    }
}
```

- [ ] **Step 2: Exportar modulo e rodar teste falhando**

Em `korangar/korangar/src/interface/mod.rs`, adicionar:

```rust
pub mod minimap;
```

Rodar:

```powershell
cd D:\ragnarok\korangar
cargo test -p korangar interface::minimap --lib
```

Expected: falha de compilacao se `TilePosition` usar tipos diferentes de `i16`, ou PASS se os campos forem compativeis. Se houver falha de tipo em `player_uv`, converter com `as f32` mantendo os mesmos testes.

- [ ] **Step 3: Rodar teste de confirmacao**

Run:

```powershell
cd D:\ragnarok\korangar
cargo test -p korangar interface::minimap --lib
```

Expected: PASS para a suite `interface::minimap`.

- [ ] **Step 4: Commit**

```powershell
cd D:\ragnarok
git add korangar/korangar/src/interface/mod.rs korangar/korangar/src/interface/minimap.rs
git commit -m "Add circular minimap calculations"
```

---

### Task 2: Settings Persistentes E Toggles

**Files:**
- Modify: `korangar/korangar/src/settings/interface.rs`
- Modify: `korangar/korangar/src/interface/windows/interface_settings.rs`
- Test: `korangar/korangar/src/settings/interface.rs`

- [ ] **Step 1: Escrever teste falhando para defaults antigos**

Em `korangar/korangar/src/settings/interface.rs`, no teste `old_interface_settings_files_load_with_cinematic_defaults`, adicionar estas assercoes ao final:

```rust
assert!(!settings.circular_minimap_enabled);
assert!(settings.circular_minimap_rotate_in_third_person);
```

Rodar:

```powershell
cd D:\ragnarok\korangar
cargo test -p korangar settings::interface::tests::old_interface_settings_files_load_with_cinematic_defaults --lib
```

Expected: FAIL porque os campos ainda nao existem.

- [ ] **Step 2: Adicionar campos em `InterfaceSettings`**

Em `InterfaceSettings`, depois de `third_person_movement_enabled`, adicionar:

```rust
#[serde(default = "default_false")]
pub circular_minimap_enabled: bool,
#[serde(default = "default_true")]
pub circular_minimap_rotate_in_third_person: bool,
```

Em `Default for InterfaceSettings`, adicionar:

```rust
circular_minimap_enabled: false,
circular_minimap_rotate_in_third_person: true,
```

- [ ] **Step 3: Adicionar toggles na janela de settings**

Em `korangar/korangar/src/interface/windows/interface_settings.rs`, depois do botao `Third-person WASD movement`, adicionar:

```rust
state_button! {
    text: "Circular minimap",
    state: self.settings_path.circular_minimap_enabled(),
    event: Toggle(self.settings_path.circular_minimap_enabled()),
},
state_button! {
    text: "Rotate minimap in third person",
    state: self.settings_path.circular_minimap_rotate_in_third_person(),
    event: Toggle(self.settings_path.circular_minimap_rotate_in_third_person()),
},
```

- [ ] **Step 4: Rodar teste de settings**

Run:

```powershell
cd D:\ragnarok\korangar
cargo test -p korangar settings::interface::tests::old_interface_settings_files_load_with_cinematic_defaults --lib
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
cd D:\ragnarok
git add korangar/korangar/src/settings/interface.rs korangar/korangar/src/interface/windows/interface_settings.rs
git commit -m "Add circular minimap settings"
```

---

### Task 3: Evento De Zoom Do Minimap E Consumo De Scroll

**Files:**
- Modify: `korangar/korangar/src/input/event.rs`
- Modify: `korangar/korangar/src/main.rs`
- Test: `korangar/korangar/src/interface/minimap.rs`

- [ ] **Step 1: Adicionar evento de input**

Em `korangar/korangar/src/input/event.rs`, depois de `ZoomCamera`, adicionar:

```rust
/// Zoom the circular minimap.
ZoomCircularMinimap {
    /// Raw scroll delta.
    scroll_delta: f32,
},
```

- [ ] **Step 2: Adicionar estado ao `Client`**

Em `korangar/korangar/src/main.rs`, importar:

```rust
use interface::minimap::{CircularMinimapState, is_inside_minimap, minimap_area};
```

No struct `Client`, perto das cameras, adicionar:

```rust
circular_minimap: CircularMinimapState,
```

Na inicializacao de `Client::init`, perto de `third_person_movement`, adicionar:

```rust
let circular_minimap = CircularMinimapState::default();
```

No literal `Self { ... }`, adicionar:

```rust
circular_minimap,
```

- [ ] **Step 3: Atualizar mapa atual quando carregar/trocar mapa**

No handler de `NetworkEvent::ChangeMap { map_name, position }`, antes de `self.async_loader.request_map_load(map_name, Some(position));`, adicionar:

```rust
self.circular_minimap.set_map_name(&map_name);
```

Se houver outro ponto que carrega mapa inicial com `request_map_load`, aplicar a mesma chamada antes da requisicao usando o nome de mapa disponivel.

- [ ] **Step 4: Consumir scroll sobre o circulo**

Em `main.rs`, no bloco que hoje trata:

```rust
if let Some(delta) = input_report.scroll {
    if is_interface_hovered {
        interface_frame.scroll(&self.client_state, delta);
    } else if !cinematic_dialog_active {
        #[cfg_attr(feature = "debug", korangar_debug::debug_condition(!render_options.use_debug_camera))]
        self.input_event_buffer.push(InputEvent::ZoomCamera { zoom_factor: delta });
    }
}
```

Substituir por:

```rust
if let Some(delta) = input_report.scroll {
    let circular_minimap_enabled = *self
        .client_state
        .follow(client_state().interface_settings().circular_minimap_enabled());
    let minimap_area = minimap_area(self.graphics_engine.get_window_size().into(), self.interface.get_scaling().get_factor());
    let minimap_hovered = circular_minimap_enabled && is_inside_minimap(minimap_area, input_report.mouse_position);

    if minimap_hovered {
        self.input_event_buffer
            .push(InputEvent::ZoomCircularMinimap { scroll_delta: delta });
    } else if is_interface_hovered {
        interface_frame.scroll(&self.client_state, delta);
    } else if !cinematic_dialog_active {
        #[cfg_attr(feature = "debug", korangar_debug::debug_condition(!render_options.use_debug_camera))]
        self.input_event_buffer.push(InputEvent::ZoomCamera { zoom_factor: delta });
    }
}
```

Se `self.interface.get_scaling().get_factor()` nao existir, use o caminho de estado ja presente em settings:

```rust
let interface_scaling = self
    .client_state
    .follow(client_state().interface_settings().scaling())
    .get_factor();
```

- [ ] **Step 5: Tratar evento de zoom do minimap**

No `match event` em `main.rs`, depois de `InputEvent::ZoomCamera`, adicionar:

```rust
InputEvent::ZoomCircularMinimap { scroll_delta } => {
    self.circular_minimap.apply_scroll(scroll_delta);
}
```

- [ ] **Step 6: Rodar testes existentes e check parcial**

Run:

```powershell
cd D:\ragnarok\korangar
cargo test -p korangar interface::minimap --lib
cargo check -p korangar
```

Expected: PASS nos testes e `cargo check` sem erros. Se o acesso a scaling tiver nome diferente, corrigir usando o tipo `Scaling` existente em `korangar/korangar/src/loaders/` e manter o calculo de area no mesmo ponto.

- [ ] **Step 7: Commit**

```powershell
cd D:\ragnarok
git add korangar/korangar/src/input/event.rs korangar/korangar/src/main.rs korangar/korangar/src/interface/minimap.rs
git commit -m "Route circular minimap zoom input"
```

---

### Task 4: Instrucao GPU Do Disco Do Minimap

**Files:**
- Modify: `korangar/korangar/src/graphics/instruction.rs`
- Create: `korangar/korangar/src/graphics/passes/interface/minimap.rs`
- Modify: `korangar/korangar/src/graphics/passes/interface/mod.rs`
- Create: `korangar/korangar/shaders/passes/interface/minimap.slang`
- Modify: `korangar/korangar/src/graphics/engine.rs`

- [ ] **Step 1: Adicionar `MinimapInstruction`**

Em `korangar/korangar/src/graphics/instruction.rs`, adicionar campo em `RenderInstruction<'a>` depois de `interface`:

```rust
pub minimap: Option<&'a MinimapInstruction>,
```

Adicionar struct perto de `InterfaceRectangleInstruction`:

```rust
#[derive(Clone, Debug)]
pub struct MinimapInstruction {
    pub screen_position: ScreenPosition,
    pub screen_size: ScreenSize,
    pub player_uv: Vector2<f32>,
    pub visible_tiles: f32,
    pub map_size_tiles: Vector2<f32>,
    pub rotation_radians: f32,
    pub tint: Color,
    pub texture: Arc<Texture>,
}
```

Em `Default for RenderInstruction`, o derive default continua valido porque `Option` tem default `None`.

- [ ] **Step 2: Criar shader do minimap**

Criar `korangar/korangar/shaders/passes/interface/minimap.slang`:

```slang
#language slang 2026

import globals;
import coordinate_space;

struct MinimapInstanceData {
    screen_position: float2;
    screen_size: float2;
    player_uv: float2;
    map_size_tiles: float2;
    visible_tiles: float;
    rotation_radians: float;
    tint: float4;
};

struct VertexInput {
    uint vertex_index: SV_VertexID;
};

struct VertexOutput {
    float4 position: SV_Position;
    [[vk::location(0)]] float2 local_uv: float2;
};

[[vk::binding(0, 0)]] var global_uniforms: ConstantBuffer<GlobalUniforms>;
[[vk::binding(1, 0)]] var nearest_sampler: SamplerState;
[[vk::binding(2, 0)]] var linear_sampler: SamplerState;
[[vk::binding(0, 1)]] var instance_data: ConstantBuffer<MinimapInstanceData>;
[[vk::binding(1, 1)]] var map_texture: Texture2D;

static const var POSITIONS = float2[6](
    float2(0.0, 0.0),
    float2(1.0, 0.0),
    float2(0.0, 1.0),
    float2(0.0, 1.0),
    float2(1.0, 0.0),
    float2(1.0, 1.0)
);

[[shader("vertex")]]
func vs_main(input: VertexInput) -> VertexOutput {
    let local_uv = POSITIONS[input.vertex_index];
    let screen_position = instance_data.screen_position + local_uv * instance_data.screen_size;

    var output: VertexOutput;
    output.position = coordinate_space::screen_to_clip_space(screen_position);
    output.local_uv = local_uv;
    return output;
}

[[shader("pixel")]]
func fs_main(input: VertexOutput) -> float4 {
    let centered = input.local_uv * 2.0 - float2(1.0);
    let radius = length(centered);

    if (radius > 1.0) {
        discard;
    }

    let angle = instance_data.rotation_radians;
    let sine = sin(angle);
    let cosine = cos(angle);
    let rotated = float2(
        centered.x * cosine - centered.y * sine,
        centered.x * sine + centered.y * cosine
    );

    let tiles_per_radius = instance_data.visible_tiles;
    let tile_offset = rotated * tiles_per_radius;
    let uv_offset = float2(
        tile_offset.x / max(instance_data.map_size_tiles.x, 1.0),
        -tile_offset.y / max(instance_data.map_size_tiles.y, 1.0)
    );
    let sample_uv = instance_data.player_uv + uv_offset;

    if (any(sample_uv < float2(0.0)) || any(sample_uv > float2(1.0))) {
        return float4(0.02, 0.025, 0.03, 0.72) * instance_data.tint;
    }

    let feather = smoothstep(1.0, 0.92, radius);
    let map_color = map_texture.Sample(linear_sampler, sample_uv) * instance_data.tint;
    return float4(map_color.rgb, map_color.a * feather);
}
```

- [ ] **Step 3: Criar drawer do minimap**

Criar `korangar/korangar/src/graphics/passes/interface/minimap.rs` seguindo o padrao de `rectangle.rs`. Estrutura principal:

```rust
use std::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};
use wgpu::util::StagingBelt;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingResource, BindingType, BlendState, BufferBindingType, BufferUsages, ColorTargetState, ColorWrites, CommandEncoder, Device,
    FragmentState, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, Queue, RenderPass, RenderPipeline,
    RenderPipelineDescriptor, ShaderStages, TextureSampleType, TextureViewDimension, VertexState,
};

use crate::graphics::passes::{
    BindGroupCount, ColorAttachmentCount, DepthAttachmentCount, Drawer, InterfaceRenderPassContext, RenderPassContext,
};
use crate::graphics::shader_compiler::ShaderCompiler;
use crate::graphics::{Buffer, Capabilities, GlobalContext, MinimapInstruction, Prepare, RenderInstruction, Texture};

const DRAWER_NAME: &str = "interface minimap";

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct InstanceData {
    screen_position: [f32; 2],
    screen_size: [f32; 2],
    player_uv: [f32; 2],
    map_size_tiles: [f32; 2],
    visible_tiles: f32,
    rotation_radians: f32,
    tint: [f32; 4],
    padding: [f32; 2],
}

pub(crate) struct InterfaceMinimapDrawer {
    instance_data_buffer: Buffer<InstanceData>,
    bind_group_layout: BindGroupLayout,
    bind_group: Option<BindGroup>,
    pipeline: RenderPipeline,
    instance_data: Option<InstanceData>,
    draw_count: u32,
}

impl Drawer<{ BindGroupCount::One }, { ColorAttachmentCount::One }, { DepthAttachmentCount::None }> for InterfaceMinimapDrawer {
    type Context = InterfaceRenderPassContext;
    type DrawData<'data> = Option<&'data MinimapInstruction>;

    fn new(
        _capabilities: &Capabilities,
        device: &Device,
        _queue: &Queue,
        shader_compiler: &ShaderCompiler,
        _global_context: &GlobalContext,
        render_pass_context: &Self::Context,
    ) -> Self {
        let shader_module = shader_compiler.create_shader_module("interface", "minimap");
        let instance_data_buffer = Buffer::with_capacity(
            device,
            format!("{DRAWER_NAME} instance data"),
            BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            size_of::<InstanceData>() as _,
        );

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some(DRAWER_NAME),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(size_of::<InstanceData>() as _),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });

        let pass_bind_group_layouts = Self::Context::bind_group_layout(device);
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some(DRAWER_NAME),
            bind_group_layouts: &[Some(pass_bind_group_layouts[0]), Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(DRAWER_NAME),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: render_pass_context.color_attachment_formats()[0],
                    blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: ColorWrites::default(),
                })],
            }),
            primitive: Default::default(),
            multisample: MultisampleState::default(),
            depth_stencil: None,
            cache: None,
            multiview_mask: None,
        });

        Self {
            instance_data_buffer,
            bind_group_layout,
            bind_group: None,
            pipeline,
            instance_data: None,
            draw_count: 0,
        }
    }

    fn draw(&mut self, pass: &mut RenderPass<'_>, _draw_data: Self::DrawData<'_>) {
        if self.draw_count == 0 {
            return;
        }

        let Some(bind_group) = &self.bind_group else {
            return;
        };

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(1, bind_group, &[]);
        pass.draw(0..6, 0..1);
    }
}

impl Prepare for InterfaceMinimapDrawer {
    fn prepare(&mut self, device: &Device, instructions: &RenderInstruction) {
        let Some(instruction) = instructions.minimap else {
            self.draw_count = 0;
            self.instance_data = None;
            self.bind_group = None;
            return;
        };

        self.instance_data = Some(InstanceData {
            screen_position: instruction.screen_position.into(),
            screen_size: instruction.screen_size.into(),
            player_uv: instruction.player_uv.into(),
            map_size_tiles: instruction.map_size_tiles.into(),
            visible_tiles: instruction.visible_tiles,
            rotation_radians: instruction.rotation_radians,
            tint: instruction.tint.components_linear(),
            padding: [0.0; 2],
        });
        self.draw_count = 1;
        self.bind_group = Some(Self::create_bind_group(device, &self.bind_group_layout, &self.instance_data_buffer, &instruction.texture));
    }

    fn upload(&mut self, device: &Device, staging_belt: &mut StagingBelt, command_encoder: &mut CommandEncoder) {
        if let Some(instance_data) = self.instance_data {
            self.instance_data_buffer
                .write(device, staging_belt, command_encoder, std::slice::from_ref(&instance_data));
        }
    }
}

impl InterfaceMinimapDrawer {
    fn create_bind_group(
        device: &Device,
        bind_group_layout: &BindGroupLayout,
        instance_data_buffer: &Buffer<InstanceData>,
        texture: &Texture,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some(DRAWER_NAME),
            layout: bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: instance_data_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(texture.get_texture_view()),
                },
            ],
        })
    }
}
```

- [ ] **Step 4: Exportar drawer**

Em `korangar/korangar/src/graphics/passes/interface/mod.rs`, adicionar:

```rust
mod minimap;
```

e:

```rust
pub(crate) use minimap::InterfaceMinimapDrawer;
```

- [ ] **Step 5: Integrar drawer no engine**

Em `korangar/korangar/src/graphics/engine.rs`:

1. Importar/usar `InterfaceMinimapDrawer` junto dos demais drawers de interface.
2. Adicionar campo em `EngineContext`:

```rust
interface_minimap_drawer: InterfaceMinimapDrawer,
```

3. Criar o drawer junto de `interface_rectangle_drawer`:

```rust
let interface_minimap_drawer = InterfaceMinimapDrawer::new(
    &capabilities,
    &device,
    &queue,
    &shader_compiler,
    &global_context,
    &interface_render_pass_context,
);
```

4. Preparar junto dos outros drawers:

```rust
context.interface_minimap_drawer.prepare(&self.device, instruction);
```

5. Fazer upload:

```rust
visitor.upload(&mut context.interface_minimap_drawer);
```

6. No pass de interface, desenhar depois dos retangulos normais:

```rust
engine_context
    .interface_minimap_drawer
    .draw(&mut render_pass, instruction.minimap);
```

- [ ] **Step 6: Check de compilacao**

Run:

```powershell
cd D:\ragnarok\korangar
cargo check -p korangar
```

Expected: PASS. Se o `Buffer::write` exigir slice com vida diferente, criar array local `let data = [instance_data];` e passar `&data`.

- [ ] **Step 7: Commit**

```powershell
cd D:\ragnarok
git add korangar/korangar/src/graphics/instruction.rs korangar/korangar/src/graphics/passes/interface/mod.rs korangar/korangar/src/graphics/passes/interface/minimap.rs korangar/korangar/shaders/passes/interface/minimap.slang korangar/korangar/src/graphics/engine.rs
git commit -m "Add minimap render instruction"
```

---

### Task 5: Texto Rotacionado Para NORTH/SOUTH/EAST/WEST

**Files:**
- Modify: `korangar/korangar/src/graphics/instruction.rs`
- Modify: `korangar/korangar/src/graphics/passes/interface/rectangle.rs`
- Modify: `korangar/korangar/shaders/modules/interface.slang`
- Modify: `korangar/korangar/shaders/passes/interface/rectangle.slang`
- Modify: `korangar/korangar/src/renderer/interface.rs`

- [ ] **Step 1: Adicionar dados de rotacao**

Em `korangar/korangar/src/graphics/instruction.rs`, adicionar:

```rust
#[derive(Clone, Copy, Debug, Default)]
pub struct InterfaceRotation {
    pub center: ScreenPosition,
    pub angle_radians: f32,
}
```

Alterar `InterfaceRectangleInstruction::Text` para incluir:

```rust
rotation: Option<InterfaceRotation>,
```

Atualizar cada criacao de `Text` com `rotation: None`.

- [ ] **Step 2: Propagar rotacao para instance data**

Em `korangar/korangar/src/graphics/passes/interface/rectangle.rs`, em `InstanceData`, adicionar:

```rust
rotation_center: [f32; 2],
rotation_angle: f32,
rotation_enabled: u32,
```

Substituir o `padding: [f32; 2]` por padding suficiente para alinhamento:

```rust
padding: [f32; 3],
```

Em cada `InstanceData` que nao usa rotacao:

```rust
rotation_center: [0.0, 0.0],
rotation_angle: 0.0,
rotation_enabled: 0,
padding: [0.0; 3],
```

No match de `InterfaceRectangleInstruction::Text`, usar:

```rust
let (rotation_center, rotation_angle, rotation_enabled) = rotation
    .map(|rotation| {
        (
            <[f32; 2]>::from(rotation.center),
            rotation.angle_radians,
            1,
        )
    })
    .unwrap_or(([0.0, 0.0], 0.0, 0));
```

e preencher esses campos.

- [ ] **Step 3: Atualizar shader module**

Em `korangar/korangar/shaders/modules/interface.slang`, adicionar campos em `RectangleInstanceData`:

```slang
public var rotation_center: float2;
public var rotation_angle: float;
public var rotation_enabled: uint;
```

No `rectangle_vertex_shader`, depois de calcular `position`, aplicar:

```slang
var final_position = position;

if (instance.rotation_enabled != 0) {
    let center_clip = coordinate_space::screen_to_clip_space(instance.rotation_center / float2(global_uniforms.interface_size));
    let offset = final_position.xy - center_clip.xy;
    let sine = sin(instance.rotation_angle);
    let cosine = cos(instance.rotation_angle);
    let rotated = float2(
        offset.x * cosine - offset.y * sine,
        offset.x * sine + offset.y * cosine
    );
    final_position.xy = center_clip.xy + rotated;
}

output.position = float4(final_position.xy, 0.0, 1.0);
```

Preservar o restante do shader, incluindo `texture_coordinates`.

- [ ] **Step 4: Adicionar API no renderer de interface**

Em `korangar/korangar/src/renderer/interface.rs`, adicionar funcao publica:

```rust
pub fn render_text_rotated(
    &self,
    text: &str,
    position: ScreenPosition,
    available_width: f32,
    clip: ScreenClip,
    color: Color,
    highlight_color: Color,
    font_size: FontSize,
    rotation: InterfaceRotation,
) {
    let before = self.instructions.borrow().len();
    self.render_text(text, position, available_width, clip, color, highlight_color, font_size);
    let mut instructions = self.instructions.borrow_mut();

    for instruction in instructions.iter_mut().skip(before) {
        if let InterfaceRectangleInstruction::Text { rotation: instruction_rotation, .. } = instruction {
            *instruction_rotation = Some(rotation);
        }
    }
}
```

Adicionar imports:

```rust
use crate::graphics::InterfaceRotation;
```

- [ ] **Step 5: Check de compilacao**

Run:

```powershell
cd D:\ragnarok\korangar
cargo check -p korangar
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
cd D:\ragnarok
git add korangar/korangar/src/graphics/instruction.rs korangar/korangar/src/graphics/passes/interface/rectangle.rs korangar/korangar/shaders/modules/interface.slang korangar/korangar/shaders/passes/interface/rectangle.slang korangar/korangar/src/renderer/interface.rs
git commit -m "Support rotated interface text"
```

---

### Task 6: Integrar Renderizacao Do Minimap No Frame

**Files:**
- Modify: `korangar/korangar/src/main.rs`
- Modify: `korangar/korangar/src/interface/minimap.rs`

- [ ] **Step 1: Adicionar helper para montar instrucao**

Em `korangar/korangar/src/interface/minimap.rs`, adicionar:

```rust
use std::sync::Arc;

use crate::graphics::{Color, MinimapInstruction, Texture};

pub fn build_minimap_instruction(
    texture: Arc<Texture>,
    area: MinimapArea,
    tile_position: TilePosition,
    map_width: u16,
    map_height: u16,
    zoom_tiles: f32,
    rotation_radians: f32,
) -> MinimapInstruction {
    MinimapInstruction {
        screen_position: area.position,
        screen_size: area.size,
        player_uv: player_uv(tile_position, map_width, map_height),
        visible_tiles: zoom_tiles,
        map_size_tiles: Vector2::new(map_width.max(1) as f32, map_height.max(1) as f32),
        rotation_radians,
        tint: Color::WHITE,
        texture,
    }
}
```

Adicionar teste:

```rust
#[test]
fn minimap_instruction_uses_player_uv_and_zoom() {
    let uv = player_uv(TilePosition { x: 10, y: 20 }, 100, 200);

    assert_relative_eq!(uv.x, 0.1);
    assert_relative_eq!(uv.y, 0.9);
}
```

- [ ] **Step 2: Carregar textura e criar instrucao em `main.rs`**

No ponto antes do literal `RenderInstruction { ... }`, adicionar:

```rust
let circular_minimap_enabled = *self
    .client_state
    .follow(client_state().interface_settings().circular_minimap_enabled());
let circular_minimap_rotation_enabled = *self
    .client_state
    .follow(client_state().interface_settings().circular_minimap_rotate_in_third_person());
let interface_scaling = self
    .client_state
    .follow(client_state().interface_settings().scaling())
    .get_factor();
let minimap_area = minimap_area(screen_size, interface_scaling);
let minimap_should_rotate = interface::minimap::should_rotate_minimap(circular_minimap_rotation_enabled, use_third_person_camera);
let minimap_rotation = match minimap_should_rotate {
    true => interface::minimap::minimap_rotation(self.third_person_camera.view_direction()).0,
    false => 0.0,
};
let minimap_instruction = circular_minimap_enabled
    .then(|| {
        let player = self.client_state.try_follow(this_entity())?;
        let texture_path = self.circular_minimap.map_texture_path()?;
        let texture = self.texture_loader.get_or_load(texture_path, ImageType::Color).ok()?;
        Some(interface::minimap::build_minimap_instruction(
            texture,
            minimap_area,
            player.get_tile_position(),
            map.width(),
            map.height(),
            self.circular_minimap.zoom_tiles(),
            minimap_rotation,
        ))
    })
    .flatten();
```

Adicionar `ImageType` ao import existente de loaders se necessario.

Se `map.width()` e `map.height()` nao existirem, adicionar funcoes simples em `korangar/korangar/src/world/map/mod.rs` que retornem as dimensoes ja armazenadas pelo `Map`. Nomes preferidos:

```rust
pub fn width(&self) -> u16
pub fn height(&self) -> u16
```

- [ ] **Step 3: Passar instrucao para `RenderInstruction`**

No literal `RenderInstruction { ... }`, depois de `interface: interface_instructions.as_slice(),`, adicionar:

```rust
minimap: minimap_instruction.as_ref(),
```

- [ ] **Step 4: Renderizar labels de bussola**

Antes de coletar `interface_instructions`, se `minimap_instruction.is_some()`, chamar `self.interface_renderer.render_text_rotated` quatro vezes. Usar offsets no espaco do disco:

```rust
if circular_minimap_enabled {
    let center = minimap_area.center();
    let radius = minimap_area.radius();
    let clip = ScreenClip::unbound();
    let color = Color::rgb_u8(240, 236, 220);
    let font_size = FontSize(12.0 * interface_scaling);
    let rotation = InterfaceRotation {
        center,
        angle_radians: minimap_rotation,
    };

    for (label, offset) in [
        ("NORTH", (0.0, -radius + 18.0 * interface_scaling)),
        ("EAST", (radius - 48.0 * interface_scaling, -6.0 * interface_scaling)),
        ("SOUTH", (0.0, radius - 28.0 * interface_scaling)),
        ("WEST", (-radius + 18.0 * interface_scaling, -6.0 * interface_scaling)),
    ] {
        self.interface_renderer.render_text_rotated(
            label,
            ScreenPosition {
                left: center.left + offset.0,
                top: center.top + offset.1,
            },
            80.0 * interface_scaling,
            clip,
            color,
            Color::WHITE,
            font_size,
            rotation,
        );
    }
}
```

Ajustar imports em `main.rs`:

```rust
use crate::graphics::InterfaceRotation;
use crate::loaders::FontSize;
```

- [ ] **Step 5: Desenhar marcador central**

Depois das labels, desenhar um circulo simples por enquanto como retangulo arredondado pequeno:

```rust
if circular_minimap_enabled {
    let center = minimap_area.center();
    self.interface_renderer.render_rectangle(
        ScreenPosition {
            left: center.left - 4.0 * interface_scaling,
            top: center.top - 4.0 * interface_scaling,
        },
        ScreenSize {
            width: 8.0 * interface_scaling,
            height: 8.0 * interface_scaling,
        },
        ScreenClip::unbound(),
        CornerDiameter(8.0 * interface_scaling),
        Color::rgb_u8(255, 245, 180),
        Color::TRANSPARENT,
        ShadowPadding::uniform(0.0),
    );
}
```

Usar os tipos de `CornerDiameter` e `ShadowPadding` ja exportados por `crate::graphics`.

- [ ] **Step 6: Rodar checks**

Run:

```powershell
cd D:\ragnarok\korangar
cargo test -p korangar interface::minimap --lib
cargo check -p korangar
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
cd D:\ragnarok
git add korangar/korangar/src/main.rs korangar/korangar/src/interface/minimap.rs korangar/korangar/src/world/map/mod.rs
git commit -m "Render circular minimap HUD"
```

---

### Task 7: Verificacao Final E Ajustes De Formato

**Files:**
- Modify as needed: somente arquivos tocados nas tarefas anteriores.

- [ ] **Step 1: Rodar formatacao**

```powershell
cd D:\ragnarok\korangar
cargo fmt --all
```

Expected: comando termina sem erro.

- [ ] **Step 2: Rodar testes unitarios focados**

```powershell
cd D:\ragnarok\korangar
cargo test -p korangar interface::minimap --lib
cargo test -p korangar settings::interface::tests::old_interface_settings_files_load_with_cinematic_defaults --lib
```

Expected: PASS.

- [ ] **Step 3: Rodar check completo do crate principal**

```powershell
cd D:\ragnarok\korangar
cargo check -p korangar
```

Expected: PASS.

- [ ] **Step 4: Verificacao manual no cliente**

Usar o ambiente local e confirmar:

- `Circular minimap` desligado: nenhum minimapa aparece.
- `Circular minimap` ligado em Prontera ou Geffen: minimapa aparece no canto superior direito.
- Andar pelo mapa: marcador central permanece fixo e mapa desliza.
- Scroll sobre o minimapa: zoom muda.
- Scroll fora do minimapa: camera ou UI continua com comportamento normal.
- Fora da terceira pessoa: disco fica norte-fixo.
- Em terceira pessoa com rotacao ligada: disco gira conforme direcao visual da camera.
- Em terceira pessoa com rotacao desligada: disco fica norte-fixo.
- Mapa sem textura: cliente nao fecha inesperadamente e HUD fica discreto.

- [ ] **Step 5: Commit final de ajustes**

Se `cargo fmt` ou a verificacao manual exigirem ajustes, commitar:

```powershell
cd D:\ragnarok
git add korangar/korangar/src docs/superpowers/plans/2026-05-01-circular-minimap.md
git commit -m "Polish circular minimap integration"
```

Se nao houver ajustes, nao criar commit vazio.

---

## Self-Review Do Plano

**Cobertura da spec:**

- Configs de ligar/desligar e rotacao: Task 2.
- HUD no canto superior direito: Task 1 e Task 6.
- Radar com jogador fixo no centro: Task 4 e Task 6.
- Zoom por scroll dentro do circulo: Task 1 e Task 3.
- Rotacao somente em terceira pessoa: Task 1 e Task 6.
- Textos `NORTH`, `SOUTH`, `EAST`, `WEST` girando com o disco: Task 5 e Task 6.
- Fallback para textura ausente: Task 6 usa `ok()?` e nao cria instrucao quando a textura nao carrega.
- Testes unitarios e verificacao manual: Task 1, Task 2 e Task 7.

**Sem lacunas intencionais:**

O plano evita campos indefinidos e fornece comandos, arquivos e snippets concretos. Quando uma API local pode ter nome diferente, o plano fornece a alternativa concreta a aplicar.

**Consistencia de tipos:**

O plano usa `CircularMinimapState`, `MinimapArea`, `MinimapInstruction`, `InterfaceRotation` e `ZoomCircularMinimap` com nomes consistentes entre as tarefas.
