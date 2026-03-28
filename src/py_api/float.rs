//! Float module - provides add_float pyfunction

use pyo3::{PyResult, pyfunction};

use crate::widgets::ipg_float::IpgFloat;
use crate::state::{IpgContainers, access_state, 
    get_id, set_state_cont_wnd_ids, set_state_of_container};


/// Add an float container widget.
///
/// Allows a widget to foat over others
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this float container belongs to.
/// container_id : str
///     Sets the Unique string identifier for the float container.
/// parent_id : str, Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// scale: float, Optional
///     Sets the scale factor for the contents
/// translate List[float], Optional
///     Sets for translation vector [x, y]
/// scale_clamped: bool, Optional
///     Whether to clamp the scale to the content container
/// 
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created opaque container.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id = None,
    scale = None,
    translate = None,
    scale_clamped = None,
    ))]
pub fn add_float(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    scale: Option<f32>,
    translate: Option<[f32; 2]>,
    scale_clamped: Option<bool>,
    ) -> PyResult<usize> 
{
    let id = get_id(None);

    let prt_id = if let Some(id) = parent_id {
        id
    } else { window_id.clone() };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_opaque".to_string());

    state.containers.insert(id, IpgContainers::IpgFloat(
        IpgFloat {
            id,
            scale,
            translate,
            scale_clamped,  
        }));

    drop(state);         
    Ok(id)
}
