//! ipg_font

use iced::font;
use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;

use crate::widgets::widget_param_update::{WidgetParamUpdate};


#[derive(Debug, Clone, Default, PartialEq)]
pub struct Font {
    pub id: usize,
    pub family: Option<FontFamily>,
    pub family_name: Option<String>,
    pub weight: Option<FontWeight>,
    pub stretch: Option<FontStretch>,
    pub style: Option<FontStyle>,
}

impl Font {
    pub fn to_iced(&self) -> iced::font::Font {
        let mut font = iced::font::Font::default();
        font.family = if let Some(family) = &self.family {
            let family = if let Some(_) = &self.family_name {
                family.to_iced(self.family_name.clone())
            } else {
                eprintln!("[WARNING] family_name must be defined if 
                Family::Name selected, default SansSerif used");
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
pub enum FontFamily {
    Name,
    Cursive,
    Fantasy,
    Monospace,
    #[default]
    SansSerif,
    Serif,
}

impl FontFamily {
    fn to_iced(&self, name: Option<String>) -> font::Family {
        match self {
            FontFamily::Name => {
                if let Some(name) = name {
                    let name: &'static str = Box::leak(name.clone().into_boxed_str());
                    font::Family::Name(name)
                } else {
                    font::Family::default()
                }
            },
            FontFamily::Cursive => font::Family::Cursive,
            FontFamily::Fantasy => font::Family::Fantasy,
            FontFamily::Monospace => font::Family::Monospace,
            FontFamily::SansSerif => font::Family::SansSerif,
            FontFamily::Serif => font::Family::Serif,
        }
    }
    
    fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("Unable to extract python object for FontFamily"),
            }
        }))
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum FontWeight {
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

impl FontWeight {
    fn to_iced(&self) -> font::Weight {
        match self {
            FontWeight::Black => font::Weight::Black,
            FontWeight::Bold => font::Weight::Bold,
            FontWeight::ExtraBold => font::Weight::ExtraBold,
            FontWeight::ExtraLight => font::Weight::ExtraLight,
            FontWeight::Light => font::Weight::Light,
            FontWeight::Medium => font::Weight::Medium,
            FontWeight::Normal => font::Weight::Normal,
            FontWeight::Semibold => font::Weight::Semibold,
            FontWeight::Thin => font::Weight::Thin,
        }
    }

    fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("Unable to extract python object for FontWeight"),
            }
        }))
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum FontStretch {
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

impl FontStretch {
    fn to_iced(&self) -> font::Stretch {
        match self {
            FontStretch::Condensed => font::Stretch::Condensed,
            FontStretch::Expanded => font::Stretch::Expanded,
            FontStretch::ExtraCondensed => font::Stretch::ExtraCondensed,
            FontStretch::ExtraExpanded => font::Stretch::ExtraExpanded,
            FontStretch::Normal => font::Stretch::Normal,
            FontStretch::SemiCondensed => font::Stretch::SemiCondensed,
            FontStretch::SemiExpanded => font::Stretch::SemiExpanded,
            FontStretch::UltraCondensed => font::Stretch::UltraCondensed,
            FontStretch::UltraExpanded => font::Stretch::UltraExpanded,
        }
    }

    fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("Unable to extract python object for FontStretch"),
            }
        }))
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

impl FontStyle {
    fn to_iced(&self) -> font::Style {
        match self {
            FontStyle::Normal => font::Style::Normal,
            FontStyle::Italic => font::Style::Italic,
            FontStyle::Oblique => font::Style::Oblique,
        }
    }

    fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("Unable to extract python object for FontStyle"),
            }
        }))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum FontParam {
    Family,
    Weight,
    Stretch,
    Style,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Font {
    type Param = FontParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            FontParam::Family => self.family = FontFamily::extract(value),
            FontParam::Weight => self.weight = FontWeight::extract(value),
            FontParam::Stretch => self.stretch = FontStretch::extract(value),
            FontParam::Style => self.style = FontStyle::extract(value),
        }
    }
}
