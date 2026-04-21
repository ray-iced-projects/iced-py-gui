//! ipg_text_rich

use std::collections::HashMap;

use iced::{Background, Border, Element, Padding};
use iced::widget::{self, Space, span};

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::state::Widgets;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};

use pyo3::{pyclass, Py, PyAny};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct RichText {
    pub id: usize,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub font_id: Option<usize>,
    pub color: Option<Color>,
    pub color_alpha: Option<f32>,
    pub rgba: Option<[f32; 4]>,
    pub align_bottom_center: Option<bool>,
    pub align_bottom_left: Option<bool>,
    pub align_bottom_right: Option<bool>,
    pub align_center_left: Option<bool>,
    pub align_center_right: Option<bool>,
    pub align_center: Option<bool>,
    pub align_top_center: Option<bool>,
    pub align_top_left: Option<bool>,
    pub align_top_right: Option<bool>,
    pub wrapping_none: Option<bool>,
    pub wrapping_glyph: Option<bool>,
    pub wrapping_word_glyph: Option<bool>,
    pub hovered_link: Option<usize>,
    pub show: bool,
}

impl RichText {

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>,
    ) -> Element<'a, Message> {

        if !self.show {return Space::new().into()}

        let mut rt = widget::rich_text()
            .on_link_click(iced::never);

        if let Some(size) = self.size {
            rt = rt.size(size);
        }
        if let Some(lh) = self.line_height {
            rt = rt.line_height(lh);
        }

        let color_opt = Color::rgba_ipg_color_to_iced(self.rgba, &self.color, self.color_alpha);

        if let Some(color) = color_opt {
            rt = rt.color(color);
        }

        rt.into()

    }
}

#[derive(Debug, Clone)]
pub struct Span {
    pub id: usize,
    pub text: String,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub color: Option<Color>,
    pub color_alpha: Option<f32>,
    pub rgba: Option<[f32; 4]>,
    pub font_id: Option<usize>,
    pub padding: Option<Padding>,
    pub underline: Option<bool>,
    pub strikethrough: Option<bool>,
    
    // Highlight
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,
    pub background_gradient_color_stop: Option<Color>,
    pub background_gradient_color_stop_alpha: Option<f32>,
    pub background_gradient_rgba_stop: Option<[f32; 4]>,
    pub background_gradient_degrees: Option<f32>,
    pub background_gradient_radians: Option<f32>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    
    // Link ?
    // pub link: Option<>, 
}

impl Span {
    pub fn construct<'a>(
        &'a self,
    ) -> Span<'a, Link, Renderer::Font> {
        
        let mut sp = span(self.text.clone());

        if let Some(size) = self.size {
            sp = sp.size(size);
        }
        if let Some(lh) = self.line_height {
            sp = sp.line_height(lh);
        }
        if let Some(font) = self.font {
            sp = sp.font(font);
        }

        let color_opt = Color::rgba_ipg_color_to_iced(self.rgba, &self.color, self.color_alpha);

        if let Some(color) = color_opt {
            sp = sp.color(color);
        }

        if let Some(padding) = self.padding {
            sp = sp.padding(padding);
        }
        if s.underline {
            sp = sp.underline(true);
        }
        if s.strikethrough {
            sp = sp.strikethrough(true);
        }

        sp

    }
    
}
#[derive(Debug, Clone, PartialEq)]
pub struct HighLight {
    pub background: Background,
    pub border: Border,
}




#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum RichTextParam {
    Color,
    ColorAlpha,
    Rgba,
    LineHeight,
    Show,
    Size,
}

impl WidgetParamUpdate for RichText {
    type Param = RichTextParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RichTextParam::Color => set_t_value(&mut self.color, value, "RichTextParam::Color"),
            RichTextParam::ColorAlpha => set_t_value(&mut self.color_alpha, value, "RichTextParam::ColorAlpha"),
            RichTextParam::Rgba => set_t_value(&mut self.rgba, value, "RichTextParam::Rgba"),
            RichTextParam::LineHeight => set_t_value(&mut self.line_height, value, "RichTextParam::LineHeight"),
            RichTextParam::Show => set_t_value(&mut self.show, value, "RichTextParam::Show"),
            RichTextParam::Size => set_t_value(&mut self.size, value, "RichTextParam::Size"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SpanParam {
    Color,
    ColorAlpha,
    FontId,
    Highlight,
    LineHeight,
    Padding,
    Rgba,
    Size,
    Text,
}

impl WidgetParamUpdate for Span {
    type Param = SpanParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            _ => ()
        }
    }
}
