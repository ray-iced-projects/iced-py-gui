//! Button widget definition
#![allow(unused)]
use iced::widget::{button, text, Button};
use iced::{alignment, Border, Color, Element, Length, Padding, Shadow, Theme, Vector};
use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;

use crate::access_callbacks;
use crate::access_user_data1;
use crate::app::Message;
use crate::graphics::bootstrap::{self, icon_to_char, icon_to_string};
use crate::graphics::colors::{IpgColor, get_color};
use crate::py_api::ipg_column;
use crate::state::IpgWidgets;
use super::styling::IpgStyleStandard;
use crate::py_api::helpers::{get_height, get_horizontal_alignment, get_padding_f64, get_radius, get_vertical_alignment, get_width, try_extract_boolean, try_extract_f32, try_extract_f64, try_extract_f64_option, try_extract_ipg_color, try_extract_ipg_horizontal_alignment, try_extract_ipg_vertical_alignment, try_extract_rgba_color, try_extract_string, try_extract_style_standard, try_extract_vec_f32, try_extract_vec_f64};

#[derive(Debug, Clone)]
pub struct IpgButton {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub label: String,
    pub width: Length,
    pub height: Length,
    pub padding: Padding,
    pub text_align_x: alignment::Horizontal,
    pub text_align_y: alignment::Vertical,
    pub text_size: f32,
    pub clip: bool,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgButtonStyleStandard>,
    pub style_arrow: Option<IpgButtonArrow>,
}

impl IpgButton {
    pub fn new(
        id: usize,
        parent_id: String,
        show: bool,
        label: String,
        width: Length,
        height: Length,
        padding: Padding,
        text_align_x: alignment::Horizontal,
        text_align_y: alignment::Vertical,
        text_size: f32,
        clip: bool,
        style_id: Option<usize>,
        style_standard: Option<IpgButtonStyleStandard>,
        style_arrow: Option<IpgButtonArrow>,
    ) -> Self {
        Self {
            id,
            parent_id,
            show,
            label,
            width,
            height,
            padding,
            text_align_x,
            text_align_y,
            text_size,
            clip,
            style_id,
            style_standard,
            style_arrow,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BTNMessage {
    OnPress,
}

pub fn construct_button<'a>(
    btn: &'a IpgButton, 
    style_widget: Option<&IpgWidgets>,
    ) -> Option<Element<'a, Message>> {
    
    if !btn.show {
        return None;
    }

    let style_opt = extract_btn_style(style_widget);

    let label = if btn.style_arrow.is_some() {
        let arrow = get_bootstrap_arrow(&btn.style_arrow);
        text(arrow).font(iced::Font::with_name("bootstrap-icons"))
    } else {
        text(btn.label.clone())
        .align_x(btn.text_align_x)
        .align_y(btn.text_align_y)
        .size(btn.text_size)
    };

    let ipg_btn: Element<BTNMessage> = 
        Button::new(label)
            .height(btn.height)
            .padding(btn.padding)
            .width(btn.width)
            .on_press(BTNMessage::OnPress)
            .clip(btn.clip)
            .style(move |theme: &Theme, status| {
                get_styling(theme, status, &style_opt, &btn.style_standard)
            })
            .into();

    Some(ipg_btn.map(move |message| Message::Button(btn.id, message)))
}

pub fn extract_btn_style(style: Option<&IpgWidgets>) -> Option<IpgButtonStyle>{
    match style {
        Some(IpgWidgets::IpgButtonStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

pub fn button_callback(id: usize, message: BTNMessage) {
    match message {
        BTNMessage::OnPress => {
            process_callback(id, "on_press".to_string());
        }
    }
}

fn process_callback(id: usize, event_name: String) {
    
    let app_cbs = access_callbacks();
    
    // Retrieve the callback
    let callback = match app_cbs.get(id, &event_name) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };
    
    drop(app_cbs);
    
    // Check user data 1
    let user_data_1_lock = access_user_data1();
    let user_data_1_opt = user_data_1_lock.get(id).map(|ud| Python::attach(|py| ud.clone_ref(py)));
    drop(user_data_1_lock);
    
    // Call the callback
    Python::attach(|py| {
        let result = if let Some(user_data) = user_data_1_opt {
            callback.call1(py, (id, user_data))
        } else {
            callback.call1(py, (id,))
        };
        
        if let Err(err) = result {
            panic!("Button callback error: {err}");
        }
    });

    // // Check user data 2
    // let user_data_2_lock = access_user_data1();
    // let user_data_2_opt = user_data_2_lock.get(id).map(|ud| Python::attach(|py| ud.clone_ref(py)));
    // drop(user_data_2_lock);

    // // Call the callback
    // Python::attach(|py| {
    //     let result = if let Some(user_data) = user_data_2_opt {
    //         callback.call1(py, (id, user_data))
    //     } else {
    //         callback.call1(py, (id,))
    //     };
        
    //     if let Err(err) = result {
    //         panic!("Button callback error: {err}");
    //     }
    // });

}

#[derive(Debug, Clone, Default)]
pub struct IpgButtonStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    background_color_disabled: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<Color>,
    pub shadow_offset_x: Option<f32>,
    pub shadow_offset_y: Option<f32>,
    pub shadow_blur_radius: Option<f32>,
    pub text_color: Option<Color>,
}

impl IpgButtonStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
        background_color_hovered: Option<Color>,
        background_color_disabled: Option<Color>,
        border_color: Option<Color>,
        border_radius: Option<Vec<f32>>,
        border_width: Option<f32>,
        shadow_color: Option<Color>,
        shadow_offset_x: Option<f32>,
        shadow_offset_y: Option<f32>,
        shadow_blur_radius: Option<f32>,
        text_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            background_color,
            background_color_hovered,
            background_color_disabled,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_x,
            shadow_offset_y,
            shadow_blur_radius,
            text_color,
        }
    }

    /// Apply user-defined style overrides to an existing iced button::Style
    pub fn apply_to(&self, style: &mut button::Style, status: button::Status) {
        if let Some(color) = self.background_color {
            if status == button::Status::Active || status == button::Status::Pressed {
                style.background = Some(color.into());
            }
        }

        if let Some(color) = self.background_color_hovered {
            if status == button::Status::Hovered {
                style.background = Some(color.into());
            }
        }

        if let Some(color) = self.background_color_disabled {
            if status == button::Status::Disabled {
                style.background = Some(color.into());
            }
        }

        if let Some(color) = self.border_color {
            style.border.color = color;
        }

        if let Some(ref radius) = self.border_radius {
            style.border.radius = get_radius(radius.clone(), "Button".to_string());
        }

        if let Some(width) = self.border_width {
            style.border.width = width;
        }

        if let Some(color) = self.shadow_color {
            style.shadow.color = color;
        }

        if let Some(offset_x) = self.shadow_offset_x {
            style.shadow.offset = Vector::new(
                offset_x,
                self.shadow_offset_y.unwrap_or(style.shadow.offset.y),
            );
        }

        if let Some(offset_y) = self.shadow_offset_y {
            style.shadow.offset = Vector::new(
                self.shadow_offset_x.unwrap_or(style.shadow.offset.x),
                offset_y,
            );
        }

        if let Some(blur_radius) = self.shadow_blur_radius {
            style.shadow.blur_radius = blur_radius;
        }

        if let Some(color) = self.text_color {
            style.text_color = color;
        }
    }
}

pub fn get_styling(theme: &Theme, status: button::Status,
                    style_opt: &Option<IpgButtonStyle>,
                    style_standard: &Option<IpgButtonStyleStandard>,
                    ) -> button::Style 
{
    let mut style = match style_standard {
        Some(_) => get_standard_style(theme, status, style_standard),
        None => button::primary(theme, status),
    };

    if let Some(user_style) = style_opt {
        user_style.apply_to(&mut style, status);
    }

    style
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonStyleStandard {
    Background,
    Danger,
    Primary,
    Secondary,
    Subtle,
    Success,
    Warning,
    Text,
}

fn get_standard_style(
    theme: &Theme, 
    status: button::Status, 
    style_standard: &Option<IpgButtonStyleStandard>
    ) -> button::Style {
    
    match style_standard {
        Some(IpgButtonStyleStandard::Background) => {
            button::background(theme, status)
        },
        Some(IpgButtonStyleStandard::Danger) => {
            button::danger(theme, status)
        },
        Some(IpgButtonStyleStandard::Primary) => {
            button::primary(theme, status)
        },
        Some(IpgButtonStyleStandard::Secondary) => {
            button::secondary(theme, status)
        },
        Some(IpgButtonStyleStandard::Subtle) => {
            button::subtle(theme, status)
        },
        Some(IpgButtonStyleStandard::Success) => {
            button::success(theme, status)
        },
        Some(IpgButtonStyleStandard::Warning) => {
            button::warning(theme, status)
        },
        Some(IpgButtonStyleStandard::Text) => {
            button::text(theme, status)
        },
        None => button::primary(theme, status),
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonParam {
    ArrowStyle,
    Height,
    HeightFill,
    Label,
    Padding,
    Clip,
    Show,
    StyleId,
    StyleStandard,
    TextAlignX,
    TextAlignY,
    TextSize,
    Width,
    WidthFill,
}

pub fn button_item_update(btn: &mut IpgButton,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_button_update(item);
    let name = "Button".to_string();
    match update {
       IpgButtonParam::ArrowStyle => {
            btn.style_arrow = Some(try_extract_button_arrow(value));
        },
        IpgButtonParam::Label => {
            btn.label = try_extract_string(value, name);
        },
        IpgButtonParam::Height => {
            let val = try_extract_f64(value, name);
            btn.height = get_height(Some(val as f32), false);
        },
        IpgButtonParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            btn.height = get_height(None, val);
        },
        IpgButtonParam::Padding => {
            btn.padding =  get_padding_f64(try_extract_vec_f64(value, name));
        },
        IpgButtonParam::Clip => {
            btn.clip = try_extract_boolean(value, name);
        }
        IpgButtonParam::Show => {
            btn.show = try_extract_boolean(value, name);
        },
        IpgButtonParam::StyleId => {
            btn.style_id = Some(try_extract_f64(value, name) as usize);
        },
        IpgButtonParam::StyleStandard => {
            btn.style_standard = Some(try_extract_button_style_standard(value, name));
        },
        IpgButtonParam::TextAlignX => {
            let h_align = try_extract_ipg_horizontal_alignment(value).unwrap();
            btn.text_align_x = get_horizontal_alignment(&h_align);
        },
        IpgButtonParam::TextAlignY => {
            let v_align = try_extract_ipg_vertical_alignment(value).unwrap();
            btn.text_align_y= get_vertical_alignment(&v_align);
        },
        IpgButtonParam::TextSize => {
            btn.text_size = try_extract_f32(value, name);
        },
        IpgButtonParam::Width => {
            let val = try_extract_f64(value, name);
            btn.width = get_width(Some(val as f32), false);
        },
        IpgButtonParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            btn.width = get_width(None, val);
        },
    }

}

fn try_extract_button_style_standard(
    value: &PyObject, 
    name: String,
    ) -> IpgButtonStyleStandard {
    
    Python::attach(|py| {

        let res = 
            value.extract::<IpgButtonStyleStandard>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for ButtonStyleStandard", name),
        }
    })
}
#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonStyleParam {
    BackgroundIpgColor,
    BackgroundRbgaColor,
    BackgroundIpgColorHovered,
    BackgroundIpgRgbaHovered,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlurRadius,
    TextIpgColor,
    TextRgbaColor,
}

pub fn button_style_update_item(style: &mut IpgButtonStyle,
                                item: &PyObject,
                                value: &PyObject,) 
{
    dbg!("button_style_update_item");
    let update = try_extract_button_style_update(item);
    let name = "ButtonStyle".to_string();
    match update {
        IpgButtonStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BackgroundRbgaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgButtonStyleParam::BackgroundIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.background_color_hovered = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BackgroundIpgRgbaHovered => {
            style.background_color_hovered = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgButtonStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgButtonStyleParam::BorderRadius => {
            dbg!(&value);
            style.border_radius = Some(try_extract_vec_f32(value, name));
        },
        IpgButtonStyleParam::BorderWidth => {
            style.border_width = Some(try_extract_f64(value, name) as f32);
        },
        IpgButtonStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.shadow_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::ShadowRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgButtonStyleParam::ShadowOffsetX => {
            style.shadow_offset_x = Some(try_extract_f64(value, name) as f32);
        },
        IpgButtonStyleParam::ShadowOffsetY => {
            style.shadow_offset_y = Some(try_extract_f64(value, name) as f32);
        },
        IpgButtonStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = Some(try_extract_f64(value, name) as f32);
        },
        IpgButtonStyleParam::TextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

pub fn try_extract_button_update(update_obj: &PyObject) -> IpgButtonParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgButtonParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button update extraction failed"),
        }
    })
}

pub fn try_extract_button_arrow(update_obj: &PyObject) -> IpgButtonArrow {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgButtonArrow>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button arrow extraction failed"),
        }
    })
}

pub fn try_extract_button_style_update(update_obj: &PyObject) -> IpgButtonStyleParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgButtonStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Button style update extraction failed"),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonArrow {
    ArrowBarLeft,
    ArrowBarRight,
    ArrowBarUp,
    ArrowClockwise,
    ArrowCounterclockwise,
    ArrowDown,
    ArrowDownCircle,
    ArrowDownCircleFill,
    ArrowDownLeft,
    ArrowDownLeftCircle,
    ArrowDownLeftCircleFill,
    ArrowDownLeftSquare,
    ArrowDownLeftSquareFill,
    ArrowDownRight,
    ArrowDownRightCircle,
    ArrowDownRightCircleFill,
    ArrowDownRightSquare,
    ArrowDownRightSquareFill,
    ArrowDownShort,
    ArrowDownSquare,
    ArrowDownSquareFill,
    ArrowDownUp,
    ArrowLeft,
    ArrowLeftCircle,
    ArrowLeftCircleFill,
    ArrowLeftRight,
    ArrowLeftShort,
    ArrowLeftSquare,
    ArrowLeftSquareFill,
    ArrowNinezerodegDown,
    ArrowNinezerodegLeft,
    ArrowNinezerodegRight,
    ArrowNinezerodegUp,
    ArrowRepeat,
    ArrowReturnLeft,
    ArrowReturnRight,
    ArrowRight,
    ArrowRightCircle,
    ArrowRightCircleFill,
    ArrowRightShort,
    ArrowRightSquare,
    ArrowRightSquareFill,
    ArrowThroughHeart,
    ArrowThroughHeartFill,
    ArrowUp,
    ArrowUpCircle,
    ArrowUpCircleFill,
    ArrowUpLeft,
    ArrowUpLeftCircle,
    ArrowUpLeftCircleFill,
    ArrowUpLeftSquare,
    ArrowUpLeftSquareFill,
    ArrowUpRight,
    ArrowUpRightCircle,
    ArrowUpRightCircleFill,
    ArrowUpRightSquare,
    ArrowUpRightSquareFill,
    ArrowUpShort,
    ArrowUpSquare,
    ArrowUpSquareFill,
    Arrows,
    ArrowsAngleContract,
    ArrowsAngleExpand,
    ArrowsCollapse,
    ArrowsCollapseVertical,
    ArrowsExpand,
    ArrowsExpandVertical,
    ArrowsFullscreen,
    ArrowsMove,
    ArrowsVertical,
}


pub fn get_bootstrap_arrow(arrow: &Option<IpgButtonArrow>) -> String {
    match arrow {
        &None => unreachable!(),
        Some(IpgButtonArrow::ArrowBarLeft) => icon_to_string(bootstrap::Bootstrap::ArrowBarLeft),
        Some(IpgButtonArrow::ArrowBarRight) => icon_to_string(bootstrap::Bootstrap::ArrowBarRight),
        Some(IpgButtonArrow::ArrowBarUp) => icon_to_string(bootstrap::Bootstrap::ArrowBarUp),
        Some(IpgButtonArrow::ArrowClockwise) => icon_to_string(bootstrap::Bootstrap::ArrowClockwise),
        Some(IpgButtonArrow::ArrowCounterclockwise) => icon_to_string(bootstrap::Bootstrap::ArrowCounterclockwise),
        Some(IpgButtonArrow::ArrowDown) => icon_to_string(bootstrap::Bootstrap::ArrowDown),
        Some(IpgButtonArrow::ArrowDownCircle) => icon_to_string(bootstrap::Bootstrap::ArrowDownCircle),
        Some(IpgButtonArrow::ArrowDownCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownCircleFill),
        Some(IpgButtonArrow::ArrowDownLeft) => icon_to_string(bootstrap::Bootstrap::ArrowDownLeft),
        Some(IpgButtonArrow::ArrowDownLeftCircle) => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircle),
        Some(IpgButtonArrow::ArrowDownLeftCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircleFill),
        Some(IpgButtonArrow::ArrowDownLeftSquare) => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquare),
        Some(IpgButtonArrow::ArrowDownLeftSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquareFill),
        Some(IpgButtonArrow::ArrowDownRight) => icon_to_string(bootstrap::Bootstrap::ArrowDownRight),
        Some(IpgButtonArrow::ArrowDownRightCircle) => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircle),
        Some(IpgButtonArrow::ArrowDownRightCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircleFill),
        Some(IpgButtonArrow::ArrowDownRightSquare) => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquare),
        Some(IpgButtonArrow::ArrowDownRightSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquareFill),
        Some(IpgButtonArrow::ArrowDownShort) => icon_to_string(bootstrap::Bootstrap::ArrowDownShort),
        Some(IpgButtonArrow::ArrowDownSquare) => icon_to_string(bootstrap::Bootstrap::ArrowDownSquare),
        Some(IpgButtonArrow::ArrowDownSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowDownSquareFill),
        Some(IpgButtonArrow::ArrowDownUp) => icon_to_string(bootstrap::Bootstrap::ArrowDownUp),
        Some(IpgButtonArrow::ArrowLeft) => icon_to_string(bootstrap::Bootstrap::ArrowLeft),
        Some(IpgButtonArrow::ArrowLeftCircle) => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircle),
        Some(IpgButtonArrow::ArrowLeftCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircleFill),
        Some(IpgButtonArrow::ArrowLeftRight) => icon_to_string(bootstrap::Bootstrap::ArrowLeftRight),
        Some(IpgButtonArrow::ArrowLeftShort) => icon_to_string(bootstrap::Bootstrap::ArrowLeftShort),
        Some(IpgButtonArrow::ArrowLeftSquare) => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquare),
        Some(IpgButtonArrow::ArrowLeftSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquareFill),
        Some(IpgButtonArrow::ArrowNinezerodegDown) => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegDown),
        Some(IpgButtonArrow::ArrowNinezerodegLeft) => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegLeft),
        Some(IpgButtonArrow::ArrowNinezerodegRight) => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegRight),
        Some(IpgButtonArrow::ArrowNinezerodegUp) => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegUp),
        Some(IpgButtonArrow::ArrowRepeat) => icon_to_string(bootstrap::Bootstrap::ArrowRepeat),
        Some(IpgButtonArrow::ArrowReturnLeft) => icon_to_string(bootstrap::Bootstrap::ArrowReturnLeft),
        Some(IpgButtonArrow::ArrowReturnRight) => icon_to_string(bootstrap::Bootstrap::ArrowReturnRight),
        Some(IpgButtonArrow::ArrowRight) => icon_to_string(bootstrap::Bootstrap::ArrowRight),
        Some(IpgButtonArrow::ArrowRightCircle) => icon_to_string(bootstrap::Bootstrap::ArrowRightCircle),
        Some(IpgButtonArrow::ArrowRightCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowRightCircleFill),
        Some(IpgButtonArrow::ArrowRightShort) => icon_to_string(bootstrap::Bootstrap::ArrowRightShort),
        Some(IpgButtonArrow::ArrowRightSquare) => icon_to_string(bootstrap::Bootstrap::ArrowRightSquare),
        Some(IpgButtonArrow::ArrowRightSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowRightSquareFill),
        Some(IpgButtonArrow::ArrowThroughHeart) => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeart),
        Some(IpgButtonArrow::ArrowThroughHeartFill) => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeartFill),
        Some(IpgButtonArrow::ArrowUp) => icon_to_string(bootstrap::Bootstrap::ArrowUp),
        Some(IpgButtonArrow::ArrowUpCircle) => icon_to_string(bootstrap::Bootstrap::ArrowUpCircle),
        Some(IpgButtonArrow::ArrowUpCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpCircleFill),
        Some(IpgButtonArrow::ArrowUpLeft) => icon_to_string(bootstrap::Bootstrap::ArrowUpLeft),
        Some(IpgButtonArrow::ArrowUpLeftCircle) => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircle),
        Some(IpgButtonArrow::ArrowUpLeftCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircleFill),
        Some(IpgButtonArrow::ArrowUpLeftSquare) => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquare),
        Some(IpgButtonArrow::ArrowUpLeftSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquareFill),
        Some(IpgButtonArrow::ArrowUpRight) => icon_to_string(bootstrap::Bootstrap::ArrowUpRight),
        Some(IpgButtonArrow::ArrowUpRightCircle) => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircle),
        Some(IpgButtonArrow::ArrowUpRightCircleFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircleFill),
        Some(IpgButtonArrow::ArrowUpRightSquare) => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquare),
        Some(IpgButtonArrow::ArrowUpRightSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquareFill),
        Some(IpgButtonArrow::ArrowUpShort) => icon_to_string(bootstrap::Bootstrap::ArrowUpShort),
        Some(IpgButtonArrow::ArrowUpSquare) => icon_to_string(bootstrap::Bootstrap::ArrowUpSquare),
        Some(IpgButtonArrow::ArrowUpSquareFill) => icon_to_string(bootstrap::Bootstrap::ArrowUpSquareFill),
        Some(IpgButtonArrow::Arrows) => icon_to_string(bootstrap::Bootstrap::Arrows),
        Some(IpgButtonArrow::ArrowsAngleContract) => icon_to_string(bootstrap::Bootstrap::ArrowsAngleContract),
        Some(IpgButtonArrow::ArrowsAngleExpand) => icon_to_string(bootstrap::Bootstrap::ArrowsAngleExpand),
        Some(IpgButtonArrow::ArrowsCollapse) => icon_to_string(bootstrap::Bootstrap::ArrowsCollapse),
        Some(IpgButtonArrow::ArrowsCollapseVertical) => icon_to_string(bootstrap::Bootstrap::ArrowsCollapseVertical),
        Some(IpgButtonArrow::ArrowsExpand) => icon_to_string(bootstrap::Bootstrap::ArrowsExpand),
        Some(IpgButtonArrow::ArrowsExpandVertical) => icon_to_string(bootstrap::Bootstrap::ArrowsExpandVertical),
        Some(IpgButtonArrow::ArrowsFullscreen) => icon_to_string(bootstrap::Bootstrap::ArrowsFullscreen),
        Some(IpgButtonArrow::ArrowsMove) => icon_to_string(bootstrap::Bootstrap::ArrowsMove),
        Some(IpgButtonArrow::ArrowsVertical) => icon_to_string(bootstrap::Bootstrap::ArrowsVertical),
    }
}

pub fn get_bootstrap_arrow_char(arrow: &IpgButtonArrow) -> char {
    match arrow {
        IpgButtonArrow::ArrowBarLeft => icon_to_char(bootstrap::Bootstrap::ArrowBarLeft),
        IpgButtonArrow::ArrowBarRight => icon_to_char(bootstrap::Bootstrap::ArrowBarRight),
        IpgButtonArrow::ArrowBarUp => icon_to_char(bootstrap::Bootstrap::ArrowBarUp),
        IpgButtonArrow::ArrowClockwise => icon_to_char(bootstrap::Bootstrap::ArrowClockwise),
        IpgButtonArrow::ArrowCounterclockwise => icon_to_char(bootstrap::Bootstrap::ArrowCounterclockwise),
        IpgButtonArrow::ArrowDown => icon_to_char(bootstrap::Bootstrap::ArrowDown),
        IpgButtonArrow::ArrowDownCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownCircle),
        IpgButtonArrow::ArrowDownCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownCircleFill),
        IpgButtonArrow::ArrowDownLeft => icon_to_char(bootstrap::Bootstrap::ArrowDownLeft),
        IpgButtonArrow::ArrowDownLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftCircle),
        IpgButtonArrow::ArrowDownLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftCircleFill),
        IpgButtonArrow::ArrowDownLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftSquare),
        IpgButtonArrow::ArrowDownLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftSquareFill),
        IpgButtonArrow::ArrowDownRight => icon_to_char(bootstrap::Bootstrap::ArrowDownRight),
        IpgButtonArrow::ArrowDownRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownRightCircle),
        IpgButtonArrow::ArrowDownRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownRightCircleFill),
        IpgButtonArrow::ArrowDownRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownRightSquare),
        IpgButtonArrow::ArrowDownRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownRightSquareFill),
        IpgButtonArrow::ArrowDownShort => icon_to_char(bootstrap::Bootstrap::ArrowDownShort),
        IpgButtonArrow::ArrowDownSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownSquare),
        IpgButtonArrow::ArrowDownSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownSquareFill),
        IpgButtonArrow::ArrowDownUp => icon_to_char(bootstrap::Bootstrap::ArrowDownUp),
        IpgButtonArrow::ArrowLeft => icon_to_char(bootstrap::Bootstrap::ArrowLeft),
        IpgButtonArrow::ArrowLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowLeftCircle),
        IpgButtonArrow::ArrowLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowLeftCircleFill),
        IpgButtonArrow::ArrowLeftRight => icon_to_char(bootstrap::Bootstrap::ArrowLeftRight),
        IpgButtonArrow::ArrowLeftShort => icon_to_char(bootstrap::Bootstrap::ArrowLeftShort),
        IpgButtonArrow::ArrowLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowLeftSquare),
        IpgButtonArrow::ArrowLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowLeftSquareFill),
        IpgButtonArrow::ArrowNinezerodegDown => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegDown),
        IpgButtonArrow::ArrowNinezerodegLeft => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegLeft),
        IpgButtonArrow::ArrowNinezerodegRight => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegRight),
        IpgButtonArrow::ArrowNinezerodegUp => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegUp),
        IpgButtonArrow::ArrowRepeat => icon_to_char(bootstrap::Bootstrap::ArrowRepeat),
        IpgButtonArrow::ArrowReturnLeft => icon_to_char(bootstrap::Bootstrap::ArrowReturnLeft),
        IpgButtonArrow::ArrowReturnRight => icon_to_char(bootstrap::Bootstrap::ArrowReturnRight),
        IpgButtonArrow::ArrowRight => icon_to_char(bootstrap::Bootstrap::ArrowRight),
        IpgButtonArrow::ArrowRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowRightCircle),
        IpgButtonArrow::ArrowRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowRightCircleFill),
        IpgButtonArrow::ArrowRightShort => icon_to_char(bootstrap::Bootstrap::ArrowRightShort),
        IpgButtonArrow::ArrowRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowRightSquare),
        IpgButtonArrow::ArrowRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowRightSquareFill),
        IpgButtonArrow::ArrowThroughHeart => icon_to_char(bootstrap::Bootstrap::ArrowThroughHeart),
        IpgButtonArrow::ArrowThroughHeartFill => icon_to_char(bootstrap::Bootstrap::ArrowThroughHeartFill),
        IpgButtonArrow::ArrowUp => icon_to_char(bootstrap::Bootstrap::ArrowUp),
        IpgButtonArrow::ArrowUpCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpCircle),
        IpgButtonArrow::ArrowUpCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpCircleFill),
        IpgButtonArrow::ArrowUpLeft => icon_to_char(bootstrap::Bootstrap::ArrowUpLeft),
        IpgButtonArrow::ArrowUpLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftCircle),
        IpgButtonArrow::ArrowUpLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftCircleFill),
        IpgButtonArrow::ArrowUpLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftSquare),
        IpgButtonArrow::ArrowUpLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftSquareFill),
        IpgButtonArrow::ArrowUpRight => icon_to_char(bootstrap::Bootstrap::ArrowUpRight),
        IpgButtonArrow::ArrowUpRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpRightCircle),
        IpgButtonArrow::ArrowUpRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpRightCircleFill),
        IpgButtonArrow::ArrowUpRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpRightSquare),
        IpgButtonArrow::ArrowUpRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpRightSquareFill),
        IpgButtonArrow::ArrowUpShort => icon_to_char(bootstrap::Bootstrap::ArrowUpShort),
        IpgButtonArrow::ArrowUpSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpSquare),
        IpgButtonArrow::ArrowUpSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpSquareFill),
        IpgButtonArrow::Arrows => icon_to_char(bootstrap::Bootstrap::Arrows),
        IpgButtonArrow::ArrowsAngleContract => icon_to_char(bootstrap::Bootstrap::ArrowsAngleContract),
        IpgButtonArrow::ArrowsAngleExpand => icon_to_char(bootstrap::Bootstrap::ArrowsAngleExpand),
        IpgButtonArrow::ArrowsCollapse => icon_to_char(bootstrap::Bootstrap::ArrowsCollapse),
        IpgButtonArrow::ArrowsCollapseVertical => icon_to_char(bootstrap::Bootstrap::ArrowsCollapseVertical),
        IpgButtonArrow::ArrowsExpand => icon_to_char(bootstrap::Bootstrap::ArrowsExpand),
        IpgButtonArrow::ArrowsExpandVertical => icon_to_char(bootstrap::Bootstrap::ArrowsExpandVertical),
        IpgButtonArrow::ArrowsFullscreen => icon_to_char(bootstrap::Bootstrap::ArrowsFullscreen),
        IpgButtonArrow::ArrowsMove => icon_to_char(bootstrap::Bootstrap::ArrowsMove),
        IpgButtonArrow::ArrowsVertical => icon_to_char(bootstrap::Bootstrap::ArrowsVertical),
    }
}
