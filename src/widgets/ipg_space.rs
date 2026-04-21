//! ipg_space

use iced::Element;
use iced::widget;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::py_api::helpers::get_len;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};


#[derive(Debug, Clone)]
pub struct Space {
    pub id: usize,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub show: bool,
}

impl Space {
    pub fn construct(
        &self
    ) -> Option<Element<'_, Message>> {

        if self.show {
            Some(widget::Space::new()
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height)).into())
        } else {
            None
        }
    }
}
#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SpaceParam {
    Fill,
    Height,
    HeightFill,
    Width,
    WidthFill,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Space {
    type Param = SpaceParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SpaceParam::Fill => set_t_value(&mut self.fill, value, "SpaceParam::Fill"),
            SpaceParam::Width => set_t_value(&mut self.width, value, "SpaceParam::Width"),
            SpaceParam::WidthFill => set_t_value(&mut self.width_fill, value, "SpaceParam::WidthFill"),
            SpaceParam::Height => set_t_value(&mut self.height, value, "SpaceParam::Height"),
            SpaceParam::HeightFill => set_t_value(&mut self.height_fill, value, "SpaceParam::HeightFill"),
        }
    }
}
