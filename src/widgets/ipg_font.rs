//! ipg_font

use iced::font;
use iced::{Pixels};
use iced::advanced::text::LineHeight;
use iced::widget::pick_list;
use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};


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
        font.family = if let Some(name) = &self.family_name {
            let name: &'static str = Box::leak(name.clone().into_boxed_str());
            iced::font::Family::Name(name)
        } else if let Some(family) = &self.family {
            family.to_iced(None)
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

#[derive(Debug, Clone, Default, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
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
    
}

#[derive(Debug, Clone, Default, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
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
}

#[derive(Debug, Clone, Default, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
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
}

#[derive(Debug, Clone, Default, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
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
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
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
            FontParam::Family => set_t_value(&mut self.family, value, "FontParam::Family"),
            FontParam::Weight => set_t_value(&mut self.weight, value, "FontParam::Weight"),
            FontParam::Stretch => set_t_value(&mut self.stretch, value, "FontParam::Stretch"),
            FontParam::Style => set_t_value(&mut self.style, value, "FontParam::Style"),
        }
    }
}

// ---------------------------------------------------------------------------
// IpgIcon — a pick_list Icon that stores a resolved font directly
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct IpgIcon {
    pub id: usize,
    /// The resolved iced Font used to render the glyph.
    pub font: iced::Font,
    /// The Unicode code point of the glyph to render.
    pub code_point: char,
    /// Optional glyph size in pixels.
    pub size: Option<f32>,
    /// Optional line height multiplier (relative). Defaults to 1.0.
    pub line_height: Option<f32>,
}

impl IpgIcon {
    /// Convert to an iced `pick_list::Icon`.
    pub fn to_iced(&self) -> pick_list::Icon<iced::Font> {
        
        pick_list::Icon {
            font: self.font,
            code_point: self.code_point,
            size: self.size.map(Pixels),
            line_height: self.line_height
                .map(LineHeight::Relative)
                .unwrap_or(LineHeight::Relative(1.0)),
            shaping: iced::advanced::text::Shaping::default(),
        }
    }
}
