//! ipg_row

use iced::{Alignment, Element, Length};
use iced::widget;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::py_api::helpers::get_padding;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_height, set_height_fill, set_lengths_fill, 
    set_opt_bool, set_opt_f32, set_opt_vec_f32, set_width, set_width_fill
};


#[derive(Debug, Clone)]
pub struct Row {
    pub id: usize,
    pub show: bool,

    pub spacing: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub width: Length,
    pub height: Length,
    pub align_bottom: Option<bool>,
    pub align_center: Option<bool>,
    pub align_top: Option<bool>,
    pub clip: Option<bool>,
}

impl Row {
    pub fn construct<'a>(
        &self, 
        content: Vec<Element<'a, Message>>,
        ) -> Element<'a, Message> {

        let row = 
            widget::Row::with_children(content)
                .width(self.width)
                .height(self.height);
                            
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

        row.into()
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
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Row {
    type Param = RowParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RowParam::AlignBottom => set_opt_bool(&mut self.align_bottom, value, "AlignBottom"),
            RowParam::AlignCenter => set_opt_bool(&mut self.align_center, value, "AlignCenter"),
            RowParam::AlignTop => set_opt_bool(&mut self.align_top, value, "AlignTop"),
            RowParam::Clip => set_opt_bool(&mut self.clip, value, "Clip"),
            RowParam::Fill => set_lengths_fill(&mut self.width, &mut self.height, value, "Fill"),
            RowParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            RowParam::Width => set_width(&mut self.width, value, "Width"),
            RowParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
            RowParam::Height => set_height(&mut self.height, value, "Height"),
            RowParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            RowParam::Spacing => set_opt_f32(&mut self.spacing, value, "Spacing"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_row() -> Row {
        Row {
            id: 0,
            show: true,
            spacing: None,
            padding: None,
            width: Length::Shrink,
            height: Length::Shrink,
            align_bottom: None,
            align_center: None,
            align_top: None,
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
    fn test_align_bottom() {
        let mut r = make_row();
        r.param_update(RowParam::AlignBottom, &py_obj(true));
        assert_eq!(r.align_bottom, Some(true));
        r.param_update(RowParam::AlignBottom, &py_none());
        assert_eq!(r.align_bottom, None);
    }

    #[test]
    fn test_align_center() {
        let mut r = make_row();
        r.param_update(RowParam::AlignCenter, &py_obj(true));
        assert_eq!(r.align_center, Some(true));
        r.param_update(RowParam::AlignCenter, &py_none());
        assert_eq!(r.align_center, None);
    }

    #[test]
    fn test_align_top() {
        let mut r = make_row();
        r.param_update(RowParam::AlignTop, &py_obj(true));
        assert_eq!(r.align_top, Some(true));
        r.param_update(RowParam::AlignTop, &py_none());
        assert_eq!(r.align_top, None);
    }

    #[test]
    fn test_clip() {
        let mut r = make_row();
        r.param_update(RowParam::Clip, &py_obj(true));
        assert_eq!(r.clip, Some(true));
        r.param_update(RowParam::Clip, &py_none());
        assert_eq!(r.clip, None);
    }

    #[test]
    fn test_fill() {
        let mut c = make_row();
        c.param_update(RowParam::Fill, &py_obj(Some(true)));
        assert_eq!(c.width, Length::Fill);
        assert_eq!(c.height, Length::Fill);
        c.param_update(RowParam::Fill, &py_obj(Some(false)));
        assert_eq!(c.width, Length::Shrink);
        assert_eq!(c.height, Length::Shrink);
        c.param_update(RowParam::Fill, &py_none());
        assert_eq!(c.width, Length::Shrink);
        assert_eq!(c.height, Length::Shrink);
    }

    #[test]
    fn test_padding() {
        let mut r = make_row();
        r.param_update(RowParam::Padding, &py_obj(vec![5.0f32, 10.0]));
        assert_eq!(r.padding, Some(vec![5.0, 10.0]));
        r.param_update(RowParam::Padding, &py_none());
        assert_eq!(r.padding, None);
    }

    #[test]
    fn test_width() {
        let mut r = make_row();
        r.param_update(RowParam::Width, &py_obj(200.0f32));
        assert_eq!(r.width, Length::Fixed(200.0));
    }

    #[test]
    fn test_width_fill() {
        let mut r = make_row();
        r.param_update(RowParam::WidthFill, &py_obj(true));
        assert_eq!(r.width, Length::Fill);
    }

    #[test]
    fn test_height() {
        let mut r = make_row();
        r.param_update(RowParam::Height, &py_obj(100.0f32));
        assert_eq!(r.height, Length::Fixed(100.0));
    }

    #[test]
    fn test_height_fill() {
        let mut r = make_row();
        r.param_update(RowParam::HeightFill, &py_obj(true));
        assert_eq!(r.height, Length::Fill);
    }

    #[test]
    fn test_spacing() {
        let mut r = make_row();
        r.param_update(RowParam::Spacing, &py_obj(8.0f32));
        assert_eq!(r.spacing, Some(8.0));
        r.param_update(RowParam::Spacing, &py_none());
        assert_eq!(r.spacing, None);
    }
}
