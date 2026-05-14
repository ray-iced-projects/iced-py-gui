//! ipg_divider
use std::collections::HashMap;

use crate::graphics::colors::Color;
use crate::{IpgState, app};
use crate::py_api::helpers::get_radius;
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::ipg_widgets::ipg_divider::divider::{self, Direction, Status, Style, 
    divider_horizontal, divider_vertical};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};

use iced::{Background, Element, Theme};
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone, PartialEq)]
pub struct Divider {
    pub id: usize,
    pub show: bool,
    pub direction: DividerDirection,
    pub sizes: Vec<f32>,
    pub handle_width: f32,
    pub handle_height: f32,
    pub handle_offsets: Option<Vec<f32>>,
    pub include_last_handle: bool,
    pub index_in_use: usize,
    pub value_in_use: f32,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum DivMessage {
    OnChange((usize, usize, f32)),
    OnRelease,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum DividerDirection {
    /// Horizontal resizing
    Horizontal,
    /// Vertical resizing
    Vertical,
}

impl Divider {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>,
    )-> Option<Element<'a, app::Message>> {

        if !self.show { return None }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_divider_style).cloned();

        let is_horizontal = self.direction == DividerDirection::Horizontal;

        let offsets = match self.handle_offsets.clone() {
            Some(offsets) => offsets,
            None => {
                if is_horizontal {
                    let mut offsets = vec![-self.handle_width/2.0; self.sizes.len()-1];
                    offsets.extend([-self.handle_width]);
                    offsets
                } else {
                    let mut offsets = vec![-self.handle_height/2.0; self.sizes.len()-1];
                    offsets.extend([-self.handle_height]);
                    offsets
                }
            }
        };
        
        let div: Element<DivMessage, Theme> = if is_horizontal {
            divider_horizontal(
                self.id,
                self.sizes.clone(),
                self.handle_width,
                self.handle_height, 
                DivMessage::OnChange
                )
                .on_release(DivMessage::OnRelease)
                .direction(Direction::Horizontal)
                .handle_offsets(offsets)
                .include_last_handle(self.include_last_handle)
                .style(move |theme: &Theme, status| {
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, status)
                    } else {
                        divider::primary(theme, status)
                    }
                })
                .into()
        } else {
            divider_vertical(
                self.id,
                self.sizes.clone(),
                self.handle_width,
                self.handle_height, 
                DivMessage::OnChange
                )
                .on_release(DivMessage::OnRelease)
                .direction(Direction::Vertical)
                .handle_offsets(offsets)
                .include_last_handle(self.include_last_handle)
                .style(move |theme: &Theme, status| {
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, status)
                    } else {
                       divider::primary(theme, status)
                    }
                }).into()
        };

        Some(div.map(move |message| app::Message::Divider(self.id, message)))

    }

}

pub fn divider_callback(state: &mut IpgState, id: usize, message: DivMessage) {
    match message {
        DivMessage::OnChange((widget_id, index, value)) => {
            if let Some(Widgets::Divider(div)) = state.widgets.get_mut(&widget_id) {
                div.index_in_use = index;
                div.value_in_use = value;
            }
            invoke_callback_with_args(widget_id, "on_change", "Divider", (index, value),
                "def cb(wid: int, data: tuple[int, float])");
        },
        DivMessage::OnRelease => {
            let (index, value) = if let Some(Widgets::Divider(div)) = state.widgets.get(&id) {
                (div.index_in_use, div.value_in_use)
            } else {
                (0, 0.0)
            };
            invoke_callback_with_args(id, "on_release", "Divider", (index, value),
                "def cb(wid: int, data: tuple[int, float])");
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DividerStyle {
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

impl DividerStyle {
    fn to_iced(
        &self,
        theme: &Theme, 
        status: Status,
    ) -> Style {

        if self.background_transparent == Some(true) {
            return divider::transparent(theme, status);
        }

        let background_color = 
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
        let background_color_hovered = 
            Color::rgba_ipg_color_to_iced(self.background_rgba_hovered, &self.background_color_hovered, self.background_color_hovered_alpha);
        let border_color = 
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);

        let mut base_style = divider::primary(theme, status);

        if let Some(bc) = background_color {
            base_style.background = Background::Color(bc);
        };

        if let Some(br) = &self.border_radius {
            base_style.border_radius = 
                get_radius(&br,  "Divider".to_string());
            }

        if let Some(bc) = border_color {
            base_style.border_color = bc;
        }

        if let Some(bw) = self.border_width {
            base_style.border_width = bw;
        }
        let mut hovered_style = base_style;

        if let Some(bch) = background_color_hovered {
            hovered_style.background = bch.into();
        }

        match status 
        {
            Status::Active => base_style,
            Status::Hovered => hovered_style,
            Status::Dragged => base_style, // active and drag are same
        }


    }
}
#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum DividerStyleParam {
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

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum DividerParam {
    Direction,
    HandleHeight,
    HandleOffsets,
    HandleWidth,
    IncludeLastHandle,
    Show,
    Sizes,
    StyleId,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Divider {
    type Param = DividerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            DividerParam::Direction => set_t_value(&mut self.direction, value, "DividerParam::Direction"),
            DividerParam::HandleHeight => set_t_value(&mut self.handle_height, value, "DividerParam::HandleHeight"),
            DividerParam::HandleOffsets => set_t_value(&mut self.handle_offsets, value, "DividerParam::HandleOffsets"),
            DividerParam::HandleWidth => set_t_value(&mut self.handle_width, value, "DividerParam::HandleWidth"),
            DividerParam::IncludeLastHandle => set_t_value(&mut self.include_last_handle, value, "DividerParam::IncludeLastHandle"),
            DividerParam::Show => set_t_value(&mut self.show, value, "DividerParam::Show"),
            DividerParam::Sizes => set_t_value(&mut self.sizes, value, "DividerParam::Sizes"),
            DividerParam::StyleId => set_t_value(&mut self.style_id, value, "DividerParam::StyleId"),
        }
    }
}

impl WidgetParamUpdate for DividerStyle {
    type Param = DividerStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            DividerStyleParam::BackgroundColor => set_t_value(&mut self.background_color, value, "DividerStyleParam::BackgroundColor"),
            DividerStyleParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "DividerStyleParam::BackgroundColorAlpha"),
            DividerStyleParam::BackgroundColorHovered => set_t_value(&mut self.background_color_hovered, value, "DividerStyleParam::BackgroundColorHovered"),
            DividerStyleParam::BackgroundColorHoveredAlpha => set_t_value(&mut self.background_color_hovered_alpha, value, "DividerStyleParam::BackgroundColorHoveredAlpha"),
            DividerStyleParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "DividerStyleParam::BackgroundRgbaColor"),
            DividerStyleParam::BackgroundRgbaHovered => set_t_value(&mut self.background_rgba_hovered, value, "DividerStyleParam::BackgroundRgbaHovered"),
            DividerStyleParam::BackgroundTransparent => set_t_value(&mut self.background_transparent, value, "DividerStyleParam::BackgroundTransparent"),
            DividerStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "DividerStyleParam::BorderColor"),
            DividerStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "DividerStyleParam::BorderColorAlpha"),
            DividerStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "DividerStyleParam::BorderRadius"),
            DividerStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "DividerStyleParam::BorderRgbaColor"),
            DividerStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "DividerStyleParam::BorderWidth"),
        }
    }
}
