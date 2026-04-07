//! ipg_text
use std::collections::HashMap;

use iced::advanced::text;
use iced::{Element, Length, alignment};
use iced::widget::text::{Shaping, Style};
use iced::widget;

use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::state::Widgets;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_height, set_height_fill, 
    set_opt_bool_from_opt, set_opt_f32, set_opt_iced_color, 
    set_opt_iced_color_from_rgba, set_string, set_width, set_width_fill
};


#[derive(Debug, Clone)]
pub struct Text {
    pub id: usize,
    pub parent_id: String,
    pub content: String,
    pub size: Option<f32>,
    pub line_height: Option<f32>,  // defaults 1.3 relative
    pub width: Length,
    pub height: Length,
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
    pub shaping: Option<TextShaping>,
    pub show: bool,
    pub color: Option<iced::Color>,
    pub wrapping: Option<TextWrapping>,
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
                .width(self.width)
                .height(self.height)
                .style(move|_|{
                    let mut style = Style::default();
                    style.color = self.color;
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
            if self.align_bottom_left == Some(true) {
                txt.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Bottom)
            } else { txt };

        let txt = 
            if self.align_bottom_center == Some(true) {
                txt.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Bottom)
            } else { txt };

        let txt = 
            if self.align_bottom_right == Some(true) {
                txt.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Bottom)
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
        if self.align_center == Some(true) {
            txt.align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
        } else { txt };


        let txt = 
            if self.align_top_left == Some(true) {
                txt.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Top)
            } else { txt };

        let txt = 
            if self.align_top_center == Some(true) {
                txt.align_x(alignment::Horizontal::Center)
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

        let txt = 
            if let Some(sh) = &self.shaping {
                txt.shaping(sh.to_iced())
            } else { txt };

        let txt = 
            if let Some(wr) = &self.wrapping {
                txt.wrapping(wr.to_iced())
            } else { txt };

        Some(txt.into())

    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
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
    Content,
    Height,
    HeightFill,
    LineHeight,
    Show,
    Size,
    TextColor, 
    TextRgba,
    TextShaping,
    TextWrapping,
    Width,
    WidthFill,
}

// The wrapping strategy of some text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[pyclass(eq, eq_int)]
pub enum TextWrapping {
    TextNone,
    #[default]
    Glyph,
    Word,
    WordOrGlyph,
}

impl TextWrapping {
    pub fn to_iced(&self) -> text::Wrapping {
        match self {
            TextWrapping::TextNone => text::Wrapping::None,
            TextWrapping::Glyph => text::Wrapping::Glyph,
            TextWrapping::Word => text::Wrapping::Word,
            TextWrapping::WordOrGlyph => text::Wrapping::WordOrGlyph,
        }
    }

    pub fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(err) => panic!("Unable to extract python object for TextWrapping: {}", err),
            }
        }))
    }

    pub fn extract_opt(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<Option<Self>>(py);
            match res {
                Ok(val) => {
                    if val.is_none() { return None }
                    val
                },
                Err(err) => panic!("Unable to extract python TextWrapping: {}", err),
            }
        })  
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum TextShaping {
    Auto,
    Basic,
    Advanced,
}

impl TextShaping {
    pub fn to_iced(&self) -> Shaping {
        match self {
            TextShaping::Auto => Shaping::Auto,
            TextShaping::Basic => Shaping::Basic,
            TextShaping::Advanced => Shaping::Advanced,
        }
    }

    pub fn extract(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => Some(val),
                Err(err) => panic!("Unable to extract python TextShaping: {}", err),
            }
        })  
    }

    pub fn extract_opt(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<Option<Self>>(py);
            match res {
                Ok(val) => {
                    if val.is_none() { return None }
                    val
                },
                Err(err) => panic!("Unable to extract python TextShaping: {}", err),
            }
        })  
    }
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Text {
    type Param = TextParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TextParam::AlignBottomCenter => set_opt_bool_from_opt(&mut self.align_bottom_center, value, "AlignBottomCenter"),
            TextParam::AlignBottomLeft => set_opt_bool_from_opt(&mut self.align_bottom_left, value, "AlignBottomLeft"),
            TextParam::AlignBottomRight => set_opt_bool_from_opt(&mut self.align_bottom_right, value, "AlignBottomRight"),
            TextParam::AlignCenter => set_opt_bool_from_opt(&mut self.align_center, value, "AlignCenter"),
            TextParam::AlignCenterLeft => set_opt_bool_from_opt(&mut self.align_center_left, value, "AlignCenterLeft"),
            TextParam::AlignCenterRight => set_opt_bool_from_opt(&mut self.align_center_right, value, "AlignCenterRight"),
            TextParam::AlignTopCenter => set_opt_bool_from_opt(&mut self.align_top_center, value, "AlignTopCenter"),
            TextParam::AlignTopLeft => set_opt_bool_from_opt(&mut self.align_top_left, value, "AlignTopLeft"),
            TextParam::AlignTopRight => set_opt_bool_from_opt(&mut self.align_top_right, value, "AlignTopRight"),
            TextParam::Content => set_string(&mut self.content, value, "Content"),
            TextParam::Height => set_height(&mut self.height, value, "Height"),
            TextParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            TextParam::LineHeight => set_opt_f32(&mut self.line_height, value, "LineHeight"),
            TextParam::Show => set_bool(&mut self.show, value, "Show"),
            TextParam::Size => set_opt_f32(&mut self.size, value, "Size"),
            TextParam::TextColor  => set_opt_iced_color(&mut self.color, value, "TextColor"),
            TextParam::TextRgba => set_opt_iced_color_from_rgba(&mut self.color, value, "TextRgba"),
            TextParam::TextShaping => self.shaping = TextShaping::extract_opt(value),
            TextParam::TextWrapping => self.wrapping = TextWrapping::extract_opt(value),
            TextParam::Width => set_width(&mut self.width, value, "Width"),
            TextParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::IntoPyObjectExt;

    fn make_text() -> Text {
        Text {
            id: 0,
            parent_id: String::new(),
            content: String::from("test"),
            size: None,
            line_height: None,
            width: Length::Shrink,
            height: Length::Shrink,
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
            shaping: None,
            show: true,
            color: None,
            wrapping: None,
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
        assert_eq!(t.height, Length::Fixed(100.0));
        t.param_update(TextParam::Height, &py_none());
        assert_eq!(t.height, Length::Shrink);
    }

    #[test]
    fn test_height_fill() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::HeightFill, &py_obj(true));
        assert_eq!(t.height, Length::Fill);
        t.param_update(TextParam::HeightFill, &py_obj(false));
        assert_eq!(t.height, Length::Shrink);
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
    fn test_text_rgba() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::TextRgba, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(t.color.is_some());
        t.param_update(TextParam::TextRgba, &py_none());
        assert!(t.color.is_none());
    }

    #[test]
    fn test_width() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::Width, &py_obj(200.0f32));
        assert_eq!(t.width, Length::Fixed(200.0));
        t.param_update(TextParam::Width, &py_none());
        assert_eq!(t.width, Length::Shrink);
    }

    #[test]
    fn test_width_fill() {
        Python::initialize();
        let mut t = make_text();
        t.param_update(TextParam::WidthFill, &py_obj(true));
        assert_eq!(t.width, Length::Fill);
        t.param_update(TextParam::WidthFill, &py_obj(false));
        assert_eq!(t.width, Length::Shrink);
    }
}
