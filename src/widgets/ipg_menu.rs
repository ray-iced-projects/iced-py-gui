//! ipg_menu

#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::{Element, Length, Renderer, Theme, Widget};

use crate::IpgState;
use crate::app::Message;
use crate::iced_widgets::popover::Position;
use crate::py_api::helpers::{get_len, get_padding};
use crate::state::Widgets;
use crate::widgets::callbacks::{CallbackName, invoke_callback_with_args};

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

#[derive(Debug, Clone)]
pub struct Menu {
    pub id: usize,
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
        None
    }
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
pub struct MenuBar {
    pub id: usize,
    pub label: String,
    pub width: f32,
    pub position: Position,
    pub spacing: Option<f32>,
    pub gap: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub container_style_id: Option<usize>,
    pub container_style_std: Option<ContainerStyleStd>,
}

#[derive(Debug, Clone, Default)]
pub struct MenuSub {
    pub id: usize,
    pub label: String,
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
    pub label: String,
    pub size: Option<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct MenuSeparator {
    pub id,
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

impl WidgetParamUpdate for MenuItem {
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
