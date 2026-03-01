//! Button module - provides add_button pyfunction

use iced::Color;
use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::{colors::IpgColor, bootstrap_arrow::IpgArrow};
use crate::py_api::helpers::{get_height, get_width};
use crate::state::{IpgWidgets, access_state, add_callback_to_mutex, 
    get_id, set_state_of_widget};
use crate::widgets::enums::{IpgAlignmentX, 
    IpgAlignmentY};
use crate::widgets::ipg_button::{IpgButton,  
    IpgButtonStyle, IpgButtonStyleStandard};


/// Add a button widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    parent_id,
    label=None,
    gen_id=None,
    on_press=None,
    width=None,
    height=None,
    width_fill=false,
    height_fill=false,
    padding=None,
    text_align_x=None,
    text_align_y=None,
    text_size=None,
    clip=None,
    style_id=None,
    style_standard=None,
    style_arrow=None,
    user_data=None,
    show=true
))]
pub fn add_button(
    parent_id: String,
    label: Option<String>,
    gen_id: Option<usize>,
    on_press: Option<PyObject>,
    width: Option<f32>,
    height: Option<f32>,
    width_fill: bool,
    height_fill: bool,
    padding: Option<Vec<f32>>,
    text_align_x: Option<IpgAlignmentX>,
    text_align_y: Option<IpgAlignmentY>,
    text_size: Option<f32>,
    clip: Option<bool>,
    style_id: Option<usize>,
    style_standard: Option<IpgButtonStyleStandard>,
    style_arrow: Option<IpgArrow>,
    user_data: Option<PyObject>,
    show: bool,
) -> PyResult<usize> {

    let id = get_id(gen_id);
    
    // Calculate dimensions
    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

    // Register widget with parent
    set_state_of_widget(id, parent_id.clone());

    // Store callback if provided
    if let Some(py) = on_press {
        add_callback_to_mutex(id, "on_press".to_string(), py);
    }

    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    // Create and store button
    let mut state = access_state();
    state.widgets.insert(
        id,
        IpgWidgets::IpgButton(
            IpgButton {
                id,
                parent_id,
                show,
                label,
                width,
                height,
                padding,
                text_align_x,
                text_align_y,
                text_size,
                clip,
                style_id,
                style_standard,
                style_arrow,
            }),
        );
    drop(state);

    Ok(id)
}

#[pyfunction]
#[pyo3(signature = (
        background_color=None, 
        background_rgba=None,
        background_gradient_color_stop=None,
        background_gradient_rgba_stop=None,
        background_gradient_degrees=None,
        background_gradient_radians=None,
        background_gradient_alpha=None,
        border_color=None, 
        border_rgba=None,
        border_radius=None, 
        border_width=1.0,
        shadow_color=None, 
        shadow_rgba=None,
        shadow_offset_xy=None, 
        shadow_blur_radius=None,
        text_color=None, 
        text_rgba=None,
        gen_id=None
        ))]
pub fn add_button_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    background_gradient_color_stop: Option<IpgColor>,
    background_gradient_rgba_stop: Option<[f32; 4]>,
    background_gradient_degrees: Option<f32>,
    background_gradient_radians: Option<f32>,
    background_gradient_alpha: Option<f32>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    shadow_color: Option<IpgColor>,
    shadow_rgba: Option<[f32; 4]>,
    shadow_offset_xy: Option<[f32; 2]>,
    shadow_blur_radius: Option<f32>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let grad_a = background_gradient_alpha.unwrap_or(1.0);

    let background_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let background_gradient_color_stop: Option<Color> =
        IpgColor::rgba_ipg_color_to_iced(background_gradient_rgba_stop, background_gradient_color_stop, grad_a, false);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0, false);
    let shadow_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(shadow_rgba, shadow_color, 1.0, false);
    let text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, 1.0, false);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgButtonStyle(
        IpgButtonStyle {
            id,
            background_color,
            background_gradient_color_stop,
            background_gradient_degrees,
            background_gradient_radians,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_xy,
            shadow_blur_radius,
            text_color,
        }));

    drop(state);
    Ok(id)
}
