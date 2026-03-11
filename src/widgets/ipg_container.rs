//! ipg_container
use std::collections::HashMap;

use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::state::IpgWidgets;

use crate::widgets::styling::{apply_background_overrides, 
    apply_border_overrides, apply_shadow_overrides_xy};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_height, 
    set_height_fill, set_iced_color_from_rgba, 
    set_opt_f32_array_2, set_opt_bool, set_opt_f32, 
    set_opt_iced_color, set_opt_usize, set_opt_vec_f32, 
    set_width, set_width_fill
};

use iced::{Color, Element, Length, Theme, alignment};
use iced::widget::{container, Space, Container};

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct IpgContainer {
    pub id: usize,
    pub show: bool,

    pub padding: Option<Vec<f32>>,
    pub width: Length,
    pub height: Length,
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
    pub style_std: Option<IpgContainerStyleStd>,
}

impl IpgContainer {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        widgets: &HashMap<usize, IpgWidgets>,
        ) -> Element<'a, Message> {
        
        if !self.show {return Space::new().into()}

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(IpgWidgets::as_container_style).cloned();

        // Since a container can have only one element and the 
        // the process sends a vec then if empty container, put in a
        // space or remove the element in the vec.
        let new_content: Element<Message> = if content.is_empty() {
            Space::new().into()
        } else {
            content.remove(0)
        };

        let cont = 
            Container::new(new_content)
                .width(self.width)
                .height(self.height)
                .padding(get_padding(&self.padding))
                .style(move|theme|
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, &self.style_std)
                    } else {
                       match &self.style_std {
                            Some(std) => std.to_iced(theme),
                            None => container::transparent(theme),
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
pub struct IpgContainerStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_gradient_color_stop: Option<Color>,
    pub background_gradient_degrees: Option<f32>,
    pub background_gradient_radians: Option<f32>,
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<Color>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,
    pub text_color: Option<Color>,
    pub snap: Option<bool>,
}

impl IpgContainerStyle {
    pub fn to_iced(
        &self, 
        theme: &Theme, 
        std_style_opt: &Option<IpgContainerStyleStd>,  
        ) -> container::Style {
        
        let mut style = 
            if let Some(st) = std_style_opt {
                st.to_iced(theme)
            } else { container::transparent(theme) };

        

        // Apply remaining optional overrides
        apply_background_overrides(
            &mut style.background, self.background_color,
            self.background_gradient_color_stop,
            self.background_gradient_degrees,
            self.background_gradient_radians,
        );

        apply_border_overrides(
            &mut style.border, self.border_color,
            &self.border_radius, self.border_width, "Container",
        );

        apply_shadow_overrides_xy(
            &mut style.shadow, self.shadow_color, 
            self.shadow_offset_xy, self.shadow_blur_radius);
        
        style.text_color = self.text_color;

        if let Some(sp) = self.snap {
            style.snap = sp;
        }

        style
        
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgContainerStyleStd {
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

impl IpgContainerStyleStd {
    pub fn to_iced(
        &self,
        theme: &Theme, 
        ) -> container::Style {
        
        match self {
            IpgContainerStyleStd::BorderedBox => {
                container::bordered_box(theme)
            },
            IpgContainerStyleStd::Danger => {
                container::danger(theme)
            },
            IpgContainerStyleStd::Dark => {
                container::dark(theme)
            },
            IpgContainerStyleStd::Primary => {
                container::primary(theme)
            },
            IpgContainerStyleStd::RoundedBox => {
                container::rounded_box(theme)
            },
            IpgContainerStyleStd::Secondary => {
                container::secondary(theme)
            },
            IpgContainerStyleStd::Success => {
                container::success(theme)
            },
            IpgContainerStyleStd::Warning => {
                container::warning(theme)
            },
            IpgContainerStyleStd::Transparent => {
                container::transparent(theme)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgContainerParam {
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
    Height,
    HeightFill,
    MaxHeight,
    MaxWidth,
    Padding,
    Width,
    WidthFill,
    Show,
    StyleId,
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgContainerStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetXY,
    ShadowBlurRadius,
    TextIpgColor,
    TextRgbaColor,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgContainer {
    type Param = IpgContainerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        let name = String::new();
        match param {
            IpgContainerParam::AlignBottomCenter => set_opt_bool(&mut self.align_bottom_center, value, name),
            IpgContainerParam::AlignBottomLeft => set_opt_bool(&mut self.align_bottom_left, value, name),
            IpgContainerParam::AlignBottomRight => set_opt_bool(&mut self.align_bottom_right, value, name),
            IpgContainerParam::AlignCenter => set_opt_bool(&mut self.align_center, value, name),
            IpgContainerParam::AlignCenterLeft => set_opt_bool(&mut self.align_center_left, value, name),
            IpgContainerParam::AlignCenterRight => set_opt_bool(&mut self.align_center_right, value, name),
            IpgContainerParam::AlignTopCenter => set_opt_bool(&mut self.align_top_center, value, name),
            IpgContainerParam::AlignTopLeft => set_opt_bool(&mut self.align_top_left, value, name),
            IpgContainerParam::AlignTopRight => set_opt_bool(&mut self.align_top_right, value, name),
            IpgContainerParam::Clip => set_opt_bool(&mut self.clip, value, name),
            IpgContainerParam::Height => set_height(&mut self.height, value, name),
            IpgContainerParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgContainerParam::MaxHeight => set_opt_f32(&mut self.max_height, value, name),
            IpgContainerParam::MaxWidth => set_opt_f32(&mut self.max_width, value, name),
            IpgContainerParam::Padding => set_opt_vec_f32(&mut self.padding, value, name),
            IpgContainerParam::Width => set_width(&mut self.width, value, name),
            IpgContainerParam::WidthFill => set_width_fill(&mut self.width, value, name),
            IpgContainerParam::Show => set_bool(&mut self.show, value, name),
            IpgContainerParam::StyleId => set_opt_usize(&mut self.style_id, value, name),
            
                    }
    }
}

impl WidgetParamUpdate for IpgContainerStyle {
    type Param = IpgContainerStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        let name = String::new();
        match param {
            IpgContainerStyleParam::BackgroundIpgColor  => set_opt_iced_color(&mut self.background_color, value, name),
            IpgContainerStyleParam::BackgroundRgbaColor => set_iced_color_from_rgba(&mut self.background_color, value, name),
            IpgContainerStyleParam::BorderIpgColor      => set_opt_iced_color(&mut self.border_color, value, name),
            IpgContainerStyleParam::BorderRgbaColor     => set_iced_color_from_rgba(&mut self.border_color, value, name),
            IpgContainerStyleParam::BorderRadius        => set_opt_vec_f32(&mut self.border_radius, value, name),
            IpgContainerStyleParam::BorderWidth         => set_opt_f32(&mut self.border_width, value, name),
            IpgContainerStyleParam::ShadowIpgColor      => set_opt_iced_color(&mut self.shadow_color, value, name),
            IpgContainerStyleParam::ShadowRgbaColor     => set_iced_color_from_rgba(&mut self.shadow_color, value, name),
            IpgContainerStyleParam::ShadowOffsetXY      => set_opt_f32_array_2(&mut self.shadow_offset_xy, value, name),
            IpgContainerStyleParam::ShadowBlurRadius    => set_opt_f32(&mut self.shadow_blur_radius, value, name),
            IpgContainerStyleParam::TextIpgColor        => set_opt_iced_color(&mut self.text_color, value, name),
            IpgContainerStyleParam::TextRgbaColor       => set_iced_color_from_rgba(&mut self.text_color, value, name),
        }
    }
}
