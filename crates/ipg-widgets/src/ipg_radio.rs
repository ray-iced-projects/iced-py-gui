//!ipg_radio


use iced::widget::radio::{self, Status};
use iced::{Color, Element, Length, Padding, Pixels, Theme};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{Column, Radio, Row};

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use ipg_helpers::{get_height, get_padding_f64, get_width, 
    try_extract_f64, try_extract_f64_option, try_extract_i64_option, try_extract_u16, 
    try_extract_vec_f64, try_extract_vec_str, try_extract_boolean};
use ipg_styling::{colors::get_color, try_extract_ipg_color, try_extract_rgba_color};
use ipg_types::{CHOICES, Message, RDMessage};
use super::ipg_enums::IpgWidgets;


#[derive(Debug, Clone)]
pub struct IpgRadio {
    pub id: usize,
    pub parent_id: String,
    pub labels: Vec<String>,
    pub direction: IpgRadioDirection,
    pub spacing: f32,
    pub padding: Padding,
    pub show: bool,
    pub is_selected: Option<usize>,
    pub width: Length,
    pub height: Length,
    pub size: f32,
    pub text_spacing: f32,
    pub text_size: f32,
    pub text_line_height: LineHeight,
    pub text_shaping: Shaping,
    pub group_index: usize,
    // pub font: Option<Font>,
    pub style_id: Option<usize>,
}

impl IpgRadio {
    pub fn new( 
        id: usize,
        parent_id: String,
        labels: Vec<String>,
        direction: IpgRadioDirection,
        spacing: f32,
        padding: Padding,
        show: bool,
        is_selected: Option<usize>,
        width: Length,
        height: Length,
        size: f32,
        text_spacing: f32,
        text_size: f32,
        text_line_height: LineHeight,
        text_shaping: Shaping,
        radio_index: usize,
        // font: Option<Font>,
        style_id: Option<usize>,
        ) -> Self {
        Self {
            id,
            parent_id,
            labels,
            direction,
            spacing,
            padding,
            show,
            is_selected,
            width,
            height,
            size,
            text_spacing,
            text_size,
            text_line_height,
            text_shaping,
            group_index: radio_index,
            // font: None,
            style_id,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IpgRadioStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    pub dot_color: Option<Color>,
    pub dot_color_hovered: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub text_color: Option<Color>,
}

impl IpgRadioStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        background_color_hovered: Option<Color>,
        dot_color: Option<Color>,
        dot_color_hovered: Option<Color>,
        border_color: Option<Color>,
        border_width: Option<f32>,
        text_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            background_color,
            background_color_hovered,
            dot_color,
            dot_color_hovered,
            border_color,
            border_width,
            text_color,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRadioDirection {
    Horizontal,
    Vertical,
}

pub fn construct_radio<'a>(radio: &'a IpgRadio, 
                        style_opt: Option<&IpgWidgets>) 
                        -> Option<Element<'a, Message>> {
    
    if !radio.show {
        return None
    }

    let style_opt = get_radio_style(style_opt);

    let selected = radio.is_selected
                                    .map(|is| CHOICES[radio.group_index][is]);

    let mut radio_elements = vec![];

    for (i, label) in  radio.labels.iter().enumerate() {
        let style: Option<IpgRadioStyle> = 
            style_opt.map(|st| IpgRadioStyle{
                id: st.id, 
                background_color: st.background_color, 
                background_color_hovered: st.background_color_hovered, 
                dot_color: st.dot_color, 
                dot_color_hovered: st.dot_color_hovered, 
                border_color: st.border_color, 
                border_width: st.border_width, 
                text_color: st.text_color });

        radio_elements.push(Radio::new(label.clone(), 
                                        CHOICES[radio.group_index][i],
                                        selected,
                                        RDMessage::RadioSelected
                                    )
                                    .size(radio.size)
                                    .spacing(radio.text_spacing)
                                    .text_size(radio.text_size)
                                    .text_line_height(radio.text_line_height)
                                    .text_shaping(radio.text_shaping)
                                    .style(move|theme: &Theme, status| {
                                        get_styling(theme, status, 
                                        style,
                                        )})
                                    .into());
    }

    let rd: Element<RDMessage> = match radio.direction {
            IpgRadioDirection::Horizontal => Row::with_children(radio_elements)
                                                    .spacing(radio.spacing)
                                                    .padding(radio.padding)
                                                    .width(radio.width)
                                                    .height(radio.height)
                                                    .into(),
            IpgRadioDirection::Vertical => Column::with_children(radio_elements)
                                                    .spacing(radio.spacing)
                                                    .padding(radio.padding)
                                                    .width(radio.width)
                                                    .height(radio.height)
                                                    .into(),
    };

    Some(rd.map(move |message| Message::Radio(radio.id, message)))

}

// fn match_widgets (widget: &mut IpgWidgets) -> &mut IpgRadio {
    
//     match widget {
//         IpgWidgets::IpgRadio(radio) => radio,
//         _ => panic!("Radio expected to find radio in IpgWidgets")
//     }
// }

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRadioParam {
    Direction,
    Labels,
    Padding,
    SelectedIndex,
    Show,
    Size,
    Spacing,
    StyleId,
    TextSpacing,
    TextSize,
    LineHeightPixels,
    LineHeightRelative,
    Width,
    WidthFill,
    Height,
    HeightFill,
}

pub fn radio_item_update(rd: &mut IpgRadio,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_radio_update(item);
    let name = "Radio".to_string();
    match update {
        IpgRadioParam::Direction => {
            rd.direction = try_extract_radio_direction(value);
        },
        IpgRadioParam::Labels => {
            rd.labels = try_extract_vec_str(value, name);
        },
        IpgRadioParam::Padding => {
            let val = try_extract_vec_f64(value, name);
            rd.padding =  get_padding_f64(val);
        },
        IpgRadioParam::SelectedIndex => {
            let index_opt = try_extract_i64_option(value);

            let selected_index = match index_opt {
                Some(index)  => index as usize,
                None => {
                    rd.is_selected = None;
                    return
                }
            };
            
            if selected_index > rd.labels.len()-1 {
                panic!("Radio selected_index is greater than the size of the labels")
            } else {
                rd.is_selected = Some(selected_index);
            }
        },
        IpgRadioParam::Show => {
            rd.show = try_extract_boolean(value, name);
        },
        IpgRadioParam::Size => {
            rd.size = try_extract_f64(value, name) as f32;
        },
        IpgRadioParam::Spacing => {
            rd.spacing = try_extract_f64(value, name) as f32;
        },
        IpgRadioParam::StyleId => {
            rd.style_id = Some(try_extract_f64(value, name) as usize);
        },
        IpgRadioParam::TextSpacing => {
            rd.text_spacing = try_extract_f64(value, name) as f32;
        },
        IpgRadioParam::TextSize => {
            rd.text_size = try_extract_f64(value, name) as f32;
        },
        IpgRadioParam::LineHeightPixels => {
            let val = try_extract_u16(value, name);
            rd.text_line_height = LineHeight::Absolute(Pixels(val.into()));
        },
        IpgRadioParam::LineHeightRelative => {
            let val = try_extract_f64(value, name) as f32;
            rd.text_line_height = LineHeight::Relative(val);
        },
        IpgRadioParam::Width => {
            match try_extract_f64_option(value) {
                Some(val) => rd.width = get_width(Some(val as f32), false),
                None => rd.width = Length::Shrink,
            }
        },
        IpgRadioParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            if val {
                rd.width = get_width(None, val);
            } else {
                rd.width = Length::Shrink;
            }
        },
        IpgRadioParam::Height => {
            match try_extract_f64_option(value) {
                Some(val) => rd.height = get_height(Some(val as f32), false),
                None => rd.height = Length::Shrink,
            }
        },
        IpgRadioParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            if val {
                rd.height = get_height(None, val);
            } else {
                rd.height = Length::Shrink;
            } 
        },
    }

}


pub fn try_extract_radio_update(update_obj: &PyObject) -> IpgRadioParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgRadioParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Radio update extraction failed"),
        }
    })
}


pub fn try_extract_radio_direction(direct_obj: &PyObject) -> IpgRadioDirection {
    Python::attach(|py| {
        let res = direct_obj.extract::<IpgRadioDirection>(py);
            
        match res {
            Ok(direction) => direction,
            Err(_) => panic!("RadioDirection failed to extract."),
        }
    })  
}

pub fn get_styling(theme: &Theme, status: Status, 
                    style_opt: Option<IpgRadioStyle>,
                    ) -> radio::Style {
    
    if style_opt.is_none() {
        return radio::default(theme, status)
    }
    
    let mut base_style = radio::default(theme, status);

    let style = style_opt.unwrap();

    base_style.text_color = style.text_color;
    
    if style.background_color.is_some() {
        base_style.background = style.background_color.unwrap().into();
    }

    if style.dot_color.is_some() {
        base_style.dot_color = style.dot_color.unwrap();
    }
    
    // border color changes to inner color during hover
    if style.border_color.is_some() {
        base_style.border_color = style.border_color.unwrap();
    }
    
    if style.border_width.is_some() {
        base_style.border_width = style.border_width.unwrap();
    }
        

    match status {
        Status::Active{..} => base_style,
        Status::Hovered{..} => {
            if style.background_color_hovered.is_some() {
                base_style.background = style.background_color_hovered.unwrap().into();
            }
            if style.dot_color_hovered.is_some() {
                base_style.dot_color = style.dot_color_hovered.unwrap();
            }
            base_style
        },
    }

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRadioStyleParam {
    BackgroundIpgColor,
    BackgroundRbgaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderWidth,
    DotIpgColor,
    DotRgbaColor,
    DotIpgColorHovered,
    DotRgbaColorHovered,
    TextIpgColor,
    TextRgbaColor,
}

pub fn radio_style_update_item(style: &mut IpgRadioStyle,
                                item: &PyObject,
                                value: &PyObject,) 
{
    let update = try_extract_radio_style_update(item);
    let name = "RadioStyle".to_string();
    match update {
        IpgRadioStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgRadioStyleParam::BackgroundRbgaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgRadioStyleParam::DotIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.dot_color = get_color(None, Some(color), 1.0, false);
        },
        IpgRadioStyleParam::DotRgbaColor => {
            style.dot_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgRadioStyleParam::DotIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.dot_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgRadioStyleParam::DotRgbaColorHovered => {
            style.dot_color_hovered = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgRadioStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgRadioStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgRadioStyleParam::BorderWidth => {
            style.border_width = Some(try_extract_f64(value, name) as f32);
        },
        IpgRadioStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgRadioStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

fn get_radio_style(style: Option<&IpgWidgets>) -> Option<IpgRadioStyle>{
    match style {
        Some(IpgWidgets::IpgRadioStyle(style)) => {
            Some(*style)
        }
        _ => None,
    }
}

pub fn try_extract_radio_style_update(update_obj: &PyObject) -> IpgRadioStyleParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgRadioStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Radio style update extraction failed"),
        }
    })
}

