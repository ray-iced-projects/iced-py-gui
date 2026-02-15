//! Button module - provides add_button pyfunction
#![allow(unused)]

use iced::Color;
use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::colors::{IpgColor, get_color};
use crate::py_api::helpers::{get_height, get_padding_f64, get_width};
use crate::state::{IpgWidgets, access_state, add_callback_to_mutex, get_id, set_state_of_widget};
use crate::widgets::enums::{IpgHorizontalAlignment, IpgVerticalAlignment};
use crate::widgets::ipg_button::{IpgButton, IpgButtonArrow, IpgButtonStyle, IpgButtonStyleStandard};
use crate::widgets::styling::IpgStyleStandard;


/// Add a button widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    parent_id,
    label,
    gen_id=None,
    on_press=None,
    width=None,
    height=None,
    width_fill=false,
    height_fill=false,
    padding=vec![5.0],
    text_align_x=IpgHorizontalAlignment::Center,
    text_align_y=IpgVerticalAlignment::Center,
    text_size=16.0,
    clip=false,
    style_id=None,
    style_standard=None,
    style_arrow=None,
    user_data=None,
    show=true
))]
pub fn add_button(
    parent_id: String,
    label: String,
    gen_id: Option<usize>,
    on_press: Option<PyObject>,
    width: Option<f32>,
    height: Option<f32>,
    width_fill: bool,
    height_fill: bool,
    padding: Vec<f64>,
    text_align_x: IpgHorizontalAlignment,
    text_align_y: IpgVerticalAlignment,
    text_size: f32,
    clip: bool,
    style_id: Option<usize>,
    style_standard: Option<IpgButtonStyleStandard>,
    style_arrow: Option<IpgButtonArrow>,
    user_data: Option<PyObject>,
    show: bool,
) -> PyResult<usize> {

    let id = get_id(gen_id);
    
    // Calculate dimensions
    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);
    let padding = get_padding_f64(padding);

    let align_x = text_align_x.to_iced();
    let align_y = text_align_y.to_iced();

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
        IpgWidgets::IpgButton(IpgButton {
            id,
            parent_id,
            show,
            label,
            width,
            height,
            padding,
            text_align_x: align_x,
            text_align_y: align_y,
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
        background_color_hovered=None, 
        background_rgba_hovered=None,
        background_color_disabled=None, 
        background_rgba_disabled=None,
        border_color=None, 
        border_rgba=None,
        border_radius=vec![0.0], 
        border_width=1.0,
        shadow_color=None, 
        shadow_rgba=None,
        shadow_offset_x=0.0, 
        shadow_offset_y=0.0,
        shadow_blur_radius=1.0,
        text_color=None, 
        text_rgba=None,
        gen_id=None
        ))]
pub fn add_button_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    background_color_hovered: Option<IpgColor>,
    background_rgba_hovered: Option<[f32; 4]>,
    background_color_disabled: Option<IpgColor>,
    background_rgba_disabled: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    shadow_color: Option<IpgColor>,
    shadow_rgba: Option<[f32; 4]>,
    shadow_offset_x: Option<f32>,
    shadow_offset_y: Option<f32>,
    shadow_blur_radius: Option<f32>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color: Option<Color> = 
        get_color(background_rgba, background_color, 1.0, false);
    let background_color_hovered: Option<Color> = 
        get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
    let background_color_disabled: Option<Color> = 
        get_color(background_rgba_disabled, background_color_disabled, 1.0, false);
    let border_color: Option<Color> = 
        get_color(border_rgba, border_color, 1.0, false);
    let shadow_color: Option<Color> = 
        get_color(shadow_rgba, shadow_color, 1.0, false);
    let text_color: Option<Color> = 
        get_color(text_rgba, text_color, 1.0, false);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgButtonStyle(
        IpgButtonStyle {
            id,
            background_color,
            background_color_hovered,
            background_color_disabled,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_x,
            shadow_offset_y,
            shadow_blur_radius,
            text_color,
        }));

    drop(state);
    Ok(id)
}
