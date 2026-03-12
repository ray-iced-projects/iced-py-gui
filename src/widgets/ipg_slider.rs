//! ipg_slider
use iced::widget::slider::{self, HandleShape, Status, Style};
use iced::{Background, Color, Element, Length, Theme, border};
use iced::widget::Slider;

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

use crate::py_api::helpers::get_radius;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool, set_f32, set_opt_f32, set_opt_iced_color, set_opt_iced_color_from_rgba, set_opt_u16, set_opt_usize, set_opt_vec_f32, set_width};
use crate::{IpgState, app};
use crate::state::IpgWidgets;
use crate::widgets::callbacks::invoke_callback_with_args;





#[derive(Debug, Clone)]
pub struct IpgSlider {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub shift_step: Option<f32>,
    pub value: f32,
    pub width: Length,
    pub height: f32,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct IpgSliderStyle {
    pub id: usize,
    pub rail_color: Option<Color>,
    pub rail_color_hovered: Option<Color>,
    pub rail_width: Option<f32>,
    pub rail_border_radius: Option<Vec<f32>>,
    pub handle_circle_radius: Option<f32>,
    pub handle_rectangle_width: Option<u16>,
    pub handle_rectangle_border_radius: Option<Vec<f32>>,
    pub handle_color: Option<Color>,
    pub handle_border_width: Option<f32>,
    pub handle_border_color: Option<Color>,
}


#[derive(Debug, Clone)]
pub enum SLMessage {
    OnChange(f32),
    OnRelease,
}

pub fn construct_slider<'a>(slider: &'a IpgSlider, 
                        style_opt: Option<&IpgWidgets>) 
                        -> Option<Element<'a, app::Message>> {

    if !slider.show {
        return None
    }

    let style = style_opt.and_then(IpgWidgets::as_slider_style).cloned();

    let sld: Element<SLMessage, Theme> = 
        Slider::new(slider.min..=slider.max, 
                    slider.value, 
                    SLMessage::OnChange
                    )
                    .on_release(SLMessage::OnRelease)
                    .step(slider.step)
                    .width(slider.width)
                    .height(slider.height)
                    .style(move|theme, status|
                    get_styling(theme, status,
                        style.clone()
                    ))
                    .into();

    Some(sld.map(move |message| app::Message::Slider(slider.id, message)))
}

pub fn slider_callback(state: &mut IpgState, id: usize, message: SLMessage) {
    match message {
        SLMessage::OnChange(value) => {
            // Update widget state directly
            if let Some(IpgWidgets::IpgSlider(slider)) = state.widgets.get_mut(&id) {
                slider.value = value;
            }
            invoke_callback_with_args(id, "on_change", "Slider", value);
        },
        SLMessage::OnRelease => {
            // Get current value from widget state
            let value = state.widgets.get(&id)
                .and_then(IpgWidgets::as_slider)
                .map(|s| s.value)
                .unwrap_or(0.0);
            invoke_callback_with_args(id, "on_release", "Slider", value);
        },
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSliderParam {
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

fn get_styling(theme: &Theme, 
                status: Status,
                style_opt: Option<IpgSliderStyle>) 
                -> Style {

    if style_opt.is_none() {
        return slider::default(theme, status)
    }     

    let style = style_opt.unwrap();

    let mut base_style = slider::default(theme, status);

    if let Some(c) = style.handle_color {
        base_style.handle.background = Background::Color(c);
    };


    if let Some(rc) = style.rail_color {
        base_style.rail.backgrounds = 
            (Background::Color(rc), Background::Color(rc));
    }

    if let Some(rw) = style.rail_width {
        base_style.rail.width = rw;
    }

    if let Some(br) = style.rail_border_radius {
        base_style.rail.border.radius = 
            get_radius(&br, "Slider".to_string());
    }

    if let Some(hcr) = style.handle_circle_radius {
        base_style.handle.shape = HandleShape::Circle{radius: hcr };
    }

    match (style.handle_rectangle_width, style.handle_rectangle_border_radius) {
        (Some(hrw), Some(br)) => {
            base_style.handle.shape = HandleShape::Rectangle {
                width: hrw,
                border_radius: get_radius(&br, "Slider".to_string()),
            };
        }
        (Some(hrw), None) => {
            base_style.handle.shape = HandleShape::Rectangle {
                width: hrw,
                border_radius: border::Radius::default(),
            };
        }
        (None, Some(br)) => {
            // Get current width if shape is already a Rectangle, otherwise use default
            let current_width = match base_style.handle.shape {
                HandleShape::Rectangle { width, .. } => width,
                _ => 8,
            };
            base_style.handle.shape = HandleShape::Rectangle {
                width: current_width,
                border_radius: get_radius(&br, "Slider".to_string()),
            };
        }
        (None, None) => {}
    }

    if let Some(hbc) = style.handle_border_color {
        base_style.handle.border_color = hbc;
    }

    if let Some(hbw) = style.handle_border_width {
        base_style.handle.border_width = hbw;
    }

    let mut hovered_style = base_style;

    if let Some(rch) = style.rail_color_hovered {
        hovered_style.rail.border.color = rch;
    }

    match status 
    {
        Status::Active => base_style,
        Status::Hovered => hovered_style,
        Status::Dragged => base_style, // active and drag are same
    }


}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSliderStyleParam {
    RailIpgColor,
    RailRbgaColor,
    RailIpgColorHovered,
    RailIpgRgbaHovered,
    RailBorderRadius,
    RailWidth,

    HandleIpgColor,
    HandleRgbaColor,
    HandleBorderIpgColor,
    HandleBorderRgbaColor,
    HandleBorderWidth,
    HandleCircleRadius,
    HandleRectangleWidth,
    HandleRectangleBorderRadius,
}



// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgSlider {
    type Param = IpgSliderParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgSliderParam::Min => set_f32(&mut self.min, value, "Min"),
            IpgSliderParam::Max => set_f32(&mut self.max, value, "Max"),
            IpgSliderParam::Step => set_f32(&mut self.step, value, "Step"),
            IpgSliderParam::Value => todo!(),
            IpgSliderParam::Width => set_width(&mut self.width, value, "Width"),
            IpgSliderParam::WidthFill => set_width(&mut self.width, value, "WidthFill"),
            IpgSliderParam::Height => set_f32(&mut self.height, value, "Height"),
            IpgSliderParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            IpgSliderParam::Show => set_bool(&mut self.show, value, "Show"),
        }
    }
}

impl WidgetParamUpdate for IpgSliderStyle {
    type Param = IpgSliderStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgSliderStyleParam::RailIpgColor => 
                set_opt_iced_color(&mut self.rail_color, value, "RailIpgColor"),
            IpgSliderStyleParam::RailRbgaColor => 
                set_opt_iced_color_from_rgba(&mut self.rail_color, value, "RailRbgaColor"),
            IpgSliderStyleParam::RailIpgColorHovered => 
                set_opt_iced_color(&mut self.rail_color_hovered, value, "RailIpgColorHovered"),
            IpgSliderStyleParam::RailIpgRgbaHovered => 
                set_opt_iced_color_from_rgba(&mut self.rail_color_hovered, value, "RailIpgRgbaHovered"),
            IpgSliderStyleParam::RailBorderRadius => 
                set_opt_vec_f32(&mut self.rail_border_radius, value, "RailBorderRadius"),
            IpgSliderStyleParam::RailWidth => 
                set_opt_f32(&mut self.rail_width, value, "RailWidth"),
            IpgSliderStyleParam::HandleIpgColor => 
                set_opt_iced_color(&mut self.handle_color, value, "HandleIpgColor"),
            IpgSliderStyleParam::HandleRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.handle_color, value, "HandleRgbaColor"),
            IpgSliderStyleParam::HandleBorderIpgColor => 
                set_opt_iced_color(&mut self.handle_border_color, value, "HandleBorderIpgColor"),
            IpgSliderStyleParam::HandleBorderRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.handle_border_color, value, "HandleBorderRgbaColor"),
            IpgSliderStyleParam::HandleBorderWidth => 
                set_opt_f32(&mut self.handle_border_width, value, "HandleBorderWidth"),
            IpgSliderStyleParam::HandleCircleRadius => 
                set_opt_f32(&mut self.handle_circle_radius, value, "HandleCircleRadius"),
            IpgSliderStyleParam::HandleRectangleWidth => 
                set_opt_u16(&mut self.handle_rectangle_width, value, "HandleRectangleWidth"),
            IpgSliderStyleParam::HandleRectangleBorderRadius => 
                set_opt_vec_f32(&mut self.handle_rectangle_border_radius, value, "HandleRectangleBorderRadius"),
        }
    }
}
