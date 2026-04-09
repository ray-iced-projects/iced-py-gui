//! ipg_checkbox

use std::collections::HashMap;

use crate::state::IpgState;
use crate::app::Message;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_opt_f32, set_opt_iced_color, set_opt_iced_color_from_rgba, set_opt_string, set_opt_usize, set_opt_vec_f32, set_t_value, set_width, set_width_fill
};
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::state::Widgets;

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap_icon::{Icon, icon_to_char};

use crate::widgets::styling::{apply_border_overrides, create_custom_theme};

use iced::advanced::text;
use iced::{Background, Element, Length, Theme};
use iced::widget::text::{LineHeight, Shaping, Wrapping};
use iced::widget::{Checkbox, checkbox};
use iced::theme::palette;

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct CheckBox {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub is_checked: bool,
    pub label: Option<String>,
    pub width: Length,
    pub size: Option<f32>,
    pub spacing: Option<f32>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub text_shaping_advanced: Option<bool>,
    pub text_shaping_basic: Option<bool>,
    pub text_wrapping_none: Option<bool>,
    pub text_wrapping_glyph: Option<bool>,
    pub text_wrapping_word_glyph: Option<bool>,
    pub text_font_id: Option<usize>,
    pub icon_font_id: Option<usize>,
    pub icon: Option<Icon>,
    pub icon_size: Option<f32>,
    pub icon_line_height: Option<f32>,
    pub icon_shaping_advanced: Option<bool>,
    pub icon_shaping_basic: Option<bool>,
    pub style_id: Option<usize>,
    pub style_std: Option<CheckboxStyleStd>,
}

impl CheckBox {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message>> {

        if !self.show {
            return None
        };

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_checkbox_style).cloned();

        // Icon related
        let code_point = 
            if let Some(ic) = self.icon {
                icon_to_char(ic)
            } else {
                icon_to_char(Icon::Check)
            };

        let size = 
            if let Some(sz) = self.icon_size {
                Some(iced::Pixels(sz))
            } else { None };

        let line_height = 
            if let Some(lh) = self.icon_line_height {
                LineHeight::Relative(lh)
            } else { LineHeight::default() };

        // default is auto so not checked
        let icon_s = 
            if self.icon_shaping_advanced.is_some() {
                Shaping::Advanced
            } else if self.icon_shaping_basic.is_some() {
                Shaping::Basic
            } else { Shaping::Auto };
        
        let icon = 
            checkbox::Icon {
                font: BOOTSTRAP_FONT,
                code_point,
                size,
                line_height,
                shaping: icon_s,
            };
        
        // Text related
        let text_line_height = 
            if let Some(lh) = self.text_line_height {
                text::LineHeight::Relative(lh)
            } else { text::LineHeight::default() };

        let chk = 
            Checkbox::new(self.is_checked)
                .on_toggle(ChkMessage::OnToggle)
                .width(self.width)
                .text_line_height(text_line_height)
                .icon(icon)
                .style(move|theme: &Theme, status| {   
                    if let Some(st) = &style_opt {
                        let is_label = if self.label.is_some() { true } else { false };
                        st.to_iced(theme, status, &self.style_std, is_label)
                    } else {
                       match &self.style_std {
                            Some(std) => std.to_iced(theme, status),
                            None => checkbox::primary(theme, status),
                        }
                    }
                }
            );
        
        let chk = 
            if let Some(lb) = &self.label {
                chk.label(lb.clone())
            } else { chk };

        let chk = 
            if let Some(sz) = self.size {
                chk.size(sz)
            } else { chk };

        let chk = 
            if let Some(sp) = self.spacing {
                chk.spacing(sp)
            } else { chk };

        // default is word so not checked
        let chk = 
            if self.text_wrapping_none.is_some() {
                chk.text_wrapping(Wrapping::None)
            } else if self.text_wrapping_glyph.is_some() {
                chk.text_wrapping(Wrapping::Glyph)
            } else if self.text_wrapping_word_glyph.is_some() {
                chk.text_wrapping(Wrapping::WordOrGlyph)
            } else { chk };

        // default is auto so not checked
        let chk = 
            if self.text_shaping_advanced.is_some() {
                chk.text_shaping(Shaping::Advanced)
            } else if self.text_shaping_basic.is_some() {
                chk.text_shaping(Shaping::Basic)
            } else { chk };

        let chk: Element<'_, ChkMessage> = chk.into();
        Some(chk.map(move |message| Message::CheckBox(self.id, message)))

    }
}

#[derive(Debug, Clone)]
pub enum ChkMessage {
    OnToggle(bool),
}

pub fn checkbox_callback(state: &mut IpgState, id: usize, message: ChkMessage) {

    match message {
        ChkMessage::OnToggle(is_checked) => {
            if let Some(Widgets::CheckBox(cb)) = state.widgets.get_mut(&id) {
                cb.is_checked = is_checked;
            }
            invoke_callback_with_args(id, "on_toggle", "Checkbox", is_checked);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CheckboxStyle {
    pub id: usize,
    pub background_color: Option<iced::Color>,
    pub border_color: Option<iced::Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub icon_color: Option<iced::Color>,
    pub text_color: Option<iced::Color>,
}

impl CheckboxStyle {
    fn to_iced(
         &self, 
        theme: &Theme, 
        status: checkbox::Status,
        std_style_opt: &Option<CheckboxStyleStd>,
        is_label: bool,
        ) -> checkbox::Style {

        let mut style = if let Some(std) = 
            std_style_opt {
                std.to_iced(theme, status)
            } else { checkbox::primary(theme, status) };
        
        if let Some(color) = self.icon_color {
                style.icon_color = color;
            };

        style.text_color = self.text_color;
        
        // custom style only depends on bkg and border color
        if self.background_color.is_none() && self.border_color.is_none() {
            return style
        }

        // if given a border_color then use border_color as is
        // if given a bkg_color then use bkg_color as is
        // the accent will be paired regardless

        let custom_theme;
        let text_color;

        let palette = if let Some(bkg) = self.background_color {
            let dark_mode = palette::is_dark(bkg);
            custom_theme = create_custom_theme(bkg, dark_mode);
            text_color = custom_theme.palette().text;
            custom_theme.extended_palette()
        } else {
            text_color = theme.palette().text;
            theme.extended_palette()
        };

        let mut style = match status {
            checkbox::Status::Active { is_checked } => styled(
                palette.background.strong.color,
                palette.background.base,
                palette.primary.base.text,
                palette.primary.base,
                is_checked,
            ),
            checkbox::Status::Hovered { is_checked } => styled(
                palette.background.strong.color,
                palette.background.weak,
                palette.primary.base.text,
                palette.primary.strong,
                is_checked,
            ),
            checkbox::Status::Disabled { is_checked } => styled(
                palette.background.weak.color,
                palette.background.weaker,
                palette.primary.base.text,
                palette.background.strong,
                is_checked,
            ),
        };

        apply_border_overrides(
            &mut style.border, self.border_color,
            &self.border_radius, self.border_width, "Checkbox",
        );

        if is_label && self.background_color.is_some() {
            let color = palette::readable(self.background_color.unwrap(), text_color);
            style.text_color = Some(color);
        }

        style
        
    }

}

fn styled(
    border_color: iced::Color,
    base: palette::Pair,
    icon_color: iced::Color,
    accent: palette::Pair,
    is_checked: bool,
) -> checkbox::Style {
    let (background, border) = if is_checked {
        (accent, accent.color)
    } else {
        (base, border_color)
    };

    checkbox::Style {
        background: Background::Color(background.color),
        icon_color,
        border: iced::Border {
            radius: 2.0.into(),
            width: 1.0,
            color: border,
        },
        text_color: None,
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum CheckboxStyleStd {
    Danger,
    Primary,
    Secondary,
    Success,
}

impl CheckboxStyleStd {
    pub fn to_iced (
        &self,
        theme: &Theme, 
        status: checkbox::Status, 
        ) -> checkbox::Style {
        
        match self {
            CheckboxStyleStd::Danger => {
                checkbox::danger(theme, status)
            },
            CheckboxStyleStd::Primary => {
                checkbox::primary(theme, status)
            },
            CheckboxStyleStd::Secondary => {
                checkbox::secondary(theme, status)
            },
            CheckboxStyleStd::Success => {
                checkbox::success(theme, status)
            },
        }
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum CheckboxParam {
    Icon,
    IconFont,
    IconLineHeight,
    IconSize,
    IconShapingAdvanced,
    IconShapingBasic,
    IsChecked,
    Label,
    Spacing,
    Style,
    StyleStandard,
    TextFont,
    TextLineHeight,
    TextShapingAdvanced,
    TextShapingBasic,
    TextWrappingNone,
    TextWrappingGlyph,
    TextWrappingWordGlyph,
    TextSize,
    Width,
    WidthFill,
    Show,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum CheckboxStyleParam {
    BackgroundColor,
    BackgroundRgbaColor,
    BorderColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    IconColor,
    IconRgbaColor,
    TextColor,
    TextRgbaColor,
}

fn extract_chk_style_standard(
    value: &PyObject, 
    ) -> CheckboxStyleStd {
    
    Python::attach(|py| {

        let res = 
            value.extract::<CheckboxStyleStd>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python object for CheckboxStyleStandard"),
        }
    })
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for CheckBox {
    type Param = CheckboxParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {

        match param {
            CheckboxParam::Icon => {
                self.icon = Some(Icon::extract(value));
            }
            CheckboxParam::IconFont     => { /* TODO */ }
            CheckboxParam::IconLineHeight => set_t_value(&mut self.icon_line_height, value, "CheckboxParam::IconLineHeight"),
            CheckboxParam::IconShapingAdvanced => set_t_value(&mut self.icon_shaping_advanced, value, "CheckboxParam::IconShapingAdvanced"),
            CheckboxParam::IconShapingBasic => set_t_value(&mut self.icon_shaping_basic, value, "CheckboxParam::IconShapingBasic"),
            CheckboxParam::IconSize     => set_opt_f32(&mut self.icon_size, value, "CheckboxParam::IconSize"),
            CheckboxParam::IsChecked    => set_bool(&mut self.is_checked, value, "CheckboxParam::IsChecked"),
            CheckboxParam::Label        => set_opt_string(&mut self.label, value, "CheckboxParam::Label"),
            CheckboxParam::Show         => set_bool(&mut self.show, value, "CheckboxParam::Show"),
            CheckboxParam::Spacing      => set_opt_f32(&mut self.spacing, value, "CheckboxParam::Spacing"),
            CheckboxParam::Style        => set_opt_usize(&mut self.style_id, value, "CheckboxParam::Style"),
            CheckboxParam::StyleStandard => self.style_std = Some(extract_chk_style_standard(value)),
            CheckboxParam::TextFont     => { /* TODO */ }
            CheckboxParam::TextLineHeight => set_opt_f32(&mut self.text_line_height, value, "CheckboxParam::TextLineHeight"),
            CheckboxParam::TextShapingAdvanced => set_t_value(&mut self.text_shaping_advanced, value, "CheckboxParam::TextShapingAdvanced"),
            CheckboxParam::TextShapingBasic => set_t_value(&mut self.text_shaping_basic, value, "CheckboxParam::TextShapingBasic"),
            CheckboxParam::TextSize     => set_opt_f32(&mut self.text_size, value, "CheckboxParam::TextSize"),
            CheckboxParam::TextWrappingGlyph => set_t_value(&mut self.text_wrapping_glyph, value, "CheckboxParam::TextWrappingGlyph"),
            CheckboxParam::TextWrappingNone => set_t_value(&mut self.text_wrapping_none, value, "CheckboxParam::TextWrappingNone"),
            CheckboxParam::TextWrappingWordGlyph => set_t_value(&mut self.text_wrapping_word_glyph, value, "CheckboxParam::TextWrappingWordGlyph"),
            CheckboxParam::Width        => set_width(&mut self.width, value, "CheckboxParam::Width"),
            CheckboxParam::WidthFill    => set_width_fill(&mut self.width, value, "CheckboxParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for CheckboxStyle {
    type Param = CheckboxStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            CheckboxStyleParam::BackgroundColor => set_opt_iced_color(&mut self.background_color, value, "BackgroundColor"),
            CheckboxStyleParam::BackgroundRgbaColor => set_opt_iced_color_from_rgba(&mut self.background_color, value, "BackgroundRgbaColor"),
            CheckboxStyleParam::BorderColor => set_opt_iced_color(&mut self.border_color, value, "BorderColor"),
            CheckboxStyleParam::BorderRgbaColor => set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgbaColor"),
            CheckboxStyleParam::BorderRadius => set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
            CheckboxStyleParam::BorderWidth => set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            CheckboxStyleParam::IconColor => set_opt_iced_color(&mut self.icon_color, value, "IconColor"),
            CheckboxStyleParam::IconRgbaColor => set_opt_iced_color_from_rgba(&mut self.icon_color, value, "IconRgbaColor"),
            CheckboxStyleParam::TextColor => set_opt_iced_color(&mut self.text_color, value, "TextColor"),
            CheckboxStyleParam::TextRgbaColor => set_opt_iced_color_from_rgba(&mut self.text_color, value, "TextRgbaColor"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::IntoPyObjectExt;

    fn make_checkbox() -> CheckBox {
        CheckBox {
            id: 0,
            parent_id: String::new(),
            show: true,
            is_checked: false,
            label: None,
            width: Length::Shrink,
            size: None,
            spacing: None,
            text_size: None,
            text_line_height: None,
            text_shaping_advanced: None,
            text_shaping_basic: None,
            text_wrapping_none: None,
            text_wrapping_glyph: None,
            text_wrapping_word_glyph: None,
            text_font_id: None,
            icon_font_id: None,
            icon: None,
            icon_size: None,
            icon_line_height: None,
            icon_shaping_advanced: None,
            icon_shaping_basic: None,
            style_id: None,
            style_std: None,
        }
    }

    fn make_checkbox_style() -> CheckboxStyle {
        CheckboxStyle::default()
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
    // CheckBox param tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_icon_line_height() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(CheckboxParam::IconLineHeight, &py_obj(2.0f32));
        assert_eq!(c.icon_line_height, Some(2.0));
        c.param_update(CheckboxParam::IconLineHeight, &py_none());
        assert_eq!(c.icon_line_height, None);
    }

    #[test]
    fn test_icon_size() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(CheckboxParam::IconSize, &py_obj(20.0f32));
        assert_eq!(c.icon_size, Some(20.0));
        c.param_update(CheckboxParam::IconSize, &py_none());
        assert_eq!(c.icon_size, None);
    }

    #[test]
    fn test_is_checked() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(CheckboxParam::IsChecked, &py_obj(true));
        assert!(c.is_checked);
        c.param_update(CheckboxParam::IsChecked, &py_obj(false));
        assert!(!c.is_checked);
    }

    #[test]
    fn test_label() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(CheckboxParam::Label, &py_obj("Check me"));
        assert_eq!(c.label, Some("Check me".to_string()));
    }

    #[test]
    fn test_show() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(CheckboxParam::Show, &py_obj(false));
        assert!(!c.show);
    }

    #[test]
    fn test_spacing() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(CheckboxParam::Spacing, &py_obj(8.0f32));
        assert_eq!(c.spacing, Some(8.0));
        c.param_update(CheckboxParam::Spacing, &py_none());
        assert_eq!(c.spacing, None);
    }

    #[test]
    fn test_text_line_height() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(CheckboxParam::TextLineHeight, &py_obj(1.5f32));
        assert_eq!(c.text_line_height, Some(1.5));
        c.param_update(CheckboxParam::TextLineHeight, &py_none());
        assert_eq!(c.text_line_height, None);
    }

    #[test]
    fn test_text_size() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(CheckboxParam::TextSize, &py_obj(14.0f32));
        assert_eq!(c.text_size, Some(14.0));
        c.param_update(CheckboxParam::TextSize, &py_none());
        assert_eq!(c.text_size, None);
    }

    #[test]
    fn test_style_id() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(CheckboxParam::Style, &py_obj(5u64));
        assert_eq!(c.style_id, Some(5));
    }

    #[test]
    fn test_width() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(CheckboxParam::Width, &py_obj(150.0f32));
        assert_eq!(c.width, Length::Fixed(150.0));
        c.param_update(CheckboxParam::Width, &py_none());
        assert_eq!(c.width, Length::Shrink);
    }

    #[test]
    fn test_width_fill() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(CheckboxParam::WidthFill, &py_obj(true));
        assert_eq!(c.width, Length::Fill);
        c.param_update(CheckboxParam::WidthFill, &py_obj(false));
        assert_eq!(c.width, Length::Shrink);
    }

    // -----------------------------------------------------------------------
    // CheckboxStyle param tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_style_background_rgba() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(CheckboxStyleParam::BackgroundRgbaColor, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.background_color.is_some());
        s.param_update(CheckboxStyleParam::BackgroundRgbaColor, &py_none());
        assert!(s.background_color.is_none());
    }

    #[test]
    fn test_style_border_rgba() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(CheckboxStyleParam::BorderRgbaColor, &py_obj(vec![0.0f32, 1.0, 0.0, 1.0]));
        assert!(s.border_color.is_some());
        s.param_update(CheckboxStyleParam::BorderRgbaColor, &py_none());
        assert!(s.border_color.is_none());
    }

    #[test]
    fn test_style_border_radius() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(CheckboxStyleParam::BorderRadius, &py_obj(vec![3.0f32, 3.0, 3.0, 3.0]));
        assert_eq!(s.border_radius, Some(vec![3.0, 3.0, 3.0, 3.0]));
    }

    #[test]
    fn test_style_border_width() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(CheckboxStyleParam::BorderWidth, &py_obj(2.0f32));
        assert_eq!(s.border_width, Some(2.0));
        s.param_update(CheckboxStyleParam::BorderWidth, &py_none());
        assert_eq!(s.border_width, None);
    }

    #[test]
    fn test_style_icon_rgba() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(CheckboxStyleParam::IconRgbaColor, &py_obj(vec![0.0f32, 0.0, 1.0, 1.0]));
        assert!(s.icon_color.is_some());
        s.param_update(CheckboxStyleParam::IconRgbaColor, &py_none());
        assert!(s.icon_color.is_none());
    }

    #[test]
    fn test_style_text_rgba() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(CheckboxStyleParam::TextRgbaColor, &py_obj(vec![1.0f32, 1.0, 1.0, 1.0]));
        assert!(s.text_color.is_some());
        s.param_update(CheckboxStyleParam::TextRgbaColor, &py_none());
        assert!(s.text_color.is_none());
    }
}