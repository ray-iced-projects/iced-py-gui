//! Stack module - provides add_stack() or with Stack() pyfunction
//! 
use pyo3::{pyfunction, PyResult};

use crate::{access_state,  
state::{Containers, get_id, set_state_cont_wnd_ids, 
    set_state_of_container}, widgets::ipg_stack::Stack};


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
/// fill : bool, Optional
///     Whether to fill both the available width and height.
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
    width_fill=None, 
    height=None, 
    height_fill=None,
    fill=None,
    hide_index=None, 
    show=true,
    ))]
pub fn add_stack(
        window_id: String,
        container_id: String,
        parent_id: Option<String>,
        width: Option<f32>,
        width_fill: Option<bool>,
        height: Option<f32>,
        height_fill: Option<bool>,
        fill: Option<bool>,
        hide_index: Option<usize>,
        show: bool,
    ) -> PyResult<usize> 
{
    let id = get_id(None);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_stack".to_string());

    state.containers.insert(id, Containers::Stack(
        Stack {
            id,  
            width,
            width_fill, 
            height,
            height_fill,
            fill,
            hide_index,
            show,
        }));

    drop(state);         
    Ok(id)

}
