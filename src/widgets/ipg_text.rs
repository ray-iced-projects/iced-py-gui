//! ipg_text
use std::collections::HashMap;

use iced::widget::text::{LineHeight, Wrapping};
use iced::widget::{self, text};
use iced::{Element, Length, Pixels, Widget, alignment};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::state::Widgets;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};

#[derive(Debug, Clone, Default)]
pub struct Text {
    pub id: usize,
    pub content: String,
    pub format_id: Option<usize>,
    pub font_id: Option<usize>,
    pub selectable: bool,
    pub style_id: Option<usize>,
    pub selection_style_id: Option<usize>,
    pub style_std: Option<TextStyleStd>,
    pub show: bool,
}

impl Text {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &self, 
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message>> {

        if !self.show {
            return None;
        }

        let font_opt = self.lookup(widgets, self.font_id).and_then(Widgets::as_font).cloned();

        let format_opt = self
            .lookup(widgets, self.format_id)
            .and_then(Widgets::as_text_format)
            .cloned();
        
        let style_opt = self
            .lookup(widgets, self.style_id)
            .and_then(Widgets::as_text_style)
            .cloned();

        let fmt = if let Some(f) = format_opt {
            f.construct()
        } else {
            text::Format::default()
        };

        let style_std = self.style_std.clone();
        
        let txt = widget::Text::new(self.content.clone())
            .width(fmt.width)
            .height(fmt.height)
            .align_x(fmt.align_x)
            .align_y(fmt.align_y)
            .selectable(self.selectable)
            .wrapping(fmt.wrapping)
            .style(move |theme| {
                if let Some(tss) = &style_std {
                    match tss {
                        TextStyleStd::Base => widget::text::base(theme),
                        TextStyleStd::Danger => widget::text::danger(theme),
                        TextStyleStd::Primary => widget::text::primary(theme),
                        TextStyleStd::Secondary => widget::text::secondary(theme),
                        TextStyleStd::Success => widget::text::success(theme),
                        TextStyleStd::Warning => widget::text::warning(theme),
                    }
                } else {
                    if let Some(st) = style_opt.clone() {
                        st.construct()
                    } else {
                        text::Style::default()
                    }
                }
            });

        let txt = if let Some(sz) = fmt.size { 
            txt.size(sz) 
        } else { txt };

        let txt = if let Some(lh) = fmt.line_height {
            txt.line_height(lh)
        } else { txt };

        let txt = if let Some(font) = font_opt {
            txt.font(font.to_iced())
        } else { txt };

        Some(txt.boxed())
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TextParam {
    Content,
    FormatId,
    FontId,
    Selectable,
    StyleId,
    SelectionStyleId,
    StyleStd,
    Show,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TextFormatParam {
    AlignBottomCenter,
    AlignBottomLeft,
    AlignBottomRight,
    AlignCenter,
    AlignCenterLeft,
    AlignCenterRight,
    AlignTopCenter,
    AlignTopLeft,
    AlignTopRight,
    Fill,
    Height,
    HeightFill,
    LineHeight,
    Size,
    Width,
    WidthFill,
    WrappingGlyph,
    WrappingNone,
    WrappingWord,
    WrappingWordGlyph,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TextStyleStd {
    Base,
    Danger,
    Primary,
    Secondary,
    Success,
    Warning,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextFormat {
    pub id: usize,
    pub align_bottom_center: Option<bool>,
    pub align_bottom_left: Option<bool>,
    pub align_bottom_right: Option<bool>,
    pub align_center_left: Option<bool>,
    pub align_center_right: Option<bool>,
    pub align_center: Option<bool>,
    pub align_top_center: Option<bool>,
    pub align_top_left: Option<bool>,
    pub align_top_right: Option<bool>,
    pub fill: Option<bool>,
    pub height_fill: Option<bool>,
    pub height: Option<f32>,
    pub line_height: Option<f32>, // defaults 1.3 relative
    pub size: Option<f32>,
    pub width_fill: Option<bool>,
    pub width: Option<f32>,
    pub wrapping_glyph: Option<bool>,
    pub wrapping_none: Option<bool>,
    pub wrapping_word: Option<bool>,
    pub wrapping_word_glyph: Option<bool>,
}

impl TextFormat {
    pub fn construct(&self) -> text::Format {
        let width = match (self.fill, self.width_fill, self.width) {
            (Some(_), None, None) => Length::Fill,
            (None, Some(_), None) => Length::Fill,
            (None, None, Some(_)) => Length::Fixed(self.width.unwrap()),
            _ => Length::Shrink,
        };

        let height = match (self.fill, self.height_fill, self.height) {
            (Some(_), None, None) => Length::Fill,
            (None, Some(_), None) => Length::Fill,
            (None, None, Some(_)) => Length::Fixed(self.height.unwrap()),
            _ => Length::Shrink,
        };

        let size = if let Some(sz) = self.size {
            Some(Pixels(sz))
        } else {
            None
        };

        let line_height = if let Some(lh) = self.line_height {
            Some(LineHeight::Relative(lh))
        } else {
            None
        };

        let (align_x, align_y) = match (
            self.align_bottom_center,
            self.align_bottom_left,
            self.align_bottom_right,
            self.align_center,
            self.align_center_left,
            self.align_center_right,
            self.align_top_center,
            self.align_top_left,
            self.align_top_right,
        ) {
            (Some(true), ..) => (text::Alignment::Center, alignment::Vertical::Bottom),
            (_, Some(true), ..) => (text::Alignment::Left, alignment::Vertical::Bottom),
            (_, _, Some(true), ..) => (text::Alignment::Right, alignment::Vertical::Bottom),
            (_, _, _, Some(true), ..) => (text::Alignment::Center, alignment::Vertical::Center),
            (_, _, _, _, Some(true), ..) => (text::Alignment::Left, alignment::Vertical::Center),
            (_, _, _, _, _, Some(true), ..) => (text::Alignment::Right, alignment::Vertical::Center),
            (_, _, _, _, _, _, Some(true), ..) => (text::Alignment::Center, alignment::Vertical::Top),
            (_, _, _, _, _, _, _, Some(true), _) => (text::Alignment::Left, alignment::Vertical::Top),
            (_, _, _, _, _, _, _, _, Some(true)) => (text::Alignment::Right, alignment::Vertical::Top),
            _ => (text::Alignment::Default, alignment::Vertical::Top),
        };

        let wrapping = match (
            self.wrapping_glyph,
            self.wrapping_none,
            self.wrapping_word,
            self.wrapping_word_glyph,
        ) {
            (Some(true), _, _, _) => Wrapping::Glyph,
            (_, Some(true), _, _) => Wrapping::None,
            (_, _, Some(true), _) => Wrapping::Word,
            (_, _, _, Some(true)) => Wrapping::WordOrGlyph,
            _ => Wrapping::default(),
        };

        text::Format {
            width,
            height,
            size,
            line_height,
            align_x,
            align_y,
            wrapping,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub id: usize,
    pub color: Option<Color>,
    pub color_alpha: Option<f32>,
    pub color_rgba: Option<[f32; 4]>,
    pub selection_color: Option<Color>,
    pub selection_color_alpha: Option<f32>,
    pub selection_color_rgba: Option<[f32; 4]>,
}

impl TextStyle {
    pub fn construct<'a>(&'a self) -> text::Style {
        let color = Color::rgba_ipg_color_to_iced(self.color_rgba, &self.color, self.color_alpha);

        let selection = Color::rgba_ipg_color_to_iced(self.selection_color_rgba, &self.color, self.color_alpha);

        text::Style { color, selection }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TextStyleParam {
    Color,
    ColorAlpha,
    ColorRgba,
    SelectionColor,
    SelectionColorAlpha,
    SelectionColorRgba,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Text {
    type Param = TextParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TextParam::Content => set_t_value(&mut self.content, value, "TextParam::Content"),
            TextParam::FormatId => set_t_value(&mut self.format_id, value, "TextParam::FormatId"),
            TextParam::FontId => set_t_value(&mut self.font_id, value, "TextParam::FontId"),
            TextParam::Selectable => set_t_value(&mut self.selectable, value, "TextParam::Selectable"),
            TextParam::StyleId => set_t_value(&mut self.style_id, value, "TextParam::StyleId"),
            TextParam::Show => set_t_value(&mut self.show, value, "TextParam::Show"),
            TextParam::SelectionStyleId => {
                set_t_value(&mut self.selection_style_id, value, "TextParam::SelectionStyleId")
            }
            TextParam::StyleStd => set_t_value(&mut self.style_std, value, "TextParam::StyleStd"),
        }
    }
}

impl WidgetParamUpdate for TextFormat {
    type Param = TextFormatParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TextFormatParam::AlignBottomCenter => {
                set_t_value(&mut self.align_bottom_center, value, "TextParam::AlignBottomCenter")
            }
            TextFormatParam::AlignBottomLeft => set_t_value(&mut self.align_bottom_left, value, "AlignBottomLeft"),
            TextFormatParam::AlignBottomRight => set_t_value(&mut self.align_bottom_right, value, "AlignBottomRight"),
            TextFormatParam::AlignCenter => set_t_value(&mut self.align_center, value, "TextParam::AlignCenter"),
            TextFormatParam::AlignCenterLeft => {
                set_t_value(&mut self.align_center_left, value, "TextParam::AlignCenterLeft")
            }
            TextFormatParam::AlignCenterRight => {
                set_t_value(&mut self.align_center_right, value, "TextParam::AlignCenterRight")
            }
            TextFormatParam::AlignTopCenter => {
                set_t_value(&mut self.align_top_center, value, "TextParam::AlignTopCenter")
            }
            TextFormatParam::AlignTopLeft => set_t_value(&mut self.align_top_left, value, "TextParam::AlignTopLeft"),
            TextFormatParam::AlignTopRight => set_t_value(&mut self.align_top_right, value, "TextParam::AlignTopRight"),
            TextFormatParam::Fill => set_t_value(&mut self.fill, value, "TextParam::Fill"),
            TextFormatParam::Height => set_t_value(&mut self.height, value, "TextParam::Height"),
            TextFormatParam::HeightFill => set_t_value(&mut self.height_fill, value, "TextParam::HeightFill"),
            TextFormatParam::LineHeight => set_t_value(&mut self.line_height, value, "TextParam::LineHeight"),
            TextFormatParam::Size => set_t_value(&mut self.size, value, "TextParam::Size"),
            TextFormatParam::Width => set_t_value(&mut self.width, value, "TextParam::Width"),
            TextFormatParam::WidthFill => set_t_value(&mut self.width_fill, value, "TextParam::WidthFill"),
            TextFormatParam::WrappingGlyph => set_t_value(&mut self.wrapping_glyph, value, "TextParam::WrappingGlyph"),
            TextFormatParam::WrappingNone => set_t_value(&mut self.wrapping_none, value, "TextParam::WrappingNone"),
            TextFormatParam::WrappingWord => set_t_value(&mut self.wrapping_word, value, "TextParam::WrappingWord"),
            TextFormatParam::WrappingWordGlyph => {
                set_t_value(&mut self.wrapping_word_glyph, value, "TextParam::WrappingWordGlyph")
            }
        }
    }
}

impl WidgetParamUpdate for TextStyle {
    type Param = TextStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TextStyleParam::Color => set_t_value(&mut self.color, value, "TextStyleParam::Color"),
            TextStyleParam::ColorAlpha => set_t_value(&mut self.color_alpha, value, "TextStyleParam::ColorAlpha"),
            TextStyleParam::ColorRgba => set_t_value(&mut self.color_rgba, value, "TextStyleParam::ColorRgba"),
            TextStyleParam::SelectionColor => {
                set_t_value(&mut self.selection_color, value, "TextStyleParam::SelectionColor")
            }
            TextStyleParam::SelectionColorAlpha => set_t_value(
                &mut self.selection_color_alpha,
                value,
                "TextStyleParam::SlectionColorAlpha",
            ),
            TextStyleParam::SelectionColorRgba => set_t_value(
                &mut self.selection_color_rgba,
                value,
                "TextStyleParam::SelectionColorRgba",
            ),
        }
    }
}
