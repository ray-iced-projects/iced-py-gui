//! ipg_text
use iced::{Color, Element, Font, Length};
use iced::widget::text::Style;
use iced::widget::Text;

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::{get_height, get_width, 
    try_extract_boolean, try_extract_f32, try_extract_f64, 
    try_extract_ipg_color, try_extract_string, try_extract_vec_f32};
use crate::app::Message;
use crate::widgets::enums::{IpgHorizontalAlignment, 
    IpgShaping, IpgVerticalAlignment, h_v_centered};


#[derive(Debug, Clone)]
pub struct IpgText {
    pub id: usize,
    pub parent_id: String,
    pub content: String,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub width: Length,
    pub height: Length,
    pub centered: Option<bool>,
    pub align_x: Option<IpgHorizontalAlignment>,
    pub align_y: Option<IpgVerticalAlignment>,
    pub font: Option<String>,
    pub shaping: Option<IpgShaping>,
    pub show: bool,
    pub style: Option<Color>,
}

pub fn construct_text(
    ipg_text: &IpgText) 
    -> Option<Element<'_, Message>> {

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
    let txt = if let Some(sz) = ipg_text.size {
        txt.size(sz)
    } else {
        txt
    };

    let txt = if let Some(lh) = ipg_text.line_height {
        txt.line_height(lh)
    } else { txt };

    let txt = if ipg_text.centered == Some(true) {
        let (h, v) = h_v_centered();
        txt.align_x(h).align_y(v)
    } else {
        txt
    };

    let txt = if let Some(align) = &ipg_text.align_x {
        txt.align_x(align.to_iced())
    } else {
        txt
    };

    let txt = if let Some(align) = &ipg_text.align_y {
        txt.align_y(align.to_iced())
    } else {
        txt
    };
    
    let txt = if let Some(name) = &ipg_text.font {
        let name: &'static str = Box::leak(name.clone().into_boxed_str());
        txt.font(Font::with_name(name))
    } else {
        txt
    };

    let txt = if let Some(sh) = &ipg_text.shaping {
        txt.shaping(sh.to_iced())
    } else {
        txt
    };

    Some(txt.into())

}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTextParam {
    Content,
    Font,
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
}

pub fn text_widget_update(txt: &mut IpgText, 
                        item: &PyObject, 
                        value: &PyObject) {

    let update = try_extract_text_update(item);
    let name = "Text".to_string();
    match update {
        IpgTextParam::Content => {
            txt.content = try_extract_string(value, name);
        },
        IpgTextParam::Font => {
            txt.font = Some(try_extract_string(value, name));
        },
        IpgTextParam::Height => {
            let val = try_extract_f64(value, name);
            txt.height = get_height(Some(val as f32), false); 
        },
        IpgTextParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            txt.height = get_height(None, val);
        },
        IpgTextParam::AlignX => {
            txt.align_x = Some(try_extract_hor_alignment(value));
        },
        IpgTextParam::AlignY => {
            txt.align_y = Some(try_extract_vert_alignment(value));
        },
        IpgTextParam::LineHeight => {
            txt.line_height = Some(try_extract_f32(value, name));
        },
        IpgTextParam::Shaping => {
            txt.shaping = IpgShaping::extract(value);
        },
        IpgTextParam::Show => {
            txt.show = try_extract_boolean(value, name);
        },
        IpgTextParam::Size => {
            txt.size = Some(try_extract_f32(value, name));
        },
        IpgTextParam::TextColor => {
            let ipg_color = Some(try_extract_ipg_color(value, name));
            txt.style = 
                IpgColor::rgba_ipg_color_to_iced(None, ipg_color, 1.0, false);
        },
        IpgTextParam::TextRgba => {
            let v = try_extract_vec_f32(value, name);
            let color_rgba = Some([v[0], v[1], v[2], v[3]]);
            txt.style = 
                IpgColor::rgba_ipg_color_to_iced(color_rgba, None, 1.0, false);
        },
        IpgTextParam::Width => {
            let val = try_extract_f64(value, name);
            txt.width = get_width(Some(val as f32), false);
        },
        IpgTextParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            txt.width = get_width(None, val);
        },
    }
}


fn try_extract_text_update(update_obj: &PyObject) -> IpgTextParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgTextParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Text update extraction failed"),
        }
    })
}

fn try_extract_hor_alignment(update_obj: &PyObject) -> IpgHorizontalAlignment {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgHorizontalAlignment>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Text HorizontalAlignment extraction failed"),
        }
    })
}

fn try_extract_vert_alignment(update_obj: &PyObject) -> IpgVerticalAlignment {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgVerticalAlignment>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Text VerticalAlignment extraction failed"),
        }
    })
}
