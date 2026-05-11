//! Draw Update modules - provides update_draw_params, delete_draw_widget, add_draw_widget,

use pyo3::pyfunction;
use pyo3::types::PyDict;
use pyo3::prelude::*;

use crate::state::access_update_canvas_draw;


///"""
///Adds a update_draw_params function.
///
///A method to dynamically update a single widget
///using many parameters in dictionary format
///
///Parameters
///----------
///wid : int
///    Sets the widget ID that is to be updated.
///updates: dict
///    The {parameter: value, ...} pairs in dict format
///    param:  Any class type ending in Param 
///        Sets the parameters to be updated, i.e. ButtonParam.Height
///    value: Any
///        The value required by the class, i.e. ButtonParam.Height: float
///
///Returns
///-------
///None
///"""
#[pyfunction]
#[pyo3(signature = (wid, updates))]
pub fn update_draw_params(
    wid: usize,
    updates: &Bound<'_, PyDict>,
) {
    let mut draw_updates = access_update_canvas_draw();
    for (param, value) in updates.iter() {
        draw_updates.updates.push((wid, param.unbind(), value.unbind()));
    }
    drop(draw_updates);
}


///"""
///Adds a delete_draw_widget function.
//
///A method to dynamically delete a widget.
//
///Parameters
///----------
///wid : int
///    Sets the widget ID that is to be targeted for deletion.
///
///Returns
///-------
///None
///"""
#[pyfunction]
#[pyo3(signature = (wid))]
pub fn delete_draw_widget(wid: usize) 
{
    let mut draw_updates = access_update_canvas_draw();

    draw_updates.deletes.push(wid);

    drop(draw_updates);
}

