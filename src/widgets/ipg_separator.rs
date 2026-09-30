//! ipg_separator
#![allow(clippy::enum_variant_names)]
use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::colors::Color;

use crate::app;
use crate::state::Widgets;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};

use iced::border::Radius;
use iced::widget::{row, Row, Text};
use iced::{Background, Border, Element, 
    Renderer, Theme, Widget };

use crate::widgets::quad::{InnerBounds, Quad};
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct Separator {
    pub id: usize,
    pub width: f32,
    pub height: f32,
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
        
        if self.dot == Some(true) {  
            Some(get_dot(self, sep_color, border_color))
        } else if let Some(lbl) = &self.label {
            Some(get_label(self, lbl.clone(), sep_color))
        } else if self.line == Some(true) {
            Some(get_line(self, sep_color))
        } else { None }

    }

}

fn get_dot(
        sep: &Separator, 
        sep_color: iced::Color,
        bd_color: iced::Color) 
        -> Element<'_, app::Message>{
    
    let dot_radius = sep.dot_radius.unwrap_or(1.0);

    // let width =  if let Some(rad) = sep.dot_radius {
    //     Length::Fixed(rad*2.0)
    //     } else { Length::Shrink };
    
    let dot_count = if let Some(dc) = sep.dot_count {
        dc
    } else {
        eprintln!("You selected Separator.Dot, so you need to use dot_count, defaulting to 10");
        10
    };

    let border_width = sep.dot_border_width.unwrap_or(1.0);

    row((0..dot_count).map(|_| {
        Quad {
            width: sep.width,
            height: sep.height,
            inner_bounds: InnerBounds::Square(dot_radius*2.0),
            quad_color: sep_color.into(),
            quad_border: Border {
                radius: dot_radius.into(),
                color: bd_color,
                width: border_width,
            },
            ..Default::default()
        }.boxed()
    }))
    .height(sep.height)
    .spacing(sep.spacing.unwrap_or(0.0))
    .boxed()

}

fn get_label(
        sep: &Separator,
        label: String,
        sep_color: iced::Color) 
        -> Element<'_, app::Message> {
    
    let q_1: Element<Message, Theme, Renderer> = Quad {
        width: sep.width,
        height: sep.height,
        inner_bounds: InnerBounds::Ratio(1.0, 1.0),
        ..separator(sep.width, sep.height, sep_color.into())
    }.boxed();
    let q_2: Element<Message, Theme, Renderer> = Quad {
        width: sep.width,
        height: sep.height,
        inner_bounds: InnerBounds::Ratio(1.0, 1.0),
        ..separator(sep.width, sep.height, sep_color.into())
    }.boxed();

    Row::with_children(vec![
                        q_1, 
                        Text::new(label).color(sep_color).boxed(),
                        q_2,
                        ])
                        .spacing(sep.spacing.unwrap_or(0.0))
                        .boxed()
}

fn get_line(
        sep: &Separator,
        sep_color: iced::Color) 
        -> Element<'_, app::Message> {
    
    Quad {
            inner_bounds: InnerBounds::Ratio(1.0, 1.0),
            quad_border: Border::default(),
            width: sep.width,
            height: sep.height,
            quad_color: sep_color.into(),
            ..Default::default()
        }.boxed()
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
    Height,
    Label,
    Spacing,
    Show,
    StyleId,
    Width,
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



fn separator(width: f32, height: f32, bg_color: Background) -> Quad {
    Quad {
        quad_color: bg_color,
        quad_border: Border {
            radius: Radius::new(4.0),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(0.98, 0.2),
        width,
        height,
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
            SeparatorParam::Height => set_t_value(&mut self.height, value, "SeparatorParam::Height"),
            SeparatorParam::Label => set_t_value(&mut self.label, value, "SeparatorParam::Label"),
            SeparatorParam::Show => set_t_value(&mut self.show, value, "SeparatorParam::Show"),
            SeparatorParam::Spacing => set_t_value(&mut self.spacing, value, "SeparatorParam::Spacing"),
            SeparatorParam::StyleId => set_t_value(&mut self.style_id, value, "SeparatorParam::StyleId"),
            SeparatorParam::Width => set_t_value(&mut self.width, value, "SeparatorParam::Width"),
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
