
use pyo3::{pyfunction, PyResult};

use crate::{access_state, py_api::helpers::get_length, 
state::{IpgContainers, get_id, set_state_cont_wnd_ids, 
    set_state_of_container}, widgets::ipg_stack::IpgStack};


/// Add a stack container widget.
///
/// A stack lays out its children on top of each other,
/// with later children drawn above earlier ones.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this stack belongs to.
/// container_id : str
///     Sets the Unique string identifier for the stack.
/// parent_id : str, Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// width_fill : bool, default False
///     Whether the stack fills available width.
/// height_fill : bool, default False
///     Whether the stack fills available height.
/// hide_index : int, Optional
///     Sets the index of the child to hide.
/// show : bool, default True
///     Whether the stack is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created stack.
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

    let width = get_length(width, width_fill);
    let height = get_length(height, height_fill);

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
