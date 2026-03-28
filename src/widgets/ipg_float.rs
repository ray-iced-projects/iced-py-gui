//! ipg_float

use iced::{Element, Vector};
use iced::widget::float;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_opt_bool, set_opt_f32, set_opt_f32_array_2};


#[derive(Clone, Debug)]
pub struct IpgFloat {
    pub id: usize,
    pub scale: Option<f32>,
    pub translate: Option<[f32; 2]>,
    pub scale_clamped: Option<bool>,
}

impl IpgFloat {
    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        ) -> Element<'a, Message> {
            
        float(content.remove(0))
        .scale(self.scale.unwrap_or(1.0))
        .translate(move |bounds, viewport| {
            if self.scale_clamped == Some(true) { 
                bounds.zoom(self.scale.unwrap_or(1.0)).offset(&viewport.shrink(10))
            } else if let Some(tr) = self.translate {
                Vector::new(tr[0], tr[1])
            } else {
                Vector::ZERO
            }
        }).into()
    }
}


#[pyclass]
#[derive(Clone, Debug, PartialEq)]
pub enum FloatParam {
    Scale,
    Translate,
    ScaleClamped,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgFloat{
    type Param = FloatParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            FloatParam::Scale => set_opt_f32(&mut self.scale, value, "FloatParam::Scale"),
            FloatParam::Translate => set_opt_f32_array_2(&mut self.translate, value, "FloatParam::Translate"),
            FloatParam::ScaleClamped => set_opt_bool(&mut self.scale_clamped, value, "FloatParam::ScaleClamped"),
        }
    }
}
