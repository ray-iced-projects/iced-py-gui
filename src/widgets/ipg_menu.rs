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

// fn get_menu_position(menu: &Menu, is_bar: bool) -> Position {
//     match (
//         menu.dropdown_open_auto,
//         menu.dropdown_open_bottom,
//         menu.dropdown_open_left,
//         menu.dropdown_open_top,
//         menu.dropdown_open_right,
//     ) {
//         (Some(true), None, None, None, None) => Position::Auto,
//         (None, Some(true), None, None, None) => Position::Bottom,
//         (None, None, Some(true), None, None) => Position::Left,
//         (None, None, None, Some(true), None) => Position::Right,
//         (None, None, None, None, Some(true)) => Position::Top,
//         _ => {
//             if is_bar {
//                 Position::Bottom
//             } else {
//                 Position::Right
//             }
//         }
//     }
// }

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
pub struct SubMenu {
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
pub struct SubMenuItem {
    pub id: usize,
    pub label: String,
    pub size: Option<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct MenuSeparator {
    pub id: usize,
}

#[derive(Debug, Clone)]
pub enum MenuMessage {}

pub fn menu_callback(state: &mut IpgState, id: usize, message: MenuMessage) {
    match message {}
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuParam {
    None
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuBarParam {
    None
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SubMenuParam {
    None
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
