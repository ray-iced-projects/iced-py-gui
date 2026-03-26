//! Divider module - provides add_divider pyfunction
#![allow(unused)]

use iced::Color;
use pyo3::{Py, PyAny, pyfunction, PyResult};

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::IpgColor, py_api::helpers::get_length, state::{IpgWidgets, 
        get_id, set_state_of_widget}, widgets::{ipg_divider::{self, 
            IpgDivider, IpgDividerDirection, IpgDividerStyle}, 
            styling::IpgStyleStandard}};
type PyObject = Py<PyAny>;



/// Add a divider widget.
///
/// A divider splits an area into resizable sections with
/// draggable handles between them.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this divider belongs to.
/// direction : IpgDividerDirection
///     Sets the direction of the divider (horizontal or vertical).
/// sizes : list of float
///     Sets the initial sizes of each section in logical pixels.
/// handle_width : float
///     Sets the width of the drag handle in logical pixels.
/// handle_height : float
///     Sets the height of the drag handle in logical pixels.
/// handle_offsets : list of float, Optional
///     Sets the offsets for each handle in logical pixels.
/// include_last_handle : bool, default True
///     Whether to include a handle after the last section.
/// on_change : callable, Optional
///     Sets the Callback method to invoke when a handle is dragged.
/// on_release : callable, Optional
///     Sets the Callback method to invoke when a handle is released.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default True
///     Whether the divider fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default True
///     Whether the divider fills available height.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_divider_style``.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the divider is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created divider.
#[pyfunction]
#[pyo3(signature = (
    parent_id,
    direction,
    sizes,
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
pub fn add_divider(
    parent_id: String,
    direction: IpgDividerDirection,
    sizes: Vec<f32>,
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

    let width = get_length(width, width_fill);

    let height = get_length(height, height_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgDivider(
        IpgDivider {
            id,
            parent_id,
            show,
            direction,
            sizes,
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

/// Add styling to a divider.
///
/// Creates a custom style that can be applied to a divider
/// via its ``style_id`` parameter.
///
/// Parameters
/// ----------
/// background_color : IpgColor, Optional
///     Sets the background color using a predefined color variant.
/// background_rgba : list of float, Optional
///     Sets the background color in rgba format as [r, g, b, a].
/// background_color_hovered : IpgColor, Optional
///     Sets the background color when hovered using a predefined color variant.
/// background_rgba_hovered : list of float, Optional
///     Sets the background color when hovered in rgba format as [r, g, b, a].
/// background_transparent : bool, Optional
///     Whether the background is transparent.
/// border_color : IpgColor, Optional
///     Sets the border color using a predefined color variant.
/// border_rgba : list of float, Optional
///     Sets the border color in rgba format as [r, g, b, a].
/// border_radius : list of float, Optional
///     Sets the radius of the corners as [all] or
///     [top-left, top-right, bottom-right, bottom-left].
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a divider's ``style_id``.
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
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0);
    let background_color_hovered: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba_hovered, background_color_hovered, 1.0);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0);

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
