//! Row module - provides add_row pyfunction

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::state::{Containers, get_id, 
    set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::ipg_row::Row;


/// Add a row widget.
///
/// A row lays out its children horizontally from left to right.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this row belongs to.
/// container_id : str
///     Sets the Unique string identifier for the row.
/// parent_id : str, optional
///     Sets the parent container ID.  Defaults to the window itself.
/// width : float, optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, Optional
///     Whether the row fills available width.
/// height : float, optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, Optional
///     Whether the row fills available height.
/// fill : bool, Optional
///     Whether to fill both the available width and height
/// align_bottom : bool, optional
///     Whether to Align children to the bottom.
/// align_center : bool, optional
///     Whether to Align children to the vertical centre.
/// align_top : bool, optional
///     Whether to Align children to the top.
/// padding : list of float, optional
///     Sets the Padding as ``[all]``, ``[vertical, horizontal]``, or
///     ``[top, right, bottom, left]``.
/// spacing : float, optional
///     Sets the Horizontal spacing between children in logical pixels.
/// clip : bool, optional
///     Whether to clip content that overflows the row.
/// show : bool, default True
///     Whether the row is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created row.
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
        align_bottom=None,
        align_center=None,
        align_top=None,
        padding=None, 
        spacing=None, 
        clip=None,
        show=true,
        ))]
pub fn add_row(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    align_bottom: Option<bool>,
    align_center: Option<bool>,
    align_top: Option<bool>,
    padding: Option<Vec<f32>>,
    spacing: Option<f32>,
    clip: Option<bool>,
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

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_row".to_string());

    state.containers.
        insert(id, Containers::Row(
            Row {
            id,  
            spacing, 
            padding, 
            width,
            width_fill, 
            height,
            height_fill,
            fill,
            align_bottom,
            align_center,
            align_top,
            clip,
            show,
        }));

    drop(state);         
    Ok(id)

}
    