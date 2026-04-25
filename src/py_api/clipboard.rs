//! Clipboard module - provides clipboard read/write pyfunctions

use pyo3::{Py, PyAny, pyfunction};

use crate::state::{
    access_clipboard_actions, add_callback_to_mutex, add_user_data_to_mutex, get_id,
};

type PyObject = Py<PyAny>;

/// Queue a clipboard write action.
///
/// Parameters
/// ----------
/// text : str
///     The text to write to the system clipboard.
#[pyfunction]
#[pyo3(signature = (text))]
pub fn clipboard_write(text: String) {
    let mut actions = access_clipboard_actions();
    actions.writes.push(text);
    drop(actions);
}

/// Queue a clipboard read action.
///
/// Parameters
/// ----------
/// on_read : callable
///     Callback invoked with `(req_id, text)` where `text` is `str | None`.
/// user_data : Any, Optional
///     Optional user data passed to callback as a third argument.
/// gen_id : int, Optional
///     Optional explicit request id.
///
/// Returns
/// -------
/// int
///     Request id used for the callback.
#[pyfunction]
#[pyo3(signature = (on_read, user_data=None, gen_id=None))]
pub fn clipboard_read(on_read: PyObject, user_data: Option<PyObject>, gen_id: Option<usize>) -> usize {
    let id = get_id(gen_id);

    add_callback_to_mutex(id, "on_read".to_string(), on_read);

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let mut actions = access_clipboard_actions();
    actions.reads.push(id);
    drop(actions);

    id
}
