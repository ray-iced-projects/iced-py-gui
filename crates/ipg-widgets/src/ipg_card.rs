//! ipg_card
use iced::{Background, Color, Element, Length, Padding, Theme};
use iced::widget::{Column, Space, Text};
use iced_aw::card::{Card, Status};
use iced_aw::style::card;

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use ipg_types::{CardMessage, Message};
use ipg_styling::{colors::get_color, try_extract_ipg_color, try_extract_rgba_color};
use ipg_helpers::{try_extract_boolean, try_extract_string, try_extract_f64, try_extract_u64};
use super::ipg_enums::IpgWidgets;


#[derive(Debug, Clone)]
pub struct IpgCard {
    pub id: usize,
    pub parent_id: String,
    pub is_open: bool,
    
    pub button_id: Option<usize>,
    pub width: Length,
    pub height: Length,
    pub max_width: f32,
    pub max_height: f32,
    pub padding_head: Padding,
    pub padding_body: Padding,
    pub padding_foot: Padding,
    pub close_size: f32,
    pub head: String,
    pub body: String,
    pub foot: Option<String>,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgCardStyleStandard>,
    pub show: bool,
}

impl IpgCard {
    pub fn new( 
        id: usize,
        parent_id: String,
        is_open: bool,
        min_max_id: Option<usize>,
        width: Length,
        height: Length,
        max_width: f32,
        max_height: f32,
        padding_head: Padding,
        padding_body: Padding,
        padding_foot: Padding,
        close_size: f32,
        head: String,
        body: String,
        foot: Option<String>,
        style_id: Option<usize>,
        style_standard: Option<IpgCardStyleStandard>,
        show: bool,
        ) -> Self {
        Self {
            id,
            parent_id,
            is_open,
            button_id: min_max_id,
            width,
            height,
            max_width,
            max_height,
            padding_head,
            padding_body,
            padding_foot,
            close_size,
            head,
            body,
            foot,
            style_id,
            style_standard,
            show,
        }
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
    pub fn new(
        id: usize,
        background: Option<Color>, 
        border_radius: Option<f32>, 
        border_width: Option<f32>, 
        border_color: Option<Color>, 
        head_background: Option<Color>, 
        head_text_color: Option<Color>, 
        body_background: Option<Color>, 
        body_text_color: Option<Color>, 
        foot_background: Option<Color>, 
        foot_text_color: Option<Color>, 
        close_color:Option<Color>,
    ) -> Self {
        Self {
            id,
            background,
            border_radius,
            border_width,
            border_color,
            head_background, 
            head_text_color, 
            body_background, 
            body_text_color, 
            foot_background, 
            foot_text_color, 
            close_color,
        }
    }
}


pub fn construct_card<'a>(crd: &'a IpgCard,
                            style_opt: Option<&'a IpgWidgets>) 
                            -> Option<Element<'a, Message>> {

    if !crd.show {return None}
    if !crd.is_open {
        let sp: Element<CardMessage> = Space::default().into();
        let sp_mapped: Element<Message> = sp.map(move |message| Message::Card(crd.id, message));
        return Some(sp_mapped)
    }

    let style = get_card_style(style_opt);

    let head: Element<CardMessage> = Text::new(crd.head.clone())
                                                .width(Length::Fill)
                                                .into();

    let body: Element<CardMessage> = Text::new(crd.body.clone())
                                                .width(Length::Fill)
                                                .into();

    let foot_opt: String= match &crd.foot {
                                        Some(foot) => foot.clone(),
                                        None => "".to_string(),
                                    };

    let foot: Element<CardMessage> = Text::new(foot_opt.clone())
                                            .width(Length::Fill)
                                            .into();

    let body: Element<CardMessage> = Column::new().push(body).into();

    let card: Element<CardMessage> = Card::new(head, body)
                                                .foot(foot)
                                                .width(crd.width)
                                                .height(crd.height)
                                                .max_width(crd.max_width)
                                                .max_height(crd.max_height)
                                                .padding_head(crd.padding_head)
                                                .padding_body(crd.padding_body)
                                                .padding_foot(crd.padding_foot)
                                                .close_size(crd.close_size)
                                                .on_close(CardMessage::OnClose)
                                                .style(move|theme: &Theme, status|{   
                                                    get_styling(theme, status,
                                                        style.clone(),
                                                        crd.style_standard.clone(),
                                                        )  
                                                    })
                                                .into();

    Some(card.map(move |message: CardMessage| Message::Card(crd.id, message)))
    
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCardParam {
    Head,
    Body,
    Foot,
    IsOpen,
    StyleId,
    Show,
}


pub fn card_item_update(crd: &mut IpgCard,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_card_update(item);
    let name = "Card".to_string();
    match update {
        IpgCardParam::Body => {
            crd.body = try_extract_string(value, name);
        },
        IpgCardParam::Foot => {
            crd.foot = Some(try_extract_string(value, name));
        },
        IpgCardParam::Head => {
            crd.head = try_extract_string(value, name);
        },
        IpgCardParam::IsOpen => {
            crd.is_open = try_extract_boolean(value, name);
        },
        IpgCardParam::StyleId => {
            crd.style_id = Some(try_extract_u64(value, name) as usize);
        },
        IpgCardParam::Show => {
            crd.show = try_extract_boolean(value, name);
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCardStyleStandard {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    White,
}

pub fn get_card_style(style: Option<&IpgWidgets>) -> Option<IpgCardStyle>{
    match style {
        Some(IpgWidgets::IpgCardStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

pub fn get_styling(
        theme: &Theme, status: Status,
        style_opt: Option<IpgCardStyle>,
        style_standard: Option<IpgCardStyleStandard>,
        ) -> card::Style 
{
    // if no systles defined
    if style_standard.is_none() && style_opt.is_none() {
        return card::primary(theme, status)
    }

    if style_standard.is_some() {
            return get_standard_style(theme, status, style_standard)
    }


    if let Some(ipg_style) = style_opt {
        let mut style = card::Style::default();
        style.background = ipg_style.background.unwrap_or(Color::WHITE).into();
        style.border_radius = ipg_style.border_radius.unwrap_or(10.0);
        style.border_width = ipg_style.border_width.unwrap_or(1.0);
        style.border_color = ipg_style.border_color.unwrap_or([0.87, 0.87, 0.87].into());
        style.head_background = if let Some(color) = ipg_style.head_background {
            color.into()
        } else {
            Background::Color([0.87, 0.87, 0.87].into())
        };
        style.head_text_color = ipg_style.head_text_color.unwrap_or(Color::BLACK);
        style.body_background = if let Some(color) = ipg_style.body_background {
            color.into()
        } else {
            Color::TRANSPARENT.into()
        };
        style.body_text_color = ipg_style.body_text_color.unwrap_or(Color::BLACK);
        style.foot_background = if let Some(color) = ipg_style.foot_background {
            color.into()
        } else {
            Color::TRANSPARENT.into()
        };
        style.foot_text_color = ipg_style.foot_text_color.unwrap_or(Color::BLACK);
        style.close_color = ipg_style.close_color.unwrap_or(Color::BLACK);

        style
    } else {
        card::Style::default()
    }

}

fn get_standard_style(
        theme: &Theme, status: Status, 
        std_style: Option<IpgCardStyleStandard>)
        -> card::Style {

    match std_style.unwrap() {
        IpgCardStyleStandard::Primary => card::primary(theme, status),
        IpgCardStyleStandard::Secondary => card::secondary(theme, status),
        IpgCardStyleStandard::Success => card::success(theme, status),
        IpgCardStyleStandard::Danger => card::danger(theme, status),
        IpgCardStyleStandard::Warning => card::warning(theme, status),
        IpgCardStyleStandard::Info => card::info(theme, status),
        IpgCardStyleStandard::Light => card::light(theme, status),
        IpgCardStyleStandard::Dark => card::danger(theme, status),
        IpgCardStyleStandard::White => card::white(theme, status),
            }
}

pub fn try_extract_card_update(update_obj: &PyObject) -> IpgCardParam {
    Python::attach(|py| {
        let res = update_obj.extract::<IpgCardParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Card update extraction failed."),
        }
    })
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

pub fn card_style_update(style: &mut IpgCardStyle,
                        item: &PyObject,
                        value: &PyObject,) {

    let update = try_extract_card_style_update(item);
    let name = "Card Style".to_string();
    
    match update {
        IpgCardStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::BackgroundRgbaColor => {
            style.background = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::BorderIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.border_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::BorderRadius => {
            style.border_radius = Some(try_extract_f64(value, name) as f32);
        },
        IpgCardStyleParam::BorderWidth => {
            style.border_width = Some(try_extract_f64(value, name) as f32);
        },
        IpgCardStyleParam::HeadBackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.head_background = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::HeadBackgroundRgbaColor => {
             style.head_background = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::HeadTextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.head_text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::HeadTextRgbaColor => {
             style.head_text_color= Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::BodyBackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.body_background = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::BodyBackgroundRgbaColor => {
             style.body_background = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::BodyTextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.body_text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::BodyTextRgbaColor => {
             style.body_text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::FootBackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.foot_background = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::FootBackgroundRgbaColor => {
             style.foot_background = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::FootTextIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.foot_text_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::FootTextRgbaColor => {
             style.foot_text_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
        IpgCardStyleParam::CloseIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.close_color = get_color(None, Some(color), 1.0, false);
        },
        IpgCardStyleParam::CloseRgbaColor => {
             style.close_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

fn try_extract_card_style_update(update_obj: &PyObject) -> IpgCardStyleParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgCardStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Card style parameter update extraction failed"),
        }
    })
}
