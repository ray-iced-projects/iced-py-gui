

use iced::widget::{button, text, column, container, row, mouse_area, space};
use iced::{Alignment, Color, Element, Subscription, Task, Theme, Widget};
use std::time::{Duration, Instant};

use crate::iced_widgets::popover::{Popover, Position};
use crate::ipg_widgets::ipg_menu::custom_quad::CustomQuad;


pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(Theme::TokyoNight)
        .subscription(App::subscription)
        .run()
}


#[derive(Debug, Clone)]
pub enum MenuItem<'a> {
    Item(&'a str),
    SubMenu {
        label: &'a str,
        items: Vec<MenuItem<'a>>,
    },
    Custom {
        key: &'a str,
    },
}

struct App<'a> {
    is_open: Vec<bool>,
    open_submenus: Vec<usize>,
    dropdown_labels: DropdownLabels<'a>,
    pending_submenu: Option<Vec<usize>>,
    pending_timer: Option<Instant>,
    pending_close: Option<Vec<usize>>,
    close_timer: Option<Instant>,
}

#[derive(Debug, Clone)]
enum Message {
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
    File(String),
}

impl <'a>App<'a> {
    fn default() -> Self {
        Self {
            is_open: vec![false; 8],
            open_submenus: Vec::new(),
            dropdown_labels: DropdownLabels::default(),
            pending_submenu: None,
            pending_timer: None,
            pending_close: None,
            close_timer: None,
        }
    }
}

impl <'a>App<'a> {
    
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Close(idx) => {
                self.is_open[idx] = !self.is_open[idx];
                if !self.is_open[idx] {
                    self.open_submenus.clear();
                }
                self.pending_submenu = None;
                self.pending_timer = None;
                Task::none()
            },
            Message::OpenOnHover(idx) => {
                self.is_open.iter_mut().for_each(|o| *o = false);
                self.is_open[idx] = true;
                self.open_submenus.clear();
                self.pending_submenu = None;
                self.pending_timer = None;
                Task::none()
            },
            Message::ToggleSubmenu(path) => {
                if self.open_submenus == path {
                    self.open_submenus.clear();
                } else {
                    self.open_submenus = path;
                }
                self.pending_submenu = None;
                self.pending_timer = None;
                Task::none()
            },
            Message::OpenSubmenuOnHover(path) => {
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
                    |path| Message::OpenSubmenuTimer(path)
                )
            },
            Message::CancelSubmenuTimer(path) => {
                if self.pending_submenu == Some(path) {
                    self.pending_submenu = None;
                    self.pending_timer = None;
                }
                Task::none()
            },
            Message::OpenSubmenuTimer(path) => {
                if self.pending_submenu == Some(path.clone()) {
                    self.open_submenus = path;
                    self.pending_submenu = None;
                    self.pending_timer = None;
                }
                Task::none()
            },
            Message::StartCloseSubmenuTimer(path) => {
                self.pending_close = Some(path.clone());
                self.close_timer = Some(Instant::now());
                
                Task::perform(
                    async move {
                        std::thread::sleep(Duration::from_millis(500));
                        path
                    },
                    |path| Message::CloseSubmenuAfterDelay(path)
                )
            },
            Message::CloseSubmenuAfterDelay(path) => {
                if self.pending_close == Some(path.clone()) {
                    // If open_submenus starts with our path, it means we're opening something deeper
                    let is_opening_child = self.open_submenus.len() > path.len() && 
                        self.open_submenus[0..path.len()] == path[..];
                    
                    if !is_opening_child {
                        self.open_submenus.clear();
                    }
                    self.pending_close = None;
                    self.close_timer = None;
                }
                Task::none()
            },
            Message::CancelCloseSubmenuTimer(path) => {
                if self.pending_close == Some(path) {
                    self.pending_close = None;
                    self.close_timer = None;
                }
                Task::none()
            },
            Message::CloseSubmenu(_path) => {
                self.open_submenus.clear();
                self.pending_submenu = None;
                self.pending_timer = None;
                self.pending_close = None;
                self.close_timer = None;
                Task::none()
            },
            Message::File(label) => {
                if label.is_empty() {
                    return Task::none();
                }
                dbg!(&label);
                self.is_open.iter_mut().for_each(|o| *o = false);
                self.open_submenus.clear();
                self.pending_submenu = None;
                self.pending_timer = None;
                self.pending_close = None;
                self.close_timer = None;
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let mb = get_menu_bar(&self);
        
        container(mb)
        .padding(10.0)
        .boxed()

    }
}


fn get_menu_bar<'a>(app: &'a App) -> Element<'a, Message> {

    let labels = ["File", "Edit", "Selection", "View", "Go", "Run", "Terminal", "Help"];

    let mut menu_bar = vec![];
    
    for (idx, label) in labels.iter().enumerate() {
        let pop = match *label {
            "File" if app.is_open[idx] => {
                let content = render_menu_with_subpopovers(&app.dropdown_labels.file, app.open_submenus.clone(), vec![idx], &app);
                Some(
                    container(content)
                        .style(|theme: &Theme| container::bordered_box(theme))
                        .padding(5.0)
                        .boxed()
                )
            },
            "Edit" if app.is_open[idx] => {
                let content = render_menu_with_subpopovers(&app.dropdown_labels.edit, app.open_submenus.clone(), vec![idx], &app);
                Some(
                    container(content)
                        .style(|theme: &Theme| container::bordered_box(theme))
                        .padding(5.0)
                        .boxed()
                )
            },
            _ => None,
        };
        
        let btn = button(text(*label))
                .on_press(Message::Close(idx))
                .style(move|theme, status|
                        button::text(theme, status)
                    )   
                .boxed();
        
        menu_bar.push(Popover::new(
            mouse_area(btn)
                .on_enter(Message::OpenOnHover(idx))
                .boxed(),
            pop,
        )
        .gap(5.0)
        .position(Position::Bottom)
        .align(Alignment::Start)
        .on_close(Message::Close(idx)).boxed());
    }
    
    row(menu_bar).boxed()

}


struct DropdownLabels<'a> {
    file: Vec<MenuItem<'a>>,
    edit: Vec<MenuItem<'a>>,
}

impl <'a>DropdownLabels<'a> {
    fn default() -> Self {
        Self {
            file: vec![
                MenuItem::Item("New File"),
                MenuItem::Item("New Window"),
                MenuItem::SubMenu {
                    label: "New Window with Profile >",
                    items: vec![
                        MenuItem::Item("Profile 1"),
                        MenuItem::Item("Profile 2"),
                        MenuItem::SubMenu {
                            label: "Additional Profiles >",
                            items: vec![
                                MenuItem::Item("Profile 3"),
                                MenuItem::Item("Profile 4"),
                                MenuItem::Item("Profile 5"),
                            ],
                        },
                    ],
                },
                MenuItem::Custom { key: "line" },
                MenuItem::Item("Open File"),
                MenuItem::Item("Open Folder"),
                MenuItem::Item("Open Wokspace from File"),
                MenuItem::SubMenu {
                    label: "Open Recent >",
                    items: vec![
                        MenuItem::Item("~/document/iced_menu"),
                    ],
                },
                MenuItem::Custom { key: "line" },
                MenuItem::Item("Save"),
                MenuItem::Item("Save As..."),
                MenuItem::Item("Save All"),
                MenuItem::Item("Revert File"),
                MenuItem::Item("Exit"),
            ],
            edit: vec![
                MenuItem::Item("Undo"),
                MenuItem::Item("Redo"),
                MenuItem::Item("Cut"),
                MenuItem::Item("Copy"),
                MenuItem::Item("Paste"),
                MenuItem::SubMenu {
                    label: "Find >",
                    items: vec![
                        MenuItem::Item("Find"),
                        MenuItem::Item("Find Next"),
                        MenuItem::Item("Find Previous"),
                    ],
                },
                MenuItem::Item("Replace"),
                MenuItem::Item("Find in Files"),
                MenuItem::Item("Replace in Files"),
            ]
        }
    }
}
/// Renders menu items with subpopovers for each submenu
fn render_menu_with_subpopovers<'a>(
    items: &'a [MenuItem<'a>],
    open_submenus: Vec<usize>,
    current_path: Vec<usize>,
    app: &'a App,
) -> Element<'a, Message> {
    let buttons: Vec<Element<'a, Message>> = items.iter().enumerate().map(|(idx, item)| {
        match item {
            MenuItem::Item(label) => {
                button(text(*label))
                    .on_press(Message::File(label.to_string()))
                    .style(|theme, status|
                        button::text(theme, status))
                    .boxed()
            }
            MenuItem::Custom { key } => {
                // Create custom widgets based on key
                match *key {
                    "line" => {
                        get_line(200.0, 2.0, Color::from_rgba(1.0, 1.0, 1.0, 0.1))
                            .boxed()
                    }
                    _ => {
                        // Fallback for unknown keys
                        space().boxed()
                    }
                }
            }
            MenuItem::SubMenu { label, items: sub_items } => {
                let mut submenu_path = current_path.clone();
                submenu_path.push(idx);
                
                // A submenu is open if it matches open_submenus exactly, OR if open_submenus is deeper and this is an ancestor
                let is_open = open_submenus == submenu_path || 
                    (open_submenus.len() > submenu_path.len() && 
                     open_submenus[0..submenu_path.len()] == submenu_path[..]);
                
                let submenu_content = if is_open {
                    let content = render_menu_with_subpopovers(
                        sub_items,
                        open_submenus.clone(),
                        submenu_path.clone(),
                        &app,
                    );
                    
                    let leave_path = submenu_path.clone();
                    let enter_path = submenu_path.clone();
                    
                    // Wrap content in mouse area first, then container for styling
                    // Nee the ma for timing notifications.
                    let with_mouse_area = mouse_area(content)
                        .on_exit(Message::StartCloseSubmenuTimer(leave_path))
                        .on_enter(Message::CancelCloseSubmenuTimer(enter_path))
                        .boxed();
                    
                    let styled_content = 
                        container(with_mouse_area)
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
                
                let btn = button(text(*label))
                    .on_press(Message::ToggleSubmenu(toggle_path))
                    .style(|theme, status| button::text(theme, status))
                    .boxed();
                
                let btn_with_mouse = if is_open {
                    // If submenu is open, exiting the button should start close timer
                    mouse_area(btn)
                        .on_enter(Message::OpenSubmenuOnHover(hover_path))
                        .on_exit(Message::StartCloseSubmenuTimer(exit_path_close))
                        .boxed()
                } else {
                    // If submenu is not open, exiting the button should cancel open timer
                    mouse_area(btn)
                        .on_enter(Message::OpenSubmenuOnHover(hover_path))
                        .on_exit(Message::CancelSubmenuTimer(exit_path_cancel))
                        .boxed()
                };
                
                Popover::new(
                    btn_with_mouse,
                    submenu_content,
                )
                .position(Position::Right)
                .align(Alignment::Start)
                .gap(0.0)
                .on_close(Message::CloseSubmenu(close_path))
                .boxed()
            }
        }
    }).collect();

    column(buttons).boxed()
}

fn get_line(
        width: f32,
        height: f32,
        fill_color: iced::Color) 
        -> Element<'static, Message> {
    
    CustomQuad::new(
        width, 
        height, 
        fill_color).boxed()
}
