//! ipg_float

use iced::{Element, Length};
use iced::widget::grid;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_height, set_height_fill, set_opt_f32, set_opt_usize};


#[derive(Clone, Debug)]
pub struct IpgGrid {
    pub id: usize,
    pub spacing: Option<f32>,
    pub columns_max_width: Option<f32>,
    pub columns_amount: Option<usize>,
    pub width: Option<f32>,
    pub height_aspect_ratio: Option<f32>,
    pub height_evenly_distribute: Length,
}

impl IpgGrid{
    pub fn construct<'a>(
        &'a self,
        content: Vec<Element<'a, Message>>,
        ) -> Element<'a, Message> {

        let grd = 
            grid(content)
                .spacing(self.spacing.unwrap_or_default())
                .width(self.width.unwrap_or_default());
        
        // columns and fluid equate to the same columns parameter
        let grd = 
            if let Some(amt) = self.columns_amount {
                grd.columns(amt)
            } else if let Some(max) = self.columns_max_width {
                grd.fluid(max)
            } else { grd };

        let grd = if let Some(har) = self.height_aspect_ratio {
            grd.height(har)
        } else {
            grd.height(self.height_evenly_distribute)
        };

        grd.into()
    }
}


#[pyclass]
#[derive(Clone, Debug, PartialEq)]
pub enum GridParam {
    ColumnsAmount,
    ColumnsMaxWidth,
    HeightAspectRatio,
    HeightEvenlyDistribute,
    HeightEvenlyDistributeFill,
    Spacing,
    Width,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgGrid{
    type Param = GridParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            GridParam::ColumnsAmount => set_opt_usize(&mut self.columns_amount, value, "GridParam::ColumnsAmount"),
            GridParam::ColumnsMaxWidth => set_opt_f32(&mut self.columns_max_width, value, "GridParam::ColumnsMaxWidth"),
            GridParam::HeightAspectRatio => set_opt_f32(&mut self.height_aspect_ratio, value, "GridParam::HeightAspectRatio"),
            GridParam::HeightEvenlyDistribute => set_height(&mut self.height_evenly_distribute, value, "GridParam::HeightEvenlyDistribute"),
            GridParam::HeightEvenlyDistributeFill => set_height_fill(&mut self.height_evenly_distribute, value, "GridParam::HeightEvenlyDistributeFill"),
            GridParam::Spacing => set_opt_f32(&mut self.spacing, value, "GridParam::Spacing"),
            GridParam::Width => set_opt_f32(&mut self.width, value, "GridParam::Width"),
        }
    }
}
