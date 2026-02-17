//! ipg_column
#![allow(unused)]

use iced::{Element, Length, Padding};
use iced::widget::Column;
use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::py_api::helpers::{get_height, 
    get_padding_f64, get_width, try_extract_boolean, 
    try_extract_f64, try_extract_ipg_alignment, try_extract_vec_f64};
use crate::widgets::enums::IpgAlignment;


#[derive(Debug, Clone)]
pub struct IpgFont {
    pub id: usize,
    pub family_name: String,
    pub font_type: Option<IpgFontType>,
    pub weight: Option<IpgFontWeight>,
    pub stretch: Option<IpgFontStretch>,
    pub style: Option<IpgFontStyle>,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgFontType {
    Serif,
    SansSerif,
    Cursive,
    Fantasy,
    Monospace,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgFontWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgFontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgFontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgFontParam {
    FamilyName,
    Type,
    Weight,
    Stretch,
    Style,
}

pub fn font_param_update(
    col: &mut IpgFont,
    item: &PyObject,
    value: &PyObject,
    )
{
    let update = try_extract_font_update(item);
    let name = "Column".to_string();
    match update {
        IpgFontParam::FamilyName => {

        },
        IpgFontParam::Type => {

        },
        IpgFontParam::Weight => {

        },
        IpgFontParam::Stretch => {

        },
        IpgFontParam::Style => {

        },
    }
}

pub fn try_extract_font_update(update_obj: &PyObject) -> IpgFontParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgFontParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Font update extraction failed"),
        }
    })
}
