//! ipg_space

use iced::{Element, Length};
use iced::widget::Space;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_height, set_height_fill, 
    set_width, set_width_fill};


#[derive(Debug, Clone)]
pub struct IpgSpace {
    pub id: usize,
    pub parent_id: String,
    pub width: Length,
    pub height: Length,
    pub show: bool,
}

pub fn construct_space(sp: &IpgSpace) -> Option<Element<'_, Message>> {

    if sp.show {
        Some(Space::new()
            .width(sp.width)
            .height(sp.height).into())
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSpaceParam {
    Height,
    HeightFill,
    Width,
    WidthFill,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgSpace {
    type Param = IpgSpaceParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgSpaceParam::Width => set_width(&mut self.width, value, "Width"),
            IpgSpaceParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
            IpgSpaceParam::Height => set_height(&mut self.height, value, "Height"),
            IpgSpaceParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_space() -> IpgSpace {
        IpgSpace {
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
        s.param_update(IpgSpaceParam::Width, &py_obj(50.0f32));
        assert_eq!(s.width, Length::Fixed(50.0));
    }

    #[test]
    fn test_width_fill() {
        let mut s = make_space();
        s.param_update(IpgSpaceParam::WidthFill, &py_obj(true));
        assert_eq!(s.width, Length::Fill);
    }

    #[test]
    fn test_height() {
        let mut s = make_space();
        s.param_update(IpgSpaceParam::Height, &py_obj(30.0f32));
        assert_eq!(s.height, Length::Fixed(30.0));
    }

    #[test]
    fn test_height_fill() {
        let mut s = make_space();
        s.param_update(IpgSpaceParam::HeightFill, &py_obj(true));
        assert_eq!(s.height, Length::Fill);
    }
}
