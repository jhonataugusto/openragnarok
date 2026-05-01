use korangar_interface::MouseMode;
use korangar_interface::element::store::{ElementStore, ElementStoreMut};
use korangar_interface::element::{BaseLayoutInfo, Element};
use korangar_interface::event::{ClickHandler, DropHandler, Event, EventQueue};
use korangar_interface::layout::area::Area;
use korangar_interface::layout::tooltip::TooltipExt;
use korangar_interface::layout::{MouseButton, Resolvers, WindowLayout, with_single_resolver};
use korangar_interface::prelude::{HorizontalAlignment, VerticalAlignment};
use korangar_networking::{InventoryItem, InventoryItemDetails};
use rust_state::{Path, State};

use crate::graphics::{Color, CornerDiameter, ShadowPadding};
use crate::input::{InputEvent, MouseInputMode};
use crate::interface::resource::ItemSource;
use crate::loaders::{FontSize, OverflowBehavior};
use crate::renderer::LayoutExt;
use crate::state::inventory::InventoryPathExt;
use crate::state::{ClientState, ClientStatePathExt, client_state};
use crate::world::ResourceMetadata;

#[derive(Default)]
struct AmountDisplay {
    amount: u16,
    string: Option<String>,
}

impl AmountDisplay {
    fn update(&mut self, new_amount: u16) {
        if self.string.is_none() || self.amount != new_amount {
            self.string = Some(new_amount.to_string());
            self.amount = new_amount;
        }
    }
}

struct ItemBoxHandler<P> {
    item_path: P,
    source: ItemSource,
    click_action: ItemBoxClickAction,
}

#[derive(Clone, Copy)]
enum ItemBoxClickAction {
    Move,
    Activate,
}

impl<P> ItemBoxHandler<P> {
    fn new(item_path: P, source: ItemSource, click_action: ItemBoxClickAction) -> Self {
        Self {
            item_path,
            source,
            click_action,
        }
    }
}

impl<P> ClickHandler<ClientState> for ItemBoxHandler<P>
where
    P: Path<ClientState, InventoryItem<ResourceMetadata>, false>,
{
    fn handle_click(&self, state: &State<ClientState>, queue: &mut EventQueue<ClientState>) {
        // Unwrapping here is fine since we only register the handler if the slot has a
        // item.
        let item = state.try_get(&self.item_path).unwrap().clone();

        match self.click_action {
            ItemBoxClickAction::Move => queue.queue(Event::SetMouseMode {
                mouse_mode: MouseMode::Custom {
                    mode: MouseInputMode::MoveItem { item, source: self.source },
                },
            }),
            ItemBoxClickAction::Activate => queue.queue(InputEvent::ActivateInventoryItem { item }),
        }
    }
}

impl<P> DropHandler<ClientState> for ItemBoxHandler<P>
where
    P: Path<ClientState, InventoryItem<ResourceMetadata>, false>,
{
    fn handle_drop(&self, _: &State<ClientState>, queue: &mut EventQueue<ClientState>, mouse_mode: &MouseMode<ClientState>) {
        if let MouseMode::Custom {
            mode: MouseInputMode::MoveItem { source, item },
        } = mouse_mode
        {
            queue.queue(InputEvent::MoveItem {
                source: *source,
                destination: self.source,
                item: item.clone(),
            });
        }
    }
}

pub struct ItemBox<A> {
    item_path: A,
    move_handler: ItemBoxHandler<A>,
    activate_handler: ItemBoxHandler<A>,
    amount_display: AmountDisplay,
    refinement_display: AmountDisplay,
    tooltip_text: String,
}

impl<A> ItemBox<A>
where
    A: Copy,
{
    /// This function is supposed to be called from a component macro
    /// and not intended to be called manually.
    #[inline(always)]
    pub fn component_new(item_path: A, source: ItemSource) -> Self {
        Self {
            item_path,
            move_handler: ItemBoxHandler::new(item_path, source, ItemBoxClickAction::Move),
            activate_handler: ItemBoxHandler::new(item_path, source, ItemBoxClickAction::Activate),
            amount_display: AmountDisplay::default(),
            refinement_display: AmountDisplay::default(),
            tooltip_text: String::new(),
        }
    }
}

impl<A> Element<ClientState> for ItemBox<A>
where
    A: Path<ClientState, InventoryItem<ResourceMetadata>, false>,
{
    type LayoutInfo = BaseLayoutInfo;

    fn create_layout_info(
        &mut self,
        state: &State<ClientState>,
        _: ElementStoreMut,
        resolvers: &mut dyn Resolvers<ClientState>,
    ) -> Self::LayoutInfo {
        with_single_resolver(resolvers, |resolver| {
            let area = resolver.with_height(40.0);

            if let Some(item) = state.try_get(&self.item_path) {
                self.tooltip_text = item_tooltip_text(item, state.get(&client_state().inventory().items()));

                if item.metadata.texture.as_ref().is_some()
                    && let InventoryItemDetails::Regular { amount, .. } = &item.details
                {
                    self.amount_display.update(*amount);
                }

                if let InventoryItemDetails::Equippable { refinement_level, .. } = &item.details
                    && *refinement_level > 0
                {
                    self.refinement_display.update(*refinement_level as u16);
                }
            }

            Self::LayoutInfo { area }
        })
    }

    fn lay_out<'a>(
        &'a self,
        state: &'a State<ClientState>,
        _: ElementStore<'a>,
        layout_info: &'a Self::LayoutInfo,
        layout: &mut WindowLayout<'a, ClientState>,
    ) {
        let (is_hovered, background_color) = match layout.get_mouse_mode() {
            MouseMode::Custom {
                mode: MouseInputMode::MoveItem { .. },
            } => match layout_info.area.check().any_mouse_mode().run(layout) {
                true => {
                    // Since we are not in default mouse mode we need to mark the window as
                    // hovered.
                    layout.set_hovered();

                    (true, Color::rgb_u8(80, 180, 180))
                }
                false => (false, Color::rgb_u8(180, 180, 80)),
            },
            _ => match layout_info.area.check().run(layout) {
                true => (true, Color::rgb_u8(60, 60, 60)),
                false => (false, Color::rgb_u8(40, 40, 40)),
            },
        };

        layout.add_rectangle(
            layout_info.area,
            CornerDiameter::uniform(20.0),
            background_color,
            Color::rgba_u8(0, 0, 0, 100),
            ShadowPadding::diagonal(2.0, 5.0),
        );

        if is_hovered {
            layout.register_drop_handler(&self.move_handler);
        }

        if let Some(item) = state.try_get(&self.item_path)
            && let Some(texture) = item.metadata.texture.as_ref()
        {
            let texture_size = layout_info.area.width.min(layout_info.area.height);
            let texture_area = Area {
                left: layout_info.area.left + (layout_info.area.width - texture_size) / 2.0,
                top: layout_info.area.top + (layout_info.area.height - texture_size) / 2.0,
                width: texture_size,
                height: texture_size,
            };

            layout.add_texture(texture_area, texture.clone(), Color::WHITE, false);

            if is_hovered {
                layout.register_click_handler(MouseButton::Left, &self.move_handler);
                layout.register_click_handler(MouseButton::DoubleLeft, &self.activate_handler);
                layout.add_tooltip(&self.tooltip_text, self.tooltip_id());
            }

            if item_is_equipped(item) {
                layout.add_text(
                    layout_info.area,
                    "E",
                    FontSize(12.0),
                    Color::rgb_u8(120, 255, 180),
                    Color::rgb_u8(10, 40, 20),
                    HorizontalAlignment::Left { offset: 3.0, border: 3.0 },
                    VerticalAlignment::Top { offset: 3.0 },
                    OverflowBehavior::Shrink,
                );
            }

            if let InventoryItemDetails::Equippable { refinement_level, .. } = item.details
                && refinement_level > 0
            {
                let refine_text = self.refinement_display.string.as_ref().unwrap();

                layout.add_text(
                    layout_info.area,
                    refine_text,
                    FontSize(12.0),
                    Color::rgb_u8(255, 220, 120),
                    Color::rgb_u8(60, 40, 10),
                    HorizontalAlignment::Right { offset: 3.0, border: 3.0 },
                    VerticalAlignment::Top { offset: 3.0 },
                    OverflowBehavior::Shrink,
                );
            }

            if matches!(item.details, InventoryItemDetails::Regular { .. }) {
                layout.add_text(
                    layout_info.area,
                    self.amount_display.string.as_ref().unwrap(),
                    // TODO: Put this in the theme
                    FontSize(12.0),
                    // TODO: Put this in the theme
                    Color::rgb_u8(255, 200, 255),
                    // TODO: Put this in the theme
                    Color::rgb_u8(255, 160, 60),
                    // TODO: Put this in the theme
                    HorizontalAlignment::Right { offset: 3.0, border: 3.0 },
                    // TODO: Put this in the theme
                    VerticalAlignment::Bottom { offset: 3.0 },
                    OverflowBehavior::Shrink,
                );
            }
        }
    }
}

fn item_is_equipped(item: &InventoryItem<ResourceMetadata>) -> bool {
    match item.details {
        InventoryItemDetails::Regular { equipped_position, .. } => !equipped_position.is_empty(),
        InventoryItemDetails::Equippable { equipped_position, .. } => !equipped_position.is_empty(),
    }
}

fn item_tooltip_text(item: &InventoryItem<ResourceMetadata>, inventory: &[InventoryItem<ResourceMetadata>]) -> String {
    let mut lines = Vec::new();
    let name = match item.metadata.name.is_empty() {
        true => format!("Item {}", item.item_id.0),
        false => item.metadata.name.clone(),
    };

    lines.push(name);
    lines.push(format!("ID: {} | Tipo: {}", item.item_id.0, item.item_type));

    match &item.details {
        InventoryItemDetails::Regular {
            amount, equipped_position, ..
        } => {
            lines.push(format!("Qtd: {amount}"));
            if !equipped_position.is_empty() {
                lines.push("Equipado".to_string());
            }
        }
        InventoryItemDetails::Equippable {
            equip_position,
            equipped_position,
            refinement_level,
            w_item_sprite_number,
            ..
        } => {
            lines.push(format!("Slot: {}", equip_position.bits()));
            if *refinement_level > 0 {
                lines.push(format!("Refino: +{refinement_level}"));
            }
            if *w_item_sprite_number > 0 {
                lines.push(format!("Visual: {w_item_sprite_number}"));
            }
            if !equipped_position.is_empty() {
                lines.push("Equipado".to_string());
            } else if let Some(current_item) = equipped_item_for_slot(inventory, item.index, *equip_position) {
                let current_name = match current_item.metadata.name.is_empty() {
                    true => format!("Item {}", current_item.item_id.0),
                    false => current_item.metadata.name.clone(),
                };
                let current_refinement = match current_item.details {
                    InventoryItemDetails::Equippable { refinement_level, .. } if refinement_level > 0 => format!(" +{refinement_level}"),
                    _ => String::new(),
                };

                lines.push(format!("Atual: {current_name}{current_refinement}"));
            }
        }
    }

    lines.join("\n")
}

fn equipped_item_for_slot(
    inventory: &[InventoryItem<ResourceMetadata>],
    item_index: ragnarok_packets::InventoryIndex,
    equip_position: ragnarok_packets::EquipPosition,
) -> Option<&InventoryItem<ResourceMetadata>> {
    inventory.iter().find(|current_item| {
        current_item.index != item_index
            && match current_item.details {
                InventoryItemDetails::Equippable { equipped_position, .. } => equipped_position.intersects(equip_position),
                InventoryItemDetails::Regular { .. } => false,
            }
    })
}
