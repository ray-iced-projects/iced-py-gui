//! ipg_scrollable

use crate::IpgState;
use crate::access_callbacks;
use crate::access_user_data1;
use crate::app::Message;
use crate::state::IpgWidgets;
use crate::widgets::ipg_container::IpgContainerStyleStd;
use crate::widgets::styling::{apply_border_overrides, apply_shadow_overrides_xy};
use crate::widgets::widget_param_update::set_opt_bool;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,
    set_opt_f32, set_opt_f32_array_2, set_opt_iced_color,
    set_opt_usize, set_opt_vec_f32, set_iced_color_from_rgba,
    set_height, set_width,
};

use std::collections::HashMap;
use iced::widget::scrollable::{self, Direction, Scrollable, 
    Viewport, Anchor, Scrollbar};
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
    pub both_scrollers: Option<bool>,
    pub scroller_x_id: Option<usize>,
    pub scroller_y_id: Option<usize>,
    pub style_id: Option<usize>,
}

impl IpgScrollable {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        content: Vec<Element<'a, Message>>,
        widgets: &'a HashMap<usize, IpgWidgets>,
        ) -> Element<'a, Message> {

        let ipg_scroll_style  = self.lookup(widgets, self.style_id)
            .and_then(IpgWidgets::as_scrollable_style).cloned();

        let content: Element<'a, Message> = Column::with_children(content).into();

        let sb_x_opt = self.lookup(widgets, self.scroller_x_id);
        let sb_y_opt = self.lookup(widgets, self.scroller_y_id);
        
        let direction = 
            match (sb_x_opt.is_some(), sb_y_opt.is_some()) {
                (true, true) => {
                    let ipg_sb_x = sb_x_opt.and_then(IpgWidgets::as_scroller).cloned().unwrap_or_default();
                    let ipg_sb_y = sb_y_opt.and_then(IpgWidgets::as_scroller).cloned().unwrap_or_default();
                    Direction::Both { vertical: ipg_sb_y.construct(), horizontal: ipg_sb_x.construct() }
                },
                (true, false) => {
                    let ipg_sb = sb_x_opt.and_then(IpgWidgets::as_scroller).cloned().unwrap_or_default();
                    if self.both_scrollers == Some(true) {
                        Direction::Both { vertical: Scrollbar::default(), horizontal: ipg_sb.construct() }
                    } else { Direction::Horizontal(ipg_sb.construct()) }
                },
                (false, true) => {
                    let ipg_sb = sb_y_opt.and_then(IpgWidgets::as_scroller).cloned().unwrap_or_default();
                    if self.both_scrollers == Some(true) {
                            Direction::Both { vertical: ipg_sb.construct(), horizontal: Scrollbar::default() }
                        } else { Direction::Vertical(ipg_sb.construct()) }
                },
                (false, false) => {
                    if self.both_scrollers == Some(true) {
                        Direction::Both { vertical: Scrollbar::default(), horizontal: Scrollbar::default() }
                    } else { Direction::Vertical(Scrollbar::default()) }
                },
            };

        Scrollable::with_direction(content, direction)
            .width(self.width)
            .height(self.height)
            .on_scroll(move|vp| 
                Message::Scrolled(vp, self.id))
            .style(move|theme, status| {
                if let Some(ipg_style) = &ipg_scroll_style {
                    ipg_style.set_style(theme, status, widgets)
                } else {
                    scrollable::default(theme, status)
                }
                
            })
            .into()
        
    }
}

/// The appearance of a scrollable.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IpgScrollableStyle {
    pub container_style_id: Option<usize>,
    pub container_style_std: Option<IpgContainerStyleStd>,
    pub vertical_rail_style_id: Option<usize>,
    pub horizontal_rail_style_id: Option<usize>,
    pub auto_scroll_style_id: Option<usize>,
    pub gap_color: Option<Color>,
}

impl IpgScrollableStyle {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn set_style(
        &self, 
        theme: &Theme, 
        status: scrollable::Status,
        widgets: &HashMap<usize, IpgWidgets>,
    ) -> scrollable::Style {
        
        let mut style = scrollable::default(theme, status);

        let ipg_container_style = self.lookup(widgets, self.container_style_id)
            .and_then(IpgWidgets::as_container_style).cloned();

        let ipg_h_rail_style = self.lookup(widgets, self.horizontal_rail_style_id)
            .and_then(IpgWidgets::as_rail_style).cloned();
        
        let ipg_v_rail_style = self.lookup(widgets, self.vertical_rail_style_id)
            .and_then(IpgWidgets::as_rail_style).cloned();
        
        let ipg_auto_scroll_style = self.lookup(widgets, self.auto_scroll_style_id)
            .and_then(IpgWidgets::as_auto_scroll_style).cloned();

        // Need default since iced doesn't default each one individually
        let mut def_h_rail = style.horizontal_rail;
        let mut def_v_rail = style.vertical_rail;
        let mut def_auto_scroll = style.auto_scroll;
        
        // container style via the container_style_id and std atyle
        if let Some(ipg_style) = &ipg_container_style {
            let ipg_std_style = 
                if let Some(style_std) = self.container_style_std.clone() {
                    Some(style_std)
                } else { None };
                    style.container = ipg_style.to_iced(theme, &ipg_std_style);
            }
        
        style.horizontal_rail = if let Some(rail) = &ipg_h_rail_style {
            rail.to_iced(&mut def_h_rail)
        } else { def_h_rail };
        
        style.vertical_rail = if let Some(rail) = &ipg_v_rail_style {
            rail.to_iced(&mut def_v_rail)
        } else { def_v_rail };
        
        if let Some(c) = self.gap_color {
            style.gap = Some(c.into());
        }
        
        style.auto_scroll = if let Some(auto) = &ipg_auto_scroll_style {
            auto.to_iced(&mut def_auto_scroll)
        } else { def_auto_scroll };
        
        style
    }
}

#[derive(Debug, Default, Clone)]
pub struct IpgScroller {
    pub id: usize,
    pub width: Option<f32>,
    pub margin: Option<f32>,
    pub scroller_width: Option<f32>,
    pub spacing: Option<f32>,
    pub anchor: Option<IpgAnchor>,
    pub hidden: Option<bool>,
}

impl IpgScroller {
    pub fn construct(&self) -> Scrollbar {

        let mut sb = Scrollbar::default();

        sb = if let Some(w) = self.width {
                sb.width(w)
            } else { sb };

        sb = if let Some(m) = self.margin {
                sb.margin(m)
            } else { sb };

        sb = if let Some(sw) = self.scroller_width {
                sb.scroller_width(sw)
            } else { sb };

        sb = if let Some(an) = &self.anchor {
                sb.anchor(an.to_iced())
            } else { sb };

        sb = if let Some(sp) = self.spacing {
                sb.spacing(sp)
            } else { sb };

        if self.hidden == Some(true) {
            sb = sb.width(0).scroller_width(0);
        }

        sb
    }
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
                Err(err) => panic!(
                    "Unable to extract python {} {}", 
                    "IpgAnchor",
                    err),
            }
        })  
    }
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
    Height,
    ScrollerXId,
    ScrollerYId,
    StyleId,
    Width,
}



#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgScrollerParam {
    Anchor,
    Hidden,
    Margin,
    ScrollerWidth,
    Spacing,
    Width,
}


#[derive(Debug, Clone, PartialEq)]
pub struct IpgRailStyle { 
    pub id: usize,
    pub background: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
    pub scroller_background: Option<Color>,
    pub scroller_border_color: Option<Color>,
    pub scroller_border_width: Option<f32>,
    pub scroller_border_radius: Option<Vec<f32>>,
    
}

impl IpgRailStyle {
    fn to_iced(&self, rail: &mut scrollable::Rail) -> scrollable::Rail {
        
        if let Some(color) = self.background {
            rail.background = Some(color.into());
        }

        apply_border_overrides(
            &mut rail.border, self.border_color,
            &self.border_radius, self.border_width, self.type_name(),
        );

        if let Some(color) = self.scroller_background {
            rail.scroller.background = color.into();
        }

        apply_border_overrides(
            &mut rail.scroller.border, self.scroller_border_color,
            &self.scroller_border_radius, self.scroller_border_width, self.type_name(),
        );

        *rail
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
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

impl IpgAutoScrollStyle {
    fn to_iced(&self, auto: &mut scrollable::AutoScroll) -> scrollable::AutoScroll {

        if let Some(color) = self.background {
            auto.background = color.into();
        }

        apply_border_overrides(
            &mut auto.border, self.border_color,
            &self.border_radius, self.border_width, self.type_name(),
        );

        apply_shadow_overrides_xy(
            &mut auto.shadow, self.shadow_color,
            self.shadow_offset, self.shadow_blur_radius,
        );

        if let Some(color) = self.shadow_icon_color {
            auto.icon = color;
        }

        *auto
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgScrollableStyleParam { 
    AutoScrollStyleId,
    ContainerStyleId,
    ContainerStyleStd,
    GapColor,
    GapRgba,
    HorizontalRailStyleId,
    VerticalRailStyleId,
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
            IpgScrollableParam::Height => set_height(&mut self.height, value, name),
            IpgScrollableParam::ScrollerXId => set_opt_usize(&mut self.scroller_x_id, value, name),
            IpgScrollableParam::ScrollerYId => set_opt_usize(&mut self.scroller_y_id, value, name),
            IpgScrollableParam::StyleId => set_opt_usize(&mut self.style_id, value, name),
            IpgScrollableParam::Width => set_width(&mut self.width, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgScrollableStyle {
    type Param = IpgScrollableStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgScrollableStyleParam::ContainerStyleId => set_opt_usize(&mut self.container_style_id, value, name),
            IpgScrollableStyleParam::ContainerStyleStd => todo!(),
            IpgScrollableStyleParam::VerticalRailStyleId => set_opt_usize(&mut self.vertical_rail_style_id, value, name),
            IpgScrollableStyleParam::HorizontalRailStyleId => set_opt_usize(&mut self.horizontal_rail_style_id, value, name),
            IpgScrollableStyleParam::AutoScrollStyleId => set_opt_usize(&mut self.auto_scroll_style_id, value, name),
            IpgScrollableStyleParam::GapColor => set_opt_iced_color(&mut self.gap_color, value, name),
            IpgScrollableStyleParam::GapRgba => set_iced_color_from_rgba(&mut self.gap_color, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgScroller {
    type Param = IpgScrollerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgScrollerParam::Anchor => self.anchor = IpgAnchor::extract(value),
            IpgScrollerParam::Hidden => set_opt_bool(&mut self.hidden, value, name),
            IpgScrollerParam::Margin => set_opt_f32(&mut self.margin, value, name),
            IpgScrollerParam::ScrollerWidth => set_opt_f32(&mut self.scroller_width, value, name),
            IpgScrollerParam::Spacing => set_opt_f32(&mut self.spacing, value, name),
            IpgScrollerParam::Width => set_opt_f32(&mut self.width, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgRailStyle {
    type Param = IpgRailStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgRailStyleParam::BackgroundColor => set_opt_iced_color(&mut self.background, value, name),
            IpgRailStyleParam::BackgroundRgba => set_iced_color_from_rgba(&mut self.background, value, name),
            IpgRailStyleParam::BorderColor => set_opt_iced_color(&mut self.border_color, value, name),
            IpgRailStyleParam::BorderRgba => set_iced_color_from_rgba(&mut self.border_color, value, name),
            IpgRailStyleParam::BorderWidth => set_opt_f32(&mut self.border_width, value, name),
            IpgRailStyleParam::BorderRadius => set_opt_vec_f32(&mut self.border_radius, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgAutoScrollStyle {
    type Param = IpgAutoScrollStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgAutoScrollStyleParam::BackgroundColor => set_opt_iced_color(&mut self.background, value, name),
            IpgAutoScrollStyleParam::BackgroundRgba => set_iced_color_from_rgba(&mut self.background, value, name),
            IpgAutoScrollStyleParam::BorderColor => set_opt_iced_color(&mut self.border_color, value, name),
            IpgAutoScrollStyleParam::BorderRgba => set_iced_color_from_rgba(&mut self.border_color, value, name),
            IpgAutoScrollStyleParam::BorderWidth => set_opt_f32(&mut self.border_width, value, name),
            IpgAutoScrollStyleParam::BorderRadius => set_opt_vec_f32(&mut self.border_radius, value, name),
            IpgAutoScrollStyleParam::ShadowColor => set_opt_iced_color(&mut self.shadow_color, value, name),
            IpgAutoScrollStyleParam::ShadowRgba => set_iced_color_from_rgba(&mut self.shadow_color, value, name),
            IpgAutoScrollStyleParam::ShadowOffset => set_opt_f32_array_2(&mut self.shadow_offset, value, name),
            IpgAutoScrollStyleParam::ShadowBlurRadius => set_opt_f32(&mut self.shadow_blur_radius, value, name),
            IpgAutoScrollStyleParam::ShadowIconColor => set_opt_iced_color(&mut self.shadow_icon_color, value, name),
            IpgAutoScrollStyleParam::ShadowIconRgba => set_iced_color_from_rgba(&mut self.shadow_icon_color, value, name),
        }
    }
}

