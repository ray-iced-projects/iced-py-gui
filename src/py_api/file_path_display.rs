//! PopUp module - provides add_popup pyfunction
use pyo3::{pyfunction, PyResult};

use crate::widgets::ipg_file_system::FileSystemDialog;
use crate::access_state;
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
        ))]
pub fn add_file_system_window(
    parent_id: String,
    opened: bool,
    select_file: Option<bool>,
    select_folder: Option<bool>,
    load_content: Option<bool>,
    ) -> PyResult<usize> 
{
    let id = get_id(None);
    
    // Register widget with parent
    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();
    
    state.widgets.insert(id, Widgets::FileSystemWindow(
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
