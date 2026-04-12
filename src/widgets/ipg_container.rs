//! ipg_container

use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::helpers::{get_len, get_padding};
use crate::state::Widgets;

use crate::widgets::styling::{apply_background_color_overrides, 
    apply_border_overrides, apply_shadow_overrides_xy};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};

use iced::{Element, Theme, alignment};
use iced::widget::{self, Space};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;



#[derive(Debug, Clone)]
pub struct Container {
    pub id: usize,
    pub show: bool,
    pub padding: Option<Vec<f32>>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub align_top_left: Option<bool>,
    pub align_top_center: Option<bool>,
    pub align_top_right: Option<bool>,
    pub align_center_left: Option<bool>,
    pub align_center: Option<bool>,
    pub align_center_right: Option<bool>,
    pub align_bottom_left: Option<bool>,
    pub align_bottom_center: Option<bool>,
    pub align_bottom_right: Option<bool>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_std: Option<ContainerStyleStd>,
}

impl Container {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        widgets: &HashMap<usize, Widgets>,
        ) -> Element<'a, Message> {
        
        if !self.show {return Space::new().into()}

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_container_style).cloned();

        // Since a container can have only one element and the 
        // the process sends a vec then if empty container, put in a
        // space or remove the element in the vec.
        let new_content: Element<Message> = if content.is_empty() {
            Space::new().into()
        } else {
            content.remove(0)
        };

        let cont = 
            widget::Container::new(new_content)
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height))
                .padding(get_padding(&self.padding))
                .style(move|theme|
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, &self.style_std)
                    } else {
                       match &self.style_std {
                            Some(std) => std.to_iced(theme),
                            None => widget::container::transparent(theme),
                        }
                    }
                );

        let cont = 
            if let Some(mw) = self.max_width {
                cont.max_width(mw)
            } else { cont };

        let cont = 
            if let Some(mh) = self.max_height {
                cont.max_width(mh)
            } else { cont };

        let cont = 
            if self.align_top_left == Some(true) {
                cont.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Top)
            } else { cont };

        let cont = 
            if self.align_top_center == Some(true) {
                cont.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Top)
            } else { cont };

        let cont = 
            if self.align_top_right == Some(true) {
                cont.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Top)
            } else { cont };
        
        let cont = 
            if self.align_center_left == Some(true) {
                cont.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Center)
            } else { cont };

        let cont = 
            if self.align_center == Some(true) {
                cont.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center)
            } else { cont };
        
        let cont = 
            if self.align_center_right == Some(true) {
                cont.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Center)
            } else { cont };

        let cont = 
            if self.align_bottom_left == Some(true) {
                cont.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Bottom)
            } else { cont };

        let cont = 
            if self.align_bottom_center == Some(true) {
                cont.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Bottom)
            } else { cont };

        let cont = 
            if self.align_bottom_right == Some(true) {
                cont.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Bottom)
            } else { cont };

        let cont = 
            if self.clip == Some(true) {
                cont.clip(true)
            } else { cont };

        cont.into()            
        
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,
    pub background_gradient_color_stop: Option<Color>,
    pub background_gradient_color_stop_alpha: Option<f32>,
    pub background_gradient_rgba_stop: Option<[f32; 4]>,
    pub background_gradient_degrees: Option<f32>,
    pub background_gradient_radians: Option<f32>,
    pub background_gradient_alpha: Option<f32>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<Color>,
    pub shadow_color_alpha: Option<f32>,
    pub shadow_rgba: Option<[f32; 4]>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,
    pub text_color: Option<Color>,
    pub text_color_alpha: Option<f32>,
    pub text_rgba: Option<[f32; 4]>,
    pub snap: Option<bool>,
}

impl ContainerStyle {
    pub fn to_iced(
        &self, 
        theme: &Theme, 
        std_style_opt: &Option<ContainerStyleStd>,  
        ) -> widget::container::Style {
        
        let mut style = 
            if let Some(st) = std_style_opt {
                st.to_iced(theme)
            } else { widget::container::transparent(theme) };

        let background_color = 
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
        let background_gradient_color_stop = 
            Color::rgba_ipg_color_to_iced(self.background_gradient_rgba_stop, &self.background_gradient_color_stop, self.background_gradient_alpha);
        let border_color = 
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);
        let shadow_color = 
            Color::rgba_ipg_color_to_iced(self.shadow_rgba, &self.shadow_color, self.shadow_color_alpha);
        let text_color = 
            Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha);

        // Apply remaining optional overrides
        apply_background_color_overrides(
            &mut style.background, background_color,
            background_gradient_color_stop,
            self.background_gradient_degrees,
            self.background_gradient_radians,
        );

        apply_border_overrides(
            &mut style.border, border_color,
            &self.border_radius, self.border_width, "Container",
        );

        apply_shadow_overrides_xy(
            &mut style.shadow, shadow_color, 
            self.shadow_offset_xy, self.shadow_blur_radius);
        
        style.text_color = text_color;

        if let Some(sp) = self.snap {
            style.snap = sp;
        }

        style
        
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ContainerStyleStd {
    BorderedBox,
    Danger,
    Dark,
    Primary,
    RoundedBox,
    Secondary,
    Success,
    Transparent,
    Warning,
}

impl ContainerStyleStd {
    pub fn to_iced(
        &self,
        theme: &Theme, 
        ) -> widget::container::Style {
        
        match self {
            ContainerStyleStd::BorderedBox => {
                widget::container::bordered_box(theme)
            },
            ContainerStyleStd::Danger => {
                widget::container::danger(theme)
            },
            ContainerStyleStd::Dark => {
                widget::container::dark(theme)
            },
            ContainerStyleStd::Primary => {
                widget::container::primary(theme)
            },
            ContainerStyleStd::RoundedBox => {
                widget::container::rounded_box(theme)
            },
            ContainerStyleStd::Secondary => {
                widget::container::secondary(theme)
            },
            ContainerStyleStd::Success => {
                widget::container::success(theme)
            },
            ContainerStyleStd::Warning => {
                widget::container::warning(theme)
            },
            ContainerStyleStd::Transparent => {
                widget::container::transparent(theme)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ContainerParam {
    AlignBottomCenter,
    AlignBottomLeft,
    AlignBottomRight,
    AlignCenter,
    AlignCenterLeft,
    AlignCenterRight,
    AlignTopCenter,
    AlignTopLeft,
    AlignTopRight,
    Clip,
    Fill,
    Height,
    HeightFill,
    MaxHeight,
    MaxWidth,
    Padding,
    Width,
    WidthFill,
    Show,
    StyleId,
    StyleStd,
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ContainerStyleParam {
    BackgroundColor, 
    BackgroundRgba,
    BackgroundColorAlpha,
    BackgroundGradientColorStop,
    BackgroundGradientRgbaStop,
    BackgroundGradientDegrees,
    BackgroundGradientRadians,
    BackgroundGradientAlpha,
    BorderColor, 
    BorderRgba,
    BorderColorAlpha,
    BorderRadius, 
    BorderWidth,
    ShadowColor, 
    ShadowRgba,
    ShadowColorAlpha,
    ShadowOffsetXY,
    ShadowBlurRadius,
    TextColor, 
    TextRgba,
    TextColorAlpha,
    Snap,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Container {
    type Param = ContainerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ContainerParam::AlignBottomCenter => set_t_value(&mut self.align_bottom_center, value, "ContainerParam::AlignBottomCenter"),
            ContainerParam::AlignBottomLeft => set_t_value(&mut self.align_bottom_left, value, "ContainerParam::AlignBottomLeft"),
            ContainerParam::AlignBottomRight => set_t_value(&mut self.align_bottom_right, value, "ContainerParam::AlignBottomRight"),
            ContainerParam::AlignCenter => set_t_value(&mut self.align_center, value, "ContainerParam::AlignCenter"),
            ContainerParam::AlignCenterLeft => set_t_value(&mut self.align_center_left, value, "ContainerParam::AlignCenterLeft"),
            ContainerParam::AlignCenterRight => set_t_value(&mut self.align_center_right, value, "ContainerParam::AlignCenterRight"),
            ContainerParam::AlignTopCenter => set_t_value(&mut self.align_top_center, value, "ContainerParam::AlignTopCenter"),
            ContainerParam::AlignTopLeft => set_t_value(&mut self.align_top_left, value, "ContainerParam::AlignTopLeft"),
            ContainerParam::AlignTopRight => set_t_value(&mut self.align_top_right, value, "ContainerParam::AlignTopRight"),
            ContainerParam::Clip => set_t_value(&mut self.clip, value, "ContainerParam::Clip"),
            ContainerParam::Fill => set_t_value(&mut self.fill, value, "ContainerParam::Fill"),
            ContainerParam::Height => set_t_value(&mut self.height, value, "ContainerParam::Height"),
            ContainerParam::HeightFill => set_t_value(&mut self.height_fill, value, "ContainerParam::HeightFill"),
            ContainerParam::MaxHeight => set_t_value(&mut self.max_height, value, "ContainerParam::MaxHeight"),
            ContainerParam::MaxWidth => set_t_value(&mut self.max_width, value, "ContainerParam::MaxWidth"),
            ContainerParam::Padding => set_t_value(&mut self.padding, value, "ContainerParam::Padding"),
            ContainerParam::Width => set_t_value(&mut self.width, value, "ContainerParam::Width"),
            ContainerParam::WidthFill => set_t_value(&mut self.width_fill, value, "ContainerParam::WidthFill"),
            ContainerParam::Show => set_t_value(&mut self.show, value, "ContainerParam::Show"),
            ContainerParam::StyleId => set_t_value(&mut self.style_id, value, "ContainerParam::StyleId"),
            ContainerParam::StyleStd => set_t_value(&mut self.style_std, value, "ContainerParam::StyleStd"),
        }
    }
}

impl WidgetParamUpdate for ContainerStyle {
    type Param = ContainerStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ContainerStyleParam::BackgroundColor  => set_t_value(&mut self.background_color, value, "ContainerStyleParam::BackgroundColor"),
            ContainerStyleParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "ContainerStyleParam::BackgroundColorAlpha"),
            ContainerStyleParam::BackgroundGradientAlpha => set_t_value(&mut self.background_gradient_alpha, value, "ContainerStyleParam::BackgroundGradientAlpha"),
            ContainerStyleParam::BackgroundGradientColorStop => set_t_value(&mut self.background_gradient_color_stop, value, "ContainerStyleParam::BackgroundGradientColorStop"),
            ContainerStyleParam::BackgroundGradientDegrees => set_t_value(&mut self.background_gradient_degrees, value, "ContainerStyleParam::BackgroundGradientDegrees"),
            ContainerStyleParam::BackgroundGradientRadians => set_t_value(&mut self.background_gradient_radians, value, "ContainerStyleParam::BackgroundGradientRadians"),
            ContainerStyleParam::BackgroundGradientRgbaStop => set_t_value(&mut self.background_gradient_rgba_stop, value, "ContainerStyleParam::BackgroundGradientRgbaStop"),
            ContainerStyleParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "ContainerStyleParam::BackgroundRgba"),
            ContainerStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "ContainerStyleParam::BorderColor"),
            ContainerStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "ContainerStyleParam::BorderColorAlpha"),
            ContainerStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "ContainerStyleParam::BorderRadius"),
            ContainerStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "ContainerStyleParam::BorderRgba"),
            ContainerStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "ContainerStyleParam::BorderWidth"),
            ContainerStyleParam::ShadowBlurRadius => set_t_value(&mut self.shadow_blur_radius, value, "ContainerStyleParam::ShadowBlurRadius"),
            ContainerStyleParam::ShadowColor => set_t_value(&mut self.shadow_color, value, "ContainerStyleParam::ShadowColor"),
            ContainerStyleParam::ShadowColorAlpha => set_t_value(&mut self.shadow_color_alpha, value, "ContainerStyleParam::ShadowColorAlpha"),
            ContainerStyleParam::ShadowOffsetXY => set_t_value(&mut self.shadow_offset_xy, value, "ContainerStyleParam::ShadowOffsetXY"),
            ContainerStyleParam::ShadowRgba => set_t_value(&mut self.shadow_rgba, value, "ContainerStyleParam::ShadowRgba"),
            ContainerStyleParam::Snap => set_t_value(&mut self.snap, value, "ContainerStyleParam::Snap"),
            ContainerStyleParam::TextColor => set_t_value(&mut self.text_color, value, "ContainerStyleParam::TextColor"),
            ContainerStyleParam::TextColorAlpha => set_t_value(&mut self.text_color_alpha, value, "ContainerStyleParam::TextColorAlpha"),
            ContainerStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "ContainerStyleParam::TextRgba"),
        }
    }
}
