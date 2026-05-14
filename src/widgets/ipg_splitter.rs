//! ipg_splitter — self-contained resizable split-pane container.
//!
//! `SplitterH` places N panels side-by-side with a vertical sash between each pair.
//! `SplitterV` stacks N panels vertically with a horizontal sash between each pair.
//! Resizing is handled internally; the user only needs an optional `on_resize`
//! callback if they want to be notified of size changes.

use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::ipg_widgets::ipg_divider::sash::{self, sash_horizontal, sash_vertical, Status, Style};
use crate::py_api::helpers::get_radius;
use crate::state::{Containers, Widgets};
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};
use crate::IpgState;

use iced::widget::{row, column, container, stack};
use iced::{Background, Element, Theme};
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


// ============================================================================
// Shared style
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct SplitterStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,
    pub background_color_hovered: Option<Color>,
    pub background_color_hovered_alpha: Option<f32>,
    pub background_rgba_hovered: Option<[f32; 4]>,
    pub background_transparent: Option<bool>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
}

impl SplitterStyle {
    pub fn to_iced(&self, theme: &Theme, status: Status) -> Style {
        if self.background_transparent == Some(true) {
            return sash::transparent(theme, status);
        }
        let background_color =
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
        let background_color_hovered =
            Color::rgba_ipg_color_to_iced(self.background_rgba_hovered, &self.background_color_hovered, self.background_color_hovered_alpha);
        let border_color =
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);

        let mut base = sash::primary(theme, status);
        if let Some(bc) = background_color {
            base.background = Background::Color(bc);
        }
        if let Some(bch) = background_color_hovered {
            if status == Status::Hovered {
                base.background = Background::Color(bch);
            }
        }
        if let Some(br) = &self.border_radius {
            base.border_radius = get_radius(br, "Splitter".to_string());
        }
        if let Some(bc) = border_color {
            base.border_color = bc;
        }
        if let Some(bw) = self.border_width {
            base.border_width = bw;
        }
        base
    }
}

// ============================================================================
// SplitterH — horizontal split (vertical sash, left/right panels)
// ============================================================================

#[derive(Debug, Clone)]
pub struct SplitterH {
    pub id: usize,
    /// Widths of each panel in order (left → right).
    pub sizes: Vec<f32>,
    /// Height of all panels (they share the same height).
    pub height: f32,
    /// Minimum width any panel can be resized to.
    pub min_size: f32,
    /// Width of the sash drag-handle in logical pixels.
    pub sash_width: f32,
    /// Whether to call `on_resize` on every drag tick or only on release.
    pub on_resize_trigger: SplitterResizeTrigger,
    pub style_id: Option<usize>,
    pub show: bool,
    // runtime drag state — not set by user
    pub index_in_use: usize,
    pub value_in_use: f32,
}

impl SplitterH {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        content: Vec<Element<'a, Message, Theme>>,
        widgets: &'a HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message, Theme>> {
        if !self.show { return None }

        let style_opt = self
            .lookup(widgets, self.style_id)
            .and_then(Widgets::as_splitter_style)
            .cloned();

        // Build the panel row
        let mut panels = vec![];
        for (i, child) in content.into_iter().enumerate() {
            let w = self.sizes.get(i).copied().unwrap_or(100.0);
            panels.push(
                container(child)
                    .width(w)
                    .height(self.height)
                    .into(),
            );
        }
        let panel_row: Element<'a, Message, Theme> = row(panels).into();

        // Build the sash overlay (vertical bars between horizontal panels)
        let sash: Element<'a, Message, Theme> = sash_horizontal(
            self.id,
            self.sizes.clone(),
            self.sash_width,
            self.height,
            move |(id, index, value)| Message::SplitterChanged(id, index, value),
        )
        .include_last_handle(false)
        .on_release(Message::SplitterReleased(self.id))
        .style(move |theme: &Theme, status| {
            if let Some(st) = &style_opt {
                st.to_iced(theme, status)
            } else {
                sash::primary(theme, status)
            }
        })
        .into();

        Some(stack([panel_row, sash]).into())
    }
}

// ============================================================================
// SplitterV — vertical split (horizontal sash, top/bottom panels)
// ============================================================================

#[derive(Debug, Clone)]
pub struct SplitterV {
    pub id: usize,
    /// Heights of each panel in order (top → bottom).
    pub sizes: Vec<f32>,
    /// Width of all panels (they share the same width).
    pub width: f32,
    /// Minimum height any panel can be resized to.
    pub min_size: f32,
    /// Height of the sash drag-handle in logical pixels.
    pub sash_height: f32,
    /// Whether to call `on_resize` on every drag tick or only on release.
    pub on_resize_trigger: SplitterResizeTrigger,
    pub style_id: Option<usize>,
    pub show: bool,
    // runtime drag state
    pub index_in_use: usize,
    pub value_in_use: f32,
}

impl SplitterV {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        content: Vec<Element<'a, Message, Theme>>,
        widgets: &'a HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message, Theme>> {
        if !self.show { return None }

        let style_opt = self
            .lookup(widgets, self.style_id)
            .and_then(Widgets::as_splitter_style)
            .cloned();

        // Build the panel column
        let mut panels = vec![];
        for (i, child) in content.into_iter().enumerate() {
            let h = self.sizes.get(i).copied().unwrap_or(100.0);
            panels.push(
                container(child)
                    .width(self.width)
                    .height(h)
                    .into(),
            );
        }
        let panel_col: Element<'a, Message, Theme> = column(panels).into();

        // Build the sash overlay (horizontal bars between vertical panels)
        let sash: Element<'a, Message, Theme> = sash_vertical(
            self.id,
            self.sizes.clone(),
            self.width,
            self.sash_height,
            move |(id, index, value)| Message::SplitterChanged(id, index, value),
        )
        .include_last_handle(false)
        .on_release(Message::SplitterReleased(self.id))
        .style(move |theme: &Theme, status| {
            if let Some(st) = &style_opt {
                st.to_iced(theme, status)
            } else {
                sash::primary(theme, status)
            }
        })
        .into();

        Some(stack([panel_col, sash]).into())
    }
}

// ============================================================================
// Shared callback
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum SplitterResizeTrigger {
    OnDrag,
    OnRelease,
}

impl Default for SplitterResizeTrigger {
    fn default() -> Self { Self::OnDrag }
}

pub fn splitter_callback(state: &mut IpgState, id: usize, index: usize, value: f32) {
    // Find whether this is H or V and update its sizes in-place.
    let (trigger, min_size, panel_count) =
        if let Some(Containers::SplitterH(sp)) = state.containers.get(&id) {
            (sp.on_resize_trigger.clone(), sp.min_size, sp.sizes.len())
        } else if let Some(Containers::SplitterV(sp)) = state.containers.get(&id) {
            (sp.on_resize_trigger.clone(), sp.min_size, sp.sizes.len())
        } else {
            return;
        };

    let value = value.max(min_size);

    // Apply diff to adjacent panel (same logic as table column resizer).
    if let Some(Containers::SplitterH(sp)) = state.containers.get_mut(&id) {
        let diff = sp.sizes[index] - value;
        sp.sizes[index] = value;
        sp.index_in_use = index;
        sp.value_in_use = value;
        if index + 1 < panel_count {
            let next = (sp.sizes[index + 1] + diff).max(min_size);
            sp.sizes[index + 1] = next;
        }
    } else if let Some(Containers::SplitterV(sp)) = state.containers.get_mut(&id) {
        let diff = sp.sizes[index] - value;
        sp.sizes[index] = value;
        sp.index_in_use = index;
        sp.value_in_use = value;
        if index + 1 < panel_count {
            let next = (sp.sizes[index + 1] + diff).max(min_size);
            sp.sizes[index + 1] = next;
        }
    }

    if trigger == SplitterResizeTrigger::OnDrag {
        // Collect updated sizes for the callback.
        let sizes: Vec<f32> = if let Some(Containers::SplitterH(sp)) = state.containers.get(&id) {
            sp.sizes.clone()
        } else if let Some(Containers::SplitterV(sp)) = state.containers.get(&id) {
            sp.sizes.clone()
        } else { vec![] };

        invoke_callback_with_args(id, "on_resize", "Splitter", (index, value, sizes),
            "def cb(wid: int, data: tuple[int, float, list[float]])");
    }
}

pub fn splitter_release_callback(state: &mut IpgState, id: usize) {
    // If trigger is OnRelease, fire now.
    let (trigger, index, value, sizes) =
        if let Some(Containers::SplitterH(sp)) = state.containers.get(&id) {
            (sp.on_resize_trigger.clone(), sp.index_in_use, sp.value_in_use, sp.sizes.clone())
        } else if let Some(Containers::SplitterV(sp)) = state.containers.get(&id) {
            (sp.on_resize_trigger.clone(), sp.index_in_use, sp.value_in_use, sp.sizes.clone())
        } else {
            return;
        };

    invoke_callback_with_args(id, "on_release", "Splitter", (index, value, sizes.clone()),
        "def cb(wid: int, data: tuple[int, float, list[float]])");

    if trigger == SplitterResizeTrigger::OnRelease {
        invoke_callback_with_args(id, "on_resize", "Splitter", (index, value, sizes),
            "def cb(wid: int, data: tuple[int, float, list[float]])");
    }
}

// ============================================================================
// Params
// ============================================================================

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SplitterHParam {
    Height,
    MinSize,
    OnResizeTrigger,
    SashWidth,
    Show,
    Sizes,
    StyleId,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SplitterVParam {
    Width,
    MinSize,
    OnResizeTrigger,
    SashHeight,
    Show,
    Sizes,
    StyleId,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SplitterStyleParam {
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRgba,
    BackgroundColorHovered,
    BackgroundColorHoveredAlpha,
    BackgroundRgbaHovered,
    BackgroundTransparent,
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
    BorderRadius,
    BorderWidth,
}

impl WidgetParamUpdate for SplitterH {
    type Param = SplitterHParam;
    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SplitterHParam::Height    => set_t_value(&mut self.height, value, "SplitterHParam::Height"),
            SplitterHParam::MinSize   => set_t_value(&mut self.min_size, value, "SplitterHParam::MinSize"),
            SplitterHParam::SashWidth => set_t_value(&mut self.sash_width, value, "SplitterHParam::SashWidth"),
            SplitterHParam::Show      => set_t_value(&mut self.show, value, "SplitterHParam::Show"),
            SplitterHParam::Sizes     => set_t_value(&mut self.sizes, value, "SplitterHParam::Sizes"),
            SplitterHParam::StyleId   => set_t_value(&mut self.style_id, value, "SplitterHParam::StyleId"),
            SplitterHParam::OnResizeTrigger => (), // set via dedicated param; ignore for now
        }
    }
}

impl WidgetParamUpdate for SplitterV {
    type Param = SplitterVParam;
    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SplitterVParam::Width      => set_t_value(&mut self.width, value, "SplitterVParam::Width"),
            SplitterVParam::MinSize    => set_t_value(&mut self.min_size, value, "SplitterVParam::MinSize"),
            SplitterVParam::SashHeight => set_t_value(&mut self.sash_height, value, "SplitterVParam::SashHeight"),
            SplitterVParam::Show       => set_t_value(&mut self.show, value, "SplitterVParam::Show"),
            SplitterVParam::Sizes      => set_t_value(&mut self.sizes, value, "SplitterVParam::Sizes"),
            SplitterVParam::StyleId    => set_t_value(&mut self.style_id, value, "SplitterVParam::StyleId"),
            SplitterVParam::OnResizeTrigger => (),
        }
    }
}

impl WidgetParamUpdate for SplitterStyle {
    type Param = SplitterStyleParam;
    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SplitterStyleParam::BackgroundColor          => set_t_value(&mut self.background_color, value, "SplitterStyleParam::BackgroundColor"),
            SplitterStyleParam::BackgroundColorAlpha     => set_t_value(&mut self.background_color_alpha, value, "SplitterStyleParam::BackgroundColorAlpha"),
            SplitterStyleParam::BackgroundRgba           => set_t_value(&mut self.background_rgba, value, "SplitterStyleParam::BackgroundRgba"),
            SplitterStyleParam::BackgroundColorHovered   => set_t_value(&mut self.background_color_hovered, value, "SplitterStyleParam::BackgroundColorHovered"),
            SplitterStyleParam::BackgroundColorHoveredAlpha => set_t_value(&mut self.background_color_hovered_alpha, value, "SplitterStyleParam::BackgroundColorHoveredAlpha"),
            SplitterStyleParam::BackgroundRgbaHovered    => set_t_value(&mut self.background_rgba_hovered, value, "SplitterStyleParam::BackgroundRgbaHovered"),
            SplitterStyleParam::BackgroundTransparent    => set_t_value(&mut self.background_transparent, value, "SplitterStyleParam::BackgroundTransparent"),
            SplitterStyleParam::BorderColor              => set_t_value(&mut self.border_color, value, "SplitterStyleParam::BorderColor"),
            SplitterStyleParam::BorderColorAlpha         => set_t_value(&mut self.border_color_alpha, value, "SplitterStyleParam::BorderColorAlpha"),
            SplitterStyleParam::BorderRgba               => set_t_value(&mut self.border_rgba, value, "SplitterStyleParam::BorderRgba"),
            SplitterStyleParam::BorderRadius             => set_t_value(&mut self.border_radius, value, "SplitterStyleParam::BorderRadius"),
            SplitterStyleParam::BorderWidth              => set_t_value(&mut self.border_width, value, "SplitterStyleParam::BorderWidth"),
        }
    }
}
