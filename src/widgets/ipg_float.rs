//! ipg_float

use iced::{Element, Vector};
use iced::widget::float;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_opt_f32, set_opt_f32_array_2, set_opt_vec_f32};


#[derive(Clone, Debug)]
pub struct Float {
    pub id: usize,
    pub scale: Option<f32>,
    pub scale_clamped: Option<f32>,
    pub clamped_padding: Option<Vec<f32>>,
    pub translate: Option<[f32; 2]>,
}

impl Float {
    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        ) -> Element<'a, Message> {

        let scale = if let Some(clamp) = self.scale_clamped {
            clamp
        } else {
            self.scale.unwrap_or(1.0)
        };

        float(content.remove(0))
        .scale(scale)
        .translate(move |bounds, viewport| {
            if let Some(clamp) = self.scale_clamped { 
                bounds.zoom(clamp).offset(&viewport.shrink(get_padding(&self.clamped_padding)))
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
    ScaleClamped,
    ClampedPadding,
    Translate,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Float{
    type Param = FloatParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            FloatParam::Scale => set_opt_f32(&mut self.scale, value, "FloatParam::Scale"),
            FloatParam::Translate => set_opt_f32_array_2(&mut self.translate, value, "FloatParam::Translate"),
            FloatParam::ScaleClamped => set_opt_f32(&mut self.scale_clamped, value, "FloatParam::ScaleClamped"),
            FloatParam::ClampedPadding => set_opt_vec_f32(&mut self.clamped_padding, value, "FloatParam::ClampedPadding"),
        }
    }
}
