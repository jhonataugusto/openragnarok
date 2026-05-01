use std::sync::Arc;

use cgmath::{Array, Matrix4, Point3, Transform, Vector2, Vector3, Zero};
use korangar_container::Cacheable;
use korangar_interface::element::StateElement;
use ragnarok_packets::{ClientTick, Direction, EntityId};
use rust_state::RustState;

use super::weapon_fallback::{WeaponAttackAction, weapon_fallback};
#[cfg(feature = "debug")]
use crate::graphics::DebugRectangleInstruction;
use crate::graphics::{Color, EntityInstruction};
use crate::loaders::Sprite;
use crate::world::{ActionEvent, Actions, Camera, EntityType};

const TILE_SIZE: f32 = 10.0;
const SPRITE_SCALE: f32 = 1.4;
const PICKUP_ANIMATION_FACTOR: f32 = 25.0;

#[allow(dead_code)]
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum AnimationActionType {
    Attack1,
    Attack2,
    Attack3,
    Die,
    Freeze1,
    Freeze2,
    Hurt,
    #[default]
    Idle,
    Pickup,
    ReadyFight,
    Sit,
    Skill,
    Special,
    Walk,
}

impl AnimationActionType {
    pub fn action_base_offset(&self, entity_type: EntityType) -> usize {
        match entity_type {
            EntityType::Hidden | EntityType::Player => match self {
                AnimationActionType::Idle => 0,
                AnimationActionType::Walk => 1,
                AnimationActionType::Sit => 2,
                AnimationActionType::Pickup => 3,
                AnimationActionType::ReadyFight => 4,
                AnimationActionType::Attack1 => 5,
                AnimationActionType::Hurt => 6,
                AnimationActionType::Freeze1 => 7,
                AnimationActionType::Die => 8,
                AnimationActionType::Freeze2 => 9,
                AnimationActionType::Attack2 => 10,
                AnimationActionType::Attack3 => 11,
                AnimationActionType::Skill => 12,
                _ => 0,
            },
            EntityType::Npc | EntityType::Monster => match self {
                AnimationActionType::Idle => 0,
                AnimationActionType::Walk => 1,
                AnimationActionType::Attack1 => 2,
                AnimationActionType::Hurt => 3,
                AnimationActionType::Die => 4,
                _ => 0,
            },
            EntityType::Warp => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attack_duration_is_never_zero() {
        let mut animation_state = AnimationState::new(EntityType::Player, ClientTick(0));

        animation_state.attack(EntityType::Player, 0, 0, false, ClientTick(10));

        assert_eq!(animation_state.duration, Some(1));
    }

    #[test]
    fn attack_can_recover_to_ready_fight() {
        let mut animation_state = AnimationState::new(EntityType::Player, ClientTick(0));

        animation_state.attack_with_recovery(EntityType::Player, 0, 150, false, true, ClientTick(10));
        animation_state.recover_finished_action(EntityType::Player, ClientTick(160));

        assert_eq!(animation_state.action_type, AnimationActionType::ReadyFight);
        assert!(animation_state.looping);
    }

    #[test]
    fn attack_recovers_to_idle_by_default() {
        let mut animation_state = AnimationState::new(EntityType::Player, ClientTick(0));

        animation_state.attack(EntityType::Player, 0, 150, false, ClientTick(10));
        animation_state.recover_finished_action(EntityType::Player, ClientTick(160));

        assert_eq!(animation_state.action_type, AnimationActionType::Idle);
        assert!(animation_state.looping);
    }

    #[test]
    fn armed_player_attack_uses_weapon_action() {
        let mut animation_state = AnimationState::new(EntityType::Player, ClientTick(0));

        animation_state.attack(EntityType::Player, 1, 150, false, ClientTick(10));

        assert_eq!(animation_state.action_type, AnimationActionType::Attack3);
        assert_eq!(animation_state.action_base_offset, 11);
    }

    #[test]
    fn sword_player_attack_uses_short_weapon_action() {
        let mut animation_state = AnimationState::new(EntityType::Player, ClientTick(0));

        animation_state.attack(EntityType::Player, 1101, 150, false, ClientTick(10));

        assert_eq!(animation_state.action_type, AnimationActionType::Attack2);
        assert_eq!(animation_state.action_base_offset, 10);
    }

    #[test]
    fn spear_player_attack_uses_long_weapon_action() {
        let mut animation_state = AnimationState::new(EntityType::Player, ClientTick(0));

        animation_state.attack(EntityType::Player, 1401, 150, false, ClientTick(10));

        assert_eq!(animation_state.action_type, AnimationActionType::Attack3);
        assert_eq!(animation_state.action_base_offset, 11);
    }

    #[test]
    fn bow_player_attack_uses_short_weapon_action() {
        let mut animation_state = AnimationState::new(EntityType::Player, ClientTick(0));

        animation_state.attack(EntityType::Player, 1701, 150, false, ClientTick(10));

        assert_eq!(animation_state.action_type, AnimationActionType::Attack2);
        assert_eq!(animation_state.action_base_offset, 10);
    }

    #[test]
    fn unarmed_player_attack_uses_unarmed_action() {
        let mut animation_state = AnimationState::new(EntityType::Player, ClientTick(0));

        animation_state.attack(EntityType::Player, 0, 150, false, ClientTick(10));

        assert_eq!(animation_state.action_type, AnimationActionType::Attack1);
        assert_eq!(animation_state.action_base_offset, 5);
    }

    #[test]
    fn monster_critical_attack_keeps_monster_attack_action() {
        let mut animation_state = AnimationState::new(EntityType::Monster, ClientTick(0));

        animation_state.attack(EntityType::Monster, 0, 150, true, ClientTick(10));

        assert_eq!(animation_state.action_type, AnimationActionType::Attack1);
        assert_eq!(animation_state.action_base_offset, 2);
    }

    #[test]
    fn hurt_uses_hurt_action() {
        let mut animation_state = AnimationState::new(EntityType::Player, ClientTick(0));

        animation_state.hurt(EntityType::Player, 200, ClientTick(10));

        assert_eq!(animation_state.action_type, AnimationActionType::Hurt);
        assert_eq!(
            animation_state.action_base_offset,
            AnimationActionType::Hurt.action_base_offset(EntityType::Player)
        );
        assert_eq!(animation_state.duration, Some(200));
        assert!(!animation_state.looping);
    }

    #[test]
    fn hurt_recovers_to_idle() {
        let mut animation_state = AnimationState::new(EntityType::Player, ClientTick(0));

        animation_state.hurt(EntityType::Player, 150, ClientTick(10));
        animation_state.recover_finished_action(EntityType::Player, ClientTick(160));

        assert_eq!(animation_state.action_type, AnimationActionType::Idle);
        assert!(animation_state.looping);
    }

    #[test]
    fn hurt_is_transient_action() {
        let mut animation_state = AnimationState::new(EntityType::Player, ClientTick(0));

        animation_state.hurt(EntityType::Player, 150, ClientTick(10));

        assert!(animation_state.is_hurt());
    }
}

#[derive(Clone, Debug)]
pub struct AnimationState {
    action_type: AnimationActionType,
    action_base_offset: usize,
    start_time: ClientTick,
    time: u32,
    duration: Option<u32>,
    factor: Option<f32>,
    looping: bool,
    recover_to_ready_fight: bool,
}

impl AnimationState {
    pub fn new(entity_type: EntityType, start_time: ClientTick) -> Self {
        let action_type = AnimationActionType::Idle;
        Self {
            action_type,
            action_base_offset: action_type.action_base_offset(entity_type),
            start_time,
            time: 0,
            duration: None,
            factor: None,
            looping: true,
            recover_to_ready_fight: false,
        }
    }

    pub fn idle(&mut self, entity_type: EntityType, client_tick: ClientTick) {
        self.action_type = AnimationActionType::Idle;
        self.action_base_offset = self.action_type.action_base_offset(entity_type);
        self.start_time = client_tick;
        self.duration = None;
        self.factor = None;
        self.looping = true;
        self.recover_to_ready_fight = false;
    }

    pub fn attack(&mut self, entity_type: EntityType, weapon: u32, attack_duration: u32, critical: bool, client_tick: ClientTick) {
        self.attack_with_recovery(entity_type, weapon, attack_duration, critical, false, client_tick);
    }

    pub fn hurt(&mut self, entity_type: EntityType, hurt_duration: u32, client_tick: ClientTick) {
        self.action_type = AnimationActionType::Hurt;
        self.action_base_offset = self.action_type.action_base_offset(entity_type);
        self.start_time = client_tick;
        self.duration = Some(hurt_duration.max(1));
        self.factor = None;
        self.looping = false;
        self.recover_to_ready_fight = false;
    }

    pub fn attack_with_recovery(
        &mut self,
        entity_type: EntityType,
        weapon: u32,
        attack_duration: u32,
        _critical: bool,
        recover_to_ready_fight: bool,
        client_tick: ClientTick,
    ) {
        self.action_type = match (entity_type, weapon != 0) {
            (EntityType::Player | EntityType::Hidden, true) => weapon_attack_action(weapon),
            _ => AnimationActionType::Attack1,
        };
        self.action_base_offset = self.action_type.action_base_offset(entity_type);
        self.start_time = client_tick;
        self.duration = Some(attack_duration.max(1));
        self.factor = None;
        self.looping = false;
        self.recover_to_ready_fight = recover_to_ready_fight;
    }

    pub fn pickup(&mut self, entity_type: EntityType, client_tick: ClientTick) {
        self.action_type = AnimationActionType::Pickup;
        self.action_base_offset = self.action_type.action_base_offset(entity_type);
        self.start_time = client_tick;
        self.duration = None;
        self.factor = Some(PICKUP_ANIMATION_FACTOR);
        self.looping = false;
        self.recover_to_ready_fight = false;
    }

    pub fn walk(&mut self, entity_type: EntityType, movement_speed: usize, client_tick: ClientTick) {
        self.action_type = AnimationActionType::Walk;
        self.action_base_offset = self.action_type.action_base_offset(entity_type);
        self.start_time = client_tick;
        self.duration = None;
        self.factor = Some(movement_speed as f32 * 100.0 / 150.0 / 5.0);
        self.looping = true;
        self.recover_to_ready_fight = false;
    }

    pub fn dead(&mut self, entity_type: EntityType, client_tick: ClientTick) {
        self.action_type = AnimationActionType::Die;
        self.action_base_offset = self.action_type.action_base_offset(entity_type);
        self.start_time = client_tick;
        self.duration = None;
        self.factor = None;
        self.looping = false;
        self.recover_to_ready_fight = false;
    }

    pub fn ready_fight(&mut self, entity_type: EntityType, client_tick: ClientTick) {
        self.action_type = AnimationActionType::ReadyFight;
        self.action_base_offset = self.action_type.action_base_offset(entity_type);
        self.start_time = client_tick;
        self.duration = None;
        self.factor = None;
        self.looping = true;
        self.recover_to_ready_fight = false;
    }

    pub fn recover_finished_action(&mut self, entity_type: EntityType, client_tick: ClientTick) {
        if self.is_attack() && self.recover_to_ready_fight {
            self.ready_fight(entity_type, client_tick);
        } else {
            self.idle(entity_type, client_tick);
        }
    }

    pub fn is_attack(&self) -> bool {
        matches!(
            self.action_type,
            AnimationActionType::Attack1 | AnimationActionType::Attack2 | AnimationActionType::Attack3
        )
    }

    pub fn is_pickup(&self) -> bool {
        self.action_type == AnimationActionType::Pickup
    }

    pub fn is_hurt(&self) -> bool {
        self.action_type == AnimationActionType::Hurt
    }

    pub fn is_walking(&self) -> bool {
        self.action_type == AnimationActionType::Walk
    }

    pub fn is_dead(&self) -> bool {
        self.action_type == AnimationActionType::Die
    }

    pub fn update(&mut self, client_tick: ClientTick) {
        self.time = client_tick.0.wrapping_sub(self.start_time.0);
    }
}

fn weapon_attack_action(weapon: u32) -> AnimationActionType {
    match weapon_fallback(weapon).map(|fallback| fallback.attack_action) {
        Some(WeaponAttackAction::Attack2) => AnimationActionType::Attack2,
        Some(WeaponAttackAction::Attack3) | None => AnimationActionType::Attack3,
    }
}

#[derive(RustState, Clone, StateElement)]
pub struct AnimationData {
    pub animation_pair: Vec<AnimationPair>,
    pub animations: Vec<Animation>,
    pub delays: Vec<f32>,
    #[hidden_element]
    pub entity_type: EntityType,
}

impl Cacheable for AnimationData {
    fn size(&self) -> usize {
        // We cache animations only by count.
        0
    }
}

#[derive(RustState, Clone, StateElement)]
pub struct AnimationPair {
    pub sprites: Arc<Sprite>,
    pub actions: Arc<Actions>,
}

#[derive(RustState, Clone, StateElement)]
pub struct Animation {
    #[hidden_element]
    pub frames: Vec<AnimationFrame>,
}

#[derive(Clone)]
pub struct AnimationFrame {
    pub event: Option<ActionEvent>,
    pub offset: Vector2<i32>,
    pub top_left: Vector2<i32>,
    pub size: Vector2<i32>,
    pub frame_parts: Vec<AnimationFramePart>,
    #[cfg(feature = "debug")]
    pub horizontal_matrix: Matrix4<f32>,
    #[cfg(feature = "debug")]
    pub vertical_matrix: Matrix4<f32>,
}

#[derive(Clone)]
pub struct AnimationFramePart {
    pub animation_index: usize,
    pub sprite_number: usize,
    pub offset: Vector2<i32>,
    pub size: Vector2<i32>,
    pub mirror: bool,
    pub angle: f32,
    pub color: Color,
    pub affine_matrix: Matrix4<f32>,
}

impl Default for AnimationFramePart {
    fn default() -> AnimationFramePart {
        AnimationFramePart {
            animation_index: usize::MAX,
            sprite_number: usize::MAX,
            offset: Vector2::<i32>::zero(),
            size: Vector2::<i32>::zero(),
            mirror: Default::default(),
            angle: Default::default(),
            color: Default::default(),
            affine_matrix: Matrix4::<f32>::zero(),
        }
    }
}

impl AnimationData {
    pub fn is_animation_over(&self, animation_state: &AnimationState) -> bool {
        let animation_action_index = animation_state.action_type.action_base_offset(self.entity_type) * 8;

        let delay_index = animation_action_index % self.delays.len();
        let animation_index = animation_action_index % self.animations.len();

        let delay = self.delays[delay_index];
        let animation = &self.animations[animation_index];

        let factor = animation_state.factor.map(|factor| delay * factor).unwrap_or_else(|| delay * 50.0);

        let frame_time = animation_state
            .duration
            .map(|duration| animation_state.time * animation.frames.len() as u32 / duration)
            .unwrap_or_else(|| (animation_state.time as f32 / factor) as u32);

        frame_time as usize > animation.frames.len()
    }

    pub fn get_frame(&self, animation_state: &AnimationState, camera: &dyn Camera, direction: Direction) -> &AnimationFrame {
        let camera_direction = camera.camera_direction();
        let direction = (camera_direction + u16::from(direction) as usize) & 7;
        let animation_action_index = animation_state.action_type.action_base_offset(self.entity_type) * 8 + direction;

        let delay_index = animation_action_index % self.delays.len();
        let animation_index = animation_action_index % self.animations.len();

        let delay = self.delays[delay_index];
        let animation = &self.animations[animation_index];

        let factor = animation_state.factor.map(|factor| delay * factor).unwrap_or_else(|| delay * 50.0);

        let frame_time = animation_state
            .duration
            .map(|duration| animation_state.time * animation.frames.len() as u32 / duration)
            .unwrap_or_else(|| (animation_state.time as f32 / factor) as u32);

        let frame_index = match animation_state.looping {
            true => frame_time as usize % animation.frames.len(),
            false => (frame_time as usize).min(animation.frames.len().saturating_sub(1)),
        };

        // Remove Doridori animation from Player
        if self.entity_type == EntityType::Player && animation_state.action_type == AnimationActionType::Idle {
            &animation.frames[0]
        } else {
            &animation.frames[frame_index]
        }
    }

    pub fn calculate_world_matrix(
        &self,
        camera: &dyn Camera,
        frame: &AnimationFrame,
        entity_position: Point3<f32>,
        scale: f32,
    ) -> Matrix4<f32> {
        // Offset the image to below the ground by frame.offset.y.
        // Add 0.5 to change from center of pixel to the lower border of pixel
        let origin_y = -frame.offset.y as f32 + 0.5;
        // TODO - TBD : Change the entity z coordinate to 0.0.
        // Add 1.0 in z-coordinate, because the entity is at point with z = 1.0.
        // The operation is performed beforehand to correctly rotate the billboard.
        let origin = Point3::new(0.0, origin_y, 0.0) * SPRITE_SCALE / TILE_SIZE + Vector3::unit_z();
        let size = Vector2::new(frame.size.x as f32, frame.size.y as f32) * (SPRITE_SCALE / TILE_SIZE) * scale;
        camera.billboard_matrix(entity_position, origin, size)
    }

    pub fn get_texture_coordinates(&self) -> (Vector2<f32>, Vector2<f32>) {
        let cell_count = Vector2::new(1, 1);
        let cell_position = Vector2::new(0, 0);
        let texture_size = Vector2::new(1.0 / cell_count.x as f32, 1.0 / cell_count.y as f32);
        let texture_position = Vector2::new(texture_size.x * cell_position.x as f32, texture_size.y * cell_position.y as f32);
        (texture_size, texture_position)
    }

    pub fn render(
        &self,
        instructions: &mut Vec<EntityInstruction>,
        camera: &dyn Camera,
        add_to_picker: bool,
        entity_id: EntityId,
        entity_position: Point3<f32>,
        animation_state: &AnimationState,
        direction: Direction,
        fade_alpha: f32,
        scale: f32,
    ) {
        let frame = self.get_frame(animation_state, camera, direction);
        let world_matrix = self.calculate_world_matrix(camera, frame, entity_position, scale);

        for (index, frame_part) in frame.frame_parts.iter().enumerate() {
            let animation_index = frame_part.animation_index;
            let sprite_number = frame_part.sprite_number;
            let texture = &self.animation_pair[animation_index].sprites.textures[sprite_number];

            let frame_size = Vector2::new(frame.size.x as f32, frame.size.y as f32);

            let (texture_size, texture_position) = self.get_texture_coordinates();
            let (depth_offset, curvature) = camera.calculate_depth_offset_and_curvature(&world_matrix, SPRITE_SCALE, SPRITE_SCALE);

            let position = world_matrix.transform_point(Point3::from_value(0.0));
            let distance = camera.distance_to(position);
            let color = frame_part.color * fade_alpha;

            instructions.push(EntityInstruction {
                world: world_matrix,
                frame_part_transform: frame_part.affine_matrix,
                texture_position,
                texture_size,
                frame_size,
                depth_offset,
                extra_depth_offset: 0.005 * index as f32,
                curvature,
                color,
                mirror: frame_part.mirror,
                entity_id,
                add_to_picker,
                texture: texture.clone(),
                distance,
            });
        }
    }

    #[cfg(feature = "debug")]
    pub fn render_debug(
        &self,
        instructions: &mut Vec<DebugRectangleInstruction>,
        camera: &dyn Camera,
        entity_position: Point3<f32>,
        animation_state: &AnimationState,
        direction: Direction,
        color_external: Color,
        color_internal: Color,
        scale: f32,
    ) {
        let frame = self.get_frame(animation_state, camera, direction);
        let world_matrix = self.calculate_world_matrix(camera, frame, entity_position, scale);
        instructions.push(DebugRectangleInstruction {
            world: world_matrix,
            color: color_external,
        });
        instructions.push(DebugRectangleInstruction {
            world: world_matrix * frame.horizontal_matrix,
            color: color_external,
        });
        instructions.push(DebugRectangleInstruction {
            world: world_matrix * frame.vertical_matrix,
            color: color_external,
        });

        for frame_part in frame.frame_parts.iter() {
            instructions.push(DebugRectangleInstruction {
                world: world_matrix * frame_part.affine_matrix,
                color: color_internal,
            });
        }
    }
}
