//! ipg_scrollable
use std::collections::HashMap;



use iced::widget::container;
use iced::widget::scrollable;
use iced::widget::scrollable::Anchor;
use iced::widget::scrollable::Rail;
use iced::widget::scrollable::Scrollbar;
use iced::widget::scrollable::Scroller;
use iced::widget::scrollable::{Direction, Scrollable, Viewport, Status, Style};
use iced::Rectangle;
use iced::{Border, Color, Element, Length, Shadow, Vector, Theme};
use iced::widget::Column;

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};

use crate::access_callbacks;
use crate::access_user_data1;
use crate::app::Message;
use crate::state::IpgWidgets;
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgScrollable {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub scrollbar_x_id: Option<usize>,
    pub scrollbar_y_id: Option<usize>,
    pub style_id: Option<usize>,
    pub scroll_y_pos: Option<f32>,
    pub scroll_x_pos: Option<f32>,
    pub bounds: Rectangle,
    pub content_bounds: Rectangle,
}

#[derive(Debug, Clone)]
pub struct IpgScrollBar {
    pub id: usize,
    pub width: Option<f32>,
    pub margin: Option<f32>,
    pub scroller_width: Option<f32>,
    pub spacing: Option<f32>,
    pub alignment: Option<IpgAnchor>,
}

#[derive(Debug, Clone)]
pub struct IpgScrollableStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Vec<f32>,
    pub border_width: f32,
    pub shadow_color: Option<Color>,
    pub shadow_offset_x: f32,
    pub shadow_offset_y: f32,
    pub shadow_blur_radius: f32,
    pub text_color: Option<Color>,
    // above container style
    pub scrollbar_color: Option<Color>,
    pub scrollbar_border_radius: Vec<f32>,
    pub scrollbar_border_width: f32,
    pub scrollbar_border_color: Option<Color>,
    pub scroller_color: Option<Color>,
    pub scroller_color_hovered: Option<Color>,
    pub scroller_color_dragged: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgAnchor {
    Start,
    End,
}

impl IpgAnchor {
    fn to_iced(&self) -> Anchor {
        match self {
            IpgAnchor::Start => Anchor::Start,
            IpgAnchor::End => Anchor::End,
        }
    }

    pub fn extract(value: &PyObject) -> Option<IpgAnchor> {
        Python::attach(|py| {
            let res = value.extract::<IpgAnchor>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => panic!("Unable to extract python IpgAnchor"),
            }
        })  
    }
}

pub fn construct_scrollable<'a>(
    ipg_scroll: &'a IpgScrollable, 
    content: Vec<Element<'a, Message>>,
    sb_x_opt: Option<&IpgWidgets>,
    sb_y_opt: Option<&IpgWidgets>,
    style_opt: Option<&IpgWidgets> ) 
    -> Element<'a, Message> {
    
    let style = get_scroll_style(style_opt);

    let content: Element<'a, Message> = Column::with_children(content).into();

    let direction = 
        match (sb_x_opt.is_some(), sp_y_opt.is_some()) {
            (true, true) => {
                let ipg_sb = sb_x_opt.unwrap();
                let sb_x = get_scrollbar(ipg_sb);
                let ipg_sb = sb_y_opt.unwrap();
                let sb_y = get_scrollbar(ipg_sb);
                Direction::Both { vertical: sb_y, horizontal: sb_x }
            },
            (true, false) => {
                let ipg_sb = ipg_scrollbar_x.unwrap();
                let sb_x = get_scrollbar(ipg_sb);
                Direction::Horizontal(sb_x)
            },
            (false, true) => {
                let ipg_sb = ipg_scrollbar_y.unwrap();
                let sb_y = get_scrollbar(ipg_sb);
                Direction::Vertical(sb_y)
            },
            (false, false) => todo!(),
        };

    Scrollable::with_direction(content, direction)
        .width(ipg_scroll.width)
        .height(ipg_scroll.height)
        .on_scroll(move|vp| 
            Message::Scrolled(vp, scroll.id))
        .style(move|theme, status| {
            get_styling(theme, status, style.clone())
        })
        .into()
    
}

fn get_scrollbar(ipg_sb: IpgScrollBar) -> Scrollbar {
    
    let sb = Scrollbar::default();
    if let Some(an) = ipg_sb.alignment {
        sb.anchor(an.to_iced());
    }
    if let Some(w) = ipg_sb.width {
        sb.width(w);
    }
    if let Some(m) = ipg_sb.margin {
        sb.margin(m);
    }
    if let Some(sw) = ipg_sb.scroller_width {
        sb.scroller_width(sw);
    }
    if let Some(sp) = ipg_sb.spacing {
        sb.spacing(sp);
    }
    sb
}

pub fn scrollable_callback(_state: &mut IpgState, id: usize, vp: Viewport) {
    let mut hmap = HashMap::new();
    hmap.insert("abs_x".to_string(), vp.absolute_offset().x);
    hmap.insert("abs_y".to_string(), vp.absolute_offset().y);
    hmap.insert("rel_x".to_string(), vp.relative_offset().x);
    hmap.insert("rel_y".to_string(), vp.relative_offset().y);
    hmap.insert("rev_x".to_string(), vp.absolute_offset_reversed().x);
    hmap.insert("rev_y".to_string(), vp.absolute_offset_reversed().y);
    
    process_callback(id, "on_scroll".to_string(), hmap);
}


pub fn process_callback(id: usize, 
                        event_name: String, 
                        hmap: HashMap<String, f32>) 
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
            if let Err(err) = callback.call1(py, (id, hmap, user_data)) {
                panic!("Scollable callback error: {err}");
            }
        });
        drop(ud1); // Drop ud1 before processing ud2
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Check user data from ud2
    // let ud2 = access_user_data2();
    // if let Some(user_data) = ud2.user_data.get(&id) {
    //     Python::attach(|py| {
    //         if let Err(err) = callback.call1(py, (id, hmap, user_data)) {
    //             panic!("Scrollable callback error: {err}");
    //         }
    //     });
    //     drop(ud2); // Drop ud2 after processing
    //     return;
    // }
    // drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with the id and hmap
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, hmap)) {
            panic!("Scollable callback error: {err}");
        }
    });

}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgScrollableParam {
    Width,
    Height,
    HBarWidth,
    HBarMargin,
    HScrollerWidth,
    HSpacing,
    HBarAlignment,
    VBarWidth,
    VBarMargin,
    VScrollerWidth,
    VSpacing,
    VBarAlignment,
    ScrollXTo,
    ScrollYTo,
}

fn get_styling(theme: &Theme, status: Status,
                style_opt: Option<IpgScrollableStyle>,
                ) -> Style 
{

    if style_opt.is_none() {
        return scrollable::default(theme, status);
    }

    let style = style_opt.unwrap();

    let background_color = if style.background_color.is_some() {
        style.background_color.unwrap()
    } else {
        Color::TRANSPARENT
    };

    let mut border = Border::default();
    let mut shadow = Shadow::default();

    if style.border_color.is_some() {
        border.color = style.border_color.unwrap();
    }

    border.radius = get_radius(style.border_radius.clone(), "Container".to_string());
    
    border.width = style.border_width;
    
    if style.shadow_color.is_some() {
        shadow.color = style.shadow_color.unwrap();
        shadow.blur_radius = style.shadow_blur_radius;
        shadow.offset = Vector{ x: style.shadow_offset_x, y: style.shadow_offset_y }
    }

    let container_style = container::Style {
            background: Some(background_color.into()),
            border,
            shadow,
            text_color: style.text_color,
        };

    let palette = theme.extended_palette();
    
    let scrollbar_color = if style.scrollbar_color.is_some() {
        style.scrollbar_color.unwrap().into()
    } else {
        palette.background.weak.color.into()
    };

    let border_radius = get_radius(style.scrollbar_border_radius.clone(), "Scrollable".to_string());
    let border_color = if style.scrollbar_border_color.is_some() {
        style.scrollbar_border_color.unwrap()
    } else {
        Color::TRANSPARENT
    };
    let border = Border{ color: border_color, width: style.border_width, radius: border_radius };

    let scroller_color = if style.scroller_color.is_some() {
        style.scroller_color.unwrap()
    } else {
        palette.background.strong.color
    };
    
    let scroller_color_hovered = if style.scroller_color_hovered.is_some() {
        style.scroller_color_hovered.unwrap()
    } else {
        palette.primary.strong.color
    };

    let scroller_color_dragged = if style.scroller_color_dragged.is_some() {
        style.scroller_color_dragged.unwrap()
    } else {
        palette.primary.base.color
    };

    let scrollbar = Rail {
        background: Some(scrollbar_color),
        border,
        scroller: Scroller {
            color: scroller_color,
            border,
        },
    };

    match status {
        Status::Active => Style {
            container: container_style,
            vertical_rail: scrollbar,
            horizontal_rail: scrollbar,
            gap: None,
        },
        Status::Hovered {
            is_horizontal_scrollbar_hovered,
            is_vertical_scrollbar_hovered,
        } => {
            let hovered_scrollbar = Rail {
                scroller: Scroller {
                    color: scroller_color_hovered,
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container_style,
                vertical_rail: if is_vertical_scrollbar_hovered {
                    hovered_scrollbar
                } else {
                    scrollbar
                },
                horizontal_rail: if is_horizontal_scrollbar_hovered {
                    hovered_scrollbar
                } else {
                    scrollbar
                },
                gap: None,
            }
        }
        Status::Dragged {
            is_horizontal_scrollbar_dragged,
            is_vertical_scrollbar_dragged,
        } => {
            let dragged_scrollbar = Rail {
                scroller: Scroller {
                    color: scroller_color_dragged,
                    ..scrollbar.scroller
                },
                ..scrollbar
            };

            Style {
                container: container_style,
                vertical_rail: if is_vertical_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                horizontal_rail: if is_horizontal_scrollbar_dragged {
                    dragged_scrollbar
                } else {
                    scrollbar
                },
                gap: None,
            }
        }
    }
    
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgScrollableStyleParam {
    BackgroundIpgColor,
    BackgroundRbgaColor,
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
    // above container style
    ScrollbarIpgColor,
    ScrollbarRgbaColor,
    ScrollbarBorderRadius,
    ScrollbarBorderWidth,
    ScrollbarBorderIpgColor,
    ScrollbarBorderRgbaColor,
    ScrollerIpgColor,
    ScrollerRgbaColor,
    ScrollerIpgColorHovered,
    ScrollerRgbaColorHovered,
    ScrollerIpgColorDragged,
    ScrollerRgbaColorDragged,
}

fn get_scroll_style(style: Option<&IpgWidgets>) -> Option<IpgScrollableStyle>{
    match style {
        Some(IpgWidgets::IpgScrollableStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

fn get_scrollbar(sb_opt: Option<&IpgWidgets>) -> Option<IpgScrollBar> {
    match style {
        Some(IpgWidgets::IpgScrollBar(sb)) => {
            Some(sb.clone())
        }
        _ => None,
    }
}


