//! Sash module - provides add_sash pyfunction and Sash class

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};

use crate::graphics::colors::Color;
use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{Containers, Widgets, get_id, set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::ipg_sash::{Sash, SashStyle, SashStyleStd};
type PyObject = Py<PyAny>;



/// Add a sash (resizable panels) container.
///
/// A sash divides its children into resizable panels separated by draggable
/// handle bars. By default it is horizontal (panels arranged left-to-right).
/// Set ``vertical_direction=True`` to arrange panels top-to-bottom instead.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window ID that this sash belongs to.
/// container_id : str
///     Sets the parent container ID that this sash belongs to.
/// initial_sizes : list of float
///     Sets the starting pixel size of each panel in the main axis direction.
///     One entry per child widget. The total should equal ``size``.
/// size : float
///     Sets the total pixel size of the sash in the main axis direction.
/// sash_size : float
///     Sets the width of each draggable handle bar in logical pixels.
/// sync_sashes : bool, Optional
///     When True, this sash joins the global sync group. All sashes with
///     ``sync_sashes=True`` share their panel sizes — resizing one updates
///     all others. Useful for syncing a header sash with a body sash.
/// sync_cross_sashes : bool, Optional
///     When True, syncs sashes that run perpendicular to the main axis.
/// parent_id : str, Optional
///     Sets an explicit parent container ID, overriding ``window_id``.
/// outer_handle_size : float, Optional
///     Sets the pixel size of the outer (edge) handle bars.
/// cross_handle_size : float, Optional
///     Sets the pixel size of the cross-axis handle bar. Setting this enables
///     cross-axis resizing (dragging the sash taller or shorter).
/// resize_mode_last_only : bool, Optional
///     When True, only the last panel absorbs resize overflow.
/// resize_mode_uniform : bool, Optional
///     When True, all panels resize by equal amounts.
/// resize_mode_proportional : bool, Optional
///     When True, panels resize proportionally to their current sizes.
/// on_resize : callable, Optional
///     Callback invoked while a handle is being dragged.
///     Signature: ``def cb(wid: int, data: tuple[int, float])`` where
///     ``data`` is ``(panel_index, new_size)``.
/// on_resize_outer : callable, Optional
///     Callback invoked when an outer handle is dragged.
///     Signature: ``def cb(wid: int, size: float)``.
/// on_release : callable, Optional
///     Callback invoked when the mouse button is released after a drag.
///     Signature: ``def cb(wid: int)``.
/// vertical_direction : bool, Optional
///     When True, panels are stacked top-to-bottom (vertical sash).
///     Default is False (horizontal sash, panels left-to-right).
/// min_size : float, Optional
///     Sets the minimum pixel size any panel may be resized to.
/// max_size : float, Optional
///     Sets the maximum total pixel size of the sash.
/// min_cross_size : float, Optional
///     Sets the minimum pixel size of the sash in the cross-axis direction.
/// max_cross_size : float, Optional
///     Sets the maximum pixel size of the sash in the cross-axis direction.
/// clip : bool, Optional
///     Sets whether to clip the content when resized samller than content.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_sash_style``.
/// style_std : int, Optional
///     Sets a standard style,
///     Subtle(default), Primary, or Transparent.
/// user_data : Any, Optional
///     Sets arbitrary data forwarded as a third argument to all callbacks.
/// show : bool, default True
///     Whether the sash is visible.
///
/// Returns
/// -------
/// int
///     The numeric container ID of the newly created sash.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id,
    initial_sizes,
    size,
    sash_size,
    sync_sashes=None,
    sync_cross_sashes=None,
    parent_id=None,
    outer_handle_size=None,
    cross_handle_size=None,
    resize_mode_last_only=None,
    resize_mode_uniform=None,
    resize_mode_proportional=None,
    on_resize=None,
    on_resize_outer=None,
    on_release=None,
    vertical_direction=None,
    min_size=None,
    max_size=None,
    min_cross_size=None,
    max_cross_size=None,
    clip=None,
    style_id=None,
    style_std=None,
    user_data=None,
    show=true,
))]
pub fn add_sash(
    window_id: String,
    container_id: String,
    initial_sizes: Vec<f32>,
    size: f32,
    sash_size: f32,
    sync_sashes: Option<bool>,
    sync_cross_sashes: Option<bool>,
    parent_id: Option<String>,
    outer_handle_size: Option<f32>,
    cross_handle_size: Option<f32>,
    resize_mode_last_only: Option<bool>,
    resize_mode_uniform: Option<bool>,
    resize_mode_proportional: Option<bool>,
    on_resize: Option<PyObject>,
    on_resize_outer: Option<PyObject>,
    on_release: Option<PyObject>,
    vertical_direction: Option<bool>,
    min_size: Option<f32>,
    max_size: Option<f32>,
    min_cross_size: Option<f32>,
    max_cross_size: Option<f32>,
    clip: Option<bool>,
    style_id: Option<usize>,
    style_std: Option<SashStyleStd>,
    user_data: Option<PyObject>,
    show: bool,
) -> PyResult<usize> {

    let id = get_id(None);

    // Store callback if provided
    if let Some(py) = on_resize {
        add_callback_to_mutex(id, "on_resize".to_string(), py);
    }

    if let Some(py) = on_resize_outer {
        add_callback_to_mutex(id, "on_resize_outer".to_string(), py);
    }

    if let Some(py) = on_release {
        add_callback_to_mutex(id, "on_release".to_string(), py);
    }

    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_sash".to_string());

    state.containers.insert(id, Containers::Sash(
        Sash {
            id,
            current_sizes: initial_sizes.clone(),
            initial_sizes,
            size,
            sash_size,
            sync_sashes,
            sync_cross_sashes,
            outer_handle_size,
            cross_handle_size,
            resize_mode_last: resize_mode_last_only,
            resize_mode_uniform,
            resize_mode_proportional,
            vertical_direction,
            min_size,
            max_size,
            min_cross_size,
            max_cross_size,
            style_id,
            style_std,
            clip,
            show,
            resize_mode: iced_sash::OuterResizeMode::LastOnly,
        }));

    drop(state);
    Ok(id)
}


/// Add styling to a sash handle bar.
///
/// Creates a custom style that can be applied to a sash via its ``style_id``
/// parameter. The style controls the appearance of the draggable handle bars.
///
/// Parameters
/// ----------
/// bkg_color : Color, Optional
///     Sets the handle background color using a predefined color variant.
/// bkg_color_alpha : float, Optional
///     Sets the alpha transparency for the background color.
/// bkg_rgba : list of float, Optional
///     Sets the handle background color in rgba format as [r, g, b, a].
/// border_color : Color, Optional
///     Sets the border color using a predefined color variant.
/// border_color_alpha : float, Optional
///     Sets the alpha transparency for the border color.
/// border_rgba : list of float, Optional
///     Sets the border color in rgba format as [r, g, b, a].
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// border_radius : float, Optional
///     Sets the corner radius of the handle bar in logical pixels.
/// gen_id : int, Optional
///     Supplies a pre-generated ID instead of allocating a new one.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a sash's ``style_id`` parameter.
#[pyfunction]
#[pyo3(signature = (
    bkg_color=None,
    bkg_color_alpha=None,
    bkg_rgba=None,
    border_color=None,
    border_color_alpha=None,
    border_rgba=None,
    border_width=None,
    border_radius=None,
    gen_id=None,
))]
pub fn add_sash_style(
    bkg_color: Option<Color>,
    bkg_color_alpha: Option<f32>,
    bkg_rgba: Option<[f32; 4]>,
    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_width: Option<f32>,
    border_radius: Option<f32>,
    gen_id: Option<usize>,
) -> PyResult<usize> {
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::SashStyle(
        SashStyle {
            id,
            bkg_color,
            bkg_color_alpha,
            bkg_rgba,
            border_color,
            border_color_alpha,
            border_rgba,
            border_width,
            border_radius,
        }));

    drop(state);

    Ok(id)
}