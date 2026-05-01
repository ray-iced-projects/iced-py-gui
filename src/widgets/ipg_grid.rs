//! ipg_float

use iced::Element;
use iced::widget::grid;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::py_api::helpers::get_len;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};


#[derive(Clone, Debug)]
pub struct Grid {
    pub id: usize,
    pub width: f32,
    pub spacing: Option<f32>,
    pub columns_max_width: Option<f32>,
    pub columns_amount: Option<usize>,
    pub height_aspect_ratio: Option<f32>,
    pub height_evenly_distribute: Option<f32>,
    pub height_evenly_distribute_fill: Option<bool>,
    pub show: bool,
}

impl Grid{
    pub fn construct<'a>(
        &'a self,
        content: Vec<Element<'a, Message>>,
        ) -> Option<Element<'a, Message>> {

        if !self.show { return None }

        let grd = 
            grid(content)
                .spacing(self.spacing.unwrap_or_default())
                .width(self.width)
                .height(get_len(None, 
                    self.height_evenly_distribute_fill, 
                    self.height_evenly_distribute));
        
        // columns and fluid equate to the same columns parameter
        let grd = 
            if let Some(amt) = self.columns_amount {
                grd.columns(amt)
            } else if let Some(max) = self.columns_max_width {
                grd.fluid(max)
            } else { grd };

        Some(grd.into())
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum GridParam {
    ColumnsAmount,
    ColumnsMaxWidth,
    HeightAspectRatio,
    HeightEvenlyDistribute,
    HeightEvenlyDistributeFill,
    Show,
    Spacing,
    Width,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Grid{
    type Param = GridParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            GridParam::ColumnsAmount => set_t_value(&mut self.columns_amount, value, "GridParam::ColumnsAmount"),
            GridParam::ColumnsMaxWidth => set_t_value(&mut self.columns_max_width, value, "GridParam::ColumnsMaxWidth"),
            GridParam::HeightAspectRatio => set_t_value(&mut self.height_aspect_ratio, value, "GridParam::HeightAspectRatio"),
            GridParam::HeightEvenlyDistribute => set_t_value(&mut self.height_evenly_distribute, value, "GridParam::HeightEvenlyDistribute"),
            GridParam::HeightEvenlyDistributeFill => set_t_value(&mut self.height_evenly_distribute, value, "GridParam::HeightEvenlyDistributeFill"),
            GridParam::Show => set_t_value(&mut self.show, value, "GridParam::Show"),
            GridParam::Spacing => set_t_value(&mut self.spacing, value, "GridParam::Spacing"),
            GridParam::Width => set_t_value(&mut self.width, value, "GridParam::Width"),
        }
    }
}
