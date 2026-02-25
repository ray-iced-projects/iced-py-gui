//! ipg_scrollable

use crate::IpgState;
use crate::access_callbacks;
use crate::access_user_data1;
use crate::app::Message;
use crate::graphics::colors::IpgColor;
use crate::state::IpgWidgets;
use crate::widgets::ipg_container;
use crate::widgets::widget_param_update::WidgetParamUpdate;

use std::collections::HashMap;
use iced::widget::scrollable;
use iced::widget::scrollable::Anchor;
use iced::widget::scrollable::Scrollbar;
use iced::widget::scrollable::{Direction, Scrollable, Viewport, Status, Style};
use iced::Rectangle;
use iced::{Color, Element, Length, Theme};
use iced::widget::Column;

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgScrollable {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub scrollbar_x_id: Option<usize>,
    pub scrollbar_y_id: Option<usize>,
    pub style_id: Option<usize>,
    pub container_style_id: Option<usize>,
    // internal use
    pub scroll_y_pos: Option<f32>,
    pub scroll_x_pos: Option<f32>,
    pub bounds: Rectangle,
    pub content_bounds: Rectangle,
}

#[derive(Debug, Default, Clone)]
pub struct IpgScrollbar {
    pub id: usize,
    pub x_direction: bool,
    pub y_direction: bool,
    pub width: Option<f32>,
    pub margin: Option<f32>,
    pub scroller_width: Option<f32>,
    pub spacing: Option<f32>,
    pub alignment: Option<IpgAnchor>,
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
    cont_style_opt: Option<&IpgWidgets>,
    style_opt: Option<&IpgWidgets> ) 
    -> Element<'a, Message> {
    
    let ipg_style_opt = get_scroll_style_widget(style_opt);
    let ipg_cont_style_opt = ipg_container::get_cont_style(cont_style_opt);

    let content: Element<'a, Message> = Column::with_children(content).into();

    let direction = 
        match (sb_x_opt.is_some(), sb_y_opt.is_some()) {
            (true, true) => {
                let ipg_sb = match_scrollbar_widget(sb_x_opt);
                let sb_x = get_scrollbar(ipg_sb);

                let ipg_sb = match_scrollbar_widget(sb_y_opt);
                let sb_y = get_scrollbar(ipg_sb);

                Direction::Both { vertical: sb_y, horizontal: sb_x }
            },
            (true, false) => {
                let ipg_sb = match_scrollbar_widget(sb_x_opt);
                let sb_x = get_scrollbar(ipg_sb);
                Direction::Horizontal(sb_x)
            },
            (false, true) => {
                let ipg_sb = match_scrollbar_widget(sb_y_opt);
                let sb_y = get_scrollbar(ipg_sb);
                Direction::Vertical(sb_y)
            },
            (false, false) => todo!(),
        };

    Scrollable::with_direction(content, direction)
        .width(ipg_scroll.width)
        .height(ipg_scroll.height)
        .on_scroll(move|vp| 
            Message::Scrolled(vp, ipg_scroll.id))
        .style(move|theme, status| {
            get_styling(theme, status, &ipg_style_opt, &ipg_cont_style_opt)
        })
        .into()
    
}

fn match_scrollbar_widget(wid: Option<&IpgWidgets>) -> IpgScrollbar {
    if let Some(w) = wid {
        match w {
            IpgWidgets::IpgScrollbar(bar) => bar.clone(),
            _ => IpgScrollbar::default()
        }
    } else {
        return IpgScrollbar::default()
    }
    
}

fn get_scrollbar(ipg_sb: IpgScrollbar) -> Scrollbar {
    
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
    BarAlignment,
    BarMargin,
    BarWidth,
    Height,
    ScrollerWidth,
    ScrollXTo,
    ScrollYTo,
    Spacing,
    Width,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgScrollbarParam {
    Alignment,
    Margin,
    ScrollerWidth,
    Spacing,
    Width,
    XDirection,
    YDirection,
}

fn get_styling(theme: &Theme, status: Status,
                style_opt: &Option<IpgScrollableStyle>,
                cont_style_opt: &Option<ipg_container::IpgContainerStyle>
                ) -> Style 
{

    let mut style = scrollable::default(theme, status);
    // container style via the container_style_id
    style.container = ipg_container::get_styling(theme, cont_style_opt);
    
            
        
    
    
    


    style
    
}

#[derive(Debug, Clone)]
pub struct IpgScrollableStyle {
    pub id: usize,
    pub container_style_id: Option<usize>,
    pub vertical_rail_id: Option<usize>,
    pub horizontal_rail_id: Option<usize>,
    pub gap_background_color: Option<Color>,
}

impl IpgScrollableStyle {
    /// Apply user-defined style overrides to an existing iced checkbox::Style
    pub fn apply_to(&self, style: &mut scrollable::Style, status: scrollable::Status) {

    }
}

pub fn get_scroll_style_widget(style: Option<&IpgWidgets>) -> Option<IpgScrollableStyle> {
    style.and_then(|s| match s {
        IpgWidgets::IpgScrollableStyle(st) => Some(st.clone()),
        _ => None,
    })
}

fn get_scrollbar_widget(sb_opt: Option<&IpgWidgets>) -> Option<IpgScrollbar> {
    sb_opt.and_then(|s| match s {
        IpgWidgets::IpgScrollbar(st) => Some(st.clone()),
        _ => None,
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpgRailStyle { 
    pub id: usize,
    pub background: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpgScrollerStyle {
    pub id: usize,
    pub background: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpgAutoScrollStyle {
    pub id: usize,
    pub background: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
    pub shadow_color: Option<Color>,
    pub shadow_offset: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,
    pub shadow_icon_color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgScrollableStyleParam {
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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRailStyleParam { 
    BackgroundColor,
    BackgroundRgba,
    BorderColor,
    BorderRgba,
    BorderWidth,
    BorderRadius,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgAutoScrollStyleParam {
    BackgroundColor,
    BackgroundRgba,
    BorderColor,
    BorderRgba,
    BorderWidth,
    BorderRadius,
    ShadowColor,
    ShadowRgba,
    ShadowOffset,
    ShadowBlurRadius,
    ShadowIconColor,
    ShadowIconRgba,
}
// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgScrollable {
    type Param = IpgScrollableParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgScrollableParam::BarAlignment => todo!(),
            IpgScrollableParam::BarMargin => todo!(),
            IpgScrollableParam::BarWidth => todo!(),
            IpgScrollableParam::Height => todo!(),
            IpgScrollableParam::ScrollerWidth => todo!(),
            IpgScrollableParam::ScrollXTo => todo!(),
            IpgScrollableParam::ScrollYTo => todo!(),
            IpgScrollableParam::Spacing => todo!(),
            IpgScrollableParam::Width => todo!(),
        }
    }
}

impl WidgetParamUpdate for IpgScrollableStyle {
    type Param = IpgScrollableStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgScrollableStyleParam::ScrollbarIpgColor => todo!(),
            IpgScrollableStyleParam::ScrollbarRgbaColor => todo!(),
            IpgScrollableStyleParam::ScrollbarBorderRadius => todo!(),
            IpgScrollableStyleParam::ScrollbarBorderWidth => todo!(),
            IpgScrollableStyleParam::ScrollbarBorderIpgColor => todo!(),
            IpgScrollableStyleParam::ScrollbarBorderRgbaColor => todo!(),
            IpgScrollableStyleParam::ScrollerIpgColor => todo!(),
            IpgScrollableStyleParam::ScrollerRgbaColor => todo!(),
            IpgScrollableStyleParam::ScrollerIpgColorHovered => todo!(),
            IpgScrollableStyleParam::ScrollerRgbaColorHovered => todo!(),
            IpgScrollableStyleParam::ScrollerIpgColorDragged => todo!(),
            IpgScrollableStyleParam::ScrollerRgbaColorDragged => todo!(),
        }
    }
}

impl WidgetParamUpdate for IpgScrollbar {
    type Param = IpgScrollbarParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgScrollbarParam::Alignment => todo!(),
            IpgScrollbarParam::Margin => todo!(),
            IpgScrollbarParam::ScrollerWidth => todo!(),
            IpgScrollbarParam::Spacing => todo!(),
            IpgScrollbarParam::Width => todo!(),
            IpgScrollbarParam::XDirection => todo!(),
            IpgScrollbarParam::YDirection => todo!(),
        }
    }
}

impl WidgetParamUpdate for IpgRailStyle {
    type Param = IpgRailStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgRailStyleParam::BackgroundColor => todo!(),
            IpgRailStyleParam::BackgroundRgba => todo!(),
            IpgRailStyleParam::BorderColor => todo!(),
            IpgRailStyleParam::BorderRgba => todo!(),
            IpgRailStyleParam::BorderWidth => todo!(),
            IpgRailStyleParam::BorderRadius => todo!(),
        }
    }
}

impl WidgetParamUpdate for IpgAutoScrollStyle {
    type Param = IpgAutoScrollStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgAutoScrollStyleParam::BackgroundColor => todo!(),
            IpgAutoScrollStyleParam::BackgroundRgba => todo!(),
            IpgAutoScrollStyleParam::BorderColor => todo!(),
            IpgAutoScrollStyleParam::BorderRgba => todo!(),
            IpgAutoScrollStyleParam::BorderWidth => todo!(),
            IpgAutoScrollStyleParam::BorderRadius => todo!(),
            IpgAutoScrollStyleParam::ShadowColor => todo!(),
            IpgAutoScrollStyleParam::ShadowRgba => todo!(),
            IpgAutoScrollStyleParam::ShadowOffset => todo!(),
            IpgAutoScrollStyleParam::ShadowBlurRadius => todo!(),
            IpgAutoScrollStyleParam::ShadowIconColor => todo!(),
            IpgAutoScrollStyleParam::ShadowIconRgba => todo!(),
        }
    }
}
