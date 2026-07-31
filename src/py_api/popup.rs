//! Column module - provides add_column pyfunction
use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::state::{Containers, get_id, set_state_cont_wnd_ids, 
    set_state_of_container};
    
use crate::widgets::ipg_popup::PopUp;



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
/// wrap : bool, optional
///     When True, children that overflow the column's height wrap onto the next column.
///     Replaces the normal vertical layout with a wrapping layout.
/// wrap_horizontal_spacing : float, optional
///     Horizontal spacing between wrapped columns in logical pixels.  Only used when ``wrap=True``.
/// wrap_align_top : bool, optional
///     Align children to the top within each wrapped column.  Only used when ``wrap=True``.
/// wrap_align_center : bool, optional
///     Align children to the vertical centre within each wrapped column.  Only used when ``wrap=True``.
/// wrap_align_bottom : bool, optional
///     Align children to the bottom within each wrapped column.  Only used when ``wrap=True``.
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
        opened=None,
        position_bottom=None,
        position_center=None,
        position_left=None,
        position_top=None,
        position_right=None,
        gap=None,
        padding=None,
        snap_within_viewport=None,
        focus_trap=None,
        ))]
pub fn add_popup(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    opened: Option<bool>,
    position_bottom: Option<bool>,
    position_center: Option<bool>,
    position_left: Option<bool>,
    position_top: Option<bool>,
    position_right: Option<bool>,
    gap: Option<f32>,
    padding: Option<f32>,
    snap_within_viewport: Option<bool>,
    focus_trap: Option<bool>,
    ) -> PyResult<usize> 
{
    let id = get_id(None);
    
    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_popup".to_string());

    state.containers
        .insert(id, Containers::PopUp(
            PopUp {
                id,  
                opened,
                position_bottom,
                position_center,
                position_left,
                position_top,
                position_right,
                gap,
                padding,
                snap_within_viewport,
                focus_trap,
            }));

drop(state);
Ok(id)

}
