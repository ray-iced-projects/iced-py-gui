//! ipg_slider
use iced::widget::slider::{self, HandleShape, Status, Style};
use iced::{Background, Color, Element, Length, Theme, border};
use iced::widget::Slider;

use pyo3::{Py, PyAny, pyclass, Python};
type PyObject = Py<PyAny>;

use crate::py_api::helpers::get_radius;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool, set_f32, set_iced_color_from_rgba, set_opt_f32, set_opt_iced_color, set_opt_u16, set_opt_usize, set_opt_vec_f32, set_width};
use crate::{IpgState, access_callbacks, access_user_data1, app};
use crate::state::IpgWidgets;
use crate::widgets::callbacks::{WidgetCallbackIn, set_or_get_widget_callback_data};





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

    let style = get_slider_style(style_opt);

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

    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
           
    match message {
        SLMessage::OnChange(value) => {
            wci.value_f64 = Some(value as f64);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(id, "on_change".to_string(), value);
        },
        SLMessage::OnRelease => {
            // to be consistent, returning value for both
            let wco = set_or_get_widget_callback_data(state, wci);
            process_callback(id, "on_release".to_string(), wco.value_f32.unwrap());
        },
    }
}

pub fn process_callback(
        id: usize, 
        event_name: String, 
        value: f32) 
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
            if let Err(err) = callback.call1(py, (id, value, user_data)) {
                panic!("Slider callback error: {err}");
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
    //         if let Err(err) = callback.call1(py, (id, value, user_data)) {
    //             panic!("Slider callback error: {err}");
    //         }
    //     });
    //     drop(ud2); // Drop ud2 after processing
    //     return;
    // }
    // drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only the id and value
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, value)) {
            panic!("Slider callback error: {err}");
        }
    });

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

fn get_slider_style(style: Option<&IpgWidgets>) -> Option<IpgSliderStyle>{
    match style {
        Some(IpgWidgets::IpgSliderStyle(style)) => {
            Some(style.clone())
        }
            _ => None,
        }
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgSlider {
    type Param = IpgSliderParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgSliderParam::Min => set_f32(&mut self.min, value, name),
            IpgSliderParam::Max => set_f32(&mut self.max, value, name),
            IpgSliderParam::Step => set_f32(&mut self.step, value, name),
            IpgSliderParam::Value => todo!(),
            IpgSliderParam::Width => set_width(&mut self.width, value, name),
            IpgSliderParam::WidthFill => set_width(&mut self.width, value, name),
            IpgSliderParam::Height => set_f32(&mut self.height, value, name),
            IpgSliderParam::StyleId => set_opt_usize(&mut self.style_id, value, name),
            IpgSliderParam::Show => set_bool(&mut self.show, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgSliderStyle {
    type Param = IpgSliderStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgSliderStyleParam::RailIpgColor => 
                set_opt_iced_color(&mut self.rail_color, value, name),
            IpgSliderStyleParam::RailRbgaColor => 
                set_iced_color_from_rgba(&mut self.rail_color, value, name),
            IpgSliderStyleParam::RailIpgColorHovered => 
                set_opt_iced_color(&mut self.rail_color_hovered, value, name),
            IpgSliderStyleParam::RailIpgRgbaHovered => 
                set_iced_color_from_rgba(&mut self.rail_color_hovered, value, name),
            IpgSliderStyleParam::RailBorderRadius => 
                set_opt_vec_f32(&mut self.rail_border_radius, value, name),
            IpgSliderStyleParam::RailWidth => 
                set_opt_f32(&mut self.rail_width, value, name),
            IpgSliderStyleParam::HandleIpgColor => 
                set_opt_iced_color(&mut self.handle_color, value, name),
            IpgSliderStyleParam::HandleRgbaColor => 
                set_iced_color_from_rgba(&mut self.handle_color, value, name),
            IpgSliderStyleParam::HandleBorderIpgColor => 
                set_opt_iced_color(&mut self.handle_border_color, value, name),
            IpgSliderStyleParam::HandleBorderRgbaColor => 
                set_iced_color_from_rgba(&mut self.handle_border_color, value, name),
            IpgSliderStyleParam::HandleBorderWidth => 
                set_opt_f32(&mut self.handle_border_width, value, name),
            IpgSliderStyleParam::HandleCircleRadius => 
                set_opt_f32(&mut self.handle_circle_radius, value, name),
            IpgSliderStyleParam::HandleRectangleWidth => 
                set_opt_u16(&mut self.handle_rectangle_width, value, name),
            IpgSliderStyleParam::HandleRectangleBorderRadius => 
                set_opt_vec_f32(&mut self.handle_rectangle_border_radius, value, name),
        }
    }
}
