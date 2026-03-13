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

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgColumnParam::AlignLeft => set_opt_bool(&mut self.align_left, value, "AlignLeft"),
            IpgColumnParam::AlignCenter => set_opt_bool(&mut self.align_center, value, "AlignCenter"),
            IpgColumnParam::AlignRight => set_opt_bool(&mut self.align_right, value, "AlignRight"),
            IpgColumnParam::Clip => set_opt_bool(&mut self.clip, value, "Clip"),
            IpgColumnParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            IpgColumnParam::Width  => set_width(&mut self.width, value, "Width"),
            IpgColumnParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
            IpgColumnParam::Height => set_height(&mut self.height, value, "Height"),
            IpgColumnParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            IpgColumnParam::Spacing => set_opt_f32(&mut self.spacing, value, "Spacing"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_column() -> IpgColumn {
        IpgColumn {
            id: 0,
            show: true,
            spacing: None,
            padding: None,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: None,
            align_left: None,
            align_center: None,
            align_right: None,
            clip: None,
        }
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::with_gil(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::with_gil(|py| py.None().into_py_any(py).unwrap())
    }

    #[test]
    fn test_align_left() {
        let mut col = make_column();
        col.param_update(IpgColumnParam::AlignLeft, &py_obj(true));
        assert_eq!(col.align_left, Some(true));
        col.param_update(IpgColumnParam::AlignLeft, &py_none());
        assert_eq!(col.align_left, None);
    }

    #[test]
    fn test_align_center() {
        let mut col = make_column();
        col.param_update(IpgColumnParam::AlignCenter, &py_obj(true));
        assert_eq!(col.align_center, Some(true));
        col.param_update(IpgColumnParam::AlignCenter, &py_none());
        assert_eq!(col.align_center, None);
    }

    #[test]
    fn test_align_right() {
        let mut col = make_column();
        col.param_update(IpgColumnParam::AlignRight, &py_obj(true));
        assert_eq!(col.align_right, Some(true));
        col.param_update(IpgColumnParam::AlignRight, &py_none());
        assert_eq!(col.align_right, None);
    }

    #[test]
    fn test_clip() {
        let mut col = make_column();
        col.param_update(IpgColumnParam::Clip, &py_obj(true));
        assert_eq!(col.clip, Some(true));
        col.param_update(IpgColumnParam::Clip, &py_none());
        assert_eq!(col.clip, None);
    }

    #[test]
    fn test_height() {
        let mut col = make_column();
        col.param_update(IpgColumnParam::Height, &py_obj(100.0f32));
        assert_eq!(col.height, Length::Fixed(100.0));
    }

    #[test]
    fn test_height_fill() {
        let mut col = make_column();
        col.param_update(IpgColumnParam::HeightFill, &py_obj(true));
        assert_eq!(col.height, Length::Fill);
    }

    #[test]
    fn test_padding() {
        let mut col = make_column();
        col.param_update(IpgColumnParam::Padding, &py_obj(vec![5.0f32, 10.0]));
        assert_eq!(col.padding, Some(vec![5.0, 10.0]));
        col.param_update(IpgColumnParam::Padding, &py_none());
        assert_eq!(col.padding, None);
    }

    #[test]
    fn test_spacing() {
        let mut col = make_column();
        col.param_update(IpgColumnParam::Spacing, &py_obj(8.0f32));
        assert_eq!(col.spacing, Some(8.0));
        col.param_update(IpgColumnParam::Spacing, &py_none());
        assert_eq!(col.spacing, None);
    }

    #[test]
    fn test_width() {
        let mut col = make_column();
        col.param_update(IpgColumnParam::Width, &py_obj(200.0f32));
        assert_eq!(col.width, Length::Fixed(200.0));
    }

    #[test]
    fn test_width_fill() {
        let mut col = make_column();
        col.param_update(IpgColumnParam::WidthFill, &py_obj(true));
        assert_eq!(col.width, Length::Fill);
    }
}
