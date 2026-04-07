//! Button widget definition
use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::bootstrap_arrow::Arrow;
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback;
use crate::py_api::helpers::get_padding;
use crate::widgets::styling::{apply_shadow_overrides_xy, 
    apply_border_overrides, apply_background_overrides, 
    get_custom_palette};
use crate::widgets::widget_param_update::{set_opt_f32_array_2, 
    set_opt_iced_color_from_rgba, set_opt_vec_f32_1_or_upto_4};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_height, 
    set_height_fill, set_opt_bool, 
    set_opt_f32, set_opt_iced_color, set_opt_string, set_opt_usize, 
    set_opt_vec_f32, set_width, set_width_fill
};

use iced::{Background, alignment};
use iced::border;
use iced::widget::{button, text};
use iced::{Element, Length, Theme};
use iced::theme::palette;

use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct Button {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub label: Option<String>,
    pub width: Length,
    pub height: Length,
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

        let btn: Element<'_, BtnMessage> =
            button(txt)
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
pub struct ButtonStyle {
    pub id: usize,
    pub background_color: Option<iced::Color>,
    pub background_gradient_color_stop: Option<iced::Color>,
    pub background_gradient_degrees: Option<f32>,
    pub background_gradient_radians: Option<f32>,
    pub border_color: Option<iced::Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<iced::Color>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,
    pub text_color: Option<iced::Color>,
}

impl ButtonStyle {
    /// Apply user-defined style overrides to an existing iced button::Style
    pub fn to_iced(
        &self, 
        theme: &Theme, 
        status: button::Status,
        std_style_opt: &Option<ButtonStyleStd>,
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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum ButtonParam {
    ArrowStyle,
    Clip,
    Height,
    HeightFill,
    IfMenuBtn,
    Label,
    Padding,
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
    name: &str,
    ) -> ButtonStyleStd {
    
    Python::attach(|py| {

        let res = 
            value.extract::<ButtonStyleStd>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for ButtonStyleStandard", name),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum ButtonStyleParam {
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

impl WidgetParamUpdate for Button {
    type Param = ButtonParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ButtonParam::ArrowStyle => {
                self.style_arrow = Arrow::extract(value);
            }
            ButtonParam::Clip => set_opt_bool(&mut self.clip, value, "ButtonParam::Clip"),
            ButtonParam::Height => set_height(&mut self.height, value, "ButtonParam::Height"),
            ButtonParam::HeightFill => set_height_fill(&mut self.height, value, "ButtonParam::HeightFill"),
            ButtonParam::IfMenuBtn => set_opt_bool(&mut self.if_menu_btn, value, "ButtonParam::IfMenuBtn"),
            ButtonParam::Label => set_opt_string(&mut self.label, value, "ButtonParam::Label"),
            ButtonParam::Padding => set_opt_vec_f32(&mut self.padding, value, "ButtonParam::Padding"),
            ButtonParam::Show => set_bool(&mut self.show, value, "Show"),
            ButtonParam::StyleId => set_opt_usize(&mut self.style_id, value, "ButtonParam::StyleId"),
            ButtonParam::StyleStandard => {
                self.style_std = Some(extract_button_style_standard(value, "ButtonParam::StyleStandard"));
            },
            ButtonParam::TextSize => set_opt_f32(&mut self.text_size, value, "ButtonParam::TextSize"),
            ButtonParam::Width => set_width(&mut self.width, value, "ButtonParam::Width"),
            ButtonParam::WidthFill => set_width_fill(&mut self.width, value, "ButtonParam::WidthFill"),
            ButtonParam::TextAlignBottomCenter => set_opt_bool(&mut self.text_bottom_center, value, "ButtonParam::TextAlignBottomCenter"),
            ButtonParam::TextAlignBottomLeft => set_opt_bool(&mut self.text_bottom_left, value, "ButtonParam::TextAlignBottomLeft"),
            ButtonParam::TextAlignBottomRight => set_opt_bool(&mut self.text_bottom_right, value, "ButtonParam::TextAlignBottomRight"),
            ButtonParam::TextAlignCenter => set_opt_bool(&mut self.text_center, value, "ButtonParam::TextAlignCenter"),
            ButtonParam::TextAlignCenterLeft => set_opt_bool(&mut self.text_center_left, value, "ButtonParam::TextAlignCenterLeft"),
            ButtonParam::TextAlignCenterRight => set_opt_bool(&mut self.text_center_right, value, "ButtonParam::TextAlignCenterRight"),
            ButtonParam::TextAlignTopCenter => set_opt_bool(&mut self.text_top_center, value, "ButtonParam::TextAlignTopCenter"),
            ButtonParam::TextAlignTopLeft => set_opt_bool(&mut self.text_top_left, value, "ButtonParam::TextAlignTopLeft"),
            ButtonParam::TextAlignTopRight => set_opt_bool(&mut self.text_top_right, value, "ButtonParam::TextAlignTopRight"),
        }
    }
}

impl WidgetParamUpdate for ButtonStyle {
    type Param = ButtonStyleParam;
    
    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ButtonStyleParam::BackgroundColor => 
                set_opt_iced_color(&mut self.background_color, value, "BackgroundColor"),
            ButtonStyleParam::BackgroundRbga => 
                set_opt_iced_color_from_rgba(&mut self.background_color, value, "BackgroundRbga"),
            ButtonStyleParam::BorderColor => 
                set_opt_iced_color(&mut self.border_color, value, "BorderColor"),
            ButtonStyleParam::BorderRgba => 
                set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgba"),
            ButtonStyleParam::BorderRadius => 
                set_opt_vec_f32_1_or_upto_4(&mut self.border_radius, value, "BorderRadius"),
            ButtonStyleParam::BorderWidth => 
                set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            ButtonStyleParam::ShadowColor => 
                set_opt_iced_color(&mut self.shadow_color, value, "ShadowColor"),
            ButtonStyleParam::ShadowRgba => 
                set_opt_iced_color_from_rgba(&mut self.shadow_color, value, "ShadowRgba"),
            ButtonStyleParam::ShadowOffsetXY => 
                set_opt_f32_array_2(&mut self.shadow_offset_xy, value, "ShadowOffsetXY"),
            ButtonStyleParam::ShadowBlurRadius => 
                set_opt_f32(&mut self.shadow_blur_radius, value, "ShadowBlurRadius"),
            ButtonStyleParam::TextColor => 
                set_opt_iced_color(&mut self.text_color, value, "TextColor"),
            ButtonStyleParam::TextRgba => 
                set_opt_iced_color_from_rgba(&mut self.text_color, value, "TextRgba"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::IntoPyObjectExt;

    fn make_button() -> Button {
        Button {
            id: 0,
            parent_id: String::new(),
            show: true,
            label: None,
            width: Length::Shrink,
            height: Length::Shrink,
            padding: None,
            text_top_left: None,
            text_top_center: None,
            text_top_right: None,
            text_center_left: None,
            text_center: None,
            text_center_right: None,
            text_bottom_left: None,
            text_bottom_center: None,
            text_bottom_right: None,
            text_size: None,
            if_menu_btn: None,
            clip: None,
            style_id: None,
            style_std: None,
            style_arrow: None,
        }
    }

    fn make_button_style() -> ButtonStyle {
        ButtonStyle::default()
    }

    fn py_obj<T>(val: T) -> PyObject
    where
        for<'py> T: pyo3::IntoPyObject<'py>,
    {
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::attach(|py| py.None())
    }

    // -----------------------------------------------------------------------
    // Button param tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_clip() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::Clip, &py_obj(true));
        assert_eq!(b.clip, Some(true));
    }

    #[test]
    fn test_height() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::Height, &py_obj(100.0f32));
        assert_eq!(b.height, Length::Fixed(100.0));
        b.param_update(ButtonParam::Height, &py_none());
        assert_eq!(b.height, Length::Shrink);
    }

    #[test]
    fn test_height_fill() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::HeightFill, &py_obj(true));
        assert_eq!(b.height, Length::Fill);
        b.param_update(ButtonParam::HeightFill, &py_obj(false));
        assert_eq!(b.height, Length::Shrink);
    }

    #[test]
    fn test_label() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::Label, &py_obj("Click"));
        assert_eq!(b.label, Some("Click".to_string()));
    }

    #[test]
    fn test_padding() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::Padding, &py_obj(vec![5.0f32, 10.0, 5.0, 10.0]));
        assert_eq!(b.padding, Some(vec![5.0, 10.0, 5.0, 10.0]));
    }

    #[test]
    fn test_show() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::Show, &py_obj(false));
        assert!(!b.show);
    }

    #[test]
    fn test_style_id() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::StyleId, &py_obj(42u64));
        assert_eq!(b.style_id, Some(42));
    }

    #[test]
    fn test_text_size() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::TextSize, &py_obj(16.0f32));
        assert_eq!(b.text_size, Some(16.0));
        b.param_update(ButtonParam::TextSize, &py_none());
        assert_eq!(b.text_size, None);
    }

    #[test]
    fn test_width() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::Width, &py_obj(200.0f32));
        assert_eq!(b.width, Length::Fixed(200.0));
        b.param_update(ButtonParam::Width, &py_none());
        assert_eq!(b.width, Length::Shrink);
    }

    #[test]
    fn test_width_fill() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::WidthFill, &py_obj(true));
        assert_eq!(b.width, Length::Fill);
        b.param_update(ButtonParam::WidthFill, &py_obj(false));
        assert_eq!(b.width, Length::Shrink);
    }

    #[test]
    fn test_text_align_bottom_center() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::TextAlignBottomCenter, &py_obj(true));
        assert_eq!(b.text_bottom_center, Some(true));
    }

    #[test]
    fn test_text_align_bottom_left() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::TextAlignBottomLeft, &py_obj(true));
        assert_eq!(b.text_bottom_left, Some(true));
    }

    #[test]
    fn test_text_align_bottom_right() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::TextAlignBottomRight, &py_obj(true));
        assert_eq!(b.text_bottom_right, Some(true));
    }

    #[test]
    fn test_text_align_center() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::TextAlignCenter, &py_obj(true));
        assert_eq!(b.text_center, Some(true));
    }

    #[test]
    fn test_text_align_center_left() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::TextAlignCenterLeft, &py_obj(true));
        assert_eq!(b.text_center_left, Some(true));
    }

    #[test]
    fn test_text_align_top_center() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::TextAlignTopCenter, &py_obj(true));
        assert_eq!(b.text_top_center, Some(true));
    }

    #[test]
    fn test_text_align_top_left() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::TextAlignTopLeft, &py_obj(true));
        assert_eq!(b.text_top_left, Some(true));
    }

    #[test]
    fn test_text_align_top_right() {
        Python::initialize();
        let mut b = make_button();
        b.param_update(ButtonParam::TextAlignTopRight, &py_obj(true));
        assert_eq!(b.text_top_right, Some(true));
    }

    // -----------------------------------------------------------------------
    // ButtonStyle param tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_style_background_color() {
        Python::initialize();
        let mut s = make_button_style();
        let rgba = vec![1.0f32, 0.0, 0.0, 1.0];
        s.param_update(ButtonStyleParam::BackgroundRbga, &py_obj(rgba));
        assert!(s.background_color.is_some());
        s.param_update(ButtonStyleParam::BackgroundRbga, &py_none());
        assert!(s.background_color.is_none());
    }

    #[test]
    fn test_style_border_color() {
        Python::initialize();
        let mut s = make_button_style();
        let rgba = vec![0.0f32, 1.0, 0.0, 1.0];
        s.param_update(ButtonStyleParam::BorderRgba, &py_obj(rgba));
        assert!(s.border_color.is_some());
        s.param_update(ButtonStyleParam::BorderRgba, &py_none());
        assert!(s.border_color.is_none());
    }

    #[test]
    #[should_panic]
    fn test_style_border_rgba_bad_len() {
        Python::initialize();
        let mut s = make_button_style();
        let rgba = vec![0.0f32, 1.0, 0.0];
        s.param_update(ButtonStyleParam::BorderRgba, &py_obj(rgba));
    }

    #[test]
    fn test_style_border_radius() {
        Python::initialize();
        let mut s = make_button_style();
        s.param_update(ButtonStyleParam::BorderRadius, &py_obj(vec![5.0f32; 4]));
        assert_eq!(s.border_radius, Some(vec![5.0, 5.0, 5.0, 5.0]));
        s.param_update(ButtonStyleParam::BorderRadius, &py_obj(vec![5.0f32]));
        assert_eq!(s.border_radius, Some(vec![5.0]));
    }

    #[test]
    #[should_panic]
    fn test_style_border_radius_bad_len() {
        Python::initialize();
        let mut s = make_button_style();
        s.param_update(ButtonStyleParam::BorderRadius, &py_obj(vec![5.0f32; 5]));
    }

    #[test]
    fn test_style_border_width() {
        Python::initialize();
        let mut s = make_button_style();
        s.param_update(ButtonStyleParam::BorderWidth, &py_obj(2.0f32));
        assert_eq!(s.border_width, Some(2.0));
        s.param_update(ButtonStyleParam::BorderWidth, &py_none());
        assert_eq!(s.border_width, None);
    }

    #[test]
    fn test_style_shadow_color() {
        Python::initialize();
        let mut s = make_button_style();
        let rgba = vec![0.0f32, 0.0, 0.0, 0.5];
        s.param_update(ButtonStyleParam::ShadowRgba, &py_obj(rgba));
        assert!(s.shadow_color.is_some());
        s.param_update(ButtonStyleParam::ShadowRgba, &py_none());
        assert!(s.shadow_color.is_none());
    }

    #[test]
    fn test_style_shadow_offset_xy() {
        Python::initialize();
        let mut s = make_button_style();
        s.param_update(ButtonStyleParam::ShadowOffsetXY, &py_obj(vec![2.0f32, 3.0]));
        assert_eq!(s.shadow_offset_xy, Some([2.0, 3.0]));
    }

    #[test]
    fn test_style_shadow_blur_radius() {
        Python::initialize();
        let mut s = make_button_style();
        s.param_update(ButtonStyleParam::ShadowBlurRadius, &py_obj(4.0f32));
        assert_eq!(s.shadow_blur_radius, Some(4.0));
        s.param_update(ButtonStyleParam::ShadowBlurRadius, &py_none());
        assert_eq!(s.shadow_blur_radius, None);
    }

    #[test]
    fn test_style_text_color() {
        Python::initialize();
        let mut s = make_button_style();
        let rgba = vec![1.0f32, 1.0, 1.0, 1.0];
        s.param_update(ButtonStyleParam::TextRgba, &py_obj(rgba));
        assert!(s.text_color.is_some());
        s.param_update(ButtonStyleParam::TextRgba, &py_none());
        assert!(s.text_color.is_none());
    }
}
