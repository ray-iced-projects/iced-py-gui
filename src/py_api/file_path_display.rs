//! PopUp module - provides add_popup pyfunction
use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::widgets::ipg_file_system::FileSystemWindow;
use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{Containers, get_id, set_state_cont_wnd_ids, 
    set_state_of_container};

use crate::ipg_widgets::ipg_file_path;



/// Adds a popup widget.
///
/// A popup is a container which is shown by a callback from a widget
/// or by setting the opened value to True by some other means.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this column belongs to.
/// container_id : str
///     Sets the Unique string identifier for the column.
/// parent_id : str,  Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// opened : bool, default False
///     Whether the popup is visible.
/// position_bottom, position_center(default), position_left, position_right: bool, Optional
///     Sets the possition of the popup.
/// gap : float, defalut 0.0
/// Sets the gap between the widhet that opens the the popup or the origin of the hidden popup widget.
/// padding : list of float,  Optional
///     Sets the Padding as ``[all]``, ``[vertical, horizontal]``, or
///     ``[top, right, bottom, left]``.
/// snap_within_viewport: bool, default True
///     Whether to keep the popup within the widnow
/// focus_trap: bool, Optional
///     When `true`, Tab and Shift+Tab are captured while the popup is open,
///     keeping keyboard focus within the popup content.
/// on_open: callbale, Optional
///     The callback function when the popup opens
/// on_close: callable, Optional
///     The callback function when the popup is closed
/// on_click_outside: callbale, Optional
///     The callback function for when the mouse clicks outside of the popup.
/// user_data: 
/// 
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created column.
/// 
#[pyfunction]
#[pyo3(signature = (
        window_id, 
        container_id, 
        parent_id=None,
        opened=false,
        select_file=None,
        select_folder=None,
        on_open=None,
        on_close=None,
        on_click_outside=None,
        user_data=None,
        ))]
pub fn add_file_system_window(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    opened: bool,
    select_file: Option<bool>,
    select_folder: Option<bool>,
    on_open: Option<PyObject>,
    on_close: Option<PyObject>,
    on_click_outside: Option<PyObject>,
    user_data: Option<PyObject>,
    ) -> PyResult<usize> 
{
    let id = get_id(None);

    if let Some(py) = on_open {
        add_callback_to_mutex(id, "on_open".to_string(), py);
    }

    if let Some(py) = on_close {
        add_callback_to_mutex(id, "on_close".to_string(), py);
    }

    if let Some(py) = on_click_outside {
        add_callback_to_mutex(id, "on_click_outside".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_popup".to_string());

    state.containers
        .insert(id, Containers::FileSystemWindow(
            FileSystemWindow {
                id,  
                opened,
                select_file,
                select_folder,
            }));

drop(state);
Ok(id)

}
