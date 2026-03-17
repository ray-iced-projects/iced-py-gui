//! Toggler provide add_toggle and add_toggle_style to a python function
use pyo3::{pyfunction, Py, PyAny, PyResult};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex, graphics::colors::IpgColor, 
    py_api::helpers::get_length, state::{IpgWidgets, get_id, 
        set_state_of_widget}, widgets::{ 
        ipg_text::{TextShaping, TextWrapping}, 
        ipg_toggle::{IpgToggler, IpgTogglerStyle}}};


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
/// text_shaping : TextShaping, Optional
///     Sets the Text shaping strategy for the label.
/// text_wrapping : TextWrapping, Optional
///     Sets the Text wrapping strategy for the label.
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
    width_fill=false, 
    size=None, 
    text_size=None,
    text_line_height=None, 
    text_center=None,
    text_left=None,
    text_right=None,
    text_shaping=None,
    text_wrapping=None, 
    spacing=None, 
    user_data=None, 
    show=true,
    font_id=None, 
    style_id=None, 
    ))]
pub fn add_toggler(
    parent_id: String,
    // ** above required
    label: Option<String>,
    gen_id: Option<usize>,
    toggled: Option<PyObject>,
    width: Option<f32>,
    width_fill: bool,
    size: Option<f32>,
    text_size: Option<f32>,
    text_line_height: Option<f32>,
    text_center: Option<bool>,
    text_left: Option<bool>,
    text_right: Option<bool>,
    text_shaping: Option<TextShaping>,
    text_wrapping: Option<TextWrapping>,
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

    let width = get_length(width, width_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgToggler(
        IpgToggler {
            id,
            parent_id,
            show,
            label,
            width,
            is_toggled: false,
            size,
            text_size,
            text_line_height,
            text_center,
            text_left,
            text_right,
            text_shaping,
            text_wrapping,
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
/// background_color : IpgColor, Optional
///     Sets the background color using a predefined color variant.
/// background_rgba : list of float, Optional
///     Sets the background color in rgba format as [r, g, b, a].
/// background_border_color : IpgColor, Optional
///     Sets the background border color using a predefined color variant.
/// background_border_rgba : list of float, Optional
///     Sets the background border color in rgba format as [r, g, b, a].
/// background_border_width : float, Optional
///     Sets the background border width in logical pixels.
/// foreground_color : IpgColor, Optional
///     Sets the foreground (thumb) color using a predefined color variant.
/// foreground_rgba : list of float, Optional
///     Sets the foreground (thumb) color in rgba format as [r, g, b, a].
/// foreground_border_color : IpgColor, Optional
///     Sets the foreground border color using a predefined color variant.
/// foreground_border_rgba : list of float, Optional
///     Sets the foreground border color in rgba format as [r, g, b, a].
/// foreground_border_width : float, Optional
///     Sets the foreground border width in logical pixels.
/// text_ipg_color : IpgColor, Optional
///     Sets the text color using a predefined color variant.
/// text_rgba_color : list of float, Optional
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
    background_rgba=None,
    background_border_color=None,
    background_border_rgba=None,
    background_border_width=None,
    foreground_color=None,
    foreground_rgba=None,
    foreground_border_color=None,
    foreground_border_rgba=None,
    foreground_border_width=None,
    text_ipg_color=None,
    text_rgba_color=None, 
    border_radius=None, 
    padding_ratio=None, 
    gen_id=None,
    ))]
pub fn add_toggler_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    background_border_color: Option<IpgColor>,
    background_border_rgba: Option<[f32; 4]>,
    background_border_width: Option<f32>,
    foreground_color: Option<IpgColor>,
    foreground_rgba: Option<[f32; 4]>,
    foreground_border_color: Option<IpgColor>,
    foreground_border_rgba: Option<[f32; 4]>,
    foreground_border_width: Option<f32>,
    text_ipg_color: Option<IpgColor>,
    text_rgba_color: Option<[f32; 4]>, 
    border_radius: Option<Vec<f32>>,
    padding_ratio: Option<f32>, 
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let background_border_color = 
        IpgColor::rgba_ipg_color_to_iced(background_border_rgba, background_border_color, 1.0, false);
    let foreground_color = 
        IpgColor::rgba_ipg_color_to_iced(foreground_rgba, foreground_color, 1.0, false);
    let foreground_border_color = 
        IpgColor::rgba_ipg_color_to_iced(foreground_border_rgba, foreground_border_color, 1.0, false);
    
    let text_color = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba_color, text_ipg_color, 1.0, false);

    let mut state = access_state();
    
    state.widgets.insert(id, IpgWidgets::IpgTogglerStyle(
        IpgTogglerStyle {
            id,
            background_color,
            background_border_color,
            background_border_width,
            foreground_color,
            foreground_border_color,
            foreground_border_width, 
            text_color, 
            border_radius, 
            padding_ratio, 
        }));

    drop(state);
    Ok(id)

}