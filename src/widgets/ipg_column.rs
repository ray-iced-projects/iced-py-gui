//! ipg_column
use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_height, set_height_fill, 
    set_opt_bool, set_opt_f32, set_opt_vec_f32, 
    set_width, set_width_fill
};

use iced::{Alignment, Element, Length};
use iced::widget::Column;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgColumn {
    pub id: usize,
    pub show: bool,
    pub spacing: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub width: Length,
    pub height: Length,
    pub max_width: Option<f32>,
    pub align_left: Option<bool>,
    pub align_center: Option<bool>,
    pub align_right: Option<bool>,
    pub clip: Option<bool>,
}

impl IpgColumn {
    pub fn construct<'a>(
        &self,
        content: Vec<Element<'a, Message>> 
        ) -> Element<'a, Message> {

        let col = 
            Column::with_children(content)
                .width(self.width)
                .height(self.height);
        
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

        col.into()

    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgColumnParam {
    AlignLeft,
    AlignCenter,
    AlignRight,
    Clip,
    Height,
    HeightFill,
    Padding,
    Spacing,
    Width,
    WidthFill,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgColumn {
    type Param = IpgColumnParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgColumnParam::AlignLeft => set_opt_bool(&mut self.align_left, value, name),
            IpgColumnParam::AlignCenter => set_opt_bool(&mut self.align_center, value, name),
            IpgColumnParam::AlignRight => set_opt_bool(&mut self.align_right, value, name),
            IpgColumnParam::Clip => set_opt_bool(&mut self.clip, value, name),
            IpgColumnParam::Padding => set_opt_vec_f32(&mut self.padding, value, name),
            IpgColumnParam::Width  => set_width(&mut self.width, value, name),
            IpgColumnParam::WidthFill => set_width_fill(&mut self.width, value, name),
            IpgColumnParam::Height => set_height(&mut self.height, value, name),
            IpgColumnParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgColumnParam::Spacing => set_opt_f32(&mut self.spacing, value, name),
        }
    }
}
