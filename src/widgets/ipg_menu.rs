//! ipg_menu

#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{Element, Length, Renderer, Theme, Widget, alignment};

use crate::IpgState;
use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::state::Widgets;
use crate::widgets::callbacks::{CallbackName, invoke_callback_with_args};

use crate::iced_widgets::popover::{Popover, Position};
use crate::widgets::ipg_button::{ButtonStyle, ButtonStyleStd};
use crate::widgets::ipg_container::ContainerStyleStd;
use crate::{
    app,
    graphics::colors::Color,
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
    pub bar_labels: Vec<String>,
    pub bar_widths: Option<Vec<f32>>,
    pub bar_container_style_id: Option<usize>,
    pub bar_container_style_std: Option<ContainerStyleStd>,
    pub bar_container_palette_id: Option<usize>,
    pub bar_labels_text_style_id: Option<usize>,
    pub bar_labels_text_font_id: Option<usize>,
    pub bar_btn_style_id: Option<usize>,
    pub bar_btn_style_std: Option<ButtonStyleStd>,
    pub bar_btn_palette_id: Option<usize>,
    pub bar_btn_font_id: Option<usize>,
    pub padding: Option<Vec<f32>>,
    pub spacing: Option<f32>,
    pub height: Option<f32>,
    pub close_on_bar_item_click: Option<bool>,
    pub close_on_bar_background_click: Option<bool>,
    pub items_close_on_click_global: Option<bool>,
    pub items_close_on_background_click_global: Option<bool>,
    pub show: bool,
    pub is_open: Vec<bool>,
    pub dropdown_is_open: Vec<HashMap<String, bool>>,
    pub dropdown_open_auto: Option<bool>,
    pub dropdown_open_top: Option<bool>,
    pub dropdown_open_bottom: Option<bool>,
    pub dropdown_open_left: Option<bool>,
    pub dropdown_open_right: Option<bool>,
}

impl Menu {
    fn lookup<'a>(&self, containers: &'a HashMap<usize, Containers>, id: Option<usize>) -> Option<&'a Containers> {
        id.and_then(|id| containers.get(&id))
    }

    fn lookup_widgets<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        grouped_content: Vec<(usize, Vec<GroupedItem<'a>>)>,
        widgets: &'a HashMap<usize, Widgets>,
        containers: &'a HashMap<usize, Containers>,
    ) -> Option<Element<'a, app::Message, Theme, Renderer>> {
        let mut bar_columns = vec![];

        let mut index = 0;

        for (menu_bar_item_id, group) in grouped_content.into_iter() {
            let dropdown_is_open = self.dropdown_is_open.get(index).cloned().unwrap_or_default();
            let dropdowns = build_dropdown_items(
                self,
                widgets,
                group,
                containers,
                menu_bar_item_id,
                index,
                &dropdown_is_open,
                "",
            );
            bar_columns.push(
                container(column(dropdowns).boxed())
                    // .width(bar_widths[index])
                    .style(move |theme| container::bordered_box(theme))
                    .boxed(),
            );
            index += 1;
        }

        // Constructs the menu bar
        let bar_widths = match self.bar_widths.clone() {
            Some(widths) => {
                if widths.len() == 1 {
                    vec![widths[0]; self.bar_labels.len()]
                } else {
                    widths
                }
            }
            None => vec![],
        };

        let bar_text_labels = configure_bar_labels(
            self,
            &self.bar_labels,
            self.bar_labels_text_style_id,
            self.bar_labels_text_font_id,
            widgets,
        );

        let id = self.id;
        let rw = row(bar_text_labels
            .into_iter()
            .zip(bar_columns.into_iter())
            .zip(bar_widths.iter())
            .enumerate()
            .map(|(idx, ((label_element, popover_element), width))| {
                let pop = Popover::new(
                    button(label_element)
                        .on_press(Message::Menu(id, MenuMessage::OpenPopover(idx)))
                        .width(*width)
                        .style(move |theme: &Theme, status| 
                            get_popover_btn_style(Some(self), None, widgets, theme, status))
                            .boxed(),
                    // menu bar popover
                    self.is_open[idx].then_some(popover_element),
                )
                .position(get_menu_position(self, true))
                .on_close(Message::Menu(id, MenuMessage::OnClose));
                mouse_area(pop)
                    .on_enter(Message::Menu(id, MenuMessage::OnBarEnter(idx)))
                    .boxed()
            }));

        // Places a container around the menu bar for styling
        let cont_style_opt = self
            .lookup_widgets(widgets, self.bar_container_style_id)
            .and_then(Widgets::as_container_style)
            .cloned();

        let cont_rw = container(rw.boxed())
            .style(move |theme| {
                if let Some(st) = &cont_style_opt {
                    st.to_iced(theme, &self.bar_container_style_std)
                } else {
                    match &self.bar_container_style_std {
                        Some(std) => std.to_iced(theme),
                        None => container::bordered_box(theme),
                    }
                }
            })
            .padding(get_padding(&self.padding))
            .boxed();

        Some(cont_rw)
    }
}

/// Recursively build popover elements for all Sub items at every depth.
fn build_dropdown_items<'a>(
    menu: &Menu,
    widgets: &'a HashMap<usize, Widgets>,
    items: Vec<GroupedItem<'a>>,
    containers: &'a HashMap<usize, Containers>,
    menu_bar_item_id: usize,
    bar_idx: usize,
    sub_is_open: &HashMap<String, bool>,
    parent_path: &str,
) -> Vec<Element<'a, app::Message>> {
    items
        .into_iter()
        .map(|item| match item {
            GroupedItem::Plain(el) => el.boxed(),
            GroupedItem::Sub {
                children, sub_item_id, ..
            } => {
                let msi = containers
                    .get(&sub_item_id)
                    .and_then(Containers::as_menu_sub_item)
                    .map(|msi| msi).unwrap();

                let path = if parent_path.is_empty() {
                    msi.label.clone()
                } else {
                    format!("{parent_path}/'SubMenu Not Found'")
                };

                let menu_bar_item = menu
                    .lookup(containers, Some(menu_bar_item_id))
                    .and_then(Containers::as_menu_bar_item);

                let is_open_now = sub_is_open.get(&path).copied().unwrap_or(false);
                // inner sub-items use plain text style and open to the right
                let sub_items = build_dropdown_items(
                    menu,
                    widgets,
                    children,
                    containers,
                    menu_bar_item_id,
                    bar_idx,
                    sub_is_open,
                    &path,
                );
                let sub_dropdown = container(column(sub_items).boxed())
                    .style(move |theme| container::bordered_box(theme))
                    .boxed();

                let position_opt = menu_bar_item.and_then(|mbi| Some(mbi.position)).or_else(|| None);

                let position = position_opt.unwrap_or(Position::Right);

                let label = msi.label.clone();

                // Build the menu bar button dropdowns for all
                Popover::new(
                    button(text(label))
                        .on_press(Message::Menu(
                            menu.id,
                            MenuMessage::OpenPopoverSub(bar_idx, path.clone()),
                        ))
                        .style(move |theme: &Theme, status| 
                            get_popover_btn_style(None, Some(msi), widgets, theme, status)
                        )
                        .boxed(),
                    is_open_now.then_some(sub_dropdown),
                )
                .position(position)
                .on_close(Message::Menu(menu.id, MenuMessage::CloseSubPopover(bar_idx, path)))
                .boxed()
            }
        })
        .collect()
}

fn configure_bar_labels<'a>(
    menu: &Menu,
    labels: &'a Vec<String>,
    style_id: Option<usize>,
    font_id: Option<usize>,
    widgets: &'a HashMap<usize, Widgets>,
) -> Vec<Element<'a, app::Message>> {
    let style_opt = menu.lookup_widgets(widgets, style_id).and_then(Widgets::as_text_style);

    let mut text_labels = vec![];

    for label in labels.iter() {
        let txt = match style_opt {
            Some(style) => style
                .construct(label.clone(), font_id, widgets, Some(MenuStyleOverrides::default()))
                .boxed(),
            None => text(label)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .width(Length::Fill)
                .boxed(),
        };
        text_labels.push(txt)
    }

    text_labels
}

fn get_popover_btn_style<'a>(
    menu_opt: Option<&Menu>,
    msi_opt: Option<&MenuSubItem>,
    widgets: &'a HashMap<usize, Widgets>,
    theme: &Theme,
    status: button::Status,
) -> button::Style {

    let (style_opt, 
        pal_opt, 
        font_opt,
        style_std_opt) = 
        if menu_opt.is_some() {
            let menu = menu_opt.unwrap();
            let style_opt = menu
            .lookup_widgets(widgets, menu.bar_btn_style_id)
            .and_then(Widgets::as_button_style)
            .cloned();

        let pal_opt = menu
            .lookup_widgets(widgets, menu.bar_btn_palette_id)
            .and_then(Widgets::as_palette)
            .cloned();

        let font_opt = menu
            .lookup_widgets(widgets, menu.bar_btn_font_id)
            .and_then(Widgets::as_font)
            .cloned();

            (style_opt, pal_opt, font_opt, menu.bar_btn_style_std.clone())
        
        } else if msi_opt.is_some() {
            let msi = msi_opt.unwrap();
            let style_opt = msi
            .lookup_widgets(widgets, msi.btn_style_id)
            .and_then(Widgets::as_button_style)
            .cloned();

        let pal_opt = msi
            .lookup_widgets(widgets, msi.btn_palette_id)
            .and_then(Widgets::as_palette)
            .cloned();

        let font_opt = msi
            .lookup_widgets(widgets, msi.btn_font_id)
            .and_then(Widgets::as_font)
            .cloned();

            (style_opt, pal_opt, font_opt, msi.btn_style_std.clone())

        } else {
            return button::Style::default();
        };

    

    let style = if style_opt.is_some() || pal_opt.is_some() {
        let btn_st = ButtonStyle::default();
        let st = style_opt.as_ref().unwrap_or(&btn_st);
        st.to_iced(theme, status, &pal_opt, &style_std_opt)
    } else {
        match &style_std_opt {
            Some(std) => std.to_iced(theme, status),
            None => button::text(theme, status),
        }
    };

    style
}

fn get_menu_position(menu: &Menu, is_bar: bool) -> Position {
    match (
        menu.dropdown_open_auto,
        menu.dropdown_open_bottom,
        menu.dropdown_open_left,
        menu.dropdown_open_top,
        menu.dropdown_open_right,
    ) {
        (Some(true), None, None, None, None) => Position::Auto,
        (None, Some(true), None, None, None) => Position::Bottom,
        (None, None, Some(true), None, None) => Position::Left,
        (None, None, None, Some(true), None) => Position::Right,
        (None, None, None, None, Some(true)) => Position::Top,
        _ => {
            if is_bar {
                Position::Bottom
            } else {
                Position::Right
            }
        }
    }
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
pub struct MenuBarItem {
    pub id: usize,
    pub position: Position,
    pub container_style_id: Option<usize>,
    pub container_style_std: Option<ContainerStyleStd>,
    pub spacing: Option<f32>,
    pub gap: Option<f32>,
    pub padding: Option<Vec<f32>>,
}

// A sub-menu item inside another dropdown.
#[derive(Debug, Clone, Default)]
pub struct MenuSubItem {
    pub id: usize,
    pub label: String,
    pub width: Option<f32>,
    pub spacing: Option<f32>,
    pub gap: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub container_style_id: Option<usize>,
    pub container_style_std: Option<ContainerStyleStd>,
    pub btn_style_id: Option<usize>,
    pub btn_style_std: Option<ButtonStyleStd>,
    pub btn_palette_id: Option<usize>,
    pub btn_font_id: Option<usize>,
    pub text_style_id: Option<usize>,
    pub text_font_id: Option<usize>,
}

impl MenuSubItem {
    fn lookup_widgets<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }
}


#[derive(Debug, Clone)]
pub enum MenuMessage {
    OpenPopover(usize),
    OpenPopoverSub(usize, String),
    CloseSubPopover(usize, String),
    OnClose,
    OnBarPress(usize),
    OnBarEnter(usize),
    OnBarExit(usize),
}

pub fn menu_callback(state: &mut IpgState, id: usize, message: MenuMessage) {
    match message {
        MenuMessage::OnBarPress(idx) => {
            invoke_callback_with_args(
                id,
                CallbackName::OnBarPress,
                "MenuBar",
                idx,
                "def cb(wid: int, bar_index: int)",
            );
        }
        MenuMessage::OnBarEnter(idx) => {
            invoke_callback_with_args(
                id,
                CallbackName::OnBarEnter,
                "MenuBar",
                idx,
                "def cb(wid: int, bar_index: int)",
            );
        }
        MenuMessage::OnBarExit(idx) => {
            invoke_callback_with_args(
                id,
                CallbackName::OnBarExit,
                "MenuBar",
                idx,
                "def cb(wid: int, bar_index: int)",
            );
        }
        MenuMessage::OpenPopover(idx) => {
            if let Some(Containers::Menu(cb)) = state.containers.get_mut(&id) {
                cb.is_open = cb.is_open.iter().enumerate().map(|(i, _)| i == idx).collect();
            }
        }
        MenuMessage::CloseSubPopover(bar_idx, path) => {
            if let Some(Containers::Menu(menu)) = state.containers.get_mut(&id) {
                if let Some(sub_map) = menu.dropdown_is_open.get_mut(bar_idx) {
                    // Close this path and any of its children
                    let prefix = format!("{path}/");
                    sub_map
                        .iter_mut()
                        .filter(|(k, _)| *k == &path || k.starts_with(&prefix))
                        .for_each(|(_, v)| *v = false);
                }
            }
        }
        MenuMessage::OpenPopoverSub(bar_idx, path) => {
            if let Some(Containers::Menu(menu)) = state.containers.get_mut(&id) {
                if let Some(sub_map) = menu.dropdown_is_open.get_mut(bar_idx) {
                    let currently_open = sub_map.get(&path).copied().unwrap_or(false);
                    if path.contains('/') {
                        // Nested sub: close siblings at same depth, keep parent open, toggle this
                        let parent = &path[..path.rfind('/').unwrap()];
                        let sibling_prefix = format!("{parent}/");
                        sub_map
                            .iter_mut()
                            .filter(|(k, _)| k.starts_with(&sibling_prefix) && k.as_str() != path.as_str())
                            .for_each(|(_, v)| *v = false);
                        sub_map.insert(path, !currently_open);
                    } else {
                        // Depth-1 sub: close all siblings, toggle this one
                        sub_map.values_mut().for_each(|v| *v = false);
                        sub_map.insert(path, !currently_open);
                    }
                }
            }
        }
        MenuMessage::OnClose => {
            if let Some(Containers::Menu(menu)) = state.containers.get_mut(&id) {
                menu.is_open.iter_mut().for_each(|v| *v = false);
                menu.dropdown_is_open
                    .iter_mut()
                    .for_each(|map| map.values_mut().for_each(|v| *v = false));
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuParam {
    BarWidths,
    CloseOnBarBackgroundClick,
    CloseOnBarItemClick,
    Height,
    ItemsCloseOnBackgroundClickGlobal,
    ItemsCloseOnClickGlobal,
    Padding,
    Show,
    Spacing,
    StyleId,
    StyleStd,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuBarItemParam {
    IsOpen,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuSubItemParam {
    Gap,
    Padding,
    Spacing,
    Width,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Menu {
    type Param = MenuParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            MenuParam::BarWidths => set_t_value(&mut self.bar_widths, value, "MenuParam::BarWidths"),
            MenuParam::Height => set_t_value(&mut self.height, value, "MenuParam::Height"),
            MenuParam::Padding => set_t_value(&mut self.padding, value, "MenuParam::Padding"),
            MenuParam::Show => set_t_value(&mut self.show, value, "MenuParam::Show"),
            MenuParam::Spacing => set_t_value(&mut self.spacing, value, "Spacing"),
            MenuParam::CloseOnBarBackgroundClick => set_t_value(
                &mut self.close_on_bar_background_click,
                value,
                "MenuParam::CloseOnBarBackgroundClick",
            ),
            MenuParam::CloseOnBarItemClick => set_t_value(
                &mut self.close_on_bar_item_click,
                value,
                "MenuParam::CloseOnBarItemClick",
            ),
            MenuParam::ItemsCloseOnBackgroundClickGlobal => set_t_value(
                &mut self.items_close_on_background_click_global,
                value,
                "MenuParam::ItemsCloseOnBackgroundClickGlobal",
            ),
            MenuParam::ItemsCloseOnClickGlobal => set_t_value(
                &mut self.items_close_on_click_global,
                value,
                "MenuParam::ItemsCloseOnClickGlobal",
            ),
            MenuParam::StyleId => set_t_value(&mut self.bar_btn_style_id, value, "MenuParam::StyleId"),
            MenuParam::StyleStd => set_t_value(&mut self.bar_btn_style_std, value, "MenuParam::StyleStd"),
        }
    }
}

impl WidgetParamUpdate for MenuBarItem {
    type Param = MenuBarItemParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            MenuBarItemParam::IsOpen => todo!(),
        }
    }
}

impl WidgetParamUpdate for MenuSubItem {
    type Param = MenuSubItemParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            MenuSubItemParam::Gap => set_t_value(&mut self.gap, value, "MenuSubItemParam::Gap"),
            MenuSubItemParam::Padding => set_t_value(&mut self.padding, value, "MenuSubItemParam::Padding"),
            MenuSubItemParam::Spacing => set_t_value(&mut self.spacing, value, "MenuSubItemParam::Spacing"),
            MenuSubItemParam::Width => set_t_value(&mut self.width, value, "MenuSubItemParam::Width"),
        }
    }
}
