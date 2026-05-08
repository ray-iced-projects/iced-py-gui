use crate::{access_state, state::{Containers, get_id, set_state_cont_wnd_ids, set_state_of_container}, widgets::ipg_draw::Draw};

use pyo3::{PyResult, pyfunction};

#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    ))]
pub fn add_draw(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    ) -> PyResult<usize>
{
    let id = get_id(None);
    
    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_container".to_string());

    state.containers.insert(id, Containers::CanvasDraw(
        Draw {
            id,
        }));

    drop(state);
    Ok(id)
}

