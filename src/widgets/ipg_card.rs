//! ipg_card definition

use std::collections::HashMap;

use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_height, set_opt_f32, 
    set_opt_iced_color, set_opt_iced_color_from_rgba, 
    set_opt_string, set_opt_usize, set_opt_vec_f32, set_width
};

use iced::widget::{Column, Text, text};
use iced::{Element, Length, Theme};

use iced_aw::widgets::card;
use iced_aw::style;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub parent_id: String,
    pub is_open: bool,
    
    pub button_id: Option<usize>,
    pub button_label: Option<String>,
    pub width: Length,
    pub height: Length,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub padding_head: Option<Vec<f32>>,
    pub padding_body: Option<Vec<f32>>,
    pub padding_foot: Option<Vec<f32>>,
    pub close_size: Option<f32>,
    pub head: Option<String>,
    pub body: Option<String>,
    pub foot: Option<String>,
    pub style_id: Option<usize>,
    pub style_std: Option<CardStyleStd>,
    pub style_button: Option<usize>,
    pub show: bool,
}

impl Card {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>, 
    )-> Option<Element<'a, Message>> {

        if !self.show || !self.is_open {return None}
        // if !self.is_open {
            // let label = if let Some(label) = self.button_label.clone() {
            //     label
            // } else {
            //     "Open card".to_string()
            // };
            // let btn: Element<CardMessage> = Button::new(Text::new(label))
            //     .on_press(CardMessage::OnOpen)
            //     .into();
            // let btn_mapped: Element<Message> = btn.map(move |message| Message::Card(self.id, message));
            // return Some(btn_mapped)
        // }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_card_style).cloned();

        let head: Element<CardMessage> = if let Some(head) = &self.head {
            Text::new(head)
                .width(Length::Fill)
                .into()
        } else {
            text("").into()
        };

        let body: Element<CardMessage> = if let Some(body) = &self.body {
            Text::new(body)
                .width(Length::Fill)
                .into()
        } else {
            text("").into()
        };

        let body: Element<CardMessage> = Column::new().push(body).into();

        let mut hd_pad = get_padding(&self.padding_head);
        let mut bd_pad = get_padding(&self.padding_body);
        let mut ft_pad = get_padding(&self.padding_foot);
            
        if self.padding.is_some() {
            let pd = get_padding(&self.padding);
            hd_pad = pd;
            bd_pad = pd;
            ft_pad = pd;
        }

        let card  = 
            card::Card::new(head, body)
                .width(self.width)
                .height(self.height)
                .padding_head(hd_pad)
                .padding_body(bd_pad)
                .padding_foot(ft_pad)
                .close_size(self.close_size.unwrap_or(16.0))
                .on_close(CardMessage::OnClose)
                .style(move |theme: &Theme, status| {
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, status, &self.style_std)
                    } else {
                       match &self.style_std {
                            Some(std) => std.to_iced(theme, status),
                            None => style::card::primary(theme, status),
                        }
                    }
                }
            );
            
        let card = if let Some(foot) = &self.foot {
            card.foot(Some(Text::new(foot)
                .width(Length::Fill)
                ))
        } else {
            card
        };

        let card = if let Some(mw) = self.max_width {
            card.max_width(mw)
        } else {
            card
        };

        let card: Element<'_, CardMessage> = if let Some(mh) = self.max_height {
            card.max_height(mh)
        } else {
            card
        }.into();
        
        Some(card.map(move |message: CardMessage| Message::Card(self.id, message)))
        
    }
}

#[derive(Debug, Clone)]
pub enum CardMessage {
    OnClose,
    OnOpen,
}

pub fn card_callback(id: usize, message: CardMessage) {
    match message {
        CardMessage::OnClose => {
            invoke_callback(id, "on_close", "Card");
        },
        CardMessage::OnOpen => {
            invoke_callback(id, "on_open", "Card");
        },
    }
}

#[derive(Debug, Clone)]
pub struct CardStyle {
    pub id: usize,
    pub background: Option<iced::Color>, 
    pub border_radius: Option<f32>, 
    pub border_width: Option<f32>, 
    pub border_color: Option<iced::Color>, 
    pub head_background: Option<iced::Color>, 
    pub head_text_color: Option<iced::Color>, 
    pub body_background: Option<iced::Color>, 
    pub body_text_color: Option<iced::Color>, 
    pub foot_background: Option<iced::Color>, 
    pub foot_text_color: Option<iced::Color>, 
    pub close_color:Option<iced::Color>,
}

impl CardStyle {
    /// Apply user-defined style overrides to an existing iced button::Style
    pub fn to_iced(
        &self, 
        theme: &Theme, 
        status: iced_aw::card::Status,
        std_style_opt: &Option<CardStyleStd>,
        ) -> style::card::Style {

            let mut style = if let Some(std) = std_style_opt {
                std.to_iced(theme, status)
            } else {
                style::card::Style::default()
            };
            
            if let Some(bkg) = self.background {
                style.background = bkg.into();
            }

            if let Some(br) = self.border_radius {
                style.border_radius = br;
            }

            if let Some(bw) = self.border_width {
                style.border_width = bw;
            }

            if let Some(bc) = self.border_color {
                style.border_color = bc;
            }

            if let Some(h_bkg) = self.head_background {
                style.head_background = h_bkg.into();
            }

            if let Some(h_tc) = self.head_text_color {
                style.head_text_color = h_tc;
            }

            if let Some(b_bkg) = self.body_background {
                style.body_background = b_bkg.into();
            }

            if let Some(b_tc) = self.body_text_color {
                style.body_text_color = b_tc;
            }

            if let Some(f_bkg) = self.foot_background {
                style.foot_background = f_bkg.into();
            }

            if let Some(f_tc) = self.foot_text_color {
                style.foot_text_color = f_tc;
            }
        
            if let Some(c) = self.close_color {
                style.close_color = c;
            }
        
            style
        }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum CardStyleStd {
    Danger,
    Dark,
    Info,
    Light,
    Primary,
    Secondary,
    Success,
    Warning,
    White,
}

impl CardStyleStd {
    pub fn to_iced(
        &self,
        theme: &Theme, 
        status: iced_aw::card::Status,
        ) -> style::card::Style {
        
        match self {
            CardStyleStd::Danger => style::card::danger(theme, status),
            CardStyleStd::Dark => style::card::dark(theme, status),
            CardStyleStd::Info => style::card::info(theme, status),
            CardStyleStd::Light => style::card::light(theme, status),
            CardStyleStd::Primary => style::card::primary(theme, status),
            CardStyleStd::Secondary => style::card::secondary(theme, status),
            CardStyleStd::Success => style::card::success(theme, status),
            CardStyleStd::Warning => style::card::warning(theme, status),
            CardStyleStd::White => style::card::white(theme, status),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum CardStyleParam {
    BackgroundColor,
    BackgroundRgbaColor,
    BorderColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    HeadBackgroundColor,
    HeadBackgroundRgbaColor,
    HeadTextColor,
    HeadTextRgbaColor,
    BodyBackgroundColor,
    BodyBackgroundRgbaColor,
    BodyTextColor,
    BodyTextRgbaColor,
    FootBackgroundColor,
    FootBackgroundRgbaColor,
    FootTextColor,
    FootTextRgbaColor,
    CloseColor,
    CloseRgbaColor,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum CardParam {
    Body,
    CloseSize,
    Foot,
    Head,
    Height,
    IsOpen,
    MaxHeight,
    MaxWidth,
    Padding,
    PaddingBody,
    PaddingFoot,
    PaddingHead,
    Show,
    StyleButton,
    StyleId,
    Width,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Card {
    type Param = CardParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            CardParam::Body => set_opt_string(&mut self.body, value, "Body"),
            CardParam::CloseSize => set_opt_f32(&mut self.close_size, value, "CloseSize"),
            CardParam::Foot => set_opt_string(&mut self.foot, value, "Foot"),
            CardParam::Head => set_opt_string(&mut self.head, value, "Head"),
            CardParam::Height => set_height(&mut self.height, value, "Height"),
            CardParam::IsOpen => set_bool(&mut self.is_open, value, "IsOpen"),
            CardParam::MaxHeight => set_opt_f32(&mut self.max_height, value, "MaxHeight"),
            CardParam::MaxWidth => set_opt_f32(&mut self.max_width, value, "MaxWidth"),
            CardParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            CardParam::PaddingBody => set_opt_vec_f32(&mut self.padding_body, value, "PaddingBody"),
            CardParam::PaddingFoot => set_opt_vec_f32(&mut self.padding_foot, value, "PaddingFoot"),
            CardParam::PaddingHead => set_opt_vec_f32(&mut self.padding_head, value, "PaddingHead"),
            CardParam::Show => set_bool(&mut self.show, value, "Show"),
            CardParam::StyleButton => set_opt_usize(&mut self.style_button, value, "StyleButton"),
            CardParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            CardParam::Width => set_width(&mut self.width, value, "Width"),
        }
    }
}

impl WidgetParamUpdate for CardStyle {
    type Param = CardStyleParam;
    
    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            CardStyleParam::BackgroundColor => set_opt_iced_color(&mut self.background, value, "BackgroundColor"),
            CardStyleParam::BackgroundRgbaColor => set_opt_iced_color_from_rgba(&mut self.background, value, "BackgroundRgbaColor"),
            CardStyleParam::BorderColor => set_opt_iced_color(&mut self.border_color, value, "BorderColor"),
            CardStyleParam::BorderRgbaColor => set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgbaColor"),
            CardStyleParam::BorderRadius => set_opt_f32(&mut self.border_radius, value, "BorderRadius"),
            CardStyleParam::BorderWidth => set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            CardStyleParam::HeadBackgroundColor => set_opt_iced_color(&mut self.head_background, value, "HeadBackgroundColor"),
            CardStyleParam::HeadBackgroundRgbaColor => set_opt_iced_color_from_rgba(&mut self.head_background, value, "HeadBackgroundRgbaColor"),
            CardStyleParam::HeadTextColor => set_opt_iced_color(&mut self.head_text_color, value, "HeadTextColor"),
            CardStyleParam::HeadTextRgbaColor => set_opt_iced_color_from_rgba(&mut self.head_text_color, value, "HeadTextRgbaColor"),
            CardStyleParam::BodyBackgroundColor => set_opt_iced_color(&mut self.body_background, value, "BodyBackgroundColor"),
            CardStyleParam::BodyBackgroundRgbaColor => set_opt_iced_color_from_rgba(&mut self.body_background, value, "BodyBackgroundRgbaColor"),
            CardStyleParam::BodyTextColor => set_opt_iced_color(&mut self.body_text_color, value, "BodyTextColor"),
            CardStyleParam::BodyTextRgbaColor => set_opt_iced_color_from_rgba(&mut self.body_text_color, value, "BodyTextRgbaColor"),
            CardStyleParam::FootBackgroundColor => set_opt_iced_color(&mut self.foot_background, value, "FootBackgroundColor"),
            CardStyleParam::FootBackgroundRgbaColor => set_opt_iced_color_from_rgba(&mut self.foot_background, value, "FootBackgroundRgbaColor"),
            CardStyleParam::FootTextColor => set_opt_iced_color(&mut self.foot_text_color, value, "FootTextColor"),
            CardStyleParam::FootTextRgbaColor => set_opt_iced_color_from_rgba(&mut self.foot_text_color, value, "FootTextRgbaColor"),
            CardStyleParam::CloseColor => set_opt_iced_color(&mut self.close_color, value, "CloseColor"),
            CardStyleParam::CloseRgbaColor => set_opt_iced_color_from_rgba(&mut self.close_color, value, "CloseRgbaColor"),
        }
    }
}