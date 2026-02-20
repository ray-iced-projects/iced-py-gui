//! ipg_column
#![allow(unused)]

use iced::{Element, Length, Padding};
use iced::widget::Column;
use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::py_api::helpers::{get_height, get_padding, 
    get_width, try_extract_boolean, try_extract_f32, 
    try_extract_vec_f32};
use crate::widgets::enums::IpgAlignment;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,
    set_opt_bool, set_opt_f32, set_opt_vec_f32,
    set_width, set_width_fill, set_height, set_height_fill, set_align,
};


#[derive(Debug, Clone)]
pub struct IpgColumn {
    pub id: usize,
    pub show: bool,
    pub spacing: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub width: Length,
    pub height: Length,
    pub max_width: Option<f32>,
    pub align_x: Option<IpgAlignment>,
    pub clip: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgColumnParam {
    AlignX,
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
    if let Some(align_x) = &ipg_col.align_x {
            col.align_x(align_x.to_iced())
        } else { col };

    let col = 
        if let Some(cp) = ipg_col.clip {
            col.clip(cp)
        } else { col };

    let col = 
            col.padding(get_padding(&ipg_col.padding));

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
        IpgColumnParam::AlignX => {
            col.align_x = Some(IpgAlignment::extract(value).unwrap());
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

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgColumn {
    type Param = IpgColumnParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgColumnParam::AlignX     => set_align(&mut self.align_x, value),
            IpgColumnParam::Clip       => set_opt_bool(&mut self.clip, value, name),
            IpgColumnParam::Padding    => set_opt_vec_f32(&mut self.padding, value, name),
            IpgColumnParam::Width      => set_width(&mut self.width, value, name),
            IpgColumnParam::WidthFill  => set_width_fill(&mut self.width, value, name),
            IpgColumnParam::Height     => set_height(&mut self.height, value, name),
            IpgColumnParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgColumnParam::Spacing    => set_opt_f32(&mut self.spacing, value, name),
        }
    }
}
