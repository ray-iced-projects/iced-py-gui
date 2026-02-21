//! MouseArea module - provides add_mousearea pyfunction

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::state::{IpgContainers, access_state, add_callback_to_mutex, get_id, set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::ipg_mousearea::{IpgMouseArea, IpgMousePointer};


/// Add a button widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None, 
    gen_id=None, 
    show=true, 
    mouse_pointer=None,
    on_press=None, 
    on_release=None,
    on_right_press=None, 
    on_right_release=None,
    on_middle_press=None, 
    on_middle_release=None,
    on_enter=None, 
    on_move=None, 
    on_exit=None,
    user_data=None,
    ))]
pub fn add_mouse_area(
    window_id: String,
    container_id: String,
    // required above
    parent_id: Option<String>,
    gen_id: Option<usize>,
    show: bool,
    mouse_pointer: Option<IpgMousePointer>,
    on_press: Option<PyObject>,
    on_release: Option<PyObject>,
    on_right_press: Option<PyObject>,
    on_right_release: Option<PyObject>,
    on_middle_press: Option<PyObject>,
    on_middle_release: Option<PyObject>,
    on_enter: Option<PyObject>,
    on_move: Option<PyObject>,
    on_exit: Option<PyObject>,
    user_data: Option<PyObject>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    if let Some(py) = on_press {
    add_callback_to_mutex(id, "on_press".to_string(), py);
    }
    
    if let Some(py) = on_release {
        add_callback_to_mutex(id, "event_name".to_string(), py);
    }
    
    if let Some(py) = on_right_press {
        add_callback_to_mutex(id, "on_right_press".to_string(), py);
    }
    
    if let Some(py) = on_right_release {
        add_callback_to_mutex(id, "on_right_release".to_string(), py);
    }
    
    if let Some(py) = on_middle_press {
        add_callback_to_mutex(id, "on_middle_press".to_string(), py);
    }
    
    if let Some(py) = on_middle_release {
        add_callback_to_mutex(id, "on_middle_release".to_string(), py);
    }
    
    if let Some(py) = on_enter {
        add_callback_to_mutex(id, "on_enter".to_string(), py);
    }
    
    if let Some(py) = on_move {
        add_callback_to_mutex(id, "on_move".to_string(), py);
    }
    
    if let Some(py) = on_exit {
        add_callback_to_mutex(id, "on_exit".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_mousearea".to_string());

    state.containers.insert(id, IpgContainers::IpgMouseArea(
        IpgMouseArea {
            id,
            mouse_pointer,  
            show, 
        }));

    drop(state);
    Ok(id)

}