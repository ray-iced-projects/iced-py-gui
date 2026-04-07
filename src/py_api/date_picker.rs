//! DatePicker module - provides add_date_picker pyfunction

use pyo3::{Py, PyAny, pyfunction, PyResult};

use crate::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex, state::{Widgets, 
        get_id, set_state_of_widget}, 
        widgets::{ipg_button::ButtonStyleStd, 
            ipg_date_picker::DatePicker}};
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
    parent_id, 
    label=None,
    gen_id=None,
    size_factor=None, 
    padding=None,
    on_submit=None, 
    user_data=None,
    show=true,
    show_calendar=None, 
    button_style_standard=None,
    button_style_id=None,
    ))]
pub fn add_date_picker(
    parent_id: String,
    // ** above required
    label: Option<String>,
    gen_id: Option<usize>,
    size_factor: Option<f32>,
    padding: Option<Vec<f32>>,
    on_submit: Option<PyObject>,
    user_data: Option<PyObject>,
    show: bool,
    show_calendar: Option<bool>,
    button_style_standard: Option<ButtonStyleStd>,
    button_style_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    if let Some(py) = on_submit {
        add_callback_to_mutex(id, "on_submit".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::DatePicker(
        DatePicker::new (
            id,
            parent_id,
            label,
            size_factor,
            padding,
            show,
            show_calendar,
            button_style_standard,
            button_style_id,
        )));

    drop(state);
    Ok(id)

}
