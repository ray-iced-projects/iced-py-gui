//! A menu widget for Iced built on Popover, Stack, and MouseArea.
//!
//! This widget enables building interactive menus with:
//! - Dropdown items with hover navigation and configurable delays
//! - Unlimited nesting depth with submenus
//! - Custom widgets as menu items (separators, checkboxes, complex layouts)
//! - Type-safe message handling via custom `UserMsg` types
//!
//! # Basic Example
//!
//! ```ignore
//! use iced_menu::{Menu, MenuConfig, MenuItem, MenuMessage};
//! use iced::Element;
//! use std::rc::Rc;
//!
//! // Define your menu action type
//! #[derive(Debug, Clone)]
//! enum MenuAction {
//!     NewFile,
//!     OpenFile,
//!     Save,
//! }
//!
//! // Create a menu configuration
//! let menu_config = MenuConfig {
//!     label: "File",
//!     label_size: 14.0,
//!     width: 200.0,
//!     items: vec![
//!         MenuItem::Item { label: "New", size: 14.0 },
//!         MenuItem::Item { label: "Open", size: 14.0 },
//!         MenuItem::Item { label: "Save", size: 14.0 },
//!     ],
//!     ..Default::default()
//! };
//!
//! // Initialize the menu
//! let mut menu = Menu::default();
//! menu.configs = vec![menu_config];
//! menu.is_open = vec![false];
//!
//! // Handle messages from the menu
//! let message = MenuMessage::ItemPressed("Save".to_string());
//! let _task = menu.update(message);
//! ```
//!
//! # Menu Items
//!
//! - **`Item`**: Simple text label with press handling
//! - **`SubMenu`**: Dropdown with children, uses default label rendering
//! - **`SubElement`**: Dropdown with custom trigger content (for complex layouts)
//! - **`Element`**: Arbitrary widget (separators, checkboxes, custom layouts)
//!
//! # Rendering
//!
//! Use `menu.construct_menu()` to render the menu widget.
//! The menu handles all hover timing, submenu open/close logic internally.
//!

use iced::widget::{button, text, column, container, row, mouse_area};
use iced::{Alignment, Element, Task, Theme, Widget};
use std::time::{Duration, Instant};
use std::rc::Rc;

use crate::iced_widgets::iced_menu::popover::{Popover, Position};


#[derive(Clone)]
/// Configuration for a menu or submenu dropdown.
///
/// # Fields
/// - `label`: The text shown on the menu button
/// - `label_size`: Font size for the label
/// - `items`: Vec of menu items (can include submenus)
/// - `width`: Fixed width of the dropdown
/// - `gap`: Optional spacing between trigger and popover
/// - `position`: Where the popover opens (Right, Left, Top, Bottom)
/// - `align`: Vertical/horizontal alignment of popover content
///
/// # Example
/// ```ignore
/// MenuConfig {
///     label: "File",
///     label_size: 14.0,
///     width: 250.0,
///     items: vec![...],
///     ..Default::default()
/// }
/// ```
pub struct MenuConfig<'a, UserMsg = ()> {
    pub label: &'a str,
    pub label_size: f32,
    pub items: Vec<MenuItem<'a, UserMsg>>,
    pub width: f32,
    pub gap: Option<f32>,
    pub position: Option<Position>,
    pub align: Option<Alignment>,
}

// Manual Default so UserMsg: Default is never required
impl<'a, UserMsg> Default for MenuConfig<'a, UserMsg> {
    fn default() -> Self {
        MenuConfig {
            label: "",
            label_size: 14.0,
            items: Vec::new(),
            width: 200.0,
            gap: None,
            position: None,
            align: None,
        }
    }
}

#[derive(Clone)]
/// Menu item variants that can appear in a dropdown.
///
/// # Variants
///
/// - **`Item`**: A clickable menu entry with label and font size
///   ```ignore
///   MenuItem::Item { label: "Save", size: 14.0 }
///   ```
///
/// - **`SubMenu`**: A dropdown trigger with automatic label rendering
///   ```ignore
///   MenuItem::SubMenu {
///       config: MenuConfig {
///           label: "File >",
///           items: vec![...],
///           ..Default::default()
///       }
///   }
///   ```
///
/// - **`SubElement`**: A dropdown trigger with custom content (row, checkbox, etc.)
///   Use when you need complex layouts like icon + label + shortcut key
///   ```ignore
///   MenuItem::SubElement {
///       content: Rc::new(|| { /* return row with label, keys, icon */ }),
///       config: MenuConfig { ... }
///   }
///   ```
///
/// - **`Element`**: Arbitrary widget content (separators, checkboxes, custom layouts)
///   ```ignore
///   MenuItem::Element(Rc::new(|| iced::widget::rule::horizontal(2.0).boxed()))
///   ```
pub enum MenuItem<'a, UserMsg = ()> {
    Item{label: &'a str, size: f32},
    SubMenu {
        config: MenuConfig<'a, UserMsg>,
    },
    /// Like SubMenu but the trigger button's interior is provided by a closure.
    /// The closure must return content (a row, text, etc.) — not a button.
    SubElement {
        content: Rc<dyn Fn() -> Element<'a, MenuMessage<UserMsg>> + 'a>,
        config: MenuConfig<'a, UserMsg>,
    },
    Element(Rc<dyn Fn() -> Element<'a, MenuMessage<UserMsg>> + 'a>),
}


/// Menu state and message handler.
///
/// Manages dropdown open/close state, submenu navigation, and hover timing.
/// Update this with messages from `get_menu()` to respond to user interactions.
///
/// # Example
/// ```ignore
/// let mut menu = Menu::default();
/// menu.configs = vec![file_menu, edit_menu];
/// menu.is_open = vec![false, false];
///
/// // In your update handler:
/// match message {
///     Message::Menu(mm) => {
///         let task = menu.update(mm);
///         // ... handle task
///     }
/// }
/// ```
pub struct Menu<'a, UserMsg = ()> {
    pub configs: Vec<MenuConfig<'a, UserMsg>>,
    pub is_open: Vec<bool>,
    pub open_submenus: Vec<usize>,
    pub pending_submenu: Option<Vec<usize>>,
    pub pending_timer: Option<Instant>,
    pub pending_close: Option<Vec<usize>>,
    pub close_timer: Option<Instant>,
}

impl<'a, UserMsg> Menu<'a, UserMsg> {
    /// Create an empty menu.
    pub fn default() -> Self {
        Self {
            configs: Vec::new(),
            is_open: Vec::new(),
            open_submenus: Vec::new(),
            pending_submenu: None,
            pending_timer: None,
            pending_close: None,
            close_timer: None,
        }
    }

    /// Initialize menu with configurations and set up state.
    ///
    /// # Arguments
    /// * `menu_configs` - Vector of MenuConfig, one per top-level menu button
    ///
    /// # Example
    /// ```ignore
    /// let mut menu = Menu::default();
    /// let configs = vec![file_config, edit_config];
    /// menu = menu.new(configs);
    /// ```
    pub fn new(&mut self, menu_configs: Vec<MenuConfig<'a, UserMsg>>) -> Self {
        let mut m = Menu::default();
        m.is_open = vec![false; menu_configs.len()];
        m.configs = menu_configs;
        m
    }
}


#[derive(Debug, Clone)]
/// Messages emitted by menu interactions.
///
/// Most variants are internal; handle `ItemPressed` and `Custom` in your update loop.
///
/// # Important Variants
///
/// - `ItemPressed(String)`: User clicked a menu item; label is provided
/// - `Custom(UserMsg)`: Custom message from a menu widget (checkbox, button, etc.)
///
/// # Example
/// ```ignore
/// match message {
///     MenuMessage::ItemPressed(label) => println!("Clicked: {}", label),
///     MenuMessage::Custom(action) => handle_custom_action(action),
///     _ => { /* internal; forward to menu.update() */ }
/// }
/// ```
pub enum MenuMessage<UserMsg = ()> {
    Close(usize),
    OpenOnHover(usize),
    CloseSubmenu(Vec<usize>),
    StartCloseSubmenuTimer(Vec<usize>),
    CloseSubmenuAfterDelay(Vec<usize>),
    CancelCloseSubmenuTimer(Vec<usize>),
    ToggleSubmenu(Vec<usize>),
    OpenSubmenuOnHover(Vec<usize>),
    OpenSubmenuTimer(Vec<usize>),
    CancelSubmenuTimer(Vec<usize>),
    ItemPressed(String),
    Custom(UserMsg),
}

impl<'a, UserMsg: Clone + Send + 'static> Menu<'a, UserMsg> {
    pub fn update(&mut self, message: MenuMessage<UserMsg>) -> Task<MenuMessage<UserMsg>> {
        match message {
            MenuMessage::Close(idx) => {
                self.is_open[idx] = !self.is_open[idx];
                if !self.is_open[idx] {
                    self.open_submenus.clear();
                }
                self.pending_submenu = None;
                self.pending_timer = None;
                Task::none()
            },
            MenuMessage::OpenOnHover(idx) => {
                self.is_open.iter_mut().for_each(|o| *o = false);
                self.is_open[idx] = true;
                self.open_submenus.clear();
                self.pending_submenu = None;
                self.pending_timer = None;
                Task::none()
            },
            MenuMessage::ToggleSubmenu(path) => {
                if self.open_submenus == path {
                    self.open_submenus.clear();
                } else {
                    self.open_submenus = path;
                }
                self.pending_submenu = None;
                self.pending_timer = None;
                Task::none()
            },
            MenuMessage::OpenSubmenuOnHover(path) => {
                let path_clone = path.clone();
                self.pending_submenu = Some(path.clone());
                self.pending_timer = Some(Instant::now());
                
                // Cancel any pending close timer for the currently open submenu
                // This prevents parent from closing when entering a child submenu
                if self.pending_close == Some(self.open_submenus.clone()) {
                    self.pending_close = None;
                    self.close_timer = None;
                }
                
                // Spawn a background task that will send OpenSubmenuTimer after 500ms
                Task::perform(
                    async move {
                        // Sleep using a busy-wait loop
                        let start = Instant::now();
                        while start.elapsed() < Duration::from_millis(500) {
                            std::thread::sleep(Duration::from_millis(10));
                        }
                        path_clone
                    },
                    |path| MenuMessage::OpenSubmenuTimer(path)
                )
            },
            MenuMessage::CancelSubmenuTimer(path) => {
                if self.pending_submenu == Some(path) {
                    self.pending_submenu = None;
                    self.pending_timer = None;
                }
                Task::none()
            },
            MenuMessage::OpenSubmenuTimer(path) => {
                if self.pending_submenu == Some(path.clone()) {
                    self.open_submenus = path;
                    self.pending_submenu = None;
                    self.pending_timer = None;
                }
                Task::none()
            },
            MenuMessage::StartCloseSubmenuTimer(path) => {
                self.pending_close = Some(path.clone());
                self.close_timer = Some(Instant::now());
                
                Task::perform(
                    async move {
                        std::thread::sleep(Duration::from_millis(500));
                        path
                    },
                    |path| MenuMessage::CloseSubmenuAfterDelay(path)
                )
            },
            MenuMessage::CloseSubmenuAfterDelay(path) => {
                if self.pending_close == Some(path.clone()) {
                    // If open_submenus starts with our path, it means we're opening something deeper
                    let is_opening_child = self.open_submenus.len() > path.len() && 
                        self.open_submenus[0..path.len()] == path[..];
                    
                    if !is_opening_child {
                        // Only close this submenu and its descendants, not parent submenus
                        // If path is [0, 2], keep [0] open; if path is [0, 2, 2], keep [0, 2] open
                        if path.len() > 0 {
                            self.open_submenus.truncate(path.len() - 1);
                        } else {
                            self.open_submenus.clear();
                        }
                    }
                    self.pending_close = None;
                    self.close_timer = None;
                }
                Task::none()
            },
            MenuMessage::CancelCloseSubmenuTimer(path) => {
                // Cancel the timer if it matches this path, or if this path is an ancestor
                // of the pending close (moving from child back into parent area)
                if let Some(pending) = &self.pending_close {
                    if *pending == path || 
                        (pending.len() > path.len() && pending[0..path.len()] == path[..]) {
                        self.pending_close = None;
                        self.close_timer = None;
                    }
                }
                Task::none()
            },
            MenuMessage::CloseSubmenu(_path) => {
                self.open_submenus.clear();
                self.pending_submenu = None;
                self.pending_timer = None;
                self.pending_close = None;
                self.close_timer = None;
                Task::none()
            },
            MenuMessage::ItemPressed(_label) => {
                // Let the application handle item presses
                Task::none()
            },
            MenuMessage::Custom(_) => {
                // Let the application handle custom widget messages
                Task::none()
            },
        }
    }

    /// Clear all open menus and pending timers.
    pub fn clear_timers(&mut self) {
        self.is_open.iter_mut().for_each(|o| *o = false);
        self.open_submenus.clear();
        self.pending_submenu = None;
        self.pending_timer = None;
        self.pending_close = None;
        self.close_timer = None;
    }

    /// Render the menu bar as an Element.
    ///
    /// Call this in your view() and map its output to your Message type.
    ///
    /// # Example
    /// ```ignore
    /// let menu = menu.get_menu().map(Message::Menu);
    /// container(menu).padding(10.0).boxed()
    /// ```
    pub fn get_menu(&'a self) -> Element<'a, MenuMessage<UserMsg>> {

        let mut menu_bar = vec![];
        
        for (idx, config) in self.configs.iter().enumerate() {
            let pop = 
                if self.is_open[idx] {
                    let content = menu_with_subpopovers(config, self.open_submenus.clone(), vec![idx]);
                    Some(
                        container(content)
                            .style(|theme: &Theme| container::bordered_box(theme))
                            .padding(5.0)
                            .boxed()
                    )
                
            } else { None };
            
            let btn = button(text(config.label))
                    .on_press(MenuMessage::Close(idx))
                    .style(move|theme, status|
                            button::text(theme, status)
                        )   
                    .boxed();
            
            menu_bar.push(Popover::new(
                mouse_area(btn)
                    .on_enter(MenuMessage::OpenOnHover(idx))
                    .boxed(),
                pop,
            )
            .gap(5.0)
            .position(Position::Bottom)
            .align(Alignment::Start)
            .on_close(MenuMessage::Close(idx)).boxed());
        }
        
        row(menu_bar).boxed()

    }

}

// Menu construction
impl<UserMsg: Clone + 'static> Menu<'static, UserMsg> {
    pub fn construct_menu(&self) -> Element<'static, MenuMessage<UserMsg>> {
        let mut menu_bar = vec![];
        
        for (idx, config) in self.configs.iter().enumerate() {
            let pop = 
                if self.is_open[idx] {
                    let content = submenu_with_subpopovers(config, self.open_submenus.clone(), vec![idx]);
                    Some(
                        container(content)
                            .style(|theme: &Theme| container::bordered_box(theme))
                            .padding(5.0)
                            .boxed()
                    )
                
            } else { None };
            let label = 
                text(config.label).size(config.label_size).boxed();
            let btn = button(label)
                    .on_press(MenuMessage::Close(idx))
                    .style(move|theme, status|
                            button::text(theme, status)
                        )   
                    .boxed();
            
            menu_bar.push(Popover::new(
                mouse_area(btn)
                    .on_enter(MenuMessage::OpenOnHover(idx))
                    .boxed(),
                pop,
            )
            .gap(5.0)
            .position(Position::Bottom)
            .align(Alignment::Start)
            .on_close(MenuMessage::Close(idx)).boxed());
        }
        
        row(menu_bar).boxed()
    }
}

/// Menu items with subpopovers for each submenu
fn menu_with_subpopovers<'a, UserMsg: Clone + 'a>(
    config: &'a MenuConfig<'a, UserMsg>,
    open_submenus: Vec<usize>,
    current_path: Vec<usize>,
) -> Element<'a, MenuMessage<UserMsg>> {
    let buttons: Vec<Element<'a, MenuMessage<UserMsg>>> = config.items.iter().enumerate().map(|(idx, item)| {
        match item {
            MenuItem::Item {label, size} => {
                let lbl = 
                    text(*label).size(*size).boxed();
                button(lbl)
                    .width(iced::Length::Fill)
                    .on_press(MenuMessage::ItemPressed(label.to_string()))
                    .style(|theme, status|
                        button::text(theme, status))
                    .boxed()
            }
            MenuItem::Element(f) => f(),
            MenuItem::SubMenu { config: sub_config } => {
                let mut submenu_path = current_path.clone();
                submenu_path.push(idx);
                
                // A submenu is open if it matches open_submenus or if open_submenus is deeper and this is an ancestor
                let is_open = open_submenus == submenu_path || 
                    (open_submenus.len() > submenu_path.len() && 
                    open_submenus[0..submenu_path.len()] == submenu_path[..]);
                
                let submenu_content = if is_open {
                    let content = menu_with_subpopovers(
                        sub_config,
                        open_submenus.clone(),
                        submenu_path.clone(),
                    );
                    
                    let leave_path = submenu_path.clone();
                    let enter_path = submenu_path.clone();
                    
                    // Wrap content in mouse area first, then container for styling
                    let with_mouse_area = mouse_area(content)
                        .on_exit(MenuMessage::StartCloseSubmenuTimer(leave_path))
                        .on_enter(MenuMessage::CancelCloseSubmenuTimer(enter_path))
                        .boxed();
                    
                    let styled_content = 
                        container(with_mouse_area)
                        .width(iced::Length::Fixed(sub_config.width))
                        .style(|theme: &Theme| container::bordered_box(theme))
                        .padding(5.0)
                        .boxed();
                    
                    Some(styled_content)
                } else {
                    None
                };
                
                let toggle_path = submenu_path.clone();
                let hover_path = submenu_path.clone();
                let exit_path_cancel = submenu_path.clone();
                let exit_path_close = submenu_path.clone();
                let close_path = submenu_path;
                
                let label = 
                    text(sub_config.label)
                    .size(sub_config.label_size)
                    .boxed();
                let btn = 
                    button(label)
                    .width(iced::Length::Fill)
                    .on_press(MenuMessage::ToggleSubmenu(toggle_path))
                    .style(|theme, status| button::text(theme, status))
                    .boxed();
                
                let btn_with_mouse = if is_open {
                    // If submenu is open, exiting the button should start close timer
                    mouse_area(btn)
                        .on_enter(MenuMessage::OpenSubmenuOnHover(hover_path))
                        .on_exit(MenuMessage::StartCloseSubmenuTimer(exit_path_close))
                        .boxed()
                } else {
                    // If submenu is not open, exiting the button should cancel open timer
                    mouse_area(btn)
                        .on_enter(MenuMessage::OpenSubmenuOnHover(hover_path))
                        .on_exit(MenuMessage::CancelSubmenuTimer(exit_path_cancel))
                        .boxed()
                };
                
                Popover::new(
                    btn_with_mouse,
                    submenu_content,
                )
                .position(sub_config.position.unwrap_or(Position::Right))
                .align(config.align.unwrap_or(Alignment::Start))
                .gap(config.gap.unwrap_or_default())
                .on_close(MenuMessage::CloseSubmenu(close_path))
                .boxed()
            }
            MenuItem::SubElement { content, config: sub_config } => {
                let mut submenu_path = current_path.clone();
                submenu_path.push(idx);

                let is_open = open_submenus == submenu_path ||
                    (open_submenus.len() > submenu_path.len() &&
                    open_submenus[0..submenu_path.len()] == submenu_path[..]);

                let submenu_content = if is_open {
                    let inner = menu_with_subpopovers(
                        sub_config,
                        open_submenus.clone(),
                        submenu_path.clone(),
                    );
                    let leave_path = submenu_path.clone();
                    let enter_path = submenu_path.clone();
                    let with_mouse_area = mouse_area(inner)
                        .on_exit(MenuMessage::StartCloseSubmenuTimer(leave_path))
                        .on_enter(MenuMessage::CancelCloseSubmenuTimer(enter_path))
                        .boxed();
                    Some(container(with_mouse_area)
                        .width(iced::Length::Fixed(sub_config.width))
                        .style(|theme: &Theme| container::bordered_box(theme))
                        .padding(5.0)
                        .boxed())
                } else {
                    None
                };

                let toggle_path = submenu_path.clone();
                let hover_path = submenu_path.clone();
                let exit_path_cancel = submenu_path.clone();
                let exit_path_close = submenu_path.clone();
                let close_path = submenu_path;

                let btn = button(content())
                    .width(iced::Length::Fill)
                    .on_press(MenuMessage::ToggleSubmenu(toggle_path))
                    .style(|theme, status| button::text(theme, status))
                    .boxed();

                let btn_with_mouse = if is_open {
                    mouse_area(btn)
                        .on_enter(MenuMessage::OpenSubmenuOnHover(hover_path))
                        .on_exit(MenuMessage::StartCloseSubmenuTimer(exit_path_close))
                        .boxed()
                } else {
                    mouse_area(btn)
                        .on_enter(MenuMessage::OpenSubmenuOnHover(hover_path))
                        .on_exit(MenuMessage::CancelSubmenuTimer(exit_path_cancel))
                        .boxed()
                };

                Popover::new(btn_with_mouse, submenu_content)
                    .position(sub_config.position.unwrap_or(Position::Right))
                    .align(sub_config.align.unwrap_or(Alignment::Start))
                    .gap(sub_config.gap.unwrap_or_default())
                    .on_close(MenuMessage::CloseSubmenu(close_path))
                    .boxed()
            }
        }
    }).collect();

    container(column(buttons))
        .width(iced::Length::Fixed(config.width))
        .boxed()
}

/// SubMenus
fn submenu_with_subpopovers<UserMsg: Clone + 'static>(
    config: &MenuConfig<'static, UserMsg>,
    open_submenus: Vec<usize>,
    current_path: Vec<usize>,
) -> Element<'static, MenuMessage<UserMsg>> {
    let buttons: Vec<Element<'static, MenuMessage<UserMsg>>> = config.items.iter().enumerate().map(|(idx, item)| {
        match item {
            MenuItem::Item{label, size} => {
                let lbl = 
                    text(*label).size(*size).boxed();
                button(lbl)
                    .width(iced::Length::Fill)
                    .on_press(MenuMessage::ItemPressed(label.to_string()))
                    .style(|theme, status|
                        button::text(theme, status))
                    .boxed()
            }
            MenuItem::Element(f) => f(),
            MenuItem::SubMenu { config: sub_config } => {
                let mut submenu_path = current_path.clone();
                submenu_path.push(idx);
                
                let is_open = open_submenus == submenu_path || 
                    (open_submenus.len() > submenu_path.len() && 
                    open_submenus[0..submenu_path.len()] == submenu_path[..]);
                
                let submenu_content = if is_open {
                    let content = submenu_with_subpopovers(
                        sub_config,
                        open_submenus.clone(),
                        submenu_path.clone(),
                    );
                    
                    let leave_path = submenu_path.clone();
                    let enter_path = submenu_path.clone();
                    
                    let with_mouse_area = mouse_area(content)
                        .on_exit(MenuMessage::StartCloseSubmenuTimer(leave_path))
                        .on_enter(MenuMessage::CancelCloseSubmenuTimer(enter_path))
                        .boxed();
                    
                    let styled_content = 
                        container(with_mouse_area)
                        .width(iced::Length::Fixed(sub_config.width))
                        .style(|theme: &Theme| container::bordered_box(theme))
                        .padding(5.0)
                        .boxed();
                    
                    Some(styled_content)
                } else {
                    None
                };
                
                let toggle_path = submenu_path.clone();
                let hover_path = submenu_path.clone();
                let exit_path_cancel = submenu_path.clone();
                let exit_path_close = submenu_path.clone();
                let close_path = submenu_path;
                
                let label = 
                    text(sub_config.label)
                    .size(sub_config.label_size)
                    .boxed();
                let btn = 
                    button(label)
                    .width(iced::Length::Fill)
                    .on_press(MenuMessage::ToggleSubmenu(toggle_path))
                    .style(|theme, status| button::text(theme, status))
                    .boxed();
                
                let btn_with_mouse = if is_open {
                    mouse_area(btn)
                        .on_enter(MenuMessage::OpenSubmenuOnHover(hover_path))
                        .on_exit(MenuMessage::StartCloseSubmenuTimer(exit_path_close))
                        .boxed()
                } else {
                    mouse_area(btn)
                        .on_enter(MenuMessage::OpenSubmenuOnHover(hover_path))
                        .on_exit(MenuMessage::CancelSubmenuTimer(exit_path_cancel))
                        .boxed()
                };
                
                Popover::new(
                    btn_with_mouse,
                    submenu_content,
                )
                .position(sub_config.position.unwrap_or(Position::Right))
                .align(sub_config.align.unwrap_or(Alignment::Start))
                .gap(sub_config.gap.unwrap_or_default())
                .on_close(MenuMessage::CloseSubmenu(close_path))
                .boxed()
            }
            MenuItem::SubElement { content, config: sub_config } => {
                let mut submenu_path = current_path.clone();
                submenu_path.push(idx);

                let is_open = open_submenus == submenu_path ||
                    (open_submenus.len() > submenu_path.len() &&
                    open_submenus[0..submenu_path.len()] == submenu_path[..]);

                let submenu_content = if is_open {
                    let inner = submenu_with_subpopovers(
                        sub_config,
                        open_submenus.clone(),
                        submenu_path.clone(),
                    );
                    let leave_path = submenu_path.clone();
                    let enter_path = submenu_path.clone();
                    let with_mouse_area = mouse_area(inner)
                        .on_exit(MenuMessage::StartCloseSubmenuTimer(leave_path))
                        .on_enter(MenuMessage::CancelCloseSubmenuTimer(enter_path))
                        .boxed();
                    Some(container(with_mouse_area)
                        .width(iced::Length::Fixed(sub_config.width))
                        .style(|theme: &Theme| container::bordered_box(theme))
                        .padding(5.0)
                        .boxed())
                } else {
                    None
                };

                let toggle_path = submenu_path.clone();
                let hover_path = submenu_path.clone();
                let exit_path_cancel = submenu_path.clone();
                let exit_path_close = submenu_path.clone();
                let close_path = submenu_path;

                let btn = button(content())
                    .width(iced::Length::Fill)
                    .on_press(MenuMessage::ToggleSubmenu(toggle_path))
                    .style(|theme, status| button::text(theme, status))
                    .boxed();

                let btn_with_mouse = if is_open {
                    mouse_area(btn)
                        .on_enter(MenuMessage::OpenSubmenuOnHover(hover_path))
                        .on_exit(MenuMessage::StartCloseSubmenuTimer(exit_path_close))
                        .boxed()
                } else {
                    mouse_area(btn)
                        .on_enter(MenuMessage::OpenSubmenuOnHover(hover_path))
                        .on_exit(MenuMessage::CancelSubmenuTimer(exit_path_cancel))
                        .boxed()
                };

                Popover::new(btn_with_mouse, submenu_content)
                    .position(sub_config.position.unwrap_or(Position::Right))
                    .align(sub_config.align.unwrap_or(Alignment::Start))
                    .gap(sub_config.gap.unwrap_or_default())
                    .on_close(MenuMessage::CloseSubmenu(close_path))
                    .boxed()
            }
        }
    }).collect();

    container(column(buttons))
        .width(iced::Length::Fixed(config.width))
        .boxed()
}
