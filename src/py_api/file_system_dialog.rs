//! FileSystemDialog module - provides file system dialog window pyfunction
use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::widgets::ipg_file_system::FileSystemDialog;
use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{Widgets, get_id, set_state_of_widget};



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
        parent_id,
        opened=false,
        select_file=None,
        select_folder=None,
        load_content=None,
        on_folder_selected=None,
        user_data=None,
        ))]
pub fn add_file_system_dialog(
    parent_id: String,
    opened: bool,
    select_file: Option<bool>,
    select_folder: Option<bool>,
    load_content: Option<bool>,
    on_folder_selected: Option<PyObject>,
    user_data: Option<PyObject>,
    ) -> PyResult<usize> 
{
    let id = get_id(None);

    // Register widget with parent
    set_state_of_widget(id, parent_id.clone());

    // Store callback if provided
    if let Some(py) = on_folder_selected {
        add_callback_to_mutex(id, "on_folder_selected".to_string(), py);
    }

    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let mut state = access_state();
    
    state.widgets.insert(id, Widgets::FileSystemDialog(
            FileSystemDialog {
                id,  
                opened,
                select_file,
                select_folder,
                load_content,
                is_loading: false,
                folder_name: None,
                file_name: None,
                file_content: None,
                selected_path: None,
            }));

drop(state);
Ok(id)

}
