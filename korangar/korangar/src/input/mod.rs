mod event;
mod key;
mod mode;

use std::mem::variant_count;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use cgmath::{InnerSpace, Vector2, Vector3};
use ragnarok_packets::{ClientTick, HotbarSlot};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta};
use winit::keyboard::KeyCode;

pub use self::event::InputEvent;
pub use self::key::Key;
pub use self::mode::{Grabbed, MouseInputMode, MouseModeExt};
use crate::graphics::{PickerTarget, ScreenPosition, ScreenSize};

const MOUSE_SCOLL_MULTIPLIER: f32 = 30.0;
const KEY_COUNT: usize = variant_count::<KeyCode>();
const DOUBLE_CLICK_TIME_MS: u32 = 250;
const HOTBAR_NUMBER_KEYS: [KeyCode; 10] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
    KeyCode::Digit8,
    KeyCode::Digit9,
    KeyCode::Digit0,
];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MovementKeyState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
}

impl MovementKeyState {
    pub fn has_movement(self) -> bool {
        self.forward || self.backward || self.left || self.right
    }
}

pub fn tile_offset_for_camera_movement(
    keys: MovementKeyState,
    camera_view_direction: Vector3<f32>,
    tile_distance: i16,
) -> Option<(i16, i16)> {
    let forward = Vector2::new(camera_view_direction.x, camera_view_direction.z);

    if !keys.has_movement() || forward.magnitude2() <= f32::EPSILON {
        return None;
    }

    let forward = forward.normalize();
    let right = Vector2::new(forward.y, -forward.x);
    let mut movement = Vector2::new(0.0, 0.0);

    if keys.forward {
        movement += forward;
    }

    if keys.backward {
        movement -= forward;
    }

    if keys.right {
        movement += right;
    }

    if keys.left {
        movement -= right;
    }

    if movement.magnitude2() <= f32::EPSILON {
        return None;
    }

    let movement = movement.normalize() * tile_distance as f32;
    let offset_x = movement.x.round() as i16;
    let offset_y = movement.y.round() as i16;

    (offset_x != 0 || offset_y != 0).then_some((offset_x, offset_y))
}

pub(crate) fn hotbar_slot_for_number_key(key_code: KeyCode) -> Option<HotbarSlot> {
    HOTBAR_NUMBER_KEYS
        .iter()
        .position(|hotbar_key| *hotbar_key == key_code)
        .map(|slot| HotbarSlot(slot as u16))
}

#[derive(Debug, Clone, Copy)]
struct PreviousMouseButton {
    button: MouseButton,
    tick: ClientTick,
}

// TODO: Rename
pub struct InputReport {
    pub mouse_click: Option<korangar_interface::layout::MouseButton>,
    pub mouse_position: ScreenPosition,
    pub mouse_delta: ScreenSize,
    pub mouse_button_released: bool,
    pub left_mouse_button_down: bool,
    pub scroll: Option<f32>,
    pub drag: Option<ScreenSize>,
    pub characters: Vec<char>,
    pub mouse_target: PickerTarget,
}

pub struct InputSystem {
    previous_mouse_position: ScreenPosition,
    new_mouse_position: ScreenPosition,
    mouse_delta: ScreenSize,
    previous_scroll_position: f32,
    new_scroll_position: f32,
    scroll_delta: f32,
    left_mouse_button: Key,
    right_mouse_button: Key,
    keys: [Key; KEY_COUNT],
    input_buffer: Vec<char>,
    picker_value: Arc<AtomicU64>,
    previous_mouse_button: Option<PreviousMouseButton>,
}

impl InputSystem {
    pub fn new(picker_value: Arc<AtomicU64>) -> Self {
        let previous_mouse_position = ScreenPosition::default();
        let new_mouse_position = ScreenPosition::default();
        let mouse_delta = ScreenSize::default();

        let previous_scroll_position = 0.0;
        let new_scroll_position = 0.0;
        let scroll_delta = 0.0;

        let left_mouse_button = Key::default();
        let right_mouse_button = Key::default();
        let keys = [Key::default(); KEY_COUNT];

        let input_buffer = Vec::new();
        let previous_mouse_button = None;

        Self {
            previous_mouse_position,
            new_mouse_position,
            mouse_delta,
            previous_scroll_position,
            new_scroll_position,
            scroll_delta,
            left_mouse_button,
            right_mouse_button,
            keys,
            input_buffer,
            picker_value,
            previous_mouse_button,
        }
    }

    pub fn reset(&mut self) {
        self.left_mouse_button.reset();
        self.right_mouse_button.reset();
        self.keys.iter_mut().for_each(|key| key.reset());
    }

    pub fn update_mouse_position(&mut self, position: PhysicalPosition<f64>) {
        self.new_mouse_position = ScreenPosition {
            left: position.x as f32,
            top: position.y as f32,
        };
    }

    pub fn update_mouse_buttons(&mut self, button: MouseButton, state: ElementState) {
        let pressed = matches!(state, ElementState::Pressed);

        match button {
            MouseButton::Left => self.left_mouse_button.set_down(pressed),
            MouseButton::Right => self.right_mouse_button.set_down(pressed),
            _ignored => {}
        }
    }

    pub fn update_mouse_wheel(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(_x, y) => self.new_scroll_position += y * MOUSE_SCOLL_MULTIPLIER,
            MouseScrollDelta::PixelDelta(position) => self.new_scroll_position += position.y as f32,
        }
    }

    pub fn update_keyboard(&mut self, key_code: KeyCode, state: ElementState) {
        let pressed = matches!(state, ElementState::Pressed);
        self.keys[key_code as usize].set_down(pressed);
    }

    pub fn buffer_character(&mut self, character: char) {
        self.input_buffer.push(character);
    }

    #[cfg_attr(feature = "debug", korangar_debug::profile("update input system"))]
    pub fn update_delta(&mut self, client_tick: ClientTick) -> InputReport {
        self.mouse_delta = self.new_mouse_position - self.previous_mouse_position;
        self.previous_mouse_position = self.new_mouse_position;

        self.scroll_delta = self.new_scroll_position - self.previous_scroll_position;
        self.previous_scroll_position = self.new_scroll_position;

        self.left_mouse_button.update();
        self.right_mouse_button.update();
        self.keys.iter_mut().for_each(|key| key.update());

        let mouse_button_released = self.left_mouse_button.released() || self.right_mouse_button.released();

        let last_pixel_value = self.picker_value.load(Ordering::Acquire);
        let mouse_target = PickerTarget::from(last_pixel_value);

        let mut mouse_click = None;

        if self.left_mouse_button.pressed() {
            if let Some(previous_mouse_button) = self.previous_mouse_button
                && previous_mouse_button.button == MouseButton::Left
                && client_tick.0.wrapping_sub(previous_mouse_button.tick.0) <= DOUBLE_CLICK_TIME_MS
            {
                self.previous_mouse_button = None;

                mouse_click = Some(korangar_interface::layout::MouseButton::DoubleLeft);
            } else {
                self.previous_mouse_button = Some(PreviousMouseButton {
                    button: MouseButton::Left,
                    tick: client_tick,
                });

                mouse_click = Some(korangar_interface::layout::MouseButton::Left);
            }
        } else if self.right_mouse_button.pressed() {
            if let Some(previous_mouse_button) = self.previous_mouse_button
                && previous_mouse_button.button == MouseButton::Right
                && client_tick.0.wrapping_sub(previous_mouse_button.tick.0) <= DOUBLE_CLICK_TIME_MS
            {
                self.previous_mouse_button = None;

                mouse_click = Some(korangar_interface::layout::MouseButton::DoubleRight);
            } else {
                self.previous_mouse_button = Some(PreviousMouseButton {
                    button: MouseButton::Right,
                    tick: client_tick,
                });

                mouse_click = Some(korangar_interface::layout::MouseButton::Right);
            }
        }

        InputReport {
            mouse_click,
            mouse_position: self.new_mouse_position,
            mouse_delta: self.mouse_delta,
            mouse_button_released,
            left_mouse_button_down: self.left_mouse_button.down(),
            scroll: (self.scroll_delta != 0.0).then_some(self.scroll_delta),
            drag: self.left_mouse_button.down().then_some(self.mouse_delta),
            characters: self.input_buffer.drain(..).collect(),
            mouse_target,
        }
    }

    fn get_key(&self, key_code: KeyCode) -> &Key {
        &self.keys[key_code as usize]
    }

    pub fn movement_key_state(&self) -> MovementKeyState {
        let alt_down = self.get_key(KeyCode::AltLeft).down() || self.get_key(KeyCode::AltRight).down();
        let control_down = self.get_key(KeyCode::ControlLeft).down() || self.get_key(KeyCode::ControlRight).down();

        if alt_down || control_down {
            return MovementKeyState::default();
        }

        MovementKeyState {
            forward: self.get_key(KeyCode::KeyW).down(),
            backward: self.get_key(KeyCode::KeyS).down(),
            left: self.get_key(KeyCode::KeyA).down(),
            right: self.get_key(KeyCode::KeyD).down(),
        }
    }

    #[cfg_attr(feature = "debug", korangar_debug::profile)]
    pub fn handle_keyboard_input(
        &mut self,
        events: &mut Vec<InputEvent>,
        #[cfg(feature = "debug")] process_mouse: bool,
        #[cfg(feature = "debug")] use_debug_camera: bool,
    ) {
        let alt_down = self.get_key(KeyCode::AltLeft).down();
        let control_down = self.get_key(KeyCode::ControlLeft).down();

        if self.get_key(KeyCode::Escape).pressed() {
            events.push(InputEvent::ToggleMenuWindow);
        }

        if alt_down && self.get_key(KeyCode::KeyE).pressed() {
            events.push(InputEvent::ToggleInventoryWindow);
        }

        if alt_down && self.get_key(KeyCode::KeyS).pressed() {
            events.push(InputEvent::ToggleSkillTreeWindow);
        }

        if alt_down && self.get_key(KeyCode::KeyA).pressed() {
            events.push(InputEvent::ToggleStatsWindow);
        }

        if alt_down && self.get_key(KeyCode::KeyZ).pressed() {
            events.push(InputEvent::ToggleFriendListWindow);
        }

        if alt_down && self.get_key(KeyCode::KeyQ).pressed() {
            events.push(InputEvent::ToggleEquipmentWindow);
        }

        if control_down && self.get_key(KeyCode::KeyS).pressed() {
            events.push(InputEvent::ToggleGameSettingsWindow);
        }

        if control_down && self.get_key(KeyCode::KeyI).pressed() {
            events.push(InputEvent::ToggleInterfaceSettingsWindow);
        }

        if control_down && self.get_key(KeyCode::KeyG).pressed() {
            events.push(InputEvent::ToggleGraphicsSettingsWindow);
        }

        if control_down && self.get_key(KeyCode::KeyA).pressed() {
            events.push(InputEvent::ToggleAudioSettingsWindow);
        }

        if control_down && self.get_key(KeyCode::KeyH).pressed() {
            events.push(InputEvent::ToggleShowInterface);
        }

        if control_down && self.get_key(KeyCode::KeyQ).pressed() {
            events.push(InputEvent::CloseTopWindow);
        }

        for key_code in HOTBAR_NUMBER_KEYS {
            let slot = hotbar_slot_for_number_key(key_code).unwrap();

            if self.get_key(key_code).pressed() {
                events.push(InputEvent::CastSkill { slot });
            }

            if self.get_key(key_code).released() {
                events.push(InputEvent::StopSkill { slot });
            }
        }

        #[cfg(feature = "debug")]
        if control_down && self.get_key(KeyCode::KeyM).pressed() {
            events.push(InputEvent::ToggleMapsWindow);
        }

        #[cfg(feature = "debug")]
        if control_down && self.get_key(KeyCode::KeyC).pressed() {
            events.push(InputEvent::ToggleClientStateInspectorWindow);
        }

        #[cfg(feature = "debug")]
        if control_down && self.get_key(KeyCode::KeyR).pressed() {
            events.push(InputEvent::ToggleRenderOptionsWindow);
        }

        #[cfg(feature = "debug")]
        if control_down && self.get_key(KeyCode::KeyP).pressed() {
            events.push(InputEvent::ToggleProfilerWindow);
        }

        #[cfg(feature = "debug")]
        if control_down && self.get_key(KeyCode::KeyO).pressed() {
            events.push(InputEvent::ToggleCommandsWindow);
        }

        #[cfg(feature = "debug")]
        if self.get_key(KeyCode::ShiftLeft).pressed() && use_debug_camera {
            events.push(InputEvent::CameraAccelerate);
        }

        #[cfg(feature = "debug")]
        if self.get_key(KeyCode::ShiftLeft).released() && use_debug_camera {
            events.push(InputEvent::CameraDecelerate);
        }

        // TODO: This should be moved.
        #[cfg(feature = "debug")]
        if self.right_mouse_button.down() && !self.right_mouse_button.pressed() && process_mouse && use_debug_camera {
            let offset = -cgmath::Vector2::new(self.mouse_delta.width, self.mouse_delta.height);
            events.push(InputEvent::CameraLookAround { offset });
        }

        #[cfg(feature = "debug")]
        if self.get_key(KeyCode::KeyW).down() && use_debug_camera {
            events.push(InputEvent::CameraMoveForward);
        }

        #[cfg(feature = "debug")]
        if self.get_key(KeyCode::KeyS).down() && use_debug_camera {
            events.push(InputEvent::CameraMoveBackward);
        }

        #[cfg(feature = "debug")]
        if self.get_key(KeyCode::KeyA).down() && use_debug_camera {
            events.push(InputEvent::CameraMoveLeft);
        }

        #[cfg(feature = "debug")]
        if self.get_key(KeyCode::KeyD).down() && use_debug_camera {
            events.push(InputEvent::CameraMoveRight);
        }

        #[cfg(feature = "debug")]
        if self.get_key(KeyCode::Space).down() && use_debug_camera {
            events.push(InputEvent::CameraMoveUp);
        }

        self.input_buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Vector3;
    use ragnarok_packets::HotbarSlot;
    use winit::keyboard::KeyCode;

    use super::{MovementKeyState, hotbar_slot_for_number_key, tile_offset_for_camera_movement};

    #[test]
    fn number_row_keys_map_to_hotbar_slots() {
        let cases = [
            (KeyCode::Digit1, HotbarSlot(0)),
            (KeyCode::Digit2, HotbarSlot(1)),
            (KeyCode::Digit3, HotbarSlot(2)),
            (KeyCode::Digit4, HotbarSlot(3)),
            (KeyCode::Digit5, HotbarSlot(4)),
            (KeyCode::Digit6, HotbarSlot(5)),
            (KeyCode::Digit7, HotbarSlot(6)),
            (KeyCode::Digit8, HotbarSlot(7)),
            (KeyCode::Digit9, HotbarSlot(8)),
            (KeyCode::Digit0, HotbarSlot(9)),
        ];

        for (key_code, expected_slot) in cases {
            assert_eq!(hotbar_slot_for_number_key(key_code), Some(expected_slot));
        }
    }

    #[test]
    fn non_number_row_keys_do_not_map_to_hotbar_slots() {
        assert_eq!(hotbar_slot_for_number_key(KeyCode::KeyJ), None);
    }

    #[test]
    fn movement_offset_uses_camera_forward_direction() {
        let keys = MovementKeyState {
            forward: true,
            ..Default::default()
        };

        assert_eq!(
            tile_offset_for_camera_movement(keys, Vector3::new(0.0, -0.5, 1.0), 5),
            Some((0, 5))
        );
    }

    #[test]
    fn movement_offset_normalizes_diagonal_input() {
        let keys = MovementKeyState {
            forward: true,
            left: true,
            ..Default::default()
        };

        assert_eq!(
            tile_offset_for_camera_movement(keys, Vector3::new(0.0, -0.5, 1.0), 5),
            Some((-4, 4))
        );
    }
}
