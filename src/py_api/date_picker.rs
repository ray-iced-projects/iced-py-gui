//! DatePicker module - provides add_date_picker pyfunction

use pyo3::{Py, PyAny, pyfunction, PyResult};
use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, ipg_widgets::ipg_date_picker::Position, state::{Containers, get_id, set_state_cont_wnd_ids, set_state_of_container}, widgets::ipg_date_picker::{DatePicker, DpContent}};
type PyObject = Py<PyAny>;



/// Add a date picker widget.
///
/// A date picker that opens a calendar from a button, allowing
/// the user to select a date.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this date picker belongs to.
/// label : str, Optional
///     Sets the Text label displayed on the button.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// size_factor : float, Optional
///     Sets the size scaling factor for the calendar.
/// padding : list of float, Optional
///     Sets the Padding as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// on_submit : callable, Optional
///     Sets the Callback method to invoke when a date is submitted.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the date picker is visible.
/// show_calendar : bool, Optional
///     Whether the calendar popup is shown.
/// button_style_standard : ButtonStyleStd, Optional
///     Sets the predefined standard style variant for the button.
/// button_style_id : int, Optional
///     Sets the ID of a custom style created with ``add_button_style``.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created date picker.
#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    parent_id=None,
    on_open=None, 
    on_submit=None, 
    on_cancel=None,
    opened=false,
    size_factor=None,
    gap=None,
    snap_within_viewport=None,
    position_bottom=None,
    position_left=None,
    position_top=None,
    position_right=None,
    user_data=None,
    gen_id=None,
    ))]
pub fn add_date_picker(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    on_open: Option<PyObject>,
    on_submit: Option<PyObject>,
    on_cancel: Option<PyObject>,
    opened: bool,
    size_factor: Option<f32>,
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

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_date_picker".to_string());

    let position = if position_bottom == Some(true) {
        Position::Bottom
    } else if position_left == Some(true) {
        Position::Left
    } else if position_top == Some(true) {
        Position::Top
    } else if position_right == Some(true) {
        Position::Right
    } else { Position::Center };

    state.containers.insert(id, Containers::DatePicker(
        DatePicker {
            id,
            opened,
            size_factor,
            gap,
            position,
            dp_content: DpContent::default(),
            snap_within_viewport,
        }));

    drop(state);
    Ok(id)

}

