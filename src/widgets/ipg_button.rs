//! Button widget definition
#![allow(unused)]
use iced::widget::{button, text, Button};
use iced::{alignment, Border, Color, Element, Length, 
    Padding, Shadow, Theme, Vector};
use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;

use crate::access_callbacks;
use crate::access_user_data1;
use crate::app::Message;
use crate::graphics::bootstrap::{self, icon_to_char, icon_to_string};
use crate::graphics::colors::IpgColor;
use crate::py_api::column;
use crate::state::IpgWidgets;
use crate::widgets::enums::{IpgHorizontalAlignment, 
    IpgVerticalAlignment};
use super::styling::IpgStyleStandard;
use crate::py_api::helpers::{get_height, get_padding_f32, 
    get_padding_f64, get_radius, get_width, try_extract_boolean, 
    try_extract_f32, try_extract_f64, try_extract_f64_option, 
    try_extract_ipg_color, try_extract_ipg_horizontal_alignment, 
    try_extract_ipg_vertical_alignment, 
    try_extract_string, try_extract_style_standard, 
    try_extract_vec_f32, try_extract_vec_f64};

#[derive(Debug, Clone)]
pub struct IpgButton {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub label: Option<String>,
    pub width: Length,
    pub height: Length,
    pub padding: Option<Vec<f32>>,
    pub text_align_x: Option<IpgHorizontalAlignment>,
    pub text_align_y: Option<IpgVerticalAlignment>,
    pub text_size: Option<f32>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgButtonStyleStandard>,
    pub style_arrow: Option<IpgButtonArrow>,
}

#[derive(Debug, Clone)]
pub enum BtnMessage {
    OnPress,
}

pub fn construct_button<'a>(
    ipg_btn: &'a IpgButton, 
    style_widget: Option<&IpgWidgets>,
    ) -> Option<Element<'a, Message>> {
    
    if !ipg_btn.show {
        return None;
    }

    let style_opt = extract_btn_style(style_widget);

    let txt = 
        if let Some(sa) = ipg_btn.style_arrow.clone() {
            let arrow = get_bootstrap_arrow(&sa);
            text(arrow).font(iced::Font::with_name("bootstrap-icons"))
        } else {
            let label = if let Some(lb) = &ipg_btn.label {
                lb.clone()
            } else {
                String::new()
            };
            text(label.clone())
        };

    let txt = 
        if let Some(align) = &ipg_btn.text_align_x {
            txt.align_x(align.to_iced())
        } else {txt};

    let txt = 
        if let Some(align) = &ipg_btn.text_align_y {
            txt.align_y(align.to_iced())
        } else {txt};
    
    let txt = if let Some(size) = ipg_btn.text_size {
        txt.size(size)
    } else {txt};
    
    let btn=
        Button::new(txt)
            .on_press(BtnMessage::OnPress)
            .width(ipg_btn.width)
            .height(ipg_btn.height)
            .style(move |theme: &Theme, status| {
                get_styling(theme, status, &style_opt, &ipg_btn.style_standard)
        });

    let btn = 
        if let Some(pd) = &ipg_btn.padding {
            btn.padding(get_padding_f32(pd))
        } else { btn };

    let btn: Element<'_, BtnMessage> = 
        if let Some(cp) = ipg_btn.clip {
            btn.clip(cp).into()
        } else { btn.into() };

    Some(btn.map(move |message| Message::Button(ipg_btn.id, message)))

}

pub fn extract_btn_style(style: Option<&IpgWidgets>) -> Option<IpgButtonStyle>{
    match style {
        Some(IpgWidgets::IpgButtonStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

pub fn button_callback(id: usize, message: BtnMessage) {
    match message {
        BtnMessage::OnPress => {
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
        return
    });

    // Check user data 2
    let user_data_2_lock = access_user_data1();
    let user_data_2_opt = user_data_2_lock.get(id).map(|ud| Python::attach(|py| ud.clone_ref(py)));
    drop(user_data_2_lock);

    // Call the callback
    Python::attach(|py| {
        let result = if let Some(user_data) = user_data_2_opt {
            callback.call1(py, (id, user_data))
        } else {
            callback.call1(py, (id,))
        };
        
        if let Err(err) = result {
            panic!("Button callback error: {err}");
        }
    });

}

#[derive(Debug, Clone, Default)]
pub struct IpgButtonStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    pub background_color_disabled: Option<Color>,
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

pub fn button_param_update(
        btn: &mut IpgButton,
        item: &PyObject,
        value: &PyObject,
    )
{
    let update = try_extract_button_param_update(item);
    let name = "Button".to_string();

    match update {
       IpgButtonParam::ArrowStyle => {
            btn.style_arrow = Some(try_extract_button_arrow(value));
        },
        IpgButtonParam::Label => {
            btn.label = Some(try_extract_string(value, name));
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
            btn.padding =  Some(try_extract_vec_f32(value, name));
        },
        IpgButtonParam::Clip => {
            btn.clip = Some(try_extract_boolean(value, name));
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
            btn.text_align_x = try_extract_ipg_horizontal_alignment(value);
        },
        IpgButtonParam::TextAlignY => {
            btn.text_align_y= try_extract_ipg_vertical_alignment(value);
        },
        IpgButtonParam::TextSize => {
            btn.text_size = Some(try_extract_f32(value, name));
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

pub fn button_style_update(style: &mut IpgButtonStyle,
                                item: &PyObject,
                                value: &PyObject,) 
{
    let update = try_extract_button_style_update(item);
    let name = "ButtonStyle".to_string();
    match update {
        IpgButtonStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BackgroundRbgaColor => {
            style.background_color = Some(Color::from(IpgColor::extract_rgba_color(value, name)));
        },
        IpgButtonStyleParam::BackgroundIpgColorHovered => {
            let color = try_extract_ipg_color(value, name);
            style.background_color_hovered = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BackgroundIpgRgbaHovered => {
            style.background_color_hovered = Some(Color::from(IpgColor::extract_rgba_color(value, name)));
        },
        IpgButtonStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(IpgColor::extract_rgba_color(value, name)));
        },
        IpgButtonStyleParam::BorderRadius => {
            style.border_radius = Some(try_extract_vec_f32(value, name));
        },
        IpgButtonStyleParam::BorderWidth => {
            style.border_width = Some(try_extract_f64(value, name) as f32);
        },
        IpgButtonStyleParam::ShadowIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.shadow_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::ShadowRgbaColor => {
            style.border_color = Some(Color::from(IpgColor::extract_rgba_color(value, name)));
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
            style.text_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgButtonStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(IpgColor::extract_rgba_color(value, name)));
        },
    }
}

pub fn try_extract_button_param_update(update_obj: &PyObject) -> IpgButtonParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgButtonParam>(py);
        match res {
            Ok(update) => update,
            Err(err) => panic!("Button update extraction failed {}", err),
        }
    })
}

pub fn try_extract_button_arrow(update_obj: &PyObject) -> IpgButtonArrow {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgButtonArrow>(py);
        match res {
            Ok(update) => update,
            Err(err) => panic!("Button arrow extraction failed {}", err),
        }
    })
}

pub fn try_extract_button_style_update(update_obj: &PyObject) -> IpgButtonStyleParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgButtonStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(err) => panic!(
                "Button style update extraction failed, 
                 are you using the style_id? {}", err),
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


pub fn get_bootstrap_arrow(arrow: &IpgButtonArrow) -> String {
    match arrow {
        IpgButtonArrow::ArrowBarLeft => icon_to_string(bootstrap::Bootstrap::ArrowBarLeft),
        IpgButtonArrow::ArrowBarRight => icon_to_string(bootstrap::Bootstrap::ArrowBarRight),
        IpgButtonArrow::ArrowBarUp => icon_to_string(bootstrap::Bootstrap::ArrowBarUp),
        IpgButtonArrow::ArrowClockwise => icon_to_string(bootstrap::Bootstrap::ArrowClockwise),
        IpgButtonArrow::ArrowCounterclockwise => icon_to_string(bootstrap::Bootstrap::ArrowCounterclockwise),
        IpgButtonArrow::ArrowDown => icon_to_string(bootstrap::Bootstrap::ArrowDown),
        IpgButtonArrow::ArrowDownCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownCircle),
        IpgButtonArrow::ArrowDownCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownCircleFill),
        IpgButtonArrow::ArrowDownLeft => icon_to_string(bootstrap::Bootstrap::ArrowDownLeft),
        IpgButtonArrow::ArrowDownLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircle),
        IpgButtonArrow::ArrowDownLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircleFill),
        IpgButtonArrow::ArrowDownLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquare),
        IpgButtonArrow::ArrowDownLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquareFill),
        IpgButtonArrow::ArrowDownRight => icon_to_string(bootstrap::Bootstrap::ArrowDownRight),
        IpgButtonArrow::ArrowDownRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircle),
        IpgButtonArrow::ArrowDownRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircleFill),
        IpgButtonArrow::ArrowDownRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquare),
        IpgButtonArrow::ArrowDownRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquareFill),
        IpgButtonArrow::ArrowDownShort => icon_to_string(bootstrap::Bootstrap::ArrowDownShort),
        IpgButtonArrow::ArrowDownSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownSquare),
        IpgButtonArrow::ArrowDownSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownSquareFill),
        IpgButtonArrow::ArrowDownUp => icon_to_string(bootstrap::Bootstrap::ArrowDownUp),
        IpgButtonArrow::ArrowLeft => icon_to_string(bootstrap::Bootstrap::ArrowLeft),
        IpgButtonArrow::ArrowLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircle),
        IpgButtonArrow::ArrowLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircleFill),
        IpgButtonArrow::ArrowLeftRight => icon_to_string(bootstrap::Bootstrap::ArrowLeftRight),
        IpgButtonArrow::ArrowLeftShort => icon_to_string(bootstrap::Bootstrap::ArrowLeftShort),
        IpgButtonArrow::ArrowLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquare),
        IpgButtonArrow::ArrowLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquareFill),
        IpgButtonArrow::ArrowNinezerodegDown => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegDown),
        IpgButtonArrow::ArrowNinezerodegLeft => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegLeft),
        IpgButtonArrow::ArrowNinezerodegRight => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegRight),
        IpgButtonArrow::ArrowNinezerodegUp => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegUp),
        IpgButtonArrow::ArrowRepeat => icon_to_string(bootstrap::Bootstrap::ArrowRepeat),
        IpgButtonArrow::ArrowReturnLeft => icon_to_string(bootstrap::Bootstrap::ArrowReturnLeft),
        IpgButtonArrow::ArrowReturnRight => icon_to_string(bootstrap::Bootstrap::ArrowReturnRight),
        IpgButtonArrow::ArrowRight => icon_to_string(bootstrap::Bootstrap::ArrowRight),
        IpgButtonArrow::ArrowRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowRightCircle),
        IpgButtonArrow::ArrowRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowRightCircleFill),
        IpgButtonArrow::ArrowRightShort => icon_to_string(bootstrap::Bootstrap::ArrowRightShort),
        IpgButtonArrow::ArrowRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowRightSquare),
        IpgButtonArrow::ArrowRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowRightSquareFill),
        IpgButtonArrow::ArrowThroughHeart => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeart),
        IpgButtonArrow::ArrowThroughHeartFill => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeartFill),
        IpgButtonArrow::ArrowUp => icon_to_string(bootstrap::Bootstrap::ArrowUp),
        IpgButtonArrow::ArrowUpCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpCircle),
        IpgButtonArrow::ArrowUpCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpCircleFill),
        IpgButtonArrow::ArrowUpLeft => icon_to_string(bootstrap::Bootstrap::ArrowUpLeft),
        IpgButtonArrow::ArrowUpLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircle),
        IpgButtonArrow::ArrowUpLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircleFill),
        IpgButtonArrow::ArrowUpLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquare),
        IpgButtonArrow::ArrowUpLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquareFill),
        IpgButtonArrow::ArrowUpRight => icon_to_string(bootstrap::Bootstrap::ArrowUpRight),
        IpgButtonArrow::ArrowUpRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircle),
        IpgButtonArrow::ArrowUpRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircleFill),
        IpgButtonArrow::ArrowUpRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquare),
        IpgButtonArrow::ArrowUpRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquareFill),
        IpgButtonArrow::ArrowUpShort => icon_to_string(bootstrap::Bootstrap::ArrowUpShort),
        IpgButtonArrow::ArrowUpSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpSquare),
        IpgButtonArrow::ArrowUpSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpSquareFill),
        IpgButtonArrow::Arrows => icon_to_string(bootstrap::Bootstrap::Arrows),
        IpgButtonArrow::ArrowsAngleContract => icon_to_string(bootstrap::Bootstrap::ArrowsAngleContract),
        IpgButtonArrow::ArrowsAngleExpand => icon_to_string(bootstrap::Bootstrap::ArrowsAngleExpand),
        IpgButtonArrow::ArrowsCollapse => icon_to_string(bootstrap::Bootstrap::ArrowsCollapse),
        IpgButtonArrow::ArrowsCollapseVertical => icon_to_string(bootstrap::Bootstrap::ArrowsCollapseVertical),
        IpgButtonArrow::ArrowsExpand => icon_to_string(bootstrap::Bootstrap::ArrowsExpand),
        IpgButtonArrow::ArrowsExpandVertical => icon_to_string(bootstrap::Bootstrap::ArrowsExpandVertical),
        IpgButtonArrow::ArrowsFullscreen => icon_to_string(bootstrap::Bootstrap::ArrowsFullscreen),
        IpgButtonArrow::ArrowsMove => icon_to_string(bootstrap::Bootstrap::ArrowsMove),
        IpgButtonArrow::ArrowsVertical => icon_to_string(bootstrap::Bootstrap::ArrowsVertical),
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
