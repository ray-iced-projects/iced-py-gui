//! ipg_column
use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_height, set_height_fill, set_lengths_fill, 
    set_opt_bool, set_opt_f32, set_opt_vec_f32, set_width, set_width_fill
};

use iced::{Alignment, Element, Length};
use iced::widget;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Column {
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

impl Column {
    pub fn construct<'a>(
        &self,
        content: Vec<Element<'a, Message>> 
        ) -> Element<'a, Message> {

        let col = 
            widget::Column::with_children(content)
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
pub enum ColumnParam {
    AlignLeft,
    AlignCenter,
    AlignRight,
    Clip,
    Fill,
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

impl WidgetParamUpdate for Column {
    type Param = ColumnParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ColumnParam::AlignLeft => set_opt_bool(&mut self.align_left, value, "AlignLeft"),
            ColumnParam::AlignCenter => set_opt_bool(&mut self.align_center, value, "AlignCenter"),
            ColumnParam::AlignRight => set_opt_bool(&mut self.align_right, value, "AlignRight"),
            ColumnParam::Clip => set_opt_bool(&mut self.clip, value, "Clip"),
            ColumnParam::Fill => set_lengths_fill(&mut self.width, &mut self.height, value, "Fill"),
            ColumnParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            ColumnParam::Width  => set_width(&mut self.width, value, "Width"),
            ColumnParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
            ColumnParam::Height => set_height(&mut self.height, value, "Height"),
            ColumnParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            ColumnParam::Spacing => set_opt_f32(&mut self.spacing, value, "Spacing"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_column() -> Column {
        Column {
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
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::attach(|py| py.None().into_py_any(py).unwrap())
    }

    #[test]
    fn test_align_left() {
        let mut col = make_column();
        col.param_update(ColumnParam::AlignLeft, &py_obj(true));
        assert_eq!(col.align_left, Some(true));
        col.param_update(ColumnParam::AlignLeft, &py_none());
        assert_eq!(col.align_left, None);
    }

    #[test]
    fn test_align_center() {
        let mut col = make_column();
        col.param_update(ColumnParam::AlignCenter, &py_obj(true));
        assert_eq!(col.align_center, Some(true));
        col.param_update(ColumnParam::AlignCenter, &py_none());
        assert_eq!(col.align_center, None);
    }

    #[test]
    fn test_align_right() {
        let mut col = make_column();
        col.param_update(ColumnParam::AlignRight, &py_obj(true));
        assert_eq!(col.align_right, Some(true));
        col.param_update(ColumnParam::AlignRight, &py_none());
        assert_eq!(col.align_right, None);
    }

    #[test]
    fn test_clip() {
        let mut col = make_column();
        col.param_update(ColumnParam::Clip, &py_obj(true));
        assert_eq!(col.clip, Some(true));
        col.param_update(ColumnParam::Clip, &py_none());
        assert_eq!(col.clip, None);
    }

    #[test]
    fn test_fill() {
        let mut c = make_column();
        c.param_update(ColumnParam::Fill, &py_obj(Some(true)));
        assert_eq!(c.width, Length::Fill);
        assert_eq!(c.height, Length::Fill);
        c.param_update(ColumnParam::Fill, &py_obj(Some(false)));
        assert_eq!(c.width, Length::Shrink);
        assert_eq!(c.height, Length::Shrink);
        c.param_update(ColumnParam::Fill, &py_none());
        assert_eq!(c.width, Length::Shrink);
        assert_eq!(c.height, Length::Shrink);
    }

    #[test]
    fn test_height() {
        let mut col = make_column();
        col.param_update(ColumnParam::Height, &py_obj(100.0f32));
        assert_eq!(col.height, Length::Fixed(100.0));
    }

    #[test]
    fn test_height_fill() {
        let mut col = make_column();
        col.param_update(ColumnParam::HeightFill, &py_obj(true));
        assert_eq!(col.height, Length::Fill);
    }

    #[test]
    fn test_padding() {
        let mut col = make_column();
        col.param_update(ColumnParam::Padding, &py_obj(vec![5.0f32, 10.0]));
        assert_eq!(col.padding, Some(vec![5.0, 10.0]));
        col.param_update(ColumnParam::Padding, &py_none());
        assert_eq!(col.padding, None);
    }

    #[test]
    fn test_spacing() {
        let mut col = make_column();
        col.param_update(ColumnParam::Spacing, &py_obj(8.0f32));
        assert_eq!(col.spacing, Some(8.0));
        col.param_update(ColumnParam::Spacing, &py_none());
        assert_eq!(col.spacing, None);
    }

    #[test]
    fn test_width() {
        let mut col = make_column();
        col.param_update(ColumnParam::Width, &py_obj(200.0f32));
        assert_eq!(col.width, Length::Fixed(200.0));
    }

    #[test]
    fn test_width_fill() {
        let mut col = make_column();
        col.param_update(ColumnParam::WidthFill, &py_obj(true));
        assert_eq!(col.width, Length::Fill);
    }
}
