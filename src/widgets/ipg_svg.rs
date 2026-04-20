//! ipg_svg

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::helpers::get_len;
use crate::widgets::enums::ContentFit;
use crate::widgets::widget_param_update::WidgetParamUpdate;
use crate::widgets::widget_param_update::set_t_value;

use iced::Element;
use iced::Radians;
use iced::Rotation;
use iced::widget;
use iced::advanced::svg;
use iced::widget::svg::{Style, };
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct Svg {
        pub id: usize,
        pub path: String,
        pub width: Option<f32>,
        pub width_fill: Option<bool>,
        pub height: Option<f32>,
        pub height_fill: Option<bool>,
        pub fill: Option<bool>,
        pub color_filter: Option<Color>,
        pub color_filter_alpha: Option<f32>,
        pub rgba_filter: Option<[f32; 4]>,
        pub content_fit: Option<ContentFit>,
        pub rotation_solid: Option<bool>,
        pub rotation_radians: Option<f32>,
        pub rotation_degrees: Option<f32>,
        pub opacity: Option<f32>,
        pub show: bool,
}

impl Svg{
    pub fn construct(&self) 
        -> Option<Element<'_, Message>> {

        if !self.show { return None }

        let svg_handle = svg::Handle::from_path(self.path.clone());

        if !std::path::Path::new(&self.path).exists() {
            eprintln!("[WARNING] Image path '{}' does not exist", self.path);
            return None
        }

        let svg = widget::Svg::new(svg_handle)
                    .width(get_len(self.fill, self.width_fill, self.width))
                    .height(get_len(self.fill, self.height_fill, self.height))
                    .style(move|_, _| {
                        let color = Color::rgba_ipg_color_to_iced(self.rgba_filter, &self.color_filter, self.color_filter_alpha);
                        Style{color}
                    });

        let svg = if let Some(cf) = &self.content_fit {
            svg.content_fit(cf.to_iced())
        } else { svg };

        let svg = match (self.rotation_solid, self.rotation_degrees, self.rotation_radians) {
            (Some(true), Some(deg), _) => svg.rotation(Rotation::Solid(Radians(deg.to_radians()))),
            (Some(true), _, Some(rad)) => svg.rotation(Rotation::Solid(Radians(rad))),
            (_, Some(deg), _) => svg.rotation(Rotation::Floating(Radians(deg.to_radians()))),
            (_, _, Some(rad)) => svg.rotation(Rotation::Floating(Radians(rad))),
            _ => svg,
        };
        

        let svg = if let Some(op) = self.opacity {
            svg.opacity(op)
        } else { svg }.into();

        Some(svg)

    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SvgParam {
    ColorFilterAlpha,
    ColorFilter,
    ContentFit,
    Fill,
    HeightFill,
    Height,
    Opacity,
    Path,
    RgbaFilter,
    RotationDegrees,
    RotationRadians,
    RotationSolid,
    WidthFill,
    Width,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Svg {
    type Param = SvgParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SvgParam::ColorFilterAlpha => set_t_value(&mut self.color_filter_alpha, value, "SvgParam::ColorFilterAlpha"),
            SvgParam::ColorFilter => set_t_value(&mut self.color_filter, value, "SvgParam::ColorFilter"),
            SvgParam::ContentFit => set_t_value(&mut self.content_fit, value, "SvgParam::ContentFit"),
            SvgParam::Fill => set_t_value(&mut self.fill, value, "SvgParam::Fill"),
            SvgParam::HeightFill => set_t_value(&mut self.height_fill, value, "SvgParam::HeightFill"),
            SvgParam::Height => set_t_value(&mut self.height, value, "SvgParam::Height"),
            SvgParam::Opacity => set_t_value(&mut self.opacity, value, "SvgParam::Opacity"),
            SvgParam::Path => set_t_value(&mut self.path, value, "SvgParam::Path"),
            SvgParam::RgbaFilter => set_t_value(&mut self.rgba_filter, value, "SvgParam::RgbaFilter"),
            SvgParam::RotationDegrees => set_t_value(&mut self.rotation_degrees, value, "SvgParam::RotationDegrees"),
            SvgParam::RotationRadians => set_t_value(&mut self.rotation_radians, value, "SvgParam::RotationRadians"),
            SvgParam::RotationSolid => set_t_value(&mut self.rotation_solid, value, "SvgParam::RotationSolid"),
            SvgParam::WidthFill => set_t_value(&mut self.width_fill, value, "SvgParam::WidthFill"),
            SvgParam::Width => set_t_value(&mut self.width, value, "SvgParam::Width"),
        }
    }
}
