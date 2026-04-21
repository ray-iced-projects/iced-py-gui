//! ColorPicker module - provides add_button pyfunction
use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::graphics::colors::Color;
use crate::{add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{Widgets, access_state, get_id, set_state_of_widget};
use crate::widgets::ipg_button::{ButtonStyleStd};
use crate::widgets::ipg_color_picker::{ColorPicker};
use crate::graphics::bootstrap_arrow::Arrow;


/// Add a color_picker widget.
///
/// A color picker that opens from a button, allowing the user
/// to select a color interactively.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this color picker belongs to.
/// label : str,  Optional
///     Sets the Text label displayed on the button.
/// gen_id : int,  Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// on_press : callable,  Optional
///     Sets the Callback method to invoke when the button is pressed.
/// on_select : callable,  Optional
///     Sets the Callback method to invoke when a color is selected.
/// on_cancel : callable,  Optional
///     Sets the Callback method to invoke when the color selection is cancelled.
/// color : Color,  Optional
///     Sets the initial color using a predefined color variant.
/// color_alpha : float,  Optional
///     Sets the alpha of the Color.
/// color_rgba : list of float,  Optional
///     Sets the initial color in rgba format as [r, g, b, a].
/// width : float,  Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the button fills available width.
/// height : float,  Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the button fills available height.
/// padding : list of float,  Optional
///     Sets the Padding as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// clip : bool,  Optional
///     Whether to clip content that overflows the button.
/// style_id : int,  Optional
///     Sets the ID of a custom style created with ``add_button_style``.
/// style_std : ButtonStyleStd,  Optional
///     Sets the predefined standard style variant.
/// style_arrow : Arrow,  Optional
///     Sets the arrow icon style for the button.
/// user_data : Any,  Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default False
///     Whether the color picker is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created color picker.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    label=None, 
    gen_id=None, 
    on_press=None, 
    on_select=None, 
    on_cancel=None,
    color=None,
    color_alpha=None,
    color_rgba=None, 
    width=None,
    width_fill=None,  
    height=None, 
    height_fill=None,
    fill=None, 
    padding=None, 
    clip=None, 
    style_id=None, 
    style_std=None, 
    style_arrow=None,
    user_data=None,
    show=false, 
    ))]
pub fn add_color_picker(
    parent_id: String,
    // ** above required
    label: Option<String>,
    gen_id: Option<usize>,
    on_press: Option<PyObject>,
    on_select: Option<PyObject>,
    on_cancel: Option<PyObject>,
    color: Option<Color>,
    color_alpha: Option<f32>,
    color_rgba: Option<[f32; 4]>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    padding: Option<Vec<f32>>,
    clip: Option<bool>,
    style_id: Option<usize>,
    style_std: Option<ButtonStyleStd>,
    style_arrow: Option<Arrow>,
    user_data: Option<PyObject>,
    show: bool,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    if let Some(py) = on_press {
        add_callback_to_mutex(id, "on_press".to_string(), py);
    }

    if let Some(py) = on_select {
        add_callback_to_mutex(id, "on_select".to_string(), py);
    }

    if let Some(py) = on_cancel {
        add_callback_to_mutex(id, "on_cancel".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::ColorPicker(
        ColorPicker {
            id,
            show,
            color,
            color_alpha,
            color_rgba,
            // button related
            label,
            width,
            width_fill,  
            height, 
            height_fill,
            fill, 
            padding,
            clip,
            style_id,
            style_std,
            style_arrow,                             
            }));

    drop(state);
    Ok(id)

}
