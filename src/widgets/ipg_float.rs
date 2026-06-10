//! ipg_float

use iced::{Element, Vector};
use iced::widget::float;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};


#[derive(Clone, Debug)]
pub struct Float {
    pub id: usize,
    pub scale: Option<f32>,
    pub scale_clamped: Option<f32>,
    pub clamped_padding: Option<Vec<f32>>,
    pub translate: Option<[f32; 2]>,
    pub show: bool,
}

impl Float {
    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        ) -> Option<Element<'a, Message>> {

        if !self.show { return None }

        if content.len() == 0 { return None }

        let scale = if let Some(clamp) = self.scale_clamped {
            clamp
        } else {
            self.scale.unwrap_or(1.0)
        };

        Some(float(content.remove(0))
        .scale(scale)
        .translate(move |bounds, viewport| {
            if let Some(clamp) = self.scale_clamped { 
                bounds.zoom(clamp).offset(&viewport.shrink(get_padding(&self.clamped_padding)))
            } else if let Some(tr) = self.translate {
                Vector::new(tr[0], tr[1])
            } else {
                Vector::ZERO
            }
        }).into())
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum FloatParam {
    Scale,
    ScaleClamped,
    ClampedPadding,
    Translate,
    Show,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Float{
    type Param = FloatParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            FloatParam::Scale => set_t_value(&mut self.scale, value, "FloatParam::Scale"),
            FloatParam::Translate => set_t_value(&mut self.translate, value, "FloatParam::Translate"),
            FloatParam::ScaleClamped => set_t_value(&mut self.scale_clamped, value, "FloatParam::ScaleClamped"),
            FloatParam::ClampedPadding => set_t_value(&mut self.clamped_padding, value, "FloatParam::ClampedPadding"),
            FloatParam::Show => set_t_value(&mut self.show, value, "FloatParam::Show"),
        }
    }
}
