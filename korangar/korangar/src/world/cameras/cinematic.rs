use cgmath::{Deg, InnerSpace, Matrix4, Point3, Vector2, Vector3, Zero};
use ragnarok_packets::Direction;

use super::{Camera, SmoothedValue};
use crate::graphics::perspective_reverse_lh;

const THRESHOLD: f32 = 0.01;
const CAMERA_HEIGHT: f32 = 95.0;
const CAMERA_DISTANCE: f32 = 230.0;
const RIGHT_OFFSET: f32 = 55.0;
const FOCUS_HEIGHT: f32 = 55.0;
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

    pub fn set_dynamic_targets(&mut self, player_position: Point3<f32>, npc_position: Point3<f32>, player_direction: Direction) {
        let (camera_position, focus_point) = Self::calculate_targets(player_position, npc_position, player_direction);
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
        let camera_position = player_position - forward * CAMERA_DISTANCE + right * RIGHT_OFFSET + Vector3::new(0.0, CAMERA_HEIGHT, 0.0);

        (camera_position, focus_point)
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
        let (target_camera_position, _) = CinematicCamera::calculate_targets(player_position, npc_position, Direction::North);

        camera.set_immediate_to(initial_camera_position, initial_focus_point);
        camera.set_dynamic_targets(player_position, npc_position, Direction::North);

        assert_relative_eq!(camera.camera_position(), initial_camera_position, epsilon = 1e-6);

        camera.update(1.0 / 60.0);

        assert!(camera.camera_position().distance(target_camera_position) < initial_camera_position.distance(target_camera_position));
    }

    #[test]
    fn camera_uses_player_direction_for_behind_offset() {
        let player_position = Point3::new(100.0, 0.0, 100.0);
        let npc_position = Point3::new(100.0, 0.0, 200.0);

        let (north_camera_position, _) = CinematicCamera::calculate_targets(player_position, npc_position, Direction::North);
        let (east_camera_position, _) = CinematicCamera::calculate_targets(player_position, npc_position, Direction::East);

        assert!(north_camera_position.z < player_position.z);
        assert!(east_camera_position.x < player_position.x);
    }
}
