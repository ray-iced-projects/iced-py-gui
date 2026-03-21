//! Events module - provides add_event_window, add_event_keyboard, add_event_mouse pyfunction

use pyo3::prelude::*;
use pyo3::{pyfunction, Py, PyAny};
type PyObject = Py<PyAny>;
use crate::{access_state, add_user_data_to_mutex};
use crate::state::{access_events, get_id};


#[pyfunction]
#[pyo3(signature = (
    enabled=false, 
    on_closed=None, 
    on_moved=None, 
    on_resized=None,
    on_redraw_requested=None,
    on_close_requested=None,
    on_focused=None, 
    on_unfocused=None,
    on_file_hovered=None,
    on_file_dropped=None,
    on_files_hovered_left=None,
    user_data=None,
    gen_id=None,
))]
pub fn add_event_window(
    enabled: bool,
    on_closed: Option<PyObject>,
    on_moved: Option<PyObject>,
    on_resized: Option<PyObject>,
    on_redraw_requested: Option<PyObject>,
    on_close_requested: Option<PyObject>,
    on_focused: Option<PyObject>,
    on_unfocused: Option<PyObject>,
    on_file_hovered: Option<PyObject>,
    on_file_dropped: Option<PyObject>,
    on_files_hovered_left: Option<PyObject>,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut events = access_events();

    if let Some(py) = on_closed {
        events.events.insert((id, "closed".to_string()), py);
    }
    if let Some(py) = on_moved {
        events.events.insert((id, "moved".to_string()), py);
    }
    if let Some(py) = on_resized {
        events.events.insert((id, "resized".to_string()), py);
    }
    if let Some(py) = on_redraw_requested {
        events.events.insert((id, "redraw requested".to_string()), py);
    }
    if let Some(py) = on_close_requested {
        events.events.insert((id, "close requested".to_string()), py);
    }
    if let Some(py) = on_focused {
        events.events.insert((id, "focused".to_string()), py);
    }
    if let Some(py) = on_unfocused {
        events.events.insert((id, "unfocused".to_string()), py);
    }
    if let Some(py) = on_file_hovered {
        events.events.insert((id, "file hovered".to_string()), py);
    }
    if let Some(py) = on_file_dropped {
        events.events.insert((id, "file dropped".to_string()), py);
    }

    if let Some(py) = on_files_hovered_left {
        events.events.insert((id, "files hovered left".to_string()), py);
    }
    
    drop(events);

    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    let mut state = access_state();

    state.window_event_id_enabled = (id, enabled);

    drop(state);
    Ok(id)
}

#[pyfunction]
#[pyo3(signature = (
    enabled=false, 
    on_key_press=None, 
    on_key_release=None,
    user_data=None,
    gen_id=None,
))]
pub fn add_event_keyboard(
    enabled: bool,
    on_key_press: Option<PyObject>,
    on_key_release: Option<PyObject>,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
    )  -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut events = access_events();

    if let Some(py) = on_key_press {
        events.events.insert((id, "key pressed".to_string()), py);
    }
    if let Some(py) = on_key_release {
        events.events.insert((id, "key released".to_string()), py);
    }

    drop(events);

    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    let mut state = access_state();

    state.keyboard_event_id_enabled = (id, enabled);

    drop(state);
    Ok(id)
}

#[pyfunction]
#[pyo3(signature = (
    enabled=false, 
    on_move=None, 
    on_enter_window=None, 
    on_exit_window=None, 
    on_left_press=None, 
    on_left_release=None,
    on_middle_press=None, 
    on_middle_release=None,
    on_right_press=None, 
    on_right_release=None,
    on_middle_scroll_line=None,
    user_data=None,
    gen_id=None,
))]
pub fn add_event_mouse(
    enabled: bool,
    on_move: Option<PyObject>,
    on_enter_window: Option<PyObject>,
    on_exit_window: Option<PyObject>,
    on_left_press: Option<PyObject>,
    on_left_release: Option<PyObject>,
    on_middle_press: Option<PyObject>,
    on_middle_release: Option<PyObject>,
    on_right_press: Option<PyObject>,
    on_right_release: Option<PyObject>,
    on_middle_scroll_line: Option<PyObject>,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut events = access_events();

    if let Some(py) = on_move {
        events.events.insert((id, "move".to_string()), py);
    }
    if let Some(py) = on_enter_window {
        events.events.insert((id, "enter window".to_string()), py);
    }
    if let Some(py) = on_exit_window {
        events.events.insert((id, "exit window".to_string()), py);
    }
    if let Some(py) = on_left_press {
        events.events.insert((id, "left press".to_string()), py);
    }
    if let Some(py) = on_left_release {
        events.events.insert((id, "left release".to_string()), py);
    }
    if let Some(py) = on_middle_press {
        events.events.insert((id, "middle press".to_string()), py);
    }
    if let Some(py) = on_middle_release {
        events.events.insert((id, "middle release".to_string()), py);
    }
    if let Some(py) = on_right_press {
        events.events.insert((id, "right press".to_string()), py);
    }
    if let Some(py) = on_right_release {
        events.events.insert((id, "right release".to_string()), py);
    }
    if let Some(py) = on_middle_scroll_line {
        events.events.insert((id, "middle scroll line".to_string()), py);
    }

    drop(events);

    
    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    let mut state = access_state();

    state.mouse_event_id_enabled = (id, enabled);

    drop(state);
    Ok(id)
}
