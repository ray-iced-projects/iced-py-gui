//! Update module - provides update_widget, delete_widget, add_widget, and show_widget pyfunctions

use pyo3::{Py, PyAny, pyfunction};
use pyo3::types::PyDict;
use pyo3::prelude::*;

use crate::state::access_update_widgets;
type PyObject = Py<PyAny>;


///"""
/// Adds a update_widget function.
///
/// A method to dynamically update a widget 
/// with a single parameter
///
/// Parameters
/// ----------
/// wid : int
///     Sets the widget ID that is to be updated.
/// param:  Any class type ending in Param 
///     Sets the parameters to be updated, i.e. ButtonParam.Height
/// value: Any
///     The value required by the class, i.e. the ButtonParam.Height: float
/// 
/// Returns
/// -------
/// None
///"""
#[pyfunction]
#[pyo3(signature = (wid, param, value))]
pub fn update_widget(
    wid: usize, 
    param: PyObject, 
    value: PyObject) 
{

    let mut all_updates = access_update_widgets();

    all_updates.updates.push((wid, param, value));

    drop(all_updates);
}


///"""
///Adds a update_widget_params function.
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
pub fn update_widget_params(
    wid: usize,
    updates: &Bound<'_, PyDict>,
) {
    let mut all_updates = access_update_widgets();
    for (param, value) in updates.iter() {
        all_updates.updates.push((wid, param.unbind(), value.unbind()));
    }
    drop(all_updates);
}


///"""
///Adds a delete_widget function.
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
pub fn delete_widget(wid: usize) 
{
    let mut all_updates = access_update_widgets();

    all_updates.deletes.push(wid);

    drop(all_updates);
}


///"""
///Adds a show_widget function.
//
///A method to dynamically show a hidden widget.
//
///Parameters
///----------
///wid : int
///    Sets the widget ID that is to be targeted for showing.
///
///Returns
///-------
///None
///"""
#[pyfunction]
#[pyo3(signature = (wid))]
pub fn show_widget(wid: usize)
{
    let mut all_updates = access_update_widgets();

    all_updates.shows.push((wid, true));

    drop(all_updates);
}


///"""
///Adds a hide_widget function.
//
///A method to dynamically hide a widget.
//
///Parameters
///----------
///wid : int
///    Sets the widget ID that is to be targeted for hiding.
///
///Returns
///-------
///None
///"""
#[pyfunction]
#[pyo3(signature = (wid))]
pub fn hide_widget(wid: usize)
{
    let mut all_updates = access_update_widgets();

    all_updates.shows.push((wid, false));

    drop(all_updates);
}


///"""
///Adds a move_widget function.
//
///A method to dynamically move a widget.
//
///Parameters
///----------
///wid : int
///    Sets the widget ID that is to be targeted for moving.
///move_after: int, Optional
///    The id of the widget where the moved widget is to be placed after
///move_beforer: int, Optional
///    The id of the widget where the moved widget is to be placed before
///target_parent_id: int, Optional
///    The parent id of the container the widget is placed into.
///Returns
///-------
///None
///"""
#[pyfunction]
#[pyo3(signature = (
    wid, 
    move_after=None,
    move_before=None,
    target_parent_id=None
    ))]
pub fn move_widget(
    wid: usize,
    move_after: Option<usize>,
    move_before: Option<usize>,
    target_parent_id: Option<usize>)
{
    let mut all_updates = access_update_widgets();
    
    all_updates.moves.push((wid, move_after, move_before, target_parent_id));
    
    drop(all_updates);
}

