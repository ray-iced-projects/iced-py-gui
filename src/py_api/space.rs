//! Space module - provides add_space pyfunction

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::state::Widgets;
use crate::state::get_id;
use crate::state::set_state_of_widget;
use crate::widgets::ipg_space::Space;


/// Add a space widget.
///
/// An empty widget used to add blank space between other widgets.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this space belongs to.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, Optional
///     Whether the space fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, Optional
///     Whether the space fills available height.
/// fill : bool, Optional
///     Whether to fill both the width and height.
/// show : bool, default True
///     Whether the space is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created space.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    gen_id=None, 
    width=None,
    width_fill=None, 
    height=None, 
    height_fill=None,
    fill=None,
    show=true
    ))]
pub fn add_space(
    parent_id: String,
    gen_id: Option<usize>,
    width: Option<f32>,
    width_fill: Option<bool>, 
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    show: bool,
    ) -> PyResult<usize>
{

    let id = get_id(gen_id);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::Space(
        Space {
            id,
            width,
            width_fill,
            height,
            height_fill,
            fill,
            show,
        }));

    drop(state);
    Ok(id)

}
