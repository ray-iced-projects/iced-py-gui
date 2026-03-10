//! Space module - provides add_space pyfunction
use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::py_api::helpers::get_length;
use crate::state::IpgWidgets;
use crate::state::get_id;
use crate::state::set_state_of_widget;
use crate::widgets::ipg_space::IpgSpace;


/// Add a space widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    gen_id=None, 
    width=None,
    width_fill=false, 
    height=None, 
    height_fill=false, 
    show=true
    ))]
pub fn add_space(
    parent_id: String,
    gen_id: Option<usize>,
    width: Option<f32>,
    width_fill: bool, 
    height: Option<f32>,
    height_fill: bool,
    show: bool,
    ) -> PyResult<usize>
{

    let id = get_id(gen_id);

    let width = get_length(width, width_fill);
    let height = get_length(height, height_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgSpace(
        IpgSpace {
            id,
            parent_id,
            width,
            height,
            show,
        }));

    drop(state);
    Ok(id)

}
