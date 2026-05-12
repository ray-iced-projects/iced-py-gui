//! ipg_menu

#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::{Element, Renderer, Theme, widget::{Space, button, column, container, text}};

use crate::{IpgState, app::{self, Message}, state::{Containers, Widgets}, widgets::{callbacks::invoke_callback, widget_param_update::{WidgetParamUpdate, set_t_value}}};

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::ipg_widgets::ipg_menu::{
    Menu as MN,
    Position,
};

// #[derive(Debug, Clone)]
// pub struct Menu {
//     pub id: usize,
//     pub padding: Option<Vec<f32>>,
//     pub spacing: Option<f32>,
//     pub width: Option<f32>,
//     pub width_fill: Option<bool>,
//     pub height: Option<f32>,
//     pub close_on_bar_item_click: Option<bool>,
//     pub close_on_bar_background_click: Option<bool>,
//     pub items_close_on_click_global: Option<bool>,
//     pub items_close_on_background_click_global: Option<bool>,
//     pub style_id: Option<usize>,
//     pub style_primary: Option<bool>,
//     pub cursor_bounds_margin: Option<f32>,
//     pub scroll_speed_line: Option<f32>,
//     pub scroll_speed_pixel: Option<f32>,
//     pub show: bool,
// }

#[derive(Debug, Clone)]
pub struct MenuItem {
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

impl MenuItem {
    pub fn construct<'a>(
        &self,
        content: Vec<Element<'a, Message>>,
        _widgets: &HashMap<usize, Widgets>,
    )-> Option<Element<'a, app::Message, Theme, Renderer>> 
    {
        let txt = text(self.label.clone());
        let btn = 
            button(txt)
                .on_press(Message::MenuMessage(self.id, MenuMessage::OnToggle));
        
        let cont_show: Element<Message> = if self.show {
            container(column(content).spacing(3.0))
                            .style(|theme| container::primary(theme)).into()
        } else {
            Space::new().into()
        };

        let mn = 
            MN::new(
                btn, 
                cont_show,
                true
            )
            .position(Position::Bottom)
            .gap(5.0);
        
        Some(mn.into())
    }
}

#[derive(Debug, Clone)]
pub enum MenuMessage {
    OnToggle,
}


pub fn menu_callback(state: &mut IpgState, id: usize, message: MenuMessage) {

    match message {
        MenuMessage::OnToggle => {
            if let Some(Containers::MenuItem(mn)) = state.containers.get_mut(&id) {
                mn.show = !mn.show;
            }
            invoke_callback(id, "on_toggle", "Menu");
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct MenuStyle {
//     pub id: usize,
//     pub bar_background_color: Option<Color>,
//     pub bar_background_color_alpha: Option<f32>,
//     pub bar_background_rgba: Option<[f32; 4]>,
    
//     pub bar_border_color: Option<Color>,
//     pub bar_border_color_alpha: Option<f32>,
//     pub bar_border_rgba: Option<[f32; 4]>,
//     pub bar_border_radius: Option<Vec<f32>>,
//     pub bar_border_width: Option<f32>,
    
//     pub bar_shadow_color: Option<Color>,
//     pub bar_shadow_color_alpha: Option<f32>,
//     pub bar_shadow_rgba: Option<[f32; 4]>,
//     pub bar_shadow_offset_xy: Option<[f32; 2]>,
//     pub bar_shadow_blur_radius: Option<f32>,

//     pub menu_background_color: Option<Color>,
//     pub menu_background_color_alpha: Option<f32>,
//     pub menu_background_rgba: Option<[f32; 4]>,
    
    
//     pub menu_border_color: Option<Color>,
//     pub menu_border_color_alpha: Option<f32>,
//     pub menu_border_rgba: Option<[f32; 4]>,
//     pub menu_border_radius: Option<Vec<f32>>,
//     pub menu_border_width: Option<f32>,
    
//     pub menu_shadow_color: Option<Color>,
//     pub menu_shadow_color_alpha: Option<f32>,
//     pub menu_shadow_rgba: Option<[f32; 4]>,
//     pub menu_shadow_offset_xy: Option<[f32; 2]>,
//     pub menu_shadow_blur_radius: Option<f32>,

//     pub path_background_color: Option<Color>,
//     pub path_background_color_alpha: Option<f32>,
//     pub path_background_rgba: Option<[f32; 4]>,
    
//     pub path_border_color: Option<Color>,
//     pub path_border_color_alpha: Option<f32>,
//     pub path_border_rgba: Option<[f32; 4]>,
//     pub path_border_radius: Option<Vec<f32>>,
//     pub path_border_width: Option<f32>,
// }

// impl MenuStyle {
//     fn to_iced(
//         &self,
//         theme: &Theme,
//         status: style::Status,
//         style_std: Option<bool>,
//         ) -> menu::Style {

//         //The base style will be either default or primary
//         let mut style = 
//             if style_std == Some(true) {
//                 menu_bar::primary(theme, status)
//             } else { menu::Style::default() };

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

// pub fn primary(theme: &Theme) -> menu::Style {
//     let palette = theme.palette();
//     let pair = palette.background.strong;
    
//     menu::Style {
//         bar_background: pair.color.into(),
//         menu_background: pair.color.into(),
//         bar_border: iced::border::rounded(2),
//         ..menu::Style::default()
//     }
// }

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuParam {
    CloseOnBarBackgroundClick,
    CloseOnBarItemClick,
    CursorBoundsMargin,
    Height,
    ItemsCloseOnBackgroundClickGlobal,
    ItemsCloseOnClickGlobal,
    Padding,
    ScrollSpeedLine,
    ScrollSpeedPixel,
    Show,
    Spacing,
    StyleId,
    StylePrimary,
    WidthFill,
    Width,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuBarItemParam {
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

// impl WidgetParamUpdate for Menu {
//     type Param = MenuParam;

//     fn param_update(&mut self, param: Self::Param, value: &PyObject) {
//         match param {
//             MenuParam::CursorBoundsMargin => set_t_value(&mut self.cursor_bounds_margin, value, "MenuParam::CheckBoundsMargin"),
//             MenuParam::Height => set_t_value(&mut self.height, value, "MenuParam::Height"),
//             MenuParam::Padding => set_t_value(&mut self.padding, value, "MenuParam::Padding"),
//             MenuParam::ScrollSpeedLine => set_t_value(&mut self.scroll_speed_line, value, "MenuParam::ScrollSpeedLine"),
//             MenuParam::ScrollSpeedPixel => set_t_value(&mut self.scroll_speed_pixel, value, "MenuParam::ScrollSpeedPixel"),
//             MenuParam::Show => set_t_value(&mut self.show, value, "MenuParam::Show"),
//             MenuParam::Spacing => set_t_value(&mut self.spacing, value, "Spacing"),
//             MenuParam::Width => set_t_value(&mut self.width, value, "MenuParam::Width"),
//             MenuParam::CloseOnBarBackgroundClick => set_t_value(&mut self.close_on_bar_background_click, value, "MenuParam::CloseOnBarBackgroundClick"),
//             MenuParam::CloseOnBarItemClick => set_t_value(&mut self.close_on_bar_item_click, value, "MenuParam::CloseOnBarItemClick"),
//             MenuParam::ItemsCloseOnBackgroundClickGlobal => set_t_value(&mut self.items_close_on_background_click_global, value, "MenuParam::ItemsCloseOnBackgroundClickGlobal"),
//             MenuParam::ItemsCloseOnClickGlobal => set_t_value(&mut self.items_close_on_click_global, value, "MenuParam::ItemsCloseOnClickGlobal"),
//             MenuParam::StyleId => set_t_value(&mut self.style_id, value, "MenuParam::StyleId"),
//             MenuParam::StylePrimary => set_t_value(&mut self.style_primary, value, "MenuParam::StylePrimary"),
//             MenuParam::WidthFill => set_t_value(&mut self.width_fill, value, "MenuParam::WidthFill"),
//         }
//     }
// }

impl WidgetParamUpdate for MenuItem {
    type Param = MenuBarItemParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            MenuBarItemParam::CloseOnBackgroundClick => set_t_value(&mut self.close_on_background_click, value, "MenuBarItemParam::CloseOnBackgroundClick"),
            MenuBarItemParam::CloseOnItemClick => set_t_value(&mut self.close_on_item_click, value, "MenuBarItemParam::CloseOnItemClick"),
            MenuBarItemParam::Offset => set_t_value(&mut self.offset, value, "MenuBarItemParam::Offset"),
            MenuBarItemParam::Padding => set_t_value(&mut self.padding, value, "MenuBarItemParam::Paddings"),
            MenuBarItemParam::Show => set_t_value(&mut self.show, value, "MenuBarItemParam::Show"),
            MenuBarItemParam::Spacing => set_t_value(&mut self.spacing, value, "MenuBarItemParam::Spacing"),
            MenuBarItemParam::Width => set_t_value(&mut self.width, value, "MenuBarItemParam::Width"),
        }
    }
}

// impl WidgetParamUpdate for MenuStyle {
//     type Param = MenuStyleParam;

//     fn param_update(&mut self, param: Self::Param, value: &PyObject) {
//         match param {
//             // bar
//             MenuStyleParam::BarBackgroundAlpha => set_t_value(&mut self.bar_background_color_alpha, value, "BarBackgroundAlpha"),
//             MenuStyleParam::BarBackgroundColor => set_t_value(&mut self.bar_background_color, value, "BarBackgroundColor"),
//             MenuStyleParam::BarBackgroundRgba => set_t_value(&mut self.bar_background_color, value, "BarBackgroundRgba"),
//             MenuStyleParam::BarBorderAlpha => set_t_value(&mut self.bar_border_color_alpha, value, "BarBorderAlpha"),
//             MenuStyleParam::BarBorderColor => set_t_value(&mut self.bar_border_color, value, "BarBorderColor"),
//             MenuStyleParam::BarBorderRadius => set_t_value(&mut self.bar_border_radius, value, "BarBorderRadius"),
//             MenuStyleParam::BarBorderRgba => set_t_value(&mut self.bar_border_color, value, "BarBorderRgba"),
//             MenuStyleParam::BarBorderWidth => set_t_value(&mut self.bar_border_width, value, "BarBorderWidth"),
//             MenuStyleParam::BarShadowAlpha => set_t_value(&mut self.bar_shadow_color_alpha, value, "BarShadowAlpha"),
//             MenuStyleParam::BarShadowBlurRadius => set_t_value(&mut self.bar_shadow_blur_radius, value, "BarShadowBlurRadius"),
//             MenuStyleParam::BarShadowColor => set_t_value(&mut self.bar_shadow_color, value, "BarShadowColor"),
//             MenuStyleParam::BarShadowOffsetXY => set_t_value(&mut self.bar_shadow_offset_xy, value, "BarShadowOffsetXY"),
//             MenuStyleParam::BarShadowRgba => set_t_value(&mut self.bar_shadow_color, value, "BarShadowRgba"),
//             // menu
//             MenuStyleParam::MenuBackgroundAlpha => set_t_value(&mut self.menu_background_color_alpha, value, "MenuBackgroundAlpha"),
//             MenuStyleParam::MenuBackgroundColor => set_t_value(&mut self.menu_background_color, value, "MenuBackgroundColor"),
//             MenuStyleParam::MenuBackgroundRgba => set_t_value(&mut self.menu_background_color, value, "MenuBackgroundRgba"),
//             MenuStyleParam::MenuBorderAlpha => set_t_value(&mut self.menu_border_color_alpha, value, "MenuBorderAlpha"),
//             MenuStyleParam::MenuBorderColor => set_t_value(&mut self.menu_border_color, value, "MenuBorderColor"),
//             MenuStyleParam::MenuBorderRadius => set_t_value(&mut self.menu_border_radius, value, "MenuBorderRadius"),
//             MenuStyleParam::MenuBorderRgba => set_t_value(&mut self.menu_border_color, value, "MenuBorderRgba"),
//             MenuStyleParam::MenuBorderWidth => set_t_value(&mut self.menu_border_width, value, "MenuBorderWidth"),
//             MenuStyleParam::MenuShadowAlpha => set_t_value(&mut self.menu_shadow_color_alpha, value, "MenuShadowAlpha"),
//             MenuStyleParam::MenuShadowBlurRadius => set_t_value(&mut self.menu_shadow_blur_radius, value, "MenuShadowBlurRadius"),
//             MenuStyleParam::MenuShadowColor => set_t_value(&mut self.menu_shadow_color, value, "MenuShadowColor"),
//             MenuStyleParam::MenuShadowOffsetXy => set_t_value(&mut self.menu_shadow_offset_xy, value, "MenuShadowOffsetXy"),
//             MenuStyleParam::MenuShadowRgba => set_t_value(&mut self.menu_shadow_color, value, "MenuShadowRgba"),
//             // path
//             MenuStyleParam::PathBackgroundAlpha => set_t_value(&mut self.path_background_color_alpha, value, "PathBackgroundAlpha"),
//             MenuStyleParam::PathBackgroundColor => set_t_value(&mut self.path_background_color, value, "PathBackgroundColor"),
//             MenuStyleParam::PathBackgroundRgba => set_t_value(&mut self.path_background_color, value, "PathBackgroundRgba"),
//             MenuStyleParam::PathBorderAlpha => set_t_value(&mut self.path_border_color_alpha, value, "PathBorderAlpha"),
//             MenuStyleParam::PathBorderColor => set_t_value(&mut self.path_border_color, value, "PathBorderColor"),
//             MenuStyleParam::PathBorderRadius => set_t_value(&mut self.path_border_radius, value, "PathBorderRadius"),
//             MenuStyleParam::PathBorderRgba => set_t_value(&mut self.path_border_color, value, "PathBorderRgba"),
//             MenuStyleParam::PathBorderWidth => set_t_value(&mut self.path_border_width, value, "PathBorderWidth"),
//         }
//     }
// }
