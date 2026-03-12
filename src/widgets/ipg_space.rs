//! ipg_space
use iced::{Element, Length};
use iced::widget::Space;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_height, set_height_fill, 
    set_width, set_width_fill};


#[derive(Debug, Clone)]
pub struct IpgSpace {
    pub id: usize,
    pub parent_id: String,
    pub width: Length,
    pub height: Length,
    pub show: bool,
}

pub fn construct_space(sp: &IpgSpace) -> Option<Element<'_, Message>> {

    if sp.show {
        Some(Space::new()
            .width(sp.width)
            .height(sp.height).into())
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSpaceParam {
    Height,
    HeightFill,
    Width,
    WidthFill,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgSpace {
    type Param = IpgSpaceParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgSpaceParam::Width => set_width(&mut self.width, value, "Width"),
            IpgSpaceParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
            IpgSpaceParam::Height => set_height(&mut self.height, value, "Height"),
            IpgSpaceParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
        }
    }
}
