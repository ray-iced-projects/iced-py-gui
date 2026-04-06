//! ipg_row

use iced::{Alignment, Element, Length};
use iced::widget::Row;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app::Message;

use crate::py_api::helpers::get_padding;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_height, set_height_fill, set_lengths_fill, 
    set_opt_bool, set_opt_f32, set_opt_vec_f32, set_width, set_width_fill
};


#[derive(Debug, Clone)]
pub struct IpgRow {
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

impl IpgRow {
    pub fn construct<'a>(
        &self, 
        content: Vec<Element<'a, Message>>,
        ) -> Element<'a, Message> {

        let row = 
            Row::with_children(content)
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


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRowParam {
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

impl WidgetParamUpdate for IpgRow {
    type Param = IpgRowParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgRowParam::AlignBottom => set_opt_bool(&mut self.align_bottom, value, "AlignBottom"),
            IpgRowParam::AlignCenter => set_opt_bool(&mut self.align_center, value, "AlignCenter"),
            IpgRowParam::AlignTop => set_opt_bool(&mut self.align_top, value, "AlignTop"),
            IpgRowParam::Clip => set_opt_bool(&mut self.clip, value, "Clip"),
            IpgRowParam::Fill => set_lengths_fill(&mut self.width, &mut self.height, value, "Fill"),
            IpgRowParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            IpgRowParam::Width => set_width(&mut self.width, value, "Width"),
            IpgRowParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
            IpgRowParam::Height => set_height(&mut self.height, value, "Height"),
            IpgRowParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            IpgRowParam::Spacing => set_opt_f32(&mut self.spacing, value, "Spacing"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_row() -> IpgRow {
        IpgRow {
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
        r.param_update(IpgRowParam::AlignBottom, &py_obj(true));
        assert_eq!(r.align_bottom, Some(true));
        r.param_update(IpgRowParam::AlignBottom, &py_none());
        assert_eq!(r.align_bottom, None);
    }

    #[test]
    fn test_align_center() {
        let mut r = make_row();
        r.param_update(IpgRowParam::AlignCenter, &py_obj(true));
        assert_eq!(r.align_center, Some(true));
        r.param_update(IpgRowParam::AlignCenter, &py_none());
        assert_eq!(r.align_center, None);
    }

    #[test]
    fn test_align_top() {
        let mut r = make_row();
        r.param_update(IpgRowParam::AlignTop, &py_obj(true));
        assert_eq!(r.align_top, Some(true));
        r.param_update(IpgRowParam::AlignTop, &py_none());
        assert_eq!(r.align_top, None);
    }

    #[test]
    fn test_clip() {
        let mut r = make_row();
        r.param_update(IpgRowParam::Clip, &py_obj(true));
        assert_eq!(r.clip, Some(true));
        r.param_update(IpgRowParam::Clip, &py_none());
        assert_eq!(r.clip, None);
    }

    #[test]
    fn test_fill() {
        let mut c = make_row();
        c.param_update(IpgRowParam::Fill, &py_obj(Some(true)));
        assert_eq!(c.width, Length::Fill);
        assert_eq!(c.height, Length::Fill);
        c.param_update(IpgRowParam::Fill, &py_obj(Some(false)));
        assert_eq!(c.width, Length::Shrink);
        assert_eq!(c.height, Length::Shrink);
        c.param_update(IpgRowParam::Fill, &py_none());
        assert_eq!(c.width, Length::Shrink);
        assert_eq!(c.height, Length::Shrink);
    }

    #[test]
    fn test_padding() {
        let mut r = make_row();
        r.param_update(IpgRowParam::Padding, &py_obj(vec![5.0f32, 10.0]));
        assert_eq!(r.padding, Some(vec![5.0, 10.0]));
        r.param_update(IpgRowParam::Padding, &py_none());
        assert_eq!(r.padding, None);
    }

    #[test]
    fn test_width() {
        let mut r = make_row();
        r.param_update(IpgRowParam::Width, &py_obj(200.0f32));
        assert_eq!(r.width, Length::Fixed(200.0));
    }

    #[test]
    fn test_width_fill() {
        let mut r = make_row();
        r.param_update(IpgRowParam::WidthFill, &py_obj(true));
        assert_eq!(r.width, Length::Fill);
    }

    #[test]
    fn test_height() {
        let mut r = make_row();
        r.param_update(IpgRowParam::Height, &py_obj(100.0f32));
        assert_eq!(r.height, Length::Fixed(100.0));
    }

    #[test]
    fn test_height_fill() {
        let mut r = make_row();
        r.param_update(IpgRowParam::HeightFill, &py_obj(true));
        assert_eq!(r.height, Length::Fill);
    }

    #[test]
    fn test_spacing() {
        let mut r = make_row();
        r.param_update(IpgRowParam::Spacing, &py_obj(8.0f32));
        assert_eq!(r.spacing, Some(8.0));
        r.param_update(IpgRowParam::Spacing, &py_none());
        assert_eq!(r.spacing, None);
    }
}
