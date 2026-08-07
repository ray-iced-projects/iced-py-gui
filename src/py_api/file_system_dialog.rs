//! FileSystemDialog module - provides file system dialog window pyfunction
use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::widgets::ipg_file_system::FileSystemDialog;
use crate::{access_state, add_callback_to_mutex};
use crate::state::{Widgets, get_id};



/// Adds a file system dialog window.
///
/// Create a file system dialog for selecting folders or files
///
/// Parameters
/// ----------
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
        select_file=None,
        select_folder=None,
        load_file=None,
        load_file_for_editor=None,
        save_file=None,
        filters=vec![],
        initial_directory=None,
        show_hidden_files=None,
        remember_last_directory=None,
        update_json_file=None,
        on_folder_selected=None,
        on_file_selected=None,
        on_file_loaded=None,
        ))]
pub fn add_file_system_dialog(
    select_file: Option<bool>,
    select_folder: Option<bool>,
    load_file: Option<bool>,
    load_file_for_editor: Option<bool>,
    save_file: Option<bool>,
    filters: Vec<String>,
    initial_directory: Option<String>,
    show_hidden_files: Option<bool>,
    remember_last_directory: Option<bool>,
    update_json_file: Option<bool>,
    on_folder_selected: Option<PyObject>,
    on_file_selected: Option<PyObject>,
    on_file_loaded: Option<PyObject>,
    ) -> PyResult<usize> 
{
    let id = get_id(None);

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

    let mut state = access_state();

    state.widgets.insert(id, Widgets::FileSystemDialog(
            FileSystemDialog {
                id,
                select_file,
                select_folder,
                load_file,
                load_file_for_editor,
                save_file,
                is_loading: false,
                folder_path: None,
                file_path: None,
                file_content: None,
                selected_path: None,
                filters,
                initial_directory,
                show_hidden_files,
                remember_last_directory,
                update_json_file,
            }));

drop(state);
Ok(id)

}

/// Return a list of dialog filters obtained from config file or internally if modified.
///
/// Returns
/// -------
/// list[String]
///     List of filter names currently in use
#[pyfunction]
pub fn get_dialog_filters() -> Vec<String> {
    use crate::config_creator::load_file_filters;
    
    match load_file_filters() {
        Ok(filters) => {
            // Extract just the filter names from the (name, extensions) tuples
            filters.into_iter().map(|(name, _)| name).collect()
        },
        Err(e) => {
            eprintln!("Failed to load filters: {}", e);
            // Return default fallback
            vec!["All Files".to_string()]
        }
    }
}

