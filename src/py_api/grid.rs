//! Grid module - provides add_float pyfunction

use pyo3::{PyResult, pyfunction};

use crate::py_api::helpers::get_length;
use crate::widgets::ipg_grid::Grid;
use crate::state::{Containers, access_state, 
    get_id, set_state_cont_wnd_ids, set_state_of_container};


/// Add an grid container widget.
///
/// Allows a widget to foat over others
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this grid container belongs to.
/// container_id : str
///     Sets the Unique string identifier for the grid container.
/// parent_id : str, Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// spacing: float, Optional
///     Sets the spacing between grid items
/// columns_max_width float, Optional
///     Makes the amount of columns dynamic, never
///     exceeding the provided max_width
/// columns_amount: int, Optional
///     Sets the number of columns in the grid
/// width: float, Optional
///     Sets the width of the grid
/// height_aspect_ratio: float, Optional
///     Sets the aspection ratio for a grid.
/// height_evenly_distribute: float, Optional
///     Sets how the cels of the grid as distributed
/// height_evenly_distribute_fill bool
///     Whether to distribute the cells based on size.
/// 
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created opaque container.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id,
    width, 
    parent_id = None,
    spacing = None,
    columns_max_width = None,
    columns_amount = None,
    height_aspect_ratio = None,
    height_evenly_distribute = None,
    height_evenly_distribute_fill = false,
    ))]
pub fn add_grid(
    window_id: String,
    container_id: String,
    width: f32,
    parent_id: Option<String>,
    spacing: Option<f32>,
    columns_max_width: Option<f32>,
    columns_amount: Option<usize>,
    height_aspect_ratio: Option<f32>,
    height_evenly_distribute: Option<f32>,
    height_evenly_distribute_fill: bool,
    ) -> PyResult<usize> 
{
    let id = get_id(None);

    let height_evenly_distribute = 
        get_length(height_evenly_distribute, 
            height_evenly_distribute_fill);

    let prt_id = if let Some(id) = parent_id {
        id
    } else { window_id.clone() };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_opaque".to_string());

    state.containers.insert(id, Containers::Grid(
        Grid {
            id,
            width,
            spacing,
            columns_max_width,
            columns_amount,
            height_aspect_ratio,
            height_evenly_distribute, 
        }));

    drop(state);         
    Ok(id)
}
