use korangar_components::item_box;
use korangar_interface::components::text_box::DefaultHandler;
use korangar_interface::element::StateElement;
use korangar_interface::window::{CustomWindow, Window};
use korangar_networking::{InventoryItem, InventoryItemDetails};
use rust_state::{Path, PathExt, RustState, Selector, State};

use crate::ItemSource;
use crate::interface::windows::WindowClass;
use crate::loaders::OverflowBehavior;
use crate::state::localization::LocalizationPathExt;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, ClientStatePathExt, client_state};
use crate::world::ResourceMetadata;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, RustState, StateElement)]
pub enum InventoryFilter {
    #[default]
    All,
    Equipment,
    Consumable,
    Ammo,
    Etc,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, RustState, StateElement)]
pub enum InventorySortMode {
    #[default]
    Slot,
    Name,
    Amount,
}

#[derive(Default, RustState, StateElement)]
pub struct InventoryWindowState {
    search_text: String,
    filter: InventoryFilter,
    sort_mode: InventorySortMode,
}

pub(crate) fn visible_inventory_indices(items: &[InventoryItem<ResourceMetadata>], state: &InventoryWindowState) -> Vec<usize> {
    let search_text = state.search_text.trim().to_lowercase();
    let mut indices = items
        .iter()
        .enumerate()
        .filter(|(_, item)| item_matches_filter(item, state.filter))
        .filter(|(_, item)| search_text.is_empty() || item.metadata.name.to_lowercase().contains(&search_text))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();

    match state.sort_mode {
        InventorySortMode::Slot => {}
        InventorySortMode::Name => indices.sort_by(|left, right| {
            items[*left]
                .metadata
                .name
                .to_lowercase()
                .cmp(&items[*right].metadata.name.to_lowercase())
                .then_with(|| items[*left].index.0.cmp(&items[*right].index.0))
        }),
        InventorySortMode::Amount => indices.sort_by(|left, right| {
            item_amount(&items[*right])
                .cmp(&item_amount(&items[*left]))
                .then_with(|| items[*left].index.0.cmp(&items[*right].index.0))
        }),
    }

    indices
}

fn item_matches_filter(item: &InventoryItem<ResourceMetadata>, filter: InventoryFilter) -> bool {
    match filter {
        InventoryFilter::All => true,
        InventoryFilter::Equipment => matches!(item.details, InventoryItemDetails::Equippable { .. }),
        InventoryFilter::Consumable => matches!(item.item_type, 0 | 2 | 11),
        InventoryFilter::Ammo => item.item_type == 10,
        InventoryFilter::Etc => {
            !matches!(item.details, InventoryItemDetails::Equippable { .. }) && !matches!(item.item_type, 0 | 2 | 10 | 11)
        }
    }
}

fn item_amount(item: &InventoryItem<ResourceMetadata>) -> u16 {
    match item.details {
        InventoryItemDetails::Regular { amount, .. } => amount,
        InventoryItemDetails::Equippable { .. } => 1,
    }
}

#[derive(Clone, Copy)]
struct FilteredInventoryPath<I, S> {
    items_path: I,
    window_state_path: S,
    slot_index: usize,
}

impl<I, S> FilteredInventoryPath<I, S> {
    fn new(items_path: I, window_state_path: S, slot_index: usize) -> Self {
        Self {
            items_path,
            window_state_path,
            slot_index,
        }
    }

    fn item_index(&self, state: &ClientState) -> Option<usize>
    where
        I: Path<ClientState, Vec<InventoryItem<ResourceMetadata>>>,
        S: Path<ClientState, InventoryWindowState>,
    {
        let items = self.items_path.follow_safe(state);
        let window_state = self.window_state_path.follow_safe(state);

        visible_inventory_indices(items, window_state).get(self.slot_index).copied()
    }
}

impl<I, S> Path<ClientState, InventoryItem<ResourceMetadata>, false> for FilteredInventoryPath<I, S>
where
    I: Path<ClientState, Vec<InventoryItem<ResourceMetadata>>>,
    S: Path<ClientState, InventoryWindowState>,
{
    fn follow<'a>(&self, state: &'a ClientState) -> Option<&'a InventoryItem<ResourceMetadata>> {
        let item_index = self.item_index(state)?;

        self.items_path.follow_safe(state).get(item_index)
    }

    fn follow_mut<'a>(&self, state: &'a mut ClientState) -> Option<&'a mut InventoryItem<ResourceMetadata>> {
        let item_index = self.item_index(state)?;

        self.items_path.follow_mut_safe(state).get_mut(item_index)
    }
}

impl<I, S> Selector<ClientState, InventoryItem<ResourceMetadata>, false> for FilteredInventoryPath<I, S>
where
    I: Path<ClientState, Vec<InventoryItem<ResourceMetadata>>>,
    S: Path<ClientState, InventoryWindowState>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a InventoryItem<ResourceMetadata>> {
        self.follow(state)
    }
}

pub struct InventoryWindow<S, P> {
    window_state_path: S,
    items_path: P,
}

impl<S, P> InventoryWindow<S, P> {
    pub fn new(window_state_path: S, items_path: P) -> Self {
        Self {
            window_state_path,
            items_path,
        }
    }
}

impl<S, P> CustomWindow<ClientState> for InventoryWindow<S, P>
where
    S: Path<ClientState, InventoryWindowState> + Copy,
    P: Path<ClientState, Vec<InventoryItem<ResourceMetadata>>> + Copy,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::Inventory)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        struct InventorySearchTextBox;

        const MAXIMUM_SEARCH_LENGTH: usize = 32;
        const INVENTORY_ROWS: usize = 4;
        const INVENTORY_COLUMNS: usize = 10;

        let search_path = self.window_state_path.search_text();
        let filter_path = self.window_state_path.filter();
        let sort_mode_path = self.window_state_path.sort_mode();

        let filter_button = move |text: &'static str, filter: InventoryFilter| {
            button! {
                text: text,
                tooltip: text,
                event: move |state: &State<ClientState>, _: &mut EventQueue<ClientState>| {
                    state.update_value(filter_path, filter);
                },
            }
        };

        let sort_button = move |text: &'static str, sort_mode: InventorySortMode| {
            button! {
                text: text,
                tooltip: text,
                event: move |state: &State<ClientState>, _: &mut EventQueue<ClientState>| {
                    state.update_value(sort_mode_path, sort_mode);
                },
            }
        };

        window! {
            title: client_state().localization().inventory_window_title(),
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: true,
            elements: (
                text_box! {
                    ghost_text: "Search",
                    state: search_path,
                    input_handler: DefaultHandler::<_, _, MAXIMUM_SEARCH_LENGTH>::new(search_path, |_: &State<ClientState>, _: &mut EventQueue<ClientState>| {}),
                    focus_id: InventorySearchTextBox,
                    overflow_behavior: OverflowBehavior::Shrink,
                },
                split! {
                    gaps: theme().window().gaps(),
                    children: (
                        filter_button("All", InventoryFilter::All),
                        filter_button("Equip", InventoryFilter::Equipment),
                        filter_button("Use", InventoryFilter::Consumable),
                        filter_button("Ammo", InventoryFilter::Ammo),
                        filter_button("Etc", InventoryFilter::Etc),
                    ),
                },
                split! {
                    gaps: theme().window().gaps(),
                    children: (
                        sort_button("Slot", InventorySortMode::Slot),
                        sort_button("Name", InventorySortMode::Name),
                        sort_button("Amount", InventorySortMode::Amount),
                    ),
                },
                std::array::from_fn::<_, INVENTORY_ROWS, _>(|row| {
                    split! {
                        gaps: theme().window().gaps(),
                        children: std::array::from_fn::<_, INVENTORY_COLUMNS, _>(|column| {
                            let slot_index = row * INVENTORY_COLUMNS + column;
                            let path = FilteredInventoryPath::new(self.items_path, self.window_state_path, slot_index);

                            item_box! {
                                item_path: path,
                                source: ItemSource::Inventory,
                            }
                        }),
                    }
                }),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use korangar_networking::{InventoryItem, InventoryItemDetails};
    use ragnarok_packets::{EquipPosition, EquippableItemFlags, InventoryIndex, ItemId, ItemOptions, RegularItemFlags};

    use super::{InventoryFilter, InventorySortMode, InventoryWindowState, visible_inventory_indices};
    use crate::world::ResourceMetadata;

    fn metadata(name: &str) -> ResourceMetadata {
        ResourceMetadata {
            texture: None,
            name: name.to_string(),
        }
    }

    fn regular(index: u16, item_id: u32, item_type: u8, name: &str, amount: u16) -> InventoryItem<ResourceMetadata> {
        InventoryItem {
            metadata: metadata(name),
            index: InventoryIndex(index),
            item_id: ItemId(item_id),
            item_type,
            slot: [0; 4],
            hire_expiration_date: 0,
            details: InventoryItemDetails::Regular {
                amount,
                equipped_position: EquipPosition::NONE,
                flags: RegularItemFlags::IDENTIFIED,
            },
        }
    }

    fn equippable(index: u16, item_id: u32, name: &str) -> InventoryItem<ResourceMetadata> {
        InventoryItem {
            metadata: metadata(name),
            index: InventoryIndex(index),
            item_id: ItemId(item_id),
            item_type: 5,
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
                flags: EquippableItemFlags::IDENTIFIED,
            },
        }
    }

    #[test]
    fn visible_inventory_indices_filter_equipment_and_search_name() {
        let items = vec![
            regular(0, 501, 0, "Red Potion", 2),
            equippable(1, 1101, "Sword"),
            regular(2, 1750, 10, "Arrow", 100),
        ];
        let state = InventoryWindowState {
            search_text: "swo".to_string(),
            filter: InventoryFilter::Equipment,
            sort_mode: InventorySortMode::Slot,
        };

        assert_eq!(visible_inventory_indices(&items, &state), vec![1]);
    }

    #[test]
    fn visible_inventory_indices_sort_by_name() {
        let items = vec![
            regular(0, 501, 0, "Red Potion", 2),
            equippable(1, 1101, "Sword"),
            regular(2, 1750, 10, "Arrow", 100),
        ];
        let state = InventoryWindowState {
            search_text: String::new(),
            filter: InventoryFilter::All,
            sort_mode: InventorySortMode::Name,
        };

        assert_eq!(visible_inventory_indices(&items, &state), vec![2, 0, 1]);
    }
}
