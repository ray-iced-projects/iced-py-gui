//! Divider module - provides add_divider pyfunction
#![allow(unused)]

use iced::Color;
use pyo3::{Py, PyAny, pyfunction, PyResult};

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, graphics::colors::IpgColor, py_api::helpers::{get_height, get_width}, state::{IpgWidgets, 
        get_id, set_state_of_widget}, widgets::{ipg_divider::{self, IpgDividerHorizontal, IpgDividerStyle, IpgDividerVertical}, 
            styling::IpgStyleStandard}};
type PyObject = Py<PyAny>;



/// Add a divider_horizontal widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    parent_id,
    widths,
    handle_width,
    handle_height,
    handle_offsets=None,
    include_last_handle=true,
    on_change=None,
    on_release=None,
    width=None,
    width_fill=true,
    height=None,
    height_fill=true,
    style_id=None,
    gen_id=None,
    user_data=None,
    show=true,
    ))]
pub fn add_divider_horizontal(
    parent_id: String,
    widths: Vec<f32>,
    handle_width: f32,
    handle_height: f32,
    handle_offsets: Option<Vec<f32>>,
    include_last_handle: bool,
    on_change: Option<PyObject>,
    on_release: Option<PyObject>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    style_id: Option<usize>,
    gen_id: Option<usize>,
    user_data: Option<PyObject>,
    show: bool,
) -> PyResult<usize>
{
    let id = get_id(gen_id);

    if let Some(py) = on_change {
        add_callback_to_mutex(id, "on_change".to_string(), py);
    }

    if let Some(py) = on_release {
        add_callback_to_mutex(id, "on_release".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let width = get_width(width, width_fill);

    let height = get_height(height, height_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgDividerHorizontal(
        IpgDividerHorizontal {
            id,
            parent_id,
            show,
            widths,
            handle_width,
            handle_height,
            handle_offsets,
            include_last_handle,
            width,
            height,
            index_in_use: 0,
            value_in_use: 0.0,
            style_id,
        }));

    drop(state);
    Ok(id)
}


/// Add a divider_vertical widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    parent_id,
    heights,
    handle_width,
    handle_height,
    handle_offsets=None,
    include_last_handle=true,
    on_change=None,
    on_release=None,
    width=None,
    width_fill=true,
    height=None,
    height_fill=true,
    style_id=None,
    gen_id=None,
    user_data=None,
    show=true,
    ))]
pub fn add_divider_vertical(
    parent_id: String,
    heights: Vec<f32>,
    handle_width: f32,
    handle_height: f32,
    handle_offsets: Option<Vec<f32>>,
    include_last_handle: bool,
    on_change: Option<PyObject>,
    on_release: Option<PyObject>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    style_id: Option<usize>,
    gen_id: Option<usize>,
    user_data: Option<PyObject>,
    show: bool,
) -> PyResult<usize>
{
    let id = get_id(gen_id);

    if let Some(py) = on_change {
        add_callback_to_mutex(id, "on_change".to_string(), py);
    }

    if let Some(py) = on_release {
        add_callback_to_mutex(id, "on_release".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let width = get_width(width, width_fill);

    let height = get_height(height, height_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgDividerVertical(
        IpgDividerVertical {
            id,
            parent_id,
            show,
            heights,
            handle_width,
            handle_height,
            handle_offsets,
            include_last_handle,
            width,
            height,
            index_in_use: 0,
            value_in_use: 0.0,
            style_id,
        }));

    drop(state);
    Ok(id)
}

/// Add a divider_style widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
        background_color=None, 
        background_rgba=None,
        background_color_hovered=None,
        background_rgba_hovered=None,
        background_transparent=None,
        border_color=None, 
        border_rgba=None,
        border_radius=None, 
        border_width=None,
        gen_id=None
        ))]
pub fn add_divider_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    background_color_hovered: Option<IpgColor>,
    background_rgba_hovered: Option<[f32; 4]>,
    background_transparent: Option<bool>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
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

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgDividerStyle(
        IpgDividerStyle {
            id,
            background_color,
            background_color_hovered,
            background_transparent,
            border_color,
            border_radius,
            border_width,
        }));

    drop(state);
    Ok(id)
}
