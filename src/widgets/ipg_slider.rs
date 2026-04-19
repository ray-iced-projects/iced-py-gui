//! ipg_slider

use std::collections::HashMap;

use iced::widget::slider::{self, HandleShape, Status, Style};
use iced::{Background, Element, Theme, border};
use iced::widget;

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

use crate::graphics::colors::Color;
use crate::py_api::helpers::{get_len, get_radius};
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};
use crate::{IpgState, app};
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback_with_args;


#[derive(Debug, Clone)]
pub struct Slider {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub shift_step: Option<f32>,
    pub value: f32,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub style_id: Option<usize>,
}

impl Slider {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self, 
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, app::Message>> {

        if !self.show { return None }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_slider_style).cloned();

        let sld = 
            widget::Slider::new(self.min..=self.max, 
                        self.value, 
                        SldMessage::OnChange
                        )
                        .on_release(SldMessage::OnRelease)
                        .step(self.step)
                        .width(get_len(None, self.width_fill, self.width))
                        .height(self.height.unwrap_or(16.0))
                        .style(move|theme, status| {
                            if let Some(st) = style_opt.clone() {
                                st.to_iced(theme, status)
                            } else {
                                iced::widget::slider::default(theme, status)
                            }
                        });
        
        let sld = if let Some(shift) = self.shift_step {
            sld.shift_step(shift)
        } else { sld };

        let sld: Element<'_, SldMessage> = sld.into();

        Some(sld.map(move |message| app::Message::Slider(self.id, message)))
    }
}
#[derive(Debug, Clone)]
pub enum SldMessage {
    OnChange(f32),
    OnRelease,
}

pub fn slider_callback(state: &mut IpgState, id: usize, message: SldMessage) {
    match message {
        SldMessage::OnChange(value) => {
            if let Some(Widgets::Slider(cb)) = state.widgets.get_mut(&id) {
                cb.value = value;
            }
            invoke_callback_with_args(id, "on_change", "Slider", value,
                "def cb(wid: int, value: float)");
        },
        SldMessage::OnRelease => {
            if let Some(Widgets::Slider(cb)) = state.widgets.get_mut(&id) {
                invoke_callback_with_args(id, "on_release", "Slider", cb.value,
                    "def cb(wid: int, value: float)");
            }
        },
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SliderParam {
    Min,
    Max,
    Step,
    Value,
    Width,
    WidthFill,
    Height,
    StyleId,
    Show,
}

#[derive(Debug, Clone)]
pub struct SliderStyle {
    pub id: usize,
    pub rail_color: Option<Color>,
    pub rail_color_alpha: Option<f32>,
    pub rail_rgba: Option<[f32; 4]>,
    pub rail_color_hovered: Option<Color>,
    pub rail_color_hovered_alpha: Option<f32>,
    pub rail_rgba_hovered: Option<[f32; 4]>,
    pub rail_width: Option<f32>,
    pub rail_border_radius: Option<Vec<f32>>,
    pub handle_circle_radius: Option<f32>,
    pub handle_rectangle_width: Option<u16>,
    pub handle_rectangle_border_radius: Option<Vec<f32>>,
    pub handle_color: Option<Color>,
    pub handle_color_alpha: Option<f32>,
    pub handle_rgba: Option<[f32; 4]>,
    pub handle_border_width: Option<f32>,
    pub handle_border_color: Option<Color>,
    pub handle_border_color_alpha: Option<f32>,
    pub handle_border_rgba: Option<[f32; 4]>,
}

impl SliderStyle {
    fn to_iced (
        &self,
        theme: &Theme, 
        status: Status,
    ) -> Style {
   
        let mut style = slider::default(theme, status);

        let rail_color = 
            Color::rgba_ipg_color_to_iced(self.rail_rgba, &self.rail_color, self.rail_color_alpha);
        let rail_color_hovered = 
            Color::rgba_ipg_color_to_iced(self.rail_rgba_hovered, &self.rail_color_hovered, self.rail_color_hovered_alpha);
        let handle_color = 
            Color::rgba_ipg_color_to_iced(self.handle_rgba, &self.handle_color, self.handle_color_alpha);
        let handle_border_color = 
            Color::rgba_ipg_color_to_iced(self.handle_border_rgba, &self.handle_border_color, self.handle_border_color_alpha);

        if let Some(c) = handle_color {
            style.handle.background = Background::Color(c);
        };


        if let Some(rc) = rail_color {
            style.rail.backgrounds = 
                (Background::Color(rc), Background::Color(rc));
        }

        if let Some(rw) = self.rail_width {
            style.rail.width = rw;
        }

        if let Some(br) = &self.rail_border_radius {
            style.rail.border.radius = 
                get_radius(&br, "Slider".to_string());
        }

        if let Some(hcr) = self.handle_circle_radius {
            style.handle.shape = HandleShape::Circle{radius: hcr };
        }

        match (self.handle_rectangle_width, &self.handle_rectangle_border_radius) {
            (Some(hrw), Some(br)) => {
                style.handle.shape = HandleShape::Rectangle {
                    width: hrw,
                    border_radius: get_radius(&br, "Slider".to_string()),
                };
            }
            (Some(hrw), None) => {
                style.handle.shape = HandleShape::Rectangle {
                    width: hrw,
                    border_radius: border::Radius::default(),
                };
            }
            (None, Some(br)) => {
                // Get current width if shape is already a Rectangle, otherwise use default
                let current_width = match style.handle.shape {
                    HandleShape::Rectangle { width, .. } => width,
                    _ => 8,
                };
                style.handle.shape = HandleShape::Rectangle {
                    width: current_width,
                    border_radius: get_radius(&br, "Slider".to_string()),
                };
            }
            (None, None) => {}
        }

        if let Some(hbc) = handle_border_color {
            style.handle.border_color = hbc;
        }

        if let Some(hbw) = self.handle_border_width {
            style.handle.border_width = hbw;
        }

        let mut hovered_style = style;

        if let Some(rch) = rail_color_hovered {
            hovered_style.rail.border.color = rch;
        }

        match status 
        {
            Status::Active => style,
            Status::Hovered => hovered_style,
            Status::Dragged => style, // active and drag are same
        }

}

}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum SliderStyleParam {
    RailColor,
    RailColorAlpha,
    RailRgba,
    RailColorHovered,
    RailRgbaHovered,
    RailBorderRadius,
    RailWidth,

    HandleColor,
    HandleColorAlpha,
    HandleRgba,
    HandleBorderColor,
    HandleBorderColorAlpha,
    HandleBorderRgba,
    HandleBorderWidth,
    HandleCircleRadius,
    HandleRectangleWidth,
    HandleRectangleBorderRadius,
}



// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Slider {
    type Param = SliderParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SliderParam::Min => set_t_value(&mut self.min, value, "Min"),
            SliderParam::Max => set_t_value(&mut self.max, value, "Max"),
            SliderParam::Step => set_t_value(&mut self.step, value, "Step"),
            SliderParam::Value => set_t_value(&mut self.value, value, "SliderParam::Value"),
            SliderParam::Width => set_t_value(&mut self.width, value, "Width"),
            SliderParam::WidthFill => set_t_value(&mut self.width_fill, value, "WidthFill"),
            SliderParam::Height => set_t_value(&mut self.height, value, "Height"),
            SliderParam::StyleId => set_t_value(&mut self.style_id, value, "StyleId"),
            SliderParam::Show => set_t_value(&mut self.show, value, "Show"),
        }
    }
}

impl WidgetParamUpdate for SliderStyle {
    type Param = SliderStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            SliderStyleParam::RailColor => set_t_value(&mut self.rail_color, value, "SliderStyleParam::RailColor"),
            SliderStyleParam::RailColorAlpha => set_t_value(&mut self.rail_color_alpha, value, "SliderStyleParam::RailColorAlpha"),
            SliderStyleParam::RailRgba => set_t_value(&mut self.rail_rgba, value, "SliderStyleParam::RailRbga"),
            SliderStyleParam::RailColorHovered => set_t_value(&mut self.rail_color_hovered, value, "SliderStyleParam::RailColorHovered"),
            SliderStyleParam::RailRgbaHovered => set_t_value(&mut self.rail_rgba_hovered, value, "SliderStyleParam::RailRgbaHovered"),
            SliderStyleParam::RailBorderRadius => set_t_value(&mut self.rail_border_radius, value, "SliderStyleParam::RailBorderRadius"),
            SliderStyleParam::RailWidth => set_t_value(&mut self.rail_width, value, "SliderStyleParam::RailWidth"),
            SliderStyleParam::HandleColor => set_t_value(&mut self.handle_color, value, "SliderStyleParam::HandleColor"),
            SliderStyleParam::HandleColorAlpha => set_t_value(&mut self.handle_color_alpha, value, "SliderStyleParam::HandleColorAlpha"),
            SliderStyleParam::HandleRgba => set_t_value(&mut self.handle_rgba, value, "SliderStyleParam::HandleRgba"),
            SliderStyleParam::HandleBorderColor => set_t_value(&mut self.handle_border_color, value, "SliderStyleParam::HandleBorderColor"),
            SliderStyleParam::HandleBorderColorAlpha => set_t_value(&mut self.handle_border_color_alpha, value, "SliderStyleParam::HandleBorderColorAlpha"),
            SliderStyleParam::HandleBorderRgba => set_t_value(&mut self.handle_border_rgba, value, "SliderStyleParam::HandleBorderRgba"),
            SliderStyleParam::HandleBorderWidth => set_t_value(&mut self.handle_border_width, value, "SliderStyleParam::HandleBorderWidth"),
            SliderStyleParam::HandleCircleRadius => set_t_value(&mut self.handle_circle_radius, value, "SliderStyleParam::HandleCircleRadius"),
            SliderStyleParam::HandleRectangleWidth => set_t_value(&mut self.handle_rectangle_width, value, "SliderStyleParam::HandleRectangleWidth"),
            SliderStyleParam::HandleRectangleBorderRadius => set_t_value(&mut self.handle_rectangle_border_radius, value, "SliderStyleParam::HandleRectangleBorderRadius"),
        }
    }
}
