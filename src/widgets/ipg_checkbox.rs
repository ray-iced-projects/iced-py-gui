//! ipg_checkbox
use crate::graphics::colors::IpgColor;
use crate::widgets::enums::IpgShaping;
use super::styling::IpgStyleStandard;
use crate::state::{access_callbacks, access_user_data1, 
    access_user_data2, IpgState};
use crate::app::Message;
use crate::py_api::helpers::{get_radius, get_width, try_extract_boolean, try_extract_f32, try_extract_string, try_extract_style_standard, try_extract_usize, try_extract_vec_f32};
use crate::widgets::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn};
use crate::state::IpgWidgets;

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap_icon::{IpgIcon, icon_to_char};

use iced::advanced::text;
use iced::{Element, Color, Length, Theme};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::Checkbox;
use iced::widget::checkbox::{self, Status};

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgCheckBox {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub is_checked: bool,
    pub label: Option<String>,
    pub width: Length,
    pub size: Option<f32>,
    pub spacing: Option<f32>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub text_shaping: Option<IpgShaping>,
    pub text_font_id: Option<usize>,
    pub icon_font_id: Option<usize>,
    pub icon: Option<IpgIcon>,
    pub icon_size: Option<f32>,
    pub icon_line_height: Option<f32>,
    pub icon_shaping: Option<IpgShaping>,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgStyleStandard>,
}

#[derive(Debug, Clone, Default)]
pub struct IpgCheckboxStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    pub accent_color: Option<Color>,
    pub accent_color_hovered: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub icon_color: Option<Color>,
    pub text_color: Option<Color>,
}

#[derive(Debug, Clone)]
pub enum ChkMessage {
    OnToggle(bool),
}

pub fn construct_checkbox<'a>(
    ipg_chk: &'a IpgCheckBox, 
    style_opt: Option<&IpgWidgets>) 
    -> Option<Element<'a, Message>> {

    if !ipg_chk.show {
        return None
    };

    let style = get_chk_style(style_opt);

    // Icon related
    let code_point = 
        if let Some(ic) = ipg_chk.icon {
            icon_to_char(ic)
        } else {
            icon_to_char(IpgIcon::Check)
        };

    let size = 
        if let Some(sz) = ipg_chk.icon_size {
            Some(iced::Pixels(sz))
        } else {
            None
        };

    let shaping = 
        if let Some(sh) = &ipg_chk.icon_shaping {
            IpgShaping::to_iced(sh)
        } else {
            Shaping::default()
        };

    let line_height = 
        if let Some(lh) = ipg_chk.icon_line_height {
            LineHeight::Relative(lh)
        } else {
            LineHeight::default()
        };

    let icon = 
        checkbox::Icon {
            font: BOOTSTRAP_FONT,
            code_point,
            size,
            line_height,
            shaping,
        };
    
    // Text related
    let text_line_height = 
        if let Some(lh) = ipg_chk.text_line_height {
            text::LineHeight::Relative(lh)
        } else {
            text::LineHeight::default()
        };

    let text_shaping = 
        if let Some(ts) = &ipg_chk.text_shaping {
            IpgShaping::to_iced(ts)
        } else {
            Shaping::default()
        };

   let chk = 
        Checkbox::new(ipg_chk.is_checked)
            .on_toggle(ChkMessage::OnToggle)
            .width(ipg_chk.width)
            .text_line_height(text_line_height)
            .text_shaping(text_shaping)
            .icon(icon)
            .style(move|theme: &Theme, status| {   
                get_styling(theme, status,
                    style.clone(), 
                    ipg_chk.style_standard.clone(),
                    ipg_chk.is_checked,
                    )  
                });
    
    let chk = 
        if let Some(lb) = &ipg_chk.label {
            chk.label(lb.clone())
        } else { chk };

    let chk = 
        if let Some(sz) = ipg_chk.size {
            chk.size(sz)
        } else { chk };

    let chk: Element<'_, ChkMessage> = 
        if let Some(sp) = ipg_chk.spacing {
            chk.spacing(sp).into()
        } else { chk.into() };

    Some(chk.map(move |message| Message::CheckBox(ipg_chk.id, message)))

}

pub fn checkbox_callback(state: &mut IpgState, id: usize, message: ChkMessage) {

    match message {
        ChkMessage::OnToggle(on_toggle) => {
            let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
            wci.on_toggle = Some(on_toggle);
            let _ = set_or_get_widget_callback_data(state, wci);

            process_callback(id, on_toggle, "on_toggle".to_string());
        }
    }
}

pub fn process_callback(
        id: usize, 
        is_checked: bool, 
        event_name: String) 
{
    let ud1 = access_user_data1();
    let app_cbs = access_callbacks();

    // Retrieve the callback
    let callback = match app_cbs.callbacks.get(&(id, event_name)) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_cbs);

    // Check user data from ud1
    if let Some(user_data) = ud1.user_data.get(&id) {
        Python::attach(|py| {
            if let Err(err) = callback.call1(py, (id, is_checked, user_data)) {
                panic!("Checkbox callback error: {err}");
            }
        });
        drop(ud1); // Drop ud1 before processing ud2
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Check user data from ud2
    let ud2 = access_user_data2();
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::attach(|py| {
            if let Err(err) = callback.call1(py, (id, is_checked, user_data)) {
                panic!("Checkbox callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only the id and is_checked
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, is_checked)) {
            panic!("Checkbox callback error: {err}");
        }
    });
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCheckboxParam {
    Icon,
    IconFont,
    IconLineHeight,
    IconSize,
    IconShaping,
    IsChecked,
    Label,
    Spacing,
    Style,
    StyleStandard,
    TextFont,
    TextLineHeight,
    TextShaping,
    TextSize,
    Width,
    WidthFill,
    Show,
}

pub fn checkbox_param_update(chk: &mut IpgCheckBox,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_checkbox_update(item);
    let name = "Checkbox".to_string();
    match update {
        IpgCheckboxParam::Icon => {
            chk.icon = Some(IpgIcon::extract(value));
        },
        IpgCheckboxParam::IconFont => {
            
        },
        IpgCheckboxParam::IconLineHeight => {
            chk.icon_line_height = Some(try_extract_f32(value, name));
        },
        IpgCheckboxParam::IconShaping => {
            chk.icon_shaping = IpgShaping::extract(value);
        },
        IpgCheckboxParam::IconSize => {
            chk.icon_size = Some(try_extract_f32(value, name));
        },
        IpgCheckboxParam::IsChecked => {
            chk.is_checked = try_extract_boolean(value, name);
        },
        IpgCheckboxParam::Label => {
            chk.label = Some(try_extract_string(value, name));
        },
        IpgCheckboxParam::Show => {
            chk.show = try_extract_boolean(value, name);
        },
        IpgCheckboxParam::Spacing => {
            chk.spacing = Some(try_extract_f32(value, name));
        },
        IpgCheckboxParam::TextLineHeight => {
            chk.text_line_height =  Some(try_extract_f32(value, name));
        },
        IpgCheckboxParam::TextShaping => {
            chk.text_shaping = IpgShaping::extract(value);
        },
        IpgCheckboxParam::TextSize => {
            chk.text_size = Some(try_extract_f32(value, name));
        },
        IpgCheckboxParam::Style => {
            chk.style_id = Some(try_extract_usize(value, name))
        },
        IpgCheckboxParam::StyleStandard => {
            let val = try_extract_style_standard(value, name);
            chk.style_standard = Some(val)
        },
        IpgCheckboxParam::Width => {
            let wd = try_extract_f32(value, name);
            chk.width =  get_width(Some(wd), false);
        },
        IpgCheckboxParam::WidthFill => {
            let wd = try_extract_boolean(value, name);
            chk.width =  get_width(None, wd);
        },
        
        IpgCheckboxParam::TextFont => todo!(),
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCheckboxStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BackgroundIpgColorHovered,
    BackgroundRgbaColorHovered,
    AccentIpgColor,
    AccentRgbaColor,
    AccentIpgColorHovered,
    AccentRgbaColorHovered,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    IconIpgColor,
    IconRgbaColor,
    TextIpgColor,
    TextRgbaColor,
}

pub fn checkbox_style_update(style: &mut IpgCheckboxStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    let update = try_extract_checkbox_style_update(item);
    let name = "CheckboxStyle".to_string();
    match update {
        IpgCheckboxStyleParam::BackgroundIpgColor => {
            let color = IpgColor::extract(value, name);
            style.background_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::BackgroundRgbaColor => {
            style.background_color = Some(Color::from(IpgColor::extract_rgba(value, name)));
        },
        IpgCheckboxStyleParam::BackgroundIpgColorHovered => {
            let color = IpgColor::extract(value, name);
            style.background_color_hovered = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::BackgroundRgbaColorHovered => {
            style.background_color_hovered = Some(Color::from(IpgColor::extract_rgba(value, name)));
        },
        IpgCheckboxStyleParam::AccentIpgColor => {
            let color = IpgColor::extract(value, name);
            style.accent_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::AccentRgbaColor => {
            style.accent_color = Some(Color::from(IpgColor::extract_rgba(value, name)));
        },
        IpgCheckboxStyleParam::AccentIpgColorHovered => {
            let color = IpgColor::extract(value, name);
            style.accent_color_hovered = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::AccentRgbaColorHovered => {
            style.accent_color_hovered = Some(Color::from(IpgColor::extract_rgba(value, name)));
        },
        IpgCheckboxStyleParam::BorderIpgColor => {
            let color = IpgColor::extract(value, name);
            style.border_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(IpgColor::extract_rgba(value, name)));
        },
        IpgCheckboxStyleParam::BorderRadius => {
            style.border_radius = Some(try_extract_vec_f32(value, name));
        },
        IpgCheckboxStyleParam::BorderWidth => {
            style.border_width = Some(try_extract_f32(value, name));
        },
        IpgCheckboxStyleParam::IconIpgColor => {
            let color = IpgColor::extract(value, name);
            style.icon_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::IconRgbaColor => {
            style.icon_color = Some(Color::from(IpgColor::extract_rgba(value, name)));
        },
        IpgCheckboxStyleParam::TextIpgColor => {
            let color = IpgColor::extract(value, name);
            style.text_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgCheckboxStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(IpgColor::extract_rgba(value, name)));
        },
    }
}

pub fn try_extract_checkbox_update(update_obj: &PyObject) -> IpgCheckboxParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgCheckboxParam>(py);

        match res {
            Ok(update) => update,
            Err(_) => panic!("Checkbox update extraction failed"),
        }
    })
}

pub fn get_styling(theme: &Theme, status: Status,
                    style_opt: Option<IpgCheckboxStyle>,
                    style_standard: Option<IpgStyleStandard>,
                    is_checked: bool, 
                    ) -> checkbox::Style 
{

    if style_standard.is_none() && style_opt.is_none() {
        return checkbox::primary(theme, status)
    }
    
    if let Some(style_std) = &style_standard {
        
        let mut std_style = match style_std {

            IpgStyleStandard::Primary => {
                checkbox::primary(theme, status) 
            },
            IpgStyleStandard::Secondary => {
                checkbox::secondary(theme, status)
            },
            IpgStyleStandard::Success => {
                checkbox::success(theme, status)
            },
            IpgStyleStandard::Danger => {
                checkbox::danger(theme, status)
            },
            IpgStyleStandard::Text => 
                panic!("StandardStyle::Text not valid for checkbox"),
            IpgStyleStandard::Warning => {
                panic!("StandardStyle::Danger not valid for checkbox")
            },
        };

        if let Some(style) = style_opt {
            if let Some(bw) = style.border_width {
                std_style.border.width = bw;
            }
            if let Some(br) = style.border_radius {
                std_style.border.radius = 
                    get_radius(br, 
                        "Checkbox".to_string());
            }
        }
       
        return std_style

    }

    if style_opt.is_none() {
        return checkbox::primary(theme, status)
    }

    let style = style_opt.unwrap();

    let mut border_style = checkbox::primary(theme, Status::Active { is_checked }).border;
    
    if let Some(bc) = style.border_color {
        border_style.color = bc;
    }

    if let Some(r) = style.border_radius {
        border_style.radius = get_radius(r, "Checkbox".to_string());
    }
    
    if let Some(bw) = style.border_width {
        border_style.width = bw;
    }

    match status {
        Status::Active { is_checked } => {
            let mut active_style = checkbox::primary(theme, Status::Active { is_checked });
            if style.background_color.is_some() && !is_checked {
                active_style.background = iced::Background::Color(style.background_color.unwrap());
            } else if style.accent_color.is_some() && is_checked {
                active_style.background = iced::Background::Color(style.accent_color.unwrap());
            }
            if let Some(ic) = style.icon_color {
                active_style.icon_color = ic;
            }

            active_style.text_color = style.text_color;
            
            active_style.border = border_style;

            active_style
        },
        Status::Hovered { is_checked } => {
            let mut hovered_style = checkbox::primary(theme, Status::Hovered { is_checked });
            if style.background_color_hovered.is_some() && !is_checked {
                hovered_style.background = iced::Background::Color(style.background_color_hovered.unwrap());
            } else if style.accent_color_hovered.is_some() && is_checked {
                hovered_style.background = iced::Background::Color(style.accent_color_hovered.unwrap());
            }
            if let Some(ic) = style.icon_color {
                hovered_style.icon_color = ic;
            }

            hovered_style.text_color = style.text_color;
            
            hovered_style.border = border_style;

            hovered_style
        },
        Status::Disabled { is_checked } => {
            checkbox::danger(theme, Status::Disabled { is_checked })
        },
    }
    
}

pub fn try_extract_checkbox_style_update(update_obj: &PyObject) -> IpgCheckboxStyleParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgCheckboxStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Checkbox style update extraction failed"),
        }
    })
}

pub fn get_chk_style(style: Option<&IpgWidgets>) -> Option<IpgCheckboxStyle>{
    match style {
        Some(IpgWidgets::IpgCheckboxStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}