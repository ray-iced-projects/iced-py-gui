//! ipg_separator
#![allow(clippy::enum_variant_names)]
use crate::app::Message;
use crate::graphics::colors::IpgColor;

use crate::app;
use crate::state::IpgWidgets;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_height, set_height_fill, 
    set_opt_f32, set_opt_iced_color, set_opt_iced_color_from_rgba, 
    set_opt_string, set_opt_u32, set_opt_usize, set_width, set_width_fill};

use iced::border::Radius;
use iced::widget::{row, Row, Text};
use iced::{Background, Border, Color, Element, 
    Length, Renderer, Theme };

use crate::widgets::quad::{InnerBounds, Quad};
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct IpgSeparator {
    pub id: usize,
    pub parent_id: String,
    pub separator_type: Option<IpgSeparatorType>,
    pub label: Option<String>,
    pub label_left_width: Option<f32>,
    pub label_right_width: Option<f32>,
    pub dot_radius: Option<f32>,
    pub dot_count: Option<u32>,
    pub dot_fill: bool,
    pub dot_border_width: Option<f32>,
    pub line_length: Option<f32>,
    pub line_thickness: Option<f32>,
    pub width: Length,
    pub height: Length,
    pub spacing: Option<f32>,
    pub style_id: Option<usize>,
    pub show: bool,
}

#[derive(Debug, Clone)]
pub struct IpgSeparatorStyle {
    pub id: usize,
    pub color: Option<Color>,
    pub border_color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSeparatorType {
    Dot,
    Label,
    Line,
}

pub fn construct_separator<'a>(
    sep: &'a IpgSeparator, 
    style_opt: Option<&IpgWidgets>) 
    -> Option<Element<'a, app::Message>> {

    if !sep.show {
        return None
    }

    let style_opt = style_opt.and_then(IpgWidgets::as_separator_style).cloned();

    let sep_color = 
        IpgColor::rgba_ipg_color_to_iced(
            None, 
            Some(IpgColor::PRIMARY), 
            1.0).unwrap();

    let mut border = Border::default();
    
    let separator: Element<'a, app::Message>  = 
        if let Some(style) = style_opt {
        
        let sep_color = 
            if let Some(c) = style.color {
                c
            } else { sep_color };
        
        if let Some(bc) = style.border_color {
            border.color = bc;
        }
        
        // returns a separator with some styling
        match sep.separator_type {
            Some(IpgSeparatorType::Dot) => {  
                get_dot(sep, sep_color, border)
            },
            Some(IpgSeparatorType::Label )=> {
                get_label(sep, sep_color)
            },
            Some(IpgSeparatorType::Line) | None=> {
                get_line(sep, sep_color)
            },
        }

    } else {
        // returns a separator with default styling
        match sep.separator_type {
            Some(IpgSeparatorType::Dot) => {
                get_dot(sep, sep_color, border)
            },
            Some(IpgSeparatorType::Label) => {
                get_label(sep, sep_color)
            },
            Some(IpgSeparatorType::Line) | None=> {
                get_line(sep, sep_color)
            },
        }
    };

    Some(separator)
    
}

fn get_dot(sep: &IpgSeparator, 
            sep_color: Color,
            border: Border) 
            -> Element<'_, app::Message>{
    
    let color = if sep.dot_fill {
        sep_color
    } else  { Color::TRANSPARENT };

    let dot_radius = if let Some(dr) = sep.dot_radius {
        dr
    } else { 1.0 };

    let width =  if let Some(rad) = sep.dot_radius {
        Length::Fixed(rad*2.0)
        } else { Length::Shrink };
    
    let dot_count = if let Some(dc) = sep.dot_count {
        dc
    } else {
        eprintln!("You selected IpgSeparator.Dot, so you need to use dot_count, defaulting to 10");
        10
    };
    
    row((0..dot_count).map(|_| {
        Quad {
            inner_bounds: InnerBounds::Square(dot_radius*2.0),
            quad_border: Border {
                radius: dot_radius.into(),
                color: border.color,
                width: border.width,
            },
            width,
            height: sep.height,
            quad_color: color.into(),
            ..Default::default()
        }.into()
    }))
    .height(sep.height)
    .spacing(sep.spacing.unwrap_or(0.0))
    .into()
}

fn get_label(sep: &IpgSeparator,
            sep_color: Color) 
            -> Element<'_, app::Message> {
    
    let q_1: Element<Message, Theme, Renderer> = Quad {
        width: Length::Fixed(sep.label_left_width.unwrap_or(0.0)),
        height: Length::Fill,
        inner_bounds: InnerBounds::Ratio(1.0, 1.0),
        ..separator(sep_color.into())
    }.into();
    let q_2: Element<Message, Theme, Renderer> = Quad {
        width: Length::Fixed(sep.label_right_width.unwrap_or(0.0)),
        height: Length::Fill,
        inner_bounds: InnerBounds::Ratio(1.0, 1.0),
        ..separator(sep_color.into())
    }.into();

    let lbl = match &sep.label {
        Some(lbl) => lbl,
        None => panic!("Separator: A label is required for IpgSeparatorType::Label.")
    };

    Row::with_children(vec![
                        q_1, 
                        Text::new(lbl).color(sep_color).into(),
                        q_2,
                        ])
                        .spacing(sep.spacing.unwrap_or(0.0))
                        .into()
}

fn get_line(sep: &IpgSeparator,
            sep_color: Color) 
            -> Element<'_, app::Message> {
    
    let length = if let Some(ll) = sep.line_length {
        Length::Fixed(ll)
    } else { Length::Fixed(20.0) };

    let thickness = if let Some(th) = sep.line_thickness {
        Length::Fixed(th)
    } else { Length::Fixed(2.0) };

    Quad {
            inner_bounds: InnerBounds::Ratio(1.0, 1.0),
            quad_border: Border::default(),
            width: length,
            height: thickness,
            quad_color: sep_color.into(),
            ..Default::default()
        }.into()
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSeparatorParam {
    DotCount,
    DotFill,
    DotBorderWidth,
    DotRadius,
    Height,
    HeightFill,
    Label,
    Spacing,
    Show,
    StyleId,
    Width,
    WidthFill,
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSeparatorStyleParam {
    IpgColor,
    RbgaColor,
    BorderIpgColor,
    BorderRgbaColor,
}



fn separator(bg_color: Background) -> Quad {
    Quad {
        quad_color: bg_color,
        quad_border: Border {
            radius: Radius::new(4.0),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(0.98, 0.2),
        width: Length::Shrink,
        height: Length::Shrink,
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgSeparator {
    type Param = IpgSeparatorParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgSeparatorParam::DotCount => set_opt_u32(&mut self.dot_count, value, "DotCount"),
            IpgSeparatorParam::DotFill => set_bool(&mut self.dot_fill, value, "DotFill"),
            IpgSeparatorParam::DotBorderWidth => set_opt_f32(&mut self.dot_border_width, value, "DotBorderWidth"),
            IpgSeparatorParam::DotRadius => set_opt_f32(&mut self.dot_radius, value, "DotRadius"),
            IpgSeparatorParam::Height => set_height(&mut self.height, value, "Height"),
            IpgSeparatorParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            IpgSeparatorParam::Label => set_opt_string(&mut self.label, value, "Label"),
            IpgSeparatorParam::Spacing => set_opt_f32(&mut self.spacing, value, "Spacing"),
            IpgSeparatorParam::Show => set_bool(&mut self.show, value, "Show"),
            IpgSeparatorParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            IpgSeparatorParam::Width => set_width(&mut self.width, value, "Width"),
            IpgSeparatorParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
        }
    }
}

impl WidgetParamUpdate for IpgSeparatorStyle {
    type Param = IpgSeparatorStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgSeparatorStyleParam::IpgColor => 
            set_opt_iced_color(&mut self.color, value, "IpgColor"),
            IpgSeparatorStyleParam::RbgaColor => 
            set_opt_iced_color_from_rgba(&mut self.color, value, "RbgaColor"),
            IpgSeparatorStyleParam::BorderIpgColor => 
            set_opt_iced_color(&mut self.color, value, "BorderIpgColor"),
            IpgSeparatorStyleParam::BorderRgbaColor => 
            set_opt_iced_color_from_rgba(&mut self.color, value, "BorderRgbaColor"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_separator() -> IpgSeparator {
        IpgSeparator {
            id: 0,
            parent_id: String::new(),
            separator_type: None,
            label: None,
            label_left_width: None,
            label_right_width: None,
            dot_radius: None,
            dot_count: None,
            dot_fill: false,
            dot_border_width: None,
            line_length: None,
            line_thickness: None,
            width: Length::Shrink,
            height: Length::Shrink,
            spacing: None,
            style_id: None,
            show: true,
        }
    }

    fn make_separator_style() -> IpgSeparatorStyle {
        IpgSeparatorStyle {
            id: 0,
            color: None,
            border_color: None,
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

    // -- IpgSeparator param tests --

    #[test]
    fn test_dot_count() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::DotCount, &py_obj(5u32));
        assert_eq!(s.dot_count, Some(5));
    }

    #[test]
    fn test_dot_fill() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::DotFill, &py_obj(true));
        assert!(s.dot_fill);
    }

    #[test]
    fn test_dot_border_width() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::DotBorderWidth, &py_obj(2.0f32));
        assert_eq!(s.dot_border_width, Some(2.0));
    }

    #[test]
    fn test_dot_radius() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::DotRadius, &py_obj(3.0f32));
        assert_eq!(s.dot_radius, Some(3.0));
    }

    #[test]
    fn test_height() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::Height, &py_obj(50.0f32));
        assert_eq!(s.height, Length::Fixed(50.0));
    }

    #[test]
    fn test_height_fill() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::HeightFill, &py_obj(true));
        assert_eq!(s.height, Length::Fill);
    }

    #[test]
    fn test_label() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::Label, &py_obj("hello".to_string()));
        assert_eq!(s.label, Some("hello".to_string()));
        s.param_update(IpgSeparatorParam::Label, &py_none());
        assert_eq!(s.label, None);
    }

    #[test]
    fn test_spacing() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::Spacing, &py_obj(4.0f32));
        assert_eq!(s.spacing, Some(4.0));
    }

    #[test]
    fn test_show() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::Show, &py_obj(false));
        assert!(!s.show);
    }

    #[test]
    fn test_style_id() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::StyleId, &py_obj(3usize));
        assert_eq!(s.style_id, Some(3));
        s.param_update(IpgSeparatorParam::StyleId, &py_none());
        assert_eq!(s.style_id, None);
    }

    #[test]
    fn test_width() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::Width, &py_obj(100.0f32));
        assert_eq!(s.width, Length::Fixed(100.0));
    }

    #[test]
    fn test_width_fill() {
        let mut s = make_separator();
        s.param_update(IpgSeparatorParam::WidthFill, &py_obj(true));
        assert_eq!(s.width, Length::Fill);
    }

    // -- IpgSeparatorStyle param tests --

    #[test]
    fn test_style_rgba_color() {
        let mut s = make_separator_style();
        s.param_update(IpgSeparatorStyleParam::RbgaColor, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.color.is_some());
    }

    #[test]
    fn test_style_border_rgba_color() {
        let mut s = make_separator_style();
        s.param_update(IpgSeparatorStyleParam::BorderRgbaColor, &py_obj(vec![0.0f32, 1.0, 0.0, 1.0]));
        assert!(s.color.is_some());
    }
}