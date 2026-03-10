//! ipg_text
use std::collections::HashMap;

use iced::advanced::text;
use iced::{Color, Element, Length, alignment};
use iced::widget::text::{Shaping, Style};
use iced::widget::Text;

use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::state::IpgWidgets;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_height, set_height_fill, 
    set_iced_color_from_rgba, set_opt_bool_from_opt, 
    set_opt_f32, set_opt_iced_color, set_string, set_width, set_width_fill
};


#[derive(Debug, Clone)]
pub struct IpgText {
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
    pub color: Option<Color>,
    pub wrapping: Option<TextWrapping>,
}

impl IpgText {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, IpgWidgets>,
    ) -> Option<Element<'a, Message>> {

        if !self.show {
            return None
        }

        let font_opt = 
            self.lookup(widgets, self.font_id)
                .and_then(IpgWidgets::as_font).cloned();

        let txt = 
            Text::new(self.content.clone()
                )
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
pub enum IpgTextParam {
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
                Err(_) => panic!("Unable to extract python object for TextWrapping"),
            }
        }))
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

    pub fn extract(value: &PyObject) -> Option<TextShaping> {
        Python::attach(|py| {
            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => panic!("Unable to extract python TextShaping"),
            }
        })  
    }
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgText {
    type Param = IpgTextParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgTextParam::AlignBottomCenter => set_opt_bool_from_opt(&mut self.align_bottom_center, value, name),
            IpgTextParam::AlignBottomLeft => set_opt_bool_from_opt(&mut self.align_bottom_left, value, name),
            IpgTextParam::AlignBottomRight => set_opt_bool_from_opt(&mut self.align_bottom_right, value, name),
            IpgTextParam::AlignCenter => set_opt_bool_from_opt(&mut self.align_center, value, name),
            IpgTextParam::AlignCenterLeft => set_opt_bool_from_opt(&mut self.align_center_left, value, name),
            IpgTextParam::AlignCenterRight => set_opt_bool_from_opt(&mut self.align_center_right, value, name),
            IpgTextParam::AlignTopCenter => set_opt_bool_from_opt(&mut self.align_top_center, value, name),
            IpgTextParam::AlignTopLeft => set_opt_bool_from_opt(&mut self.align_top_left, value, name),
            IpgTextParam::AlignTopRight => set_opt_bool_from_opt(&mut self.align_top_right, value, name),
            IpgTextParam::Content => set_string(&mut self.content, value, name),
            IpgTextParam::Height => set_height(&mut self.height, value, name),
            IpgTextParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgTextParam::LineHeight => set_opt_f32(&mut self.line_height, value, name),
            IpgTextParam::Show => set_bool(&mut self.show, value, name),
            IpgTextParam::Size => set_opt_f32(&mut self.size, value, name),
            IpgTextParam::TextColor  => set_opt_iced_color(&mut self.color, value, name),
            IpgTextParam::TextRgba => set_iced_color_from_rgba(&mut self.color, value, name),
            IpgTextParam::TextShaping => self.shaping = TextShaping::extract(value),
            IpgTextParam::TextWrapping => self.wrapping = TextWrapping::extract(value),
            IpgTextParam::Width => set_width(&mut self.width, value, name),
            IpgTextParam::WidthFill => set_width_fill(&mut self.width, value, name),
        }
    }
}
