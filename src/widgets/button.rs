//! Button widget definition

use iced::widget::button::{Status, Style};
use iced::widget::{text, Button};
use iced::{alignment, Border, Color, Element, Length, Padding, Shadow, Theme};
use pyo3::{pyclass, Python};

use crate::access_callbacks;
use crate::access_user_data;
use crate::app::Message;
use super::styling::IpgStyleStandard;


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
    pub style_standard: Option<IpgStyleStandard>,
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
        style_standard: Option<IpgStyleStandard>,
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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonArrow {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

#[derive(Debug, Clone)]
pub enum BTNMessage {
    OnPress,
}

pub fn construct_button(btn: &IpgButton) -> Option<Element<'_, Message>> {
    if !btn.show {
        return None;
    }

    let label = text(btn.label.clone())
        .align_x(btn.text_align_x)
        .align_y(btn.text_align_y)
        .size(btn.text_size);

    let style_standard = btn.style_standard.clone();
    
    let ipg_btn: Element<BTNMessage> = Button::new(label)
        .height(btn.height)
        .padding(btn.padding)
        .width(btn.width)
        .on_press(BTNMessage::OnPress)
        .clip(btn.clip)
        .style(move |theme: &Theme, status| {
            get_styling(theme, status, style_standard.clone())
        })
        .into();

    Some(ipg_btn.map(move |message| Message::Button(btn.id, message)))
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

    // Check user data
    let user_data_lock = access_user_data();
    let user_data_opt = user_data_lock.get(id).map(|ud| Python::attach(|py| ud.clone_ref(py)));
    drop(user_data_lock);

    // Call the callback
    Python::attach(|py| {
        let result = if let Some(user_data) = user_data_opt {
            callback.call1(py, (id, user_data))
        } else {
            callback.call1(py, (id,))
        };
        
        if let Err(err) = result {
            panic!("Button callback error: {err}");
        }
    });
}

fn get_styling(theme: &Theme, status: Status, style_standard: Option<IpgStyleStandard>) -> Style {
    match style_standard {
        Some(IpgStyleStandard::Primary) => match status {
            Status::Active | Status::Pressed => {
                Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.0, 0.478, 1.0))),
                    text_color: Color::WHITE,
                    border: Border::default().rounded(4.0),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            Status::Hovered => {
                Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.0, 0.4, 0.8))),
                    text_color: Color::WHITE,
                    border: Border::default().rounded(4.0),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            Status::Disabled => {
                Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
                    text_color: Color::from_rgb(0.7, 0.7, 0.7),
                    border: Border::default().rounded(4.0),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
        },
        Some(IpgStyleStandard::Success) => match status {
            Status::Active | Status::Pressed => {
                Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.157, 0.655, 0.271))),
                    text_color: Color::WHITE,
                    border: Border::default().rounded(4.0),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            Status::Hovered => {
                Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.1, 0.5, 0.2))),
                    text_color: Color::WHITE,
                    border: Border::default().rounded(4.0),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            Status::Disabled => {
                Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
                    text_color: Color::from_rgb(0.7, 0.7, 0.7),
                    border: Border::default().rounded(4.0),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
        },
        Some(IpgStyleStandard::Danger) => match status {
            Status::Active | Status::Pressed => {
                Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.863, 0.208, 0.271))),
                    text_color: Color::WHITE,
                    border: Border::default().rounded(4.0),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            Status::Hovered => {
                Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.7, 0.15, 0.2))),
                    text_color: Color::WHITE,
                    border: Border::default().rounded(4.0),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            Status::Disabled => {
                Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
                    text_color: Color::from_rgb(0.7, 0.7, 0.7),
                    border: Border::default().rounded(4.0),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
        },
        _ => {
            // Default styling from theme
            iced::widget::button::primary(theme, status)
        }
    }
}
