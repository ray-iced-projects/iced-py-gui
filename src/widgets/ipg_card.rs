//! ipg_card definition
use std::collections::HashMap;

use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::state::IpgWidgets;
use crate::widgets::callbacks::invoke_callback;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_height, set_opt_f32, 
    set_opt_iced_color, set_opt_iced_color_from_rgba, 
    set_opt_string, set_opt_usize, set_opt_vec_f32, set_width
};

use iced::widget::{Column, Text, text};
use iced::{Color, Element, Length, Theme};

use iced_aw::widgets::card::Card;
use iced_aw::style;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgCard {
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
    pub style_std: Option<IpgCardStyleStd>,
    pub style_button: Option<usize>,
    pub show: bool,
}

impl IpgCard {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, IpgWidgets>, 
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
                .and_then(IpgWidgets::as_card_style).cloned();

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
            Card::new(head, body)
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
pub struct IpgCardStyle {
    pub id: usize,
    pub background: Option<Color>, 
    pub border_radius: Option<f32>, 
    pub border_width: Option<f32>, 
    pub border_color: Option<Color>, 
    pub head_background: Option<Color>, 
    pub head_text_color: Option<Color>, 
    pub body_background: Option<Color>, 
    pub body_text_color: Option<Color>, 
    pub foot_background: Option<Color>, 
    pub foot_text_color: Option<Color>, 
    pub close_color:Option<Color>,
}

impl IpgCardStyle {
    /// Apply user-defined style overrides to an existing iced button::Style
    pub fn to_iced(
        &self, 
        theme: &Theme, 
        status: iced_aw::card::Status,
        std_style_opt: &Option<IpgCardStyleStd>,
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
pub enum IpgCardStyleStd {
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

impl IpgCardStyleStd {
    pub fn to_iced(
        &self,
        theme: &Theme, 
        status: iced_aw::card::Status,
        ) -> style::card::Style {
        
        match self {
            IpgCardStyleStd::Danger => style::card::danger(theme, status),
            IpgCardStyleStd::Dark => style::card::dark(theme, status),
            IpgCardStyleStd::Info => style::card::info(theme, status),
            IpgCardStyleStd::Light => style::card::light(theme, status),
            IpgCardStyleStd::Primary => style::card::primary(theme, status),
            IpgCardStyleStd::Secondary => style::card::secondary(theme, status),
            IpgCardStyleStd::Success => style::card::success(theme, status),
            IpgCardStyleStd::Warning => style::card::warning(theme, status),
            IpgCardStyleStd::White => style::card::white(theme, status),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCardStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    HeadBackgroundIpgColor,
    HeadBackgroundRgbaColor,
    HeadTextIpgColor,
    HeadTextRgbaColor,
    BodyBackgroundIpgColor,
    BodyBackgroundRgbaColor,
    BodyTextIpgColor,
    BodyTextRgbaColor,
    FootBackgroundIpgColor,
    FootBackgroundRgbaColor,
    FootTextIpgColor,
    FootTextRgbaColor,
    CloseIpgColor,
    CloseRgbaColor,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCardParam {
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

impl WidgetParamUpdate for IpgCard {
    type Param = IpgCardParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgCardParam::Body => set_opt_string(&mut self.body, value, "Body"),
            IpgCardParam::CloseSize => set_opt_f32(&mut self.close_size, value, "CloseSize"),
            IpgCardParam::Foot => set_opt_string(&mut self.foot, value, "Foot"),
            IpgCardParam::Head => set_opt_string(&mut self.head, value, "Head"),
            IpgCardParam::Height => set_height(&mut self.height, value, "Height"),
            IpgCardParam::IsOpen => set_bool(&mut self.is_open, value, "IsOpen"),
            IpgCardParam::MaxHeight => set_opt_f32(&mut self.max_height, value, "MaxHeight"),
            IpgCardParam::MaxWidth => set_opt_f32(&mut self.max_width, value, "MaxWidth"),
            IpgCardParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            IpgCardParam::PaddingBody => set_opt_vec_f32(&mut self.padding_body, value, "PaddingBody"),
            IpgCardParam::PaddingFoot => set_opt_vec_f32(&mut self.padding_foot, value, "PaddingFoot"),
            IpgCardParam::PaddingHead => set_opt_vec_f32(&mut self.padding_head, value, "PaddingHead"),
            IpgCardParam::Show => set_bool(&mut self.show, value, "Show"),
            IpgCardParam::StyleButton => set_opt_usize(&mut self.style_button, value, "StyleButton"),
            IpgCardParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            IpgCardParam::Width => set_width(&mut self.width, value, "Width"),
        }
    }
}

impl WidgetParamUpdate for IpgCardStyle {
    type Param = IpgCardStyleParam;
    
    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgCardStyleParam::BackgroundIpgColor => set_opt_iced_color(&mut self.background, value, "BackgroundIpgColor"),
            IpgCardStyleParam::BackgroundRgbaColor => set_opt_iced_color_from_rgba(&mut self.background, value, "BackgroundRgbaColor"),
            IpgCardStyleParam::BorderIpgColor => set_opt_iced_color(&mut self.border_color, value, "BorderIpgColor"),
            IpgCardStyleParam::BorderRgbaColor => set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgbaColor"),
            IpgCardStyleParam::BorderRadius => set_opt_f32(&mut self.border_radius, value, "BorderRadius"),
            IpgCardStyleParam::BorderWidth => set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            IpgCardStyleParam::HeadBackgroundIpgColor => set_opt_iced_color(&mut self.head_background, value, "HeadBackgroundIpgColor"),
            IpgCardStyleParam::HeadBackgroundRgbaColor => set_opt_iced_color_from_rgba(&mut self.head_background, value, "HeadBackgroundRgbaColor"),
            IpgCardStyleParam::HeadTextIpgColor => set_opt_iced_color(&mut self.head_text_color, value, "HeadTextIpgColor"),
            IpgCardStyleParam::HeadTextRgbaColor => set_opt_iced_color_from_rgba(&mut self.head_text_color, value, "HeadTextRgbaColor"),
            IpgCardStyleParam::BodyBackgroundIpgColor => set_opt_iced_color(&mut self.body_background, value, "BodyBackgroundIpgColor"),
            IpgCardStyleParam::BodyBackgroundRgbaColor => set_opt_iced_color_from_rgba(&mut self.body_background, value, "BodyBackgroundRgbaColor"),
            IpgCardStyleParam::BodyTextIpgColor => set_opt_iced_color(&mut self.body_text_color, value, "BodyTextIpgColor"),
            IpgCardStyleParam::BodyTextRgbaColor => set_opt_iced_color_from_rgba(&mut self.body_text_color, value, "BodyTextRgbaColor"),
            IpgCardStyleParam::FootBackgroundIpgColor => set_opt_iced_color(&mut self.foot_background, value, "FootBackgroundIpgColor"),
            IpgCardStyleParam::FootBackgroundRgbaColor => set_opt_iced_color_from_rgba(&mut self.foot_background, value, "FootBackgroundRgbaColor"),
            IpgCardStyleParam::FootTextIpgColor => set_opt_iced_color(&mut self.foot_text_color, value, "FootTextIpgColor"),
            IpgCardStyleParam::FootTextRgbaColor => set_opt_iced_color_from_rgba(&mut self.foot_text_color, value, "FootTextRgbaColor"),
            IpgCardStyleParam::CloseIpgColor => set_opt_iced_color(&mut self.close_color, value, "CloseIpgColor"),
            IpgCardStyleParam::CloseRgbaColor => set_opt_iced_color_from_rgba(&mut self.close_color, value, "CloseRgbaColor"),
        }
    }
}