//! ipg_card definition

use std::collections::HashMap;

use crate::app::Message;
use crate::py_api::helpers::{get_len, get_padding};
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_opt_f32, set_opt_iced_color, set_opt_iced_color_from_rgba, set_t_value
};

use iced::widget::{Space, Text};
use iced::{Element, Length, Theme};

use iced_aw::widgets::card;
use iced_aw::style;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Card {
    pub id: usize,
    pub is_open: bool,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub padding: Option<Vec<f32>>,
    // pub padding_head: Option<Vec<f32>>,
    pub padding_body: Option<Vec<f32>>,
    pub padding_foot: Option<Vec<f32>>,
    pub close_icon: Option<bool>,
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
        mut content: Vec<Element<'a, Message>>,
        widgets: &HashMap<usize, Widgets>, 
    )-> Element<'a, Message> {

        if !self.show || !self.is_open {return Space::new().into()}

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_card_style).cloned();

        // expect [body], [head, body] or [head, body, foot]
        let (head, body, foot) = 
            if content.len() >= 3 {
                (content.remove(0), content.remove(0), Some(content.remove(0)))
            } else if content.len() >= 2 {
                (content.remove(0), content.remove(0), None)
            } else if content.len() >= 1 {
                (Element::from(Space::new()), content.remove(0), None)
            } else {
                eprint!("[WARNING] Expected the Card to hold a least one widget, therefore not constructed");
                return Space::new().into()
            };
        
        // header padding has a bug for now.
        // let mut hd_pad = get_padding(&self.padding_head);
        let mut bd_pad = get_padding(&self.padding_body);
        let mut ft_pad = get_padding(&self.padding_foot);
            
        if self.padding.is_some() {
            let pd = get_padding(&self.padding);
            // hd_pad = pd;
            bd_pad = pd;
            ft_pad = pd;
        }

        let card  = 
            card::Card::new(head, body)
                .foot(foot)
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height))
                // .padding_head(hd_pad)
                .padding_body(bd_pad)
                .padding_foot(ft_pad)
                .close_size(self.close_size.unwrap_or(16.0))
                .on_close(Message::Card(self.id, CardMessage::OnClose))
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

        let card = if let Some(mh) = self.max_height {
            card.max_height(mh)
        } else {
            card
        };

        card.into()
        
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

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
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

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
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

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum CardParam {
    IsOpen,
    Width,
    WidthFill,
    Height,
    HeightFill,
    Fill,
    MaxWidth,
    MaxHeight,
    Padding,
    PaddingHead,
    PaddingBody,
    PaddingFoot,
    CloseIcon,
    CloseSize,
    Head,
    Body,
    Foot,
    StyleId,
    StyleStd,
    StyleButton,
    Show,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Card {
    type Param = CardParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            CardParam::Body => set_t_value(&mut self.body, value, "CardParam::Body"),
            CardParam::CloseIcon => todo!(),
            CardParam::CloseSize => set_t_value(&mut self.close_size, value, "CardParam::CloseSize"),
            CardParam::Fill => todo!(),
            CardParam::Foot => set_t_value(&mut self.foot, value, "CardParam::Foot"),
            CardParam::Head => set_t_value(&mut self.head, value, "CardParam::Head"),
            CardParam::Height => set_t_value(&mut self.height, value, "CardParam::Height"),
            CardParam::HeightFill => todo!(),
            CardParam::IsOpen => set_t_value(&mut self.is_open, value, "CardParam::IsOpen"),
            CardParam::MaxHeight => set_t_value(&mut self.max_height, value, "CardParam::MaxHeight"),
            CardParam::MaxWidth => set_t_value(&mut self.max_width, value, "CardParam::MaxWidth"),
            CardParam::Padding => set_t_value(&mut self.padding, value, "CardParam::Padding"),
            CardParam::PaddingBody => set_t_value(&mut self.padding_body, value, "CardParam::PaddingBody"),
            CardParam::PaddingFoot => set_t_value(&mut self.padding_foot, value, "CardParam::PaddingFoot"),
            CardParam::PaddingHead => eprintln!("[WARNING] PaddingHead is currently disabled due to a bug"),
            CardParam::Show => set_t_value(&mut self.show, value, "CardParam::Show"),
            CardParam::StyleButton => set_t_value(&mut self.style_button, value, "CardParam::StyleButton"),
            CardParam::StyleId => set_t_value(&mut self.style_id, value, "CardParam::StyleId"),
            CardParam::StyleStd => todo!(),
            CardParam::Width => set_t_value(&mut self.width, value, "CardParam::Width"),
            CardParam::WidthFill => todo!(),
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