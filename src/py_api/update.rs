//! Update module - provides update_widget, delete_widget, add_widget, and show_widget pyfunctions

use pyo3::{Py, PyAny, pyfunction};
use pyo3::types::PyDict;
use pyo3::prelude::*;

use crate::state::access_update_widgets;
type PyObject = Py<PyAny>;

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

#[pyfunction]
#[pyo3(signature = (wid))]
pub fn delete_widget(wid: usize) 
{
    let mut all_updates = access_update_widgets();

    all_updates.deletes.push(wid);

    drop(all_updates);
}

#[pyfunction]
#[pyo3(signature = (wid))]
pub fn show_widget(wid: usize)
{
    let mut all_updates = access_update_widgets();

    all_updates.shows.push((wid, true));

    drop(all_updates);
}

#[pyfunction]
#[pyo3(signature = (wid))]
pub fn hide_widget(wid: usize)
{
    let mut all_updates = access_update_widgets();

    all_updates.shows.push((wid, false));

    drop(all_updates);
}

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

