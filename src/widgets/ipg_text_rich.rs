//! ipg_text_rich
use iced::{Background, Border, Color, Element, Font, Padding};
use iced::widget::span;
use crate::app::Message;
use crate::graphics::colors::IpgColor;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool, set_opt_f32};

use pyo3::{pyclass, Py, PyAny};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;



#[derive(Debug, Clone)]
pub struct IpgRichText {
    pub id: usize,
    pub parent_id: String,
    pub spans: Vec<IpgSpan>,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub color: Option<Color>,
    pub padding: Padding,
    pub show: bool,
}


#[derive(Debug, Clone)]
pub struct IpgSpan {
    pub id: usize,
    pub rich_text_id: usize,
    pub text: String,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub color: Option<Color>,
    pub font: Option<Font>,
    pub highlight: Option<IpgHighLight>,
    pub padding: Option<Padding>,
    pub underline: bool,
    pub strikethrough: bool, 
}


#[derive(Debug, Clone, PartialEq)]
pub struct IpgHighLight {
    pub background: Background,
    pub border: Border,
}

impl IpgRichText {
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

        let mut rt = iced::widget::rich_text(spans)
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


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRichTextParam {
    LineHeight,
    Show,
    Size,
    TextColor, 
    TextRgba,
}

impl WidgetParamUpdate for IpgRichText {
    type Param = IpgRichTextParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgRichTextParam::LineHeight => set_opt_f32(&mut self.line_height, value, "IpgRichTextParam::LineHeight"),
            IpgRichTextParam::Show => set_bool(&mut self.show, value, "IpgRichTextParam::Show"),
            IpgRichTextParam::Size => set_opt_f32(&mut self.size, value, "IpgRichTextParam::Size"),
            IpgRichTextParam::TextColor => {
                self.color = IpgColor::rgba_ipg_color_to_iced(None, None, None);
            },
            IpgRichTextParam::TextRgba => {
                self.color = IpgColor::rgba_ipg_color_to_iced(None, None, None);
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSpanParam {
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