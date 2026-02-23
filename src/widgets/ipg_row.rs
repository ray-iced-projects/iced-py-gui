//! ipg_row
use iced::{Length, Element};
use iced::widget::Row;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::py_api::helpers::get_padding;
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


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgRow {
    type Param = IpgRowParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgRowParam::Align => set_align(&mut self.align_y, value, name),
            IpgRowParam::Clip => set_opt_bool(&mut self.clip, value, name),
            IpgRowParam::Padding => set_opt_vec_f32(&mut self.padding, value, name),
            IpgRowParam::Width => set_width(&mut self.width, value, name),
            IpgRowParam::WidthFill => set_width_fill(&mut self.width, value, name),
            IpgRowParam::Height => set_height(&mut self.height, value, name),
            IpgRowParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgRowParam::Spacing => set_opt_f32(&mut self.spacing, value, name),
        }
    }
}
