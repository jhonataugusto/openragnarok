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

impl Inventory {
    pub fn fill(&mut self, async_loader: &AsyncLoader, items: Vec<InventoryItem<NoMetadata>>) {
        self.items = items
            .into_iter()
            .map(|item| async_loader.request_inventory_item_metadata_load(item))
            .collect();
    }

    pub fn add_item(&mut self, async_loader: &AsyncLoader, item: InventoryItem<NoMetadata>) {
        if let Some(found_item) = self.items.iter_mut().find(|inventory_item| inventory_item.index == item.index) {
            let InventoryItemDetails::Regular { amount, .. } = &mut found_item.details else {
                panic!();
            };

            let InventoryItemDetails::Regular { amount: added_amount, .. } = item.details else {
                panic!();
            };

            *amount += added_amount;
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

    pub fn remove_item(&mut self, index: InventoryIndex, remove_amount: u16) {
        let position = self
            .items
            .iter()
            .position(|item| item.index == index)
            .expect("item not in inventory");

        if let InventoryItemDetails::Regular { amount, .. } = &mut self.items[position].details
            && *amount > remove_amount
        {
            *amount -= remove_amount;
            return;
        }

        self.items.remove(position);
    }

    pub fn update_equipped_position(&mut self, index: InventoryIndex, new_equipped_position: EquipPosition) {
        let item = self.items.iter_mut().find(|item| item.index == index).unwrap();

        let InventoryItemDetails::Equippable { equipped_position, .. } = &mut item.details else {
            // This can happen for ammunition for example.
            return;
        };

        *equipped_position = new_equipped_position;
    }

    pub fn equippable_item_id(&self, index: InventoryIndex) -> Option<ItemId> {
        let item = self.items.iter().find(|item| item.index == index)?;

        matches!(item.details, InventoryItemDetails::Equippable { .. }).then_some(item.item_id)
    }
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
}
