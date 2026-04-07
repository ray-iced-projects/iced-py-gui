//! ipg_toggler

use iced::advanced::text;
use iced::widget;
use iced::{Element, Length, Theme};
use iced::theme::palette::{deviate, mix};

use pyo3::{pyclass, Py, PyAny};

use crate::app::Message;
use crate::py_api::helpers::get_radius;
use crate::state::Widgets;
use crate::widgets::ipg_text::TextWrapping;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool, 
    set_opt_bool, set_opt_f32, set_opt_iced_color, set_opt_iced_color_from_rgba, 
    set_opt_string, set_opt_text_shaping, set_opt_text_wrapping, set_opt_usize, 
    set_opt_vec_f32, set_width, set_width_fill};
use crate::IpgState;
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::ipg_text::TextShaping;
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Toggler {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub is_toggled: bool,
    pub label: Option<String>,
    pub width: Length,
    pub size: Option<f32>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub text_center: Option<bool>,
    pub text_left: Option<bool>,
    pub text_right: Option<bool>,
    pub text_shaping: Option<TextShaping>,
    pub text_wrapping: Option<TextWrapping>,
    pub spacing: Option<f32>,
    pub font_id: Option<usize>,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum TOGMessage {
    Toggled(bool),
}

pub fn construct_toggler<'a>(
    ipg_tog: &'a Toggler, 
    widget: Option<&Widgets>,
    font_opt:  Option<&'a Widgets>) 
    -> Option<Element<'a, Message>> {
    
    if !ipg_tog.show {
        return None
    }

    let style_opt = 
        widget.and_then(Widgets::as_toggler_style).cloned();

    let mut tog =  
        widget::Toggler::new(ipg_tog.is_toggled)
            .on_toggle(TOGMessage::Toggled)
            .width(ipg_tog.width)
            .style(move|theme: &Theme, status| {
                if let Some(st) = &style_opt {
                    st.set_style(theme, status)
                } else {
                    widget::toggler::default(theme, status)
                } 
            });

    if let Some(lb)  = &ipg_tog.label {
        tog = tog.label(lb);
    }

    if let Some(sz) = ipg_tog.size {
        tog = tog.size(sz);
    }

    if ipg_tog.text_center == Some(true){
        tog = tog.text_alignment(text::Alignment::Center);
    }

    if ipg_tog.text_left == Some(true){
        tog = tog.text_alignment(text::Alignment::Left);
    }

    if ipg_tog.text_right == Some(true){
        tog = tog.text_alignment(text::Alignment::Right);
    }

    if let Some(lh) = ipg_tog.text_line_height {
        tog = tog.text_line_height(lh);
    }

    if let Some(ts) = ipg_tog.text_size {
        tog = tog.text_size(ts);
    }

    if let Some(tw) = ipg_tog.text_wrapping {
        tog = tog.text_wrapping(tw.to_iced());
    }

    if let Some(sp) = ipg_tog.spacing {
        tog = tog.spacing(sp);
    }

    if let Some(wd) = font_opt {
        match wd {
            Widgets::Font(font) => {
                tog = tog.font(font.to_iced());
            },
            _ => ()
        }
    }

    let tog: Element<'_, TOGMessage> = tog.into();
    Some(tog.map(move |message| Message::Toggler(ipg_tog.id, message)))

}


pub fn toggle_callback(state: &mut IpgState, id: usize, message: TOGMessage) {
    match message {
        TOGMessage::Toggled(is_toggled) => {
            // Update widget state directly
            if let Some(Widgets::Toggler(tog)) = state.widgets.get_mut(&id) {
                tog.is_toggled = is_toggled;
            }
            invoke_callback_with_args(id, "toggled", "Toggler", is_toggled);
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum TogglerParam {
    FontId,
    Label,
    Show,
    Size,
    Spacing,
    StyleId,
    TextCenter,
    TextLeft,
    TextRight,
    TextLineHeight,
    TextShaping,
    TextSize,
    TextWrapping,
    Width,
    WidthFill,
}

#[derive(Debug, Clone)]
pub struct TogglerStyle {
    pub id: usize,
    pub background_color: Option<iced::Color>,
    pub background_border_color: Option<iced::Color>,
    pub background_border_width: Option<f32>,

    pub foreground_color: Option<iced::Color>,
    pub foreground_border_color: Option<iced::Color>,
    pub foreground_border_width: Option<f32>,
    
    pub text_color: Option<iced::Color>,
    pub border_radius: Option<Vec<f32>>,
    pub padding_ratio: Option<f32>,
}

//      Status	            Background	       Foreground
// Active + toggled	        bg_base	            fg_base
// Active + untoggled	    deviate(bg, 0.15)	deviate(fg, 0.15)
// Hovered + toggled	    bg_base	            fg_base at 50% alpha
// Hovered + untoggled	    deviate(bg, 0.15)	deviate(fg, 0.1)
// Disabled + toggled	    deviate(bg, 0.1)	mix(fg, bg, 0.4)
// Disabled + untoggled	    deviate(bg, 0.2)	mix(fg, bg, 0.4)
impl TogglerStyle {
    fn set_style(
        &self, 
        theme: &Theme, 
        status: widget::toggler::Status, 
        ) -> widget::toggler::Style {
        
        // Use user-supplied base colors or fall back to theme palette colors
        let palette = theme.extended_palette();

        let bg_base = self.background_color
            .unwrap_or(palette.primary.base.color);
        let fg_base = self.foreground_color
            .unwrap_or(palette.primary.base.text);

        // Derive variants: deviate auto-lightens dark colors and darkens light ones
        let bg_untoggled = deviate(bg_base, 0.15);      // like "strong"
        let bg_disabled_on = deviate(bg_base, 0.1);
        let bg_disabled_off = deviate(bg_base, 0.2);

        let background = match status {
            widget::toggler::Status::Active { is_toggled } | widget::toggler::Status::Hovered { is_toggled } => {
                if is_toggled { bg_base } else { bg_untoggled }
            }
            widget::toggler::Status::Disabled { is_toggled } => {
                if is_toggled { bg_disabled_on } else { bg_disabled_off }
            }
        };

        let fg_untoggled = deviate(fg_base, 0.15);
        let fg_hovered_on = iced::Color { a: 0.5, ..fg_base };
        let fg_hovered_off = deviate(fg_base, 0.1);
        let fg_disabled = mix(fg_base, background, 0.4);

        let foreground = match status {
            widget::toggler::Status::Active { is_toggled } => {
                if is_toggled { fg_base } else { fg_untoggled }
            }
            widget::toggler::Status::Hovered { is_toggled } => {
                if is_toggled { fg_hovered_on } else { fg_hovered_off }
            }
            widget::toggler::Status::Disabled { .. } => fg_disabled,
        };

        let border_radius = if let Some(br) = &self.border_radius {
            Some(get_radius(&br, "toggle".to_string()))
        } else { None };

        widget::toggler::Style {
            background: background.into(),
            foreground: foreground.into(),
            foreground_border_width: self.foreground_border_width.unwrap_or(0.0),
            foreground_border_color: self.foreground_border_color.unwrap_or(iced::Color::TRANSPARENT),
            background_border_width: self.background_border_width.unwrap_or(0.0),
            background_border_color: iced::Color::TRANSPARENT,
            text_color: self.text_color,
            border_radius,
            padding_ratio: self.padding_ratio.unwrap_or(0.1),
        }

    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum TogglerStyleParam {
    BackgroundColor,
    BackgroundRgbaColor,
    BackgroundBorderColor,
    BackgroundBorderRgbaColor,
    BackgroundBorderWidth,
    ForegroundColor,
    ForegroundRgbaColor,
    ForegroundBorderColor,
    ForegroundBorderRgbaColor,
    ForegroundBorderWidth,
    TextColor,
    TextRgbaColor,
    BorderRadius,
    PaddingRatio,
}



// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Toggler {
    type Param = TogglerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TogglerParam::FontId => set_opt_usize(&mut self.font_id, value, "FontId"),
            TogglerParam::Label => set_opt_string(&mut self.label, value, "Label"),
            TogglerParam::Show => set_bool(&mut self.show, value, "Show"),
            TogglerParam::Size => set_opt_f32(&mut self.size, value, "Size"),
            TogglerParam::Spacing => set_opt_f32(&mut self.spacing, value, "Spacing"),
            TogglerParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            TogglerParam::TextCenter => set_opt_bool(&mut self.text_center, value, "TextCenter"),
            TogglerParam::TextLeft => set_opt_bool(&mut self.text_left, value, "TextLeft"),
            TogglerParam::TextRight => set_opt_bool(&mut self.text_right, value, "TextRight"),
            TogglerParam::TextLineHeight => set_opt_f32(&mut self.text_line_height, value, "TextLineHeight"),
            TogglerParam::TextShaping => set_opt_text_shaping(&mut self.text_shaping, value, "TextShaping"),
            TogglerParam::TextSize => set_opt_f32(&mut self.text_size, value, "TextSize"),
            TogglerParam::TextWrapping => set_opt_text_wrapping(&mut self.text_wrapping, value, "TextWrapping"),
            TogglerParam::Width => set_width(&mut self.width, value, "Width"),
            TogglerParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
        }
    }
}

impl WidgetParamUpdate for TogglerStyle {
    type Param = TogglerStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TogglerStyleParam::BackgroundColor =>
                set_opt_iced_color(&mut self.background_color, value, "BackgroundColor"),
            TogglerStyleParam::BackgroundRgbaColor =>
                set_opt_iced_color_from_rgba(&mut self.background_color, value, "BackgroundRgbaColor"),
            TogglerStyleParam::BackgroundBorderColor =>
                set_opt_iced_color(&mut self.background_border_color, value, "BackgroundBorderColor"),
            TogglerStyleParam::BackgroundBorderRgbaColor =>
                set_opt_iced_color_from_rgba(&mut self.background_border_color, value, "BackgroundBorderRgbaColor"),
            TogglerStyleParam::BackgroundBorderWidth =>
                set_opt_f32(&mut self.background_border_width, value, "BackgroundBorderWidth"),
            TogglerStyleParam::ForegroundColor =>
                set_opt_iced_color(&mut self.foreground_color, value, "ForegroundColor"),
            TogglerStyleParam::ForegroundRgbaColor =>
                set_opt_iced_color_from_rgba(&mut self.foreground_color, value, "ForegroundRgbaColor"),
            TogglerStyleParam::ForegroundBorderColor =>
                set_opt_iced_color(&mut self.foreground_border_color, value, "ForegroundBorderColor"),
            TogglerStyleParam::ForegroundBorderRgbaColor =>
                set_opt_iced_color_from_rgba(&mut self.foreground_border_color, value, "ForegroundBorderRgbaColor"),
            TogglerStyleParam::ForegroundBorderWidth =>
                set_opt_f32(&mut self.foreground_border_width, value, "ForegroundBorderWidth"),
            TogglerStyleParam::TextColor =>
                set_opt_iced_color(&mut self.text_color, value, "TextColor"),
            TogglerStyleParam::TextRgbaColor =>
                set_opt_iced_color_from_rgba(&mut self.text_color, value, "TextRgbaColor"),
            TogglerStyleParam::BorderRadius =>
                set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
            TogglerStyleParam::PaddingRatio =>
                set_opt_f32(&mut self.padding_ratio, value, "PaddingRatio"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_toggler() -> Toggler {
        Toggler {
            id: 0,
            parent_id: String::new(),
            show: true,
            is_toggled: false,
            label: None,
            width: Length::Shrink,
            size: None,
            text_size: None,
            text_line_height: None,
            text_center: None,
            text_left: None,
            text_right: None,
            text_shaping: None,
            text_wrapping: None,
            spacing: None,
            font_id: None,
            style_id: None,
        }
    }

    fn make_toggler_style() -> TogglerStyle {
        TogglerStyle {
            id: 0,
            background_color: None,
            background_border_color: None,
            background_border_width: None,
            foreground_color: None,
            foreground_border_color: None,
            foreground_border_width: None,
            text_color: None,
            border_radius: None,
            padding_ratio: None,
        }
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::attach(|py| py.None().into_py_any(py).unwrap())
    }

    // -- Toggler param tests --

    #[test]
    fn test_font_id() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::FontId, &py_obj(2usize));
        assert_eq!(t.font_id, Some(2));
        t.param_update(TogglerParam::FontId, &py_none());
        assert_eq!(t.font_id, None);
    }

    #[test]
    fn test_label() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::Label, &py_obj("On/Off".to_string()));
        assert_eq!(t.label, Some("On/Off".to_string()));
        t.param_update(TogglerParam::Label, &py_none());
        assert_eq!(t.label, None);
    }

    #[test]
    fn test_show() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::Show, &py_obj(false));
        assert!(!t.show);
    }

    #[test]
    fn test_size() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::Size, &py_obj(24.0f32));
        assert_eq!(t.size, Some(24.0));
    }

    #[test]
    fn test_spacing() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::Spacing, &py_obj(10.0f32));
        assert_eq!(t.spacing, Some(10.0));
    }

    #[test]
    fn test_style_id() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::StyleId, &py_obj(5usize));
        assert_eq!(t.style_id, Some(5));
        t.param_update(TogglerParam::StyleId, &py_none());
        assert_eq!(t.style_id, None);
    }

    #[test]
    fn test_text_center() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::TextCenter, &py_obj(true));
        assert_eq!(t.text_center, Some(true));
        t.param_update(TogglerParam::TextCenter, &py_none());
        assert_eq!(t.text_center, None);
    }

    #[test]
    fn test_text_left() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::TextLeft, &py_obj(true));
        assert_eq!(t.text_left, Some(true));
    }

    #[test]
    fn test_text_right() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::TextRight, &py_obj(true));
        assert_eq!(t.text_right, Some(true));
    }

    #[test]
    fn test_text_line_height() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::TextLineHeight, &py_obj(1.5f32));
        assert_eq!(t.text_line_height, Some(1.5));
    }

    #[test]
    fn test_text_size() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::TextSize, &py_obj(14.0f32));
        assert_eq!(t.text_size, Some(14.0));
    }

    #[test]
    fn test_width() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::Width, &py_obj(200.0f32));
        assert_eq!(t.width, Length::Fixed(200.0));
    }

    #[test]
    fn test_width_fill() {
        let mut t = make_toggler();
        t.param_update(TogglerParam::WidthFill, &py_obj(true));
        assert_eq!(t.width, Length::Fill);
    }

    // -- TogglerStyle param tests --

    #[test]
    fn test_style_background_rgba() {
        let mut s = make_toggler_style();
        s.param_update(TogglerStyleParam::BackgroundRgbaColor, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.background_color.is_some());
    }

    #[test]
    fn test_style_background_border_rgba() {
        let mut s = make_toggler_style();
        s.param_update(TogglerStyleParam::BackgroundBorderRgbaColor, &py_obj(vec![0.0f32, 1.0, 0.0, 1.0]));
        assert!(s.background_border_color.is_some());
    }

    #[test]
    fn test_style_background_border_width() {
        let mut s = make_toggler_style();
        s.param_update(TogglerStyleParam::BackgroundBorderWidth, &py_obj(2.0f32));
        assert_eq!(s.background_border_width, Some(2.0));
    }

    #[test]
    fn test_style_foreground_rgba() {
        let mut s = make_toggler_style();
        s.param_update(TogglerStyleParam::ForegroundRgbaColor, &py_obj(vec![0.0f32, 0.0, 1.0, 1.0]));
        assert!(s.foreground_color.is_some());
    }

    #[test]
    fn test_style_foreground_border_rgba() {
        let mut s = make_toggler_style();
        s.param_update(TogglerStyleParam::ForegroundBorderRgbaColor, &py_obj(vec![1.0f32, 1.0, 0.0, 1.0]));
        assert!(s.foreground_border_color.is_some());
    }

    #[test]
    fn test_style_foreground_border_width() {
        let mut s = make_toggler_style();
        s.param_update(TogglerStyleParam::ForegroundBorderWidth, &py_obj(1.5f32));
        assert_eq!(s.foreground_border_width, Some(1.5));
    }

    #[test]
    fn test_style_text_rgba() {
        let mut s = make_toggler_style();
        s.param_update(TogglerStyleParam::TextRgbaColor, &py_obj(vec![0.0f32, 0.0, 0.0, 1.0]));
        assert!(s.text_color.is_some());
    }

    #[test]
    fn test_style_border_radius() {
        let mut s = make_toggler_style();
        s.param_update(TogglerStyleParam::BorderRadius, &py_obj(vec![4.0f32, 4.0, 4.0, 4.0]));
        assert_eq!(s.border_radius, Some(vec![4.0, 4.0, 4.0, 4.0]));
        s.param_update(TogglerStyleParam::BorderRadius, &py_none());
        assert_eq!(s.border_radius, None);
    }

    #[test]
    fn test_style_padding_ratio() {
        let mut s = make_toggler_style();
        s.param_update(TogglerStyleParam::PaddingRatio, &py_obj(0.2f32));
        assert_eq!(s.padding_ratio, Some(0.2));
    }
}
