//! ipg_column
use crate::app::Message;
use crate::py_api::helpers::{get_len, get_padding};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};

use iced::{Alignment, Element};
use iced::widget;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Column {
    pub id: usize,
    pub show: bool,
    pub spacing: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub max_width: Option<f32>,
    pub align_left: Option<bool>,
    pub align_center: Option<bool>,
    pub align_right: Option<bool>,
    pub clip: Option<bool>,
}

impl Column {
    pub fn construct<'a>(
        &self,
        content: Vec<Element<'a, Message>> 
        ) -> Element<'a, Message> {

        let col = 
            widget::Column::with_children(content)
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height));
        
        let col = 
            if self.align_left ==  Some(true) {
                    col.align_x(Alignment::Start)
                } else { col };

        let col = 
            if self.align_center ==  Some(true) {
                    col.align_x(Alignment::Center)
                } else { col };

        let col = 
            if self.align_right ==  Some(true) {
                    col.align_x(Alignment::End)
                } else { col };

        let col = 
            if let Some(cp) = self.clip {
                col.clip(cp)
            } else { col };

        let col = 
                col.padding(get_padding(&self.padding));

        let col = 
            if let Some(sp) = self.spacing {
                col.spacing(sp)
            } else { col };

        let col =
            if let Some(mw) = self.max_width {
                col.max_width(mw)
            } else { col };

        col.into()

    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ColumnParam {
    AlignLeft,
    AlignCenter,
    AlignRight,
    Clip,
    Fill,
    Height,
    HeightFill,
    MaxWidth,
    Padding,
    Spacing,
    Width,
    WidthFill,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Column {
    type Param = ColumnParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ColumnParam::AlignCenter => set_t_value(&mut self.align_center, value, "ColumnParam::AlignCenter"),
            ColumnParam::AlignLeft => set_t_value(&mut self.align_left, value, "ColumnParam::AlignLeft"),
            ColumnParam::AlignRight => set_t_value(&mut self.align_right, value, "ColumnParam::AlignRight"),
            ColumnParam::Clip => set_t_value(&mut self.clip, value, "ColumnParam::Clip"),
            ColumnParam::Fill => set_t_value(&mut self.fill, value, "ColumnParam::Fill"),
            ColumnParam::Height => set_t_value(&mut self.height, value, "ColumnParam::Height"),
            ColumnParam::HeightFill => set_t_value(&mut self.height_fill, value, "ColumnParam::HeightFill"),
            ColumnParam::MaxWidth => set_t_value(&mut self.max_width, value, "ColumnParam::MaxWidth"),
            ColumnParam::Padding => set_t_value(&mut self.padding, value, "ColumnParam::Padding"),
            ColumnParam::Spacing => set_t_value(&mut self.spacing, value, "ColumnParam::Spacing"),
            ColumnParam::Width  => set_t_value(&mut self.width, value, "ColumnParam::Width"),
            ColumnParam::WidthFill => set_t_value(&mut self.width_fill, value, "ColumnParam::WidthFill"),
        }
    }
}
