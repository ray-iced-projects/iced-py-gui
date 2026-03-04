//! ipg_container
use std::collections::HashMap;

use crate::app::Message;
use crate::state::IpgWidgets;
use crate::widgets::enums::{IpgAlignmentX, 
    IpgAlignmentY};
use crate::widgets::styling::{apply_background_overrides, 
    apply_border_overrides, apply_shadow_overrides_xy};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_halign, set_height, 
    set_height_fill, set_iced_color_from_rgba, 
    set_opt_f32_array_2, set_opt_bool, set_opt_f32, 
    set_opt_iced_color, set_opt_usize, set_opt_vec_f32, 
    set_valign, set_width, set_width_fill
};

use iced::{Color, Element, Length, Theme};
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
    pub align_x: Option<IpgAlignmentX>,
    pub align_y: Option<IpgAlignmentY>,
    pub center_x: Option<bool>,
    pub center_y: Option<bool>,
    pub center: Option<bool>,
    pub align_left: Option<bool>,
    pub align_right: Option<bool>,
    pub align_top: Option<bool>,
    pub align_botton: Option<bool>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgContainerStyleStd>,
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
                .style(move|theme|
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, &self.style_standard)
                    } else {
                       match &self.style_standard {
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
            if let Some(align) = &self.align_x {
                cont.align_x(align.to_iced())
            } else { cont };

        let cont = 
            if let Some(align) = &self.align_y {
                cont.align_y(align.to_iced())
            } else { cont };

        let cont = 
            if self.center == Some(true) {
                cont.center(self.width)
            } else { cont };

        let cont = 
            if self.center_x == Some(true) {
                cont.center_x(self.width)
            } else { cont };

        let cont = 
            if self.center_y == Some(true) {
                cont.center_y(self.height)
            } else { cont };

        let cont = 
            if self.align_left == Some(true) {
                cont.align_left(self.width)
            } else { cont };

        let cont = 
            if self.align_right == Some(true) {
                cont.align_right(self.width)
            } else { cont };

        let cont = 
            if self.align_top == Some(true) {
                cont.align_top(self.height)
            } else { cont };

        let cont = 
            if self.align_botton == Some(true) {
                cont.align_bottom(self.height)
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
            IpgContainerStyleStd::BorderedBox => todo!(),
            IpgContainerStyleStd::Danger => {
                container::danger(theme)
            },
            IpgContainerStyleStd::Dark => todo!(),
            IpgContainerStyleStd::Primary => {
                container::primary(theme)
            },
            IpgContainerStyleStd::RoundedBox => todo!(),
            IpgContainerStyleStd::Secondary => {
                container::secondary(theme)
            },
            IpgContainerStyleStd::Success => {
                container::success(theme)
            },
            IpgContainerStyleStd::Warning => {
                container::warning(theme)
            },
            IpgContainerStyleStd::Transparent => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgContainerParam {
    AlignBotton,
    AlignLeft,
    AlignRight,
    AlignTop,
    AlignX,
    AlignY,
    Center,
    CenterX,
    CenterY,
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

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgContainerParam::AlignBotton => set_opt_bool(&mut self.align_botton, value, name),
            IpgContainerParam::AlignLeft => set_opt_bool(&mut self.align_left, value, name),
            IpgContainerParam::AlignRight => set_opt_bool(&mut self.align_right, value, name),
            IpgContainerParam::AlignTop => set_opt_bool(&mut self.align_top, value, name),
            IpgContainerParam::AlignX => set_halign(&mut self.align_x, value, name),
            IpgContainerParam::AlignY => set_valign(&mut self.align_y, value, name),
            IpgContainerParam::Center => set_opt_bool(&mut self.center, value, name),
            IpgContainerParam::CenterX => set_opt_bool(&mut self.center_x, value, name),
            IpgContainerParam::CenterY => set_opt_bool(&mut self.center_y, value, name),
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

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
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
