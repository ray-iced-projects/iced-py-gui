//! ipg_menu

#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;
use std::time::{Duration, Instant};

use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{Alignment, Element, Length, Renderer, Task, Theme, Widget};

use crate::IpgState;
use crate::app::Message;
use crate::iced_widgets::popover::{Popover, Position};
use crate::py_api::helpers::{get_len, get_padding};
use crate::state::Widgets;
use crate::widgets::callbacks::{CallbackName, invoke_callback};

use crate::widgets::ipg_button::{ButtonStyle, ButtonStyleStd};
use crate::widgets::ipg_container::ContainerStyleStd;
use crate::{
    app,
    state::Containers,
    widgets::widget_param_update::{WidgetParamUpdate, set_t_value},
};

use pyo3::{Py, PyAny, pyclass};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;

/// A dropdown item that is either a plain widget or a sub-menu.
/// This is produced by `get_menu_children` in `app.rs` and consumed
/// by `Menu::construct`.
pub enum GroupedItem<'a> {
    Plain(Element<'a, app::Message>),
    Sub {
        trigger: Element<'a, app::Message>,
        children: Vec<GroupedItem<'a>>,
        sub_item_id: usize,
    },
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub id: usize,
    pub is_open: Vec<bool>,
    pub open_submenus: Vec<usize>,
    pub pending_submenu: Option<Vec<usize>>,
    pub pending_timer: Option<Instant>,
    pub pending_close: Option<Vec<usize>>,
    pub close_timer: Option<Instant>,
}

impl Menu {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }
    
    pub fn construct<'a>(
        &'a self,
        grouped_content: Vec<(usize, Vec<GroupedItem<'a>>)>,
        widgets: &'a HashMap<usize, Widgets>,
        containers: &'a HashMap<usize, Containers>,
    ) -> Option<Element<'a, app::Message, Theme, Renderer>> {
        
        if grouped_content.is_empty() {
            return None;
        }

        let menu_id = self.id;
        let mut bar_elements: Vec<Element<'a, app::Message>> = vec![];

        for (bar_idx, (bar_id, items)) in grouped_content.into_iter().enumerate() {
            let bar = match containers.get(&bar_id) {
                Some(Containers::MenuBar(b)) => b,
                _ => continue,
            };
            
            let is_open = self.is_open.get(bar_idx).copied().unwrap_or(false);

            let dropdown = if is_open {
                let content = render_items(items, &self.open_submenus, vec![bar_idx], menu_id, containers, bar.dropdown_width, bar.spacing.unwrap_or_default());

                let style_opt = 
                    self.lookup(widgets, bar.container_style_id)
                        .and_then(Widgets::as_container_style).cloned();
                
                let padding = get_padding(&bar.padding);
                
                Some(
                    container(content)
                        .padding(padding)
                        .style(move|theme|
                            if let Some(st) = &style_opt {
                                st.to_iced(theme, &bar.container_style_std)
                            } else {
                            match &bar.container_style_std {
                                    Some(std) => std.to_iced(theme),
                                    None => container::bordered_box(theme),
                                }
                            }
                        )
                        .boxed(),
                )
            } else {
                None
            };

            let label_el = text(bar.label.clone()).size(bar.label_size.unwrap_or(13.0)).boxed();
            let btn = button(label_el)
                .on_press(Message::Menu(menu_id, MenuMessage::Close(bar_idx)))
                .width(bar.width)
                .padding(0)
                .style(|theme, status| button::text(theme, status))
                .boxed();

            let btn_with_hover = mouse_area(btn)
                .on_enter(Message::Menu(menu_id, MenuMessage::OpenOnHover(bar_idx)))
                .boxed();

            let pop = Popover::new(btn_with_hover, dropdown)
                .gap(bar.gap.unwrap_or(5.0))
                .position(Position::Bottom)
                .align(Alignment::Start)
                .on_close(Message::Menu(menu_id, MenuMessage::Close(bar_idx)))
                .boxed();

            bar_elements.push(pop);
        }

        if bar_elements.is_empty() {
            return None;
        }

        Some(row(bar_elements).padding(10.0).boxed())
    }
}

fn render_items<'a>(
    items: Vec<GroupedItem<'a>>,
    open_submenus: &[usize],
    current_path: Vec<usize>,
    menu_id: usize,
    containers: &'a HashMap<usize, Containers>,
    bar_width: f32,
    spacing: f32,
) -> Element<'a, Message> {
    let mut elements: Vec<Element<'a, Message>> = vec![];

    for (idx, item) in items.into_iter().enumerate() {
        match item {
            GroupedItem::Plain(el) => {
                elements.push(el);
            }
            GroupedItem::Sub {
                trigger,
                children,
                sub_item_id,
            } => {
                let mut submenu_path = current_path.clone();
                submenu_path.push(idx);

                let is_open = open_submenus == submenu_path.as_slice()
                    || (open_submenus.len() > submenu_path.len()
                        && open_submenus[..submenu_path.len()] == submenu_path[..]);

                let (sub_width, sub_position, sub_gap) = match containers.get(&sub_item_id) {
                    Some(Containers::SubMenu(sm)) => (sm.width, sm.position, sm.gap.unwrap_or(5.0)),
                    _ => (200.0, Position::Right, 5.0),
                };

                let submenu_el = if is_open {
                    let inner = render_items(children, open_submenus, submenu_path.clone(), menu_id, containers, bar_width, spacing);
                    let leave_path = submenu_path.clone();
                    let enter_path = submenu_path.clone();
                    let with_mouse = mouse_area(inner)
                        .on_exit(Message::Menu(menu_id, MenuMessage::StartCloseSubmenuTimer(leave_path)))
                        .on_enter(Message::Menu(menu_id, MenuMessage::CancelCloseSubmenuTimer(enter_path)))
                        .boxed();
                    Some(
                        container(with_mouse)
                            .width(Length::Fixed(sub_width))
                            .style(|theme: &Theme| container::bordered_box(theme))
                            .padding(5.0)
                            .boxed(),
                    )
                } else {
                    None
                };

                let toggle_path = submenu_path.clone();
                let hover_path = submenu_path.clone();
                let exit_cancel = submenu_path.clone();
                let exit_close = submenu_path.clone();
                let close_path = submenu_path;

                let btn = button(trigger)
                    .width(Length::Fill)
                    .on_press(Message::Menu(menu_id, MenuMessage::ToggleSubmenu(toggle_path)))
                    .padding(0.0)
                    .style(|theme, status| button::text(theme, status))
                    .boxed();

                let btn_with_mouse = if is_open {
                    mouse_area(btn)
                        .on_enter(Message::Menu(menu_id, MenuMessage::OpenSubmenuOnHover(hover_path)))
                        .on_exit(Message::Menu(menu_id, MenuMessage::StartCloseSubmenuTimer(exit_close)))
                        .boxed()
                } else {
                    mouse_area(btn)
                        .on_enter(Message::Menu(menu_id, MenuMessage::OpenSubmenuOnHover(hover_path)))
                        .on_exit(Message::Menu(menu_id, MenuMessage::CancelSubmenuTimer(exit_cancel)))
                        .boxed()
                };

                let pop = Popover::new(btn_with_mouse, submenu_el)
                    .position(sub_position)
                    .align(Alignment::Start)
                    .gap(sub_gap)
                    .on_close(Message::Menu(menu_id, MenuMessage::CloseSubmenu(close_path)))
                    .boxed();

                elements.push(pop);
            }
        }
    }

    column(elements).width(bar_width).spacing(spacing).boxed()
}

pub fn get_position(dropdowns: [Option<bool>; 5]) -> Position {
    match dropdowns {
        [Some(true), None, None, None, None] => Position::Auto,
        [None, Some(true), None, None, None] => Position::Bottom,
        [None, None, Some(true), None, None] => Position::Left,
        [None, None, None, Some(true), None] => Position::Right,
        [None, None, None, None, Some(true)] => Position::Top,
        _ => Position::Right,
    }
}

pub struct MenuStyleOverrides {
    pub width_fill: Option<bool>,
    pub align_center: Option<bool>,
}

impl MenuStyleOverrides {
    pub fn default() -> Self {
        Self {
            width_fill: Some(true),
            align_center: Some(true),
        }
    }
}

// The dropdown via the bar button
#[derive(Debug, Clone, Default)]
pub struct MenuBar {
    pub id: usize,
    pub label: String,
    pub label_size: Option<f32>,
    pub width: f32,
    pub dropdown_width: f32,
    pub position: Position,
    pub spacing: Option<f32>,
    pub gap: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub container_style_id: Option<usize>,
    pub container_style_std: Option<ContainerStyleStd>,
}

#[derive(Debug, Clone, Default)]
pub struct SubMenu {
    pub id: usize,
    pub width: f32,
    pub position: Position,
    pub spacing: Option<f32>,
    pub gap: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub container_style_id: Option<usize>,
    pub container_style_std: Option<ContainerStyleStd>,
}

#[derive(Debug, Clone, Default)]
pub struct MenuItem {
    pub id: usize,
}

#[derive(Debug, Clone)]
pub enum MenuMessage {
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
    OnPress,
}

pub fn menu_callback(state: &mut IpgState, id: usize, message: MenuMessage) -> Task<Message> {
    match message {
        MenuMessage::OnPress => {
            // id is the MenuButton's id; invoke its Python callback then close all menus
            invoke_callback(id, CallbackName::OnPress, "MenuButton");
            for (_, cont) in state.containers.iter_mut() {
                if let Containers::Menu(m) = cont {
                    m.is_open.iter_mut().for_each(|o| *o = false);
                    m.open_submenus.clear();
                    m.pending_submenu = None;
                    m.pending_timer = None;
                    m.pending_close = None;
                    m.close_timer = None;
                }
            }
            Task::none()
        }
        MenuMessage::Close(idx) => {
            let menu = match state.containers.get_mut(&id) {
                Some(Containers::Menu(m)) => m,
                _ => return Task::none(),
            };
            if let Some(flag) = menu.is_open.get_mut(idx) {
                *flag = !*flag;
                if !*flag {
                    menu.open_submenus.clear();
                }
            } else {
                menu.is_open.resize(idx + 1, false);
                menu.is_open[idx] = true;
            }
            menu.pending_submenu = None;
            menu.pending_timer = None;
            Task::none()
        }
        MenuMessage::OpenOnHover(idx) => {
            let menu = match state.containers.get_mut(&id) {
                Some(Containers::Menu(m)) => m,
                _ => return Task::none(),
            };
            if menu.is_open.len() <= idx {
                menu.is_open.resize(idx + 1, false);
            }
            menu.is_open.iter_mut().for_each(|o| *o = false);
            menu.is_open[idx] = true;
            menu.open_submenus.clear();
            menu.pending_submenu = None;
            menu.pending_timer = None;
            Task::none()
        }
        MenuMessage::ToggleSubmenu(path) => {
            let menu = match state.containers.get_mut(&id) {
                Some(Containers::Menu(m)) => m,
                _ => return Task::none(),
            };
            if menu.open_submenus == path {
                menu.open_submenus.clear();
            } else {
                menu.open_submenus = path;
            }
            menu.pending_submenu = None;
            menu.pending_timer = None;
            Task::none()
        }
        MenuMessage::OpenSubmenuOnHover(path) => {
            let menu = match state.containers.get_mut(&id) {
                Some(Containers::Menu(m)) => m,
                _ => return Task::none(),
            };
            let path_clone = path.clone();
            menu.pending_submenu = Some(path.clone());
            menu.pending_timer = Some(Instant::now());
            if menu.pending_close == Some(menu.open_submenus.clone()) {
                menu.pending_close = None;
                menu.close_timer = None;
            }
            Task::perform(
                async move {
                    let start = Instant::now();
                    while start.elapsed() < Duration::from_millis(500) {
                        std::thread::sleep(Duration::from_millis(10));
                    }
                    path_clone
                },
                move |p| Message::Menu(id, MenuMessage::OpenSubmenuTimer(p)),
            )
        }
        MenuMessage::CancelSubmenuTimer(path) => {
            let menu = match state.containers.get_mut(&id) {
                Some(Containers::Menu(m)) => m,
                _ => return Task::none(),
            };
            if menu.pending_submenu == Some(path) {
                menu.pending_submenu = None;
                menu.pending_timer = None;
            }
            Task::none()
        }
        MenuMessage::OpenSubmenuTimer(path) => {
            let menu = match state.containers.get_mut(&id) {
                Some(Containers::Menu(m)) => m,
                _ => return Task::none(),
            };
            if menu.pending_submenu == Some(path.clone()) {
                menu.open_submenus = path;
                menu.pending_submenu = None;
                menu.pending_timer = None;
            }
            Task::none()
        }
        MenuMessage::StartCloseSubmenuTimer(path) => {
            let menu = match state.containers.get_mut(&id) {
                Some(Containers::Menu(m)) => m,
                _ => return Task::none(),
            };
            menu.pending_close = Some(path.clone());
            menu.close_timer = Some(Instant::now());
            Task::perform(
                async move {
                    std::thread::sleep(Duration::from_millis(500));
                    path
                },
                move |p| Message::Menu(id, MenuMessage::CloseSubmenuAfterDelay(p)),
            )
        }
        MenuMessage::CloseSubmenuAfterDelay(path) => {
            let menu = match state.containers.get_mut(&id) {
                Some(Containers::Menu(m)) => m,
                _ => return Task::none(),
            };
            if menu.pending_close == Some(path.clone()) {
                let is_opening_child =
                    menu.open_submenus.len() > path.len() && menu.open_submenus[..path.len()] == path[..];
                if !is_opening_child {
                    if path.is_empty() {
                        menu.open_submenus.clear();
                    } else {
                        menu.open_submenus.truncate(path.len() - 1);
                    }
                }
                menu.pending_close = None;
                menu.close_timer = None;
            }
            Task::none()
        }
        MenuMessage::CancelCloseSubmenuTimer(path) => {
            let menu = match state.containers.get_mut(&id) {
                Some(Containers::Menu(m)) => m,
                _ => return Task::none(),
            };
            if let Some(pending) = &menu.pending_close {
                if *pending == path || (pending.len() > path.len() && pending[..path.len()] == path[..]) {
                    menu.pending_close = None;
                    menu.close_timer = None;
                }
            }
            Task::none()
        }
        MenuMessage::CloseSubmenu(_) => {
            let menu = match state.containers.get_mut(&id) {
                Some(Containers::Menu(m)) => m,
                _ => return Task::none(),
            };
            menu.open_submenus.clear();
            menu.pending_submenu = None;
            menu.pending_timer = None;
            menu.pending_close = None;
            menu.close_timer = None;
            Task::none()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuParam {
    None,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuBarParam {
    None,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SubMenuParam {
    None,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Menu {
    type Param = MenuParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            MenuParam::None => todo!(),
        }
    }
}

impl WidgetParamUpdate for MenuBar {
    type Param = MenuBarParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            MenuBarParam::None => todo!(),
        }
    }
}

impl WidgetParamUpdate for MenuItem {
    type Param = SubMenuParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SubMenuParam::None => todo!(),
        }
    }
}
