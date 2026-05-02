use korangar_interface::window::{CustomWindow, Window};
use rust_state::Path;

use crate::interface::windows::WindowClass;
use crate::loaders::OverflowBehavior;
use crate::settings::{InterfaceSettings, InterfaceSettingsCapabilities, InterfaceSettingsCapabilitiesPathExt, InterfaceSettingsPathExt};
use crate::state::localization::LocalizationPathExt;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, ClientStatePathExt, client_state};

pub struct InterfaceSettingsWindow<A, B> {
    settings_path: A,
    capabilities_path: B,
}

impl<A, B> InterfaceSettingsWindow<A, B> {
    pub fn new(settings_path: A, capabilities_path: B) -> Self {
        Self {
            settings_path,
            capabilities_path,
        }
    }
}

impl<A, B> CustomWindow<ClientState> for InterfaceSettingsWindow<A, B>
where
    A: Path<ClientState, InterfaceSettings>,
    B: Path<ClientState, InterfaceSettingsCapabilities>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::InterfaceSettings)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        let elements = (
            split! {
                children: (
                    text! {
                        text: client_state().localization().language_text(),
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.language(),
                        options: self.capabilities_path.languages(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: client_state().localization().scaling_text(),
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.scaling(),
                        options: self.capabilities_path.scalings(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: client_state().localization().menu_theme_text(),
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.menu_theme(),
                        options: self.capabilities_path.menu_themes(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: client_state().localization().in_game_theme_text(),
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.in_game_theme(),
                        options: self.capabilities_path.in_game_themes(),
                    }
                )
            },
            split! {
                children: (
                    text! {
                        text: client_state().localization().world_theme_text(),
                        overflow_behavior: OverflowBehavior::Shrink,
                    },
                    drop_down! {
                        selected: self.settings_path.world_theme(),
                        options: self.capabilities_path.world_themes(),
                    }
                )
            },
            state_button! {
                text: "NPC cinematic dialog",
                state: self.settings_path.npc_cinematic_dialog_enabled(),
                event: Toggle(self.settings_path.npc_cinematic_dialog_enabled()),
            },
            state_button! {
                text: "Dynamic cinematic camera",
                state: self.settings_path.npc_cinematic_dynamic_camera_enabled(),
                event: Toggle(self.settings_path.npc_cinematic_dynamic_camera_enabled()),
            },
            state_button! {
                text: "Text sound",
                state: self.settings_path.npc_cinematic_text_sound_enabled(),
                event: Toggle(self.settings_path.npc_cinematic_text_sound_enabled()),
            },
            state_button! {
                text: "Third-person WASD movement",
                state: self.settings_path.third_person_movement_enabled(),
                event: Toggle(self.settings_path.third_person_movement_enabled()),
            },
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
        );

        window! {
            title: client_state().localization().interface_settings_window_title(),
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            elements,
        }
    }
}
