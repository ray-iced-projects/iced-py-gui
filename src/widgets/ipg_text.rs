//! ipg_text
use std::collections::HashMap;

use iced::{Element, alignment};
use iced::widget::text::{Style, Wrapping};
use iced::widget;

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::helpers::get_len;
use crate::state::Widgets;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value
};


#[derive(Debug, Clone)]
pub struct Text {
    pub id: usize,
    pub content: String,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub align_bottom_center: Option<bool>,
    pub align_bottom_left: Option<bool>,
    pub align_bottom_right: Option<bool>,
    pub align_center_left: Option<bool>,
    pub align_center_right: Option<bool>,
    pub align_center: Option<bool>,
    pub align_top_center: Option<bool>,
    pub align_top_left: Option<bool>,
    pub align_top_right: Option<bool>,
    pub font_id: Option<usize>,
    pub size: Option<f32>,
    pub line_height: Option<f32>,  // defaults 1.3 relative
    pub color: Option<Color>,
    pub color_alpha: Option<f32>,
    pub color_rgba: Option<[f32; 4]>,
    pub color_std: Option<TextColorStd>,
    pub wrapping_none: Option<bool>,
    pub wrapping_glyph: Option<bool>,
    pub wrapping_word_glyph: Option<bool>,
    pub show: bool,
}

impl Text {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message>> {

        if !self.show {
            return None
        }

        let font_opt = 
            self.lookup(widgets, self.font_id)
                .and_then(Widgets::as_font).cloned();

        let txt = 
            widget::Text::new(self.content.clone())
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height))
                .style(move|theme|{
                    if let Some(cs) = &self.color_std {
                        match cs {
                            TextColorStd::Base => widget::text::base(theme),
                            TextColorStd::Danger => widget::text::danger(theme),
                            TextColorStd::Primary => widget::text::primary(theme),
                            TextColorStd::Secondary => widget::text::secondary(theme),
                            TextColorStd::Success => widget::text::success(theme),
                            TextColorStd::Warning => widget::text::warning(theme),
                        }
                    } else { Style::default() }
                });

        let color= 
            Color::rgba_ipg_color_to_iced(self.color_rgba, &self.color, self.color_alpha);

        let txt = 
            if let Some(c) = color {
                txt.color(c)
            } else { txt };
            
        let txt = 
            if let Some(sz) = self.size {
                txt.size(sz)
            } else { txt };

        let txt = 
            if let Some(lh) = self.line_height {
                txt.line_height(lh)
            } else { txt };

        let txt = 
            if self.align_bottom_center == Some(true) {
                txt.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Bottom)
            } else { txt };

        let txt = 
            if self.align_bottom_left == Some(true) {
                txt.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Bottom)
            } else { txt };

        let txt = 
            if self.align_bottom_right == Some(true) {
                txt.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Bottom)
            } else { txt };

        let txt = 
        if self.align_center == Some(true) {
            txt.align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
        } else { txt };

        let txt = 
            if self.align_center_left == Some(true) {
                txt.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Center)
            } else { txt };

        let txt = 
            if self.align_center_right == Some(true) {
                txt.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Center)
            } else { txt };

        let txt = 
            if self.align_top_center == Some(true) {
                txt.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Top)
            } else { txt };

        let txt = 
            if self.align_top_left == Some(true) {
                txt.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Top)
            } else { txt };

        let txt = 
            if self.align_top_right == Some(true) {
                txt.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Top)
            } else { txt };
        
        // Set the font
        let txt = 
            if let Some(f) = font_opt {
                txt.font(f.to_iced())
            } else { txt };

        // default is word so not checked
        // set to Some(true) in case user sets to false versus None
        let txt = 
            if self.wrapping_none == Some(true) {
                txt.wrapping(Wrapping::None)
            } else if self.wrapping_glyph == Some(true) {
                txt.wrapping(Wrapping::Glyph)
            } else if self.wrapping_word_glyph == Some(true) {
                txt.wrapping(Wrapping::WordOrGlyph)
            } else { txt };

        Some(txt.into())

    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TextParam {
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
    ColorRgba,
    ColorStd,
    Content,
    Fill,
    Height,
    HeightFill,
    LineHeight,
    Show,
    Size,
    Width,
    WidthFill,
    WrappingGlyph,
    WrappingNone,
    WrappingWordGlyph,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TextColorStd {
    Base,
    Danger,
    Primary,
    Secondary,
    Success,
    Warning,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Text {
    type Param = TextParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TextParam::AlignBottomCenter => set_t_value(&mut self.align_bottom_center, value, "TextParam::AlignBottomCenter"),
            TextParam::AlignBottomLeft => set_t_value(&mut self.align_bottom_left, value, "AlignBottomLeft"),
            TextParam::AlignBottomRight => set_t_value(&mut self.align_bottom_right, value, "AlignBottomRight"),
            TextParam::AlignCenter => set_t_value(&mut self.align_center, value, "TextParam::AlignCenter"),
            TextParam::AlignCenterLeft => set_t_value(&mut self.align_center_left, value, "TextParam::AlignCenterLeft"),
            TextParam::AlignCenterRight => set_t_value(&mut self.align_center_right, value, "TextParam::AlignCenterRight"),
            TextParam::AlignTopCenter => set_t_value(&mut self.align_top_center, value, "TextParam::AlignTopCenter"),
            TextParam::AlignTopLeft => set_t_value(&mut self.align_top_left, value, "TextParam::AlignTopLeft"),
            TextParam::AlignTopRight => set_t_value(&mut self.align_top_right, value, "TextParam::AlignTopRight"),
            TextParam::Color => set_t_value(&mut self.color, value, "TextParam::Color"),
            TextParam::ColorAlpha => set_t_value(&mut self.color_alpha, value, "TextParam::ColorAlpha"),
            TextParam::ColorRgba => set_t_value(&mut self.color_rgba, value, "TextParam::ColorRgba"),
            TextParam::ColorStd => set_t_value(&mut self.color_std, value, "TextParam::ColorStd"),
            TextParam::Content => set_t_value(&mut self.content, value, "TextParam::Content"),
            TextParam::Fill => set_t_value(&mut self.fill, value, "TextParam::Fill"),
            TextParam::Height => set_t_value(&mut self.height, value, "TextParam::Height"),
            TextParam::HeightFill => set_t_value(&mut self.height_fill, value, "TextParam::HeightFill"),
            TextParam::LineHeight => set_t_value(&mut self.line_height, value, "TextParam::LineHeight"),
            TextParam::Show => set_t_value(&mut self.show, value, "TextParam::Show"),
            TextParam::Size => set_t_value(&mut self.size, value, "TextParam::Size"),
            TextParam::Width => set_t_value(&mut self.width, value, "TextParam::Width"),
            TextParam::WidthFill => set_t_value(&mut self.width_fill, value, "TextParam::WidthFill"),
            TextParam::WrappingGlyph => set_t_value(&mut self.wrapping_glyph, value, "TextParam::WrappingGlyph"),
            TextParam::WrappingNone => set_t_value(&mut self.wrapping_none, value, "TextParam::WrappingNone"),
            TextParam::WrappingWordGlyph => set_t_value(&mut self.wrapping_word_glyph, value, "TextParam::WrappingWordGlyph"),
        }
    }
}
