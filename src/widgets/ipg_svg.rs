//! ipg_svg

use crate::app::Message;
use crate::widgets::enums::ContentFit;
use crate::widgets::enums::Rotation;
use crate::widgets::widget_param_update::WidgetParamUpdate;
use crate::widgets::widget_param_update::set_bool;
use crate::widgets::widget_param_update::set_height;
use crate::widgets::widget_param_update::set_opt_f32;
use crate::widgets::widget_param_update::set_opt_iced_color;
use crate::widgets::widget_param_update::set_string;
use crate::widgets::widget_param_update::set_width;

use iced::{Length, Element};
use iced::widget;
use iced::advanced::svg;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct Svg {
        pub id: usize,
        pub svg_path: String,
        pub width: Length,
        pub height: Length,
        pub color_filter: Option<iced::Color>,
        pub content_fit: Option<ContentFit>,
        pub rotation_type: Option<Rotation>,
        pub rotation_radians: Option<f32>,
        pub opacity: Option<f32>,
        pub show: bool,
}

impl Svg{
    pub fn construct(&self) 
        -> Option<Element<'_, Message>> {

        if !self.show {
            return None
        }

        let svg_handle = svg::Handle::from_path(self.svg_path.clone());

        let svg = widget::Svg::new(svg_handle)
                    .width(self.width)
                    .height(self.height);

        let svg = if let Some(cf) = &self.content_fit {
            svg.content_fit(cf.to_iced())
        } else { svg };

        let svg = if let Some(rt) = &self.rotation_type {
            svg.rotation(rt.to_iced(self.rotation_radians))
        } else { svg };

        let svg = if let Some(op) = self.opacity {
            svg.opacity(op)
        } else { svg }.into();

        Some(svg)

    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SvgParam {
    ColorFilter,
    ContentFit,
    Height,
    Opacity,
    RotationRadians,
    RotationType,
    Show,
    SvgPath,
    Width,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Svg {
    type Param = SvgParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SvgParam::ColorFilter => set_opt_iced_color(&mut self.color_filter, value, "ColorFilter"),
            SvgParam::ContentFit => self.content_fit = ContentFit::extract(value),
            SvgParam::Height => set_height(&mut self.height, value, "Height"),
            SvgParam::Opacity => set_opt_f32(&mut self.opacity, value, "Opacity"),
            SvgParam::RotationRadians => set_opt_f32(&mut self.rotation_radians, value, "RotationRadians"),
            SvgParam::RotationType => self.rotation_type = Rotation::extract(value),
            SvgParam::Show => set_bool(&mut self.show, value, "Show"),
            SvgParam::SvgPath => set_string(&mut self.svg_path, value, "SvgPath"),
            SvgParam::Width => set_width(&mut self.width, value, "Width"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_svg() -> Svg {
        Svg {
            id: 0,
            svg_path: "test.svg".to_string(),
            width: Length::Shrink,
            height: Length::Shrink,
            color_filter: None,
            content_fit: None,
            rotation_type: None,
            rotation_radians: None,
            opacity: None,
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
        let mut s = make_svg();
        s.param_update(SvgParam::Height, &py_obj(100.0f32));
        assert_eq!(s.height, Length::Fixed(100.0));
    }

    #[test]
    fn test_opacity() {
        let mut s = make_svg();
        s.param_update(SvgParam::Opacity, &py_obj(0.5f32));
        assert_eq!(s.opacity, Some(0.5));
        s.param_update(SvgParam::Opacity, &py_none());
        assert_eq!(s.opacity, None);
    }

    #[test]
    fn test_rotation_radians() {
        let mut s = make_svg();
        s.param_update(SvgParam::RotationRadians, &py_obj(1.57f32));
        assert_eq!(s.rotation_radians, Some(1.57));
    }

    #[test]
    fn test_show() {
        let mut s = make_svg();
        s.param_update(SvgParam::Show, &py_obj(false));
        assert!(!s.show);
    }

    #[test]
    fn test_svg_path() {
        let mut s = make_svg();
        s.param_update(SvgParam::SvgPath, &py_obj("new.svg".to_string()));
        assert_eq!(s.svg_path, "new.svg");
    }

    #[test]
    fn test_width() {
        let mut s = make_svg();
        s.param_update(SvgParam::Width, &py_obj(200.0f32));
        assert_eq!(s.width, Length::Fixed(200.0));
    }
}

