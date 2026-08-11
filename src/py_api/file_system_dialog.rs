//! FileSystemDialog module - provides file system dialog window pyfunction
use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::widgets::ipg_file_system::FileSystemDialog;
use crate::access_state;
use crate::state::{CallbackName, Widgets, add_callback_name_to_mutex, get_id};



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
        select_files=None,
        select_folder=None,
        select_folders=None,
        load_file=None,
        load_file_for_editor=None,
        save_file=None,
        file_name=None,
        file_content=None,
        filters=vec![],
        set_directory=None,
        show_hidden_files=None,
        remember_last_directory=None,
        update_json_file=None,
        results_callback=None,
        ))]
pub fn add_file_system_dialog(
    select_file: Option<bool>,
    select_files: Option<bool>,
    select_folder: Option<bool>,
    select_folders: Option<bool>,
    load_file: Option<bool>,
    load_file_for_editor: Option<bool>,
    save_file: Option<bool>,
    file_name: Option<String>,
    file_content: Option<String>,
    filters: Vec<String>,
    set_directory: Option<String>,
    show_hidden_files: Option<bool>,
    remember_last_directory: Option<bool>,
    update_json_file: Option<bool>,
    results_callback: Option<PyObject>,
    ) -> PyResult<usize> 
{
    let id = get_id(None);

    // Store callback if provided
    if let Some(py) = results_callback {
        add_callback_name_to_mutex(id, CallbackName::Result, py);
    }

    let mut state = access_state();

    state.widgets.insert(id, Widgets::FileSystemDialog(
            FileSystemDialog {
                id,
                select_file,
                select_files,
                select_folder,
                select_folders,
                load_file,
                load_file_for_editor,
                save_file,
                is_loading: false,
                folder_path: None,
                folder_paths: None,
                file_path: None,
                file_paths: None,
                file_name,
                file_content,
                selected_path: None,
                filters,
                set_directory,
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

