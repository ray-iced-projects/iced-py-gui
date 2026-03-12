//! ipg_stack
use iced::{Element, Length};
use iced::widget::Stack;
use pyo3::{pyclass, Py, PyAny};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::widgets::widget_param_update::{WidgetParamUpdate, 
    set_bool, set_height, set_opt_usize, set_width};

    
#[derive(Debug, Clone)]
pub struct IpgStack {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub hide_index: Option<usize>,
    pub show: bool,
}

impl <'a> IpgStack {
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
        
        Stack::with_children(content)
                    .width(self.width)
                    .height(self.height)
                    .into()

    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgStackParam {
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

impl WidgetParamUpdate for IpgStack {
    type Param = IpgStackParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgStackParam::Height => set_height(&mut self.height, value, "Height"),
            IpgStackParam::HeightFill => set_height(&mut self.height, value, "HeightFill"),
            IpgStackParam::HideIndex => set_opt_usize(&mut self.hide_index, value, "HideIndex"),
            IpgStackParam::Show => set_bool(&mut self.show, value, "Show"),
            IpgStackParam::Width => set_width(&mut self.width, value, "Width"),
            IpgStackParam::WidthFill => set_width(&mut self.width, value, "WidthFill"),
        }
    }
}

