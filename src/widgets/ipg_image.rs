//!ipg_image

use crate::app;
use crate::widgets::enums::ContentFit;
use crate::widgets::enums::ColorFilter;
use crate::widgets::enums::Rotation;
use crate::widgets::widget_param_update::set_bool;
use crate::widgets::widget_param_update::set_opt_f32;
use crate::widgets::widget_param_update::set_string;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, 
};


use iced::Element;
use iced::widget;
use iced::advanced::image;

use pyo3::pyclass;
use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Image {
        pub id: usize,
        pub parent_id: String,
        pub image_path: String,
        pub content_fit: Option<ContentFit>,
        pub filter_method: Option<ColorFilter>,
        pub rotation_method: Option<Rotation>,
        pub rotation_radians: Option<f32>,
        pub opacity: Option<f32>,
        pub show: bool,
}

impl Image {
    pub fn construct(&self)
        -> Option<Element<'_, app::Message>> {

        if !self.show {
            return None
        }

        let fit = if let Some(f) = &self.content_fit {
            f.to_iced()
        } else {
            ContentFit::default().to_iced()
        };

        let filter = if let Some(f) = &self.filter_method {
            f.to_iced()
        } else {
            ColorFilter::default().to_iced()
        };

        let rotation = if let Some(r) = &self.rotation_method {
            r.to_iced(self.rotation_radians)
        } else {
            iced::Rotation::default()
        };

        let op = if let Some(op) = self.opacity {
            op
        } else { 1.0 };

        let img = 
            widget::Image::<image::Handle>::new(self.image_path.clone())
                .content_fit(fit)
                .filter_method(filter)
                .rotation(rotation)
                .opacity(op)
                .into();

        Some(img)

    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum ImageParam {
    ContentFit,
    FilterMethod,
    ImagePath,
    RotationMethod,
    RotationRadians,
    Show,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Image {
    type Param = ImageParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ImageParam::ContentFit => self.content_fit = ContentFit::extract(value),
            ImageParam::FilterMethod => self.filter_method = ColorFilter::extract(value),
            ImageParam::ImagePath => set_string(&mut self.image_path, value, "ImageParam::ImagePath"),
            ImageParam::RotationMethod => self.rotation_method = Rotation::extract(value),
            ImageParam::RotationRadians => set_opt_f32(&mut self.rotation_radians, value, "ImageParam::RotationRadians"),
            ImageParam::Show => set_bool(&mut self.show, value, "ImageParam::Show"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_image() -> Image {
        Image {
            id: 0,
            parent_id: String::new(),
            image_path: "test.png".to_string(),
            content_fit: None,
            filter_method: None,
            rotation_method: None,
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
    fn test_image_path() {
        let mut img = make_image();
        img.param_update(ImageParam::ImagePath, &py_obj("new.png".to_string()));
        assert_eq!(img.image_path, "new.png");
    }

    #[test]
    fn test_rotation_radians() {
        let mut img = make_image();
        img.param_update(ImageParam::RotationRadians, &py_obj(1.57f32));
        assert_eq!(img.rotation_radians, Some(1.57));
        img.param_update(ImageParam::RotationRadians, &py_none());
        assert_eq!(img.rotation_radians, None);
    }

    #[test]
    fn test_show() {
        let mut img = make_image();
        img.param_update(ImageParam::Show, &py_obj(false));
        assert!(!img.show);
    }

}


