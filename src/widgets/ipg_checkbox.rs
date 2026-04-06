//! ipg_checkbox

use std::collections::HashMap;

use crate::state::IpgState;
use crate::app::Message;
use crate::widgets::ipg_text::{TextShaping, TextWrapping};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_opt_f32, set_opt_iced_color, set_opt_iced_color_from_rgba, set_opt_string, set_opt_usize, set_opt_vec_f32, set_width, set_width_fill
};
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::state::IpgWidgets;

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap_icon::{IpgIcon, icon_to_char};

use crate::widgets::styling::{apply_border_overrides, create_custom_theme};

use iced::advanced::text;
use iced::{Background, Element, Length, Theme};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{Checkbox, checkbox};
use iced::theme::palette;

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgCheckBox {
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
    pub text_shaping: Option<TextShaping>,
    pub text_wrapping: Option<TextWrapping>,
    pub text_font_id: Option<usize>,
    pub icon_font_id: Option<usize>,
    pub icon: Option<IpgIcon>,
    pub icon_size: Option<f32>,
    pub icon_line_height: Option<f32>,
    pub icon_shaping: Option<TextShaping>,
    pub style_id: Option<usize>,
    pub style_std: Option<IpgCheckboxStyleStd>,
}

impl IpgCheckBox {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, IpgWidgets>,
    ) -> Option<Element<'a, Message>> {

        if !self.show {
            return None
        };

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(IpgWidgets::as_checkbox_style).cloned();

        // Icon related
        let code_point = 
            if let Some(ic) = self.icon {
                icon_to_char(ic)
            } else {
                icon_to_char(IpgIcon::Check)
            };

        let size = 
            if let Some(sz) = self.icon_size {
                Some(iced::Pixels(sz))
            } else { None };

        let shaping = 
            if let Some(sh) = &self.icon_shaping {
                TextShaping::to_iced(sh)
            } else { Shaping::default() };

        let line_height = 
            if let Some(lh) = self.icon_line_height {
                LineHeight::Relative(lh)
            } else { LineHeight::default() };

        let icon = 
            checkbox::Icon {
                font: BOOTSTRAP_FONT,
                code_point,
                size,
                line_height,
                shaping,
            };
        
        // Text related
        let text_line_height = 
            if let Some(lh) = self.text_line_height {
                text::LineHeight::Relative(lh)
            } else { text::LineHeight::default() };

        let text_shaping = 
            if let Some(ts) = &self.text_shaping {
                TextShaping::to_iced(ts)
            } else { Shaping::default() };

    let chk = 
            Checkbox::new(self.is_checked)
                .on_toggle(ChkMessage::OnToggle)
                .width(self.width)
                .text_line_height(text_line_height)
                .text_shaping(text_shaping)
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

        let chk: Element<'_, ChkMessage> = 
            if let Some(sp) = self.spacing {
                chk.spacing(sp).into()
            } else { chk.into() };

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
            if let Some(IpgWidgets::IpgCheckBox(cb)) = state.widgets.get_mut(&id) {
                cb.is_checked = is_checked;
            }
            invoke_callback_with_args(id, "on_toggle", "Checkbox", is_checked);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct IpgCheckboxStyle {
    pub id: usize,
    pub background_color: Option<iced::Color>,
    pub border_color: Option<iced::Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub icon_color: Option<iced::Color>,
    pub text_color: Option<iced::Color>,
}

impl IpgCheckboxStyle {
    fn to_iced(
         &self, 
        theme: &Theme, 
        status: checkbox::Status,
        std_style_opt: &Option<IpgCheckboxStyleStd>,
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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCheckboxStyleStd {
    Danger,
    Primary,
    Secondary,
    Success,
}

impl IpgCheckboxStyleStd {
    pub fn to_iced (
        &self,
        theme: &Theme, 
        status: checkbox::Status, 
        ) -> checkbox::Style {
        
        match self {
            IpgCheckboxStyleStd::Danger => {
                checkbox::danger(theme, status)
            },
            IpgCheckboxStyleStd::Primary => {
                checkbox::primary(theme, status)
            },
            IpgCheckboxStyleStd::Secondary => {
                checkbox::secondary(theme, status)
            },
            IpgCheckboxStyleStd::Success => {
                checkbox::success(theme, status)
            },
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCheckboxParam {
    Icon,
    IconFont,
    IconLineHeight,
    IconSize,
    IconShaping,
    IsChecked,
    Label,
    Spacing,
    Style,
    StyleStandard,
    TextFont,
    TextLineHeight,
    TextShaping,
    TextSize,
    Width,
    WidthFill,
    Show,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCheckboxStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    IconIpgColor,
    IconRgbaColor,
    TextIpgColor,
    TextRgbaColor,
}

fn extract_chk_style_standard(
    value: &PyObject, 
    ) -> IpgCheckboxStyleStd {
    
    Python::attach(|py| {

        let res = 
            value.extract::<IpgCheckboxStyleStd>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python object for CheckboxStyleStandard"),
        }
    })
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgCheckBox {
    type Param = IpgCheckboxParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {

        match param {
            IpgCheckboxParam::Icon => {
                self.icon = Some(IpgIcon::extract(value));
            }
            IpgCheckboxParam::IconFont     => { /* TODO */ }
            IpgCheckboxParam::IconLineHeight => set_opt_f32(&mut self.icon_line_height, value, "IconLineHeight"),
            IpgCheckboxParam::IconSize     => set_opt_f32(&mut self.icon_size, value, "IconSize"),
            IpgCheckboxParam::IconShaping  => self.icon_shaping = TextShaping::extract(value),
            IpgCheckboxParam::IsChecked    => set_bool(&mut self.is_checked, value, "IsChecked"),
            IpgCheckboxParam::Label        => set_opt_string(&mut self.label, value, "Label"),
            IpgCheckboxParam::Show         => set_bool(&mut self.show, value, "Show"),
            IpgCheckboxParam::Spacing      => set_opt_f32(&mut self.spacing, value, "Spacing"),
            IpgCheckboxParam::TextLineHeight => set_opt_f32(&mut self.text_line_height, value, "TextLineHeight"),
            IpgCheckboxParam::TextShaping  => self.text_shaping = TextShaping::extract(value),
            IpgCheckboxParam::TextSize     => set_opt_f32(&mut self.text_size, value, "TextSize"),
            IpgCheckboxParam::Style        => set_opt_usize(&mut self.style_id, value, "Style"),
            IpgCheckboxParam::StyleStandard => self.style_std = Some(extract_chk_style_standard(value)),
            IpgCheckboxParam::Width        => set_width(&mut self.width, value, "Width"),
            IpgCheckboxParam::WidthFill    => set_width_fill(&mut self.width, value, "WidthFill"),
            IpgCheckboxParam::TextFont     => { /* TODO */ }
        }
    }
}

impl WidgetParamUpdate for IpgCheckboxStyle {
    type Param = IpgCheckboxStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgCheckboxStyleParam::BackgroundIpgColor => set_opt_iced_color(&mut self.background_color, value, "BackgroundIpgColor"),
            IpgCheckboxStyleParam::BackgroundRgbaColor => set_opt_iced_color_from_rgba(&mut self.background_color, value, "BackgroundRgbaColor"),
            IpgCheckboxStyleParam::BorderIpgColor => set_opt_iced_color(&mut self.border_color, value, "BorderIpgColor"),
            IpgCheckboxStyleParam::BorderRgbaColor => set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgbaColor"),
            IpgCheckboxStyleParam::BorderRadius => set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
            IpgCheckboxStyleParam::BorderWidth => set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            IpgCheckboxStyleParam::IconIpgColor => set_opt_iced_color(&mut self.icon_color, value, "IconIpgColor"),
            IpgCheckboxStyleParam::IconRgbaColor => set_opt_iced_color_from_rgba(&mut self.icon_color, value, "IconRgbaColor"),
            IpgCheckboxStyleParam::TextIpgColor => set_opt_iced_color(&mut self.text_color, value, "TextIpgColor"),
            IpgCheckboxStyleParam::TextRgbaColor => set_opt_iced_color_from_rgba(&mut self.text_color, value, "TextRgbaColor"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::IntoPyObjectExt;

    fn make_checkbox() -> IpgCheckBox {
        IpgCheckBox {
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
            text_shaping: None,
            text_wrapping: None,
            text_font_id: None,
            icon_font_id: None,
            icon: None,
            icon_size: None,
            icon_line_height: None,
            icon_shaping: None,
            style_id: None,
            style_std: None,
        }
    }

    fn make_checkbox_style() -> IpgCheckboxStyle {
        IpgCheckboxStyle::default()
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
    // IpgCheckBox param tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_icon_line_height() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(IpgCheckboxParam::IconLineHeight, &py_obj(2.0f32));
        assert_eq!(c.icon_line_height, Some(2.0));
        c.param_update(IpgCheckboxParam::IconLineHeight, &py_none());
        assert_eq!(c.icon_line_height, None);
    }

    #[test]
    fn test_icon_size() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(IpgCheckboxParam::IconSize, &py_obj(20.0f32));
        assert_eq!(c.icon_size, Some(20.0));
        c.param_update(IpgCheckboxParam::IconSize, &py_none());
        assert_eq!(c.icon_size, None);
    }

    #[test]
    fn test_is_checked() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(IpgCheckboxParam::IsChecked, &py_obj(true));
        assert!(c.is_checked);
        c.param_update(IpgCheckboxParam::IsChecked, &py_obj(false));
        assert!(!c.is_checked);
    }

    #[test]
    fn test_label() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(IpgCheckboxParam::Label, &py_obj("Check me"));
        assert_eq!(c.label, Some("Check me".to_string()));
    }

    #[test]
    fn test_show() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(IpgCheckboxParam::Show, &py_obj(false));
        assert!(!c.show);
    }

    #[test]
    fn test_spacing() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(IpgCheckboxParam::Spacing, &py_obj(8.0f32));
        assert_eq!(c.spacing, Some(8.0));
        c.param_update(IpgCheckboxParam::Spacing, &py_none());
        assert_eq!(c.spacing, None);
    }

    #[test]
    fn test_text_line_height() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(IpgCheckboxParam::TextLineHeight, &py_obj(1.5f32));
        assert_eq!(c.text_line_height, Some(1.5));
        c.param_update(IpgCheckboxParam::TextLineHeight, &py_none());
        assert_eq!(c.text_line_height, None);
    }

    #[test]
    fn test_text_size() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(IpgCheckboxParam::TextSize, &py_obj(14.0f32));
        assert_eq!(c.text_size, Some(14.0));
        c.param_update(IpgCheckboxParam::TextSize, &py_none());
        assert_eq!(c.text_size, None);
    }

    #[test]
    fn test_style_id() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(IpgCheckboxParam::Style, &py_obj(5u64));
        assert_eq!(c.style_id, Some(5));
    }

    #[test]
    fn test_width() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(IpgCheckboxParam::Width, &py_obj(150.0f32));
        assert_eq!(c.width, Length::Fixed(150.0));
        c.param_update(IpgCheckboxParam::Width, &py_none());
        assert_eq!(c.width, Length::Shrink);
    }

    #[test]
    fn test_width_fill() {
        Python::initialize();
        let mut c = make_checkbox();
        c.param_update(IpgCheckboxParam::WidthFill, &py_obj(true));
        assert_eq!(c.width, Length::Fill);
        c.param_update(IpgCheckboxParam::WidthFill, &py_obj(false));
        assert_eq!(c.width, Length::Shrink);
    }

    // -----------------------------------------------------------------------
    // IpgCheckboxStyle param tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_style_background_rgba() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(IpgCheckboxStyleParam::BackgroundRgbaColor, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.background_color.is_some());
        s.param_update(IpgCheckboxStyleParam::BackgroundRgbaColor, &py_none());
        assert!(s.background_color.is_none());
    }

    #[test]
    fn test_style_border_rgba() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(IpgCheckboxStyleParam::BorderRgbaColor, &py_obj(vec![0.0f32, 1.0, 0.0, 1.0]));
        assert!(s.border_color.is_some());
        s.param_update(IpgCheckboxStyleParam::BorderRgbaColor, &py_none());
        assert!(s.border_color.is_none());
    }

    #[test]
    fn test_style_border_radius() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(IpgCheckboxStyleParam::BorderRadius, &py_obj(vec![3.0f32, 3.0, 3.0, 3.0]));
        assert_eq!(s.border_radius, Some(vec![3.0, 3.0, 3.0, 3.0]));
    }

    #[test]
    fn test_style_border_width() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(IpgCheckboxStyleParam::BorderWidth, &py_obj(2.0f32));
        assert_eq!(s.border_width, Some(2.0));
        s.param_update(IpgCheckboxStyleParam::BorderWidth, &py_none());
        assert_eq!(s.border_width, None);
    }

    #[test]
    fn test_style_icon_rgba() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(IpgCheckboxStyleParam::IconRgbaColor, &py_obj(vec![0.0f32, 0.0, 1.0, 1.0]));
        assert!(s.icon_color.is_some());
        s.param_update(IpgCheckboxStyleParam::IconRgbaColor, &py_none());
        assert!(s.icon_color.is_none());
    }

    #[test]
    fn test_style_text_rgba() {
        Python::initialize();
        let mut s = make_checkbox_style();
        s.param_update(IpgCheckboxStyleParam::TextRgbaColor, &py_obj(vec![1.0f32, 1.0, 1.0, 1.0]));
        assert!(s.text_color.is_some());
        s.param_update(IpgCheckboxStyleParam::TextRgbaColor, &py_none());
        assert!(s.text_color.is_none());
    }
}