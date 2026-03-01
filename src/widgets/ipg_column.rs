//! ipg_column
use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::widgets::enums::IpgAlignment;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,
    set_opt_bool, set_opt_f32, set_opt_vec_f32,
    set_width, set_width_fill, set_height, set_height_fill, set_align,
};

use iced::{Element, Length};
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
    pub align_x: Option<IpgAlignment>,
    pub clip: Option<bool>,
}

pub fn construct_column<'a>(
    ipg_col: &IpgColumn, 
    content: Vec<Element<'a, Message>> 
    ) -> Element<'a, Message> {

    let col = 
        Column::with_children(content)
            .width(ipg_col.width)
            .height(ipg_col.height);
    
    let col = 
    if let Some(align_x) = &ipg_col.align_x {
            col.align_x(align_x.to_iced())
        } else { col };

    let col = 
        if let Some(cp) = ipg_col.clip {
            col.clip(cp)
        } else { col };

    let col = 
            col.padding(get_padding(&ipg_col.padding));

    let col = 
        if let Some(sp) = ipg_col.spacing {
            col.spacing(sp)
        } else { col };

    col.into()

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgColumnParam {
    AlignX,
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
            IpgColumnParam::AlignX     => set_align(&mut self.align_x, value, name),
            IpgColumnParam::Clip       => set_opt_bool(&mut self.clip, value, name),
            IpgColumnParam::Padding    => set_opt_vec_f32(&mut self.padding, value, name),
            IpgColumnParam::Width      => set_width(&mut self.width, value, name),
            IpgColumnParam::WidthFill  => set_width_fill(&mut self.width, value, name),
            IpgColumnParam::Height     => set_height(&mut self.height, value, name),
            IpgColumnParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgColumnParam::Spacing    => set_opt_f32(&mut self.spacing, value, name),
        }
    }
}
