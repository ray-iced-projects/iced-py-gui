use std::collections::HashMap;

use iced::{Element, Theme};

use iced_sash::{Id, OuterResizeMode, SashH, SashV, Status, Style};
pub use iced_sash::resize as sash_resize;

use crate::app::Message;
use crate::graphics::colors::{self, Color};
use crate::py_api::helpers::get_radius;
use crate::state::{Containers, IpgState, Widgets};
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Sash {
    pub id: usize,
    pub initial_sizes: Vec<f32>,
    pub current_sizes: Vec<f32>,
    pub size: f32,
    pub sash_size: f32,
    pub sync_sashes: Option<bool>,
    pub sync_cross_sashes: Option<bool>,
    pub outer_handle_size: Option<f32>,
    pub cross_handle_size: Option<f32>,
    pub resize_mode_last: Option<bool>,
    pub resize_mode_uniform: Option<bool>,
    pub resize_mode_proportional: Option<bool>,
    pub vertical_direction: Option<bool>,
    pub min_size: Option<f32>,
    pub max_size: Option<f32>,
    pub min_cross_size: Option<f32>,
    pub max_cross_size: Option<f32>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_std: Option<SashStyleStd>,
    pub show: bool,
    pub resize_mode: OuterResizeMode,
}

impl Sash {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &self,
        content: Vec<Element<'a, Message>>, 
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message>> {

        if !self.show { return None }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_sash_style).cloned();

        let widget_id = self.id;

        let sh = if self.vertical_direction == Some(true) {
            SashV::new(content, self.initial_sizes.clone(), self.size, self.sash_size)
        } else {
            SashH::new(content, self.initial_sizes.clone(), self.size, self.sash_size)
        }
        .on_resize(move |id, index, size| Message::Sash(widget_id, SashMessage::ResizedH(id, index, size)))
        .on_release(move |id, index| Message::Sash(widget_id, SashMessage::Released(id, index)))
        .on_outer_resize(move |id, size| Message::Sash(widget_id, SashMessage::ResizedOuter(id, size)))
        .on_cross_resize(move |id, size| Message::Sash(widget_id, SashMessage::ResizedCrossH(id, size)))
        .max_size_maybe(self.max_size)
        .max_cross_size_maybe(self.max_cross_size);

        let sh = if self.sync_sashes == Some(true) {
            sh.sync_sashes(self.current_sizes.clone())
        } else { sh };

        let sh = if self.sync_cross_sashes == Some(true) {
            sh.sync_cross_sashes(self.size)
        } else { sh };

        let sh = if let Some(sz) = self.outer_handle_size {
            sh.outer_handle(sz)
        } else { sh };

        let sh = if let Some(sz) = self.cross_handle_size {
            sh.cross_handle(sz)
        } else { sh };

        let sh = if let Some(min) = self.min_size {
            sh.min_size(min)
        } else { sh };

        let sh = if let Some(min) = self.min_cross_size {
            sh.min_cross_size(min)
        } else { sh };

        let sh = if self.clip == Some(true) {
            sh.clip(true)
        } else { sh };

        let sh = sh.outer_resize_mode(self.resize_mode);

        let sh = if let Some(st) = style_opt {
            sh.style(move|theme, status| {   
                    st.to_iced(theme, status)})
        } else { sh };

        Some(sh.into())
    }
}


#[derive(Debug, Clone)]
pub enum SashMessage {
    ResizedH(Id, usize, f32),
    ResizedV(Id, usize, f32),
    ResizedOuter(Id, f32),
    ResizedCrossH(Id, f32),
    Released(Id, usize),
}

pub fn sash_callback(state: &mut IpgState, widget_id: usize, message: SashMessage) {
    match message {
        SashMessage::ResizedH(_id, index, size) => {
            // Update the resized sash's current_sizes
            let (sync_enabled, updated_sizes) = {
                let sash = match state.containers.get_mut(&widget_id) {
                    Some(Containers::Sash(s)) => s,
                    _ => return,
                };
                let min = sash.min_size.unwrap_or(0.0);
                sash_resize(&mut sash.current_sizes, index, size, min);
                (sash.sync_sashes == Some(true), sash.current_sizes.clone())
            };

            // Propagate to all other sashes in the sync group
            if sync_enabled {
                for (id, container) in state.containers.iter_mut() {
                    if *id == widget_id { continue; }
                    if let Containers::Sash(s) = container {
                        if s.sync_sashes == Some(true) {
                            s.current_sizes = updated_sizes.clone();
                        }
                    }
                }
            }

            // Fire Python callback if registered: def cb(wid: int, data: tuple[int, float])
            invoke_callback_with_args(widget_id, "on_resize", "SashH", (index, size),
                "def cb(wid: int, data: tuple[int, float])");
        },
        SashMessage::ResizedV(_id, _index, _size) => {
        }
        SashMessage::ResizedOuter(_id, new_total) => {
            // Update the resized sash's current_sizes
            let (sync_enabled, updated_sizes) = {
                let sash = match state.containers.get_mut(&widget_id) {
                    Some(Containers::Sash(s)) => s,
                    _ => return,
                };
                let min = sash.min_size.unwrap_or(0.0);
                iced_sash::apply_outer_resize(
                        &mut sash.current_sizes,
                        new_total,
                        sash.resize_mode,
                        min,
                    );
                (sash.sync_sashes == Some(true), sash.current_sizes.clone())
            };

            // Propagate to all other sashes in the sync group
            if sync_enabled {
                for (id, container) in state.containers.iter_mut() {
                    if *id == widget_id { continue; }
                    if let Containers::Sash(s) = container {
                        if s.sync_sashes == Some(true) {
                            s.current_sizes = updated_sizes.clone();
                        }
                    }
                }
            }

            // Fire Python callback if registered: def cb(wid: int, data: tuple[int, float])
            invoke_callback_with_args(widget_id, "on_resize_outer", "Sash", new_total,
                "def cb(wid: int, size: float)");
        }
        SashMessage::Released(_id, index) => {
            invoke_callback_with_args(widget_id, "on_release", "Sash", index,
                "def cb(wid: int, index: int)");
        },
        SashMessage::ResizedCrossH(_id, size) => {
            // Update the resized sash's current_sizes
            let (sync_cross_enabled, updated_size) = {
                let sash = match state.containers.get_mut(&widget_id) {
                    Some(Containers::Sash(s)) => s,
                    _ => return,
                };
                sash.size = size;
                (sash.sync_cross_sashes == Some(true), sash.size)
            };

            // Propagate to all other sashes in the sync group
            if sync_cross_enabled {
                for (id, container) in state.containers.iter_mut() {
                    if *id == widget_id { continue; }
                    if let Containers::Sash(s) = container {
                        if s.sync_cross_sashes == Some(true) {
                            s.size = updated_size;
                        }
                    }
                }
            }
        },
    }
    
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SashStyleStd {
    Primary,
    Subtle,
    Transparent,
}

#[derive(Debug, Clone)]
pub struct SashStyle {
    pub id: usize,
    pub bkg_color: Option<Color>,
    pub bkg_color_alpha: Option<f32>,
    pub bkg_rgba: Option<[f32; 4]>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_width: Option<f32>,
    pub border_radius: Option<f32>,
}

impl SashStyle {
    fn to_iced(
        &self,
        theme: &Theme,
        status: Status,
    ) -> Style {

        let mut style = iced_sash::subtle(theme, status);

        let bkg_color = Color::rgba_ipg_color_to_iced(self.bkg_rgba, &self.bkg_color, self.bkg_color_alpha);
        let bc_color = Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);

        if let Some(c) = bkg_color {
            let bkg = colors::background(c);
            style.background = match status {
                Status::Active => bkg.base.color.into(),
                Status::Hovered => bkg.weak.color.into(),
                Status::Dragged => bkg.strong.color.into(),
                Status::Disabled => bkg.weak.color.into(),
            };
        }

        if let Some(bc) = bc_color {
            style.border_color = bc;
        }

        if let Some(bw) = self.border_width {
            style.border_width = bw;
        }

        if let Some(br) = self.border_radius {
            style.border_radius = get_radius(&vec![br], "Sash".to_string())
        }

        style

    }
}



#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SashParam {
    Clip,
    InitialSizes,
    MaxSize,
    MinSize,
    OuterHandleSize,
    ResizeModeLastOnly,
    ResizeModeProportional,
    ResizeModeUniform,
    SashSize,
    Show,
    StyleId,
    SyncSashes,
    SyncCrossSashes,
    VerticalDirection,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SashStyleParam {
    BkgColor,
    BkgColorAlpha,
    BkgRgba,
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
    BorderWidth,
    BorderRadius,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Sash {
    type Param = SashParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SashParam::Clip => set_t_value(&mut self.clip, value, "SashParam::Clip"),
            SashParam::InitialSizes => set_t_value(&mut self.initial_sizes, value, "SashParam::InitialSizes"),
            SashParam::MaxSize => set_t_value(&mut self.max_size, value, "SashParam::MaxSize"),
            SashParam::MinSize => set_t_value(&mut self.min_size, value, "SashParam::MinSize"),
            SashParam::OuterHandleSize => set_t_value(&mut self.outer_handle_size, value, "SashParam::OuterHandleSize"),
            SashParam::ResizeModeLastOnly => {
                set_t_value(&mut self.resize_mode_last, value, "SashParam::ResizeModeLastOnly");
                self.resize_mode = OuterResizeMode::LastOnly;
            },
            SashParam::ResizeModeProportional => {
                set_t_value(&mut self.resize_mode_proportional, value, "SashParam::ResizeModeProportional");
                self.resize_mode = OuterResizeMode::Proportional;
            },
            SashParam::ResizeModeUniform => {
                set_t_value(&mut self.resize_mode_uniform, value, "SashParam::ResizeModeUniform");
                self.resize_mode = OuterResizeMode::Uniform;
            },
            SashParam::SashSize => set_t_value(&mut self.sash_size, value, "SashParam::SashSize"),
            SashParam::Show => set_t_value(&mut self.show, value, "SashParam::Show"),
            SashParam::StyleId => set_t_value(&mut self.style_id, value, "SashParam::StyleId"),
            SashParam::SyncSashes => set_t_value(&mut self.sync_sashes, value, "SashParam::SyncSashes"),
            SashParam::SyncCrossSashes => set_t_value(&mut self.sync_cross_sashes, value, "SashParam::SyncCrossSashes"),
            SashParam::VerticalDirection => set_t_value(&mut self.vertical_direction, value, "SashParam::VerticalDirection"),
        }
    }
}

impl WidgetParamUpdate for SashStyle {
    type Param = SashStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SashStyleParam::BkgColor => set_t_value(&mut self.bkg_color, value, "name"),
            SashStyleParam::BkgColorAlpha => set_t_value(&mut self.bkg_color_alpha, value, "name"),
            SashStyleParam::BkgRgba => set_t_value(&mut self.bkg_rgba, value, "name"),
            SashStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "name"),
            SashStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "name"),
            SashStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "name"),
            SashStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "name"),
            SashStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "name"),
        }
    }
}
