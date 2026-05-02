use std::sync::Arc;

use cgmath::{InnerSpace, Rad, Vector2, Vector3};
use ragnarok_packets::TilePosition;

use crate::graphics::{Color, MinimapInstruction, ScreenPosition, ScreenSize, Texture};

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

struct MinimapInstructionValues {
    screen_position: ScreenPosition,
    screen_size: ScreenSize,
    player_uv: Vector2<f32>,
    visible_tiles: f32,
    map_size_tiles: Vector2<f32>,
    rotation_radians: f32,
    tint: Color,
}

fn minimap_instruction_values(
    area: MinimapArea,
    window_size: ScreenSize,
    tile_position: TilePosition,
    map_width: u16,
    map_height: u16,
    zoom_tiles: f32,
    rotation_radians: f32,
) -> MinimapInstructionValues {
    MinimapInstructionValues {
        screen_position: area.position / window_size,
        screen_size: area.size / window_size,
        player_uv: player_uv(tile_position, map_width, map_height),
        visible_tiles: zoom_tiles,
        map_size_tiles: Vector2::new(map_width as f32, map_height as f32),
        rotation_radians,
        tint: Color::WHITE,
    }
}

pub fn build_minimap_instruction(
    texture: Arc<Texture>,
    area: MinimapArea,
    window_size: ScreenSize,
    tile_position: TilePosition,
    map_width: u16,
    map_height: u16,
    zoom_tiles: f32,
    rotation_radians: f32,
) -> MinimapInstruction {
    let values = minimap_instruction_values(
        area,
        window_size,
        tile_position,
        map_width,
        map_height,
        zoom_tiles,
        rotation_radians,
    );

    MinimapInstruction {
        screen_position: values.screen_position,
        screen_size: values.screen_size,
        player_uv: values.player_uv,
        visible_tiles: values.visible_tiles,
        map_size_tiles: values.map_size_tiles,
        rotation_radians: values.rotation_radians,
        tint: values.tint,
        texture,
    }
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

    use super::*;

    fn assert_approx_eq(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.0001,
            "expected {actual} to be approximately {expected}"
        );
    }

    #[test]
    fn minimap_area_is_top_right() {
        let area = minimap_area(
            ScreenSize {
                width: 1280.0,
                height: 720.0,
            },
            1.0,
        );

        assert_eq!(area, MinimapArea {
            position: ScreenPosition { left: 1084.0, top: 16.0 },
            size: ScreenSize {
                width: 180.0,
                height: 180.0
            }
        });
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

        assert!(is_inside_minimap(area, ScreenPosition { left: 190.0, top: 110.0 }));
        assert!(is_inside_minimap(area, ScreenPosition { left: 280.0, top: 110.0 }));
        assert!(!is_inside_minimap(area, ScreenPosition { left: 281.0, top: 110.0 }));
    }

    #[test]
    fn zoom_is_clamped() {
        assert_eq!(clamp_zoom(1.0), MIN_ZOOM_TILES);
        assert_eq!(clamp_zoom(48.0), 48.0);
        assert_eq!(clamp_zoom(200.0), MAX_ZOOM_TILES);
    }

    #[test]
    fn scroll_changes_zoom_with_limits() {
        let mut state = CircularMinimapState::default();

        state.apply_scroll(100.0);
        assert_approx_eq(state.zoom_tiles(), 33.6);

        state.apply_scroll(10_000.0);
        assert_eq!(state.zoom_tiles(), MIN_ZOOM_TILES);

        state.apply_scroll(-10_000.0);
        assert_eq!(state.zoom_tiles(), MAX_ZOOM_TILES);
    }

    #[test]
    fn player_tile_maps_to_uv_with_inverted_y_axis() {
        let uv = player_uv(TilePosition { x: 25, y: 75 }, 100, 100);

        assert_approx_eq(uv.x, 0.25);
        assert_approx_eq(uv.y, 0.25);
    }

    #[test]
    fn rotation_requires_config_and_third_person() {
        assert!(should_rotate_minimap(true, true));
        assert!(!should_rotate_minimap(true, false));
        assert!(!should_rotate_minimap(false, true));
        assert!(!should_rotate_minimap(false, false));
    }

    #[test]
    fn camera_direction_maps_to_rotation_angle() {
        assert_approx_eq(minimap_rotation(Vector3::new(0.0, 1.0, 0.0)).0, 0.0);
        assert_approx_eq(minimap_rotation(Vector3::new(0.0, 0.0, 1.0)).0, 0.0);
        assert_approx_eq(minimap_rotation(Vector3::new(1.0, 0.0, 0.0)).0, FRAC_PI_2);
    }

    #[test]
    fn map_texture_path_uses_map_base_name() {
        let mut state = CircularMinimapState::default();

        state.set_map_name("data\\prontera.gat");

        assert_eq!(
            map_texture_path("data\\prontera.gat"),
            "\u{c720}\u{c800}\u{c778}\u{d130}\u{d398}\u{c774}\u{c2a4}\\map\\prontera.bmp"
        );
        assert_eq!(
            state.map_texture_path(),
            Some("\u{c720}\u{c800}\u{c778}\u{d130}\u{d398}\u{c774}\u{c2a4}\\map\\prontera.bmp")
        );
    }

    #[test]
    fn minimap_instruction_values_use_area_player_map_and_rotation() {
        let area = MinimapArea {
            position: ScreenPosition { left: 10.0, top: 20.0 },
            size: ScreenSize {
                width: 180.0,
                height: 180.0,
            },
        };
        let window_size = ScreenSize {
            width: 1000.0,
            height: 500.0,
        };

        let values = minimap_instruction_values(area, window_size, TilePosition { x: 25, y: 75 }, 100, 200, 42.0, 1.25);

        assert_eq!(values.screen_position, ScreenPosition { left: 0.01, top: 0.04 });
        assert_eq!(values.screen_size, ScreenSize { width: 0.18, height: 0.36 });
        assert_approx_eq(values.player_uv.x, 0.25);
        assert_approx_eq(values.player_uv.y, 0.625);
        assert_eq!(values.visible_tiles, 42.0);
        assert_eq!(values.map_size_tiles, Vector2::new(100.0, 200.0));
        assert_eq!(values.rotation_radians, 1.25);
        assert_eq!(values.tint, Color::WHITE);
    }
}
