//! Button widget definition
use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::bootstrap_arrow::Arrow;
use crate::graphics::colors::Color;
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback;
use crate::py_api::helpers::{get_len, get_padding, get_radius};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value
};

use iced::{Border, Shadow, Vector, alignment, gradient};
use iced::border::{self, Radius};
use iced::widget::{button, text};
use iced::{Element, Theme};
use iced::theme::palette;

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Button {
    pub id: usize,
    pub show: bool,
    pub label: Option<String>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub padding: Option<Vec<f32>>,
    pub text_top_left: Option<bool>,
    pub text_top_center: Option<bool>,
    pub text_top_right: Option<bool>,
    pub text_center_left: Option<bool>,
    pub text_center: Option<bool>,
    pub text_center_right: Option<bool>,
    pub text_bottom_left: Option<bool>,
    pub text_bottom_center: Option<bool>,
    pub text_bottom_right: Option<bool>,
    pub text_size: Option<f32>,
    pub if_menu_btn: Option<bool>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_std: Option<ButtonStyleStd>,
    pub style_arrow: Option<Arrow>,
}

impl Button {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }
    
    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>,
        ) -> Option<Element<'a, Message>> {
        
        if !self.show { return None }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_button_style).cloned();

        let txt = 
            if let Some(sa) = self.style_arrow.clone() {
                let arrow = Arrow::to_string(&sa);
                text(arrow).font(iced::Font::new("bootstrap-icons"))
            } else {
                let label = if let Some(lb) = &self.label {
                    lb.clone()
                } else {
                    String::new()
                };
                text(label.clone())
            };

        // Center is the default but is overridden by any other alignment
        // so center needs to be set first
        let txt = 
            txt.align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center);

        let txt = if self.if_menu_btn == Some(true) {
            txt.align_x(alignment::Horizontal::Left)
                .align_y(alignment::Vertical::Center)
        } else { txt };

        let txt = 
            if self.text_top_left == Some(true) {
                txt.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Top)
            } else { txt };

        let txt = 
            if self.text_top_center == Some(true) {
                txt.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Top)
            } else { txt };

        let txt = 
            if self.text_top_right == Some(true) {
                txt.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Top)
            } else { txt };
        
        let txt = 
            if self.text_center_left == Some(true) {
                txt.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Center)
            } else { txt };

        let txt = 
            if self.text_center_right == Some(true) {
                txt.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Center)
            } else { txt };

        let txt = 
            if self.text_bottom_left == Some(true) {
                txt.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Bottom)
            } else { txt };

        let txt = 
            if self.text_bottom_center == Some(true) {
                txt.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Bottom)
            } else { txt };

        let txt = 
            if self.text_bottom_right == Some(true) {
                txt.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Bottom)
            } else { txt };
        
        let txt = 
            if let Some(size) = self.text_size {
                txt.size(size)
            } else {txt};

        let txt = if self.clip == Some(true) {
            txt.wrapping(text::Wrapping::None)
        } else { txt };

        let btn =
            button(txt)
                .padding(get_padding(&self.padding))
                .on_press(Message::Button(self.id, BtnMessage::OnPress))
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height))
                .clip(self.clip.unwrap_or(false))
                .style(move |theme: &Theme, status| {
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, status)
                    } else {
                       match &self.style_std {
                            Some(std) => std.to_iced(theme, status),
                            None => {
                                if self.if_menu_btn == Some(true) {
                                    button::text(theme, status)
                                } else {
                                    button::primary(theme, status)
                                }
                            },
                        }
                    }
                }
            )
            .into();

        Some(btn)

    }
}

#[derive(Debug, Clone)]
pub enum BtnMessage {
    OnPress,
}

pub fn button_callback(id: usize, message: BtnMessage) {
    match message {
        BtnMessage::OnPress => {
            invoke_callback(id, "on_press", "Button");
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ButtonStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,

    pub text_color: Option<Color>,
    pub text_color_alpha: Option<f32>,
    pub text_rgba: Option<[f32; 4]>,

    pub text_color_active: Option<Color>,
    pub text_color_alpha_active: Option<f32>,
    pub text_rgba_active: Option<[f32; 4]>,

    pub text_color_hovered: Option<Color>,
    pub text_color_alpha_hovered: Option<f32>,
    pub text_rgba_hovered: Option<[f32; 4]>,

    pub text_color_pressed: Option<Color>,
    pub text_color_alpha_pressed: Option<f32>,
    pub text_rgba_pressed: Option<[f32; 4]>,

    pub text_color_disabled: Option<Color>,
    pub text_color_alpha_disabled: Option<f32>,
    pub text_rgba_disabled: Option<[f32; 4]>,
    
    pub gradient_color_stops: Option<Vec<Option<Color>>>,
    pub gradient_color_alpha_stops: Option<Vec<Option<f32>>>,
    pub gradient_rgba_stops: Option<Vec<Option<[f32; 4]>>>,
    pub gradient_offset_stops: Option<Vec<Option<f32>>>,
    pub gradient_degrees: Option<f32>,
    pub gradient_radians: Option<f32>,

    pub border_color_active: Option<Color>,
    pub border_color_alpha_active: Option<f32>,
    pub border_rgba_active: Option<[f32; 4]>,

    pub border_color_hovered: Option<Color>,
    pub border_color_alpha_hovered: Option<f32>,
    pub border_rgba_hovered: Option<[f32; 4]>,

    pub border_color_pressed: Option<Color>,
    pub border_color_alpha_pressed: Option<f32>,
    pub border_rgba_pressed: Option<[f32; 4]>,

    pub border_color_disabled: Option<Color>,
    pub border_color_alpha_disabled: Option<f32>,
    pub border_rgba_disabled: Option<[f32; 4]>,

    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    
    pub shadow_color: Option<Color>,
    pub shadow_color_alpha: Option<f32>,
    pub shadow_rgba: Option<[f32; 4]>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,

    pub snap: Option<bool>,
}

impl ButtonStyle {
    /// Apply user-defined style overrides to an existing iced button::Style
    pub fn to_iced(
        &self, 
        theme: &Theme, 
        status: button::Status,
        ) -> button::Style {

        // custom style requires bkg_color, text_color, and primary_color
        let background_color = 
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
        
        let text_color = 
            Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha);
        
        let text_color_active = 
            Color::rgba_ipg_color_to_iced(self.text_rgba_active, &self.text_color_active, self.text_color_alpha_active);
        let text_color_hovered = 
            Color::rgba_ipg_color_to_iced(self.text_rgba_hovered, &self.text_color_hovered, self.text_color_alpha_hovered);
        let text_color_pressed = 
            Color::rgba_ipg_color_to_iced(self.text_rgba_pressed, &self.text_color_pressed, self.text_color_alpha_pressed);
        let text_color_disabled = 
            Color::rgba_ipg_color_to_iced(self.text_rgba_disabled, &self.text_color_disabled, self.text_color_alpha_disabled);

        let bkg_grad_color_stops = 
            Color::gradient_stops_to_iced(&self.gradient_rgba_stops, &self.gradient_color_stops, &self.gradient_color_alpha_stops, self.gradient_offset_stops.clone());

        let grad_radians = if let Some(rad) = self.gradient_radians {
                rad
            } else if let Some(deg) = self.gradient_degrees {
                deg.to_radians()
            } else { 0.0 };
        
        let border_color_active = 
            Color::rgba_ipg_color_to_iced(self.border_rgba_active, &self.border_color_active, self.border_color_alpha_active);
        let border_color_hovered = 
            Color::rgba_ipg_color_to_iced(self.border_rgba_hovered, &self.border_color_hovered, self.border_color_alpha_hovered);
        let border_color_pressed = 
            Color::rgba_ipg_color_to_iced(self.border_rgba_pressed, &self.border_color_pressed, self.border_color_alpha_pressed);
        let border_color_disabled = 
            Color::rgba_ipg_color_to_iced(self.border_rgba_disabled, &self.border_color_disabled, self.border_color_alpha_disabled);
    
        let shd_color =
            Color::rgba_ipg_color_to_iced(self.shadow_rgba, &self.shadow_color, self.shadow_color_alpha);
        
        
        let palette = theme.palette();

        // One can use the theme text color but the background and primary 
        // are needed together to produce the correct colors
        let txt_color = if let Some(c) = text_color {
            c
        } else { theme.palette().background.base.text};

        let background_opt = if let Some(bkg) = background_color {
            Some(palette::Background::new(bkg, txt_color))
        } else { None };


        let bkg = background_opt.unwrap_or(palette.background);
        let bkg_color = 
            match status {
                button::Status::Active => bkg.base.color,
                button::Status::Hovered => bkg.weaker.color,
                button::Status::Pressed => bkg.strong.color,
                button::Status::Disabled => bkg.base.color.scale_alpha(0.5),
            };

        let linear = 
            if let Some(stops) = bkg_grad_color_stops {
                let linear = gradient::Linear::new(grad_radians);
                for stop in stops.iter() {
                    let bkg = palette::Background::new(stop.color, txt_color);
                    match status {
                        button::Status::Active =>  linear.add_stop(stop.offset, bkg.base.color),
                        button::Status::Hovered => linear.add_stop(stop.offset, bkg.weak.color),
                        button::Status::Pressed => linear.add_stop(stop.offset, bkg.strong.color),
                        button::Status::Disabled => linear.add_stop(stop.offset, bkg.base.color.scale_alpha(0.5)),
                    };
                }
                 Some(linear)
            } else { None };

        // Check to see if individual colors are defined.
        let text_color = 
            if background_opt.is_none() && text_color.is_none() {
                match status {
                    button::Status::Active => text_color_active.unwrap_or(txt_color),
                    button::Status::Hovered => text_color_hovered.unwrap_or(txt_color),
                    button::Status::Pressed => text_color_pressed.unwrap_or(txt_color),
                    button::Status::Disabled => text_color_disabled.unwrap_or(txt_color.scale_alpha(0.5)),
                }
            } else {
                if status == button::Status::Disabled {
                    txt_color.scale_alpha(0.5)
                } else {
                    txt_color
                }
            };


        // border color
        let bc_color = 
            if let Some(bkg) = background_opt {
                match status {
                    button::Status::Active => bkg.base.color,
                    button::Status::Hovered => bkg.weaker.color,
                    button::Status::Pressed => bkg.strong.color,
                    button::Status::Disabled => bkg.base.color.scale_alpha(0.5)
                }
            } else {
                match status {
                    button::Status::Active => border_color_active.unwrap_or(palette.background.base.color),
                    button::Status::Hovered => border_color_hovered.or(border_color_active).unwrap_or(palette.background.weaker.color),
                    button::Status::Pressed => border_color_pressed.or(border_color_active).unwrap_or(palette.background.strong.color),
                    button::Status::Disabled => border_color_disabled
                        .or(border_color_active.map(|c| c.scale_alpha(0.5)))
                        .unwrap_or(palette.background.base.color.scale_alpha(0.5)),
                }
            };

        let shadow = 
            if shd_color.is_some() && self.shadow_blur_radius.is_some() {
                let c = palette::Background::new(shd_color.unwrap(), txt_color);
                
                let offset = if let Some(of) = self.shadow_offset_xy {
                    Vector{ x: of[0], y: of[1] }
                } else {Vector::default()};

                let blur_radius = if let Some(br) = self.shadow_blur_radius {
                    br
                } else { 0.0 };
                let color = match status {
                    button::Status::Active => c.base.color,
                    button::Status::Hovered => c.weaker.color,
                    button::Status::Pressed => c.strong.color,
                    button::Status::Disabled => c.base.color.scale_alpha(0.5),
                };

                Shadow {
                    color,
                    offset,
                    blur_radius,
                }
                    
            } else { Shadow::default() };

        let radius = if let Some(rd) = &self.border_radius{
            get_radius(&rd, "button".to_string())
        } else { Radius::default() };

        let width = if let Some(w) = self.border_width {
            w
        } else { 0.0 };

        let snap = if let Some(sn) = self.snap {
            sn
        } else { false };

        
        let background = if let Some(lin) = linear {
            Some(iced::Background::Gradient(lin.into()))
        } else {
            Some(bkg_color.into())
        };

        match status {
            button::Status::Active => button::Style { 
                background, 
                text_color, 
                border: Border {
                    color: bc_color,
                    width,
                    radius,
                }, 
                shadow, 
                snap },
            button::Status::Hovered => button::Style { 
                background, 
                text_color, 
                border: Border {
                    color: bc_color,
                    width,
                    radius,
                }, 
                shadow,
                snap },
            button::Status::Pressed => button::Style { 
                background, 
                text_color,
                border: Border {
                    color: bc_color,
                    width,
                    radius,
                }, 
                shadow,  
                snap },
            button::Status::Disabled => button::Style { 
                background, 
                text_color, 
                border: Border {
                    color: bc_color,
                    width,
                    radius,
                }, 
                shadow, 
                snap },
        }

    }

}


fn styled(pair: palette::Pair) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(pair.color)),
        text_color: pair.text,
        border: border::rounded(2),
        ..button::Style::default()
    }
}

fn disabled(style: button::Style) -> button::Style {
    button::Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ButtonStyleStd {
    Background,
    Danger,
    Primary,
    Secondary,
    Subtle,
    Success,
    Warning,
    Text,
}

impl ButtonStyleStd {
    pub fn to_iced(
        &self,
        theme: &Theme, 
        status: button::Status, 
        ) -> button::Style {
        
        match self {
            ButtonStyleStd::Background => {
                button::background(theme, status)
            },
            ButtonStyleStd::Danger => {
                button::danger(theme, status)
            },
            ButtonStyleStd::Primary => {
                button::primary(theme, status)
            },
            ButtonStyleStd::Secondary => {
                button::secondary(theme, status)
            },
            ButtonStyleStd::Subtle => {
                button::subtle(theme, status)
            },
            ButtonStyleStd::Success => {
                button::success(theme, status)
            },
            ButtonStyleStd::Warning => {
                button::warning(theme, status)
            },
            ButtonStyleStd::Text => {
                button::text(theme, status)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ButtonParam {
    Clip,
    Fill,
    Height,
    HeightFill,
    IfMenuBtn,
    Label,
    Padding,
    Show,
    StyleArrow,
    StyleId,
    StyleStd,
    TextAlignBottomCenter,
    TextAlignBottomLeft,
    TextAlignBottomRight,
    TextAlignCenter,
    TextAlignCenterLeft,
    TextAlignCenterRight,
    TextAlignTopCenter,
    TextAlignTopLeft,
    TextAlignTopRight,
    TextSize,
    Width,
    WidthFill,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ButtonStyleParam {
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRgba,

    TextColor,
    TextColorAlpha,
    TextRgba,

    TextColorActive,
    TextColorAlphaActive,
    TextRgbaActive,

    TextColorHovered,
    TextColorAlphaHovered,
    TextRgbaHovered,

    TextColorPressed,
    TextColorAlphaPressed,
    TextRgbaPressed,

    TextColorDisabled,
    TextColorAlphaDisabled,
    TextRgbaDisabled,

    GradientColorStops,
    GradientColorAlphaStops,
    GradientRgbaStops,
    GradientDegrees,
    GradientRadians,

    BorderColorActive,
    BorderColorAlphaActive,
    BorderRgbaActive,

    BorderColorHovered,
    BorderColorAlphaHovered,
    BorderRgbaHovered,

    BorderColorPressed,
    BorderColorAlphaPressed,
    BorderRgbaPressed,

    BorderColorDisabled,
    BorderColorAlphaDisabled,
    BorderRgbaDisabled,

    BorderRadius,
    BorderWidth,

    ShadowColor,
    ShadowColorAlpha,
    ShadowRgba,
    ShadowOffsetXy,
    ShadowBlurRadius,

    Snap,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Button {
    type Param = ButtonParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ButtonParam::Clip => set_t_value(&mut self.clip, value, "ButtonParam::Clip"),
            ButtonParam::Height => set_t_value(&mut self.height, value, "ButtonParam::Height"),
            ButtonParam::HeightFill => set_t_value(&mut self.height, value, "ButtonParam::HeightFill"),
            ButtonParam::IfMenuBtn => set_t_value(&mut self.if_menu_btn, value, "ButtonParam::IfMenuBtn"),
            ButtonParam::Fill => set_t_value(&mut self.fill, value, "ButtonParam::Fill"),
            ButtonParam::Label => set_t_value(&mut self.label, value, "ButtonParam::Label"),
            ButtonParam::Padding => set_t_value(&mut self.padding, value, "ButtonParam::Padding"),
            ButtonParam::Show => set_t_value(&mut self.show, value, "Show"),
            ButtonParam::StyleArrow => set_t_value(&mut self.style_arrow, value, "ButtonParam::StyleArrow"),
            ButtonParam::StyleId => set_t_value(&mut self.style_id, value, "ButtonParam::StyleId"),
            ButtonParam::StyleStd => set_t_value(&mut self.style_std, value, "ButtonParam::StyleStd"),
            ButtonParam::TextAlignBottomCenter => set_t_value(&mut self.text_bottom_center, value, "ButtonParam::TextAlignBottomCenter"),
            ButtonParam::TextAlignBottomLeft => set_t_value(&mut self.text_bottom_left, value, "ButtonParam::TextAlignBottomLeft"),
            ButtonParam::TextAlignBottomRight => set_t_value(&mut self.text_bottom_right, value, "ButtonParam::TextAlignBottomRight"),
            ButtonParam::TextAlignCenter => set_t_value(&mut self.text_center, value, "ButtonParam::TextAlignCenter"),
            ButtonParam::TextAlignCenterLeft => set_t_value(&mut self.text_center_left, value, "ButtonParam::TextAlignCenterLeft"),
            ButtonParam::TextAlignCenterRight => set_t_value(&mut self.text_center_right, value, "ButtonParam::TextAlignCenterRight"),
            ButtonParam::TextAlignTopCenter => set_t_value(&mut self.text_top_center, value, "ButtonParam::TextAlignTopCenter"),
            ButtonParam::TextAlignTopLeft => set_t_value(&mut self.text_top_left, value, "ButtonParam::TextAlignTopLeft"),
            ButtonParam::TextAlignTopRight => set_t_value(&mut self.text_top_right, value, "ButtonParam::TextAlignTopRight"),
            ButtonParam::TextSize => set_t_value(&mut self.text_size, value, "ButtonParam::TextSize"),
            ButtonParam::Width => set_t_value(&mut self.width, value, "ButtonParam::Width"),
            ButtonParam::WidthFill => set_t_value(&mut self.width, value, "ButtonParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for ButtonStyle {
    type Param = ButtonStyleParam;
    
    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ButtonStyleParam::BackgroundColor => set_t_value(&mut self.background_color, value, "ButtonStyleParam::BackgroundColor"),
            ButtonStyleParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "ButtonStyleParam::BackgroundColorAlpha"),
            ButtonStyleParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "ButtonStyleParam::BackgroundRgba"),
            ButtonStyleParam::TextColor => set_t_value(&mut self.text_color, value, "ButtonStyleParam::TextColor"),
            ButtonStyleParam::TextColorAlpha => set_t_value(&mut self.text_color_alpha, value, "ButtonStyleParam::TextColorAlpha"),
            ButtonStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "ButtonStyleParam::TextRgba"),
            ButtonStyleParam::TextColorActive => set_t_value(&mut self.text_color_active, value, "ButtonStyleParam::TextColorActive"),
            ButtonStyleParam::TextColorAlphaActive => set_t_value(&mut self.text_color_alpha_active, value, "ButtonStyleParam::TextColorAlphaActive"),
            ButtonStyleParam::TextRgbaActive => set_t_value(&mut self.text_rgba_active, value, "ButtonStyleParam::TextRgbaActive"),
            ButtonStyleParam::TextColorHovered => set_t_value(&mut self.text_color_hovered, value, "ButtonStyleParam::TextColorHovered"),
            ButtonStyleParam::TextColorAlphaHovered => set_t_value(&mut self.text_color_alpha_hovered, value, "ButtonStyleParam::TextColorAlphaHovered"),
            ButtonStyleParam::TextRgbaHovered => set_t_value(&mut self.text_rgba_hovered, value, "ButtonStyleParam::TextRgbaHovered"),
            ButtonStyleParam::TextColorPressed => set_t_value(&mut self.text_color_pressed, value, "ButtonStyleParam::TextColorPressed"),
            ButtonStyleParam::TextColorAlphaPressed => set_t_value(&mut self.text_color_alpha_pressed, value, "ButtonStyleParam::TextColorAlphaPressed"),
            ButtonStyleParam::TextRgbaPressed => set_t_value(&mut self.text_rgba_pressed, value, "ButtonStyleParam::TextRgbaPressed"),
            ButtonStyleParam::TextColorDisabled => set_t_value(&mut self.text_color_disabled, value, "ButtonStyleParam::TextColorDisabled"),
            ButtonStyleParam::TextColorAlphaDisabled => set_t_value(&mut self.text_color_alpha_disabled, value, "ButtonStyleParam::TextColorAlphaDisabled"),
            ButtonStyleParam::TextRgbaDisabled => set_t_value(&mut self.text_rgba_disabled, value, "ButtonStyleParam::TextRgbaDisabled"),
            ButtonStyleParam::GradientColorStops => set_t_value(&mut self.gradient_color_stops, value, "ButtonStyleParam::GradientColorStops"),
            ButtonStyleParam::GradientColorAlphaStops => set_t_value(&mut self.gradient_color_alpha_stops, value, "ButtonStyleParam::GradientColorAlphaStops"),
            ButtonStyleParam::GradientRgbaStops => set_t_value(&mut self.gradient_rgba_stops, value, "ButtonStyleParam::GradientRgbaStops"),
            ButtonStyleParam::GradientDegrees => set_t_value(&mut self.gradient_degrees, value, "ButtonStyleParam::GradientDegrees"),
            ButtonStyleParam::GradientRadians => set_t_value(&mut self.gradient_radians, value, "ButtonStyleParam::GradientRadians"),
            ButtonStyleParam::BorderColorActive => set_t_value(&mut self.border_color_active, value, "ButtonStyleParam::BorderColorActive"),
            ButtonStyleParam::BorderColorAlphaActive => set_t_value(&mut self.border_color_alpha_active, value, "ButtonStyleParam::BorderColorAlphaActive"),
            ButtonStyleParam::BorderRgbaActive => set_t_value(&mut self.border_rgba_active, value, "ButtonStyleParam::BorderRgbaActive"),
            ButtonStyleParam::BorderColorHovered => set_t_value(&mut self.border_color_hovered, value, "ButtonStyleParam::BorderColorHovered"),
            ButtonStyleParam::BorderColorAlphaHovered => set_t_value(&mut self.border_color_alpha_hovered, value, "ButtonStyleParam::BorderColorAlphaHovered"),
            ButtonStyleParam::BorderRgbaHovered => set_t_value(&mut self.border_rgba_hovered, value, "ButtonStyleParam::BorderRgbaHovered"),
            ButtonStyleParam::BorderColorPressed => set_t_value(&mut self.border_color_pressed, value, "ButtonStyleParam::BorderColorPressed"),
            ButtonStyleParam::BorderColorAlphaPressed => set_t_value(&mut self.border_color_alpha_pressed, value, "ButtonStyleParam::BorderColorAlphaPressed"),
            ButtonStyleParam::BorderRgbaPressed => set_t_value(&mut self.border_rgba_pressed, value, "ButtonStyleParam::BorderRgbaPressed"),
            ButtonStyleParam::BorderColorDisabled => set_t_value(&mut self.border_color_disabled, value, "ButtonStyleParam::BorderColorDisabled"),
            ButtonStyleParam::BorderColorAlphaDisabled => set_t_value(&mut self.border_color_alpha_disabled, value, "ButtonStyleParam::BorderColorAlphaDisabled"),
            ButtonStyleParam::BorderRgbaDisabled => set_t_value(&mut self.border_rgba_disabled, value, "ButtonStyleParam::BorderRgbaDisabled"),
            ButtonStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "ButtonStyleParam::BorderRadius"),
            ButtonStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "ButtonStyleParam::BorderWidth"),
            ButtonStyleParam::ShadowColor => set_t_value(&mut self.shadow_color, value, "ButtonStyleParam::ShadowColor"),
            ButtonStyleParam::ShadowColorAlpha => set_t_value(&mut self.shadow_color_alpha, value, "ButtonStyleParam::ShadowColorAlpha"),
            ButtonStyleParam::ShadowRgba => set_t_value(&mut self.shadow_rgba, value, "ButtonStyleParam::ShadowRgba"),
            ButtonStyleParam::ShadowOffsetXy => set_t_value(&mut self.shadow_offset_xy, value, "ButtonStyleParam::ShadowOffsetXy"),
            ButtonStyleParam::ShadowBlurRadius => set_t_value(&mut self.shadow_blur_radius, value, "ButtonStyleParam::ShadowBlurRadius"),
            ButtonStyleParam::Snap => set_t_value(&mut self.snap, value, "ButtonStyleParam::Snap"),
        }
    }
}
