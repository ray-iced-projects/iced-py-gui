//! ipg_text
use iced::advanced::text;
use iced::{Color, Element, Length};
use iced::widget::text::Style;
use iced::widget::Text;

use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::state::IpgWidgets;
use crate::widgets::enums::{IpgHorizontalAlignment, 
    IpgShaping, IpgVerticalAlignment, h_v_centered};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_halign, 
    set_height, set_height_fill, set_iced_color_from_rgba, 
    set_opt_f32, set_opt_iced_color, set_opt_text_shaping, 
    set_string, set_valign, set_width, set_width_fill
};


#[derive(Debug, Clone)]
pub struct IpgText {
    pub id: usize,
    pub parent_id: String,
    pub content: String,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub width: Length,
    pub height: Length,
    pub center: Option<bool>,
    pub align_x: Option<IpgHorizontalAlignment>,
    pub align_y: Option<IpgVerticalAlignment>,
    pub font_id: Option<usize>,
    pub shaping: Option<IpgShaping>,
    pub show: bool,
    pub style: Option<Color>,
    pub wrapping: Option<IpgWrapping>,
}

pub fn construct_text<'a>(
    ipg_text: &'a IpgText,
    font_opt:  Option<&'a IpgWidgets>) 
    -> Option<Element<'a, Message>> {

    if !ipg_text.show {
        return None
    }

    let txt = Text::new(ipg_text.content.clone()
                        )
                        .style(move|_theme|{
                            let mut style = Style::default();
                            style.color = ipg_text.style;
                            style
                            }
                        );
    let txt = 
        if let Some(sz) = ipg_text.size {
            txt.size(sz)
        } else { txt };

    let txt = 
        if let Some(lh) = ipg_text.line_height {
            txt.line_height(lh)
        } else { txt };

    let txt = 
        if ipg_text.center == Some(true) {
            let (h, v) = h_v_centered();
            txt.align_x(h).align_y(v)
        } else { txt };

    let txt = 
        if let Some(align) = &ipg_text.align_x {
            txt.align_x(align.to_iced())
        } else { txt };

    let txt = 
        if let Some(align) = &ipg_text.align_y {
            txt.align_y(align.to_iced())
        } else { txt };
    
    let txt = 
        if let Some(wd) = font_opt {
            match wd {
                IpgWidgets::IpgFont(font) => {
                    txt.font(font.to_iced())
                },
                _ => txt
            }
        } else { txt };

    let txt = 
        if let Some(sh) = &ipg_text.shaping {
            txt.shaping(sh.to_iced())
        } else { txt };

    let txt = 
        if let Some(wr) = &ipg_text.wrapping {
            txt.wrapping(wr.to_iced())
        } else { txt };

    Some(txt.into())

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
    Shaping,
    Size,
    TextColor, 
    TextRgba,
    Width,
    WidthFill,
    Wrapping,
}

// The wrapping strategy of some text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[pyclass(eq, eq_int)]
pub enum IpgWrapping {
    None,
    #[default]
    Glyph,
    Word,
    WordOrGlyph,
}

impl IpgWrapping {
    fn to_iced(&self) -> text::Wrapping {
        match self {
            IpgWrapping::None => text::Wrapping::None,
            IpgWrapping::Glyph => text::Wrapping::Glyph,
            IpgWrapping::Word => text::Wrapping::Word,
            IpgWrapping::WordOrGlyph => text::Wrapping::WordOrGlyph,
        }
    }

    fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("Unable to extract python object for IpgWrapping"),
            }
        }))
    }
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgText {
    type Param = IpgTextParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgTextParam::Content => set_string(&mut self.content, value, name),
            IpgTextParam::Height => set_height(&mut self.height, value, name),
            IpgTextParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgTextParam::AlignX => set_halign(&mut self.align_x, value, name),
            IpgTextParam::AlignY => set_valign(&mut self.align_y, value, name),
            IpgTextParam::LineHeight => set_opt_f32(&mut self.line_height, value, name),
            IpgTextParam::Shaping => set_opt_text_shaping(&mut self.shaping, value, name),
            IpgTextParam::Show => set_bool(&mut self.show, value, name),
            IpgTextParam::Size => set_opt_f32(&mut self.size, value, name),
            IpgTextParam::TextColor  => 
                set_opt_iced_color(&mut self.style, value, name),
            IpgTextParam::TextRgba => 
                set_iced_color_from_rgba(&mut self.style, value, name),
            IpgTextParam::Width => set_width(&mut self.width, value, name),
            IpgTextParam::WidthFill => set_width_fill(&mut self.width, value, name),
            IpgTextParam::Wrapping => self.wrapping = IpgWrapping::extract(value),
        }
    }
}
