//! Window Events module - provides add_event_window pyfunction

use pyo3::prelude::*;
use pyo3::{pyfunction, Py, PyAny};
type PyObject = Py<PyAny>;
use crate::{access_state, access_user_data1};
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
    user_data=None
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
    ) -> PyResult<usize>
{
    let id = get_id(None);

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

    let mut callback_user_data = access_user_data1();

    if let Some(py) = user_data {
        callback_user_data.user_data.insert(id, py);
    }
    
    drop(callback_user_data);
    
    let mut state = access_state();

    state.window_event_id_enabled = (id, enabled);

    drop(state);
    Ok(id)
}
