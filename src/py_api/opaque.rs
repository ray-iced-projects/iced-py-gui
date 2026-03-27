//! Opague module - provides add_opaque pyfunction


use pyo3::{PyResult, pyfunction};

use crate::widgets::ipg_opaque::IpgOpaque;
use crate::state::{IpgContainers, access_state, 
    get_id, set_state_cont_wnd_ids, set_state_of_container};


/// Add an opaque container widget.
///
/// An opaque container blocks mouse events from passing through
/// to widgets underneath it, useful for overlay scenarios.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this opaque container belongs to.
/// container_id : str
///     Sets the Unique string identifier for the opaque container.
/// parent_id : str, Optional
///     Sets the parent container ID.  Defaults to the window itself.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created opaque container.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    ))]
pub fn add_opaque(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    ) -> PyResult<usize> 
{
    let id = get_id(None);

    let prt_id = if let Some(id) = parent_id {
        id
    } else { window_id.clone() };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_opaque".to_string());

    state.containers.insert(id, IpgContainers::IpgOpaque(
        IpgOpaque {
            id,  
        }));

    drop(state);         
    Ok(id)
}
