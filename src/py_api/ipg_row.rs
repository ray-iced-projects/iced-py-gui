//! Row module - provides add_row pyfunction

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::py_api::helpers::{get_height, get_padding_f64, get_width};
use crate::state::{IpgContainers, get_id, set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::enums::IpgAlignment;
use crate::widgets::row::IpgRow;


/// Add a row widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
        window_id, 
        container_id, 
        parent_id=None,
        align=IpgAlignment::Start, 
        width=None, 
        height=None, 
        width_fill=false, 
        height_fill=false,
        padding=vec![0.0], 
        spacing=10.0, 
        clip=false,
        show=true,
        ))]
pub fn add_row(
    window_id: String,
    container_id: String,
    // required above
    parent_id: Option<String>,
    align: IpgAlignment,
    width: Option<f32>,
    height: Option<f32>,
    width_fill: bool,
    height_fill: bool,
    padding: Vec<f64>,
    spacing: f32,
    clip: bool,
    show: bool,
    ) -> PyResult<usize> 
{
    let id = get_id(None);

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

    let padding = get_padding_f64(padding);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_row".to_string());

    state.containers.
        insert(id, IpgContainers::IpgRow(IpgRow::new(
            id,  
            show, 
            spacing, 
            padding, 
            width, 
            height, 
            align,
            clip,
        )));

    drop(state);         
    Ok(id)

}
    