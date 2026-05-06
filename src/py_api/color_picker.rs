//! ColorPicker module - provides add_color_picker pyfunction

use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::{add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{Containers, access_state, get_id, set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::ipg_color_picker::ColorPicker;
use crate::ipg_widgets::ipg_color_picker::{ColorOutFormat, ColorPickerState, Position};

/// Add a color_picker widget.
///
/// A color picker that opens from a trigger widget (supplied as its child),
/// allowing the user to select a color interactively using HSV canvas, RGBA
/// sliders, a hue slider, and an optional palette panel showing tint/shade
/// variants of the selected color.
///
/// Parameters
/// ----------
/// window_id : str
///     The window this color picker belongs to.
/// container_id : str
///     A unique string ID for this color picker (used as its parent ID by
///     the trigger widget placed inside it).
/// on_open : callable, optional
///     Callback invoked when the picker opens/closes. Receives ``(id, opened: bool)``.
/// on_submit : callable, optional
///     Callback invoked when the user clicks Submit. Receives ``(id, color: str)``.
/// on_cancel : callable, optional
///     Callback invoked when the user clicks Cancel.
/// color_output_format : ColorOutFormat, optional
///     Format for the submitted color string. One of ``Integer`` (default),
///     ``Float``, ``Hex``, or ``Percent``.
/// gap : int, optional
///     Gap in pixels between the trigger widget and the picker panel.
/// snap_within_viewport : bool, optional
///     When ``True``, clamps the picker panel inside the window bounds.
/// position_bottom : bool, optional
///     Opens the picker below the trigger widget.
/// position_left : bool, optional
///     Opens the picker to the left of the trigger widget.
/// position_top : bool, optional
///     Opens the picker above the trigger widget.
/// position_right : bool, optional
///     Opens the picker to the right of the trigger widget.
/// user_data : Any, optional
///     Arbitrary data forwarded unchanged to all callbacks.
/// gen_id : int, optional
///     Pre-reserved widget ID (from ``generate_id``).
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created color picker.
#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    parent_id=None,
    on_open=None, 
    on_submit=None, 
    on_cancel=None,
    opened=false,
    color_format_int=None,
    color_format_rgba=None,
    color_format_hex=None,
    color_format_percent=None,
    gap=None,
    snap_within_viewport=None,
    position_bottom=None,
    position_left=None,
    position_top=None,
    position_right=None,
    user_data=None,
    gen_id=None,
    ))]
pub fn add_color_picker(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    on_open: Option<PyObject>,
    on_submit: Option<PyObject>,
    on_cancel: Option<PyObject>,
    opened: bool,
    color_format_int: Option<bool>,
    color_format_rgba: Option<bool>,
    color_format_hex: Option<bool>,
    color_format_percent: Option<bool>,
    gap: Option<u32>,
    snap_within_viewport: Option<bool>,
    position_bottom: Option<bool>,
    position_left: Option<bool>,
    position_top: Option<bool>,
    position_right: Option<bool>,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    if let Some(py) = on_open {
        add_callback_to_mutex(id, "on_open".to_string(), py);
    }

    if let Some(py) = on_submit {
        add_callback_to_mutex(id, "on_submit".to_string(), py);
    }

    if let Some(py) = on_cancel {
        add_callback_to_mutex(id, "on_cancel".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_color_picker".to_string());

    let mut cp = ColorPickerState::new(70, 30, 200);
    cp.format = 
        if color_format_int == Some(true) {
            Some(ColorOutFormat::Integer)
        } else if color_format_rgba == Some(true) {
            Some(ColorOutFormat::Float)
        } else if color_format_hex == Some(true) {
            Some(ColorOutFormat::Hex)
        } else if color_format_percent == Some(true) {
            Some(ColorOutFormat::Percent)
        } else { Some(ColorOutFormat::Integer) };

    let position = if position_bottom == Some(true) {
        Position::Bottom
    } else if position_left == Some(true) {
        Position::Left
    } else if position_top == Some(true) {
        Position::Top
    } else if position_right == Some(true) {
        Position::Right
    } else { Position::Center };

    state.containers.insert(id, Containers::ColorPicker(
        ColorPicker {
            id,
            opened,
            gap,
            position,
            snap_within_viewport,
            cp,
        }));

    drop(state);
    Ok(id)

}
