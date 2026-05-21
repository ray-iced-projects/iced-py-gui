use iced::Element;

use iced_sash::{Id, OuterResizeMode, SashH, SashV};
pub use iced_sash::resize as sash_resize;

use crate::app::Message;
use crate::state::{Containers, IpgState};
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
    pub style_id: Option<usize>,
    pub show: bool,
    pub resize_mode: OuterResizeMode,
}

impl Sash {
    pub fn construct<'a>(
        &self,
        content: Vec<Element<'a, Message>> 
    ) -> Option<Element<'a, Message>> {

        if !self.show { return None }

        let widget_id = self.id;

        let sh = if self.vertical_direction == Some(true) {
            let sh = SashV::new(
                    content,
                    self.initial_sizes.clone(),
                    self.size,
                    self.sash_size,
                )
                .on_resize(move |id, index, size| Message::Sash(widget_id, SashMessage::ResizedH(id, index, size)));

            let sh = if self.sync_sashes == Some(true) {
                sh.sync_sashes(self.current_sizes.clone())
            } else { sh };
            sh
        } else {
            let sh = SashH::new(
                    content,
                    self.initial_sizes.clone(),
                    self.size,
                    self.sash_size,
                )
                .on_resize(move |id, index, size| Message::Sash(widget_id, SashMessage::ResizedH(id, index, size)))
                .on_outer_resize(move |id, size| Message::Sash(widget_id, SashMessage::ResizedOuter(id, size)));

            let sh = if self.sync_sashes == Some(true) {
                sh.sync_sashes(self.current_sizes.clone())
            } else { sh };

            let sh = if let Some(sz) = self.outer_handle_size {
                sh.outer_handle(sz)
            } else { sh };

            // default so set first
            let sh = sh.outer_resize_mode(iced_sash::OuterResizeMode::LastOnly);

            let sh = if self.resize_mode_proportional == Some(true) {
                sh.outer_resize_mode(iced_sash::OuterResizeMode::Proportional)
            } else { sh };

            let sh = if self.resize_mode_uniform == Some(true) {
                sh.outer_resize_mode(iced_sash::OuterResizeMode::Uniform)
            } else { sh };

            sh
        };

        Some(sh.into())
    }
}


#[derive(Debug, Clone)]
pub enum SashMessage {
    ResizedH(Id, usize, f32),
    ResizedV(Id, usize, f32),
    ResizedOuter(Id, f32),
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
                let min = sash.min_size.unwrap_or(50.0);
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
        SashMessage::Released(_id, _index) => todo!(),
        
        }
    
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SashParam {
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
    None
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Sash {
    type Param = SashParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
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

// impl WidgetParamUpdate for SashStyle {
//     type Param = SashStyleParam;

//     fn param_update(&mut self, param: Self::Param, value: &PyObject) {
//         match param {
//         }
//     }
// }
