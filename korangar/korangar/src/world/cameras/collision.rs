use cgmath::{InnerSpace, MetricSpace, Point3, Vector3};
use ragnarok_formats::map::TileFlags;

use crate::loaders::GAT_TILE_SIZE;
use crate::world::Map;

pub(super) fn required_ground_contact_offset(desired_camera_y: f32, ground_height: Option<f32>, clearance: f32) -> f32 {
    ground_height
        .map(|ground_height| (ground_height + clearance - desired_camera_y).max(0.0))
        .unwrap_or(0.0)
}

pub(super) fn resolve_camera_collision(
    map: &Map,
    anchor_position: Point3<f32>,
    desired_camera_position: Point3<f32>,
    step_length: f32,
    padding: f32,
    fallback_offset: Vector3<f32>,
) -> Point3<f32> {
    let object_intersection_fraction = map.first_object_intersection_fraction(anchor_position, desired_camera_position, padding);

    resolve_camera_collision_with_clearance(
        anchor_position,
        desired_camera_position,
        object_intersection_fraction,
        |position| is_camera_position_clear(map, position),
        step_length,
        padding,
        fallback_offset,
    )
}

pub(super) fn resolve_camera_collision_with_clearance(
    anchor_position: Point3<f32>,
    desired_camera_position: Point3<f32>,
    object_intersection_fraction: Option<f32>,
    is_position_clear: impl Fn(Point3<f32>) -> bool,
    step_length: f32,
    padding: f32,
    fallback_offset: Vector3<f32>,
) -> Point3<f32> {
    let target_vector = desired_camera_position - anchor_position;
    let target_distance = target_vector.magnitude();

    if target_distance <= f32::EPSILON {
        return desired_camera_position;
    }

    let target_direction = target_vector / target_distance;
    let adjusted_target_distance = object_intersection_fraction
        .map(|fraction| (target_distance * fraction - padding).max(0.0))
        .unwrap_or(target_distance);
    let steps = (adjusted_target_distance / step_length).ceil().max(1.0) as usize;
    let mut last_clear_position = None;

    for step in 1..=steps {
        let distance = (step as f32 * step_length).min(adjusted_target_distance);
        let sample_position = anchor_position + target_direction * distance;

        if is_position_clear(sample_position) {
            last_clear_position = Some(sample_position);
            continue;
        }

        return last_clear_position
            .map(|position| {
                let padded_distance = (anchor_position.distance(position) - padding).max(0.0);
                anchor_position + target_direction * padded_distance
            })
            .unwrap_or(anchor_position + fallback_offset);
    }

    anchor_position + target_direction * adjusted_target_distance
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
