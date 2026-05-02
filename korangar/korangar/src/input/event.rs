#[cfg(feature = "debug")]
use cgmath::Vector2;
#[cfg(feature = "debug")]
use korangar_debug::profiling::FrameMeasurement;
use korangar_interface::event::{ClickHandler, Event, EventQueue};
use korangar_networking::{InventoryItem, InventoryItemDetails, ShopItem};
use ragnarok_packets::{
    AccountId, BuyOrSellOption, CharacterId, CharacterServerInformation, EntityId, EquipPosition, HotbarSlot, InventoryIndex, ShopId,
    SkillId, SoldItemInformation, StatUpType, TilePosition,
};
use rust_state::State;

use crate::interface::resource::{ItemSource, SkillSource};
use crate::loaders::ServiceId;
use crate::state::ClientState;
use crate::state::skills::LearnableSkill;
#[cfg(feature = "debug")]
use crate::world::MarkerIdentifier;
use crate::world::ResourceMetadata;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryItemActivation {
    Equip { index: InventoryIndex, position: EquipPosition },
    Unequip { index: InventoryIndex },
    Use { index: InventoryIndex },
}

pub fn inventory_item_activation<Meta>(item: &InventoryItem<Meta>) -> Option<InventoryItemActivation> {
    const ITEM_TYPE_HEALING: u8 = 0;
    const ITEM_TYPE_USABLE: u8 = 2;
    const ITEM_TYPE_AMMO: u8 = 10;
    const ITEM_TYPE_DELAY_CONSUME: u8 = 11;

    match &item.details {
        InventoryItemDetails::Equippable {
            equip_position,
            equipped_position,
            ..
        } => match equipped_position.is_empty() {
            true => Some(InventoryItemActivation::Equip {
                index: item.index,
                position: *equip_position,
            }),
            false => Some(InventoryItemActivation::Unequip { index: item.index }),
        },
        InventoryItemDetails::Regular { equipped_position, .. }
            if !equipped_position.is_empty() && equipped_position.intersects(EquipPosition::AMMO) =>
        {
            Some(InventoryItemActivation::Unequip { index: item.index })
        }
        InventoryItemDetails::Regular { equipped_position, .. }
            if item.item_type == ITEM_TYPE_AMMO || is_known_ammunition_item(item.item_id.0) =>
        {
            match equipped_position.is_empty() {
                true => Some(InventoryItemActivation::Equip {
                    index: item.index,
                    position: EquipPosition::AMMO,
                }),
                false => Some(InventoryItemActivation::Unequip { index: item.index }),
            }
        }
        InventoryItemDetails::Regular { .. }
            if matches!(item.item_type, ITEM_TYPE_HEALING | ITEM_TYPE_USABLE | ITEM_TYPE_DELAY_CONSUME) =>
        {
            Some(InventoryItemActivation::Use { index: item.index })
        }
        InventoryItemDetails::Regular { .. } => None,
    }
}

fn is_known_ammunition_item(item_id: u32) -> bool {
    matches!(
        item_id,
        // Arrows.
        1750..=1776
            // Bullets, shells, spheres and other Gunslinger ammunition.
            | 13200..=13250
    )
}

/// An event triggered by the user through mouse or keyboard input.
#[derive(Clone, Debug)]
pub enum InputEvent {
    /// Log in to the login server.
    LogIn {
        /// Id of the selected service.
        service_id: ServiceId,
        /// Account username.
        username: String,
        /// Account password.
        password: String,
    },
    /// Select a character server.
    SelectServer {
        /// Selected character server.
        character_server_information: CharacterServerInformation,
    },
    /// Respawn the player.
    Respawn,
    /// Log out of the map server.
    LogOut,
    /// Log out of the character server.
    LogOutCharacter,
    /// Exit Korangar.
    Exit,
    /// Zoom the player camera.
    ZoomCamera {
        /// Amount to zoom.
        zoom_factor: f32,
    },
    /// Zoom the circular minimap.
    ZoomCircularMinimap {
        /// Scroll amount to apply.
        scroll_delta: f32,
    },
    /// Rotate the player camera.
    RotateCamera {
        /// Amount of rotation.
        rotation: f32,
    },
    /// Rotate the third-person camera.
    RotateThirdPersonCamera {
        /// Horizontal camera rotation.
        yaw: f32,
        /// Vertical camera rotation.
        pitch: f32,
    },
    /// Reset the player camera rotation.
    ResetCameraRotation,
    /// Open or close the menu window. Only works while playing.
    ToggleMenuWindow,
    /// Open or close the inventory window. Only works while playing.
    ToggleInventoryWindow,
    /// Open or close the equipment window. Only works while playing.
    ToggleEquipmentWindow,
    /// Open or close the skill tree window. Only works while playing.
    ToggleSkillTreeWindow,
    /// Open or close the stats window. Only works while playing.
    ToggleStatsWindow,
    /// Open or close the game settings window.
    ToggleGameSettingsWindow,
    /// Open or close the interface settings window.
    ToggleInterfaceSettingsWindow,
    /// Open or close the graphics settings window.
    ToggleGraphicsSettingsWindow,
    /// Open or close the audio settings window.
    ToggleAudioSettingsWindow,
    /// Open or close the friend list window. Only works while playing.
    ToggleFriendListWindow,
    /// Close the most recently opened or clicked closable window.
    CloseTopWindow,
    /// Toggle if the user interface should be rendered or not.
    ToggleShowInterface,
    /// Select a character to start playing.
    SelectCharacter {
        /// Slot that the selected character is in.
        slot: usize,
    },
    /// Open a window to create a new character.
    OpenCharacterCreationWindow {
        /// Slot in which to create the new character.
        slot: usize,
    },
    /// Create a new character.
    CreateCharacter {
        /// Slot in which to create the new character.
        slot: usize,
        /// Name of the new character.
        name: String,
    },
    /// Delete a character.
    DeleteCharacter {
        /// Id of the character to be deleted.
        character_id: CharacterId,
    },
    /// Switch the characters of two slots.
    SwitchCharacterSlot {
        /// First slot.
        origin_slot: usize,
        /// Second slot.
        destination_slot: usize,
    },
    /// Start moving the player.
    PlayerMove {
        /// Destination of the move.
        destination: TilePosition,
    },
    /// Interact with an entity. The type of interaction depends on the entity
    /// type.
    PlayerInteract {
        /// Id of the entity to interact with.
        entity_id: EntityId,
    },
    /// Pick up an item from the ground.
    PickUpItem {
        /// Id of the item entity to pick up.
        entity_id: EntityId,
    },
    /// Send a chat message.
    SendMessage {
        /// Text of the message.
        text: String,
    },
    /// Action for the "Next"-button in a dialog.
    NextDialog {
        /// Id of the NPC the player is in a dialog with.
        npc_id: EntityId,
    },
    /// Action for the "Close"-button in a dialog.
    CloseDialog {
        /// Id of the NPC the player is in a dialog with.
        npc_id: EntityId,
    },
    /// Reveal or advance the active cinematic dialog.
    AdvanceCinematicDialog,
    /// Choose an option in a dialog.
    ChooseDialogOption {
        /// Id of the NPC the player is in a dialog with.
        npc_id: EntityId,
        /// Id of the option.
        option: i8,
    },
    /// Move an item in the user interface.
    MoveItem {
        /// Source of the move.
        source: ItemSource,
        /// Destination of the move.
        destination: ItemSource,
        /// Item to move.
        item: InventoryItem<ResourceMetadata>,
    },
    /// Activate an inventory item through a shortcut such as double click.
    ActivateInventoryItem {
        /// Item to equip, unequip, or use.
        item: InventoryItem<ResourceMetadata>,
    },
    /// Move a skill in the user interface.
    MoveSkill {
        /// Source of the move.
        source: SkillSource,
        /// Destination of the move.
        destination: SkillSource,
        /// Skill to move.
        skill: LearnableSkill,
    },
    /// Cast a skill.
    CastSkill {
        /// Slot of the hotbar that the skill is bound to.
        slot: HotbarSlot,
    },
    /// Stop a skill.
    StopSkill {
        /// Slot of the hotbar that the skill is bound to.
        slot: HotbarSlot,
    },
    /// Add a new friend.
    AddFriend {
        /// Name of the character to befriend.
        character_name: String,
    },
    /// Remove a current friend.
    RemoveFriend {
        /// Account id of the friend.
        account_id: AccountId,
        /// Character id of the friend.
        character_id: CharacterId,
    },
    /// Reject a pending friend request.
    RejectFriendRequest {
        /// Account id of the requestor.
        account_id: AccountId,
        /// Character id of the requestor.
        character_id: CharacterId,
    },
    /// Accept a pending friend request.
    AcceptFriendRequest {
        /// Account id of the requestor.
        account_id: AccountId,
        /// Character id of the requestor.
        character_id: CharacterId,
    },
    /// Buy items from a shop.
    BuyItems {
        /// Items to buy.
        items: Vec<ShopItem<u32>>,
    },
    /// Close the shop.
    CloseShop,
    /// Choose whether to buy or sell items at a shop.
    BuyOrSell {
        /// Id of the open shop.
        shop_id: ShopId,
        /// Whether to sell or buy items.
        buy_or_sell: BuyOrSellOption,
    },
    /// Sell items to a shop.
    SellItems {
        /// Items to sell.
        items: Vec<SoldItemInformation>,
    },
    /// Up a stat.
    StatUp { stat_type: StatUpType },
    /// Distribute skill points to meet all requirements for a given skill and
    /// put a single point into the provided skill. If the player does not
    /// have enough skill points, this will skill as much of the
    /// dependencies as possible.
    DistributePointsForSkill {
        /// Id of the skill to level up.
        skill_id: SkillId,
    },
    /// Level up a skill.
    LevelUpSkills {
        /// List of skills to level up by one. This list is allowed to contain
        /// the same skill id multiple times and they will be applied
        /// sequentially from start to end.
        skill_ids: Vec<SkillId>,
    },
    /// Reload the language from disk.
    #[cfg(feature = "debug")]
    ReloadLanguage,
    /// Save the language to disk.
    #[cfg(feature = "debug")]
    SaveLanguage,
    /// Warp the player.
    #[cfg(feature = "debug")]
    WarpToMap {
        /// Map name. Can be the same as the current map.
        map_name: String,
        /// Position on the new map after the warp.
        position: TilePosition,
    },
    /// Open a window with the details for a marker.
    #[cfg(feature = "debug")]
    OpenMarkerDetails {
        /// Id of the marker to inspect.
        marker_identifier: MarkerIdentifier,
    },
    /// Open or close the render options window.
    #[cfg(feature = "debug")]
    ToggleRenderOptionsWindow,
    /// Open the map data window.
    #[cfg(feature = "debug")]
    OpenMapDataWindow,
    /// Open or close the client state inspector window.
    #[cfg(feature = "debug")]
    ToggleClientStateInspectorWindow,
    /// Open or close the maps window. Only works while playing.
    #[cfg(feature = "debug")]
    ToggleMapsWindow,
    /// Open or close the commands window. Only works while playing.
    #[cfg(feature = "debug")]
    ToggleCommandsWindow,
    /// Open the theme inspector window.
    #[cfg(feature = "debug")]
    ToggleThemeInspectorWindow,
    /// Open or close the profiler window.
    #[cfg(feature = "debug")]
    ToggleProfilerWindow,
    /// Open or close the packet inspector window.
    #[cfg(feature = "debug")]
    TogglePacketInspectorWindow,
    /// Open the cache statistics window.
    #[cfg(feature = "debug")]
    ToggleCacheStatisticsWindow,
    /// Move the view direction of the debug camera.
    #[cfg(feature = "debug")]
    CameraLookAround {
        /// Offset of the view direction.
        offset: Vector2<f32>,
    },
    /// Move the debug camera forward.
    #[cfg(feature = "debug")]
    CameraMoveForward,
    /// Move the debug camera backward.
    #[cfg(feature = "debug")]
    CameraMoveBackward,
    /// Move the debug camera left.
    #[cfg(feature = "debug")]
    CameraMoveLeft,
    /// Move the debug camera right.
    #[cfg(feature = "debug")]
    CameraMoveRight,
    /// Move the debug camera up.
    #[cfg(feature = "debug")]
    CameraMoveUp,
    /// Set the debug camera speed to its higher value.
    #[cfg(feature = "debug")]
    CameraAccelerate,
    /// Set the debug camera speed to its lower value.
    #[cfg(feature = "debug")]
    CameraDecelerate,
    /// Open a window to inspect a frame.
    #[cfg(feature = "debug")]
    InspectFrame { measurement: FrameMeasurement },
}

impl From<InputEvent> for Event<ClientState> {
    fn from(custom_event: InputEvent) -> Self {
        Event::Application { custom_event }
    }
}

impl ClickHandler<ClientState> for InputEvent {
    fn handle_click(&self, _: &State<ClientState>, queue: &mut EventQueue<ClientState>) {
        queue.queue(self.clone());
    }
}

#[cfg(test)]
mod tests {
    use korangar_networking::InventoryItemDetails;
    use ragnarok_packets::{EquipPosition, EquippableItemFlags, InventoryIndex, ItemId, ItemOptions, RegularItemFlags};

    use super::{InventoryItemActivation, inventory_item_activation};
    use crate::world::ResourceMetadata;

    fn test_metadata() -> ResourceMetadata {
        ResourceMetadata {
            texture: None,
            name: String::new(),
        }
    }

    fn regular_item(
        index: InventoryIndex,
        item_type: u8,
        equipped_position: EquipPosition,
    ) -> korangar_networking::InventoryItem<ResourceMetadata> {
        regular_item_with_id(index, ItemId(1), item_type, equipped_position)
    }

    fn regular_item_with_id(
        index: InventoryIndex,
        item_id: ItemId,
        item_type: u8,
        equipped_position: EquipPosition,
    ) -> korangar_networking::InventoryItem<ResourceMetadata> {
        korangar_networking::InventoryItem {
            metadata: test_metadata(),
            index,
            item_id,
            item_type,
            slot: [0; 4],
            hire_expiration_date: 0,
            details: InventoryItemDetails::Regular {
                amount: 1,
                equipped_position,
                flags: RegularItemFlags::IDENTIFIED,
            },
        }
    }

    fn equippable_item(
        index: InventoryIndex,
        equip_position: EquipPosition,
        equipped_position: EquipPosition,
    ) -> korangar_networking::InventoryItem<ResourceMetadata> {
        korangar_networking::InventoryItem {
            metadata: test_metadata(),
            index,
            item_id: ItemId(1),
            item_type: 5,
            slot: [0; 4],
            hire_expiration_date: 0,
            details: InventoryItemDetails::Equippable {
                equip_position,
                equipped_position,
                bind_on_equip_type: 0,
                w_item_sprite_number: 0,
                option_count: 0,
                option_data: std::array::from_fn(|_| ItemOptions {
                    index: 0,
                    value: 0,
                    parameter: 0,
                }),
                refinement_level: 0,
                enchantment_level: 0,
                flags: EquippableItemFlags::IDENTIFIED,
            },
        }
    }

    #[test]
    fn inventory_item_activation_equips_unequipped_equippable() {
        let item = equippable_item(InventoryIndex(7), EquipPosition::RIGHT_HAND, EquipPosition::NONE);

        assert_eq!(
            inventory_item_activation(&item),
            Some(InventoryItemActivation::Equip {
                index: InventoryIndex(7),
                position: EquipPosition::RIGHT_HAND,
            })
        );
    }

    #[test]
    fn inventory_item_activation_unequips_equipped_equippable() {
        let item = equippable_item(InventoryIndex(8), EquipPosition::RIGHT_HAND, EquipPosition::RIGHT_HAND);

        assert_eq!(
            inventory_item_activation(&item),
            Some(InventoryItemActivation::Unequip { index: InventoryIndex(8) })
        );
    }

    #[test]
    fn inventory_item_activation_equips_ammunition() {
        let item = regular_item(InventoryIndex(9), 10, EquipPosition::NONE);

        assert_eq!(
            inventory_item_activation(&item),
            Some(InventoryItemActivation::Equip {
                index: InventoryIndex(9),
                position: EquipPosition::AMMO,
            })
        );
    }

    #[test]
    fn inventory_item_activation_equips_known_arrow_even_with_unexpected_item_type() {
        let item = regular_item_with_id(InventoryIndex(9), ItemId(1750), 0, EquipPosition::NONE);

        assert_eq!(
            inventory_item_activation(&item),
            Some(InventoryItemActivation::Equip {
                index: InventoryIndex(9),
                position: EquipPosition::AMMO,
            })
        );
    }

    #[test]
    fn inventory_item_activation_unequips_equipped_ammunition_even_with_unexpected_item_type() {
        let item = regular_item_with_id(InventoryIndex(9), ItemId(1750), 0, EquipPosition::AMMO);

        assert_eq!(
            inventory_item_activation(&item),
            Some(InventoryItemActivation::Unequip { index: InventoryIndex(9) })
        );
    }

    #[test]
    fn inventory_item_activation_uses_consumable() {
        let item = regular_item(InventoryIndex(10), 2, EquipPosition::NONE);

        assert_eq!(
            inventory_item_activation(&item),
            Some(InventoryItemActivation::Use { index: InventoryIndex(10) })
        );
    }
}
