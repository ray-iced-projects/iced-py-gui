//! ipg_menu

#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::widget::{button, column, container, mouse_area, row, text};
use iced::{Element, Renderer, Theme, Widget};

use crate::IpgState;
use crate::app::Message;
use crate::state::Widgets;
use crate::widgets::callbacks::{CallbackName, invoke_callback_with_args};

use crate::iced_widgets::popover::{Popover, Position};
use crate::widgets::ipg_button::{ButtonStyle, ButtonStyleStd};
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
    pub bar_items: Vec<String>,
    pub bar_widths: Option<Vec<f32>>,
    pub padding: Option<Vec<f32>>,
    pub spacing: Option<f32>,
    pub height: Option<f32>,
    pub close_on_bar_item_click: Option<bool>,
    pub close_on_bar_background_click: Option<bool>,
    pub items_close_on_click_global: Option<bool>,
    pub items_close_on_background_click_global: Option<bool>,
    pub style_id: Option<usize>,
    pub style_std: Option<ButtonStyleStd>,
    pub palette_id: Option<usize>,
    pub font_id: Option<usize>,
    pub show: bool,
    pub is_open: Vec<bool>,
    pub sub_is_open: Vec<HashMap<String, bool>>,
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
        widgets: &HashMap<usize, Widgets>,
        containers: &HashMap<usize, Containers>,
    ) -> Option<Element<'a, app::Message, Theme, Renderer>> {

        let style_opt = self
            .lookup_widgets(widgets, self.style_id)
            .and_then(Widgets::as_button_style)
            .cloned();

        let c_pal_opt = self
            .lookup_widgets(widgets, self.palette_id)
            .and_then(Widgets::as_palette)
            .cloned();

        let font_opt = self.lookup_widgets(
            widgets, self.font_id).and_then(Widgets::as_font).cloned();

        let bar_widths = match self.bar_widths.clone() {
            Some(widths) => {
                if widths.len() == 1 {
                    vec![widths[0]; self.bar_items.len()]
                } else {
                    widths
                }
            }
            None => vec![],
        };

        let mut bar_columns = vec![];

        let mut index = 0;
        for (menu_bar_item_id, group) in grouped_content.into_iter() {
            
            let menu_bar_item = self
                .lookup(containers, Some(menu_bar_item_id))
                .and_then(Containers::as_menu_bar_item);
            
            let style_opt = menu_bar_item
                .and_then(|mbi| mbi.style_id)
                .and_then(|id| self.lookup_widgets(widgets, Some(id)))
                .and_then(Widgets::as_button_style)
                .cloned()
                .or_else(|| {
                    self.lookup_widgets(widgets, self.style_id)
                        .and_then(Widgets::as_button_style)
                        .cloned()
                });

            let c_pal_opt = menu_bar_item
                .and_then(|mbi| mbi.palette_id)
                .and_then(|id| self.lookup_widgets(widgets, Some(id)))
                .and_then(Widgets::as_palette)
                .cloned()
                .or_else(|| {
                    self.lookup_widgets(widgets, self.palette_id)
                        .and_then(Widgets::as_palette)
                        .cloned()
                });

            let font_opt = menu_bar_item
                .and_then(|mbi| mbi.font_id)
                .and_then(|id| self.lookup_widgets(widgets, Some(id)))
                .and_then(Widgets::as_font)
                .cloned()
                .or_else(|| {
                    self.lookup_widgets(widgets, self.font_id)
                        .and_then(Widgets::as_font)
                        .cloned()
                });

            let mut inner_col = vec![];
            for grp in group.into_iter() {
                match grp {
                    GroupedItem::Plain(element) => 
                    inner_col.push(element.boxed()),
                    GroupedItem::Sub {
                        trigger: _,
                        children,
                        sub_item_id,
                    } => {
                        let label = self
                            .lookup(containers, Some(sub_item_id))
                            .and_then(Containers::as_menu_sub_item)
                            .map(|sm| sm.label.clone())
                            .unwrap_or_else(|| "No Label Found".to_string());

                        let path = label.clone();
                        let sub_is_open = self.sub_is_open.get(index).cloned().unwrap_or_default();
                        let is_open_now = sub_is_open.get(&path).copied().unwrap_or(false);
                        let sub_items = build_sub_items(children, containers, self.id, index, &sub_is_open, &path);
                        let sub_col = column(sub_items).boxed();

                        let style_opt_clone = style_opt.clone();
                        let c_pal_opt_clone = c_pal_opt.clone();

                        inner_col.push(
                            Popover::new(
                                button(text(label))
                                    .on_press(Message::Menu(self.id, MenuMessage::OpenPopoverSub(index, path)))
                                    .style(move |theme: &Theme, status| {
                                        if style_opt_clone.is_some() || c_pal_opt_clone.is_some() {
                                            let btn_st = ButtonStyle::default();
                                            let st = style_opt_clone.as_ref().unwrap_or(&btn_st);
                                            st.to_iced(theme, status, &c_pal_opt_clone, &self.style_std)
                                        } else {
                                            match &self.style_std {
                                                Some(std) => std.to_iced(theme, status),
                                                None => button::background(theme, status),
                                            }
                                        }
                                    })
                                    .boxed(),
                                is_open_now.then_some(sub_col),
                            )
                            .position(Position::Right)
                            .on_close(Message::Menu(self.id, MenuMessage::OnClose))
                            .boxed(),
                        );
                    }
                }
            }
            bar_columns.push(
                container(column(inner_col).boxed())
                    // .width(bar_widths[index])
                    .style(move |theme| container::bordered_box(theme))
                    .boxed(),
            );
            index += 1;
        }

        let id = self.id;
        let rw = row(self
            .bar_items
            .iter()
            .zip(bar_columns.into_iter())
            .zip(bar_widths.iter())
            .enumerate()
            .map(|(idx, ((label, col), width))| {
                let style_opt_clone = style_opt.clone();
                let c_pal_opt_clone = c_pal_opt.clone();
                let pop = Popover::new(
                    button(text(label.as_str()))
                        .on_press(Message::Menu(id, MenuMessage::OpenPopover(idx)))
                        .width(*width)
                        .style(move |theme: &Theme, status| {
                            if style_opt_clone.is_some() || c_pal_opt_clone.is_some() {
                                let btn_st = ButtonStyle::default();
                                let st = style_opt_clone.as_ref().unwrap_or(&btn_st);
                                st.to_iced(theme, status, &c_pal_opt_clone, &self.style_std)
                            } else {
                                match &self.style_std {
                                    Some(std) => std.to_iced(theme, status),
                                    None => button::background(theme, status),
                                }
                            }
                        })
                        .boxed(),
                    self.is_open[idx].then_some(col),
                )
                .position(Position::Bottom)
                .on_close(Message::Menu(id, MenuMessage::OnClose));
                mouse_area(pop)
                    .on_enter(Message::Menu(id, MenuMessage::OnBarEnter(idx)))
                    .boxed()
            }));

        let rw = rw.boxed();
        Some(rw)
    }
}

/// Recursively build popover elements for nested Sub items.
fn build_sub_items<'a>(
    items: Vec<GroupedItem<'a>>,
    containers: &HashMap<usize, Containers>,
    menu_id: usize,
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
                let label = containers
                    .get(&sub_item_id)
                    .and_then(Containers::as_menu_sub_item)
                    .map(|sm| sm.label.clone())
                    .unwrap_or_else(|| "Sub".to_string());

                let path = if parent_path.is_empty() {
                    label.clone()
                } else {
                    format!("{parent_path}/{label}")
                };

                let is_open_now = sub_is_open.get(&path).copied().unwrap_or(false);
                let sub_items = build_sub_items(children, containers, menu_id, bar_idx, sub_is_open, &path);
                let sub_col = column(sub_items).boxed();

                Popover::new(
                    button(text(label))
                        .on_press(Message::Menu(menu_id, MenuMessage::OpenPopoverSub(bar_idx, path)))
                        .boxed(),
                    is_open_now.then_some(sub_col),
                )
                .position(Position::Right)
                .on_close(Message::Menu(menu_id, MenuMessage::OnClose))
                .boxed()
            }
        })
        .collect()
}

#[derive(Debug, Clone, Default)]
pub struct MenuBarItem {
    pub id: usize,
    pub style_id: Option<usize>,
    pub style_std: Option<ButtonStyleStd>,
    pub palette_id: Option<usize>,
    pub font_id: Option<usize>,
}


#[derive(Debug, Clone)]
pub enum MenuMessage {
    OpenPopover(usize),
    OpenPopoverSub(usize, String),
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
        MenuMessage::OpenPopoverSub(bar_idx, path) => {
            if let Some(Containers::Menu(menu)) = state.containers.get_mut(&id) {
                if let Some(sub_map) = menu.sub_is_open.get_mut(bar_idx) {
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
                menu.sub_is_open
                    .iter_mut()
                    .for_each(|map| map.values_mut().for_each(|v| *v = false));
            }
        }
    }
}

/// A sub-menu item inside a dropdown.  The first child added to a
/// `MenuSubItem` context-manager is the trigger widget (shown in the
/// dropdown list); the remaining children become the items of the
/// child menu that opens on hover.
#[derive(Debug, Clone)]
pub struct MenuSubItem {
    pub id: usize,
    pub label: String,
    pub width: Option<f32>,
    pub spacing: Option<f32>,
    pub offset: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub close_on_item_click: Option<bool>,
    pub close_on_background_click: Option<bool>,
    pub show: bool,
}

#[derive(Debug, Clone)]
pub struct MenuStyle {
    pub id: usize,
    pub bar_background_color: Option<Color>,
    pub bar_background_color_alpha: Option<f32>,
    pub bar_background_rgba: Option<[f32; 4]>,

    pub bar_border_color: Option<Color>,
    pub bar_border_color_alpha: Option<f32>,
    pub bar_border_rgba: Option<[f32; 4]>,
    pub bar_border_radius: Option<Vec<f32>>,
    pub bar_border_width: Option<f32>,

    pub bar_shadow_color: Option<Color>,
    pub bar_shadow_color_alpha: Option<f32>,
    pub bar_shadow_rgba: Option<[f32; 4]>,
    pub bar_shadow_offset_xy: Option<[f32; 2]>,
    pub bar_shadow_blur_radius: Option<f32>,

    pub menu_background_color: Option<Color>,
    pub menu_background_color_alpha: Option<f32>,
    pub menu_background_rgba: Option<[f32; 4]>,

    pub menu_border_color: Option<Color>,
    pub menu_border_color_alpha: Option<f32>,
    pub menu_border_rgba: Option<[f32; 4]>,
    pub menu_border_radius: Option<Vec<f32>>,
    pub menu_border_width: Option<f32>,

    pub menu_shadow_color: Option<Color>,
    pub menu_shadow_color_alpha: Option<f32>,
    pub menu_shadow_rgba: Option<[f32; 4]>,
    pub menu_shadow_offset_xy: Option<[f32; 2]>,
    pub menu_shadow_blur_radius: Option<f32>,

    pub path_background_color: Option<Color>,
    pub path_background_color_alpha: Option<f32>,
    pub path_background_rgba: Option<[f32; 4]>,

    pub path_border_color: Option<Color>,
    pub path_border_color_alpha: Option<f32>,
    pub path_border_rgba: Option<[f32; 4]>,
    pub path_border_radius: Option<Vec<f32>>,
    pub path_border_width: Option<f32>,
}

// impl MenuStyle {
//     fn to_iced(
//         &self,
//         theme: &Theme,
//         status: menu::style::status::Status,
//         style_std: Option<bool>,
//         ) -> menu::style::menu_bar::Style {

//         //The base style will be either default or primary
//         let mut style =
//             if style_std == Some(true) {
//                 menu::style::menu_bar::primary(theme, status)
//             } else { menu::style::menu_bar::Style::default() };

//         let bar_background_color =
//         Color::rgba_ipg_color_to_iced(
//             self.bar_background_rgba,
//             &self.bar_background_color,
//             self.bar_background_color_alpha);

//         let bar_border_color =
//             Color::rgba_ipg_color_to_iced(
//                 self.bar_border_rgba,
//                 &self.bar_border_color,
//                 self.bar_border_color_alpha);

//         let bar_shadow_color =
//             Color::rgba_ipg_color_to_iced(
//                 self.bar_shadow_rgba,
//                 &self.bar_shadow_color,
//                 self.bar_shadow_color_alpha);

//         let menu_background_color =
//             Color::rgba_ipg_color_to_iced(
//                 self.menu_background_rgba,
//                 &self.menu_background_color,
//                 self.menu_background_color_alpha);

//         let menu_border_color =
//             Color::rgba_ipg_color_to_iced(
//                 self.menu_border_rgba,
//                 &self.menu_border_color,
//                 self.menu_border_color_alpha);

//         let menu_shadow_color =
//             Color::rgba_ipg_color_to_iced(
//                 self.menu_shadow_rgba,
//                 &self.menu_shadow_color,
//                 self.menu_shadow_color_alpha);

//         let path_background_color =
//             Color::rgba_ipg_color_to_iced(
//                 self.path_background_rgba,
//                 &self.path_background_color,
//                 self.path_background_color_alpha);

//         let path_border_color =
//             Color::rgba_ipg_color_to_iced(
//                 self.path_border_rgba,
//                 &self.path_border_color,
//                 self.path_border_color_alpha);

//         // making defaults square
//         style.bar_border.radius = 0.0.into();
//         style.menu_border.radius = 0.0.into();

//         // bar
//         if let Some(color) = bar_background_color {
//             style.bar_background = color.into()
//         }

//         apply_border_overrides(
//             &mut style.bar_border, bar_border_color,
//             &self.bar_border_radius, self.bar_border_width, "Menu-bar",
//         );

//         apply_shadow_overrides_xy(
//             &mut style.bar_shadow, bar_shadow_color,
//             self.bar_shadow_offset_xy, self.bar_shadow_blur_radius);

//         // menu
//         if let Some(color) = menu_background_color {
//             style.menu_background = color.into()
//         }

//         apply_border_overrides(
//             &mut style.menu_border, menu_border_color,
//             &self.menu_border_radius, self.menu_border_width, "Menu-menu",
//         );

//         apply_shadow_overrides_xy(
//             &mut style.menu_shadow, menu_shadow_color,
//             self.menu_shadow_offset_xy, self.menu_shadow_blur_radius);

//         // path
//         if let Some(color) = path_background_color {
//             style.path = color.into()
//         }

//         apply_border_overrides(
//             &mut style.path_border, path_border_color,
//             &self.path_border_radius, self.path_border_width, "Menu-path",
//         );

//         style

//     }
// }

// pub fn primary(theme: &Theme) -> menu::style::menu_bar::Style {
//     let palette = theme.palette();
//     let pair = palette.background.strong;

//     menu::style::menu_bar::Style {
//         bar_background: pair.color.into(),
//         menu_background: pair.color.into(),
//         bar_border: iced::border::rounded(2),
//         ..menu::style::menu_bar::Style::default()
//     }
// }

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
    CloseOnBackgroundClick,
    CloseOnItemClick,
    Offset,
    Padding,
    Show,
    Spacing,
    Width,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuStyleParam {
    BarBackgroundColor,
    BarBackgroundRgba,
    BarBackgroundAlpha,
    BarBorderColor,
    BarBorderRgba,
    BarBorderAlpha,
    BarBorderRadius,
    BarBorderWidth,
    BarShadowColor,
    BarShadowRgba,
    BarShadowAlpha,
    BarShadowOffsetXY,
    BarShadowBlurRadius,

    MenuBackgroundColor,
    MenuBackgroundRgba,
    MenuBackgroundAlpha,
    MenuBorderColor,
    MenuBorderRgba,
    MenuBorderAlpha,
    MenuBorderRadius,
    MenuBorderWidth,
    MenuShadowColor,
    MenuShadowRgba,
    MenuShadowAlpha,
    MenuShadowOffsetXy,
    MenuShadowBlurRadius,

    PathBackgroundColor,
    PathBackgroundRgba,
    PathBackgroundAlpha,
    PathBorderColor,
    PathBorderRgba,
    PathBorderAlpha,
    PathBorderRadius,
    PathBorderWidth,
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
            MenuParam::StyleId => set_t_value(&mut self.style_id, value, "MenuParam::StyleId"),
            MenuParam::StyleStd => set_t_value(&mut self.style_std, value, "MenuParam::StyleStd"),
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
            MenuSubItemParam::CloseOnBackgroundClick => set_t_value(
                &mut self.close_on_background_click,
                value,
                "MenuSubItemParam::CloseOnBackgroundClick",
            ),
            MenuSubItemParam::CloseOnItemClick => set_t_value(
                &mut self.close_on_item_click,
                value,
                "MenuSubItemParam::CloseOnItemClick",
            ),
            MenuSubItemParam::Offset => set_t_value(&mut self.offset, value, "MenuSubItemParam::Offset"),
            MenuSubItemParam::Padding => set_t_value(&mut self.padding, value, "MenuSubItemParam::Padding"),
            MenuSubItemParam::Show => set_t_value(&mut self.show, value, "MenuSubItemParam::Show"),
            MenuSubItemParam::Spacing => set_t_value(&mut self.spacing, value, "MenuSubItemParam::Spacing"),
            MenuSubItemParam::Width => set_t_value(&mut self.width, value, "MenuSubItemParam::Width"),
        }
    }
}

impl WidgetParamUpdate for MenuStyle {
    type Param = MenuStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            // bar
            MenuStyleParam::BarBackgroundAlpha => {
                set_t_value(&mut self.bar_background_color_alpha, value, "BarBackgroundAlpha")
            }
            MenuStyleParam::BarBackgroundColor => {
                set_t_value(&mut self.bar_background_color, value, "BarBackgroundColor")
            }
            MenuStyleParam::BarBackgroundRgba => {
                set_t_value(&mut self.bar_background_color, value, "BarBackgroundRgba")
            }
            MenuStyleParam::BarBorderAlpha => set_t_value(&mut self.bar_border_color_alpha, value, "BarBorderAlpha"),
            MenuStyleParam::BarBorderColor => set_t_value(&mut self.bar_border_color, value, "BarBorderColor"),
            MenuStyleParam::BarBorderRadius => set_t_value(&mut self.bar_border_radius, value, "BarBorderRadius"),
            MenuStyleParam::BarBorderRgba => set_t_value(&mut self.bar_border_color, value, "BarBorderRgba"),
            MenuStyleParam::BarBorderWidth => set_t_value(&mut self.bar_border_width, value, "BarBorderWidth"),
            MenuStyleParam::BarShadowAlpha => set_t_value(&mut self.bar_shadow_color_alpha, value, "BarShadowAlpha"),
            MenuStyleParam::BarShadowBlurRadius => {
                set_t_value(&mut self.bar_shadow_blur_radius, value, "BarShadowBlurRadius")
            }
            MenuStyleParam::BarShadowColor => set_t_value(&mut self.bar_shadow_color, value, "BarShadowColor"),
            MenuStyleParam::BarShadowOffsetXY => {
                set_t_value(&mut self.bar_shadow_offset_xy, value, "BarShadowOffsetXY")
            }
            MenuStyleParam::BarShadowRgba => set_t_value(&mut self.bar_shadow_color, value, "BarShadowRgba"),
            // menu
            MenuStyleParam::MenuBackgroundAlpha => {
                set_t_value(&mut self.menu_background_color_alpha, value, "MenuBackgroundAlpha")
            }
            MenuStyleParam::MenuBackgroundColor => {
                set_t_value(&mut self.menu_background_color, value, "MenuBackgroundColor")
            }
            MenuStyleParam::MenuBackgroundRgba => {
                set_t_value(&mut self.menu_background_color, value, "MenuBackgroundRgba")
            }
            MenuStyleParam::MenuBorderAlpha => set_t_value(&mut self.menu_border_color_alpha, value, "MenuBorderAlpha"),
            MenuStyleParam::MenuBorderColor => set_t_value(&mut self.menu_border_color, value, "MenuBorderColor"),
            MenuStyleParam::MenuBorderRadius => set_t_value(&mut self.menu_border_radius, value, "MenuBorderRadius"),
            MenuStyleParam::MenuBorderRgba => set_t_value(&mut self.menu_border_color, value, "MenuBorderRgba"),
            MenuStyleParam::MenuBorderWidth => set_t_value(&mut self.menu_border_width, value, "MenuBorderWidth"),
            MenuStyleParam::MenuShadowAlpha => set_t_value(&mut self.menu_shadow_color_alpha, value, "MenuShadowAlpha"),
            MenuStyleParam::MenuShadowBlurRadius => {
                set_t_value(&mut self.menu_shadow_blur_radius, value, "MenuShadowBlurRadius")
            }
            MenuStyleParam::MenuShadowColor => set_t_value(&mut self.menu_shadow_color, value, "MenuShadowColor"),
            MenuStyleParam::MenuShadowOffsetXy => {
                set_t_value(&mut self.menu_shadow_offset_xy, value, "MenuShadowOffsetXy")
            }
            MenuStyleParam::MenuShadowRgba => set_t_value(&mut self.menu_shadow_color, value, "MenuShadowRgba"),
            // path
            MenuStyleParam::PathBackgroundAlpha => {
                set_t_value(&mut self.path_background_color_alpha, value, "PathBackgroundAlpha")
            }
            MenuStyleParam::PathBackgroundColor => {
                set_t_value(&mut self.path_background_color, value, "PathBackgroundColor")
            }
            MenuStyleParam::PathBackgroundRgba => {
                set_t_value(&mut self.path_background_color, value, "PathBackgroundRgba")
            }
            MenuStyleParam::PathBorderAlpha => set_t_value(&mut self.path_border_color_alpha, value, "PathBorderAlpha"),
            MenuStyleParam::PathBorderColor => set_t_value(&mut self.path_border_color, value, "PathBorderColor"),
            MenuStyleParam::PathBorderRadius => set_t_value(&mut self.path_border_radius, value, "PathBorderRadius"),
            MenuStyleParam::PathBorderRgba => set_t_value(&mut self.path_border_color, value, "PathBorderRgba"),
            MenuStyleParam::PathBorderWidth => set_t_value(&mut self.path_border_width, value, "PathBorderWidth"),
        }
    }
}
