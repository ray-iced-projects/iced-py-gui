//! ipg_separator
#![allow(clippy::enum_variant_names)]
use crate::app::Message;
use crate::graphics::colors::IpgColor;

use crate::app;
use crate::state::IpgWidgets;

use iced::border::Radius;
use iced::widget::{row, Row, Text};
use iced::{Background, Border, Color, Element, 
    Length, Renderer, Theme };

use pyo3::{pyclass, Py, PyAny, Python};
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
    pub dot_count: Option<usize>,
    pub dot_fill: bool,
    pub dot_border_width: Option<f32>,
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

pub fn construct_separator<'a>(sep: &'a IpgSeparator, 
                            style_opt: Option<&IpgWidgets>) 
                            -> Option<Element<'a, app::Message>> {

    if !sep.show {
        return None
    }

    let style_opt = get_sep_style(style_opt);

    let mut sep_color = IpgColor::rgba_ipg_color_to_iced(
                                    None, 
                                    Some(IpgColor::PRIMARY), 
                                    1.0, 
                                    false).unwrap();

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
    } else {
        Color::TRANSPARENT
    };

    // Shrink doesn't seem to work so sub in radius
    let width =  if let Some(rad) = sep.dot_radius {
        Length::Fixed(rad*2.0)
        } else { Length::Shrink };
    
    row((0..sep.dot_count).map(|_| {
        Quad {
            inner_bounds: InnerBounds::Square(radius * 2.0),
            quad_border: Border {
                radius: Radius::new(radius),
                color: border.color,
                width: sep.dot_border_width,
            },
            width,
            height,
            quad_color: color.into(),
            ..Default::default()
        }.into()
    }))
    .height(height)
    .spacing(sep.spacing)
    .into()
}

fn get_label(sep: &IpgSeparator,
            sep_color: Color) 
            -> Element<'_, app::Message> {

    let q_1: Element<Message, Theme, Renderer> = Quad {
        width: Length::Fixed(sep.label_left_width),
        height: sep.height,
        ..separator(sep_color.into())
    }.into();
    let q_2: Element<Message, Theme, Renderer> = Quad {
        width: Length::Fixed(sep.label_right_width),
        height: sep.height,
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
                        .spacing(sep.spacing)
                        .into()
}

fn get_line(sep: &IpgSeparator,
            sep_color: Color) 
            -> Element<'_, app::Message> {
    Quad {
        width: sep.width,
        height: sep.height,
        ..separator(sep_color.into())
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


pub fn separator_item_update(sep: &mut IpgSeparator,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_separator_update(item);
    let name = "Separator".to_string();
    match update {
        IpgSeparatorParam::DotBorderWidth => {
            sep.dot_border_width = try_extract_f64(value, name) as f32;
        },
        IpgSeparatorParam::DotCount => {
            sep.dot_count = try_extract_i64(value, name) as usize;
        },
        IpgSeparatorParam::DotFill => {
            sep.dot_fill = try_extract_boolean(value, name);
        },
        IpgSeparatorParam::DotRadius => {
            sep.dot_radius = try_extract_f64(value, name) as f32;
        },
        IpgSeparatorParam::Label => {
            sep.label = Some(try_extract_string(value, name));
        },
        IpgSeparatorParam::Height => {
            let val = try_extract_f64(value, name);
            sep.height = get_height(Some(val as f32), false);
        },
        IpgSeparatorParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            sep.height = get_height(None, val);
        },
        IpgSeparatorParam::Spacing => {
            sep.spacing = try_extract_f64(value, name) as f32;
        },
        IpgSeparatorParam::Show => {
            sep.show = try_extract_boolean(value, name);
        },
        IpgSeparatorParam::StyleId => {
            sep.style_id = Some(try_extract_f64(value, name) as usize);
        },
        IpgSeparatorParam::Width => {
            let val = try_extract_f64(value, name);
            sep.width = get_width(Some(val as f32), false);
        },
        IpgSeparatorParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            sep.width = get_width(None, val);
        },
    }

}

fn try_extract_separator_update(update_obj: &PyObject) -> IpgSeparatorParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgSeparatorParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Separator update extraction failed"),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSeparatorStyleParam {
    IpgColor,
    RbgaColor,
    BorderIpgColor,
    BorderRgbaColor,
}

pub fn separator_style_update_item(style: &mut IpgSeparatorStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    let update = try_extract_separator_style_update(item);
    let name = "SeparatorStyle".to_string();
    match update {
        IpgSeparatorStyleParam::IpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.color = get_color(None, Some(color), 1.0, false);
        },
        IpgSeparatorStyleParam::RbgaColor => {
            style.color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgSeparatorStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgSeparatorStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

fn try_extract_separator_style_update(update_obj: &PyObject) -> IpgSeparatorStyleParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgSeparatorStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Separator type update extraction failed"),
        }
    })
}

pub fn get_sep_style(style: Option<&IpgWidgets>) -> Option<IpgSeparatorStyle>{
    match style {
        Some(IpgWidgets::IpgSeparatorStyle(style)) => {
            Some(style.clone())
        }
            _ => None,
        }
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

