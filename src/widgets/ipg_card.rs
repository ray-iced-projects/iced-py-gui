//! ipg_card definition

use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::helpers::{get_len, get_padding};
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};

use iced::widget::Space;
use iced::{Element, Theme};

use crate::iced_aw_widgets::card::{aw_card, aw_status};
use crate::iced_aw_widgets::card::aw_style;

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
    pub close_icon_size: Option<f32>,
    pub style_id: Option<usize>,
    pub style_std: Option<CardStyleStd>,
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
    )-> Option<Element<'a, Message>> {

        if !self.show || !self.is_open {return None}

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_card_style).cloned();

        // expect [body], [head, body] or [head, body, foot]
        let (head, body, foot) = 
            if content.len() == 3 {
                (content.remove(0), content.remove(0), Some(content.remove(0)))
            } else if content.len() == 2 {
                (content.remove(0), content.remove(0), None)
            } else if content.len() == 1 {
                (Element::from(Space::new()), content.remove(0), None)
            } else {
                eprint!("[WARNING] Expected the Card to hold a least one widget or less then 3, therefore not constructed");
                return None
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
            aw_card::Card::new(head, body)
                .foot(foot)
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height))
                // .padding_head(hd_pad)
                .padding_body(bd_pad)
                .padding_foot(ft_pad)
                .close_size(self.close_icon_size.unwrap_or(16.0))
                .on_close(Message::Card(self.id, CardMessage::OnClose))
                .style(move |theme: &Theme, status| {
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, status, &self.style_std)
                    } else {
                       match &self.style_std {
                            Some(std) => std.to_iced(theme, status),
                            None => aw_style::primary(theme, status),
                        }
                    }
                }
            );

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

        Some(card.into())
        
    }
}

#[derive(Debug, Clone)]
pub enum CardMessage {
    OnClose,
}

pub fn card_callback(id: usize, message: CardMessage) {
    match message {
        CardMessage::OnClose => {
            invoke_callback(id, "on_close", "Card");
        },
    }
}

#[derive(Debug, Clone)]
pub struct CardStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,
    pub border_radius: Option<f32>, 
    pub border_width: Option<f32>, 
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>, 
    pub head_background_color: Option<Color>,
    pub head_background_color_alpha: Option<f32>,
    pub head_background_rgba: Option<[f32; 4]>, 
    pub body_background_color: Option<Color>,
    pub body_background_color_alpha: Option<f32>,
    pub body_background_rgba: Option<[f32; 4]>, 
    pub foot_background_color: Option<Color>,
    pub foot_background_color_alpha: Option<f32>,
    pub foot_background_rgba: Option<[f32; 4]>, 
}

impl CardStyle {
    /// Apply user-defined style overrides to an existing iced button::Style
    pub fn to_iced(
        &self, 
        theme: &Theme, 
        status: aw_status::Status,
        std_style_opt: &Option<CardStyleStd>,
        ) -> aw_style::Style {

            // convert colors
            let background = 
                Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
            let border_color = 
                Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);
            let head_background = 
                Color::rgba_ipg_color_to_iced(self.head_background_rgba, &self.head_background_color, self.head_background_color_alpha);
            let body_background = 
                Color::rgba_ipg_color_to_iced(self.body_background_rgba, &self.body_background_color, self.body_background_color_alpha);
            let foot_background = 
                Color::rgba_ipg_color_to_iced(self.foot_background_rgba, &self.foot_background_color, self.foot_background_color_alpha);

            let mut style = if let Some(std) = std_style_opt {
                std.to_iced(theme, status)
            } else {
                aw_style::Style::default()
            };
            
            if let Some(bkg) = background {
                style.background = bkg.into();
            }

            if let Some(br) = self.border_radius {
                style.border_radius = br;
            }

            if let Some(bw) = self.border_width {
                style.border_width = bw;
            }

            if let Some(bc) = border_color {
                style.border_color = bc;
            }

            if let Some(h_bkg) = head_background {
                style.head_background = h_bkg.into();
            }

            if let Some(b_bkg) = body_background {
                style.body_background = b_bkg.into();
            }

            if let Some(f_bkg) = foot_background {
                style.foot_background = f_bkg.into();
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
        status: aw_status::Status,
        ) -> aw_style::Style {
        
        match self {
            CardStyleStd::Danger => aw_style::danger(theme, status),
            CardStyleStd::Dark => aw_style::dark(theme, status),
            CardStyleStd::Info => aw_style::info(theme, status),
            CardStyleStd::Light => aw_style::light(theme, status),
            CardStyleStd::Primary => aw_style::primary(theme, status),
            CardStyleStd::Secondary => aw_style::secondary(theme, status),
            CardStyleStd::Success => aw_style::success(theme, status),
            CardStyleStd::Warning => aw_style::warning(theme, status),
            CardStyleStd::White => aw_style::white(theme, status),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum CardStyleParam {
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRgba,
    BorderRadius, 
    BorderWidth,
    BorderColor,
    BorderColorAlpha,
    BorderRgba, 
    HeadBackgroundColor,
    HeadBackgroundColorAlpha,
    HeadBackgroundRgba, 
    BodyBackgroundColor,
    BodyBackgroundColorAlpha,
    BodyBackgroundRgba, 
    FootBackgroundColor,
    FootBackgroundColorAlpha,
    FootBackgroundRgba,
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
    // PaddingHead,
    PaddingBody,
    PaddingFoot,
    CloseIconSize,
    StyleId,
    StyleStd,
    Show,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Card {
    type Param = CardParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            CardParam::CloseIconSize => set_t_value(&mut self.close_icon_size, value, "CardParam::CloseSize"),
            CardParam::Fill => set_t_value(&mut self.fill, value, "CardParam::Fill"),
            CardParam::Height => set_t_value(&mut self.height, value, "CardParam::Height"),
            CardParam::HeightFill => set_t_value(&mut self.height_fill, value, "CardParam::HeightFill"),
            CardParam::IsOpen => set_t_value(&mut self.is_open, value, "CardParam::IsOpen"),
            CardParam::MaxHeight => set_t_value(&mut self.max_height, value, "CardParam::MaxHeight"),
            CardParam::MaxWidth => set_t_value(&mut self.max_width, value, "CardParam::MaxWidth"),
            CardParam::Padding => set_t_value(&mut self.padding, value, "CardParam::Padding"),
            CardParam::PaddingBody => set_t_value(&mut self.padding_body, value, "CardParam::PaddingBody"),
            CardParam::PaddingFoot => set_t_value(&mut self.padding_foot, value, "CardParam::PaddingFoot"),
            CardParam::Show => set_t_value(&mut self.show, value, "CardParam::Show"),
            CardParam::StyleId => set_t_value(&mut self.style_id, value, "CardParam::StyleId"),
            CardParam::StyleStd => set_t_value(&mut self.style_std, value, "CardParam::StyleStd"),
            CardParam::Width => set_t_value(&mut self.width, value, "CardParam::Width"),
            CardParam::WidthFill => set_t_value(&mut self.width_fill, value, "CardParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for CardStyle {
    type Param = CardStyleParam;
    
    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            CardStyleParam::BackgroundColor => set_t_value(&mut self.background_color, value, "CardStyleParam::BackgroundColor"),
            CardStyleParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "CardStyleParam::BackgroundColorAlpha"),
            CardStyleParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "CardStyleParam::BackgroundRgbaColor"),
            CardStyleParam::BodyBackgroundColor => set_t_value(&mut self.body_background_color, value, "CardStyleParam::BodyBackgroundColor"),
            CardStyleParam::BodyBackgroundColorAlpha => set_t_value(&mut self.body_background_color_alpha, value, "CardStyleParam::BodyBackgroundColorAlpha"),
            CardStyleParam::BodyBackgroundRgba => set_t_value(&mut self.body_background_rgba, value, "CardStyleParam::BodyBackgroundRgbaColor"),
            CardStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "BorderColor"),
            CardStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "CardStyleParam::BorderColorAlpha"),
            CardStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "CardStyleParam::BorderRadius"),
            CardStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "CardStyleParam::BorderRgbaColor"),
            CardStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "CardStyleParam::BorderWidth"),
            CardStyleParam::FootBackgroundColor => set_t_value(&mut self.foot_background_color, value, "CardStyleParam::FootBackgroundColor"),
            CardStyleParam::FootBackgroundColorAlpha => set_t_value(&mut self.foot_background_color_alpha, value, "CardStyleParam::FootBackgroundColorAlpha"),
            CardStyleParam::FootBackgroundRgba => set_t_value(&mut self.foot_background_rgba, value, "CardStyleParam::FootBackgroundRgbaColor"),
            CardStyleParam::HeadBackgroundColor => set_t_value(&mut self.head_background_color, value, "CardStyleParam::HeadBackgroundColor"),
            CardStyleParam::HeadBackgroundColorAlpha => set_t_value(&mut self.head_background_color_alpha, value, "CardStyleParam::HeadBackgroundColorAlpha"),
            CardStyleParam::HeadBackgroundRgba => set_t_value(&mut self.head_background_rgba, value, "CardStyleParam::HeadBackgroundRgbaColor"),
        }
    }
}
