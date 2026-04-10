//! ipg_text
use std::collections::HashMap;

use iced::{Element, alignment};
use iced::widget::text::{Shaping, Style, Wrapping};
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
    pub parent_id: String,
    pub content: String,
    pub size: Option<f32>,
    pub line_height: Option<f32>,  // defaults 1.3 relative
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
    pub shaping_advanced: Option<bool>,
    pub shaping_basic: Option<bool>,
    pub show: bool,
    pub color: Option<Color>,
    pub color_alpha: Option<f32>,
    pub color_rgba: Option<[f32; 4]>,
    pub wrapping_none: Option<bool>,
    pub wrapping_glyph: Option<bool>,
    pub wrapping_word_glyph: Option<bool>,
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
        let color= 
            Color::rgba_ipg_color_to_iced(self.color_rgba, &self.color, self.color_alpha);

        let txt = 
            widget::Text::new(self.content.clone())
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height))
                .style(move|_|{
                    let mut style = Style::default();
                    style.color = color;
                    style
                    }
                );
            
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

        // default is auto so not checked
        // set to Some(true) in case user sets to false versus None
        let txt = 
            if self.shaping_advanced == Some(true) {
                txt.shaping(Shaping::Advanced)
            } else if self.shaping_basic == Some(true) {
                txt.shaping(Shaping::Basic)
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
    Content,
    Fill,
    Height,
    HeightFill,
    LineHeight,
    ShapingAdvanced,
    ShapingBasic,
    Show,
    Size,
    Width,
    WidthFill,
    WrappingGlyph,
    WrappingNone,
    WrappingWordGlyph,
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
            TextParam::Content => set_t_value(&mut self.content, value, "TextParam::Content"),
            TextParam::Fill => set_t_value(&mut self.fill, value, "TextParam::Fill"),
            TextParam::Height => set_t_value(&mut self.height, value, "TextParam::Height"),
            TextParam::HeightFill => set_t_value(&mut self.height_fill, value, "TextParam::HeightFill"),
            TextParam::LineHeight => set_t_value(&mut self.line_height, value, "TextParam::LineHeight"),
            TextParam::ShapingAdvanced => set_t_value(&mut self.shaping_advanced, value, "TextParam::ShapingAdvanced"),
            TextParam::ShapingBasic => set_t_value(&mut self.shaping_basic, value, "TextParam::ShapingBasic"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{IntoPyObjectExt, Python};

    fn make_text() -> Text {
        Text {
            id: 0,
            parent_id: String::new(),
            content: String::from("test"),
            size: None,
            line_height: None,
            width: None,
            width_fill: None,
            height: None,
            height_fill: None,
            fill: None,
            align_bottom_center: None,
            align_bottom_left: None,
            align_bottom_right: None,
            align_center_left: None,
            align_center_right: None,
            align_center: None,
            align_top_center: None,
            align_top_left: None,
            align_top_right: None,
            font_id: None,
            shaping_advanced: None,
            shaping_basic: None,
            show: true,
            color: None,
            color_alpha: None,
            color_rgba: None,
            wrapping_none: None,
            wrapping_glyph: None,
            wrapping_word_glyph: None,
        }
    }

    /// Helper to create a PyObject from a Rust value.
    fn py_obj<T>(val: T) -> PyObject 
    where
        for<'py> T: pyo3::IntoPyObject<'py>,
    {
        Python::attach(|py| {
            val.into_py_any(py).unwrap()
        })
    }

    fn py_none() -> PyObject {
        Python::attach(|py| py.None())
    }

    #[test]
    fn test_align_bottom_center() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::AlignBottomCenter, &py_obj(true));
        assert_eq!(t.align_bottom_center, Some(true));
        t.param_update(TextParam::AlignBottomCenter, &py_none());
        assert_eq!(t.align_bottom_center, None);
    }

    #[test]
    fn test_align_bottom_left() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::AlignBottomLeft, &py_obj(true));
        assert_eq!(t.align_bottom_left, Some(true));
        t.param_update(TextParam::AlignBottomLeft, &py_none());
        assert_eq!(t.align_bottom_left, None);
    }

    #[test]
    fn test_align_bottom_right() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::AlignBottomRight, &py_obj(true));
        assert_eq!(t.align_bottom_right, Some(true));
        t.param_update(TextParam::AlignBottomRight, &py_none());
        assert_eq!(t.align_bottom_right, None);
    }

    #[test]
    fn test_color() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::Color, &py_obj(Color::BLACK));
        assert_eq!(t.color, Some(Color::BLACK));
        t.param_update(TextParam::Color, &py_none());
        assert!(t.color.is_none());
    }

    #[test]
    fn test_color_alpha() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::ColorAlpha, &py_obj(0.5));
        assert_eq!(t.color_alpha, Some(0.5));
        t.param_update(TextParam::ColorAlpha, &py_none());
        assert!(t.color_alpha.is_none());
    }

    #[test]
    fn test_color_rgba() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::ColorRgba, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert_eq!(t.color_rgba, Some([1.0f32, 0.0, 0.0, 1.0]));
        t.param_update(TextParam::ColorRgba, &py_none());
        assert!(t.color.is_none());
    }

    #[test]
    fn test_content() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::Content, &py_obj("Hello"));
        assert_eq!(t.content, "Hello");
    }

    #[test]
    fn test_height() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::Height, &py_obj(100.0f32));
        assert_eq!(get_len(t.fill, t.height_fill, t.height), Length::Fixed(100.0f32));
        t.param_update(TextParam::Height, &py_none());
        assert_eq!(get_len(t.fill, t.height_fill, t.height), Length::Shrink);
    }

    #[test]
    fn test_height_fill() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::HeightFill, &py_obj(true));
        assert_eq!(get_len(t.fill, t.height_fill, t.height), Length::Fill);
        t.param_update(TextParam::HeightFill, &py_obj(false));
        assert_eq!(get_len(t.fill, t.height_fill, t.height), Length::Shrink);
    }

    #[test]
    fn test_fill() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::Fill, &py_obj(true));
        assert_eq!(t.fill, Some(true));
        t.param_update(TextParam::Fill, &py_obj(false));
        assert_eq!(t.fill, Some(false));
    }

    #[test]
    fn test_line_height() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::LineHeight, &py_obj(1.5f32));
        assert_eq!(t.line_height, Some(1.5));
        t.param_update(TextParam::LineHeight, &py_none());
        assert_eq!(t.line_height, None);
    }

    #[test]
    fn test_show() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::Show, &py_obj(false));
        assert!(!t.show);
    }

    #[test]
    fn test_size() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::Size, &py_obj(20.0f32));
        assert_eq!(t.size, Some(20.0));
        t.param_update(TextParam::Size, &py_none());
        assert_eq!(t.size, None);
    }

    #[test]
    fn test_width() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::Width, &py_obj(200.0f32));
        assert_eq!(get_len(t.fill, t.width_fill, t.width), Length::Fixed(200.0));
        t.param_update(TextParam::Width, &py_none());
        assert_eq!(get_len(t.fill, t.width_fill, t.width), Length::Shrink);
    }

    #[test]
    fn test_width_fill() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::WidthFill, &py_obj(true));
        assert_eq!(get_len(t.fill, t.width_fill, t.width), Length::Fill);
        t.param_update(TextParam::WidthFill, &py_obj(false));
        assert_eq!(get_len(t.fill, t.width_fill, t.width), Length::Shrink);
    }
}
