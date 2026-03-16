//! Events module - provides add_event_window pyfunction

use pyo3::prelude::*;
use pyo3::{pyfunction, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::state::{IpgContainers, access_state, get_id};
use crate::widgets::ipg_events::IpgWindowEvent;


/// Add a window eventt.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    enabled: bool,
    on_closed: Optional[Callable]=None,
    on_moved: Optional[Callable]=None,
    on_resized: Optional[Callable]=None,
    on_redraw_requested: Optional[Callable]=None,
    on_close_requested: Optional[Callable]=None,
    on_focused: Optional[Callable]=None,
    on_unfocused: Optional[Callable]=None,
    on_file_hovered: Optional[Callable]=None,
    on_file_dropped: Optional[Callable]=None,
    on_files_hovered_left: Optional[Callable]=None,
    user_data: Optional[Any]=None,
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
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, IpgContainers::IpgWindowEvent(
        IpgWindowEvent { id, enabled }));

    drop(state);
    Ok(id)
}

