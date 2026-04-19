//! ipg_scrollable

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::helpers::get_len;
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::styling::{apply_border_overrides, apply_shadow_overrides_xy};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value
};

use std::collections::HashMap;
use iced::widget::scrollable;
use iced::widget::scrollable::Viewport;
use iced::{Element, Theme};
use iced::widget::Column;

use pyo3::pyclass;
use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Scrollable {
    pub id: usize,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub auto_scroll: Option<bool>,
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
        
        let both_scrollers = 
            if sb_x_opt.is_some() && sb_y_opt.is_some() {
                true
            } else { false };

        let direction = 
            match (sb_x_opt.is_some(), sb_y_opt.is_some()) {
                (true, true) => {
                    let ipg_sb_x = sb_x_opt.and_then(Widgets::as_scroller).cloned().unwrap_or_default();
                    let ipg_sb_y = sb_y_opt.and_then(Widgets::as_scroller).cloned().unwrap_or_default();
                    scrollable::Direction::Both { vertical: ipg_sb_y.construct(), horizontal: ipg_sb_x.construct() }
                },
                (true, false) => {
                    let ipg_sb = sb_x_opt.and_then(Widgets::as_scroller).cloned().unwrap_or_default();
                    if both_scrollers {
                        scrollable::Direction::Both { vertical: scrollable::Scrollbar::default(), horizontal: ipg_sb.construct() }
                    } else { scrollable::Direction::Horizontal(ipg_sb.construct()) }
                },
                (false, true) => {
                    let ipg_sb = sb_y_opt.and_then(Widgets::as_scroller).cloned().unwrap_or_default();
                    if both_scrollers {
                            scrollable::Direction::Both { vertical: ipg_sb.construct(), horizontal: scrollable::Scrollbar::default() }
                        } else { scrollable::Direction::Vertical(ipg_sb.construct()) }
                },
                (false, false) => {
                    if both_scrollers {
                        scrollable::Direction::Both { vertical: scrollable::Scrollbar::default(), horizontal: scrollable::Scrollbar::default() }
                    } else { scrollable::Direction::Vertical(scrollable::Scrollbar::default()) }
                },
            };

        scrollable::Scrollable::with_direction(content, direction)
            .width(get_len(self.fill, self.width_fill, self.width))
            .height(get_len(self.fill, self.height_fill, self.height))
            .auto_scroll(self.auto_scroll.unwrap_or(false))
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
    pub vertical_rail_style_id: Option<usize>,
    pub horizontal_rail_style_id: Option<usize>,
    pub auto_scroll_style_id: Option<usize>,
    pub gap_color: Option<Color>,
    pub gap_color_alpha: Option<f32>,
    pub gap_rgba: Option<[f32; 4]>
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
        
        style.horizontal_rail = if let Some(rail) = &ipg_h_rail_style {
            rail.to_iced(&mut def_h_rail)
        } else { def_h_rail };
        
        style.vertical_rail = if let Some(rail) = &ipg_v_rail_style {
            rail.to_iced(&mut def_v_rail)
        } else { def_v_rail };
        
        let gap_color = 
            Color::rgba_ipg_color_to_iced(self.gap_rgba, &self.gap_color, self.gap_color_alpha);

        if let Some(c) = gap_color {
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
    pub anchor_start: Option<bool>,
    pub anchor_end: Option<bool>,
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

        sb = if self.anchor_start == Some(true) {
                sb.anchor(scrollable::Anchor::Start)
            } else { sb };

        sb = if self.anchor_end == Some(true) {
                sb.anchor(scrollable::Anchor::End)
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

pub fn scrollable_callback(id: usize, vp: Viewport) {
    let mut hmap = HashMap::new();
    hmap.insert("abs_x".to_string(), vp.absolute_offset().x);
    hmap.insert("abs_y".to_string(), vp.absolute_offset().y);
    hmap.insert("rel_x".to_string(), vp.relative_offset().x);
    hmap.insert("rel_y".to_string(), vp.relative_offset().y);
    hmap.insert("rev_x".to_string(), vp.absolute_offset_reversed().x);
    hmap.insert("rev_y".to_string(), vp.absolute_offset_reversed().y);
    
    let bounds = vp.bounds();
    hmap.insert("bounds_x".to_string(), bounds.x);
    hmap.insert("bounds_y".to_string(), bounds.y);
    hmap.insert("bounds_width".to_string(), bounds.width);
    hmap.insert("bounds_height".to_string(), bounds.height);

    let content = vp.content_bounds();
    hmap.insert("content_x".to_string(), content.x);
    hmap.insert("content_y".to_string(), content.y);
    hmap.insert("content_width".to_string(), content.width);
    hmap.insert("content_height".to_string(), content.height);
    
    invoke_callback_with_args(id, "on_scroll", "Scrollable", hmap,
                "def cb(wid: int, scroll_data: dict)");
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ScrollableParam {
    Fill,
    Height,
    HeightFill,
    ScrollerXId,
    ScrollerYId,
    StyleId,
    Width,
    WidthFill,
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ScrollerParam {
    AnchorStart,
    AnchorEnd,
    Hidden,
    Margin,
    ScrollerWidth,
    Spacing,
    Width,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RailStyle { 
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
    pub scroller_background_color: Option<Color>,
    pub scroller_background_color_alpha: Option<f32>,
    pub scroller_background_rgba: Option<[f32; 4]>,
    pub scroller_border_color: Option<Color>,
    pub scroller_border_color_alpha: Option<f32>,
    pub scroller_border_rgba: Option<[f32; 4]>,
    pub scroller_border_width: Option<f32>,
    pub scroller_border_radius: Option<Vec<f32>>,
}

impl RailStyle {
    fn to_iced(&self, rail: &mut scrollable::Rail) -> scrollable::Rail {
        
        let background = 
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
        let border_color = 
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);

        let scroller_background = 
            Color::rgba_ipg_color_to_iced(self.scroller_background_rgba, &self.scroller_background_color, self.scroller_background_color_alpha);
        let scroller_border_color = 
            Color::rgba_ipg_color_to_iced(self.scroller_border_rgba, &self.scroller_border_color, self.scroller_border_color_alpha);

        if let Some(color) = background {
            rail.background = Some(color.into());
        }

        apply_border_overrides(
            &mut rail.border, border_color,
            &self.border_radius, self.border_width, self.type_name(),
        );

        if let Some(color) = scroller_background {
            rail.scroller.background = color.into();
        }

        apply_border_overrides(
            &mut rail.scroller.border, scroller_border_color,
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
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
    pub shadow_color: Option<Color>,
    pub shadow_color_alpha: Option<f32>,
    pub shadow_rgba: Option<[f32; 4]>,
    pub shadow_offset: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,
    pub shadow_icon_color: Option<Color>,
    pub shadow_icon_color_alpha: Option<f32>,
    pub shadow_icon_rgba: Option<[f32; 4]>,
}

impl AutoScrollStyle {
    fn to_iced(&self, auto: &mut scrollable::AutoScroll) -> scrollable::AutoScroll {

        let background = 
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
        let border_color = 
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);
        let shadow_color = 
            Color::rgba_ipg_color_to_iced(self.shadow_rgba, &self.shadow_color, self.shadow_color_alpha);
        let shadow_icon_color = 
            Color::rgba_ipg_color_to_iced(self.shadow_icon_rgba, &self.shadow_icon_color, self.shadow_icon_color_alpha);

        if let Some(color) = background {
            auto.background = color.into();
        }

        apply_border_overrides(
            &mut auto.border, border_color,
            &self.border_radius, self.border_width, self.type_name(),
        );

        apply_shadow_overrides_xy(
            &mut auto.shadow, shadow_color,
            self.shadow_offset, self.shadow_blur_radius,
        );

        if let Some(color) = shadow_icon_color {
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
    GapColor,
    GapRgba,
    HorizontalRailStyleId,
    VerticalRailStyleId,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum RailStyleParam { 
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRgba,
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
    BorderWidth,
    BorderRadius,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum AutoScrollStyleParam {
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRgba,
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
    BorderWidth,
    BorderRadius,
    ShadowColor,
    ShadowColorAlpha,
    ShadowRgba,
    ShadowOffset,
    ShadowBlurRadius,
    ShadowIconColor,
    ShadowIconColorAlpha,
    ShadowIconRgba,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Scrollable {
    type Param = ScrollableParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ScrollableParam::Fill => set_t_value(&mut self.fill, value, "ScrollableParam::Fill"),
            ScrollableParam::Height => set_t_value(&mut self.height, value, "ScrollableParam::Height"),
            ScrollableParam::HeightFill => set_t_value(&mut self.height_fill, value, "ScrollableParam::HeightFill"),
            ScrollableParam::ScrollerXId => set_t_value(&mut self.scroller_x_id, value, "ScrollableParam::ScrollerXId"),
            ScrollableParam::ScrollerYId => set_t_value(&mut self.scroller_y_id, value, "ScrollableParam::ScrollerYId"),
            ScrollableParam::StyleId => set_t_value(&mut self.style_id, value, "ScrollableParam::StyleId"),
            ScrollableParam::Width => set_t_value(&mut self.width, value, "ScrollableParam::Width"),
            ScrollableParam::WidthFill => set_t_value(&mut self.width_fill, value, "ScrollableParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for ScrollableStyle {
    type Param = ScrollableStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ScrollableStyleParam::VerticalRailStyleId => set_t_value(&mut self.vertical_rail_style_id, value, "ScrollableStyleParam::VerticalRailStyleId"),
            ScrollableStyleParam::HorizontalRailStyleId => set_t_value(&mut self.horizontal_rail_style_id, value, "ScrollableStyleParam::HorizontalRailStyleId"),
            ScrollableStyleParam::AutoScrollStyleId => set_t_value(&mut self.auto_scroll_style_id, value, "ScrollableStyleParam::AutoScrollStyleId"),
            ScrollableStyleParam::GapColor => set_t_value(&mut self.gap_color, value, "ScrollableStyleParam::GapColor"),
            ScrollableStyleParam::GapRgba => set_t_value(&mut self.gap_color, value, "ScrollableStyleParam::GapRgba"),
        }
    }
}

impl WidgetParamUpdate for Scroller {
    type Param = ScrollerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ScrollerParam::AnchorEnd => set_t_value(&mut self.anchor_end, value, "ScrollerParam::AnchorEnd"),
            ScrollerParam::AnchorStart => set_t_value(&mut self.anchor_start, value, "ScrollerParam::AnchorStart"),
            ScrollerParam::Hidden => set_t_value(&mut self.hidden, value, "ScrollerParam::Hidden"),
            ScrollerParam::Margin => set_t_value(&mut self.margin, value, "ScrollerParam::Margin"),
            ScrollerParam::ScrollerWidth => set_t_value(&mut self.scroller_width, value, "ScrollerParam::ScrollerWidth"),
            ScrollerParam::Spacing => set_t_value(&mut self.spacing, value, "ScrollerParam::Spacing"),
            ScrollerParam::Width => set_t_value(&mut self.width, value, "ScrollerParam::Width"),
        }
    }
}

impl WidgetParamUpdate for RailStyle {
    type Param = RailStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RailStyleParam::BackgroundColor => set_t_value(&mut self.background_color, value, "RailStyleParam::BackgroundColor"),
            RailStyleParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "RailStyleParam::BackgroundColorAlpha"),
            RailStyleParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "RailStyleParam::BackgroundRgba"),
            RailStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "RailStyleParam::BorderColor"),
            RailStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "RailStyleParam::BorderColorAlpha"),
            RailStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "RailStyleParam::BorderRadius"),
            RailStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "RailStyleParam::BorderRgba"),
            RailStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "RailStyleParam::BorderWidth"),
        }
    }
}

impl WidgetParamUpdate for AutoScrollStyle {
    type Param = AutoScrollStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            AutoScrollStyleParam::BackgroundColor => set_t_value(&mut self.background_color, value, "AutoScrollStyleParam::BackgroundColor"),
            AutoScrollStyleParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "AutoScrollStyleParam::BackgroundColorAlpha"),
            AutoScrollStyleParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "AutoScrollStyleParam::BackgroundRgba"),
            AutoScrollStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "AutoScrollStyleParam::BorderColor"),
            AutoScrollStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "AutoScrollStyleParam::BorderColorAlpha"),
            AutoScrollStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "AutoScrollStyleParam::BorderRadius"),
            AutoScrollStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "AutoScrollStyleParam::BorderRgba"),
            AutoScrollStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "AutoScrollStyleParam::BorderWidth"),
            AutoScrollStyleParam::ShadowBlurRadius => set_t_value(&mut self.shadow_blur_radius, value, "AutoScrollStyleParam::ShadowBlurRadius"),
            AutoScrollStyleParam::ShadowColor => set_t_value(&mut self.shadow_color, value, "AutoScrollStyleParam::ShadowColor"),
            AutoScrollStyleParam::ShadowColorAlpha => set_t_value(&mut self.shadow_color_alpha, value, "AutoScrollStyleParam::ShadowColorAlpha"),
            AutoScrollStyleParam::ShadowIconColor => set_t_value(&mut self.shadow_icon_color, value, "AutoScrollStyleParam::ShadowIconColor"),
            AutoScrollStyleParam::ShadowIconColorAlpha => set_t_value(&mut self.shadow_icon_color_alpha, value, "AutoScrollStyleParam::ShadowIconColorAlpha"),
            AutoScrollStyleParam::ShadowIconRgba => set_t_value(&mut self.shadow_icon_rgba, value, "AutoScrollStyleParam::ShadowIconRgba"),
            AutoScrollStyleParam::ShadowOffset => set_t_value(&mut self.shadow_offset, value, "AutoScrollStyleParam::ShadowOffset"),
            AutoScrollStyleParam::ShadowRgba => set_t_value(&mut self.shadow_rgba, value, "AutoScrollStyleParam::ShadowRgba"),
        }
    }
}
