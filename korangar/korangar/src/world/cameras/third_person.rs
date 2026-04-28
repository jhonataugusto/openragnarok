use cgmath::{Array, Deg, InnerSpace, Matrix4, Point3, Quaternion, Rad, Rotation, Rotation3, Vector2, Vector3, Zero};

use super::{Camera, SmoothedValue};
use crate::graphics::perspective_reverse_lh;
use crate::loaders::GAT_TILE_SIZE;

const ZOOM_SPEED: f32 = 1.0;
const LOOK_AROUND_SPEED: f32 = 0.005;
const MINIMUM_DISTANCE: f32 = GAT_TILE_SIZE;
const MAXIMUM_DISTANCE: f32 = GAT_TILE_SIZE * 5.0;
const DEFAULT_DISTANCE: f32 = GAT_TILE_SIZE * 3.0;
const MINIMUM_PITCH: f32 = -65_f32.to_radians();
const MAXIMUM_PITCH: f32 = 65_f32.to_radians();
const HEIGHT_OFFSET: f32 = GAT_TILE_SIZE;
const VERTICAL_FOV: Deg<f32> = Deg(45.0);
const THRESHOLD: f32 = 0.01;
const LOOK_UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

pub struct ThirdPersonCamera {
    focus_point: Point3<SmoothedValue>,
    camera_position: Point3<f32>,
    orientation: Quaternion<f32>,
    yaw: f32,
    pitch: f32,
    camera_distance: SmoothedValue,
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
    view_projection_matrix: Matrix4<f32>,
}

impl ThirdPersonCamera {
    pub fn new() -> Self {
        Self {
            focus_point: [SmoothedValue::new(0.0, THRESHOLD, 3.0); 3].into(),
            camera_position: Point3::from_value(0.0),
            orientation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            camera_distance: SmoothedValue::new(DEFAULT_DISTANCE, THRESHOLD, 4.0),
            view_matrix: Matrix4::zero(),
            projection_matrix: Matrix4::zero(),
            view_projection_matrix: Matrix4::zero(),
        }
    }

    pub fn set_focus_point(&mut self, position: Point3<f32>) {
        self.focus_point.x.set(position.x);
        self.focus_point.y.set(position.y);
        self.focus_point.z.set(position.z);
    }

    pub fn set_smoothed_focus_point(&mut self, position: Point3<f32>) {
        self.focus_point.x.set_desired(position.x);
        self.focus_point.y.set_desired(position.y);
        self.focus_point.z.set_desired(position.z);
    }

    pub fn soft_zoom(&mut self, zoom_factor: f32) {
        self.camera_distance
            .move_desired_clamp(zoom_factor * ZOOM_SPEED, MINIMUM_DISTANCE, MAXIMUM_DISTANCE);
    }

    pub fn look_around(&mut self, yaw: f32, pitch: f32) {
        self.yaw += yaw * LOOK_AROUND_SPEED;
        self.pitch = (self.pitch + pitch * LOOK_AROUND_SPEED).clamp(MINIMUM_PITCH, MAXIMUM_PITCH);
        self.update_orientation();
    }

    fn update_orientation(&mut self) {
        let yaw = Quaternion::from_axis_angle(Vector3::unit_y(), Rad(self.yaw));
        let pitch = Quaternion::from_axis_angle(Vector3::unit_x(), Rad(self.pitch));
        self.orientation = (yaw * pitch).normalize();
    }

    pub fn update(&mut self, delta_time: f64) {
        self.focus_point.x.update(delta_time);
        self.focus_point.y.update(delta_time);
        self.focus_point.z.update(delta_time);
        self.camera_distance.update(delta_time);

        let view_distance = self.camera_distance.get_current();
        self.camera_position = self.focus_point() + Vector3::unit_y() * HEIGHT_OFFSET - self.view_direction() * view_distance;
    }
}

impl Camera for ThirdPersonCamera {
    fn camera_position(&self) -> Point3<f32> {
        self.camera_position
    }

    fn focus_point(&self) -> Point3<f32> {
        self.focus_point.map(|component| component.get_current())
    }

    fn generate_view_projection(&mut self, window_size: Vector2<usize>) {
        let aspect_ratio = window_size.x as f32 / window_size.y as f32;
        self.view_matrix = Matrix4::look_to_lh(self.camera_position, self.view_direction(), LOOK_UP);
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
        self.orientation.rotate_vector(Vector3::unit_z())
    }
}

#[cfg(test)]
mod tests {
    use cgmath::{InnerSpace, Point3, Vector2, Vector3, assert_relative_eq};

    use super::*;

    #[test]
    fn default_orientation_matches_debug_camera_forward() {
        let mut camera = ThirdPersonCamera::new();
        camera.set_focus_point(Point3::new(0.0, 0.0, 0.0));
        camera.update(1.0 / 60.0);

        assert_relative_eq!(camera.view_direction(), Vector3::unit_z(), epsilon = 1e-6);
        assert!(camera.camera_position().z < 0.0);
    }

    #[test]
    fn camera_starts_one_tile_above_focus_point() {
        let focus_point = Point3::new(20.0, 7.0, 30.0);
        let mut camera = ThirdPersonCamera::new();
        camera.set_focus_point(focus_point);
        camera.update(1.0 / 60.0);

        assert_relative_eq!(camera.camera_position().y, focus_point.y + GAT_TILE_SIZE, epsilon = 1e-6);
    }

    #[test]
    fn default_distance_starts_close_to_player() {
        let focus_point = Point3::new(0.0, 0.0, 0.0);
        let mut camera = ThirdPersonCamera::new();
        camera.set_focus_point(focus_point);
        camera.update(1.0 / 60.0);
        let camera_offset = camera.camera_position() - focus_point;

        assert_relative_eq!(
            Vector2::new(camera_offset.x, camera_offset.z).magnitude(),
            GAT_TILE_SIZE * 3.0,
            epsilon = 1e-6
        );
    }

    #[test]
    fn camera_distance_clamps_between_one_and_five_tiles() {
        let focus_point = Point3::new(0.0, 0.0, 0.0);
        let mut camera = ThirdPersonCamera::new();
        camera.set_focus_point(focus_point);

        camera.soft_zoom(-1000.0);
        camera.update(1.0);
        let camera_offset = camera.camera_position() - focus_point;
        assert_relative_eq!(
            Vector2::new(camera_offset.x, camera_offset.z).magnitude(),
            GAT_TILE_SIZE,
            epsilon = 1e-6
        );

        camera.soft_zoom(1000.0);
        camera.update(1.0);
        let camera_offset = camera.camera_position() - focus_point;
        assert_relative_eq!(
            Vector2::new(camera_offset.x, camera_offset.z).magnitude(),
            GAT_TILE_SIZE * 5.0,
            epsilon = 1e-6
        );
    }

    #[test]
    fn positive_yaw_rotates_towards_positive_x() {
        let mut camera = ThirdPersonCamera::new();
        camera.look_around(100.0, 0.0);

        assert!(camera.view_direction().x > 0.0);
    }

    #[test]
    fn pitch_is_clamped_around_debug_camera_forward() {
        let mut camera = ThirdPersonCamera::new();
        camera.look_around(0.0, 10000.0);
        assert!(camera.view_direction().y.abs() <= MAXIMUM_PITCH.sin() + 0.0001);

        camera.look_around(0.0, -20000.0);
        assert!(camera.view_direction().y.abs() <= MAXIMUM_PITCH.sin() + 0.0001);
    }
}
