//! MouseArea module - provides add_mousearea pyfunction

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::state::{Containers, access_state, add_callback_to_mutex, get_id, set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::ipg_mouse_area::{MouseArea, MousePointer};


/// Add a mouse area widget.
///
/// A mouse area wraps child widgets and provides callbacks
/// for mouse actions.  The mouse area assumes the size of
/// its children.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this mouse area belongs to.
/// container_id : str
///     Sets the Unique string identifier for the mouse area.
/// parent_id : str, Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// show : bool, default True
///     Whether the mouse area is visible.
/// mouse_pointer : MousePointer, Optional
///     Sets the mouse pointer style when hovering over the area.
/// on_press : callable, Optional
///     Sets the Callback method to invoke when the left mouse button is pressed.
/// on_release : callable, Optional
///     Sets the Callback method to invoke when the left mouse button is released.
/// on_right_press : callable, Optional
///     Sets the Callback method to invoke when the right mouse button is pressed.
/// on_right_release : callable, Optional
///     Sets the Callback method to invoke when the right mouse button is released.
/// on_middle_press : callable, Optional
///     Sets the Callback method to invoke when the middle mouse button is pressed.
/// on_middle_release : callable, Optional
///     Sets the Callback method to invoke when the middle mouse button is released.
/// on_enter : callable, Optional
///     Sets the Callback method to invoke when the mouse enters the area.
/// on_move : callable, Optional
///     Sets the Callback method to invoke when the mouse moves over the area.
/// on_exit : callable, Optional
///     Sets the Callback method to invoke when the mouse exits the area.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show: bool
///     Whether to show the container or not
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created mouse area.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    enabled=None, 
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
    show=true,
    ))]
pub fn add_mouse_area(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    enabled: Option<bool>,
    mouse_pointer: Option<MousePointer>,
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
    show: bool,
    ) -> PyResult<usize>
{
    let id = get_id(None);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    if let Some(py) = on_press {
    add_callback_to_mutex(id, "on_press".to_string(), py);
    }
    
    if let Some(py) = on_release {
        add_callback_to_mutex(id, "on_release".to_string(), py);
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

    state.containers.insert(id, Containers::MouseArea(
        MouseArea {
            id,
            mouse_pointer,  
            enabled,
            show,
        }));

    drop(state);
    Ok(id)

}
