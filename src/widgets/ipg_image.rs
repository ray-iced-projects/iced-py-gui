//!ipg_image

use crate::app;
use crate::widgets::enums::IpgContentFit;
use crate::widgets::enums::IpgColorFilter;
use crate::widgets::enums::IpgRotation;
use crate::widgets::widget_param_update::set_bool;
use crate::widgets::widget_param_update::set_opt_f32;
use crate::widgets::widget_param_update::set_string;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, 
};


use iced::Element;
use iced::widget::Image;
use iced::advanced::image;

use pyo3::pyclass;
use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgImage {
        pub id: usize,
        pub parent_id: String,
        pub image_path: String,
        pub content_fit: Option<IpgContentFit>,
        pub filter_method: Option<IpgColorFilter>,
        pub rotation_method: Option<IpgRotation>,
        pub rotation_radians: Option<f32>,
        pub opacity: Option<f32>,
        pub show: bool,
}

impl IpgImage {
    pub fn construct(&self)
        -> Option<Element<'_, app::Message>> {

        if !self.show {
            return None
        }

        let fit = if let Some(f) = &self.content_fit {
            f.to_iced()
        } else {
            IpgContentFit::default().to_iced()
        };

        let filter = if let Some(f) = &self.filter_method {
            f.to_iced()
        } else {
            IpgColorFilter::default().to_iced()
        };

        let rotation = if let Some(r) = &self.rotation_method {
            r.to_iced(self.rotation_radians)
        } else {
            IpgRotation::default()
        };

        let op = if let Some(op) = self.opacity {
            op
        } else { 1.0 };

        let img = 
            Image::<image::Handle>::new(self.image_path.clone())
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
pub enum IpgImageParam {
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

impl WidgetParamUpdate for IpgImage {
    type Param = IpgImageParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgImageParam::ContentFit => self.content_fit = IpgContentFit::extract(value),
            IpgImageParam::FilterMethod => self.filter_method = IpgColorFilter::extract(value),
            IpgImageParam::ImagePath => set_string(&mut self.image_path, value, "IpgImageParam::ImagePath"),
            IpgImageParam::RotationMethod => self.rotation_method = IpgRotation::extract(value),
            IpgImageParam::RotationRadians => set_opt_f32(&mut self.rotation_radians, value, "IpgImageParam::RotationRadians"),
            IpgImageParam::Show => set_bool(&mut self.show, value, "IpgImageParam::Show"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_image() -> IpgImage {
        IpgImage {
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
        img.param_update(IpgImageParam::ImagePath, &py_obj("new.png".to_string()));
        assert_eq!(img.image_path, "new.png");
    }

    #[test]
    fn test_rotation_radians() {
        let mut img = make_image();
        img.param_update(IpgImageParam::RotationRadians, &py_obj(1.57f32));
        assert_eq!(img.rotation_radians, Some(1.57));
        img.param_update(IpgImageParam::RotationRadians, &py_none());
        assert_eq!(img.rotation_radians, None);
    }

    #[test]
    fn test_show() {
        let mut img = make_image();
        img.param_update(IpgImageParam::Show, &py_obj(false));
        assert!(!img.show);
    }

}


