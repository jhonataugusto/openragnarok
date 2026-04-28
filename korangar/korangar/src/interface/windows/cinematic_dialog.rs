use std::cell::UnsafeCell;

use korangar_interface::element::store::ElementStoreMut;
use korangar_interface::element::{Element, ElementBox, ErasedElement, StateElement};
use korangar_interface::layout::{Resolvers, with_single_resolver};
use korangar_interface::window::{CustomWindow, Window};
use ragnarok_packets::EntityId;
use rust_state::{Path, PathExt, RustState, Selector, State};

use super::WindowClass;
use crate::input::InputEvent;
use crate::state::ClientState;
use crate::state::theme::InterfaceThemeType;

pub const TYPEWRITER_CHARACTERS_PER_SECOND: f32 = 35.0;

#[derive(Debug, Clone, PartialEq, Eq, RustState, StateElement)]
pub struct CinematicDialogChoice {
    text: String,
    option: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinematicDialogAdvance {
    RevealedText,
    Next { npc_id: EntityId },
    Close { npc_id: EntityId },
    WaitingForChoice,
    Idle,
}

#[derive(RustState, StateElement)]
pub struct CinematicDialogElement {
    #[hidden_element]
    element: UnsafeCell<ElementBox<ClientState>>,
}

impl CinematicDialogElement {
    fn new<E>(element: E) -> Self
    where
        E: Element<ClientState> + 'static,
    {
        Self {
            element: UnsafeCell::new(ErasedElement::new(element)),
        }
    }
}

impl std::fmt::Debug for CinematicDialogElement {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("CinematicDialogElement").finish_non_exhaustive()
    }
}

#[derive(Debug, RustState, StateElement)]
pub struct CinematicDialogWindowState {
    texts: Vec<String>,
    choices: Vec<CinematicDialogChoice>,
    choice_elements: Vec<CinematicDialogElement>,
    npc_id: EntityId,
    visible_characters: usize,
    character_progress: f32,
    has_next_button: bool,
    has_close_button: bool,
    clear_next: bool,
    active: bool,
}

impl CinematicDialogWindowState {
    pub fn initialize(&mut self, npc_id: EntityId) -> &mut Self {
        self.npc_id = npc_id;
        self.active = true;
        self
    }

    pub fn add_text(&mut self, text: String) {
        if self.clear_next {
            self.texts.clear();
            self.choices.clear();
            self.choice_elements.clear();
            self.visible_characters = 0;
            self.character_progress = 0.0;
            self.has_next_button = false;
            self.has_close_button = false;
            self.clear_next = false;
        }

        self.texts.push(text);
        self.active = true;
    }

    pub fn add_next_button(&mut self) {
        self.choices.clear();
        self.choice_elements.clear();
        self.has_next_button = true;
        self.has_close_button = false;
        self.clear_next = true;
    }

    pub fn add_close_button(&mut self) {
        self.choices.clear();
        self.choice_elements.clear();
        self.has_next_button = false;
        self.has_close_button = true;
    }

    pub fn add_choice_buttons(&mut self, choices: Vec<String>) {
        use korangar_interface::prelude::*;

        self.has_next_button = false;
        self.has_close_button = false;
        self.choices = choices
            .into_iter()
            .enumerate()
            .map(|(index, text)| CinematicDialogChoice {
                text,
                option: index as i8 + 1,
            })
            .collect();

        let npc_id = self.npc_id;

        self.choice_elements = self
            .choices
            .iter()
            .map(|choice| {
                let option = choice.option;
                let text = choice.text.clone();

                CinematicDialogElement::new(button! {
                    text: text,
                    event: move |_: &State<ClientState>, queue: &mut EventQueue<ClientState>| {
                        queue.queue(InputEvent::ChooseDialogOption { npc_id, option });
                    },
                })
            })
            .collect();
    }

    pub fn update_typewriter(&mut self, delta_seconds: f32) -> usize {
        if self.is_text_complete() {
            return 0;
        }

        self.character_progress += delta_seconds * TYPEWRITER_CHARACTERS_PER_SECOND;
        let next_visible = self.character_progress.floor() as usize;
        let total = self.total_characters();
        let clamped = next_visible.min(total);
        let previous = self.visible_characters;

        self.visible_characters = clamped;
        self.visible_non_space_count(previous, clamped)
    }

    pub fn visible_text(&self) -> String {
        self.full_text().chars().take(self.visible_characters).collect()
    }

    pub fn reveal_all(&mut self) {
        self.visible_characters = self.total_characters();
        self.character_progress = self.visible_characters as f32;
    }

    pub fn advance(&mut self) -> CinematicDialogAdvance {
        if !self.active {
            return CinematicDialogAdvance::Idle;
        }

        if !self.is_text_complete() {
            self.reveal_all();
            return CinematicDialogAdvance::RevealedText;
        }

        if !self.choices.is_empty() {
            return CinematicDialogAdvance::WaitingForChoice;
        }

        if self.has_next_button {
            return CinematicDialogAdvance::Next { npc_id: self.npc_id };
        }

        if self.has_close_button {
            self.end();
            return CinematicDialogAdvance::Close { npc_id: self.npc_id };
        }

        CinematicDialogAdvance::Idle
    }

    pub fn end(&mut self) {
        self.texts.clear();
        self.choices.clear();
        self.choice_elements.clear();
        self.visible_characters = 0;
        self.character_progress = 0.0;
        self.has_next_button = false;
        self.has_close_button = false;
        self.clear_next = false;
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn npc_id(&self) -> EntityId {
        self.npc_id
    }

    pub fn is_text_complete(&self) -> bool {
        self.visible_characters >= self.total_characters()
    }

    fn total_characters(&self) -> usize {
        self.full_text().chars().count()
    }

    fn full_text(&self) -> String {
        self.texts.join("\n")
    }

    fn visible_non_space_count(&self, previous: usize, current: usize) -> usize {
        self.full_text()
            .chars()
            .skip(previous)
            .take(current.saturating_sub(previous))
            .filter(|character| !character.is_whitespace())
            .count()
    }
}

impl Default for CinematicDialogWindowState {
    fn default() -> Self {
        Self {
            texts: Vec::new(),
            choices: Vec::new(),
            choice_elements: Vec::new(),
            npc_id: EntityId(0),
            visible_characters: 0,
            character_progress: 0.0,
            has_next_button: false,
            has_close_button: false,
            clear_next: false,
            active: false,
        }
    }
}

struct VisibleText<A> {
    path: A,
    text: UnsafeCell<String>,
}

impl<A> VisibleText<A> {
    fn new(path: A) -> Self {
        Self {
            path,
            text: UnsafeCell::default(),
        }
    }
}

impl<A> Selector<ClientState, String> for VisibleText<A>
where
    A: Path<ClientState, CinematicDialogWindowState>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let visible_text = self.path.follow_safe(state).visible_text();

        unsafe {
            *self.text.get() = visible_text;
            Some(self.text.as_ref_unchecked())
        }
    }
}

struct ChoiceListElement<A> {
    elements_path: A,
}

impl<A> Element<ClientState> for ChoiceListElement<A>
where
    A: Path<ClientState, Vec<CinematicDialogElement>>,
{
    type LayoutInfo = ();

    fn create_layout_info(&mut self, state: &State<ClientState>, mut store: ElementStoreMut, resolvers: &mut dyn Resolvers<ClientState>) {
        with_single_resolver(resolvers, |resolver| {
            state
                .get(&self.elements_path)
                .iter()
                .enumerate()
                .for_each(|(index, cinematic_dialog_element)| {
                    let element = unsafe { &mut *cinematic_dialog_element.element.get() };

                    element.create_layout_info(state, store.child_store(index as u64), resolver)
                });
        })
    }

    fn lay_out<'a>(
        &'a self,
        state: &'a State<ClientState>,
        store: korangar_interface::element::store::ElementStore<'a>,
        _: &'a Self::LayoutInfo,
        layout: &mut korangar_interface::layout::WindowLayout<'a, ClientState>,
    ) {
        state
            .get(&self.elements_path)
            .iter()
            .enumerate()
            .for_each(|(index, cinematic_dialog_element)| {
                let element = unsafe { &*cinematic_dialog_element.element.get() };

                element.lay_out(state, store.child_store(index as u64), &(), layout)
            });
    }
}

pub struct CinematicDialogWindow<A> {
    window_state_path: A,
}

impl<A> CinematicDialogWindow<A> {
    pub fn new(window_state_path: A) -> Self {
        Self { window_state_path }
    }
}

impl<A> CustomWindow<ClientState> for CinematicDialogWindow<A>
where
    A: Path<ClientState, CinematicDialogWindowState>,
{
    fn window_class() -> Option<WindowClass> {
        Some(WindowClass::CinematicDialog)
    }

    fn to_window<'a>(self) -> impl Window<ClientState> + 'a {
        use korangar_interface::prelude::*;

        window! {
            title: "Dialog",
            class: Self::window_class(),
            theme: InterfaceThemeType::InGame,
            closable: false,
            elements: (
                ChoiceListElement {
                    elements_path: self.window_state_path.choice_elements(),
                },
                text! {
                    text: VisibleText::new(self.window_state_path),
                },
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typewriter_reveals_at_configured_rate() {
        let mut state = CinematicDialogWindowState::default();
        state.initialize(EntityId(7)).add_text("Hello".to_string());

        let sound_count = state.update_typewriter(1.0 / 35.0);

        assert_eq!(state.visible_text(), "H");
        assert_eq!(sound_count, 1);
    }

    #[test]
    fn advance_reveals_before_next() {
        let mut state = CinematicDialogWindowState::default();
        state.initialize(EntityId(7)).add_text("Hello".to_string());
        state.add_next_button();

        assert_eq!(state.advance(), CinematicDialogAdvance::RevealedText);
        assert_eq!(state.visible_text(), "Hello");
        assert_eq!(state.advance(), CinematicDialogAdvance::Next { npc_id: EntityId(7) });
    }

    #[test]
    fn spaces_do_not_request_text_sounds() {
        let mut state = CinematicDialogWindowState::default();
        state.initialize(EntityId(7)).add_text("A B".to_string());

        let sound_count = state.update_typewriter(3.0 / 35.0);

        assert_eq!(state.visible_text(), "A B");
        assert_eq!(sound_count, 2);
    }

    #[test]
    fn choices_create_buttons_for_each_server_option() {
        let mut state = CinematicDialogWindowState::default();
        state.initialize(EntityId(9));
        state.add_choice_buttons(vec!["First".to_string(), "Second".to_string()]);

        assert_eq!(state.choices.len(), 2);
        assert_eq!(state.choice_elements.len(), 2);
    }
}
