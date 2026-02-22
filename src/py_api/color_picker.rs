//! ColorPicker module - provides add_button pyfunction
use iced::Color;
use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::{get_height, get_width};
use crate::{add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{IpgWidgets, access_state, get_id, set_state_of_widget};
use crate::widgets::ipg_button::{IpgButtonStyleStandard};
use crate::widgets::ipg_color_picker::{IpgColorPicker, IpgColorPickerStyle};
use crate::graphics::bootstrap_arrow::IpgArrow;


/// Add a color_picker widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    label=None, 
    gen_id=None, 
    on_press=None, 
    on_select=None, 
    on_cancel=None,
    color=None,
    color_rgba=None, 
    width=None,
    width_fill=false,  
    height=None, 
    height_fill=false, 
    padding=None, 
    clip=None, 
    style_id=None, 
    style_standard=None, 
    style_arrow=None,
    user_data=None,
    show=false, 
    ))]
pub fn add_color_picker(
    parent_id: String,
    // ** above required
    label: Option<String>,
    gen_id: Option<usize>,
    on_press: Option<PyObject>,
    on_select: Option<PyObject>,
    on_cancel: Option<PyObject>,
    mut color: Option<IpgColor>,
    color_rgba: Option<[f32; 4]>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    padding: Option<Vec<f32>>,
    clip: Option<bool>,
    style_id: Option<usize>,
    style_standard: Option<IpgButtonStyleStandard>,
    style_arrow: Option<IpgArrow>,
    user_data: Option<PyObject>,
    show: bool,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    // default to rgba if no color
    let mut rgba = color_rgba;
    if color.is_none() && rgba.is_none() {
        rgba = Some([0.5, 0.2, 0.7, 1.0]);
        color = None;
    }

    if color.is_some() && rgba.is_some() {
        rgba = None;
    }

    let color = 
        IpgColor::rgba_ipg_color_to_iced(rgba, color, 1.0, false).unwrap();

    if let Some(py) = on_press {
        add_callback_to_mutex(id, "on_press".to_string(), py);
    }

    if let Some(py) = on_select {
        add_callback_to_mutex(id, "on_select".to_string(), py);
    }

    if let Some(py) = on_cancel {
        add_callback_to_mutex(id, "on_cancel".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgColorPicker(
        IpgColorPicker {
            id,
            parent_id,
            show,
            color,
            // button related
            label,
            width,
            height,
            padding,
            clip,
            style_id,
            style_standard,
            style_arrow,                             
            }));

    drop(state);
    Ok(id)

}

// Add a color_picker_style widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = ( 
    background_color=None, 
    background_rgba=None,
    background_color_hovered=None, 
    background_rgba_hovered=None,
    border_color=None, 
    border_rgba=None,
    border_radius=None,
    border_width=None,
    shadow_color=None, 
    shadow_rgba=None,
    shadow_offset_x=None, 
    shadow_offset_y=None,
    shadow_blur_radius=None,
    text_color=None, 
    text_rgba=None,
    gen_id=None
    ))]
pub fn add_color_picker_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    background_color_hovered: Option<IpgColor>,
    background_rgba_hovered: Option<[f32; 4]>,
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
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let background_color_hovered: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba_hovered, background_color_hovered, 1.0, false);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0, false);
    let shadow_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(shadow_rgba, shadow_color, 1.0, false);
    let text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, 1.0, false);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgColorPickerStyle(
        IpgColorPickerStyle {
            id,
            background_color,
            background_color_hovered,
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
