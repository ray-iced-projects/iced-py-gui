
use pyo3::{pyfunction, PyResult};

use crate::{access_state, py_api::helpers::{get_height, get_width}, 
state::{IpgContainers, get_id, set_state_cont_wnd_ids, 
    set_state_of_container}, widgets::ipg_stack::IpgStack};


#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    width=None, 
    height=None, 
    width_fill=false, 
    height_fill=false,
    hide_index=None, 
    show=true,
    ))]
pub fn add_stack(
        window_id: String,
        container_id: String,
        // required above
        parent_id: Option<String>,
        width: Option<f32>,
        height: Option<f32>,
        width_fill: bool,
        height_fill: bool,
        hide_index: Option<usize>,
        show: bool,
    ) -> PyResult<usize> 
{
    let id = get_id(None);

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_stack".to_string());

    state.containers.insert(id, IpgContainers::IpgStack(
        IpgStack {
            id,  
            width, 
            height,
            hide_index,
            show,
        }));

    drop(state);         
    Ok(id)

}
