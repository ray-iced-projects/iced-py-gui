//! ipg_stack

use iced::Element;
use iced::widget;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::py_api::helpers::get_len;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};

    
#[derive(Debug, Clone)]
pub struct Stack {
    pub id: usize,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub hide_index: Option<usize>,
    pub show: bool,
}

impl <'a> Stack {
    pub fn construct(
        &self,
        mut content: Vec<Element<'a, Message>> 
    ) -> Element<'a, Message> {
        
        content = if self.hide_index.is_some() {
            let index = self.hide_index.unwrap();
            if index >= content.len() {
                panic!("Stack: The hide_index exceeds the number of stack containers.");
            }

            for i in (0..index).rev() {
                content.remove(i);
            }
            content
        } else {
            content
        };
        
        widget::Stack::with_children(content)
                    .width(get_len(self.fill, self.width_fill, self.width))
                    .height(get_len(self.fill, self.height_fill, self.height))
                    .into()

    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum StackParam {
    Fill,
    Height,
    HeightFill,
    HideIndex,
    Show,
    Width,
    WidthFill,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Stack {
    type Param = StackParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            StackParam::Fill => set_t_value(&mut self.fill, value, "StackParam::Fill"),
            StackParam::Height => set_t_value(&mut self.height, value, "StackParam::Height"),
            StackParam::HeightFill => set_t_value(&mut self.height_fill, value, "StackParam::HeightFill"),
            StackParam::HideIndex => set_t_value(&mut self.hide_index, value, "StackParam::HideIndex"),
            StackParam::Show => set_t_value(&mut self.show, value, "StackParam::Show"),
            StackParam::Width => set_t_value(&mut self.width, value, "StackParam::Width"),
            StackParam::WidthFill => set_t_value(&mut self.width_fill, value, "StackParam::WidthFill"),
        }
    }
}
