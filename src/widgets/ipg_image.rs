//!ipg_image

use crate::app;
use crate::py_api::helpers::{get_len, get_radius};
use crate::widgets::enums::{ContentFit, FilterMethod, Rotation};
use crate::widgets::widget_param_update::set_t_value;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, 
};


use iced::Element;
use iced::Rectangle;
use iced::widget;
use iced::advanced::image;

use pyo3::pyclass;
use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Image {
        pub id: usize,
        pub path: String,
        pub width: Option<f32>,
        pub width_fill: Option<bool>,
        pub height: Option<f32>,
        pub height_fill: Option<bool>,
        pub fill: Option<bool>,
        pub crop_x: Option<u32>,
        pub crop_y: Option<u32>,
        pub crop_width: Option<u32>,
        pub crop_height: Option<u32>,
        pub border_radius: Option<Vec<f32>>,
        pub content_fit: Option<ContentFit>,
        pub filter_method: Option<FilterMethod>,
        pub rotation_method: Option<Rotation>,
        pub rotation_radians: Option<f32>,
        pub rotation_degrees: Option<f32>,
        pub opacity: Option<f32>,
        pub scale: Option<f32>,
        pub expand: Option<bool>,
        pub show: bool,
}

impl Image {
    pub fn construct(&self)
        -> Option<Element<'_, app::Message>> {

        if !self.show {
            return None
        }

        let img: widget::Image<image::Handle> = 
            widget::Image::<image::Handle>::new(self.path.clone())
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height))
                .into();

        let img = if let (Some(w), Some(h)) = 
            (self.crop_width, self.crop_height) 
        {
            let x = self.crop_x.unwrap_or(0);
            let y = self.crop_y.unwrap_or(0);
            img.crop(Rectangle { x, y, width: w, height: h })
        } else {
            eprint!("[WARNING] Either crop_width or crop_height image cropping is a None value so the cropping was not performed");
            img
        };

        let img = if let Some(br) = &self.border_radius {
            img.border_radius(get_radius(br, "image".to_string()))
        } else { img };

        let img = if let Some(f) = &self.content_fit {
            img.content_fit(f.to_iced())
        } else { img };

        let img = if let Some(fm) = &self.filter_method {
            img.filter_method(fm.to_iced())
        } else { img };

        let img = if let Some(r) = &self.rotation_method {
            if let Some(d) = self.rotation_degrees {
                img.rotation(r.to_iced(Some(d.to_radians())))
            } else if let Some(rad) = self.rotation_radians {
                img.rotation(r.to_iced(Some(rad)))
            } else {
                eprint!("[WARNING] Rotation not performed since radians or degrees were not supplied");
                img
            }
        } else { img };

        let img = if let Some(op) = self.opacity {
            img.opacity(op)
        } else { img };
                
        let img = if let Some(sc) = self.scale {
            img.scale(sc)
        } else { img };

        let img = if let Some(ex) = self.expand {
            img.expand(ex)
        } else { img };

        Some(img.into())

    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ImageParam {
    BorderRadius,
    ContentFit,
    CropHeight,
    CropWidth,
    CropX,
    CropY,
    Expand,
    Fill,
    FilterMethod,
    HeightFill,
    Height,
    ImagePath,
    Opacity,
    RotationDegrees,
    RotationMethod,
    RotationRadians,
    Scale,
    Show,
    WidthFill,
    Width,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Image {
    type Param = ImageParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ImageParam::BorderRadius => set_t_value(&mut self.border_radius, value, "ImageParam::BorderRadius"),
            ImageParam::ContentFit => set_t_value(&mut self.content_fit, value, "ImageParam::ContentFit"),
            ImageParam::CropHeight => set_t_value(&mut self.crop_height, value, "ImageParam::CropHeight"),
            ImageParam::CropWidth => set_t_value(&mut self.crop_width, value, "ImageParam::CropWidth"),
            ImageParam::CropX => set_t_value(&mut self.crop_x, value, "ImageParam::CropX"),
            ImageParam::CropY => set_t_value(&mut self.crop_y, value, "ImageParam::CropY"),
            ImageParam::Expand => set_t_value(&mut self.expand, value, "ImageParam::Expand"),
            ImageParam::Fill => set_t_value(&mut self.fill, value, "ImageParam::Fill"),
            ImageParam::FilterMethod => set_t_value(&mut self.filter_method, value, "ImageParam::FilterMethod"),
            ImageParam::Height => set_t_value(&mut self.height, value, "ImageParam::Height"),
            ImageParam::HeightFill => set_t_value(&mut self.height_fill, value, "ImageParam::HeightFill"),
            ImageParam::ImagePath => set_t_value(&mut self.path, value, "ImageParam::ImagePath"),
            ImageParam::Opacity => set_t_value(&mut self.opacity, value, "ImageParam::Opacity"),
            ImageParam::RotationDegrees => set_t_value(&mut self.rotation_degrees, value, "ImageParam::RotationDegrees"),
            ImageParam::RotationMethod => set_t_value(&mut self.rotation_method, value, "ImageParam::RotationMethod"),
            ImageParam::RotationRadians => set_t_value(&mut self.rotation_radians, value, "ImageParam::RotationRadians"),
            ImageParam::Scale => set_t_value(&mut self.scale, value, "ImageParam::Scale"),
            ImageParam::Show => set_t_value(&mut self.show, value, "ImageParam::Show"),
            ImageParam::Width => set_t_value(&mut self.width, value, "ImageParam::Width"),
            ImageParam::WidthFill => set_t_value(&mut self.width_fill, value, "ImageParam::WidthFill"),
        }
    }
}
