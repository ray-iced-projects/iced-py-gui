//! ipg_text_input
use iced::widget::text::LineHeight;
use iced::widget::text_input;
use iced::widget::text_input::{Style, Status};
use iced::{Color, Element, Length, Padding, Pixels, Theme};
use iced::widget::TextInput;

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;

use ipg_helpers::{get_padding_f64, get_radius, get_width, try_extract_vec_f32, 
    try_extract_boolean, try_extract_f64, try_extract_string, try_extract_u16, 
    try_extract_vec_f64};
use ipg_styling::{colors::get_color, try_extract_ipg_color, try_extract_rgba_color};
use ipg_types::{Message, TIMessage};
use super::ipg_enums::IpgWidgets;

#[derive(Debug, Clone)]
pub struct IpgTextInput {
    pub id: usize,
    pub parent_id: String,
    pub placeholder: String,
    pub value: String,
    pub is_secure: bool,
    // font: Option<Font>,
    pub width: Length,
    pub padding: Padding,
    pub size: f32,
    pub line_height: LineHeight,
    // icon: Option<Message>,
    pub style_id: Option<usize>,
    pub show: bool,
}

impl IpgTextInput {
    pub fn new( 
        id: usize,
        parent_id: String,
        placeholder: String,
        is_secure: bool,
        // font: Option<Font>,
        width: Length,
        padding: Padding,
        size: f32,
        line_height: LineHeight,
        // icon: Option<Message>,
        style_id: Option<usize>,
        show: bool,
        ) -> Self {
        Self {
            id,
            parent_id,
            placeholder,
            value: "".to_string(),
            is_secure,
            // font,
            width,
            padding,
            size,
            line_height,
            // icon,
            style_id,
            show,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgTextInputStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_color_hovered: Option<Color>,
    pub border_color_focused: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
    // pub icon_color: Option<Color>,
    pub placeholder_color: Option<Color>,
    pub value_color: Option<Color>,
    pub selection_color: Option<Color>,
}

impl IpgTextInputStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        border_color: Option<Color>,
        border_color_hovered: Option<Color>,
        border_color_focused: Option<Color>,
        border_width: Option<f32>,
        border_radius: Option<Vec<f32>>,
        // icon_color: Option<Color>,
        placeholder_color: Option<Color>,
        value_color: Option<Color>,
        selection_color: Option<Color>,
    )  -> Self {
        Self {
            id,
            background_color,
            border_color,
            border_color_hovered,
            border_color_focused,
            border_width,
            border_radius,
            // icon_color,
            placeholder_color,
            value_color,
            selection_color,
        }
    }
}

pub fn construct_text_input<'a>(input: &'a IpgTextInput, 
                            style_opt: Option<&IpgWidgets>) 
                            -> Option<Element<'a, Message>> {
    
    if !input.show {
        return None
    }

    let style = get_text_input_style(style_opt);
    
    let txt: Element<TIMessage> =  TextInput::new(input.placeholder.as_str(), 
                                                input.value.as_str()
                                            )
                                            .on_input(TIMessage::OnInput)
                                            .on_submit(TIMessage::OnSubmit(input.value.clone()))
                                            .on_paste(TIMessage::OnPaste)
                                            .secure(input.is_secure)
                                            .width(input.width)
                                            .padding(input.padding)
                                            .size(input.size)
                                            .line_height(input.line_height)
                                            // .icon(text_input::Icon {
                                            //     font: BOOTSTRAP_FONT,
                                            //     code_point: required::icon_to_char(required::Icon::CaretRightFill),
                                            //     size: Some(Pixels(60.0)),
                                            //     spacing: 5.0,
                                            //     side: text_input::Side::Right,
                                            // })
                                            .style(move|theme, status|
                                                get_styling(theme, status, 
                                                    style.clone(),
                                                ))
                                            .into();

    Some(txt.map(move |message| Message::TextInput(input.id, message)))

}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTextInputParam {
    Placeholder,
    Value,
    IsSecure,
    Width,
    Padding,
    Size,
    LineHeightPixels,
    LineHeightRelative,
    StyleId,
}

pub fn text_input_item_update(ti: &mut IpgTextInput,
                                item: &PyObject,
                                value: &PyObject,
                                )
{
    let update = try_extract_text_input_update(item);
    let name = "TextInput".to_string();
    match update {
        IpgTextInputParam::Placeholder => {
            ti.placeholder = try_extract_string(value, name);
        },
        IpgTextInputParam::Value => {
            ti.value = try_extract_string(value, name);
        },
        IpgTextInputParam::IsSecure => {
            ti.is_secure = try_extract_boolean(value, name);
        },
        IpgTextInputParam::Width => {
            let val = try_extract_f64(value, name);
            ti.width = get_width(Some(val as f32), false);
        },
        IpgTextInputParam::Padding => {
            let val = try_extract_vec_f64(value, name);
            ti.padding =  get_padding_f64(val);
        },
        IpgTextInputParam::Size => {
            ti.size = try_extract_f64(value, name) as f32;
        },
        IpgTextInputParam::LineHeightPixels => {
            let val = try_extract_u16(value, name);
            ti.line_height = LineHeight::Absolute(Pixels(val.into()));
        },
        IpgTextInputParam::LineHeightRelative => {
            let val = try_extract_f64(value, name) as f32;
            ti.line_height = LineHeight::Relative(val);
        },
        IpgTextInputParam::StyleId => {
            ti.style_id = Some(try_extract_f64(value, name) as usize);
        },
    }
}


fn try_extract_text_input_update(update_obj: &PyObject) -> IpgTextInputParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgTextInputParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("TextInput update extraction failed"),
        }
    })
}

fn get_styling(theme: &Theme, 
                status: Status, 
                style_opt: Option<IpgTextInputStyle>
                ) -> Style {

    if style_opt.is_none() {
        return text_input::default(theme, status)
    }     

    let style = style_opt.unwrap();

    let mut style_base = text_input::default(theme, Status::Active);

    if style.background_color.is_some() {
        style_base.background = style.background_color.unwrap().into();
    }

    if style.border_width.is_some() {
        style_base.border.width = style.border_width.unwrap();
    }

    if style.border_radius.is_some() {
        style_base.border.radius = get_radius(style.border_radius.clone().unwrap(),
                                        "TextInput".to_string());
    }

    if style.border_color.is_some() {
        style_base.border.color = style.border_color.unwrap();
    }

    // if style.icon_color.is_some() {
    //     style_base.icon = style.icon_color.unwrap();
    // }

    if style.placeholder_color.is_some() {
        style_base.placeholder = style.placeholder_color.unwrap();
    }

    if style.value_color.is_some() {
        style_base.value = style.value_color.unwrap();
    }

    if style.selection_color.is_some() {
        style_base.selection = style.selection_color.unwrap();
    }

    let palette = theme.extended_palette();

    match status {
        Status::Active =>style_base,
        Status::Hovered => {
            if style.border_color_hovered.is_some() {
                style_base.border.color = style.border_color_hovered.unwrap();
            } else {
                style_base.border.color = palette.background.base.text;
            }
            style_base
        },
        Status::Focused => {
            if style.border_color_focused.is_some() {
                style_base.border.color = style.border_color_focused.unwrap();
            } else {
                style_base.border.color = palette.primary.strong.color;
            }
            style_base
        },
        Status::Disabled => {
            style_base.value = style_base.placeholder;
            style_base.background = palette.background.weak.color.into();
            
            style_base
        }
    }
}

fn get_text_input_style(style: Option<&IpgWidgets>) -> Option<IpgTextInputStyle>{
    match style {
        Some(IpgWidgets::IpgTextInputStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTextInputStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderIpgColorHovered,
    BorderRgbaColorHovered,
    BorderIpgColorFocused,
    BorderRgbaColorFocused,
    BorderWidth,
    BorderRadius,
    // icon_color,
    PlaceholderIpgColor,
    PlaceholderRgbaColor,
    ValueIpgColor,
    ValueRgbaColor,
    SelectionIpgColor,
    SelectionRgbaColor,
}

pub fn text_input_style_update_item(style: &mut IpgTextInputStyle,
                                    item: &PyObject,
                                    value: &PyObject,) 
{
    let update = try_extract_text_input_style_update(item);
    let name = "TextInputStyle".to_string();
    match update {
        IpgTextInputStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTextInputStyleParam::BackgroundRgbaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTextInputStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTextInputStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTextInputStyleParam::BorderIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.border_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgTextInputStyleParam::BorderRgbaColorHovered => {
            style.border_color_hovered = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTextInputStyleParam::BorderIpgColorFocused => {
            let color = try_extract_ipg_color(value, name);
            style.border_color_focused = get_color(None, Some(color), 1.0, false);
        },
        IpgTextInputStyleParam::BorderRgbaColorFocused => {
            style.border_color_focused = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTextInputStyleParam::BorderWidth => {
            style.border_width = Some(try_extract_f64(value, name) as f32);
        },
        IpgTextInputStyleParam::BorderRadius => {
            style.border_radius = Some(try_extract_vec_f32(value, name))
        },
        IpgTextInputStyleParam::PlaceholderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.placeholder_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTextInputStyleParam::PlaceholderRgbaColor => {
            style.placeholder_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTextInputStyleParam::ValueIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.value_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTextInputStyleParam::ValueRgbaColor => {
            style.value_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgTextInputStyleParam::SelectionIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.selection_color = get_color(None, Some(color), 1.0, false);
        },
        IpgTextInputStyleParam::SelectionRgbaColor => {
            style.selection_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }

}

fn try_extract_text_input_style_update(update_obj: &PyObject) -> IpgTextInputStyleParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgTextInputStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Text Input style update extraction failed"),
        }
    })
}
