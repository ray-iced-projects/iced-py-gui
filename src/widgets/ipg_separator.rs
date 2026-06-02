//! ipg_separator
#![allow(clippy::enum_variant_names)]
use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::colors::Color;

use crate::app;
use crate::py_api::helpers::get_len;
use crate::state::Widgets;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};

use iced::border::Radius;
use iced::widget::{row, Row, Text};
use iced::{Background, Border, Element, 
    Length, Renderer, Theme };

use crate::widgets::quad::{InnerBounds, Quad};
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct Separator {
    pub id: usize,
    pub dot: Option<bool>,
    pub label: Option<String>,
    pub line: Option<bool>,
    pub label_left_width: Option<f32>,
    pub label_right_width: Option<f32>,
    pub dot_radius: Option<f32>,
    pub dot_count: Option<u32>,
    pub dot_fill: Option<bool>,
    pub dot_border_width: Option<f32>,
    pub line_length: Option<f32>,
    pub line_thickness: Option<f32>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub spacing: Option<f32>,
    pub style_id: Option<usize>,
    pub show: bool,
}

impl Separator {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self, 
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, app::Message>> {

        if !self.show { return None }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_separator_style).cloned();


        let (sep_color, border_color)= 
            if let Some(st) = style_opt {
                let sc = match Color::rgba_ipg_color_to_iced(st.rgba, &st.color, st.color_alpha){
                    Some(c) => c,
                    None => Color::LIGHT_BLUE.to_iced(),
                };
                let bc = match Color::rgba_ipg_color_to_iced(st.border_rgba, &st.border_color, st.border_color_alpha) {
                    Some(c) => c,
                    None => Color::LIGHT_BLUE.to_iced(),
                };
                (sc, bc)
            } else {
                (Color::LIGHT_BLUE.to_iced(), Color::LIGHT_BLUE.to_iced())
            };
        
        // returns a separator with some styling
        let separator = 
                if self.dot == Some(true) {  
                    Some(get_dot(self, sep_color, border_color))
                } else if let Some(lbl) = &self.label {
                    Some(get_label(self, lbl.clone(), sep_color))
                } else if self.line == Some(true) {
                    Some(get_line(self, sep_color))
                } else { None };

        separator
        
    }

}

fn get_dot(sep: &Separator, 
            sep_color: iced::Color,
            bd_color: iced::Color) 
            -> Element<'_, app::Message>{
    
    let dot_radius = if let Some(dr) = sep.dot_radius {
        dr
    } else { 1.0 };

    let width =  if let Some(rad) = sep.dot_radius {
        Length::Fixed(rad*2.0)
        } else { Length::Shrink };
    
    let dot_count = if let Some(dc) = sep.dot_count {
        dc
    } else {
        eprintln!("You selected Separator.Dot, so you need to use dot_count, defaulting to 10");
        10
    };

    let border_width = if let Some(bw) = sep.dot_border_width {
        bw
    } else { 1.0 };

    let height = get_len(sep.fill, sep.height_fill, sep.height);

    row((0..dot_count).map(|_| {
        Quad {
            inner_bounds: InnerBounds::Square(dot_radius*2.0),
            quad_border: Border {
                radius: dot_radius.into(),
                color: bd_color,
                width: border_width,
            },
            width,
            height,
            quad_color: sep_color.into(),
            ..Default::default()
        }.into()
    }))
    .height(height)
    .spacing(sep.spacing.unwrap_or(0.0))
    .into()
}

fn get_label(sep: &Separator,
            label: String,
            sep_color: iced::Color) 
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

    Row::with_children(vec![
                        q_1, 
                        Text::new(label).color(sep_color).into(),
                        q_2,
                        ])
                        .spacing(sep.spacing.unwrap_or(0.0))
                        .into()
}

fn get_line(sep: &Separator,
            sep_color: iced::Color) 
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

#[derive(Debug, Clone)]
pub struct SeparatorStyle {
    pub id: usize,
    pub color: Option<Color>,
    pub color_alpha: Option<f32>,
    pub rgba: Option<[f32; 4]>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SeparatorParam {
    DotCount,
    DotFill,
    DotBorderWidth,
    DotRadius,
    Fill,
    Height,
    HeightFill,
    Label,
    Spacing,
    Show,
    StyleId,
    Width,
    WidthFill,
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SeparatorStyleParam {
    Color,
    ColorAlpha,
    Rbga,
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
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

impl WidgetParamUpdate for Separator {
    type Param = SeparatorParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SeparatorParam::DotBorderWidth => set_t_value(&mut self.dot_border_width, value, "SeparatorParam::DotBorderWidth"),
            SeparatorParam::DotCount => set_t_value(&mut self.dot_count, value, "SeparatorParam::DotCount"),
            SeparatorParam::DotFill => set_t_value(&mut self.dot_fill, value, "SeparatorParam::DotFill"),
            SeparatorParam::DotRadius => set_t_value(&mut self.dot_radius, value, "SeparatorParam::DotRadius"),
            SeparatorParam::Fill => set_t_value(&mut self.fill, value, "SeparatorParam::Fill"),
            SeparatorParam::Height => set_t_value(&mut self.height, value, "SeparatorParam::Height"),
            SeparatorParam::HeightFill => set_t_value(&mut self.height_fill, value, "SeparatorParam::HeightFill"),
            SeparatorParam::Label => set_t_value(&mut self.label, value, "SeparatorParam::Label"),
            SeparatorParam::Show => set_t_value(&mut self.show, value, "SeparatorParam::Show"),
            SeparatorParam::Spacing => set_t_value(&mut self.spacing, value, "SeparatorParam::Spacing"),
            SeparatorParam::StyleId => set_t_value(&mut self.style_id, value, "SeparatorParam::StyleId"),
            SeparatorParam::Width => set_t_value(&mut self.width, value, "SeparatorParam::Width"),
            SeparatorParam::WidthFill => set_t_value(&mut self.width_fill, value, "SeparatorParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for SeparatorStyle {
    type Param = SeparatorStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SeparatorStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "SeparatorStyleParam::BorderColor"),
            SeparatorStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "SeparatorStyleParam::BorderColorAlpha"),
            SeparatorStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "SeparatorStyleParam::BorderRgba"),
            SeparatorStyleParam::Color => set_t_value(&mut self.color, value, "SeparatorStyleParam::Color"),
            SeparatorStyleParam::ColorAlpha => set_t_value(&mut self.color_alpha, value, "SeparatorStyleParam::ColorAlpha"),
            SeparatorStyleParam::Rbga => set_t_value(&mut self.rgba, value, "SeparatorStyleParam::Rbga"),
        }
    }
}
