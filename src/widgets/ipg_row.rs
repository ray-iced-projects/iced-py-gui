//! ipg_row
#![allow(unused)]

use iced::{Alignment, Padding, Length, Element};
use iced::widget::Row;

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::py_api::helpers::{get_height, get_padding, 
    get_width, try_extract_boolean, try_extract_f32, try_extract_vec_f32};
use crate::widgets::enums::IpgAlignment;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,
    set_opt_bool, set_opt_f32, set_opt_vec_f32,
    set_width, set_width_fill, set_height, set_height_fill, set_align,
};



#[derive(Debug, Clone)]
pub struct IpgRow {
    pub id: usize,
    pub show: bool,

    pub spacing: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub width: Length,
    pub height: Length,
    pub align_y: Option<IpgAlignment>,
    pub clip: Option<bool>,
}

pub fn construct_row<'a>(
    ipg_row: &IpgRow, 
    content: Vec<Element<'a, Message>>,
    ) -> Element<'a, Message> {

    let row = 
        Row::with_children(content)
            .width(ipg_row.width)
            .height(ipg_row.height);
                        
    let row = 
        if let Some(align) = &ipg_row.align_y {
            row.align_y(IpgAlignment::to_iced(align))
        } else { row };

    let row = 
        row.padding(get_padding(&ipg_row.padding));

    let row = 
        if let Some(sp) = ipg_row.spacing {
            row.spacing(sp)
        } else { row };

    let row = 
        if let Some(cp) = ipg_row.clip {
            row.clip(cp)
        } else { row };

    row.into()
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRowParam {
    Align,
    Clip,
    Padding,
    Width,
    WidthFill,
    Height,
    HeightFill,
    Spacing,
}

pub fn row_item_update(ipg_row: &mut IpgRow,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_row_update(item);
    let name = "Row".to_string();
    match update {
        IpgRowParam::Align => {
            ipg_row.align_y = IpgAlignment::extract(value);
        },
        IpgRowParam::Clip => {
            ipg_row.clip = Some(try_extract_boolean(value, name));
        },
        IpgRowParam::Padding => {
            ipg_row.padding =  Some(try_extract_vec_f32(value, name));
        },
        IpgRowParam::Width => {
            let val = try_extract_f32(value, name);
            ipg_row.width = get_width(Some(val), false);
        },
        IpgRowParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            ipg_row.width = get_width(None, val);
        },
        IpgRowParam::Height => {
            let val = try_extract_f32(value, name);
            ipg_row.height = get_height(Some(val), false);
        },
        IpgRowParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            ipg_row.height = get_height(None, val);
        },
        IpgRowParam::Spacing => {
            ipg_row.spacing = Some(try_extract_f32(value, name));
        },
    }
}

pub fn try_extract_row_update(update_obj: &PyObject) -> IpgRowParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgRowParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Row update extraction failed"),
        }
    })
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgRow {
    type Param = IpgRowParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgRowParam::Align      => set_align(&mut self.align_y, value, name),
            IpgRowParam::Clip       => set_opt_bool(&mut self.clip, value, name),
            IpgRowParam::Padding    => set_opt_vec_f32(&mut self.padding, value, name),
            IpgRowParam::Width      => set_width(&mut self.width, value, name),
            IpgRowParam::WidthFill  => set_width_fill(&mut self.width, value, name),
            IpgRowParam::Height     => set_height(&mut self.height, value, name),
            IpgRowParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgRowParam::Spacing    => set_opt_f32(&mut self.spacing, value, name),
        }
    }
}
