//! ColorPicker module - provides add_button pyfunction
use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::py_api::helpers::{get_height, get_width};
use crate::{add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{IpgWidgets, access_state, get_id, set_state_of_widget};
use crate::widgets::ipg_button::{IpgButtonArrow, IpgButtonStyleStandard};
use crate::widgets::ipg_color_picker::{IpgColorPicker};


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
    color_rgba=[0.5, 0.2, 0.7, 1.0], 
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
    color_rgba:[f32; 4],
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    padding: Option<Vec<f32>>,
    clip: Option<bool>,
    style_id: Option<usize>,
    style_standard: Option<IpgButtonStyleStandard>,
    style_arrow: Option<IpgButtonArrow>,
    user_data: Option<PyObject>,
    show: bool,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let color = iced::Color::from(color_rgba);

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
