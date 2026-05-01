use std::sync::Arc;

use korangar_interface::element::StateElement;
use korangar_networking::{InventoryItem, InventoryItemDetails, NoMetadata};
use ragnarok_packets::{EquipPosition, InventoryIndex, ItemId};
use rust_state::RustState;

use crate::graphics::Texture;
use crate::loaders::AsyncLoader;
use crate::world::ResourceMetadata;

#[derive(Default, RustState, StateElement)]
pub struct Inventory {
    // TODO: Unhide this.
    #[hidden_element]
    items: Vec<InventoryItem<ResourceMetadata>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InventoryItemRemoval {
    Missing,
    Updated,
    Removed { was_equipped_ammunition: bool },
}

impl Inventory {
    pub fn fill(&mut self, async_loader: &AsyncLoader, items: Vec<InventoryItem<NoMetadata>>) {
        self.items = items
            .into_iter()
            .map(|item| async_loader.request_inventory_item_metadata_load(item))
            .collect();
    }

    pub fn add_item(&mut self, async_loader: &AsyncLoader, item: InventoryItem<NoMetadata>) {
        if let Some(found_item) = self.items.iter_mut().find(|inventory_item| inventory_item.index == item.index) {
            if !try_add_regular_item_amount(&mut found_item.details, &item.details) {
                *found_item = async_loader.request_inventory_item_metadata_load(item);
            }
        } else {
            let item = async_loader.request_inventory_item_metadata_load(item);

            self.items.push(item);
        }
    }

    pub fn update_item_sprite(&mut self, item_id: ItemId, texture: Arc<Texture>) {
        self.items.iter_mut().filter(|item| item.item_id == item_id).for_each(|item| {
            item.metadata.texture = Some(texture.clone());
        });
    }

    pub fn remove_item(&mut self, index: InventoryIndex, remove_amount: u16) -> InventoryItemRemoval {
        let Some(position) = self.items.iter().position(|item| item.index == index) else {
            return InventoryItemRemoval::Missing;
        };

        if let InventoryItemDetails::Regular { amount, .. } = &mut self.items[position].details
            && *amount > remove_amount
        {
            *amount -= remove_amount;
            return InventoryItemRemoval::Updated;
        }

        let was_equipped_ammunition = matches!(
            self.items[position].details,
            InventoryItemDetails::Regular {
                equipped_position,
                ..
            } if equipped_position.intersects(EquipPosition::AMMO)
        );

        self.items.remove(position);

        InventoryItemRemoval::Removed { was_equipped_ammunition }
    }

    pub fn update_equipped_position(&mut self, index: InventoryIndex, new_equipped_position: EquipPosition) {
        let Some(position) = self.items.iter().position(|item| item.index == index) else {
            return;
        };

        if new_equipped_position.intersects(EquipPosition::AMMO) {
            self.items.iter_mut().for_each(|item| {
                if item.index == index {
                    return;
                }

                if let InventoryItemDetails::Regular { equipped_position, .. } = &mut item.details
                    && equipped_position.intersects(EquipPosition::AMMO)
                {
                    *equipped_position = EquipPosition::NONE;
                }
            });
        }

        let item = &mut self.items[position];

        match &mut item.details {
            InventoryItemDetails::Equippable { equipped_position, .. } => {
                *equipped_position = new_equipped_position;
            }
            InventoryItemDetails::Regular { equipped_position, .. }
                if new_equipped_position.intersects(EquipPosition::AMMO) || equipped_position.intersects(EquipPosition::AMMO) =>
            {
                *equipped_position = new_equipped_position;
            }
            InventoryItemDetails::Regular { .. } => {}
        };
    }

    pub fn clear_equipped_ammunition(&mut self) {
        self.items.iter_mut().for_each(|item| {
            if let InventoryItemDetails::Regular { equipped_position, .. } = &mut item.details
                && equipped_position.intersects(EquipPosition::AMMO)
            {
                *equipped_position = EquipPosition::NONE;
            }
        });
    }

    pub fn clear_equipped_ammunition_item(&mut self, index: InventoryIndex) {
        let Some(item) = self.items.iter_mut().find(|item| item.index == index) else {
            return;
        };

        if let InventoryItemDetails::Regular { equipped_position, .. } = &mut item.details
            && equipped_position.intersects(EquipPosition::AMMO)
        {
            *equipped_position = EquipPosition::NONE;
        }
    }

    pub fn equippable_item_id(&self, index: InventoryIndex) -> Option<ItemId> {
        let item = self.items.iter().find(|item| item.index == index)?;

        matches!(item.details, InventoryItemDetails::Equippable { .. }).then_some(item.item_id)
    }
}

fn try_add_regular_item_amount(existing: &mut InventoryItemDetails, added: &InventoryItemDetails) -> bool {
    let (InventoryItemDetails::Regular { amount, .. }, InventoryItemDetails::Regular { amount: added_amount, .. }) = (existing, added)
    else {
        return false;
    };

    *amount += added_amount;
    true
}

#[cfg(test)]
mod tests {
    use korangar_networking::InventoryItem;
    use ragnarok_packets::{EquippableItemFlags, ItemOptions, RegularItemFlags};

    use super::*;

    fn test_metadata() -> ResourceMetadata {
        ResourceMetadata {
            texture: None,
            name: String::new(),
        }
    }

    fn test_equippable_item(index: InventoryIndex, item_id: ItemId) -> InventoryItem<ResourceMetadata> {
        InventoryItem {
            metadata: test_metadata(),
            index,
            item_id,
            item_type: 4,
            slot: [0; 4],
            hire_expiration_date: 0,
            details: InventoryItemDetails::Equippable {
                equip_position: EquipPosition::RIGHT_HAND,
                equipped_position: EquipPosition::NONE,
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
                flags: EquippableItemFlags::empty(),
            },
        }
    }

    fn test_regular_item(index: InventoryIndex, item_id: ItemId, equipped_position: EquipPosition) -> InventoryItem<ResourceMetadata> {
        test_regular_item_with_amount(index, item_id, 1, equipped_position)
    }

    fn test_regular_item_with_amount(
        index: InventoryIndex,
        item_id: ItemId,
        amount: u16,
        equipped_position: EquipPosition,
    ) -> InventoryItem<ResourceMetadata> {
        InventoryItem {
            metadata: test_metadata(),
            index,
            item_id,
            item_type: 10,
            slot: [0; 4],
            hire_expiration_date: 0,
            details: InventoryItemDetails::Regular {
                amount,
                equipped_position,
                flags: RegularItemFlags::empty(),
            },
        }
    }

    #[test]
    fn equippable_item_id_returns_the_item_id_for_equipment() {
        let inventory = Inventory {
            items: vec![InventoryItem {
                metadata: test_metadata(),
                index: InventoryIndex(2),
                item_id: ItemId(1201),
                item_type: 4,
                slot: [0; 4],
                hire_expiration_date: 0,
                details: InventoryItemDetails::Equippable {
                    equip_position: EquipPosition::RIGHT_HAND,
                    equipped_position: EquipPosition::NONE,
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
                    flags: EquippableItemFlags::empty(),
                },
            }],
        };

        assert_eq!(inventory.equippable_item_id(InventoryIndex(2)), Some(ItemId(1201)));
    }

    #[test]
    fn equippable_item_id_ignores_regular_items() {
        let inventory = Inventory {
            items: vec![InventoryItem {
                metadata: test_metadata(),
                index: InventoryIndex(2),
                item_id: ItemId(501),
                item_type: 0,
                slot: [0; 4],
                hire_expiration_date: 0,
                details: InventoryItemDetails::Regular {
                    amount: 1,
                    equipped_position: EquipPosition::NONE,
                    flags: RegularItemFlags::empty(),
                },
            }],
        };

        assert_eq!(inventory.equippable_item_id(InventoryIndex(2)), None);
    }

    #[test]
    fn non_regular_items_do_not_use_stack_amount_merge() {
        let mut existing = test_equippable_item(InventoryIndex(2), ItemId(1201)).details;
        let added = test_equippable_item(InventoryIndex(2), ItemId(1701)).details;

        assert!(!try_add_regular_item_amount(&mut existing, &added));
    }

    #[test]
    fn remove_item_ignores_missing_index() {
        let mut inventory = Inventory {
            items: vec![test_regular_item(InventoryIndex(9), ItemId(1750), EquipPosition::NONE)],
        };

        let removal = inventory.remove_item(InventoryIndex(10), 1);

        assert_eq!(removal, InventoryItemRemoval::Missing);
        assert_eq!(inventory.items.len(), 1);
    }

    #[test]
    fn remove_item_decrements_regular_stack() {
        let mut inventory = Inventory {
            items: vec![test_regular_item_with_amount(
                InventoryIndex(9),
                ItemId(1750),
                3,
                EquipPosition::AMMO,
            )],
        };

        let removal = inventory.remove_item(InventoryIndex(9), 1);

        assert_eq!(removal, InventoryItemRemoval::Updated);
        let InventoryItemDetails::Regular {
            amount, equipped_position, ..
        } = inventory.items[0].details
        else {
            panic!("expected regular item");
        };

        assert_eq!(amount, 2);
        assert_eq!(equipped_position, EquipPosition::AMMO);
    }

    #[test]
    fn remove_item_reports_equipped_ammunition_when_stack_is_exhausted() {
        let mut inventory = Inventory {
            items: vec![test_regular_item(InventoryIndex(9), ItemId(1750), EquipPosition::AMMO)],
        };

        let removal = inventory.remove_item(InventoryIndex(9), 1);

        assert_eq!(removal, InventoryItemRemoval::Removed {
            was_equipped_ammunition: true
        });
        assert!(inventory.items.is_empty());
    }

    #[test]
    fn remove_item_reports_non_ammunition_removal() {
        let mut inventory = Inventory {
            items: vec![test_equippable_item(InventoryIndex(2), ItemId(1701))],
        };

        let removal = inventory.remove_item(InventoryIndex(2), 1);

        assert_eq!(removal, InventoryItemRemoval::Removed {
            was_equipped_ammunition: false
        });
        assert!(inventory.items.is_empty());
    }

    #[test]
    fn update_equipped_position_marks_regular_ammunition_as_equipped() {
        let mut inventory = Inventory {
            items: vec![test_regular_item(InventoryIndex(9), ItemId(1750), EquipPosition::NONE)],
        };

        inventory.update_equipped_position(InventoryIndex(9), EquipPosition::AMMO);

        let InventoryItemDetails::Regular { equipped_position, .. } = inventory.items[0].details else {
            panic!("expected regular item");
        };

        assert_eq!(equipped_position, EquipPosition::AMMO);
    }

    #[test]
    fn update_equipped_position_clears_previous_ammunition() {
        let mut inventory = Inventory {
            items: vec![
                test_regular_item(InventoryIndex(9), ItemId(1750), EquipPosition::AMMO),
                test_regular_item(InventoryIndex(10), ItemId(1751), EquipPosition::NONE),
            ],
        };

        inventory.update_equipped_position(InventoryIndex(10), EquipPosition::AMMO);

        let InventoryItemDetails::Regular {
            equipped_position: first_position,
            ..
        } = inventory.items[0].details
        else {
            panic!("expected regular item");
        };
        let InventoryItemDetails::Regular {
            equipped_position: second_position,
            ..
        } = inventory.items[1].details
        else {
            panic!("expected regular item");
        };

        assert_eq!(first_position, EquipPosition::NONE);
        assert_eq!(second_position, EquipPosition::AMMO);
    }

    #[test]
    fn update_equipped_position_ignores_missing_index() {
        let mut inventory = Inventory {
            items: vec![test_regular_item(InventoryIndex(9), ItemId(1750), EquipPosition::AMMO)],
        };

        inventory.update_equipped_position(InventoryIndex(10), EquipPosition::AMMO);

        let InventoryItemDetails::Regular { equipped_position, .. } = inventory.items[0].details else {
            panic!("expected regular item");
        };

        assert_eq!(equipped_position, EquipPosition::AMMO);
    }

    #[test]
    fn clear_equipped_ammunition_clears_all_regular_ammo() {
        let mut inventory = Inventory {
            items: vec![
                test_regular_item(InventoryIndex(9), ItemId(1750), EquipPosition::AMMO),
                test_regular_item(InventoryIndex(10), ItemId(1751), EquipPosition::AMMO),
            ],
        };

        inventory.clear_equipped_ammunition();

        for item in inventory.items {
            let InventoryItemDetails::Regular { equipped_position, .. } = item.details else {
                panic!("expected regular item");
            };

            assert_eq!(equipped_position, EquipPosition::NONE);
        }
    }

    #[test]
    fn clear_equipped_ammunition_item_only_clears_matching_regular_ammo() {
        let mut inventory = Inventory {
            items: vec![
                test_regular_item(InventoryIndex(9), ItemId(1750), EquipPosition::AMMO),
                test_regular_item(InventoryIndex(10), ItemId(1751), EquipPosition::AMMO),
            ],
        };

        inventory.clear_equipped_ammunition_item(InventoryIndex(10));

        let InventoryItemDetails::Regular {
            equipped_position: first_position,
            ..
        } = inventory.items[0].details
        else {
            panic!("expected regular item");
        };
        let InventoryItemDetails::Regular {
            equipped_position: second_position,
            ..
        } = inventory.items[1].details
        else {
            panic!("expected regular item");
        };

        assert_eq!(first_position, EquipPosition::AMMO);
        assert_eq!(second_position, EquipPosition::NONE);
    }
}
