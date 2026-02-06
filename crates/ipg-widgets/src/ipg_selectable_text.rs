//!ipg_selectable_text
#![allow(clippy::enum_variant_names)]
use iced::Color;
use iced::{Length, Element};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::{LineHeight, Shaping, Style};
use iced::widget::{MouseArea, Text};
use iced::mouse::Interaction;

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;

use ipg_alignment::{IpgHorizontalAlignment, IpgVerticalAlignment};
use ipg_helpers::{get_height, get_width, try_extract_boolean,
                    try_extract_f64, try_extract_string, try_extract_vec_f32};
use ipg_styling::{colors::get_color, try_extract_ipg_color};
use ipg_types::{Message, SLTXTMessage};


#[derive(Debug, Clone)]
pub struct IpgSelectableText {
        pub id: usize,
        pub parent_id: String,
        pub content: String,
        pub width: Length,
        pub height: Length,
        pub horizontal_alignment: IpgHorizontalAlignment,
        pub vertical_alignment: IpgVerticalAlignment,
        pub line_height: LineHeight,
        pub size: f32,
        pub show: bool,
        // pub font: Font,
        pub shaping: Shaping,
        pub text_color: Option<Color>,
}

impl IpgSelectableText {
    pub fn new( 
        id: usize,
        parent_id: String,
        content: String,
        width: Length,
        height: Length,
        horizontal_alignment: IpgHorizontalAlignment,
        vertical_alignment: IpgVerticalAlignment,
        line_height: LineHeight,
        size: f32,
        show: bool,
        // font: Font,
        shaping: Shaping,
        text_color: Option<Color>,
        ) -> Self {
        Self {
            id,
            parent_id,
            content,
            width,
            height,
            horizontal_alignment,
            vertical_alignment,
            line_height,
            size,
            show,
            // font: Font,
            shaping,
            text_color,
        }
    }
}

pub fn construct_selectable_text(sl_text: &IpgSelectableText) 
                                -> Option<Element<'_, Message>> {
    if !sl_text.show {
        return None
    }
    
    let hor_align = get_horizontal_align(&sl_text.horizontal_alignment);
    let vert_align = get_vertical_align(&sl_text.vertical_alignment);
    
    let content: Element<'_, SLTXTMessage> = 
                        Text::new(sl_text.content.clone())
                            .size(sl_text.size)
                            .line_height(sl_text.line_height)
                            .width(sl_text.width)
                            .height(sl_text.height)
                            .align_x(hor_align)
                            .align_y(vert_align)
                            // font: Font,
                            .shaping(sl_text.shaping)
                            .style(move|_theme|{
                                Style{color: sl_text.text_color}
                                }
                            )
                            .into();

    let ma: Element<'_, SLTXTMessage> = 
                MouseArea::new(content)
                    .on_press(SLTXTMessage::OnPress)
                    .on_release(SLTXTMessage::OnRelease)
                    .on_right_press(SLTXTMessage::OnRightPress)
                    .on_right_release(SLTXTMessage::OnRightRelease)
                    .on_middle_press(SLTXTMessage::OnMiddlePress)
                    .on_middle_release(SLTXTMessage::OnMiddleRelease)
                    .on_move(SLTXTMessage::OnMove)
                    .on_enter(SLTXTMessage::OnEnter)
                    .on_exit(SLTXTMessage::OnExit)
                    .interaction(Interaction::Pointer)
                    .into();

    Some(ma.map(move |message| Message::SelectableText(sl_text.id, message)))

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSelectableTextParam {
    Text,
    Width,
    WidthFill,
    Height,
    HeightFill,
    HorizontalAlign,
    VerticalAlign,
    LineHeight,
    TextColor, 
    TextRgba,
    Size,
    Show,
}

pub fn selectable_text_item_update(st: &mut IpgSelectableText,
                                        item: &PyObject,
                                        value: &PyObject
                                    )
{
    let update = try_extract_selectable_update(item);
    let name = "SelectableText".to_string();
    match update {
        IpgSelectableTextParam::Text => {
            st.content = try_extract_string(value, name);
        },
        IpgSelectableTextParam::Width => {
            let val = try_extract_f64(value, name);
            st.width = get_width(Some(val as f32), false);
        },
        IpgSelectableTextParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            st.width = get_width(None, val);
        },
        IpgSelectableTextParam::Height => {
            let val = try_extract_f64(value, name);
            st.height = get_height(Some(val as f32), false);
        },
        IpgSelectableTextParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            st.height = get_height(None, val);
        }
        IpgSelectableTextParam::HorizontalAlign => {
            st.horizontal_alignment = try_extract_hor_align(value);
        },
        IpgSelectableTextParam::VerticalAlign => {
            st.vertical_alignment = try_extract_vert_align(value);
        },
        IpgSelectableTextParam::LineHeight => {
            let val = try_extract_f64(value, name) as f32;
            st.line_height = LineHeight::Relative(val);
        },
        IpgSelectableTextParam::Size => {
            st.size = try_extract_f64(value, name) as f32;
        },
        IpgSelectableTextParam::TextColor => {
            let ipg_color = Some(try_extract_ipg_color(value, name));
            st.text_color = get_color(None, ipg_color, 1.0, false);
        },
        IpgSelectableTextParam::TextRgba => {
            let v = try_extract_vec_f32(value, name);
            let color_rgba = Some([v[0], v[1], v[2], v[3]]);
            st.text_color = get_color(color_rgba, None, 1.0, false);
        },
        IpgSelectableTextParam::Show => {
            st.show = try_extract_boolean(value, name);
        },
    }
}

fn try_extract_selectable_update(update_obj: &PyObject) -> IpgSelectableTextParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgSelectableTextParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button update extraction failed"),
        }
    })
}

fn try_extract_hor_align(value: &PyObject) -> IpgHorizontalAlignment {
    
    Python::attach(|py| {
        let res = value.extract::<IpgHorizontalAlignment>(py);
        match res {
            Ok(h_align) => h_align,
            Err(_) => panic!("IpgSectableText: unable to extract IpgSelectableTextHorAlign"),
        }
    })
}

fn try_extract_vert_align(value: &PyObject) -> IpgVerticalAlignment {
    
    Python::attach(|py| {
        let res = value.extract::<IpgVerticalAlignment>(py);
        match res {
            Ok(v_align) => v_align,
            Err(_) => panic!("IpgSelectableText: unable to extract IpgSelectableTextHorAlign"),
        }
    })
}

fn get_horizontal_align(ha: &IpgHorizontalAlignment) -> Horizontal {
    match ha {
        IpgHorizontalAlignment::Left => Horizontal::Left,
        IpgHorizontalAlignment::Center => Horizontal::Center,
        IpgHorizontalAlignment::Right => Horizontal::Right,
    }
}

fn get_vertical_align(va: &IpgVerticalAlignment) -> Vertical {
    match va {
        IpgVerticalAlignment::Top => Vertical::Top,
        IpgVerticalAlignment::Center => Vertical::Center,
        IpgVerticalAlignment::Bottom => Vertical::Bottom,
    }
}
