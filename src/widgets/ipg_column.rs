//! ipg_column
#![allow(unused)]

use iced::{Element, Length, Padding};
use iced::widget::Column;
use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::py_api::helpers::{get_height, get_padding_f32, 
    get_width, try_extract_boolean, try_extract_f32, 
    try_extract_vec_f32};
use crate::widgets::enums::{IpgAlignment, IpgHorizontalAlignment};


#[derive(Debug, Clone)]
pub struct IpgColumn {
    pub id: usize,
    pub show: bool,
    pub spacing: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub width: Length,
    pub height: Length,
    pub max_width: Option<f32>,
    pub align_x: Option<IpgHorizontalAlignment>,
    pub clip: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgColumnParam {
    Align,
    Clip,
    Padding,
    Width,
    WidthFill,
    Height,
    HeightFill,
    Spacing,
}

pub fn construct_column<'a>(
    ipg_col: &IpgColumn, 
    content: Vec<Element<'a, Message>> 
    ) -> Element<'a, Message> {

    let col = 
        Column::with_children(content)
            .width(ipg_col.width)
            .height(ipg_col.height);
    
    let col = 
    if let Some(align) = &ipg_col.align_x {
            col.align_x(align.to_iced())
        } else { col };

    let col = 
        if let Some(cp) = ipg_col.clip {
            col.clip(cp)
        } else { col };

    let col = 
        if let Some(pd) = &ipg_col.padding {
            col.padding(get_padding_f32(pd))
        } else { col };

    let col = 
        if let Some(sp) = ipg_col.spacing {
            col.spacing(sp)
        } else { col };

    col.into()

}


pub fn column_item_update(
    col: &mut IpgColumn,
    item: &PyObject,
    value: &PyObject,
    )
{
    let update = try_extract_column_update(item);
    let name = "Column".to_string();
    match update {
        IpgColumnParam::Align => {
            col.align_x = Some(IpgHorizontalAlignment::extract(value).unwrap());
        },
        IpgColumnParam::Clip => {
            col.clip = Some(try_extract_boolean(value, name));
        },
        IpgColumnParam::Padding => {
            col.padding =  Some(try_extract_vec_f32(value, name));
        },
        IpgColumnParam::Width => {
            let val = try_extract_f32(value, name);
            col.width = get_width(Some(val), false);
        },
        IpgColumnParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            col.width = get_width(None, val);
        },
        IpgColumnParam::Height => {
            let val = try_extract_f32(value, name);
            col.height = get_height(Some(val), false);
        },
        IpgColumnParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            col.height = get_height(None, val);
        },
        IpgColumnParam::Spacing => {
            col.spacing = Some(try_extract_f32(value, name));
        },
    }
}

pub fn try_extract_column_update(update_obj: &PyObject) -> IpgColumnParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgColumnParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Column update extraction failed"),
        }
    })
}
