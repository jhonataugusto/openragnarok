# NPC Cinematic Dialog Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Criar um modo opcional de dialogo cinematico para NPCs no Korangar, preservando o dialogo classico como fallback.

**Architecture:** A feature usa os mesmos eventos de dialogo existentes e escolhe, no cliente, entre `DialogWindow` classica e uma nova UI cinematica. O estado cinematico fica em `ClientState`, a camera cinematica fica no runtime de `Client`, e o audio ganha uma pequena API de pitch para som por letra.

**Tech Stack:** Rust 2024, Korangar UI macros, `rust-state`, `cgmath`, `korangar-audio`, `cargo test`.

---

## Estrutura De Arquivos

- Modify: `korangar/korangar/src/settings/interface.rs`
  - Adiciona settings persistentes e compatibilidade com RON antigo.
- Modify: `korangar/korangar/src/interface/windows/interface_settings.rs`
  - Mostra toggles da feature em Interface Settings.
- Create: `korangar/korangar/src/interface/windows/cinematic_dialog.rs`
  - Define estado puro do dialogo cinematico, typewriter, janela e elementos de opcoes.
- Modify: `korangar/korangar/src/interface/windows/mod.rs`
  - Exporta a janela nova e adiciona `WindowClass::CinematicDialog`.
- Modify: `korangar/korangar/src/state/mod.rs`
  - Adiciona `cinematic_dialog_window` ao `ClientState`.
- Modify: `korangar/korangar/src/input/event.rs`
  - Adiciona `AdvanceCinematicDialog`.
- Create: `korangar/korangar/src/world/cameras/cinematic.rs`
  - Implementa camera cinematica dinamica.
- Modify: `korangar/korangar/src/world/cameras/mod.rs`
  - Exporta `CinematicCamera`.
- Modify: `korangar/korangar/src/world/entity/mod.rs`
  - Expoe direcao atual da entidade para enquadramento cinematico.
- Modify: `korangar/korangar/src/main.rs`
  - Roteia eventos de dialogo, input, camera e audio para o modo cinematico.
- Modify: `korangar/korangar-audio/src/sound/static_sound/settings.rs`
  - Adiciona pitch nas configuracoes de som estatico.
- Modify: `korangar/korangar-audio/src/sound/static_sound/data.rs`
  - Adiciona builder `pitch`.
- Modify: `korangar/korangar-audio/src/sound/static_sound/sound.rs`
  - Processa som estatico com cursor fracionario para pitch.
- Modify: `korangar/korangar-audio/src/lib.rs`
  - Expoe `play_sound_effect_with_pitch`.

---

### Task 1: Settings E Toggles

**Files:**
- Modify: `korangar/korangar/src/settings/interface.rs`
- Modify: `korangar/korangar/src/interface/windows/interface_settings.rs`

- [ ] **Step 1: Escrever teste de compatibilidade RON antigo**

Adicione ao final de `korangar/korangar/src/settings/interface.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn old_interface_settings_files_load_with_cinematic_defaults() {
        let data = r#"(
            language: English,
            scaling: (factor: 1.0),
            menu_theme: "default",
            in_game_theme: "default",
            world_theme: "default",
        )"#;

        let settings: InterfaceSettings = ron::from_str(data).unwrap();

        assert!(!settings.npc_cinematic_dialog_enabled);
        assert!(settings.npc_cinematic_dynamic_camera_enabled);
        assert!(settings.npc_cinematic_text_sound_enabled);
    }
}
```

- [ ] **Step 2: Rodar o teste e confirmar falha**

Run:

```powershell
cargo test -p korangar old_interface_settings_files_load_with_cinematic_defaults
```

Working directory: `D:\ragnarok\korangar`

Expected: FAIL porque os campos ainda nao existem em `InterfaceSettings`.

- [ ] **Step 3: Adicionar campos e defaults**

Em `korangar/korangar/src/settings/interface.rs`, adicione estas funcoes antes de `InterfaceSettings`:

```rust
fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}
```

Atualize `InterfaceSettings`:

```rust
#[derive(Clone, Serialize, Deserialize, RustState, StateElement)]
pub struct InterfaceSettings {
    pub language: Language,
    pub scaling: Scaling,
    pub menu_theme: String,
    pub in_game_theme: String,
    pub world_theme: String,
    #[serde(default = "default_false")]
    pub npc_cinematic_dialog_enabled: bool,
    #[serde(default = "default_true")]
    pub npc_cinematic_dynamic_camera_enabled: bool,
    #[serde(default = "default_true")]
    pub npc_cinematic_text_sound_enabled: bool,
}
```

Atualize `Default`:

```rust
Self {
    language: Language::English,
    scaling: Scaling::new(1.0),
    menu_theme: DEFAULT_THEME_NAME.to_string(),
    in_game_theme: DEFAULT_THEME_NAME.to_string(),
    world_theme: DEFAULT_THEME_NAME.to_string(),
    npc_cinematic_dialog_enabled: false,
    npc_cinematic_dynamic_camera_enabled: true,
    npc_cinematic_text_sound_enabled: true,
}
```

- [ ] **Step 4: Adicionar toggles na janela de Interface Settings**

Em `korangar/korangar/src/interface/windows/interface_settings.rs`, no tuple `elements`, adicione depois de `world_theme`:

```rust
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
```

- [ ] **Step 5: Rodar teste**

Run:

```powershell
cargo test -p korangar old_interface_settings_files_load_with_cinematic_defaults
```

Working directory: `D:\ragnarok\korangar`

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git -C D:\ragnarok\korangar add korangar/src/settings/interface.rs korangar/src/interface/windows/interface_settings.rs
git -C D:\ragnarok\korangar commit -m "feat: add cinematic dialog settings"
```

---

### Task 2: Estado Puro Do Dialogo Cinematico

**Files:**
- Create: `korangar/korangar/src/interface/windows/cinematic_dialog.rs`
- Modify: `korangar/korangar/src/interface/windows/mod.rs`
- Modify: `korangar/korangar/src/state/mod.rs`

- [ ] **Step 1: Criar arquivo com estado e testes**

Crie `korangar/korangar/src/interface/windows/cinematic_dialog.rs` com o estado inicial e testes:

```rust
use ragnarok_packets::EntityId;
use rust_state::RustState;

const TYPEWRITER_CHARACTERS_PER_SECOND: f32 = 35.0;

#[derive(Debug, Clone, PartialEq, Eq, RustState)]
pub struct CinematicDialogChoice {
    text: String,
    option: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinematicDialogAdvance {
    RevealedText,
    Next { npc_id: EntityId },
    Close { npc_id: EntityId },
    Choose { npc_id: EntityId, option: i8 },
    WaitingForChoice,
    Idle,
}

#[derive(Debug, RustState)]
pub struct CinematicDialogWindowState {
    texts: Vec<String>,
    choices: Vec<CinematicDialogChoice>,
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
        self.has_next_button = true;
        self.has_close_button = false;
        self.clear_next = true;
    }

    pub fn add_close_button(&mut self) {
        self.has_next_button = false;
        self.has_close_button = true;
    }

    pub fn add_choice_buttons(&mut self, choices: Vec<String>) {
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

    pub fn choose(&mut self, option: i8) -> Option<CinematicDialogAdvance> {
        self.choices
            .iter()
            .any(|choice| choice.option == option)
            .then_some(CinematicDialogAdvance::Choose {
                npc_id: self.npc_id,
                option,
            })
    }

    pub fn end(&mut self) {
        self.texts.clear();
        self.choices.clear();
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
```

- [ ] **Step 2: Adicionar testes no mesmo arquivo**

Adicione ao final de `cinematic_dialog.rs`:

```rust
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
    fn choices_preserve_server_option_numbers() {
        let mut state = CinematicDialogWindowState::default();
        state.initialize(EntityId(9));
        state.add_choice_buttons(vec!["First".to_string(), "Second".to_string()]);

        assert_eq!(
            state.choose(2),
            Some(CinematicDialogAdvance::Choose {
                npc_id: EntityId(9),
                option: 2
            })
        );
    }
}
```

- [ ] **Step 3: Registrar modulo e estado**

Em `korangar/korangar/src/interface/windows/mod.rs`, adicione:

```rust
mod cinematic_dialog;
pub use self::cinematic_dialog::{CinematicDialogAdvance, CinematicDialogWindow, CinematicDialogWindowState};
```

No enum `WindowClass`, adicione:

```rust
CinematicDialog,
```

Em `korangar/korangar/src/state/mod.rs`, atualize imports:

```rust
    ChatWindowState, CinematicDialogWindowState, DialogWindowState, FriendListWindowState, LoginWindowState,
```

Adicione campo em `ClientState`:

```rust
/// Internal state of the cinematic dialog window.
cinematic_dialog_window: CinematicDialogWindowState,
```

Na criacao de player resources:

```rust
let cinematic_dialog_window = CinematicDialogWindowState::default();
```

No literal `ClientState { ... }`:

```rust
cinematic_dialog_window,
```

- [ ] **Step 4: Rodar testes do estado**

Run:

```powershell
cargo test -p korangar cinematic_dialog
```

Working directory: `D:\ragnarok\korangar`

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git -C D:\ragnarok\korangar add korangar/src/interface/windows/cinematic_dialog.rs korangar/src/interface/windows/mod.rs korangar/src/state/mod.rs
git -C D:\ragnarok\korangar commit -m "feat: add cinematic dialog state"
```

---

### Task 3: UI Cinematica

**Files:**
- Modify: `korangar/korangar/src/interface/windows/cinematic_dialog.rs`

- [ ] **Step 1: Adicionar window e selectors**

No topo de `cinematic_dialog.rs`, acrescente imports usados pela UI:

```rust
use std::cell::UnsafeCell;

use korangar_interface::element::store::ElementStoreMut;
use korangar_interface::element::{Element, ElementBox, ErasedElement, StateElement};
use korangar_interface::layout::{Resolvers, with_single_resolver};
use korangar_interface::window::{CustomWindow, Window};
use rust_state::{Path, Selector, State};

use super::WindowClass;
use crate::input::InputEvent;
use crate::state::theme::InterfaceThemeType;
use crate::state::{ClientState, ClientStatePathExt, client_state};
```

Adicione um selector de texto visivel:

```rust
struct VisibleText<A> {
    path: A,
}

impl<A> Selector<ClientState, String> for VisibleText<A>
where
    A: Path<ClientState, CinematicDialogWindowState>,
{
    fn select<'a>(&'a self, state: &'a ClientState) -> Option<&'a String> {
        let text = self.path.follow_safe(state).visible_text();
        Some(Box::leak(Box::new(text)))
    }
}
```

- [ ] **Step 2: Adicionar elementos de escolha**

Adicione um wrapper seguindo o mesmo padrao seguro-local de `DialogElement`:

```rust
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
```

Adicione campo `choice_elements: Vec<CinematicDialogElement>` ao `CinematicDialogWindowState`.

No `Default`, inicialize:

```rust
choice_elements: Vec::new(),
```

Em `add_choice_buttons`, construa botoes:

```rust
self.choice_elements = self
    .choices
    .iter()
    .map(|choice| {
        use korangar_interface::prelude::*;

        let npc_id = self.npc_id;
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
```

Em `end`, limpe:

```rust
self.choice_elements.clear();
```

- [ ] **Step 3: Implementar layout da janela**

Adicione `ChoiceListElement`:

```rust
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
            state.get(&self.elements_path).iter().enumerate().for_each(|(index, element)| {
                let element = unsafe { &mut *element.element.get() };
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
        state.get(&self.elements_path).iter().enumerate().for_each(|(index, element)| {
            let element = unsafe { &*element.element.get() };
            element.lay_out(state, store.child_store(index as u64), &(), layout)
        });
    }
}
```

Adicione a window:

```rust
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
                    text: VisibleText {
                        path: self.window_state_path,
                    },
                },
            ),
        }
    }
}
```

- [ ] **Step 4: Rodar build/teste focado**

Run:

```powershell
cargo test -p korangar cinematic_dialog
```

Working directory: `D:\ragnarok\korangar`

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git -C D:\ragnarok\korangar add korangar/src/interface/windows/cinematic_dialog.rs korangar/src/interface/windows/mod.rs
git -C D:\ragnarok\korangar commit -m "feat: render cinematic dialog window"
```

---

### Task 4: Roteamento De Eventos E Input

**Files:**
- Modify: `korangar/korangar/src/input/event.rs`
- Modify: `korangar/korangar/src/main.rs`

- [ ] **Step 1: Adicionar evento de avancar**

Em `korangar/korangar/src/input/event.rs`, perto dos eventos de dialogo:

```rust
/// Reveal or advance the active cinematic dialog.
AdvanceCinematicDialog,
```

- [ ] **Step 2: Importar tipos cinematicos em `main.rs`**

Atualize o import de windows:

```rust
use crate::interface::windows::{
    AudioSettingsWindow, BuyOrSellWindow, BuyWindow, CharacterCreationWindow, CharacterOverviewWindow, CharacterSelectionWindow,
    ChatTextBox, ChatWindow, CinematicDialogAdvance, CinematicDialogWindow, DialogWindow, EquipmentWindow, ErrorWindow,
    FriendListWindow, FriendRequestWindow, GameSettingsWindow, GraphicsSettingsWindow, HotbarWindow, InterfaceSettingsWindow,
    InventoryWindow, MenuWindow, RespawnWindow, SellCartWindow, SellWindow, ServerSelectionWindow, SkillTreeWindow, StatsWindow,
    WindowClass,
};
```

- [ ] **Step 3: Adicionar helpers no impl `Client`**

Adicione metodos privados em `impl Client`:

```rust
fn can_use_cinematic_dialog(&self, npc_id: EntityId) -> bool {
    *self
        .client_state
        .follow(client_state().interface_settings().npc_cinematic_dialog_enabled())
        && self.client_state.try_follow(this_entity()).is_some()
        && self
            .client_state
            .follow(client_state().entities())
            .iter()
            .any(|entity| entity.get_entity_id() == npc_id)
}

fn is_cinematic_dialog_active(&self) -> bool {
    self.client_state
        .follow(client_state().cinematic_dialog_window())
        .is_active()
}

fn open_cinematic_dialog_window(&mut self) {
    self.interface.close_window_with_class(WindowClass::Dialog);
    self.interface
        .open_window(CinematicDialogWindow::new(client_state().cinematic_dialog_window()));
}

fn close_cinematic_dialog_window(&mut self) {
    self.client_state
        .follow_mut(client_state().cinematic_dialog_window())
        .end();
    self.interface.close_window_with_class(WindowClass::CinematicDialog);
}
```

- [ ] **Step 4: Roteirar network events de dialogo**

Substitua os quatro handlers atuais de dialogo por esta forma:

```rust
NetworkEvent::OpenDialog { text, npc_id } => {
    if self.can_use_cinematic_dialog(npc_id) {
        self.client_state
            .follow_mut(client_state().cinematic_dialog_window())
            .initialize(npc_id)
            .add_text(text);
        self.open_cinematic_dialog_window();
    } else {
        self.client_state
            .follow_mut(client_state().dialog_window())
            .initialize(npc_id)
            .add_text(text);
        self.interface.open_window(DialogWindow::new(client_state().dialog_window()));
    }
}
NetworkEvent::AddNextButton { npc_id } => {
    if self.can_use_cinematic_dialog(npc_id) || self.is_cinematic_dialog_active() {
        self.client_state
            .follow_mut(client_state().cinematic_dialog_window())
            .initialize(npc_id)
            .add_next_button();
        self.open_cinematic_dialog_window();
    } else {
        self.client_state
            .follow_mut(client_state().dialog_window())
            .initialize(npc_id)
            .add_next_button();
        self.interface.open_window(DialogWindow::new(client_state().dialog_window()));
    }
}
NetworkEvent::AddCloseButton { npc_id } => {
    if self.is_cinematic_dialog_active() {
        self.client_state
            .follow_mut(client_state().cinematic_dialog_window())
            .initialize(npc_id)
            .add_close_button();
    } else if self.interface.is_window_with_class_open(WindowClass::Dialog) {
        self.client_state
            .follow_mut(client_state().dialog_window())
            .initialize(npc_id)
            .add_close_button();
    }
}
NetworkEvent::AddChoiceButtons { choices, npc_id } => {
    if self.can_use_cinematic_dialog(npc_id) || self.is_cinematic_dialog_active() {
        self.client_state
            .follow_mut(client_state().cinematic_dialog_window())
            .initialize(npc_id)
            .add_choice_buttons(choices);
        self.open_cinematic_dialog_window();
    } else {
        self.client_state
            .follow_mut(client_state().dialog_window())
            .initialize(npc_id)
            .add_choice_buttons(choices);
        self.interface.open_window(DialogWindow::new(client_state().dialog_window()));
    }
}
```

- [ ] **Step 5: Interceptar mouse/teclado para revelar e avancar**

Depois de `let interface_has_focus = self.interface.has_focus();`, adicione:

```rust
let cinematic_dialog_active = self.is_cinematic_dialog_active();
let cinematic_advance_pressed = cinematic_dialog_active
    && (matches!(
        input_report.mouse_click,
        Some(korangar_interface::layout::MouseButton::Left | korangar_interface::layout::MouseButton::DoubleLeft)
    ) || input_report.characters.contains(&'\x0d')
        || input_report.characters.contains(&' '));

if cinematic_advance_pressed {
    self.input_event_buffer.push(InputEvent::AdvanceCinematicDialog);
}
```

Altere chamadas de camera/input para respeitar `cinematic_dialog_active`:

```rust
if !cinematic_dialog_active && self.interface.get_mouse_mode().is_rotating_camera() {
    let rotation = input_report.mouse_delta.width;
    self.input_event_buffer.push(InputEvent::RotateCamera { rotation });
}

if !cinematic_dialog_active && !interface_has_focus {
    self.input_system.handle_keyboard_input(/* parametros existentes */);
}
```

Na area de processamento de mouse no render frame, envolva interacoes de mundo, caminhada, scroll de camera e rotacao com:

```rust
if !cinematic_dialog_active {
    // codigo existente de PlayerMove, PlayerInteract, PickUpItem, RotateCamera e ZoomCamera
}
```

Mantenha `interface_frame.click(...)` fora desse bloqueio para que os baloes de escolha continuem clicaveis.

- [ ] **Step 6: Processar `AdvanceCinematicDialog`**

No match de `InputEvent`, adicione:

```rust
InputEvent::AdvanceCinematicDialog => {
    let advance = self
        .client_state
        .follow_mut(client_state().cinematic_dialog_window())
        .advance();

    match advance {
        CinematicDialogAdvance::RevealedText | CinematicDialogAdvance::WaitingForChoice | CinematicDialogAdvance::Idle => {}
        CinematicDialogAdvance::Next { npc_id } => {
            let _ = self.networking_system.next_dialog(npc_id);
        }
        CinematicDialogAdvance::Close { npc_id } => {
            let _ = self.networking_system.close_dialog(npc_id);
            self.close_cinematic_dialog_window();
        }
        CinematicDialogAdvance::Choose { npc_id, option } => {
            let _ = self.networking_system.choose_dialog_option(npc_id, option);
        }
    }
}
```

Atualize handlers existentes:

```rust
InputEvent::CloseDialog { npc_id } => {
    let _ = self.networking_system.close_dialog(npc_id);
    self.client_state.follow_mut(client_state().dialog_window()).end();
    self.interface.close_window_with_class(WindowClass::Dialog);
    self.close_cinematic_dialog_window();
}
InputEvent::ChooseDialogOption { npc_id, option } => {
    let _ = self.networking_system.choose_dialog_option(npc_id, option);

    if option == -1 {
        self.interface.close_window_with_class(WindowClass::Dialog);
        self.close_cinematic_dialog_window();
    }
}
```

- [ ] **Step 7: Rodar testes/build**

Run:

```powershell
cargo test -p korangar cinematic_dialog
```

Working directory: `D:\ragnarok\korangar`

Expected: PASS.

- [ ] **Step 8: Commit**

```powershell
git -C D:\ragnarok\korangar add korangar/src/input/event.rs korangar/src/main.rs
git -C D:\ragnarok\korangar commit -m "feat: route npc dialogs to cinematic mode"
```

---

### Task 5: Camera Cinematica Dinamica

**Files:**
- Create: `korangar/korangar/src/world/cameras/cinematic.rs`
- Modify: `korangar/korangar/src/world/cameras/mod.rs`
- Modify: `korangar/korangar/src/world/entity/mod.rs`
- Modify: `korangar/korangar/src/main.rs`

- [ ] **Step 1: Expor direcao da entidade**

Em `korangar/korangar/src/world/entity/mod.rs`, adicione em `impl Entity`:

```rust
pub fn get_direction(&self) -> Direction {
    self.get_common().direction
}
```

- [ ] **Step 2: Criar camera cinematica**

Crie `korangar/korangar/src/world/cameras/cinematic.rs`:

```rust
use cgmath::{Array, InnerSpace, Matrix4, Point3, Vector2, Vector3, Zero};
use ragnarok_packets::Direction;

use super::{Camera, SmoothedValue};
use crate::graphics::perspective_reverse_lh;

const THRESHOLD: f32 = 0.01;
const CAMERA_HEIGHT: f32 = 95.0;
const CAMERA_DISTANCE: f32 = 230.0;
const RIGHT_OFFSET: f32 = 55.0;
const FOCUS_HEIGHT: f32 = 55.0;
const VERTICAL_FOV: cgmath::Deg<f32> = cgmath::Deg(18.0);
const LOOK_UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

pub struct CinematicCamera {
    focus_point: Point3<SmoothedValue>,
    camera_position: Point3<SmoothedValue>,
    view_direction: Vector3<f32>,
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
    view_projection_matrix: Matrix4<f32>,
}

impl CinematicCamera {
    pub fn new() -> Self {
        Self {
            focus_point: [SmoothedValue::new(0.0, THRESHOLD, 8.0); 3].into(),
            camera_position: [SmoothedValue::new(0.0, THRESHOLD, 8.0); 3].into(),
            view_direction: Vector3::unit_z(),
            view_matrix: Matrix4::zero(),
            projection_matrix: Matrix4::zero(),
            view_projection_matrix: Matrix4::zero(),
        }
    }

    pub fn set_immediate(&mut self, player_position: Point3<f32>, npc_position: Point3<f32>, player_direction: Direction) {
        let (camera_position, focus_point) = Self::calculate_targets(player_position, npc_position, player_direction);
        self.camera_position.x.set(camera_position.x);
        self.camera_position.y.set(camera_position.y);
        self.camera_position.z.set(camera_position.z);
        self.focus_point.x.set(focus_point.x);
        self.focus_point.y.set(focus_point.y);
        self.focus_point.z.set(focus_point.z);
    }

    pub fn set_dynamic_targets(&mut self, player_position: Point3<f32>, npc_position: Point3<f32>, player_direction: Direction) {
        let (camera_position, focus_point) = Self::calculate_targets(player_position, npc_position, player_direction);
        self.camera_position.x.set_desired(camera_position.x);
        self.camera_position.y.set_desired(camera_position.y);
        self.camera_position.z.set_desired(camera_position.z);
        self.focus_point.x.set_desired(focus_point.x);
        self.focus_point.y.set_desired(focus_point.y);
        self.focus_point.z.set_desired(focus_point.z);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.camera_position.x.update(delta_time);
        self.camera_position.y.update(delta_time);
        self.camera_position.z.update(delta_time);
        self.focus_point.x.update(delta_time);
        self.focus_point.y.update(delta_time);
        self.focus_point.z.update(delta_time);

        self.view_direction = (self.focus_point() - self.camera_position()).normalize();
    }

    fn calculate_targets(
        player_position: Point3<f32>,
        npc_position: Point3<f32>,
        player_direction: Direction,
    ) -> (Point3<f32>, Point3<f32>) {
        let forward = direction_vector(player_direction);
        let right = Vector3::new(forward.z, 0.0, -forward.x).normalize();
        let focus_point = Point3::new(
            (player_position.x + npc_position.x) * 0.5,
            (player_position.y + npc_position.y) * 0.5 + FOCUS_HEIGHT,
            (player_position.z + npc_position.z) * 0.5,
        );
        let camera_position = player_position - forward * CAMERA_DISTANCE + right * RIGHT_OFFSET + Vector3::new(0.0, CAMERA_HEIGHT, 0.0);

        (camera_position, focus_point)
    }
}

impl Camera for CinematicCamera {
    fn camera_position(&self) -> Point3<f32> {
        self.camera_position.map(|component| component.get_current())
    }

    fn focus_point(&self) -> Point3<f32> {
        self.focus_point.map(|component| component.get_current())
    }

    fn generate_view_projection(&mut self, window_size: Vector2<usize>) {
        let aspect_ratio = window_size.x as f32 / window_size.y as f32;
        self.view_matrix = Matrix4::look_to_lh(self.camera_position(), self.view_direction, LOOK_UP);
        self.projection_matrix = perspective_reverse_lh(VERTICAL_FOV, aspect_ratio);
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
    }

    fn look_up_vector(&self) -> Vector3<f32> {
        LOOK_UP
    }

    fn view_projection_matrices(&self) -> (Matrix4<f32>, Matrix4<f32>) {
        (self.view_matrix, self.projection_matrix)
    }

    fn view_projection_matrix(&self) -> Matrix4<f32> {
        self.view_projection_matrix
    }

    fn view_direction(&self) -> Vector3<f32> {
        self.view_direction
    }
}

fn direction_vector(direction: Direction) -> Vector3<f32> {
    match direction {
        Direction::North => Vector3::new(0.0, 0.0, 1.0),
        Direction::NorthEast => Vector3::new(1.0, 0.0, 1.0).normalize(),
        Direction::East => Vector3::new(1.0, 0.0, 0.0),
        Direction::SouthEast => Vector3::new(1.0, 0.0, -1.0).normalize(),
        Direction::South => Vector3::new(0.0, 0.0, -1.0),
        Direction::SouthWest => Vector3::new(-1.0, 0.0, -1.0).normalize(),
        Direction::West => Vector3::new(-1.0, 0.0, 0.0),
        Direction::NorthWest => Vector3::new(-1.0, 0.0, 1.0).normalize(),
    }
}
```

- [ ] **Step 3: Exportar camera**

Em `korangar/korangar/src/world/cameras/mod.rs`:

```rust
mod cinematic;
pub use self::cinematic::CinematicCamera;
```

- [ ] **Step 4: Adicionar camera ao `Client`**

Em `main.rs`, importe `CinematicCamera` junto das cameras.

Adicione campo:

```rust
cinematic_camera: CinematicCamera,
```

Na inicializacao de cameras:

```rust
let cinematic_camera = CinematicCamera::new();
```

No literal `Client { ... }`:

```rust
cinematic_camera,
```

- [ ] **Step 5: Atualizar camera por frame**

Adicione helper em `impl Client`:

```rust
fn cinematic_dialog_targets(&self) -> Option<(Point3<f32>, Point3<f32>, Direction)> {
    let dialog = self.client_state.follow(client_state().cinematic_dialog_window());
    if !dialog.is_active() {
        return None;
    }

    let player = self.client_state.try_follow(this_entity())?;
    let npc_id = dialog.npc_id();
    let npc = self
        .client_state
        .follow(client_state().entities())
        .iter()
        .find(|entity| entity.get_entity_id() == npc_id)?;

    Some((player.get_position(), npc.get_position(), player.get_direction()))
}
```

Adicione getter em `CinematicDialogWindowState`:

```rust
pub fn npc_id(&self) -> EntityId {
    self.npc_id
}
```

No bloco que atualiza main camera, antes de escolher `current_camera`:

```rust
let use_cinematic_camera = self
    .client_state
    .follow(client_state().interface_settings().npc_cinematic_dynamic_camera_enabled())
    && self.cinematic_dialog_targets().is_some();

if let Some((player_position, npc_position, player_direction)) = self.cinematic_dialog_targets() {
    self.cinematic_camera
        .set_dynamic_targets(player_position, npc_position, player_direction);
    self.cinematic_camera.update(delta_time);
    self.cinematic_camera.generate_view_projection(window_size);
}
```

Atualize as escolhas de `current_camera`:

```rust
let current_camera: &(dyn Camera + Send + Sync) = match currently_playing {
    _ if use_cinematic_camera => &self.cinematic_camera,
    _ if render_options.use_debug_camera => &self.debug_camera,
    true => &self.player_camera,
    false => &self.start_camera,
};
```

Repita a mesma prioridade nas demais escolhas de camera no arquivo.

- [ ] **Step 6: Rodar build**

Run:

```powershell
cargo check -p korangar
```

Working directory: `D:\ragnarok\korangar`

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git -C D:\ragnarok\korangar add korangar/src/world/cameras/cinematic.rs korangar/src/world/cameras/mod.rs korangar/src/world/entity/mod.rs korangar/src/main.rs
git -C D:\ragnarok\korangar commit -m "feat: add cinematic dialog camera"
```

---

### Task 6: Som Por Letra Com Pitch

**Files:**
- Modify: `korangar/korangar-audio/src/sound/static_sound/settings.rs`
- Modify: `korangar/korangar-audio/src/sound/static_sound/data.rs`
- Modify: `korangar/korangar-audio/src/sound/static_sound/sound.rs`
- Modify: `korangar/korangar-audio/src/lib.rs`
- Modify: `korangar/korangar/src/main.rs`

- [ ] **Step 1: Adicionar pitch nas settings de som estatico**

Em `settings.rs`:

```rust
pub(crate) pitch: f32,
```

No default:

```rust
pitch: 1.0,
```

- [ ] **Step 2: Adicionar builder em `StaticSoundData`**

Em `data.rs`, adicione:

```rust
pub(crate) fn pitch(&self, pitch: f32) -> Self {
    let mut new = self.clone();
    new.settings.pitch = pitch.clamp(0.5, 2.0);
    new
}
```

- [ ] **Step 3: Processar pitch no som estatico**

Em `sound.rs`, adicione campo:

```rust
playback_cursor: f32,
pitch: f32,
```

No construtor:

```rust
playback_cursor: starting_frame_index as f32,
pitch: settings.pitch,
```

Substitua o acesso ao frame no `process` por:

```rust
let current_frame = if self.transport.playing {
    let frame_index = self.playback_cursor.floor() as usize;
    frame_at_index(frame_index, &self.frames).unwrap_or_default()
} else {
    Frame::ZERO
};

self.playback_cursor += self.pitch.max(0.01);
self.transport.seek_to(self.playback_cursor.floor() as usize, num_frames(&self.frames));
if !self.transport.playing {
    self.playback_state_manager.mark_as_stopped();
    self.update_shared_playback_state();
}
```

Remova a chamada antiga de `self.transport.increment_position(...)` nesse trecho para evitar incremento duplo.

- [ ] **Step 4: Expor API no `AudioEngine`**

Em `korangar-audio/src/lib.rs`, adicione metodo publico:

```rust
/// Plays a sound effect with a pitch multiplier.
pub fn play_sound_effect_with_pitch(&self, sound_effect_key: SoundEffectKey, pitch: f32) {
    self.engine_context
        .lock()
        .unwrap()
        .play_sound_effect_with_pitch(sound_effect_key, pitch)
}
```

No `impl EngineContext<F>`, adicione:

```rust
fn play_sound_effect_with_pitch(&mut self, sound_effect_key: SoundEffectKey, pitch: f32) {
    match self
        .cache
        .get(&sound_effect_key)
        .map(|cached_sound_effect| cached_sound_effect.0.clone())
    {
        Some(data) => {
            if let Err(_error) = self.sound_effect_track.play(data.pitch(pitch)) {
                #[cfg(feature = "debug")]
                print_debug!("[{}] can't play pitched sound effect: {:?}", "error".red(), _error);
            }
        }
        None => {
            queue_sound_effect_playback(
                self.game_file_loader.clone(),
                self.async_response_sender.clone(),
                &self.sound_effect_paths,
                &mut self.queued_sound_effect,
                &mut self.loading_sound_effect,
                sound_effect_key,
                QueuedSoundEffectType::Sound,
            );
        }
    }
}
```

- [ ] **Step 5: Tocar som no typewriter**

Em `main.rs`, adicione constante perto dos outros sons:

```rust
const CINEMATIC_DIALOG_TEXT_SOUND_EFFECT: &str = "버튼소리.wav";
```

Adicione campo em `Client`:

```rust
cinematic_dialog_text_sound_effect: SoundEffectKey,
```

Na inicializacao de sons:

```rust
let cinematic_dialog_text_sound_effect = audio_engine.load(CINEMATIC_DIALOG_TEXT_SOUND_EFFECT);
```

No literal `Client { ... }`:

```rust
cinematic_dialog_text_sound_effect,
```

Depois de atualizar o typewriter por frame:

```rust
let text_sound_enabled = *self
    .client_state
    .follow(client_state().interface_settings().npc_cinematic_text_sound_enabled());

if text_sound_enabled {
    for index in 0..new_text_sound_count.min(4) {
        let pitch = 0.94 + ((client_tick.0 as usize + index) % 7) as f32 * 0.02;
        self.audio_engine
            .play_sound_effect_with_pitch(self.cinematic_dialog_text_sound_effect, pitch);
    }
}
```

- [ ] **Step 6: Rodar testes e check**

Run:

```powershell
cargo check -p korangar-audio
cargo check -p korangar
```

Working directory: `D:\ragnarok\korangar`

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git -C D:\ragnarok\korangar add korangar-audio/src/sound/static_sound/settings.rs korangar-audio/src/sound/static_sound/data.rs korangar-audio/src/sound/static_sound/sound.rs korangar-audio/src/lib.rs korangar/src/main.rs
git -C D:\ragnarok\korangar commit -m "feat: add pitched cinematic text sounds"
```

---

### Task 7: Integracao Final E Verificacao Manual

**Files:**
- Modify: `korangar/korangar/src/main.rs`
- Modify: `docs/superpowers/specs/2026-04-28-npc-cinematic-dialog-design.md`

- [ ] **Step 1: Garantir fallback classico**

Revise `main.rs` e confirme que todos os caminhos que chamam `can_use_cinematic_dialog(npc_id) == false` executam o bloco classico:

```rust
self.client_state
    .follow_mut(client_state().dialog_window())
    .initialize(npc_id)
    .add_text(text);
self.interface.open_window(DialogWindow::new(client_state().dialog_window()));
```

- [ ] **Step 2: Garantir limpeza no fechamento**

Confirme que `CloseDialog`, `ChooseDialogOption { option: -1 }`, logout, troca de mapa e fechamento de shop chamam:

```rust
self.close_cinematic_dialog_window();
```

Nos pontos que ja fecham `WindowClass::Dialog`, adicione tambem:

```rust
self.interface.close_window_with_class(WindowClass::CinematicDialog);
```

- [ ] **Step 3: Rodar verificacao automatica**

Run:

```powershell
cargo test -p korangar cinematic_dialog
cargo check -p korangar
```

Working directory: `D:\ragnarok\korangar`

Expected: PASS.

- [ ] **Step 4: Verificacao manual no cliente**

Run:

```powershell
.\play.bat
```

Working directory: `D:\ragnarok`

Expected:

- Com `NPC cinematic dialog` desligado, falar com NPC abre a janela classica.
- Com `NPC cinematic dialog` ligado, falar com NPC abre a UI cinematica.
- Mouse esquerdo, Enter e Espaco revelam texto parcial e depois avancam.
- Opcoes aparecem em baloes acima da caixa e enviam a escolha correta.
- Movimento, scroll e rotacao manual ficam bloqueados enquanto a UI cinematica esta ativa.
- Ao fechar, movimento e camera normal voltam.

- [ ] **Step 5: Atualizar spec com resultado**

Em `docs/superpowers/specs/2026-04-28-npc-cinematic-dialog-design.md`, adicione:

```markdown
## Resultado De Implementacao

- Verificacao automatica: `cargo test -p korangar cinematic_dialog`; `cargo check -p korangar`
- Verificacao manual: dialogo classico desligado, dialogo cinematico ligado, typewriter, opcoes, bloqueio de input e retorno de camera normal confirmados no cliente local.
```

- [ ] **Step 6: Commit final**

```powershell
git -C D:\ragnarok\korangar add korangar/src/main.rs
git -C D:\ragnarok add docs/superpowers/specs/2026-04-28-npc-cinematic-dialog-design.md
git -C D:\ragnarok\korangar commit -m "fix: complete cinematic dialog integration"
git -C D:\ragnarok commit -m "docs: record cinematic dialog verification"
```

---

## Self-Review

Spec coverage:

- Opcao global em Interface Settings: Task 1.
- Aplicar a todos os dialogos de NPC quando ativo: Task 4.
- Preservar dialogo classico e fallback: Tasks 4 e 7.
- Movimento/camera manual travados: Task 4.
- Camera dinamica sem cortes: Task 5.
- Typewriter com revelar antes de avancar: Tasks 2 e 4.
- Mouse esquerdo, Enter e Espaco: Task 4.
- Som por letra com toggle e pitch: Tasks 1 e 6.
- Opcoes em baloes empilhados: Task 3.
- Verificacao manual no cliente real: Task 7.

Varredura de lacunas: o plano nao usa marcadores de trabalho aberto nem campos a preencher.

Type consistency:

- `CinematicDialogWindowState` e o path `client_state().cinematic_dialog_window()` sao introduzidos antes de uso em `main.rs`.
- `CinematicDialogAdvance` e `AdvanceCinematicDialog` sao introduzidos antes de uso no roteamento.
- `CinematicCamera` e `Entity::get_direction()` sao introduzidos antes de uso no runtime de camera.
