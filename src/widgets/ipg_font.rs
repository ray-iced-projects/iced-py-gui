//! ipg_font
#![allow(unused)]
use iced::font;
use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;

use crate::widgets::widget_param_update::{WidgetParamUpdate};


#[derive(Debug, Clone, Default, PartialEq)]
pub struct IpgFont {
    pub id: usize,
    pub family: Option<IpgFontFamily>,
    pub family_name: Option<String>,
    pub weight: Option<IpgFontWeight>,
    pub stretch: Option<IpgFontStretch>,
    pub style: Option<IpgFontStyle>,
}

impl IpgFont {
    pub fn to_iced(&self) -> iced::font::Font {
        let mut font = iced::font::Font::default();
        font.family = if let Some(family) = &self.family {
            let family = if let Some(name) = &self.family_name {
                family.to_iced(self.family_name.clone())
            } else {
                eprintln!("[WARNING] family_name must be defined if 
                IpgFamily::Name selected, default SansSerif used");
                iced::font::Family::default()
            };
            family
        } else { iced::font::Family::default() };

        font.weight = if let Some(wt) = &self.weight {
            wt.to_iced()
        } else { iced::font::Weight::default() };

        font.stretch = if let Some(st) = &self.stretch {
            st.to_iced()
        } else { iced::font::Stretch::default() };

        font.style = if let Some(st) = &self.style {
            st.to_iced()
        } else { iced::font::Style::default() };

        font

    }
}

#[derive(Debug, Clone, Default, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgFontFamily {
    Name,
    Cursive,
    Fantasy,
    Monospace,
    #[default]
    SansSerif,
    Serif,
}

impl IpgFontFamily {
    fn to_iced(&self, name: Option<String>) -> font::Family {
        match self {
            IpgFontFamily::Name => {
                if let Some(name) = name {
                    let name: &'static str = Box::leak(name.clone().into_boxed_str());
                    font::Family::Name(name)
                } else {
                    font::Family::default()
                }
            },
            IpgFontFamily::Cursive => font::Family::Cursive,
            IpgFontFamily::Fantasy => font::Family::Fantasy,
            IpgFontFamily::Monospace => font::Family::Monospace,
            IpgFontFamily::SansSerif => font::Family::SansSerif,
            IpgFontFamily::Serif => font::Family::Serif,
        }
    }
    
    fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("Unable to extract python object for IpgFontFamily"),
            }
        }))
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgFontWeight {
    Black,
    Bold,
    ExtraBold,
    ExtraLight,
    Light,
    Medium,
    #[default]
    Normal,
    Semibold,
    Thin,
}

impl IpgFontWeight {
    fn to_iced(&self) -> font::Weight {
        match self {
            IpgFontWeight::Black => font::Weight::Black,
            IpgFontWeight::Bold => font::Weight::Bold,
            IpgFontWeight::ExtraBold => font::Weight::ExtraBold,
            IpgFontWeight::ExtraLight => font::Weight::ExtraLight,
            IpgFontWeight::Light => font::Weight::Light,
            IpgFontWeight::Medium => font::Weight::Medium,
            IpgFontWeight::Normal => font::Weight::Normal,
            IpgFontWeight::Semibold => font::Weight::Semibold,
            IpgFontWeight::Thin => font::Weight::Thin,
        }
    }

    fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("Unable to extract python object for IpgFontWeight"),
            }
        }))
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgFontStretch {
    Condensed,
    Expanded,
    ExtraCondensed,
    ExtraExpanded,
    #[default]
    Normal,
    SemiCondensed,
    SemiExpanded,
    UltraCondensed,
    UltraExpanded,
}

impl IpgFontStretch {
    fn to_iced(&self) -> font::Stretch {
        match self {
            IpgFontStretch::Condensed => font::Stretch::Condensed,
            IpgFontStretch::Expanded => font::Stretch::Expanded,
            IpgFontStretch::ExtraCondensed => font::Stretch::ExtraCondensed,
            IpgFontStretch::ExtraExpanded => font::Stretch::ExtraExpanded,
            IpgFontStretch::Normal => font::Stretch::Normal,
            IpgFontStretch::SemiCondensed => font::Stretch::SemiCondensed,
            IpgFontStretch::SemiExpanded => font::Stretch::SemiExpanded,
            IpgFontStretch::UltraCondensed => font::Stretch::UltraCondensed,
            IpgFontStretch::UltraExpanded => font::Stretch::UltraExpanded,
        }
    }

    fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("Unable to extract python object for IpgFontStretch"),
            }
        }))
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgFontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

impl IpgFontStyle {
    fn to_iced(&self) -> font::Style {
        match self {
            IpgFontStyle::Normal => font::Style::Normal,
            IpgFontStyle::Italic => font::Style::Italic,
            IpgFontStyle::Oblique => font::Style::Oblique,
        }
    }

    fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("Unable to extract python object for IpgFontStyle"),
            }
        }))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgFontParam {
    Family,
    Weight,
    Stretch,
    Style,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgFont {
    type Param = IpgFontParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, _name: String) {
        match param {
            IpgFontParam::Family => self.family = IpgFontFamily::extract(value),
            IpgFontParam::Weight => self.weight = IpgFontWeight::extract(value),
            IpgFontParam::Stretch => self.stretch = IpgFontStretch::extract(value),
            IpgFontParam::Style => self.style = IpgFontStyle::extract(value),
        }
    }
}
