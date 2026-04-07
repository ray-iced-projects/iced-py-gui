//! ipg_stack

use iced::{Element, Length};
use iced::widget;
use pyo3::{pyclass, Py, PyAny};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::widgets::widget_param_update::{WidgetParamUpdate, 
    set_bool, set_height, set_height_fill, set_opt_usize, set_width, set_width_fill};

    
#[derive(Debug, Clone)]
pub struct Stack {
    pub id: usize,
    pub width: Length,
    pub height: Length,
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
                    .width(self.width)
                    .height(self.height)
                    .into()

    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum StackParam {
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
            StackParam::Height => set_height(&mut self.height, value, "Height"),
            StackParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            StackParam::HideIndex => set_opt_usize(&mut self.hide_index, value, "HideIndex"),
            StackParam::Show => set_bool(&mut self.show, value, "Show"),
            StackParam::Width => set_width(&mut self.width, value, "Width"),
            StackParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_stack() -> Stack {
        Stack {
            id: 0,
            width: Length::Shrink,
            height: Length::Shrink,
            hide_index: None,
            show: true,
        }
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::attach(|py| py.None().into_py_any(py).unwrap())
    }

    #[test]
    fn test_height() {
        let mut s = make_stack();
        s.param_update(StackParam::Height, &py_obj(100.0f32));
        assert_eq!(s.height, Length::Fixed(100.0));
    }

    #[test]
    fn test_height_fill() {
        let mut s = make_stack();
        s.param_update(StackParam::HeightFill, &py_obj(true));
        assert_eq!(s.height, Length::Fill);
    }

    #[test]
    fn test_hide_index() {
        let mut s = make_stack();
        s.param_update(StackParam::HideIndex, &py_obj(2usize));
        assert_eq!(s.hide_index, Some(2));
        s.param_update(StackParam::HideIndex, &py_none());
        assert_eq!(s.hide_index, None);
    }

    #[test]
    fn test_show() {
        let mut s = make_stack();
        s.param_update(StackParam::Show, &py_obj(false));
        assert!(!s.show);
    }

    #[test]
    fn test_width() {
        let mut s = make_stack();
        s.param_update(StackParam::Width, &py_obj(200.0f32));
        assert_eq!(s.width, Length::Fixed(200.0));
    }

    #[test]
    fn test_width_fill() {
        let mut s = make_stack();
        s.param_update(StackParam::WidthFill, &py_obj(true));
        assert_eq!(s.width, Length::Fill);
    }
}

