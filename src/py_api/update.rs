#![allow(unused)]

use pyo3::{pyfunction, Py, PyAny};
use pyo3_polars::PyDataFrame;

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
#[pyo3(signature = (
    wid, 
    param, 
    value))]
pub fn update_dataframe(
    wid: usize, 
    param: PyObject, 
    value: PyDataFrame) 
{
    let mut all_updates = access_update_widgets();

    all_updates.dataframes.push((wid, param, value));

    drop(all_updates);
}
