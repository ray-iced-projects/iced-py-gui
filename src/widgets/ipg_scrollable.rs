//! ipg_scrollable

use crate::IpgState;
use crate::access_callbacks;
use crate::access_user_data1;
use crate::app::Message;
use crate::state::Widgets;
use crate::widgets::ipg_container::ContainerStyleStd;
use crate::widgets::styling::{apply_border_overrides, apply_shadow_overrides_xy};
use crate::widgets::widget_param_update::set_lengths_fill;
use crate::widgets::widget_param_update::set_opt_bool;
use crate::widgets::widget_param_update::set_opt_iced_color_from_rgba;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,
    set_opt_f32, set_opt_f32_array_2, set_opt_iced_color,
    set_opt_usize, set_opt_vec_f32, 
    set_height, set_width,
};

use std::collections::HashMap;
use iced::widget::scrollable;
use iced::{Element, Length, Theme};
use iced::widget::Column;

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Scrollable {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub both_scrollers: Option<bool>,
    pub scroller_x_id: Option<usize>,
    pub scroller_y_id: Option<usize>,
    pub style_id: Option<usize>,
}

impl Scrollable {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        content: Vec<Element<'a, Message>>,
        widgets: &'a HashMap<usize, Widgets>,
        ) -> Element<'a, Message> {

        let ipg_scroll_style  = self.lookup(widgets, self.style_id)
            .and_then(Widgets::as_scrollable_style).cloned();

        let content: Element<'a, Message> = Column::with_children(content).into();

        let sb_x_opt = self.lookup(widgets, self.scroller_x_id);
        let sb_y_opt = self.lookup(widgets, self.scroller_y_id);
        
        let direction = 
            match (sb_x_opt.is_some(), sb_y_opt.is_some()) {
                (true, true) => {
                    let ipg_sb_x = sb_x_opt.and_then(Widgets::as_scroller).cloned().unwrap_or_default();
                    let ipg_sb_y = sb_y_opt.and_then(Widgets::as_scroller).cloned().unwrap_or_default();
                    scrollable::Direction::Both { vertical: ipg_sb_y.construct(), horizontal: ipg_sb_x.construct() }
                },
                (true, false) => {
                    let ipg_sb = sb_x_opt.and_then(Widgets::as_scroller).cloned().unwrap_or_default();
                    if self.both_scrollers == Some(true) {
                        scrollable::Direction::Both { vertical: scrollable::Scrollbar::default(), horizontal: ipg_sb.construct() }
                    } else { scrollable::Direction::Horizontal(ipg_sb.construct()) }
                },
                (false, true) => {
                    let ipg_sb = sb_y_opt.and_then(Widgets::as_scroller).cloned().unwrap_or_default();
                    if self.both_scrollers == Some(true) {
                            scrollable::Direction::Both { vertical: ipg_sb.construct(), horizontal: scrollable::Scrollbar::default() }
                        } else { scrollable::Direction::Vertical(ipg_sb.construct()) }
                },
                (false, false) => {
                    if self.both_scrollers == Some(true) {
                        scrollable::Direction::Both { vertical: scrollable::Scrollbar::default(), horizontal: scrollable::Scrollbar::default() }
                    } else { scrollable::Direction::Vertical(scrollable::Scrollbar::default()) }
                },
            };

        scrollable::Scrollable::with_direction(content, direction)
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
pub struct ScrollableStyle {
    pub container_style_id: Option<usize>,
    pub container_style_std: Option<ContainerStyleStd>,
    pub vertical_rail_style_id: Option<usize>,
    pub horizontal_rail_style_id: Option<usize>,
    pub auto_scroll_style_id: Option<usize>,
    pub gap_color: Option<iced::Color>,
}

impl ScrollableStyle {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn set_style(
        &self, 
        theme: &Theme, 
        status: scrollable::Status,
        widgets: &HashMap<usize, Widgets>,
    ) -> scrollable::Style {
        
        let mut style = scrollable::default(theme, status);

        let ipg_container_style = self.lookup(widgets, self.container_style_id)
            .and_then(Widgets::as_container_style).cloned();

        let ipg_h_rail_style = self.lookup(widgets, self.horizontal_rail_style_id)
            .and_then(Widgets::as_rail_style).cloned();
        
        let ipg_v_rail_style = self.lookup(widgets, self.vertical_rail_style_id)
            .and_then(Widgets::as_rail_style).cloned();
        
        let ipg_auto_scroll_style = self.lookup(widgets, self.auto_scroll_style_id)
            .and_then(Widgets::as_auto_scroll_style).cloned();

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
pub struct Scroller {
    pub id: usize,
    pub width: Option<f32>,
    pub margin: Option<f32>,
    pub scroller_width: Option<f32>,
    pub spacing: Option<f32>,
    pub anchor: Option<Anchor>,
    pub hidden: Option<bool>,
}

impl Scroller {
    pub fn construct(&self) -> scrollable::Scrollbar {

        let mut sb = scrollable::Scrollbar::default();

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

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum Anchor {
    Start,
    End,
}

impl Anchor {
    fn to_iced(&self) -> scrollable::Anchor {
        match self {
            Anchor::Start => scrollable::Anchor::Start,
            Anchor::End => scrollable::Anchor::End,
        }
    }

    pub fn extract(value: &PyObject) -> Option<Anchor> {
        Python::attach(|py| {
            let res = value.extract::<Anchor>(py);
            match res {
                Ok(val) => Some(val),
                Err(err) => panic!(
                    "Unable to extract python {} {}", 
                    "Anchor",
                    err),
            }
        })  
    }
}

pub fn scrollable_callback(_state: &mut IpgState, id: usize, vp: scrollable::Viewport) {
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


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ScrollableParam {
    Fill,
    Height,
    ScrollerXId,
    ScrollerYId,
    StyleId,
    Width,
}



#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ScrollerParam {
    Anchor,
    Hidden,
    Margin,
    ScrollerWidth,
    Spacing,
    Width,
}


#[derive(Debug, Clone, PartialEq)]
pub struct RailStyle { 
    pub id: usize,
    pub background: Option<iced::Color>,
    pub border_color: Option<iced::Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
    pub scroller_background: Option<iced::Color>,
    pub scroller_border_color: Option<iced::Color>,
    pub scroller_border_width: Option<f32>,
    pub scroller_border_radius: Option<Vec<f32>>,
    
}

impl RailStyle {
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
pub struct AutoScrollStyle {
    pub id: usize,
    pub background: Option<iced::Color>,
    pub border_color: Option<iced::Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
    pub shadow_color: Option<iced::Color>,
    pub shadow_offset: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,
    pub shadow_icon_color: Option<iced::Color>,
}

impl AutoScrollStyle {
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

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ScrollableStyleParam { 
    AutoScrollStyleId,
    ContainerStyleId,
    ContainerStyleStd,
    GapColor,
    GapRgba,
    HorizontalRailStyleId,
    VerticalRailStyleId,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum RailStyleParam { 
    BackgroundColor,
    BackgroundRgba,
    BorderColor,
    BorderRgba,
    BorderWidth,
    BorderRadius,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum AutoScrollStyleParam {
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

impl WidgetParamUpdate for Scrollable {
    type Param = ScrollableParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ScrollableParam::Fill => set_lengths_fill(&mut self.width, &mut self.height, value, "Fill"),
            ScrollableParam::Height => set_height(&mut self.height, value, "Height"),
            ScrollableParam::ScrollerXId => set_opt_usize(&mut self.scroller_x_id, value, "ScrollerXId"),
            ScrollableParam::ScrollerYId => set_opt_usize(&mut self.scroller_y_id, value, "ScrollerYId"),
            ScrollableParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            ScrollableParam::Width => set_width(&mut self.width, value, "Width"),
        }
    }
}

impl WidgetParamUpdate for ScrollableStyle {
    type Param = ScrollableStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ScrollableStyleParam::ContainerStyleId => set_opt_usize(&mut self.container_style_id, value, "ContainerStyleId"),
            ScrollableStyleParam::ContainerStyleStd => todo!(),
            ScrollableStyleParam::VerticalRailStyleId => set_opt_usize(&mut self.vertical_rail_style_id, value, "VerticalRailStyleId"),
            ScrollableStyleParam::HorizontalRailStyleId => set_opt_usize(&mut self.horizontal_rail_style_id, value, "HorizontalRailStyleId"),
            ScrollableStyleParam::AutoScrollStyleId => set_opt_usize(&mut self.auto_scroll_style_id, value, "AutoScrollStyleId"),
            ScrollableStyleParam::GapColor => set_opt_iced_color(&mut self.gap_color, value, "GapColor"),
            ScrollableStyleParam::GapRgba => set_opt_iced_color_from_rgba(&mut self.gap_color, value, "GapRgba"),
        }
    }
}

impl WidgetParamUpdate for Scroller {
    type Param = ScrollerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ScrollerParam::Anchor => self.anchor = Anchor::extract(value),
            ScrollerParam::Hidden => set_opt_bool(&mut self.hidden, value, "Hidden"),
            ScrollerParam::Margin => set_opt_f32(&mut self.margin, value, "Margin"),
            ScrollerParam::ScrollerWidth => set_opt_f32(&mut self.scroller_width, value, "ScrollerWidth"),
            ScrollerParam::Spacing => set_opt_f32(&mut self.spacing, value, "Spacing"),
            ScrollerParam::Width => set_opt_f32(&mut self.width, value, "Width"),
        }
    }
}

impl WidgetParamUpdate for RailStyle {
    type Param = RailStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RailStyleParam::BackgroundColor => set_opt_iced_color(&mut self.background, value, "BackgroundColor"),
            RailStyleParam::BackgroundRgba => set_opt_iced_color_from_rgba(&mut self.background, value, "BackgroundRgba"),
            RailStyleParam::BorderColor => set_opt_iced_color(&mut self.border_color, value, "BorderColor"),
            RailStyleParam::BorderRgba => set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgba"),
            RailStyleParam::BorderWidth => set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            RailStyleParam::BorderRadius => set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
        }
    }
}

impl WidgetParamUpdate for AutoScrollStyle {
    type Param = AutoScrollStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            AutoScrollStyleParam::BackgroundColor => set_opt_iced_color(&mut self.background, value, "BackgroundColor"),
            AutoScrollStyleParam::BackgroundRgba => set_opt_iced_color_from_rgba(&mut self.background, value, "BackgroundRgba"),
            AutoScrollStyleParam::BorderColor => set_opt_iced_color(&mut self.border_color, value, "BorderColor"),
            AutoScrollStyleParam::BorderRgba => set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgba"),
            AutoScrollStyleParam::BorderWidth => set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            AutoScrollStyleParam::BorderRadius => set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
            AutoScrollStyleParam::ShadowColor => set_opt_iced_color(&mut self.shadow_color, value, "ShadowColor"),
            AutoScrollStyleParam::ShadowRgba => set_opt_iced_color_from_rgba(&mut self.shadow_color, value, "ShadowRgba"),
            AutoScrollStyleParam::ShadowOffset => set_opt_f32_array_2(&mut self.shadow_offset, value, "ShadowOffset"),
            AutoScrollStyleParam::ShadowBlurRadius => set_opt_f32(&mut self.shadow_blur_radius, value, "ShadowBlurRadius"),
            AutoScrollStyleParam::ShadowIconColor => set_opt_iced_color(&mut self.shadow_icon_color, value, "ShadowIconColor"),
            AutoScrollStyleParam::ShadowIconRgba => set_opt_iced_color_from_rgba(&mut self.shadow_icon_color, value, "ShadowIconRgba"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_scrollable() -> Scrollable {
        Scrollable {
            id: 0,
            width: Length::Shrink,
            height: Length::Shrink,
            both_scrollers: None,
            scroller_x_id: None,
            scroller_y_id: None,
            style_id: None,
        }
    }

    fn make_scrollable_style() -> ScrollableStyle {
        ScrollableStyle::default()
    }

    fn make_scroller() -> Scroller {
        Scroller::default()
    }

    fn make_rail_style() -> RailStyle {
        RailStyle {
            id: 0,
            background: None,
            border_color: None,
            border_width: None,
            border_radius: None,
            scroller_background: None,
            scroller_border_color: None,
            scroller_border_width: None,
            scroller_border_radius: None,
        }
    }

    fn make_auto_scroll_style() -> AutoScrollStyle {
        AutoScrollStyle {
            id: 0,
            background: None,
            border_color: None,
            border_width: None,
            border_radius: None,
            shadow_color: None,
            shadow_offset: None,
            shadow_blur_radius: None,
            shadow_icon_color: None,
        }
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::attach(|py| py.None().into_py_any(py).unwrap())
    }

    // -- Scrollable param tests --
    #[test]
    fn test_fill() {
        let mut c = make_scrollable();
        c.param_update(ScrollableParam::Fill, &py_obj(Some(true)));
        assert_eq!(c.width, Length::Fill);
        assert_eq!(c.height, Length::Fill);
        c.param_update(ScrollableParam::Fill, &py_obj(Some(false)));
        assert_eq!(c.width, Length::Shrink);
        assert_eq!(c.height, Length::Shrink);
        c.param_update(ScrollableParam::Fill, &py_none());
        assert_eq!(c.width, Length::Shrink);
        assert_eq!(c.height, Length::Shrink);
    }

    #[test]
    fn test_scrollable_height() {
        let mut s = make_scrollable();
        s.param_update(ScrollableParam::Height, &py_obj(200.0f32));
        assert_eq!(s.height, Length::Fixed(200.0));
    }

    #[test]
    fn test_scrollable_width() {
        let mut s = make_scrollable();
        s.param_update(ScrollableParam::Width, &py_obj(300.0f32));
        assert_eq!(s.width, Length::Fixed(300.0));
    }

    #[test]
    fn test_scrollable_scroller_x_id() {
        let mut s = make_scrollable();
        s.param_update(ScrollableParam::ScrollerXId, &py_obj(1usize));
        assert_eq!(s.scroller_x_id, Some(1));
        s.param_update(ScrollableParam::ScrollerXId, &py_none());
        assert_eq!(s.scroller_x_id, None);
    }

    #[test]
    fn test_scrollable_scroller_y_id() {
        let mut s = make_scrollable();
        s.param_update(ScrollableParam::ScrollerYId, &py_obj(2usize));
        assert_eq!(s.scroller_y_id, Some(2));
        s.param_update(ScrollableParam::ScrollerYId, &py_none());
        assert_eq!(s.scroller_y_id, None);
    }

    #[test]
    fn test_scrollable_style_id() {
        let mut s = make_scrollable();
        s.param_update(ScrollableParam::StyleId, &py_obj(5usize));
        assert_eq!(s.style_id, Some(5));
        s.param_update(ScrollableParam::StyleId, &py_none());
        assert_eq!(s.style_id, None);
    }

    // -- ScrollableStyle param tests --

    #[test]
    fn test_scrollable_style_container_style_id() {
        let mut s = make_scrollable_style();
        s.param_update(ScrollableStyleParam::ContainerStyleId, &py_obj(3usize));
        assert_eq!(s.container_style_id, Some(3));
        s.param_update(ScrollableStyleParam::ContainerStyleId, &py_none());
        assert_eq!(s.container_style_id, None);
    }

    #[test]
    fn test_scrollable_style_vertical_rail_id() {
        let mut s = make_scrollable_style();
        s.param_update(ScrollableStyleParam::VerticalRailStyleId, &py_obj(4usize));
        assert_eq!(s.vertical_rail_style_id, Some(4));
    }

    #[test]
    fn test_scrollable_style_horizontal_rail_id() {
        let mut s = make_scrollable_style();
        s.param_update(ScrollableStyleParam::HorizontalRailStyleId, &py_obj(6usize));
        assert_eq!(s.horizontal_rail_style_id, Some(6));
    }

    #[test]
    fn test_scrollable_style_auto_scroll_id() {
        let mut s = make_scrollable_style();
        s.param_update(ScrollableStyleParam::AutoScrollStyleId, &py_obj(7usize));
        assert_eq!(s.auto_scroll_style_id, Some(7));
    }

    #[test]
    fn test_scrollable_style_gap_rgba() {
        let mut s = make_scrollable_style();
        s.param_update(ScrollableStyleParam::GapRgba, &py_obj(vec![0.5f32, 0.5, 0.5, 1.0]));
        assert!(s.gap_color.is_some());
    }

    // -- Scroller param tests --

    #[test]
    fn test_scroller_hidden() {
        let mut s = make_scroller();
        s.param_update(ScrollerParam::Hidden, &py_obj(true));
        assert_eq!(s.hidden, Some(true));
        s.param_update(ScrollerParam::Hidden, &py_none());
        assert_eq!(s.hidden, None);
    }

    #[test]
    fn test_scroller_margin() {
        let mut s = make_scroller();
        s.param_update(ScrollerParam::Margin, &py_obj(5.0f32));
        assert_eq!(s.margin, Some(5.0));
    }

    #[test]
    fn test_scroller_scroller_width() {
        let mut s = make_scroller();
        s.param_update(ScrollerParam::ScrollerWidth, &py_obj(10.0f32));
        assert_eq!(s.scroller_width, Some(10.0));
    }

    #[test]
    fn test_scroller_spacing() {
        let mut s = make_scroller();
        s.param_update(ScrollerParam::Spacing, &py_obj(3.0f32));
        assert_eq!(s.spacing, Some(3.0));
    }

    #[test]
    fn test_scroller_width() {
        let mut s = make_scroller();
        s.param_update(ScrollerParam::Width, &py_obj(8.0f32));
        assert_eq!(s.width, Some(8.0));
    }

    // -- RailStyle param tests --

    #[test]
    fn test_rail_background_rgba() {
        let mut s = make_rail_style();
        s.param_update(RailStyleParam::BackgroundRgba, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.background.is_some());
    }

    #[test]
    fn test_rail_border_rgba() {
        let mut s = make_rail_style();
        s.param_update(RailStyleParam::BorderRgba, &py_obj(vec![0.0f32, 1.0, 0.0, 1.0]));
        assert!(s.border_color.is_some());
    }

    #[test]
    fn test_rail_border_width() {
        let mut s = make_rail_style();
        s.param_update(RailStyleParam::BorderWidth, &py_obj(2.0f32));
        assert_eq!(s.border_width, Some(2.0));
    }

    #[test]
    fn test_rail_border_radius() {
        let mut s = make_rail_style();
        s.param_update(RailStyleParam::BorderRadius, &py_obj(vec![4.0f32, 4.0, 4.0, 4.0]));
        assert_eq!(s.border_radius, Some(vec![4.0, 4.0, 4.0, 4.0]));
        s.param_update(RailStyleParam::BorderRadius, &py_none());
        assert_eq!(s.border_radius, None);
    }

    // -- AutoScrollStyle param tests --

    #[test]
    fn test_auto_background_rgba() {
        let mut s = make_auto_scroll_style();
        s.param_update(AutoScrollStyleParam::BackgroundRgba, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.background.is_some());
    }

    #[test]
    fn test_auto_border_rgba() {
        let mut s = make_auto_scroll_style();
        s.param_update(AutoScrollStyleParam::BorderRgba, &py_obj(vec![0.0f32, 1.0, 0.0, 1.0]));
        assert!(s.border_color.is_some());
    }

    #[test]
    fn test_auto_border_width() {
        let mut s = make_auto_scroll_style();
        s.param_update(AutoScrollStyleParam::BorderWidth, &py_obj(1.5f32));
        assert_eq!(s.border_width, Some(1.5));
    }

    #[test]
    fn test_auto_border_radius() {
        let mut s = make_auto_scroll_style();
        s.param_update(AutoScrollStyleParam::BorderRadius, &py_obj(vec![2.0f32, 2.0, 2.0, 2.0]));
        assert_eq!(s.border_radius, Some(vec![2.0, 2.0, 2.0, 2.0]));
    }

    #[test]
    fn test_auto_shadow_rgba() {
        let mut s = make_auto_scroll_style();
        s.param_update(AutoScrollStyleParam::ShadowRgba, &py_obj(vec![0.0f32, 0.0, 0.0, 0.5]));
        assert!(s.shadow_color.is_some());
    }

    #[test]
    fn test_auto_shadow_offset() {
        let mut s = make_auto_scroll_style();
        s.param_update(AutoScrollStyleParam::ShadowOffset, &py_obj(vec![2.0f32, 3.0]));
        assert_eq!(s.shadow_offset, Some([2.0, 3.0]));
    }

    #[test]
    fn test_auto_shadow_blur_radius() {
        let mut s = make_auto_scroll_style();
        s.param_update(AutoScrollStyleParam::ShadowBlurRadius, &py_obj(5.0f32));
        assert_eq!(s.shadow_blur_radius, Some(5.0));
    }

    #[test]
    fn test_auto_shadow_icon_rgba() {
        let mut s = make_auto_scroll_style();
        s.param_update(AutoScrollStyleParam::ShadowIconRgba, &py_obj(vec![1.0f32, 1.0, 1.0, 1.0]));
        assert!(s.shadow_icon_color.is_some());
    }
}

