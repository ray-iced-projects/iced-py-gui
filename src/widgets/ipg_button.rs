//! Button widget definition
use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::bootstrap_arrow::IpgArrow;
use crate::state::IpgWidgets;
use crate::widgets::callbacks::invoke_callback;
use crate::py_api::helpers::get_padding;
use crate::widgets::styling::{apply_shadow_overrides_xy, 
    apply_border_overrides, apply_background_overrides, 
    get_custom_palette};
use crate::widgets::widget_param_update::set_opt_f32_array_2;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_height, 
    set_height_fill, set_iced_color_from_rgba, set_opt_bool, 
    set_opt_f32, set_opt_iced_color, set_opt_string, set_opt_usize, 
    set_opt_vec_f32, set_width, set_width_fill
};

use iced::{Background, alignment};
use iced::border;
use iced::widget::{button, text, Button};
use iced::{Color, Element, Length, Theme};
use iced::theme::palette;

use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct IpgButton {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub label: Option<String>,
    pub width: Length,
    pub height: Length,
    pub padding: Option<Vec<f32>>,
    pub text_align_top_left: Option<bool>,
    pub text_align_top_center: Option<bool>,
    pub text_align_top_right: Option<bool>,
    pub text_align_center_left: Option<bool>,
    pub text_align_center: Option<bool>,
    pub text_align_center_right: Option<bool>,
    pub text_align_bottom_left: Option<bool>,
    pub text_align_bottom_center: Option<bool>,
    pub text_align_bottom_right: Option<bool>,
    pub text_size: Option<f32>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_std: Option<IpgButtonStyleStd>,
    pub style_arrow: Option<IpgArrow>,
}

impl IpgButton {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }
    
    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, IpgWidgets>,
        ) -> Option<Element<'a, Message>> {
        
        if !self.show { return None }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(IpgWidgets::as_button_style).cloned();

        let txt = 
            if let Some(sa) = self.style_arrow.clone() {
                let arrow = IpgArrow::to_string(&sa);
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
        // so center needs to be the first one tested
        let txt = 
                txt.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center);

        let txt = 
            if self.text_align_top_left == Some(true) {
                txt.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Top)
            } else { txt };

        let txt = 
            if self.text_align_top_center == Some(true) {
                txt.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Top)
            } else { txt };

        let txt = 
            if self.text_align_top_right == Some(true) {
                txt.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Top)
            } else { txt };
        
        let txt = 
            if self.text_align_center_left == Some(true) {
                txt.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Center)
            } else { txt };

        let txt = 
            if self.text_align_center_right == Some(true) {
                txt.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Center)
            } else { txt };

        let txt = 
            if self.text_align_bottom_left == Some(true) {
                txt.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Bottom)
            } else { txt };

        let txt = 
            if self.text_align_bottom_center == Some(true) {
                txt.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Bottom)
            } else { txt };

        let txt = 
            if self.text_align_bottom_right == Some(true) {
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

        let btn: Element<'_, BtnMessage>=
            Button::new(txt)
                .padding(get_padding(&self.padding))
                .on_press(BtnMessage::OnPress)
                .width(self.width)
                .height(self.height)
                .clip(self.clip.unwrap_or(false))
                .style(move |theme: &Theme, status| {
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, status, &self.style_std)
                    } else {
                       match &self.style_std {
                            Some(std) => std.to_iced(theme, status),
                            None => button::primary(theme, status),
                        }
                    }
                }
            ).into();

        Some(btn.map(move |message| Message::Button(self.id, message)))

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
pub struct IpgButtonStyle {
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
}

impl IpgButtonStyle {
    /// Apply user-defined style overrides to an existing iced button::Style
    pub fn to_iced(
        &self, 
        theme: &Theme, 
        status: button::Status,
        std_style_opt: &Option<IpgButtonStyleStd>,
        ) -> button::Style{
        
        // If the user supplied a background_color, build a custom palette style.
        // Otherwise, use the standard style (or default to primary).
        let mut style = if let Some(bkg) = self.background_color {
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
        apply_background_overrides(
            &mut style.background, self.background_color,
            self.background_gradient_color_stop,
            self.background_gradient_degrees,
            self.background_gradient_radians,
        );

        apply_border_overrides(
            &mut style.border, self.border_color,
            &self.border_radius, self.border_width, "Button",
        );

        apply_shadow_overrides_xy(
            &mut style.shadow, self.shadow_color, 
            self.shadow_offset_xy, self.shadow_blur_radius);
        
        if let Some(tc) = self.text_color {
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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonStyleStd {
    Background,
    Danger,
    Primary,
    Secondary,
    Subtle,
    Success,
    Warning,
    Text,
}

impl IpgButtonStyleStd {
    pub fn to_iced(
        &self,
        theme: &Theme, 
        status: button::Status, 
        ) -> button::Style {
        
        match self {
            IpgButtonStyleStd::Background => {
                button::background(theme, status)
            },
            IpgButtonStyleStd::Danger => {
                button::danger(theme, status)
            },
            IpgButtonStyleStd::Primary => {
                button::primary(theme, status)
            },
            IpgButtonStyleStd::Secondary => {
                button::secondary(theme, status)
            },
            IpgButtonStyleStd::Subtle => {
                button::subtle(theme, status)
            },
            IpgButtonStyleStd::Success => {
                button::success(theme, status)
            },
            IpgButtonStyleStd::Warning => {
                button::warning(theme, status)
            },
            IpgButtonStyleStd::Text => {
                button::text(theme, status)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonParam {
    ArrowStyle,
    Height,
    HeightFill,
    Label,
    Padding,
    Clip,
    Show,
    StyleId,
    StyleStandard,
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

pub fn extract_button_style_standard(
    value: &PyObject, 
    name: String,
    ) -> IpgButtonStyleStd {
    
    Python::attach(|py| {

        let res = 
            value.extract::<IpgButtonStyleStd>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for ButtonStyleStandard", name),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonStyleParam {
    BackgroundColor,
    BackgroundRbga,
    BorderColor,
    BorderRgba,
    BorderRadius,
    BorderWidth,
    ShadowColor,
    ShadowRgba,
    ShadowOffsetXY,
    ShadowBlurRadius,
    TextColor,
    TextRgba,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgButton {
    type Param = IpgButtonParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        let name= String::new();
        match param {
            IpgButtonParam::ArrowStyle => {
                self.style_arrow = IpgArrow::extract(value);
            }
            IpgButtonParam::Clip => set_opt_bool(&mut self.clip, value, name),
            IpgButtonParam::Height => set_height(&mut self.height, value, name),
            IpgButtonParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgButtonParam::Label => set_opt_string(&mut self.label, value, "Label"),
            IpgButtonParam::Padding => set_opt_vec_f32(&mut self.padding, value, name),
            IpgButtonParam::Show => set_bool(&mut self.show, value, name),
            IpgButtonParam::StyleId => set_opt_usize(&mut self.style_id, value, name),
            IpgButtonParam::StyleStandard => {
                self.style_std = Some(extract_button_style_standard(value, name));
            },
            IpgButtonParam::TextSize => set_opt_f32(&mut self.text_size, value, name),
            IpgButtonParam::Width => set_width(&mut self.width, value, name),
            IpgButtonParam::WidthFill => set_width_fill(&mut self.width, value, name),
            IpgButtonParam::TextAlignBottomCenter => set_opt_bool(&mut self.text_align_bottom_center, value, name),
            IpgButtonParam::TextAlignBottomLeft => set_opt_bool(&mut self.text_align_bottom_left, value, name),
            IpgButtonParam::TextAlignBottomRight => set_opt_bool(&mut self.text_align_bottom_right, value, name),
            IpgButtonParam::TextAlignCenter => set_opt_bool(&mut self.text_align_center, value, name),
            IpgButtonParam::TextAlignCenterLeft => set_opt_bool(&mut self.text_align_center_left, value, name),
            IpgButtonParam::TextAlignCenterRight => set_opt_bool(&mut self.text_align_bottom_right, value, name),
            IpgButtonParam::TextAlignTopCenter => set_opt_bool(&mut self.text_align_top_center, value, name),
            IpgButtonParam::TextAlignTopLeft => set_opt_bool(&mut self.text_align_top_left, value, name),
            IpgButtonParam::TextAlignTopRight => set_opt_bool(&mut self.text_align_top_right, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgButtonStyle {
    type Param = IpgButtonStyleParam;
    
    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        let name = String::new();
        match param {
            IpgButtonStyleParam::BackgroundColor => 
                set_opt_iced_color(&mut self.background_color, value, name),
            IpgButtonStyleParam::BackgroundRbga => 
                set_iced_color_from_rgba(&mut self.background_color, value, name),
            IpgButtonStyleParam::BorderColor => 
                set_opt_iced_color(&mut self.border_color, value, name),
            IpgButtonStyleParam::BorderRgba => 
                set_iced_color_from_rgba(&mut self.border_color, value, name),
            IpgButtonStyleParam::BorderRadius => 
                set_opt_vec_f32(&mut self.border_radius, value, name),
            IpgButtonStyleParam::BorderWidth => 
                set_opt_f32(&mut self.border_width, value, name),
            IpgButtonStyleParam::ShadowColor => 
                set_opt_iced_color(&mut self.shadow_color, value, name),
            IpgButtonStyleParam::ShadowRgba => 
                set_iced_color_from_rgba(&mut self.shadow_color, value, name),
            IpgButtonStyleParam::ShadowOffsetXY => 
                set_opt_f32_array_2(&mut self.shadow_offset_xy, value, name),
            IpgButtonStyleParam::ShadowBlurRadius => 
                set_opt_f32(&mut self.shadow_blur_radius, value, name),
            IpgButtonStyleParam::TextColor => 
                set_opt_iced_color(&mut self.text_color, value, name),
            IpgButtonStyleParam::TextRgba => 
                set_iced_color_from_rgba(&mut self.text_color, value, name),
        }
    }
}
