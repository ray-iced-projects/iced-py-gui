//! Toggler provide add_toggle and add_toggle_style to a python function
use pyo3::{pyfunction, Py, PyAny, PyResult};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex, graphics::colors::Color, 
    state::{Widgets, get_id, 
        set_state_of_widget}, widgets::{  
        ipg_toggle::{Toggler, TogglerStyle}}};


/// Add a toggler widget.
///
/// A toggle switch with an optional text label.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this toggler belongs to.
/// label : str, Optional
///     Sets the Text label displayed next to the toggler.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// toggled : callable, Optional
///     Sets the Callback method to invoke when the toggler is toggled.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the toggler fills available width.
/// size : float, Optional
///     Sets the size of the toggler in logical pixels.
/// text_size : float, Optional
///     Sets the Font size for the label text.
/// text_line_height : float, Optional
///     Sets the Line height for the label text.
/// text_center : bool, Optional
///     Whether to Align the label to the centre.
/// text_left : bool, Optional
///     Whether to Align the label to the left.
/// text_right : bool, Optional
///     Whether to Align the label to the right.
/// spacing : float, Optional
///     Sets the Spacing between the toggler and the label.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the toggler is visible.
/// font_id : int, Optional
///     Sets the Font ID for the label text.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_toggler_style``.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created toggler.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    label=None, 
    gen_id=None, 
    toggled=None, 
    width=None, 
    width_fill=None, 
    size=None, 
    text_size=None,
    text_line_height=None, 
    text_center=None,
    text_left=None,
    text_right=None,
    wrapping_none=None,
    wrapping_glyph=None,
    wrapping_word_glyph=None,
    spacing=None, 
    user_data=None, 
    show=true,
    font_id=None, 
    style_id=None, 
    ))]
pub fn add_toggler(
    parent_id: String,
    label: Option<String>,
    gen_id: Option<usize>,
    toggled: Option<PyObject>,
    width: Option<f32>,
    width_fill: Option<bool>,
    size: Option<f32>,
    text_size: Option<f32>,
    text_line_height: Option<f32>,
    text_center: Option<bool>,
    text_left: Option<bool>,
    text_right: Option<bool>,
    wrapping_none: Option<bool>,
    wrapping_glyph: Option<bool>,
    wrapping_word_glyph: Option<bool>,
    spacing: Option<f32>,
    user_data: Option<PyObject>,
    show: bool,
    font_id: Option<usize>,
    style_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    if let Some(py) = toggled {
        add_callback_to_mutex(id, "toggled".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::Toggler(
        Toggler {
            id,
            show,
            label,
            width,
            width_fill,
            is_toggled: false,
            size,
            text_size,
            text_line_height,
            text_center,
            text_left,
            text_right,
            wrapping_none,
            wrapping_glyph,
            wrapping_word_glyph,
            spacing,
            font_id,
            style_id,                           
        }));

    drop(state);
    Ok(id)

}


/// Add styling to a toggler.
///
/// Creates a custom style that can be applied to a toggler
/// via its ``style_id`` parameter.
///
/// Parameters
/// ----------
/// background_color : Color, Optional
///     Sets the background color using a predefined color variant.
/// background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// background_rgba : list of float, Optional
///     Sets the background color in rgba format as [r, g, b, a].
/// background_border_color : Color, Optional
///     Sets the background border color using a predefined color variant.
/// background_border_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// background_border_rgba : list of float, Optional
///     Sets the background border color in rgba format as [r, g, b, a].
/// background_border_width : float, Optional
///     Sets the background border width in logical pixels.
/// foreground_color : Color, Optional
///     Sets the foreground (thumb) color using a predefined color variant.
/// foreground_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// foreground_rgba : list of float, Optional
///     Sets the foreground (thumb) color in rgba format as [r, g, b, a].
/// foreground_border_color : Color, Optional
///     Sets the foreground border color using a predefined color variant.
/// foreground_border_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// foreground_border_rgba : list of float, Optional
///     Sets the foreground border color in rgba format as [r, g, b, a].
/// foreground_border_width : float, Optional
///     Sets the foreground border width in logical pixels.
/// text_color : Color, Optional
///     Sets the text color using a predefined color variant.
/// text_alpha : float, Optional
///     Sets the alpha of the Color.
/// text_color_rgba : list of float, Optional
///     Sets the text color in rgba format as [r, g, b, a].
/// border_radius : list of float, Optional
///     Sets the radius of the corners as [all] or
///     [top-left, top-right, bottom-right, bottom-left].
/// padding_ratio : float, Optional
///     Sets the padding ratio for the toggler.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a toggler's ``style_id``.
#[pyfunction]
#[pyo3(signature = (
    background_color=None,
    background_color_alpha=None,
    background_rgba=None,
    background_border_color=None,
    background_border_color_alpha=None,
    background_border_rgba=None,
    background_border_width=None,
    foreground_color=None,
    foreground_color_alpha=None,
    foreground_rgba=None,
    foreground_border_color=None,
    foreground_border_color_alpha=None,
    foreground_border_rgba=None,
    foreground_border_width=None,
    text_color=None,
    text_color_alpha=None,
    text_rgba=None, 
    border_radius=None, 
    padding_ratio=None, 
    gen_id=None,
    ))]
pub fn add_toggler_style(
    background_color: Option<Color>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    background_border_color: Option<Color>,
    background_border_color_alpha: Option<f32>,
    background_border_rgba: Option<[f32; 4]>,
    background_border_width: Option<f32>,
    foreground_color: Option<Color>,
    foreground_color_alpha: Option<f32>,
    foreground_rgba: Option<[f32; 4]>,
    foreground_border_color: Option<Color>,
    foreground_border_color_alpha: Option<f32>,
    foreground_border_rgba: Option<[f32; 4]>,
    foreground_border_width: Option<f32>,
    text_color: Option<Color>,
    text_color_alpha: Option<f32>,
    text_rgba: Option<[f32; 4]>, 
    border_radius: Option<Vec<f32>>,
    padding_ratio: Option<f32>, 
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();
    
    state.widgets.insert(id, Widgets::TogglerStyle(
        TogglerStyle {
            id,
            background_color,
            background_color_alpha,
            background_rgba,
            background_border_color,
            background_border_color_alpha,
            background_border_rgba,
            background_border_width,
            foreground_color,
            foreground_color_alpha,
            foreground_rgba,
            foreground_border_color,
            foreground_border_color_alpha,
            foreground_border_rgba,
            foreground_border_width,
            text_color,
            text_color_alpha,
            text_rgba, 
            border_radius, 
            padding_ratio, 
        }));

    drop(state);
    Ok(id)

}