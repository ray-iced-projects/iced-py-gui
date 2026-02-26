//! ipg_scrollable

use crate::IpgState;
use crate::access_callbacks;
use crate::access_user_data1;
use crate::app::Message;
use crate::state::IpgWidgets;
use crate::widgets::ipg_container;
use crate::widgets::styling::{apply_border_overrides, apply_shadow_overrides_xy};
use crate::widgets::widget_param_update::set_opt_bool;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,
    set_opt_f32, set_opt_f32_array_2, set_opt_iced_color,
    set_opt_usize, set_opt_vec_f32, set_iced_color_from_rgba,
    set_height, set_width,
};

use std::collections::HashMap;
use iced::widget::scrollable;
use iced::widget::scrollable::Anchor;
use iced::widget::scrollable::Scrollbar;
use iced::widget::scrollable::{Direction, Scrollable, 
    Viewport, Status, Style};
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
    pub container_style_id: Option<usize>,
    pub rail_x_style_id: Option<usize>,
    pub rail_y_style_id: Option<usize>,
    pub auto_scroll_style_id: Option<usize>,
    pub gap_background_color: Option<Color>,
}

impl IpgScrollable {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        content: Vec<Element<'a, Message>>,
        widgets: &HashMap<usize, IpgWidgets>,
        ) -> Element<'a, Message> {

        let sb_x_opt = self.lookup(widgets, self.scrollbar_x_id);
        let sb_y_opt = self.lookup(widgets, self.scrollbar_y_id);

        let ipg_cont_style_opt = self.lookup(widgets, self.container_style_id)
            .and_then(IpgWidgets::as_container_style).cloned();
        
        let ipg_rail_x_style_opt = self.lookup(widgets, self.rail_x_style_id)
            .and_then(IpgWidgets::as_rail_style).cloned();
        
        let ipg_rail_y_style_opt = self.lookup(widgets, self.rail_y_style_id)
            .and_then(IpgWidgets::as_rail_style).cloned();
        
        let ipg_auto_scroll_style_opt = self.lookup(widgets, self.auto_scroll_style_id)
            .and_then(IpgWidgets::as_auto_scroll_style).cloned();

        let content: Element<'a, Message> = Column::with_children(content).into();

        let direction = 
            match (sb_x_opt.is_some(), sb_y_opt.is_some()) {
                (true, true) => {
                    let ipg_sb_x = sb_x_opt.and_then(IpgWidgets::as_scrollbar).cloned().unwrap_or_default();
                    let ipg_sb_y = sb_y_opt.and_then(IpgWidgets::as_scrollbar).cloned().unwrap_or_default();
                    Direction::Both { vertical: ipg_sb_y.construct(), horizontal: ipg_sb_x.construct() }
                },
                (true, false) => {
                    let ipg_sb = sb_x_opt.and_then(IpgWidgets::as_scrollbar).cloned().unwrap_or_default();
                    Direction::Horizontal(ipg_sb.construct())
                },
                (false, true) => {
                    let ipg_sb = sb_y_opt.and_then(IpgWidgets::as_scrollbar).cloned().unwrap_or_default();
                    Direction::Vertical(ipg_sb.construct())
                },
                (false, false) => Direction::Vertical(Scrollbar::default()),
            };

        Scrollable::with_direction(content, direction)
            .width(self.width)
            .height(self.height)
            .on_scroll(move|vp| 
                Message::Scrolled(vp, self.id))
            .style(move|theme, status| {
                get_styling(
                    theme, 
                    status, 
                    &ipg_cont_style_opt,
                    &ipg_rail_x_style_opt,
                    &ipg_rail_y_style_opt,
                    &ipg_auto_scroll_style_opt,
                    self.gap_background_color
                )
            })
            .into()
        
    }
}



#[derive(Debug, Default, Clone)]
pub struct IpgScrollbar {
    pub id: usize,
    pub width: Option<f32>,
    pub margin: Option<f32>,
    pub scroller_width: Option<f32>,
    pub spacing: Option<f32>,
    pub anchor: Option<IpgAnchor>,
    pub hidden: Option<bool>,
}

impl IpgScrollbar {
    pub fn construct(&self) -> Scrollbar {
        
        let sb = Scrollbar::default();

        if let Some(w) = self.width {
            sb.width(w);
        }

        if let Some(m) = self.margin {
            sb.margin(m);
        }

        if let Some(sw) = self.scroller_width {
            sb.scroller_width(sw);
        }

        if let Some(an) = &self.anchor {
            sb.anchor(an.to_iced());
        }

        if let Some(sp) = self.spacing {
            sb.spacing(sp);
        }

        if let Some(hd) = self.hidden {
            if hd {
                sb.width(0).scroller_width(0);
            }
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
                Err(_) => panic!("Unable to extract python IpgAnchor"),
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
    AutoScrollStyleId,
    ContainerStyleId,
    GapBackgroundColor,
    GapBackgroundRgba,
    Height,
    RailXStyleId,
    RailYStyleId,
    ScrollbarXId,
    ScrollbarYId,
    Width,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgScrollbarParam {
    Anchor,
    Hidden,
    Margin,
    ScrollerWidth,
    Spacing,
    Width,
}

fn get_styling(
    theme: &Theme, status: Status,
    cont_style_opt: &Option<ipg_container::IpgContainerStyle>, // container
    ipg_rail_x_style_opt: &Option<IpgRailStyle>, // horizontal_rail
    ipg_rail_y_style_opt: &Option<IpgRailStyle>, // vertical_rail
    ipg_auto_scroll_style_opt: &Option<IpgAutoScrollStyle>, // auto_scroll
    gap_bkg_color: Option<Color>, // gap
    ) -> Style 
{

    let mut style = scrollable::default(theme, status);

    // Need default since iced doesn't default each one individually
    let def_h_rail = style.horizontal_rail;
    let def_v_rail = style.vertical_rail;
    let def_auto_scroll = style.auto_scroll;
    
    // container style via the container_style_id
    style.container = ipg_container::get_styling(theme, cont_style_opt);
    
    style.horizontal_rail = get_rail_styling(ipg_rail_x_style_opt, def_h_rail);
    
    style.vertical_rail = get_rail_styling(ipg_rail_y_style_opt, def_v_rail);
    
    if let Some(c) = gap_bkg_color {
        style.gap = Some(c.into());
    }
    
    style.auto_scroll = get_auto_styling(ipg_auto_scroll_style_opt, def_auto_scroll);
    
    style
    
}

fn get_rail_styling(
    style_opt: &Option<IpgRailStyle>,
    default: scrollable::Rail,
) -> scrollable::Rail {
    let style = match style_opt {
        Some(s) => s,
        None => return default,
    };

    let mut rail = default;

    if let Some(color) = style.background {
        rail.background = Some(color.into());
    }

    apply_border_overrides(
        &mut rail.border, style.border_color,
        &style.border_radius, style.border_width, "Scrollable rail",
    );

    rail
}

fn get_auto_styling(
    style_opt: &Option<IpgAutoScrollStyle>,
    default: scrollable::AutoScroll,
) -> scrollable::AutoScroll {
    let style = match style_opt {
        Some(s) => s,
        None => return default,
    };

    let mut auto = default;

    if let Some(color) = style.background {
        auto.background = color.into();
    }

    apply_border_overrides(
        &mut auto.border, style.border_color,
        &style.border_radius, style.border_width, "Scrollable auto_scroll",
    );

    apply_shadow_overrides_xy(
        &mut auto.shadow, style.shadow_color,
        style.shadow_offset, style.shadow_blur_radius,
    );

    if let Some(color) = style.shadow_icon_color {
        auto.icon = color;
    }

    auto
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
            IpgScrollableParam::AutoScrollStyleId => set_opt_usize(&mut self.auto_scroll_style_id, value, name),
            IpgScrollableParam::ContainerStyleId => set_opt_usize(&mut self.container_style_id, value, name),
            IpgScrollableParam::GapBackgroundColor => set_opt_iced_color(&mut self.gap_background_color, value, name),
            IpgScrollableParam::GapBackgroundRgba => set_iced_color_from_rgba(&mut self.gap_background_color, value, name),
            IpgScrollableParam::Height => set_height(&mut self.height, value, name),
            IpgScrollableParam::RailXStyleId => set_opt_usize(&mut self.rail_x_style_id, value, name),
            IpgScrollableParam::RailYStyleId => set_opt_usize(&mut self.rail_y_style_id, value, name),
            IpgScrollableParam::ScrollbarXId => set_opt_usize(&mut self.scrollbar_x_id, value, name),
            IpgScrollableParam::ScrollbarYId => set_opt_usize(&mut self.scrollbar_y_id, value, name),
            IpgScrollableParam::Width => set_width(&mut self.width, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgScrollbar {
    type Param = IpgScrollbarParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgScrollbarParam::Anchor => self.anchor = IpgAnchor::extract(value),
            IpgScrollbarParam::Hidden => set_opt_bool(&mut self.hidden, value, name),
            IpgScrollbarParam::Margin => set_opt_f32(&mut self.margin, value, name),
            IpgScrollbarParam::ScrollerWidth => set_opt_f32(&mut self.scroller_width, value, name),
            IpgScrollbarParam::Spacing => set_opt_f32(&mut self.spacing, value, name),
            IpgScrollbarParam::Width => set_opt_f32(&mut self.width, value, name),
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
