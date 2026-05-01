//! ipg_text_rich

use std::collections::HashMap;

use iced::widget::{self, span, text};
use iced::{Border, Element};

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::helpers::{get_len, get_padding, get_radius};
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::widget_param_update::{set_t_value, WidgetParamUpdate};

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
    pub show: bool,
}

impl RichText {
    pub fn construct<'a>(
        &'a self,
        child_ids: &[usize],
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message>> {
        
        if !self.show { return None }

        let spans: Vec<text::Span<'static, usize, iced::Font>> = child_ids
            .iter()
            .filter_map(|id| widgets.get(id))
            .filter_map(Widgets::as_span)
            .map(|span| span.to_iced(widgets))
            .collect();

        let mut rt = widget::rich_text(spans);

        rt = rt.width(get_len(self.fill, self.width_fill, self.width));
        rt = rt.height(get_len(self.fill, self.height_fill, self.height));

        if let Some(size) = self.size {
            rt = rt.size(size);
        }
        if let Some(lh) = self.line_height {
            rt = rt.line_height(lh);
        }

        if self.wrapping_none == Some(true) {
            rt = rt.wrapping(text::Wrapping::None);
        } else if self.wrapping_glyph == Some(true) {
            rt = rt.wrapping(text::Wrapping::Glyph);
        } else if self.wrapping_word_glyph == Some(true) {
            rt = rt.wrapping(text::Wrapping::WordOrGlyph);
        }

        let font_opt = self
            .font_id
            .and_then(|id| widgets.get(&id))
            .and_then(Widgets::as_font)
            .map(|font| font.to_iced());

        if let Some(font) = font_opt {
            rt = rt.font(font);
        }

        let color_opt = Color::rgba_ipg_color_to_iced(self.rgba, &self.color, self.color_alpha);
        if let Some(color) = color_opt {
            rt = rt.color(color);
        }

        if self.align_bottom_center == Some(true) {
            rt = rt
                .align_x(text::Alignment::Center)
                .align_y(iced::alignment::Vertical::Bottom);
        }
        if self.align_bottom_left == Some(true) {
            rt = rt
                .align_x(text::Alignment::Left)
                .align_y(iced::alignment::Vertical::Bottom);
        }
        if self.align_bottom_right == Some(true) {
            rt = rt
                .align_x(text::Alignment::Right)
                .align_y(iced::alignment::Vertical::Bottom);
        }
        if self.align_center == Some(true) {
            rt = rt
                .align_x(text::Alignment::Center)
                .align_y(iced::alignment::Vertical::Center);
        }
        if self.align_center_left == Some(true) {
            rt = rt
                .align_x(text::Alignment::Left)
                .align_y(iced::alignment::Vertical::Center);
        }
        if self.align_center_right == Some(true) {
            rt = rt
                .align_x(text::Alignment::Right)
                .align_y(iced::alignment::Vertical::Center);
        }
        if self.align_top_center == Some(true) {
            rt = rt
                .align_x(text::Alignment::Center)
                .align_y(iced::alignment::Vertical::Top);
        }
        if self.align_top_left == Some(true) {
            rt = rt
                .align_x(text::Alignment::Left)
                .align_y(iced::alignment::Vertical::Top);
        }
        if self.align_top_right == Some(true) {
            rt = rt
                .align_x(text::Alignment::Right)
                .align_y(iced::alignment::Vertical::Top);
        }

        rt = rt.on_link_click(move |link| Message::RichTextLinkClicked(self.id, link));

        Some(rt.into())
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
    pub padding: Option<Vec<f32>>,
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

    // Link
    pub link: Option<usize>,
}

impl Span {
    pub fn to_iced(&self, widgets: &HashMap<usize, Widgets>) -> text::Span<'static, usize, iced::Font> {
        let mut sp = span(self.text.clone());

        if let Some(size) = self.size {
            sp = sp.size(size);
        }
        if let Some(lh) = self.line_height {
            sp = sp.line_height(lh);
        }

        let font_opt = self
            .font_id
            .and_then(|id| widgets.get(&id))
            .and_then(Widgets::as_font)
            .map(|font| font.to_iced());

        if let Some(font) = font_opt {
            sp = sp.font(font);
        }

        let color_opt = Color::rgba_ipg_color_to_iced(self.rgba, &self.color, self.color_alpha);
        if let Some(color) = color_opt {
            sp = sp.color(color);
        }

        let background_color = Color::rgba_ipg_color_to_iced(
            self.background_rgba,
            &self.background_color,
            self.background_color_alpha,
        );
        if let Some(background) = background_color {
            sp = sp.background(background);
        }

        let border_color =
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);

        if border_color.is_some() || self.border_radius.is_some() || self.border_width.is_some() {
            let radius = if let Some(radius) = &self.border_radius {
                get_radius(radius, "Span".to_string())
            } else {
                iced::border::Radius::default()
            };

            let border = Border {
                color: border_color.unwrap_or(iced::Color::TRANSPARENT),
                width: self.border_width.unwrap_or(0.0),
                radius,
            };

            sp = sp.border(border);
        }
        
        if self.padding.is_some() {
            sp = sp.padding(get_padding(&self.padding));
        }
        if let Some(underline) = self.underline {
            sp = sp.underline(underline);
        }
        if let Some(strikethrough) = self.strikethrough {
            sp = sp.strikethrough(strikethrough);
        }
        if let Some(link) = self.link {
            sp = sp.link(link);
        }

        sp
    }
}

pub fn rich_text_callback(id: usize, link_id: usize) {
    invoke_callback_with_args(
        id,
        "on_link_click",
        "RichText",
        link_id,
        "def callback(wid: int, link_id: int)",
    );
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum RichTextParam {
    AlignBottomCenter,
    AlignBottomLeft,
    AlignBottomRight,
    AlignCenter,
    AlignCenterLeft,
    AlignCenterRight,
    AlignTopCenter,
    AlignTopLeft,
    AlignTopRight,
    Color,
    ColorAlpha,
    Fill,
    FontId,
    Height,
    HeightFill,
    LineHeight,
    Rgba,
    Show,
    Size,
    Width,
    WidthFill,
    WrappingGlyph,
    WrappingNone,
    WrappingWordGlyph,
}

impl WidgetParamUpdate for RichText {
    type Param = RichTextParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RichTextParam::AlignBottomCenter => set_t_value(&mut self.align_bottom_center, value, "RichTextParam::AlignBottomCenter"),
            RichTextParam::AlignBottomLeft => set_t_value(&mut self.align_bottom_left, value, "RichTextParam::AlignBottomLeft"),
            RichTextParam::AlignBottomRight => set_t_value(&mut self.align_bottom_right, value, "RichTextParam::AlignBottomRight"),
            RichTextParam::AlignCenter => set_t_value(&mut self.align_center, value, "RichTextParam::AlignCenter"),
            RichTextParam::AlignCenterLeft => set_t_value(&mut self.align_center_left, value, "RichTextParam::AlignCenterLeft"),
            RichTextParam::AlignCenterRight => set_t_value(&mut self.align_center_right, value, "RichTextParam::AlignCenterRight"),
            RichTextParam::AlignTopCenter => set_t_value(&mut self.align_top_center, value, "RichTextParam::AlignTopCenter"),
            RichTextParam::AlignTopLeft => set_t_value(&mut self.align_top_left, value, "RichTextParam::AlignTopLeft"),
            RichTextParam::AlignTopRight => set_t_value(&mut self.align_top_right, value, "RichTextParam::AlignTopRight"),
            RichTextParam::Color => set_t_value(&mut self.color, value, "RichTextParam::Color"),
            RichTextParam::ColorAlpha => set_t_value(&mut self.color_alpha, value, "RichTextParam::ColorAlpha"),
            RichTextParam::Fill => set_t_value(&mut self.fill, value, "RichTextParam::Fill"),
            RichTextParam::FontId => set_t_value(&mut self.font_id, value, "RichTextParam::FontId"),
            RichTextParam::Height => set_t_value(&mut self.height, value, "RichTextParam::Height"),
            RichTextParam::HeightFill => set_t_value(&mut self.height_fill, value, "RichTextParam::HeightFill"),
            RichTextParam::LineHeight => set_t_value(&mut self.line_height, value, "RichTextParam::LineHeight"),
            RichTextParam::Rgba => set_t_value(&mut self.rgba, value, "RichTextParam::Rgba"),
            RichTextParam::Show => set_t_value(&mut self.show, value, "RichTextParam::Show"),
            RichTextParam::Size => set_t_value(&mut self.size, value, "RichTextParam::Size"),
            RichTextParam::Width => set_t_value(&mut self.width, value, "RichTextParam::Width"),
            RichTextParam::WidthFill => set_t_value(&mut self.width_fill, value, "RichTextParam::WidthFill"),
            RichTextParam::WrappingGlyph => set_t_value(&mut self.wrapping_glyph, value, "RichTextParam::WrappingGlyph"),
            RichTextParam::WrappingNone => set_t_value(&mut self.wrapping_none, value, "RichTextParam::WrappingNone"),
            RichTextParam::WrappingWordGlyph => set_t_value(&mut self.wrapping_word_glyph, value, "RichTextParam::WrappingWordGlyph"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SpanParam {
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundGradientColorStop,
    BackgroundGradientColorStopAlpha,
    BackgroundGradientDegrees,
    BackgroundGradientRadians,
    BackgroundGradientRgbaStop,
    BackgroundRgba,
    BorderColor,
    BorderColorAlpha,
    BorderRadius,
    BorderRgba,
    BorderWidth,
    Color,
    ColorAlpha,
    FontId,
    LineHeight,
    Link,
    Rgba,
    Size,
    Strikethrough,
    Text,
    Underline,
}

impl WidgetParamUpdate for Span {
    type Param = SpanParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SpanParam::BackgroundColor => set_t_value(&mut self.background_color, value, "SpanParam::BackgroundColor"),
            SpanParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "SpanParam::BackgroundColorAlpha"),
            SpanParam::BackgroundGradientColorStop => set_t_value(&mut self.background_gradient_color_stop, value, "SpanParam::BackgroundGradientColorStop"),
            SpanParam::BackgroundGradientColorStopAlpha => set_t_value(&mut self.background_gradient_color_stop_alpha, value, "SpanParam::BackgroundGradientColorStopAlpha"),
            SpanParam::BackgroundGradientDegrees => set_t_value(&mut self.background_gradient_degrees, value, "SpanParam::BackgroundGradientDegrees"),
            SpanParam::BackgroundGradientRadians => set_t_value(&mut self.background_gradient_radians, value, "SpanParam::BackgroundGradientRadians"),
            SpanParam::BackgroundGradientRgbaStop => set_t_value(&mut self.background_gradient_rgba_stop, value, "SpanParam::BackgroundGradientRgbaStop"),
            SpanParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "SpanParam::BackgroundRgba"),
            SpanParam::BorderColor => set_t_value(&mut self.border_color, value, "SpanParam::BorderColor"),
            SpanParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "SpanParam::BorderColorAlpha"),
            SpanParam::BorderRadius => set_t_value(&mut self.border_radius, value, "SpanParam::BorderRadius"),
            SpanParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "SpanParam::BorderRgba"),
            SpanParam::BorderWidth => set_t_value(&mut self.border_width, value, "SpanParam::BorderWidth"),
            SpanParam::Color => set_t_value(&mut self.color, value, "SpanParam::Color"),
            SpanParam::ColorAlpha => set_t_value(&mut self.color_alpha, value, "SpanParam::ColorAlpha"),
            SpanParam::FontId => set_t_value(&mut self.font_id, value, "SpanParam::FontId"),
            SpanParam::LineHeight => set_t_value(&mut self.line_height, value, "SpanParam::LineHeight"),
            SpanParam::Link => set_t_value(&mut self.link, value, "SpanParam::Link"),
            SpanParam::Rgba => set_t_value(&mut self.rgba, value, "SpanParam::Rgba"),
            SpanParam::Size => set_t_value(&mut self.size, value, "SpanParam::Size"),
            SpanParam::Strikethrough => set_t_value(&mut self.strikethrough, value, "SpanParam::Strikethrough"),
            SpanParam::Text => set_t_value(&mut self.text, value, "SpanParam::Text"),
            SpanParam::Underline => set_t_value(&mut self.underline, value, "SpanParam::Underline"),
        }
    }
}
