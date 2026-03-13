//! Column module - provides add_column pyfunction
use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::py_api::helpers::get_length;
use crate::state::{IpgContainers, get_id, set_state_cont_wnd_ids, 
    set_state_of_container};
use crate::widgets::ipg_column::IpgColumn;


/// Add a column widget.
///
/// A column lays out its children vertically from top to bottom.
///
/// Parameters
/// ----------
/// window_id : str
///     The window this column belongs to.
/// container_id : str
///     Unique string identifier for the column.
/// parent_id : str, optional
///     The parent container ID.  Defaults to the window itself.
/// width : float, optional
///     Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the column fills available width.
/// height : float, optional
///     Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the column fills available height.
/// max_width : float, optional
///     Maximum width in logical pixels.
/// padding : list of float, optional
///     Padding as ``[all]``, ``[vertical, horizontal]``, or
///     ``[top, right, bottom, left]``.
/// spacing : float, optional
///     Vertical spacing between children in logical pixels.
/// align_left : bool, optional
///     Align children to the left.
/// align_center : bool, optional
///     Align children to the horizontal centre.
/// align_right : bool, optional
///     Align children to the right.
/// clip : bool, optional
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
        width_fill=false,
        height=None, 
        height_fill=false,
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
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
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
    
    let width = get_length(width, width_fill);
    let height = get_length(height, height_fill);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_column".to_string());

    state.containers
        .insert(id, IpgContainers::IpgColumn(
            IpgColumn {
                id,  
                show, 
                spacing, 
                padding, 
                width, 
                height, 
                max_width, 
                align_left,
                align_center,
                align_right,
                clip,
            }));

drop(state);
Ok(id)

}

