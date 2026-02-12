//! Button module - provides add_button pyfunction

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};

use crate::py_api::helpers::{get_height, get_padding_f64, get_width};
use crate::state::{access_state, add_callback, add_user_data, IpgIds, IpgWidgets};
use crate::widgets::enums::{IpgHorizontalAlignment, IpgVerticalAlignment};
use crate::widgets::button::{IpgButton, IpgButtonArrow};
use crate::widgets::styling::IpgStyleStandard;

type PyObject = Py<PyAny>;


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
    style_standard: Option<IpgStyleStandard>,
    style_arrow: Option<IpgButtonArrow>,
    user_data: Option<PyObject>,
    show: bool,
) -> PyResult<usize> {
    let mut state = access_state();

    // Get or generate ID
    let id = match gen_id {
        Some(gid) => gid,
        None => {
            state.last_id += 1;
            state.last_id
        }
    };

    drop(state);

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
        add_callback(id, "on_press".to_string(), py);
    }

    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data(id, py);
    }

    // Create and store button
    let mut state = access_state();
    state.widgets.insert(
        id,
        IpgWidgets::IpgButton(IpgButton::new(
            id,
            parent_id,
            show,
            label,
            width,
            height,
            padding,
            align_x,
            align_y,
            text_size,
            clip,
            style_id,
            style_standard,
            style_arrow,
        )),
    );
    drop(state);

    Ok(id)
}


/// Find the parent container's usize ID from its string ID
fn find_parent_uid(ipg_ids: &[IpgIds], parent_id: String) -> usize {
    for id_info in ipg_ids.iter() {
        if id_info.container_id == Some(parent_id.clone()) {
            return id_info.id;
        }
    }
    panic!("Parent id {:?} not found in find_parent_uid()", parent_id)
}

/// Set up widget state - registers the widget with its parent container
fn set_state_of_widget(id: usize, parent_id: String) {
    let state = access_state();

    // Find the window string ID from the container string ID
    let wnd_id_str = match state.container_wnd_str_ids.get(&parent_id) {
        Some(id) => id.clone(),
        None => panic!(
            "The main window id could not be found using parent_id {}, check that your parent_id matches a container",
            parent_id
        ),
    };

    // Find the window usize ID
    let wnd_id_usize = match state.windows_str_ids.get(&wnd_id_str) {
        Some(id) => *id,
        None => panic!("window id {} could not be found in set_state_of_widget", wnd_id_str),
    };

    drop(state);

    let mut state = access_state();

    // Find the parent's usize ID
    let parent_uid = find_parent_uid(state.ids.get(&wnd_id_usize).unwrap(), parent_id.clone());

    // Register this widget with the window's ID tracking
    state.ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds {
        id,
        parent_uid,
        container_id: None,
        parent_id,
        is_container: false,
    });

    drop(state);
}



