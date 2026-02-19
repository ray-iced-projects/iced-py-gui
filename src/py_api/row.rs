//! Row module - provides add_row pyfunction

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::py_api::helpers::{get_height, get_width};
use crate::state::{IpgContainers, get_id, 
    set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::enums::IpgAlignment;
use crate::widgets::ipg_row::IpgRow;


/// Add a row widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
        window_id, 
        container_id, 
        parent_id=None,
        width=None,
        width_fill=false,  
        height=None, 
        height_fill=false,
        align_y=None,
        padding=None, 
        spacing=None, 
        clip=None,
        show=true,
        ))]
pub fn add_row(
    window_id: String,
    container_id: String,
    // required above
    parent_id: Option<String>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    align_y: Option<IpgAlignment>,
    padding: Option<Vec<f32>>,
    spacing: Option<f32>,
    clip: Option<bool>,
    show: bool,
    ) -> PyResult<usize> 
{
    let id = get_id(None);

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_row".to_string());

    state.containers.
        insert(id, IpgContainers::IpgRow(
            IpgRow {
            id,  
            show, 
            spacing, 
            padding, 
            width, 
            height, 
            align_y,
            clip,
        }));

    drop(state);         
    Ok(id)

}
    