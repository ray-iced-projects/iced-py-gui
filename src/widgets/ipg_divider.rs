//! ipg_divider
use std::collections::HashMap;

use crate::{IpgState, app};
use crate::py_api::helpers::get_radius;
use crate::state::IpgWidgets;
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::divider::{self, Direction, Status, Style, 
    divider_horizontal, divider_vertical};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_f32, 
    set_opt_bool, set_opt_f32, set_opt_usize, 
    set_opt_vec_f32, set_vec_f32, set_opt_iced_color, 
    set_rgba_color_via_ipg,
};

use iced::{Background, Color, Element, Length, Theme};
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone, PartialEq)]
pub struct IpgDivider {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub direction: IpgDividerDirection,
    pub sizes: Vec<f32>,
    pub handle_width: f32,
    pub handle_height: f32,
    pub handle_offsets: Option<Vec<f32>>,
    pub include_last_handle: bool,
    pub width: Length,
    pub height: Length,
    pub index_in_use: usize,
    pub value_in_use: f32,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum DivMessage {
    OnChange((usize, usize, f32)),
    OnRelease,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgDividerDirection {
    /// Horizontal resizing
    Horizontal,
    /// Vertical resizing
    Vertical,
}

impl IpgDivider {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, IpgWidgets>,
    )-> Option<Element<'a, app::Message>> {

        if !self.show { return None }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(IpgWidgets::as_divider_style).cloned();

        let is_horizontal = self.direction == IpgDividerDirection::Horizontal;

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
                .width(self.width)
                .height(self.height)
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
                .width(self.width)
                .height(self.height)
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
            if let Some(IpgWidgets::IpgDivider(div)) = state.widgets.get_mut(&widget_id) {
                div.index_in_use = index;
                div.value_in_use = value;
            }
            invoke_callback_with_args(widget_id, "on_change", "Divider", (index, value));
        },
        DivMessage::OnRelease => {
            let (index, value) = if let Some(IpgWidgets::IpgDivider(div)) = state.widgets.get(&id) {
                (div.index_in_use, div.value_in_use)
            } else {
                (0, 0.0)
            };
            invoke_callback_with_args(id, "on_release", "Divider", (index, value));
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpgDividerStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    pub background_transparent: Option<bool>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
}

impl IpgDividerStyle {
    fn to_iced(
        &self,
        theme: &Theme, 
        status: Status,
    ) -> Style {

        if self.background_transparent == Some(true) {
            return divider::transparent(theme, status);
        }

        let mut base_style = divider::primary(theme, status);

        if let Some(bc) = self.background_color {
            base_style.background = Background::Color(bc);
        };

        if let Some(br) = &self.border_radius {
            base_style.border_radius = 
                get_radius(&br,  "Divider".to_string());
            }

        if let Some(bc) = self.border_color {
            base_style.border_color = bc;
        }

        if let Some(bw) = self.border_width {
            base_style.border_width = bw;
        }
        let mut hovered_style = base_style;

        if let Some(bch) = self.background_color_hovered {
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
#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgDividerStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BackgroundTransparent,
    BorderIpgColor,
    BorderRgbaColor,
    BorderWidth,
    BorderRadius,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgDividerParam {
    HandleWidth,
    HandleHeight,
    Sizes,
    StyleId,
    Show,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgDivider {
    type Param = IpgDividerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        let name = String::new();
        match param {
            IpgDividerParam::HandleWidth  => set_f32(&mut self.handle_width, value, name),
            IpgDividerParam::HandleHeight => set_f32(&mut self.handle_height, value, name),
            IpgDividerParam::Sizes        => set_vec_f32(&mut self.sizes, value, name),
            IpgDividerParam::StyleId      => set_opt_usize(&mut self.style_id, value, name),
            IpgDividerParam::Show         => set_bool(&mut self.show, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgDividerStyle {
    type Param = IpgDividerStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        let name = String::new();
        match param {
            IpgDividerStyleParam::BackgroundIpgColor   => set_opt_iced_color(&mut self.background_color, value, name),
            IpgDividerStyleParam::BackgroundRgbaColor  => set_rgba_color_via_ipg(&mut self.background_color, value, name),
            IpgDividerStyleParam::BackgroundTransparent => set_opt_bool(&mut self.background_transparent, value, name),
            IpgDividerStyleParam::BorderIpgColor       => set_opt_iced_color(&mut self.border_color, value, name),
            IpgDividerStyleParam::BorderRgbaColor      => set_rgba_color_via_ipg(&mut self.border_color, value, name),
            IpgDividerStyleParam::BorderWidth          => set_opt_f32(&mut self.border_width, value, name),
            IpgDividerStyleParam::BorderRadius         => set_opt_vec_f32(&mut self.border_radius, value, name),
        }
    }
}
