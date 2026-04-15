//! ipg_menu

#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::{Element, Length, Renderer, Theme, border};
use iced_aw::{MenuBar, menu::{DrawPath, Item, ScrollSpeed}};
use iced_aw::menu;
use crate::{app, py_api::helpers::get_padding, 
state::Widgets, widgets::{styling::{apply_border_overrides, apply_shadow_overrides_xy}, widget_param_update::{
    WidgetParamUpdate, set_bool, set_height, set_height_fill, set_opt_f32, set_opt_f32_array_2, set_opt_iced_color, set_opt_iced_color_from_rgba, set_opt_vec_f32_1_or_upto_4, set_t_value, set_vec_f32, set_width
}}};



use pyo3::{pyclass, Py, PyAny};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct Menu {
    pub id: usize,
    pub item_offsets: Vec<f32>,
    pub item_paddings: Vec<f32>,
    pub item_spacings: Vec<f32>,
    pub item_widths: Vec<Length>,
    pub items_close_on_click: Option<Vec<Option<bool>>>,
    pub items_close_on_background_click: Option<Vec<Option<bool>>>,
    pub items_close_on_click_global: Option<bool>,
    pub items_close_on_background_click_global: Option<bool>,
    pub bar_height: Length,
    pub bar_paddings: Vec<f32>,
    pub bar_spacing: Option<f32>,
    pub bar_width: Length,
    pub close_on_bar_item_click: Option<bool>,
    pub close_on_bar_background_click: Option<bool>,
    pub style_id: Option<usize>,
    pub style_std_primary: Option<bool>,
    pub cursor_bounds_margin: Option<f32>,
    pub scroll_speed_line: Option<f32>,
    pub scroll_speed_pixel: Option<f32>,
    pub show: bool,
    pub is_checked: bool,
    pub is_toggled: bool,
}

impl Menu {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self, 
        grouped_content: Vec<Vec<Element<'a, app::Message>>>,
        widgets: &HashMap<usize, Widgets>,
        )-> Element<'a, app::Message, Theme, Renderer> 
    {
        
        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_menu_style).cloned();

        
        let mut bar_items: Vec<Item<app::Message, Theme, Renderer>> = vec![];

        let item_widths = if self.item_widths.len() == 1 {
                vec![self.item_widths[0]; grouped_content.len()]
            } else {
                self.item_widths.clone()
            };
        
        let item_offsets = if self.item_offsets.len() == 1 {
            vec![self.item_offsets[0]; grouped_content.len()]
        } else {
            self.item_offsets.clone()
        };

        let item_spacings = if self.item_spacings.len() == 1 {
            vec![self.item_spacings[0]; grouped_content.len()]
        } else {
            self.item_spacings.clone()
        };

        let items_close_on_click = if let Some(icoc) = &self.items_close_on_click {
            icoc.clone()
        } else {
            vec![None; grouped_content.len()]
        };

        let items_close_on_background_click = if let Some(icobc) = &self.items_close_on_background_click {
            icobc.clone()
        } else {
            vec![None; grouped_content.len()]
        };

        let mut index = 0usize;
        
        for (_bar_index, mut group) in grouped_content.into_iter().enumerate() {
            if group.is_empty() {
                continue;
            }

            // First element is the bar widget, rest are dropdown items
            let menu_bar = group.remove(0);

            let items: Vec<Item<app::Message, Theme, Renderer>> = group
                .into_iter()
                .map(|el| Item::new(el))
                .collect();

            let mut menu = iced_aw::Menu::new(items)
                //.max_width(100.0)
                .spacing(item_spacings[index])
                .width(item_widths[index])
                .offset(item_offsets[index])
                .padding(get_padding(&Some(self.item_paddings.clone())));

            if let Some(v) = items_close_on_click[index] {
                menu = menu.close_on_item_click(v);
            }
            if let Some(v) = items_close_on_background_click[index] {
                menu = menu.close_on_background_click(v);
            }
            
            let bar_item = 
                Item::with_menu(
                    menu_bar, 
                    menu
                    );
            index += 1;
            bar_items.push(bar_item);
            
        }
        
        let mut mb: MenuBar<'a, app::Message, Theme, Renderer> = 
            MenuBar::new(bar_items)
                .close_on_item_click_global(self.items_close_on_click_global.unwrap_or_default())
                .close_on_background_click_global(self.items_close_on_background_click_global.unwrap_or_default())
                .draw_path(DrawPath::FakeHovering)
                .spacing(self.bar_spacing.unwrap_or(0.0))
                .padding(get_padding(&Some(self.bar_paddings.clone())))
                .width(self.bar_width)
                .height(self.bar_height)
                .safe_bounds_margin(self.cursor_bounds_margin.unwrap_or(50.0))
                .scroll_speed(ScrollSpeed{
                    line: self.scroll_speed_line.unwrap_or(60.0),
                    pixel: self.scroll_speed_pixel.unwrap_or(1.0)
                })
                .style(move |theme: &Theme, _| {
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, self.style_std_primary)
                    } else if self.style_std_primary == Some(true) {
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
    pub show: bool,
}



#[derive(Debug, Clone)]
pub struct MenuStyle {
    pub id: usize,
    pub bar_background_color: Option<iced::Color>,
    pub bar_border_color: Option<iced::Color>,
    pub bar_border_radius: Option<Vec<f32>>,
    pub bar_border_width: Option<f32>,
    pub bar_shadow_color: Option<iced::Color>,
    pub bar_shadow_offset_xy: Option<[f32; 2]>,
    pub bar_shadow_blur_radius: Option<f32>,
    pub bar_background_alpha: Option<f32>,
    pub bar_border_alpha: Option<f32>,
    pub bar_shadow_alpha: Option<f32>,

    pub menu_background_color: Option<iced::Color>,
    pub menu_border_color: Option<iced::Color>,
    pub menu_border_radius: Option<Vec<f32>>,
    pub menu_border_width: Option<f32>,
    pub menu_shadow_color: Option<iced::Color>,
    pub menu_shadow_offset_xy: Option<[f32; 2]>,
    pub menu_shadow_blur_radius: Option<f32>,
    pub menu_background_alpha: Option<f32>,
    pub menu_border_alpha: Option<f32>,
    pub menu_shadow_alpha: Option<f32>,

    pub path_background_color: Option<iced::Color>,
    pub path_border_color: Option<iced::Color>,
    pub path_border_radius: Option<Vec<f32>>,
    pub path_border_width: Option<f32>,
    pub path_background_alpha: Option<f32>,
    pub path_border_alpha: Option<f32>,
}

impl MenuStyle {
    fn to_iced(
        &self,
        theme: &Theme, 
        primary_opt: Option<bool>
        ) -> menu::Style {

        //The base style will be either default or primary
        let mut style = 
            if primary_opt == Some(true) {
                primary(theme)
            } else { menu::Style::default() };

        // making defaults square
        style.bar_border.radius = 0.0.into();
        style.menu_border.radius = 0.0.into();

        // bar
        if let Some(color) = self.bar_background_color {
            style.bar_background = color.into()
        }

        apply_border_overrides(
            &mut style.bar_border, self.bar_border_color,
            &self.bar_border_radius, self.bar_border_width, "Menu-bar",
        );

        apply_shadow_overrides_xy(
            &mut style.bar_shadow, self.bar_shadow_color, 
            self.bar_shadow_offset_xy, self.bar_shadow_blur_radius);
        
        // menu
        if let Some(color) = self.menu_background_color {
            style.menu_background = color.into()
        }

        apply_border_overrides(
            &mut style.menu_border, self.menu_border_color,
            &self.menu_border_radius, self.menu_border_width, "Menu-menu",
        );

        apply_shadow_overrides_xy(
            &mut style.menu_shadow, self.menu_shadow_color, 
            self.menu_shadow_offset_xy, self.menu_shadow_blur_radius);

        // path
        if let Some(color) = self.path_background_color {
            style.path = color.into()
        }

        apply_border_overrides(
            &mut style.path_border, self.path_border_color,
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
        bar_border: border::rounded(2),
        ..menu::Style::default()
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuParam {
    BarHeight,
    BarHeightFill,
    BarPadding,
    BarSpacing,
    BarWidth,
    CursorBoundsMargin,
    ScrollSpeedLine,
    ScrollSpeedPixel,
    Show,
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
            MenuParam::BarHeight => set_height(&mut self.bar_height, value, "BarHeight"),
            MenuParam::BarHeightFill => set_height_fill(&mut self.bar_height, value, "BarHeightFill"),
            MenuParam::BarPadding => set_vec_f32(&mut self.bar_paddings, value, "BarPadding"),
            MenuParam::BarSpacing => set_opt_f32(&mut self.bar_spacing, value, "BarSpacing"),
            MenuParam::BarWidth => set_width(&mut self.bar_width, value, "BarWidth"),
            MenuParam::CursorBoundsMargin => set_opt_f32(&mut self.cursor_bounds_margin, value, "CheckBoundsMargin"),
            MenuParam::ScrollSpeedLine => set_t_value(&mut self.scroll_speed_line, value, "MenuParam::ScrollSpeedLine"),
            MenuParam::ScrollSpeedPixel => set_t_value(&mut self.scroll_speed_pixel, value, "MenuParam::ScrollSpeedPixel"),
            MenuParam::Show => set_bool(&mut self.show, value, "Show"),
        }
    }
}

impl WidgetParamUpdate for MenuStyle {
    type Param = MenuStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            // bar
            MenuStyleParam::BarBackgroundColor => set_opt_iced_color(&mut self.bar_background_color, value, "BarBackgroundColor"),
            MenuStyleParam::BarBackgroundRgba => set_opt_iced_color_from_rgba(&mut self.bar_background_color, value, "BarBackgroundRgba"),
            MenuStyleParam::BarBackgroundAlpha => set_opt_f32(&mut self.bar_background_alpha, value, "BarBackgroundAlpha"),
            MenuStyleParam::BarBorderColor => set_opt_iced_color(&mut self.bar_border_color, value, "BarBorderColor"),
            MenuStyleParam::BarBorderRgba => set_opt_iced_color_from_rgba(&mut self.bar_border_color, value, "BarBorderRgba"),
            MenuStyleParam::BarBorderAlpha => set_opt_f32(&mut self.bar_border_alpha, value, "BarBorderAlpha"),
            MenuStyleParam::BarBorderRadius => set_opt_vec_f32_1_or_upto_4(&mut self.bar_border_radius, value, "BarBorderRadius"),
            MenuStyleParam::BarBorderWidth => set_opt_f32(&mut self.bar_border_width, value, "BarBorderWidth"),
            MenuStyleParam::BarShadowColor => set_opt_iced_color(&mut self.bar_shadow_color, value, "BarShadowColor"),
            MenuStyleParam::BarShadowRgba => set_opt_iced_color_from_rgba(&mut self.bar_shadow_color, value, "BarShadowRgba"),
            MenuStyleParam::BarShadowAlpha => set_opt_f32(&mut self.bar_shadow_alpha, value, "BarShadowAlpha"),
            MenuStyleParam::BarShadowOffsetXY => set_opt_f32_array_2(&mut self.bar_shadow_offset_xy, value, "BarShadowOffsetXY"),
            MenuStyleParam::BarShadowBlurRadius => set_opt_f32(&mut self.bar_shadow_blur_radius, value, "BarShadowBlurRadius"),
            // menu
            MenuStyleParam::MenuBackgroundColor => set_opt_iced_color(&mut self.menu_background_color, value, "MenuBackgroundColor"),
            MenuStyleParam::MenuBackgroundRgba => set_opt_iced_color_from_rgba(&mut self.menu_background_color, value, "MenuBackgroundRgba"),
            MenuStyleParam::MenuBackgroundAlpha => set_opt_f32(&mut self.menu_background_alpha, value, "MenuBackgroundAlpha"),
            MenuStyleParam::MenuBorderColor => set_opt_iced_color(&mut self.menu_border_color, value, "MenuBorderColor"),
            MenuStyleParam::MenuBorderRgba => set_opt_iced_color_from_rgba(&mut self.menu_border_color, value, "MenuBorderRgba"),
            MenuStyleParam::MenuBorderAlpha => set_opt_f32(&mut self.menu_border_alpha, value, "MenuBorderAlpha"),
            MenuStyleParam::MenuBorderRadius => set_opt_vec_f32_1_or_upto_4(&mut self.menu_border_radius, value, "MenuBorderRadius"),
            MenuStyleParam::MenuBorderWidth => set_opt_f32(&mut self.menu_border_width, value, "MenuBorderWidth"),
            MenuStyleParam::MenuShadowColor => set_opt_iced_color(&mut self.menu_shadow_color, value, "MenuShadowColor"),
            MenuStyleParam::MenuShadowRgba => set_opt_iced_color_from_rgba(&mut self.menu_shadow_color, value, "MenuShadowRgba"),
            MenuStyleParam::MenuShadowAlpha => set_opt_f32(&mut self.menu_shadow_alpha, value, "MenuShadowAlpha"),
            MenuStyleParam::MenuShadowOffsetXy => set_opt_f32_array_2(&mut self.menu_shadow_offset_xy, value, "MenuShadowOffsetXy"),
            MenuStyleParam::MenuShadowBlurRadius => set_opt_f32(&mut self.menu_shadow_blur_radius, value, "MenuShadowBlurRadius"),
            // path
            MenuStyleParam::PathBackgroundColor => set_opt_iced_color(&mut self.path_background_color, value, "PathBackgroundColor"),
            MenuStyleParam::PathBackgroundRgba => set_opt_iced_color_from_rgba(&mut self.path_background_color, value, "PathBackgroundRgba"),
            MenuStyleParam::PathBackgroundAlpha => set_opt_f32(&mut self.path_background_alpha, value, "PathBackgroundAlpha"),
            MenuStyleParam::PathBorderColor => set_opt_iced_color(&mut self.path_border_color, value, "PathBorderColor"),
            MenuStyleParam::PathBorderRgba => set_opt_iced_color_from_rgba(&mut self.path_border_color, value, "PathBorderRgba"),
            MenuStyleParam::PathBorderAlpha => set_opt_f32(&mut self.path_border_alpha, value, "PathBorderAlpha"),
            MenuStyleParam::PathBorderRadius => set_opt_vec_f32_1_or_upto_4(&mut self.path_border_radius, value, "PathBorderRadius"),
            MenuStyleParam::PathBorderWidth => set_opt_f32(&mut self.path_border_width, value, "PathBorderWidth"),
        }
    }
}