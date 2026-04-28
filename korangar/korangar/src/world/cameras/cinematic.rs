use cgmath::{Deg, InnerSpace, Matrix4, MetricSpace, Point3, Vector2, Vector3, Zero};
use ragnarok_formats::map::TileFlags;
use ragnarok_packets::Direction;

use super::{Camera, SmoothedValue};
use crate::graphics::perspective_reverse_lh;
use crate::loaders::GAT_TILE_SIZE;
use crate::world::Map;

const THRESHOLD: f32 = 0.01;
const CAMERA_HEIGHT: f32 = 18.0;
const CAMERA_DISTANCE: f32 = 230.0;
const RIGHT_OFFSET: f32 = 55.0;
const FOCUS_HEIGHT: f32 = 12.0;
const CAMERA_COLLISION_STEP_LENGTH: f32 = GAT_TILE_SIZE * 0.5;
const CAMERA_COLLISION_PADDING: f32 = GAT_TILE_SIZE * 1.5;
const VERTICAL_FOV: Deg<f32> = Deg(18.0);
const LOOK_UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

pub struct CinematicCamera {
    focus_point: Point3<SmoothedValue>,
    camera_position: Point3<SmoothedValue>,
    view_direction: Vector3<f32>,
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
    view_projection_matrix: Matrix4<f32>,
}

impl CinematicCamera {
    pub fn new() -> Self {
        Self {
            focus_point: [SmoothedValue::new(0.0, THRESHOLD, 8.0); 3].into(),
            camera_position: [SmoothedValue::new(0.0, THRESHOLD, 8.0); 3].into(),
            view_direction: Vector3::unit_z(),
            view_matrix: Matrix4::zero(),
            projection_matrix: Matrix4::zero(),
            view_projection_matrix: Matrix4::zero(),
        }
    }

    pub fn set_immediate_to(&mut self, camera_position: Point3<f32>, focus_point: Point3<f32>) {
        self.camera_position.x.set(camera_position.x);
        self.camera_position.y.set(camera_position.y);
        self.camera_position.z.set(camera_position.z);
        self.focus_point.x.set(focus_point.x);
        self.focus_point.y.set(focus_point.y);
        self.focus_point.z.set(focus_point.z);
        self.update_view_direction();
    }

    pub fn set_dynamic_targets(
        &mut self,
        map: Option<&Map>,
        player_position: Point3<f32>,
        npc_position: Point3<f32>,
        player_direction: Direction,
    ) {
        let (camera_position, focus_point) = Self::calculate_targets(map, player_position, npc_position, player_direction);
        self.camera_position.x.set_desired(camera_position.x);
        self.camera_position.y.set_desired(camera_position.y);
        self.camera_position.z.set_desired(camera_position.z);
        self.focus_point.x.set_desired(focus_point.x);
        self.focus_point.y.set_desired(focus_point.y);
        self.focus_point.z.set_desired(focus_point.z);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.camera_position.x.update(delta_time);
        self.camera_position.y.update(delta_time);
        self.camera_position.z.update(delta_time);
        self.focus_point.x.update(delta_time);
        self.focus_point.y.update(delta_time);
        self.focus_point.z.update(delta_time);
        self.update_view_direction();
    }

    fn calculate_targets(
        map: Option<&Map>,
        player_position: Point3<f32>,
        npc_position: Point3<f32>,
        player_direction: Direction,
    ) -> (Point3<f32>, Point3<f32>) {
        let forward = direction_vector(player_direction);
        let right = Vector3::new(forward.z, 0.0, -forward.x).normalize();
        let focus_point = Point3::new(
            (player_position.x + npc_position.x) * 0.5,
            (player_position.y + npc_position.y) * 0.5 + FOCUS_HEIGHT,
            (player_position.z + npc_position.z) * 0.5,
        );
        let desired_camera_position =
            player_position - forward * CAMERA_DISTANCE - right * RIGHT_OFFSET + Vector3::new(0.0, CAMERA_HEIGHT, 0.0);
        let camera_position = map
            .map(|map| Self::resolve_camera_collision(map, player_position, desired_camera_position))
            .unwrap_or(desired_camera_position);

        (camera_position, focus_point)
    }

    fn resolve_camera_collision(map: &Map, player_position: Point3<f32>, desired_camera_position: Point3<f32>) -> Point3<f32> {
        let target_vector = desired_camera_position - player_position;
        let target_distance = target_vector.magnitude();

        if target_distance <= f32::EPSILON {
            return desired_camera_position;
        }

        let target_direction = target_vector / target_distance;
        let adjusted_target_distance = map
            .first_object_intersection_fraction(player_position, desired_camera_position, CAMERA_COLLISION_PADDING)
            .map(|fraction| (target_distance * fraction - CAMERA_COLLISION_PADDING).max(0.0))
            .unwrap_or(target_distance);
        let steps = (adjusted_target_distance / CAMERA_COLLISION_STEP_LENGTH).ceil().max(1.0) as usize;
        let mut last_clear_position = None;

        for step in 1..=steps {
            let distance = (step as f32 * CAMERA_COLLISION_STEP_LENGTH).min(adjusted_target_distance);
            let sample_position = player_position + target_direction * distance;

            if Self::is_camera_position_clear(map, sample_position) {
                last_clear_position = Some(sample_position);
                continue;
            }

            return last_clear_position
                .map(|position| {
                    let padded_distance = (player_position.distance(position) - CAMERA_COLLISION_PADDING).max(0.0);
                    player_position + target_direction * padded_distance
                })
                .unwrap_or(player_position + Vector3::new(0.0, CAMERA_HEIGHT, 0.0));
        }

        player_position + target_direction * adjusted_target_distance
    }

    fn is_camera_position_clear(map: &Map, position: Point3<f32>) -> bool {
        if position.x < 0.0 || position.z < 0.0 {
            return false;
        }

        let tile_position = ragnarok_packets::TilePosition {
            x: (position.x / GAT_TILE_SIZE).floor() as u16,
            y: (position.z / GAT_TILE_SIZE).floor() as u16,
        };

        map.get_tile(tile_position)
            .map(|tile| tile.flags.contains(TileFlags::WALKABLE) && tile.flags.contains(TileFlags::SNIPABLE))
            .unwrap_or(false)
    }

    fn update_view_direction(&mut self) {
        self.view_direction = (self.focus_point() - self.camera_position()).normalize();
    }
}

impl Camera for CinematicCamera {
    fn camera_position(&self) -> Point3<f32> {
        self.camera_position.map(|component| component.get_current())
    }

    fn focus_point(&self) -> Point3<f32> {
        self.focus_point.map(|component| component.get_current())
    }

    fn generate_view_projection(&mut self, window_size: Vector2<usize>) {
        let aspect_ratio = window_size.x as f32 / window_size.y as f32;
        self.view_matrix = Matrix4::look_to_lh(self.camera_position(), self.view_direction, LOOK_UP);
        self.projection_matrix = perspective_reverse_lh(VERTICAL_FOV, aspect_ratio);
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
    }

    fn look_up_vector(&self) -> Vector3<f32> {
        LOOK_UP
    }

    fn view_projection_matrices(&self) -> (Matrix4<f32>, Matrix4<f32>) {
        (self.view_matrix, self.projection_matrix)
    }

    fn view_projection_matrix(&self) -> Matrix4<f32> {
        self.view_projection_matrix
    }

    fn view_direction(&self) -> Vector3<f32> {
        self.view_direction
    }
}

fn direction_vector(direction: Direction) -> Vector3<f32> {
    match direction {
        Direction::North => Vector3::new(0.0, 0.0, 1.0),
        Direction::NorthEast => Vector3::new(1.0, 0.0, 1.0).normalize(),
        Direction::East => Vector3::new(1.0, 0.0, 0.0),
        Direction::SouthEast => Vector3::new(1.0, 0.0, -1.0).normalize(),
        Direction::South => Vector3::new(0.0, 0.0, -1.0),
        Direction::SouthWest => Vector3::new(-1.0, 0.0, -1.0).normalize(),
        Direction::West => Vector3::new(-1.0, 0.0, 0.0),
        Direction::NorthWest => Vector3::new(-1.0, 0.0, 1.0).normalize(),
    }
}

#[cfg(test)]
mod tests {
    use cgmath::{MetricSpace, assert_relative_eq};

    use super::*;

    #[test]
    fn camera_smooths_from_seeded_runtime_camera() {
        let mut camera = CinematicCamera::new();
        let initial_camera_position = Point3::new(10.0, 20.0, 30.0);
        let initial_focus_point = Point3::new(5.0, 10.0, 15.0);
        let player_position = Point3::new(100.0, 0.0, 100.0);
        let npc_position = Point3::new(120.0, 0.0, 160.0);
        let (target_camera_position, _) = CinematicCamera::calculate_targets(None, player_position, npc_position, Direction::North);

        camera.set_immediate_to(initial_camera_position, initial_focus_point);
        camera.set_dynamic_targets(None, player_position, npc_position, Direction::North);

        assert_relative_eq!(camera.camera_position(), initial_camera_position, epsilon = 1e-6);

        camera.update(1.0 / 60.0);

        assert!(camera.camera_position().distance(target_camera_position) < initial_camera_position.distance(target_camera_position));
    }

    #[test]
    fn camera_uses_player_direction_for_behind_offset() {
        let player_position = Point3::new(100.0, 0.0, 100.0);
        let npc_position = Point3::new(100.0, 0.0, 200.0);

        let (north_camera_position, _) = CinematicCamera::calculate_targets(None, player_position, npc_position, Direction::North);
        let (east_camera_position, _) = CinematicCamera::calculate_targets(None, player_position, npc_position, Direction::East);

        assert!(north_camera_position.z < player_position.z);
        assert!(east_camera_position.x < player_position.x);
        assert!(north_camera_position.x < player_position.x);
    }

    #[test]
    fn camera_stays_near_entity_height() {
        let player_position = Point3::new(100.0, 0.0, 100.0);
        let npc_position = Point3::new(120.0, 0.0, 160.0);

        let (camera_position, focus_point) = CinematicCamera::calculate_targets(None, player_position, npc_position, Direction::North);

        assert_relative_eq!(camera_position.y, player_position.y + CAMERA_HEIGHT, epsilon = 1e-6);
        assert_relative_eq!(
            focus_point.y,
            (player_position.y + npc_position.y) * 0.5 + FOCUS_HEIGHT,
            epsilon = 1e-6
        );
        assert!(camera_position.y < 25.0);
        assert!(focus_point.y < camera_position.y);
    }
}
