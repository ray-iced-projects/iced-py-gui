//! ipg_row
use iced::{Alignment, Element, Length};
use iced::widget::Row;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::py_api::helpers::get_padding;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,
    set_opt_bool, set_opt_f32, set_opt_vec_f32,
    set_width, set_width_fill, set_height, set_height_fill,
};


#[derive(Debug, Clone)]
pub struct IpgRow {
    pub id: usize,
    pub show: bool,

    pub spacing: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub width: Length,
    pub height: Length,
    pub align_bottom: Option<bool>,
    pub align_center: Option<bool>,
    pub align_top: Option<bool>,
    pub clip: Option<bool>,
}

impl IpgRow {
    pub fn construct<'a>(
        &self, 
        content: Vec<Element<'a, Message>>,
        ) -> Element<'a, Message> {

        let row = 
            Row::with_children(content)
                .width(self.width)
                .height(self.height);
                            
        let row = 
            if self.align_top ==  Some(true) {
                    row.align_y(Alignment::Start)
                } else { row };

        let row = 
            if self.align_center ==  Some(true) {
                    row.align_y(Alignment::Center)
                } else { row };

        let row = 
            if self.align_bottom ==  Some(true) {
                    row.align_y(Alignment::End)
                } else { row };

        let row = 
            row.padding(get_padding(&self.padding));

        let row = 
            if let Some(sp) = self.spacing {
                row.spacing(sp)
            } else { row };

        let row = 
            if let Some(cp) = self.clip {
                row.clip(cp)
            } else { row };

        row.into()
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRowParam {
    AlignBottom,
    AlignCenter,
    AlignTop,
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

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        let name = String::new();
        match param {
            IpgRowParam::AlignBottom => set_opt_bool(&mut self.align_bottom, value, name),
            IpgRowParam::AlignCenter => set_opt_bool(&mut self.align_center, value, name),
            IpgRowParam::AlignTop => set_opt_bool(&mut self.align_top, value, name),
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
