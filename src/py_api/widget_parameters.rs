//! Update module - provides update_widget, delete_widget, add_widget, and show_widget pyfunctions

use pyo3::{Py, PyAny, pyfunction};
use pyo3::types::PyDict;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::state::{access_update_widgets, access_callback_widgets, Widgets};
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


///"""
///Returns a dict of all parameters for the widget with the given id.
///
///Must be called during a callback (e.g. a button's on_press handler). All
///widgets ids can be used.
///
///Parameters
///----------
///widget_id : int
///    A widget's id.
///
///Returns
///-------
///dict
///    Field name -> value for every parameter of the widget.
///"""
#[pyfunction]
#[pyo3(signature = (widget_id))]
pub fn get_widget_parameters(py: Python<'_>, widget_id: usize) -> PyResult<PyObject> {
    let widget = {
        let snapshot = access_callback_widgets();
        snapshot.get(&widget_id).cloned()
    };

    match widget {
        Some(Widgets::Button(button)) => {
            Ok(button.to_py_dict(py)?.into_any().unbind())
        }
        Some(Widgets::CheckBox(checkbox)) => {
            Ok(checkbox.to_py_dict(py)?.into_any().unbind())
        }
        Some(other) => Err(PyValueError::new_err(format!(
            "get_widget_parameters does not yet support {other:?}"))),
        None => Err(PyValueError::new_err(format!(
            "No widget snapshot found for id {widget_id}. \
             get_widget_parameters must be called during a widget callback."))),
    }
}


///"""
///Returns a dict of all style parameters for the style with the given id.
///
///Must be called during a callback whose style_id is set.
///
///Parameters
///----------
///style_id : int
///    A widget's style_id.
///
///Returns
///-------
///dict
///    Field name -> value for every style parameter.
///"""
#[pyfunction]
#[pyo3(signature = (style_id))]
pub fn get_widget_style_parameters(py: Python<'_>, style_id: usize) -> PyResult<PyObject> {
    let widget = {
        let snapshot = access_callback_widgets();
        snapshot.get(&style_id).cloned()
    };

    match widget {
        Some(Widgets::ButtonStyle(style)) => {
            Ok(style.to_py_dict(py)?.into_any().unbind())
        }
        Some(Widgets::CheckboxStyle(style)) => {
            Ok(style.to_py_dict(py)?.into_any().unbind())
        }
        Some(other) => Err(PyValueError::new_err(format!(
            "get_widget_style_parameters does not yet support {other:?}"))),
        None => Err(PyValueError::new_err(format!(
            "No style snapshot found for id {style_id}. \
             get_widget_style_parameters must be called during a callback or sid is wrong."))),
    }
}


///"""
///Returns a dict of all font parameters for the font with the given id.
///
///Must be called during a callback whose font_id is set.
///
///Parameters
///----------
///font_id : int
///    A widget's font_id.
///
///Returns
///-------
///dict
///    Field name -> value for every font parameter.
///"""
#[pyfunction]
#[pyo3(signature = (font_id))]
pub fn get_widget_font_parameters(py: Python<'_>, font_id: usize) -> PyResult<PyObject> {
    let widget = {
        let snapshot = access_callback_widgets();
        snapshot.get(&font_id).cloned()
    };

    match widget {
        Some(Widgets::Font(font)) => {
            Ok(font.to_py_dict(py)?.into_any().unbind())
        }
        Some(_) => Err(PyValueError::new_err(
            "get_widget_font_parameters: id does not reference a font")),
        None => Err(PyValueError::new_err(format!(
            "No font snapshot found for id {font_id}. \
             get_widget_font_parameters must be called during acallback or the fid is wrong."))),
    }
}


///"""
///Returns a dict of all palette parameters for the palette with the given id.
///
///Must be called during a callback whose's palette_id is set.
///
///Parameters
///----------
///palette_id : int
///    A widget's palette_id.
///
///Returns
///-------
///dict
///    Palette pairs (base, weak, strong, ...) and status mappings.
///"""
#[pyfunction]
#[pyo3(signature = (palette_id))]
pub fn get_widget_palette_parameters(py: Python<'_>, palette_id: usize) -> PyResult<PyObject> {
    let widget = {
        let snapshot = access_callback_widgets();
        snapshot.get(&palette_id).cloned()
    };

    match widget {
        Some(Widgets::Palette(palette)) => {
            Ok(palette.to_py_dict(py)?.into_any().unbind())
        }
        Some(_) => Err(PyValueError::new_err(
            "get_widget_palette_parameters: id does not reference a palette")),
        None => Err(PyValueError::new_err(format!(
            "No palette snapshot found for id {palette_id}. \
             get_widget_palette_parameters must be called during a callback or the pid is wrong."))),
    }
}
