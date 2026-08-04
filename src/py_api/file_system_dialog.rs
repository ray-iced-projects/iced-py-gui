//! FileSystemDialog module - provides file system dialog window pyfunction
use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::widgets::ipg_file_system::FileSystemDialog;
use crate::{access_state, add_callback_to_mutex};
use crate::state::{Containers, get_id, set_state_cont_wnd_ids, set_state_of_container};



/// Adds a file system dialog window.
///
/// Open a file system dialog for selecting folders or files
///
/// Parameters
/// ----------
/// parent_id : str,  Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// opened : bool, default False
///     Whether the dialog is visible.
/// select_file : bool, Optional
///     Whether to select a file name
/// select_folder : bool, Optional
///     Whether to select a folder name
/// load_content : bool, Optional
///     Whether to load a file based on select_file
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
        parent_id,
        select_file=None,
        select_folder=None,
        load_file=None,
        load_file_for_editor=None,
        on_folder_selected=None,
        on_file_selected=None,
        on_file_loaded=None,
        ))]
pub fn add_file_system_dialog(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    select_file: Option<bool>,
    select_folder: Option<bool>,
    load_file: Option<bool>,
    load_file_for_editor: Option<bool>,
    on_folder_selected: Option<PyObject>,
    on_file_selected: Option<PyObject>,
    on_file_loaded: Option<PyObject>,
    ) -> PyResult<usize> 
{
    let id = get_id(None);
    
    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    // Store callback if provided
    if let Some(py) = on_folder_selected {
        add_callback_to_mutex(id, "on_folder_selected".to_string(), py);
    }

    if let Some(py) = on_file_selected {
        add_callback_to_mutex(id, "on_file_selected".to_string(), py);
    }

    if let Some(py) = on_file_loaded {
        add_callback_to_mutex(id, "on_file_loaded".to_string(), py);
    }

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_file_system_dialog".to_string());

    state.containers.insert(id, Containers::FileSystemDialog(
            FileSystemDialog {
                id,
                select_file,
                select_folder,
                load_file,
                load_file_for_editor,
                is_loading: false,
                folder_path: None,
                file_path: None,
                file_content: None,
                selected_path: None,
            }));

drop(state);
Ok(id)

}
