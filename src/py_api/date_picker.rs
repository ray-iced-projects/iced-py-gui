//! DatePicker module - provides add_date_picker pyfunction
#![allow(unused)]

use pyo3::{Py, PyAny, pyfunction, PyResult};

use crate::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex, state::{IpgWidgets, 
        get_id, set_state_of_widget}, 
        widgets::{ipg_button::IpgButtonStyleStandard, 
            ipg_date_picker::IpgDatePicker, 
            styling::IpgStyleStandard}};
type PyObject = Py<PyAny>;



/// Add a date picker widget.
///
/// Returns the widget ID.
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
    button_style_standard: Option<IpgButtonStyleStandard>,
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

    state.widgets.insert(id, IpgWidgets::IpgDatePicker(
        IpgDatePicker::new (
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
