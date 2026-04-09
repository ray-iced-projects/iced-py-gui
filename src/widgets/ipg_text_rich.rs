//! ipg_text_rich

use iced::{Background, Border, Element, Font, Padding};
use iced::widget::{self, span};

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool, set_opt_f32};

use pyo3::{pyclass, Py, PyAny};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;



#[derive(Debug, Clone)]
pub struct RichText {
    pub id: usize,
    pub parent_id: String,
    pub spans: Vec<Span>,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub color: Option<iced::Color>,
    pub padding: Padding,
    pub show: bool,
}


#[derive(Debug, Clone)]
pub struct Span {
    pub id: usize,
    pub rich_text_id: usize,
    pub text: String,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub color: Option<iced::Color>,
    pub font: Option<Font>,
    pub highlight: Option<HighLight>,
    pub padding: Option<Padding>,
    pub underline: bool,
    pub strikethrough: bool, 
}


#[derive(Debug, Clone, PartialEq)]
pub struct HighLight {
    pub background: Background,
    pub border: Border,
}

impl RichText {
    pub fn construct<'a>(
        &'a self,
        ) -> Option<Element<'a, Message>> {

        if !self.show {
            return None
        }

        let spans: Vec<_> = self.spans.iter().map(|s| {
            let mut sp = span(s.text.clone());

            if let Some(size) = s.size {
                sp = sp.size(size);
            }
            if let Some(lh) = s.line_height {
                sp = sp.line_height(lh);
            }
            if let Some(font) = s.font {
                sp = sp.font(font);
            }
            if let Some(color) = s.color {
                sp = sp.color(color);
            }
            if let Some(ref hl) = s.highlight {
                sp = sp.background(hl.background).border(hl.border);
            }
            if let Some(padding) = s.padding {
                sp = sp.padding(padding);
            }
            if s.underline {
                sp = sp.underline(true);
            }
            if s.strikethrough {
                sp = sp.strikethrough(true);
            }
            sp
        }).collect();

        let mut rt = widget::rich_text(spans)
            .on_link_click(iced::never);

        if let Some(size) = self.size {
            rt = rt.size(size);
        }
        if let Some(lh) = self.line_height {
            rt = rt.line_height(lh);
        }
        if let Some(color) = self.color {
            rt = rt.color(color);
        }

        Some(rt.into())
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum RichTextParam {
    LineHeight,
    Show,
    Size,
    TextColor, 
    TextRgba,
}

impl WidgetParamUpdate for RichText {
    type Param = RichTextParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RichTextParam::LineHeight => set_opt_f32(&mut self.line_height, value, "RichTextParam::LineHeight"),
            RichTextParam::Show => set_bool(&mut self.show, value, "RichTextParam::Show"),
            RichTextParam::Size => set_opt_f32(&mut self.size, value, "RichTextParam::Size"),
            RichTextParam::TextColor => {
                self.color = Color::rgba_ipg_color_to_iced(None, &None, None);
            },
            RichTextParam::TextRgba => {
                self.color = Color::rgba_ipg_color_to_iced(None, &None, None);
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SpanParam {
    Text,
    Bold,
    Italic,
    Color,
    ColorRgba,
    LineHeight,
    Padding,
    Size,
    Strikethrough,
    Underline,
}