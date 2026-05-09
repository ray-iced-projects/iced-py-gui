use std::collections::HashMap;

use crate::{access_state, state::{Containers, get_id, set_state_cont_wnd_ids, set_state_of_container}, widgets::ipg_draw::{Draw, extract_curves}};

use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    curves=None,
    ))]
pub fn add_draw(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    curves: Option<PyObject>,
    ) -> PyResult<usize>
{
    let id = get_id(None);
    
    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    // If a curves callable was provided, call it and parse the JSON it returns
    // into canvas widgets.  The result is stored in Draw so that clone_state()
    // can pre-populate CanvasState before the runtime starts.
    let (draw_curves, draw_text_curves) = if let Some(py) = &curves {
        extract_curves(py)?
    } else { (HashMap::new(), HashMap::new()) };
    
    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_container".to_string());

    state.containers.insert(id, Containers::CanvasDraw(
        Draw {
            id,
            curves: draw_curves,
            text_curves: draw_text_curves,
        }));

    drop(state);
    Ok(id)
}
