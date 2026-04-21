//! Button widget definition
use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::bootstrap_arrow::Arrow;
use crate::graphics::colors::Color;
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback;
use crate::py_api::helpers::{get_len, get_padding};
use crate::widgets::styling::{apply_shadow_overrides_xy, 
    apply_border_overrides, apply_background_color_overrides, 
    get_custom_palette};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value
};

use iced::{Background, alignment};
use iced::border;
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
                text(arrow).font(iced::Font::with_name("bootstrap-icons"))
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
                        st.to_iced(theme, status, &self.style_std)
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
    pub background_gradient_color_stop: Option<Color>,
    pub background_gradient_color_stop_alpha: Option<f32>,
    pub background_gradient_rgba_stop: Option<[f32; 4]>,
    pub background_gradient_degrees: Option<f32>,
    pub background_gradient_radians: Option<f32>,
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
}

impl ButtonStyle {
    /// Apply user-defined style overrides to an existing iced button::Style
    pub fn to_iced(
        &self, 
        theme: &Theme, 
        status: button::Status,
        std_style_opt: &Option<ButtonStyleStd>,
        ) -> button::Style{

        // convert the colors
        let bkg_color = 
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
        let bkg_grad_color_stop = 
            Color::rgba_ipg_color_to_iced(self.background_gradient_rgba_stop, &self.background_gradient_color_stop, self.background_gradient_color_stop_alpha);
        let bdr_color = 
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);
        let shd_color =
            Color::rgba_ipg_color_to_iced(self.shadow_rgba, &self.shadow_color, self.shadow_color_alpha);
        let txt_color = 
            Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha);
        
        // If the user supplied a background_color, build a custom palette style.
        // Otherwise, use the standard style (or default to primary).
        let mut style = if let Some(bkg) = bkg_color {
            let (palette, _text_color) = get_custom_palette(bkg);
            let base = styled(palette.primary.base);
            match status {
                button::Status::Active | button::Status::Pressed => base,
                button::Status::Hovered => button::Style {
                    background: Some(Background::Color(palette.primary.strong.color)),
                    ..base
                },
                button::Status::Disabled => disabled(base),
            }
        } else if let Some(std) = std_style_opt {
            std.to_iced(theme, status)
        } else {
            button::primary(theme, status)
        };

        // Apply remaining optional overrides
        apply_background_color_overrides(
            &mut style.background, bkg_color,
            bkg_grad_color_stop,
            self.background_gradient_degrees,
            self.background_gradient_radians,
        );

        apply_border_overrides(
            &mut style.border, bdr_color,
            &self.border_radius, self.border_width, "Button",
        );

        apply_shadow_overrides_xy(
            &mut style.shadow, shd_color, 
            self.shadow_offset_xy, self.shadow_blur_radius);
        
        if let Some(tc) = txt_color {
            style.text_color = tc; 
        }

        style

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
    BackgroundGradientColorStop,
    BackgroundGradientColorStopAlpha,
    BackgroundGradientRgbaStop,
    BackgroundGradientDegrees,
    BackgroundGradientRadians,
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
    BorderRadius, 
    BorderWidth,
    ShadowColor,
    ShadowColorAlpha,
    ShadowRgba,
    ShadowOffsetXY,
    ShadowBlurRadius,
    TextColor,
    TextColorAlpha,
    TextRgba,
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
            ButtonStyleParam::BackgroundGradientColorStop => set_t_value(&mut self.background_gradient_color_stop, value, "ButtonStyleParam::BackgroundGradientColorStop"),
            ButtonStyleParam::BackgroundGradientColorStopAlpha => set_t_value(&mut self.background_gradient_color_stop_alpha, value, "ButtonStyleParam::BackgroundGradientColorStopAlpha"),
            ButtonStyleParam::BackgroundGradientDegrees => set_t_value(&mut self.background_gradient_degrees, value, "ButtonStyleParam::BackgroundGradientDegrees"),
            ButtonStyleParam::BackgroundGradientRadians => set_t_value(&mut self.background_gradient_radians, value, "ButtonStyleParam::BackgroundGradientRadians"),
            ButtonStyleParam::BackgroundGradientRgbaStop => set_t_value(&mut self.background_gradient_rgba_stop, value, "ButtonStyleParam::BackgroundGradientRgbaStop"),
            ButtonStyleParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "ButtonStyleParam::BackgroundRbga"),
            ButtonStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "ButtonStyleParam::BorderColor"),
            ButtonStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "ButtonStyleParam::BorderColorAlpha"),
            ButtonStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "ButtonStyleParam::BorderRadius"),
            ButtonStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "ButtonStyleParam::BorderRgba"),
            ButtonStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "ButtonStyleParam::BorderWidth"),
            ButtonStyleParam::ShadowBlurRadius => set_t_value(&mut self.shadow_blur_radius, value, "ButtonStyleParam::ShadowBlurRadius"),
            ButtonStyleParam::ShadowColor => set_t_value(&mut self.shadow_color, value, "ButtonStyleParam::ShadowColor"),
            ButtonStyleParam::ShadowColorAlpha => set_t_value(&mut self.shadow_color_alpha, value, "ButtonStyleParam::ShadowColorAlpha"),
            ButtonStyleParam::ShadowOffsetXY => set_t_value(&mut self.shadow_offset_xy, value, "ButtonStyleParam::ShadowOffsetXY"),
            ButtonStyleParam::ShadowRgba => set_t_value(&mut self.shadow_rgba, value, "ButtonStyleParam::ShadowRgba"),
            ButtonStyleParam::TextColor => set_t_value(&mut self.text_color, value, "ButtonStyleParam::TextColor"),
            ButtonStyleParam::TextColorAlpha => set_t_value(&mut self.text_color_alpha, value, "ButtonStyleParam::TextColorAlpha"),
            ButtonStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "ButtonStyleParam::TextRgba"),
        }
    }
}
