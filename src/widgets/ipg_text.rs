//! ipg_text
use iced::{Color, Element, Font, Length};
use iced::widget::text::Style;
use iced::widget::Text;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::try_extract_vec_f32;
use crate::app::Message;
use crate::widgets::enums::{IpgHorizontalAlignment, 
    IpgShaping, IpgVerticalAlignment, h_v_centered};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_halign, set_height, 
    set_height_fill, set_opt_f32, set_opt_string, 
    set_opt_text_shaping, set_string, set_valign, 
    set_width, set_width_fill
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

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgText {
    type Param = IpgTextParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgTextParam::Content    => set_string(&mut self.content, value, name),
            IpgTextParam::Font       => set_opt_string(&mut self.font, value, name),
            IpgTextParam::Height     => set_height(&mut self.height, value, name),
            IpgTextParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgTextParam::AlignX     => set_halign(&mut self.align_x, value, name),
            IpgTextParam::AlignY     => set_valign(&mut self.align_y, value, name),
            IpgTextParam::LineHeight => set_opt_f32(&mut self.line_height, value, name),
            IpgTextParam::Shaping    => set_opt_text_shaping(&mut self.shaping, value, name),
            IpgTextParam::Show       => set_bool(&mut self.show, value, name),
            IpgTextParam::Size       => set_opt_f32(&mut self.size, value, name),
            IpgTextParam::TextColor  => {
                let ipg_color = Some(IpgColor::extract(value, name));
                self.style = IpgColor::rgba_ipg_color_to_iced(None, ipg_color, 1.0, false);
            }
            IpgTextParam::TextRgba   => {
                let v = try_extract_vec_f32(value, name);
                let color_rgba = Some([v[0], v[1], v[2], v[3]]);
                self.style = IpgColor::rgba_ipg_color_to_iced(color_rgba, None, 1.0, false);
            }
            IpgTextParam::Width      => set_width(&mut self.width, value, name),
            IpgTextParam::WidthFill  => set_width_fill(&mut self.width, value, name),
        }
    }
}
