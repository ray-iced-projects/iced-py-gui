//! ipg_row

use iced::{Alignment, Element, alignment};
use iced::widget;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::py_api::helpers::{get_len, get_padding};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};


#[derive(Debug, Clone)]
pub struct Row {
    pub id: usize,
    pub spacing: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub align_bottom: Option<bool>,
    pub align_center: Option<bool>,
    pub align_top: Option<bool>,
    pub clip: Option<bool>,
    pub wrap: Option<bool>,
    pub wrap_vertical_spacing: Option<f32>,
    pub wrap_align_left: Option<bool>,
    pub wrap_align_center: Option<bool>,
    pub wrap_align_right: Option<bool>,
    pub show: bool,
}

impl Row {
    pub fn construct<'a>(
        &self, 
        content: Vec<Element<'a, Message>>,
        ) -> Option<Element<'a, Message>> {

        if !self.show { return None }

        let row = 
            widget::Row::with_children(content)
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height));
                            
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

        let wrap = if self.wrap.is_none() {
            return Some(row.into())
        } else {
            row.wrap()
        };

        let wrap = 
            if let Some(sp) = self.wrap_vertical_spacing {
                wrap.vertical_spacing(sp)
            } else { wrap };

        let align = 
            match (self.wrap_align_center, self.wrap_align_left, self.wrap_align_right) {
                (Some(true), Some(false), Some(false)) => Some(alignment::Horizontal::Center),
                (Some(false), Some(true), Some(false)) => Some(alignment::Horizontal::Left),
                (Some(false), Some(false), Some(true)) => Some(alignment::Horizontal::Right),
                _ => None
            };
            
        let wrap = 
            if let Some(align) = align {
                wrap.align_x(align)
            } else { wrap };

        Some(wrap.into())
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum RowParam {
    AlignBottom,
    AlignCenter,
    AlignTop,
    Clip,
    Fill,
    Height,
    HeightFill,
    Padding,
    Spacing,
    Width,
    WidthFill,
    Show,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Row {
    type Param = RowParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RowParam::AlignBottom => set_t_value(&mut self.align_bottom, value, "RowParam::AlignBottomAlignBottom"),
            RowParam::AlignCenter => set_t_value(&mut self.align_center, value, "RowParam::AlignBottomAlignCenter"),
            RowParam::AlignTop => set_t_value(&mut self.align_top, value, "RowParam::AlignBottomAlignTop"),
            RowParam::Clip => set_t_value(&mut self.clip, value, "RowParam::AlignBottomClip"),
            RowParam::Fill => set_t_value(&mut self.fill, value, "RowParam::AlignBottomFill"),
            RowParam::Padding => set_t_value(&mut self.padding, value, "RowParam::AlignBottomPadding"),
            RowParam::Width => set_t_value(&mut self.width, value, "RowParam::AlignBottomWidth"),
            RowParam::WidthFill => set_t_value(&mut self.width_fill, value, "RowParam::AlignBottomWidthFill"),
            RowParam::Height => set_t_value(&mut self.height, value, "RowParam::AlignBottomHeight"),
            RowParam::HeightFill => set_t_value(&mut self.height_fill, value, "RowParam::AlignBottomHeightFill"),
            RowParam::Spacing => set_t_value(&mut self.spacing, value, "RowParam::AlignBottomSpacing"),
            RowParam::Show => set_t_value(&mut self.show, value, "RowParam::Show"),
        }
    }
}
