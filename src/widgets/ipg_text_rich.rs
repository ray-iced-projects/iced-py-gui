//! ipg_text_rich
use iced::advanced::text::Highlight;
use iced::{Background, Border, Color, Element, Font, Padding, Pixels, Renderer, Theme};
use iced::widget::text::{LineHeight, Rich, Span};
use iced::widget::{rich_text, span};
use crate::app::Message;

use pyo3::{pyclass, Py, PyAny, Python};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;

use super::helpers::{get_height, 
    get_width, try_extract_boolean, 
    try_extract_f64, try_extract_ipg_color, try_extract_string, 
    try_extract_vec_f32};
use super::ipg_enums::{IpgHorizontalAlignment, IpgVerticalAlignment};

#[derive(Debug, Clone)]
pub struct IpgRichText {
    pub id: usize,
    pub parent_id: String,
    pub size: Option<f32>,
    pub line_height: Option<LineHeight>,
    pub color: Option<Color>,
    pub highlight: Option<IpgHighlight>,
    pub padding: Padding,
    pub underline: bool,
    pub strikethrough: bool,
    pub show: bool,
    pub style_id: Option<usize>,
}


#[derive(Debug, Clone)]
pub struct IpgSpan {
    pub id: usize,
    pub parent_id: String,
    pub text: String,
    pub size: Option<f32>,
    pub line_height: Option<LineHeight>,
    pub color: Option<Color>,
    pub font: Option<Font>,
    pub highlight: Option<IpgHighlight>,
    pub padding: Option<Padding>,
    pub underline: bool,
    pub strikethrough: bool, 
    pub show: bool,
    pub style_id: Option<usize>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct IpgHighlight {
    pub background: Background,
    pub border: Border,
}

impl IpgRichText {
    pub fn construct<'a>(
        &'a self,
        text: &IpgRichText,
        ) -> Option<Element<'a, Message>> {

        if !text.show {
            return None
        }

        let rt = 
            rich_text![
                span("I am white!").color(Color::WHITE),
                span(" "),
                span("And I am bold!").font(Font { weight: iced::font::Weight::Bold, ..Font::default() }),
            ]
            .on_link_click(iced::never)
            .size(20);

        Some(rt.into())

    }
}

fn get_highlight(hl: IpgHighlight) -> Option<Highlight> {
    Some(Highlight{ 
        background: hl.background, 
        border: hl.border })
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTextParam {
    Content,
    Height,
    HeightFill,
    AlignX,
    AlignY,
    LineHeight,
    Show,
    Size,
    TextColor, 
    TextRgba,
    Width,
    WidthFill,
}


