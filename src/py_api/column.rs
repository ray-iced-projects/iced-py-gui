//! Column module - provides add_column pyfunction
use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::state::{Containers, get_id, set_state_cont_wnd_ids, 
    set_state_of_container};
use crate::widgets::ipg_column::Column;



/// Add a column widget.
///
/// A column lays out its children vertically from top to bottom.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this column belongs to.
/// container_id : str
///     Sets the Unique string identifier for the column.
/// parent_id : str,  Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// width : float,  Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the column fills available width.
/// height : float,  Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the column fills available height.
/// fill : bool, Optional
///     Whether to fill both the available width and height
/// max_width : float,  Optional
///     Sets the Maximum width in logical pixels.
/// padding : list of float,  Optional
///     Sets the Padding as ``[all]``, ``[vertical, horizontal]``, or
///     ``[top, right, bottom, left]``.
/// spacing : float,  Optional
///     Sets the Vertical spacing between children in logical pixels.
/// align_left : bool,  Optional
///     Whether to Align children to the left.
/// align_center : bool,  Optional
///     Whether to Align children to the horizontal centre.
/// align_right : bool,  Optional
///     Whether to Align children to the right.
/// clip : bool,  Optional
///     Whether to clip content that overflows the column.
/// show : bool, default True
///     Whether the column is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created column.
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
        max_width=None,
        padding=None,
        spacing=None,
        align_left=None,
        align_center=None,
        align_right=None, 
        clip=None, 
        show=true,
        ))]
pub fn add_column(
    window_id: String,
    container_id: String,
    // **above required
    parent_id: Option<String>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    max_width: Option<f32>,
    padding: Option<Vec<f32>>,
    spacing: Option<f32>,
    align_left: Option<bool>,
    align_center: Option<bool>,
    align_right: Option<bool>,
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

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_column".to_string());

    state.containers
        .insert(id, Containers::Column(
            Column {
                id,  
                show, 
                spacing, 
                padding, 
                width, 
                width_fill,
                height, 
                height_fill,
                fill, 
                max_width, 
                align_left,
                align_center,
                align_right,
                clip,
            }));

drop(state);
Ok(id)

}

