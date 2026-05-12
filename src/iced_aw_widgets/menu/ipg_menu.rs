//! ipg_menu

#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::{Element, Length, Renderer, Theme};
use iced_aw::{MenuBar, menu::{DrawPath, Item, ScrollSpeed}, style::{self, menu_bar}};
use iced_aw::menu;
use crate::{app, graphics::colors::Color, py_api::helpers::{get_len, get_padding}, state::{Containers, Widgets}, widgets::{styling::{apply_border_overrides, 
    apply_shadow_overrides_xy}, widget_param_update::{
    WidgetParamUpdate, set_t_value}}};



use pyo3::{pyclass, Py, PyAny};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct Menu {
    pub id: usize,
    pub padding: Option<Vec<f32>>,
    pub spacing: Option<f32>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub close_on_bar_item_click: Option<bool>,
    pub close_on_bar_background_click: Option<bool>,
    pub items_close_on_click_global: Option<bool>,
    pub items_close_on_background_click_global: Option<bool>,
    pub style_id: Option<usize>,
    pub style_primary: Option<bool>,
    pub cursor_bounds_margin: Option<f32>,
    pub scroll_speed_line: Option<f32>,
    pub scroll_speed_pixel: Option<f32>,
    pub show: bool,
}

impl Menu {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self, 
        grouped_content: Vec<(usize, Vec<Element<'a, app::Message>>)>,
        widgets: &HashMap<usize, Widgets>,
        containers: &HashMap<usize, Containers>,
        )-> Element<'a, app::Message, Theme, Renderer> 
    {
        
        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_menu_style).cloned();

        
        let mut bar_items: Vec<Item<app::Message, Theme, Renderer>> = vec![];
        
        for (bar_item_id, mut group) in grouped_content.into_iter() {
            if group.is_empty() {
                continue;
            }

            let bar_item_data = containers.get(&bar_item_id)
                .and_then(Containers::as_menu_bar_item)
                .expect("MenuBarItem not found in containers");

            // First element is the bar widget, rest are dropdown items
            let menu_bar = group.remove(0);

            let items: Vec<Item<app::Message, Theme, Renderer>> = group
                .into_iter()
                .map(|el| Item::new(el))
                .collect();

            let mut menu = iced_aw::Menu::new(items)
                .spacing(bar_item_data.spacing.unwrap_or(0.0))
                .width({
                        if let Some(width) = bar_item_data.width {
                            iced::Length::Fixed(width)
                        } else {
                            Length::Shrink
                        }
                    })
                .offset(bar_item_data.offset.unwrap_or(0.0))
                .padding(get_padding(&bar_item_data.padding));

            if let Some(v) = bar_item_data.close_on_item_click {
                menu = menu.close_on_item_click(v);
            }
            if let Some(v) = bar_item_data.close_on_background_click {
                menu = menu.close_on_background_click(v);
            }
            
            let bar_item = 
                Item::with_menu(
                    menu_bar, 
                    menu
                    );
            bar_items.push(bar_item);
            
        }
        
        let mut mb: MenuBar<'a, app::Message, Theme, Renderer> = 
            MenuBar::new(bar_items)
                .close_on_item_click_global(self.items_close_on_click_global.unwrap_or_default())
                .close_on_background_click_global(self.items_close_on_background_click_global.unwrap_or_default())
                .draw_path(DrawPath::FakeHovering)
                .spacing(self.spacing.unwrap_or(0.0))
                .padding(get_padding(&self.padding))
                .width(get_len(None, self.width_fill, self.width))
                .height(get_len(None, None, self.height))
                .safe_bounds_margin(self.cursor_bounds_margin.unwrap_or(50.0))
                .scroll_speed(ScrollSpeed{
                    line: self.scroll_speed_line.unwrap_or(60.0),
                    pixel: self.scroll_speed_pixel.unwrap_or(1.0)
                })
                .style(move |theme: &Theme, status| {
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, status, self.style_primary)
                    } else if self.style_primary == Some(true) {
                        primary(theme)
                    } else {
                        primary(theme)
                    }
                });

        if let Some(v) = self.close_on_bar_item_click {
            mb = mb.close_on_item_click(v);
        }
        if let Some(v) = self.close_on_bar_background_click {
            mb = mb.close_on_background_click(v);
        }

        mb.into()
    }
}


#[derive(Debug, Clone)]
pub struct MenuBarItem {
    pub id: usize,
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

impl MenuStyle {
    fn to_iced(
        &self,
        theme: &Theme,
        status: style::Status,
        style_std: Option<bool>,
        ) -> menu::Style {

        //The base style will be either default or primary
        let mut style = 
            if style_std == Some(true) {
                menu_bar::primary(theme, status)
            } else { menu::Style::default() };

        let bar_background_color = 
        Color::rgba_ipg_color_to_iced(
            self.bar_background_rgba, 
            &self.bar_background_color, 
            self.bar_background_color_alpha);
        
        let bar_border_color = 
            Color::rgba_ipg_color_to_iced(
                self.bar_border_rgba, 
                &self.bar_border_color, 
                self.bar_border_color_alpha);
        
        let bar_shadow_color = 
            Color::rgba_ipg_color_to_iced(
                self.bar_shadow_rgba, 
                &self.bar_shadow_color, 
                self.bar_shadow_color_alpha);

        let menu_background_color = 
            Color::rgba_ipg_color_to_iced(
                self.menu_background_rgba, 
                &self.menu_background_color, 
                self.menu_background_color_alpha);
        
        let menu_border_color = 
            Color::rgba_ipg_color_to_iced(
                self.menu_border_rgba, 
                &self.menu_border_color, 
                self.menu_border_color_alpha);
        
        let menu_shadow_color = 
            Color::rgba_ipg_color_to_iced(
                self.menu_shadow_rgba, 
                &self.menu_shadow_color, 
                self.menu_shadow_color_alpha);

        let path_background_color = 
            Color::rgba_ipg_color_to_iced(
                self.path_background_rgba, 
                &self.path_background_color, 
                self.path_background_color_alpha);
        
        let path_border_color = 
            Color::rgba_ipg_color_to_iced(
                self.path_border_rgba, 
                &self.path_border_color, 
                self.path_border_color_alpha);

        // making defaults square
        style.bar_border.radius = 0.0.into();
        style.menu_border.radius = 0.0.into();

        // bar
        if let Some(color) = bar_background_color {
            style.bar_background = color.into()
        }

        apply_border_overrides(
            &mut style.bar_border, bar_border_color,
            &self.bar_border_radius, self.bar_border_width, "Menu-bar",
        );

        apply_shadow_overrides_xy(
            &mut style.bar_shadow, bar_shadow_color, 
            self.bar_shadow_offset_xy, self.bar_shadow_blur_radius);
        
        // menu
        if let Some(color) = menu_background_color {
            style.menu_background = color.into()
        }

        apply_border_overrides(
            &mut style.menu_border, menu_border_color,
            &self.menu_border_radius, self.menu_border_width, "Menu-menu",
        );

        apply_shadow_overrides_xy(
            &mut style.menu_shadow, menu_shadow_color, 
            self.menu_shadow_offset_xy, self.menu_shadow_blur_radius);

        // path
        if let Some(color) = path_background_color {
            style.path = color.into()
        }

        apply_border_overrides(
            &mut style.path_border, path_border_color,
            &self.path_border_radius, self.path_border_width, "Menu-path",
        );

        style

    }
}

pub fn primary(theme: &Theme) -> menu::Style {
    let palette = theme.extended_palette();
    let pair = palette.background.strong;
    
    menu::Style {
        bar_background: pair.color.into(),
        menu_background: pair.color.into(),
        bar_border: iced::border::rounded(2),
        ..menu::Style::default()
    }
}

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

impl WidgetParamUpdate for Menu {
    type Param = MenuParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            MenuParam::CursorBoundsMargin => set_t_value(&mut self.cursor_bounds_margin, value, "MenuParam::CheckBoundsMargin"),
            MenuParam::Height => set_t_value(&mut self.height, value, "MenuParam::Height"),
            MenuParam::Padding => set_t_value(&mut self.padding, value, "MenuParam::Padding"),
            MenuParam::ScrollSpeedLine => set_t_value(&mut self.scroll_speed_line, value, "MenuParam::ScrollSpeedLine"),
            MenuParam::ScrollSpeedPixel => set_t_value(&mut self.scroll_speed_pixel, value, "MenuParam::ScrollSpeedPixel"),
            MenuParam::Show => set_t_value(&mut self.show, value, "MenuParam::Show"),
            MenuParam::Spacing => set_t_value(&mut self.spacing, value, "Spacing"),
            MenuParam::Width => set_t_value(&mut self.width, value, "MenuParam::Width"),
            MenuParam::CloseOnBarBackgroundClick => set_t_value(&mut self.close_on_bar_background_click, value, "MenuParam::CloseOnBarBackgroundClick"),
            MenuParam::CloseOnBarItemClick => set_t_value(&mut self.close_on_bar_item_click, value, "MenuParam::CloseOnBarItemClick"),
            MenuParam::ItemsCloseOnBackgroundClickGlobal => set_t_value(&mut self.items_close_on_background_click_global, value, "MenuParam::ItemsCloseOnBackgroundClickGlobal"),
            MenuParam::ItemsCloseOnClickGlobal => set_t_value(&mut self.items_close_on_click_global, value, "MenuParam::ItemsCloseOnClickGlobal"),
            MenuParam::StyleId => set_t_value(&mut self.style_id, value, "MenuParam::StyleId"),
            MenuParam::StylePrimary => set_t_value(&mut self.style_primary, value, "MenuParam::StylePrimary"),
            MenuParam::WidthFill => set_t_value(&mut self.width_fill, value, "MenuParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for MenuBarItem {
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

impl WidgetParamUpdate for MenuStyle {
    type Param = MenuStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            // bar
            MenuStyleParam::BarBackgroundAlpha => set_t_value(&mut self.bar_background_color_alpha, value, "BarBackgroundAlpha"),
            MenuStyleParam::BarBackgroundColor => set_t_value(&mut self.bar_background_color, value, "BarBackgroundColor"),
            MenuStyleParam::BarBackgroundRgba => set_t_value(&mut self.bar_background_color, value, "BarBackgroundRgba"),
            MenuStyleParam::BarBorderAlpha => set_t_value(&mut self.bar_border_color_alpha, value, "BarBorderAlpha"),
            MenuStyleParam::BarBorderColor => set_t_value(&mut self.bar_border_color, value, "BarBorderColor"),
            MenuStyleParam::BarBorderRadius => set_t_value(&mut self.bar_border_radius, value, "BarBorderRadius"),
            MenuStyleParam::BarBorderRgba => set_t_value(&mut self.bar_border_color, value, "BarBorderRgba"),
            MenuStyleParam::BarBorderWidth => set_t_value(&mut self.bar_border_width, value, "BarBorderWidth"),
            MenuStyleParam::BarShadowAlpha => set_t_value(&mut self.bar_shadow_color_alpha, value, "BarShadowAlpha"),
            MenuStyleParam::BarShadowBlurRadius => set_t_value(&mut self.bar_shadow_blur_radius, value, "BarShadowBlurRadius"),
            MenuStyleParam::BarShadowColor => set_t_value(&mut self.bar_shadow_color, value, "BarShadowColor"),
            MenuStyleParam::BarShadowOffsetXY => set_t_value(&mut self.bar_shadow_offset_xy, value, "BarShadowOffsetXY"),
            MenuStyleParam::BarShadowRgba => set_t_value(&mut self.bar_shadow_color, value, "BarShadowRgba"),
            // menu
            MenuStyleParam::MenuBackgroundAlpha => set_t_value(&mut self.menu_background_color_alpha, value, "MenuBackgroundAlpha"),
            MenuStyleParam::MenuBackgroundColor => set_t_value(&mut self.menu_background_color, value, "MenuBackgroundColor"),
            MenuStyleParam::MenuBackgroundRgba => set_t_value(&mut self.menu_background_color, value, "MenuBackgroundRgba"),
            MenuStyleParam::MenuBorderAlpha => set_t_value(&mut self.menu_border_color_alpha, value, "MenuBorderAlpha"),
            MenuStyleParam::MenuBorderColor => set_t_value(&mut self.menu_border_color, value, "MenuBorderColor"),
            MenuStyleParam::MenuBorderRadius => set_t_value(&mut self.menu_border_radius, value, "MenuBorderRadius"),
            MenuStyleParam::MenuBorderRgba => set_t_value(&mut self.menu_border_color, value, "MenuBorderRgba"),
            MenuStyleParam::MenuBorderWidth => set_t_value(&mut self.menu_border_width, value, "MenuBorderWidth"),
            MenuStyleParam::MenuShadowAlpha => set_t_value(&mut self.menu_shadow_color_alpha, value, "MenuShadowAlpha"),
            MenuStyleParam::MenuShadowBlurRadius => set_t_value(&mut self.menu_shadow_blur_radius, value, "MenuShadowBlurRadius"),
            MenuStyleParam::MenuShadowColor => set_t_value(&mut self.menu_shadow_color, value, "MenuShadowColor"),
            MenuStyleParam::MenuShadowOffsetXy => set_t_value(&mut self.menu_shadow_offset_xy, value, "MenuShadowOffsetXy"),
            MenuStyleParam::MenuShadowRgba => set_t_value(&mut self.menu_shadow_color, value, "MenuShadowRgba"),
            // path
            MenuStyleParam::PathBackgroundAlpha => set_t_value(&mut self.path_background_color_alpha, value, "PathBackgroundAlpha"),
            MenuStyleParam::PathBackgroundColor => set_t_value(&mut self.path_background_color, value, "PathBackgroundColor"),
            MenuStyleParam::PathBackgroundRgba => set_t_value(&mut self.path_background_color, value, "PathBackgroundRgba"),
            MenuStyleParam::PathBorderAlpha => set_t_value(&mut self.path_border_color_alpha, value, "PathBorderAlpha"),
            MenuStyleParam::PathBorderColor => set_t_value(&mut self.path_border_color, value, "PathBorderColor"),
            MenuStyleParam::PathBorderRadius => set_t_value(&mut self.path_border_radius, value, "PathBorderRadius"),
            MenuStyleParam::PathBorderRgba => set_t_value(&mut self.path_border_color, value, "PathBorderRgba"),
            MenuStyleParam::PathBorderWidth => set_t_value(&mut self.path_border_width, value, "PathBorderWidth"),
        }
    }
}