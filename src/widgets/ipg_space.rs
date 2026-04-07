//! ipg_space

use iced::{Element, Length};
use iced::widget;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_height, set_height_fill, 
    set_width, set_width_fill};


#[derive(Debug, Clone)]
pub struct Space {
    pub id: usize,
    pub parent_id: String,
    pub width: Length,
    pub height: Length,
    pub show: bool,
}

pub fn construct_space(sp: &Space) -> Option<Element<'_, Message>> {

    if sp.show {
        Some(widget::Space::new()
            .width(sp.width)
            .height(sp.height).into())
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum SpaceParam {
    Height,
    HeightFill,
    Width,
    WidthFill,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Space {
    type Param = SpaceParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SpaceParam::Width => set_width(&mut self.width, value, "Width"),
            SpaceParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
            SpaceParam::Height => set_height(&mut self.height, value, "Height"),
            SpaceParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_space() -> Space {
        Space {
            id: 0,
            parent_id: String::new(),
            width: Length::Shrink,
            height: Length::Shrink,
            show: true,
        }
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    #[test]
    fn test_width() {
        let mut s = make_space();
        s.param_update(SpaceParam::Width, &py_obj(50.0f32));
        assert_eq!(s.width, Length::Fixed(50.0));
    }

    #[test]
    fn test_width_fill() {
        let mut s = make_space();
        s.param_update(SpaceParam::WidthFill, &py_obj(true));
        assert_eq!(s.width, Length::Fill);
    }

    #[test]
    fn test_height() {
        let mut s = make_space();
        s.param_update(SpaceParam::Height, &py_obj(30.0f32));
        assert_eq!(s.height, Length::Fixed(30.0));
    }

    #[test]
    fn test_height_fill() {
        let mut s = make_space();
        s.param_update(SpaceParam::HeightFill, &py_obj(true));
        assert_eq!(s.height, Length::Fill);
    }
}
