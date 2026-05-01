use std::cell::UnsafeCell;

use korangar_interface::window::{CustomWindow, Window};
use rust_state::{Path, PathExt, Selector};

use crate::graphics::{Color, CornerDiameter};
use crate::input::InputEvent;
use crate::interface::windows::WindowClass;
use crate::loaders::OverflowBehavior;
use crate::state::localization::LocalizationPathExt;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, ClientStatePathExt, client_state};
use crate::world::{Player, PlayerPathExt};

#[derive(Clone, Copy)]
enum PlayerText {
    Job,
    Health,
    Spell,
    Zeny,
    Weight,
}

#[derive(Clone, Copy)]
enum PlayerFraction {
    Health,
    Spell,
}

struct PlayerTextSelector<P> {
    player_path: P,
    text_kind: PlayerText,
    text: UnsafeCell<String>,
}

impl<P> PlayerTextSelector<P> {
    fn new(player_path: P, text_kind: PlayerText) -> Self {
        Self {
            player_path,
            text_kind,
            text: UnsafeCell::default(),
        }
    }
}

impl<P> Selector<ClientState, String> for PlayerTextSelector<P>
where
    P: Path<ClientState, Player>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let player = self.player_path.follow_safe(state);
        let value = match self.text_kind {
            PlayerText::Job => player.job_name.clone(),
            PlayerText::Health => {
                let common = player.get_common();
                format!("HP  {} / {}", common.health_points, common.maximum_health_points)
            }
            PlayerText::Spell => format!("SP  {} / {}", player.spell_points, player.maximum_spell_points),
            PlayerText::Zeny => player.zeny.to_string(),
            PlayerText::Weight => format!("{} / {}", player.weight, player.maximum_weight),
        };

        unsafe { *self.text.get() = value };
        unsafe { Some(self.text.as_ref_unchecked()) }
    }
}

struct PlayerFractionSelector<P> {
    player_path: P,
    fraction_kind: PlayerFraction,
    value: UnsafeCell<f32>,
}

impl<P> PlayerFractionSelector<P> {
    fn new(player_path: P, fraction_kind: PlayerFraction) -> Self {
        Self {
            player_path,
            fraction_kind,
            value: UnsafeCell::new(0.0),
        }
    }
}

impl<P> Selector<ClientState, f32> for PlayerFractionSelector<P>
where
    P: Path<ClientState, Player>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a f32> {
        let player = self.player_path.follow_safe(state);
        let value = match self.fraction_kind {
            PlayerFraction::Health => {
                let common = player.get_common();
                ratio(common.health_points, common.maximum_health_points)
            }
            PlayerFraction::Spell => ratio(player.spell_points, player.maximum_spell_points),
        };

        unsafe { *self.value.get() = value };
        unsafe { Some(self.value.as_ref_unchecked()) }
    }
}

fn ratio(current: usize, maximum: usize) -> f32 {
    match maximum {
        0 => 0.0,
        maximum => current as f32 / maximum as f32,
    }
}

pub struct CharacterOverviewWindow<A, B> {
    player_name_path: A,
    player_path: B,
}

impl<A, B> CharacterOverviewWindow<A, B> {
    pub fn new(player_name_path: A, player_path: B) -> Self {
        Self {
            player_name_path,
            player_path,
        }
    }
}

impl<A, B> CustomWindow<ClientState> for CharacterOverviewWindow<A, B>
where
    A: Path<ClientState, String>,
    B: Path<ClientState, Player> + Copy,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::CharacterOverview)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        macro_rules! value_row {
            ($label:expr, $selector:expr, $color:expr) => {
                split! {
                    children: (
                        text! {
                            text: $label,
                            overflow_behavior: OverflowBehavior::Shrink,
                        },
                        text! {
                            text: $selector,
                            color: $color,
                            horizontal_alignment: HorizontalAlignment::Right { offset: 0.0, border: 3.0 },
                            overflow_behavior: OverflowBehavior::Shrink,
                        },
                    ),
                }
            };
        }

        macro_rules! resource_bar {
            ($text_kind:expr, $fraction_kind:expr, $fill:expr, $back:expr) => {
                bar! {
                    text: PlayerTextSelector::new(self.player_path, $text_kind),
                    fraction: PlayerFractionSelector::new(self.player_path, $fraction_kind),
                    foreground_color: Color::WHITE,
                    fill_color: $fill,
                    background_color: $back,
                    height: 24.0,
                    corner_diameter: CornerDiameter::uniform(6.0),
                    overflow_behavior: OverflowBehavior::Shrink,
                    lerp_duration_ms: 250u32,
                }
            };
        }

        window! {
            title: client_state().localization().character_overview_window_title(),
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            minimum_width: 360.0,
            maximum_width: 360.0,
            elements: (
                fragment! {
                    gaps: 2.0,
                    children: (
                        text! {
                            text: self.player_name_path,
                            color: Color::rgb_u8(255, 184, 72),
                            horizontal_alignment: HorizontalAlignment::Center { offset: 0.0, border: 3.0 },
                            overflow_behavior: OverflowBehavior::Shrink,
                        },
                        text! {
                            text: PlayerTextSelector::new(self.player_path, PlayerText::Job),
                            color: Color::rgb_u8(165, 210, 255),
                            horizontal_alignment: HorizontalAlignment::Center { offset: 0.0, border: 3.0 },
                            overflow_behavior: OverflowBehavior::Shrink,
                        },
                    ),
                },
                split! {
                    gaps: 8.0,
                    children: (
                        value_row!(
                            client_state().localization().base_level_text(),
                            PartialEqDisplaySelector::new(self.player_path.base_level()),
                            Color::rgb_u8(130, 235, 255)
                        ),
                        value_row!(
                            client_state().localization().job_level_text(),
                            PartialEqDisplaySelector::new(self.player_path.job_level()),
                            Color::rgb_u8(130, 235, 255)
                        ),
                    ),
                },
                resource_bar!(
                    PlayerText::Health,
                    PlayerFraction::Health,
                    Color::rgb_u8(205, 62, 62),
                    Color::rgb_u8(70, 24, 28)
                ),
                resource_bar!(
                    PlayerText::Spell,
                    PlayerFraction::Spell,
                    Color::rgb_u8(52, 112, 215),
                    Color::rgb_u8(22, 34, 74)
                ),
                value_row!("Zeny", PlayerTextSelector::new(self.player_path, PlayerText::Zeny), Color::rgb_u8(255, 211, 92)),
                value_row!("Weight", PlayerTextSelector::new(self.player_path, PlayerText::Weight), Color::rgb_u8(143, 232, 126)),
                button! {
                    text: client_state().localization().inventory_button_text(),
                    event: InputEvent::ToggleInventoryWindow,
                },
                button! {
                    text: client_state().localization().equipment_button_text(),
                    event: InputEvent::ToggleEquipmentWindow,
                },
                button! {
                    text: client_state().localization().skill_tree_button_text(),
                    event: InputEvent::ToggleSkillTreeWindow,
                },
                button! {
                    text: client_state().localization().stats_button_text(),
                    event: InputEvent::ToggleStatsWindow,
                },
                button! {
                    text: client_state().localization().friend_list_button_text(),
                    event: InputEvent::ToggleFriendListWindow,
                },
                button! {
                    text: client_state().localization().menu_button_text(),
                    event: InputEvent::ToggleMenuWindow,
                },
            ),
        }
    }
}
