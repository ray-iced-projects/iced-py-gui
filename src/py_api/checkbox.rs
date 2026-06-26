//! Checkbox module - provides add_checkbox pyfunction

use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::bootstrap::bootstrap_icon::Icon;
use crate::graphics::colors::Color;
use crate::state::{Widgets, access_state, 
    add_callback_to_mutex, get_id, set_state_of_widget};
use crate::widgets::ipg_checkbox::{CheckBox, CheckboxStyle, CheckboxStyleStd};


/// Add a checkbox widget.
///
/// A checkbox with a text label that can be toggled on and off.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this checkbox belongs to.
/// on_toggle : callable,  Optional
///     Sets the Callback method to invoke when the checkbox is toggled.
/// is_checked : bool, default False
///     Whether the checkbox starts in the checked state.
/// label : str,  Optional
///     Sets the Text label displayed next to the checkbox.
/// width : float,  Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the checkbox fills available width.
/// size : float,  Optional
///     Sets the size of the checkbox square in logical pixels.
/// spacing : float,  Optional
///     Sets the Spacing between the checkbox square and the label.
/// text_size : float,  Optional
///     Sets the Font size for the label text.
/// text_line_height : float,  Optional
///     Sets the Line height for the label text.
/// text_wrapping_none: Optional[bool]
///     Sets the wrapping mode of the label to no wrapping, default is on a word.
/// text_wrapping_glyph: Optional[bool]
///     Sets the wrapping mode of the label to wrap on a glyph.
/// text_wrapping_word_glyph: Optional[bool]
///     Sets the wrapping mode of the label to wrap on a glyph or glyph.
/// text_font_id : int,  Optional
///     Sets the Font ID for the label text.
/// icon_font_id : int,  Optional
///     Sets the Font ID for the checkbox icon.
/// icon : Icon,  Optional
///     Sets the Icon displayed inside the checkbox when checked.
/// icon_size : float,  Optional
///     Sets the Size of the checkbox icon.
/// icon_line_height : float,  Optional
///     Sets the Line height of the checkbox icon.
/// user_data : Any,  Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the checkbox is visible.
/// style_id : int,  Optional
///     Sets the ID of a custom style created with ``add_checkbox_style``.
/// style_std : CheckboxStyleStd,  Optional
///     Sets the predefined standard style variant.
/// gen_id : int,  Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created checkbox.
/// 
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    on_toggle=None, 
    is_checked=false, 
    label=None, 
    width=None, 
    fill=None, 
    size=None, 
    spacing=None, 
    text_size=None,
    line_height=None,
    text_wrapping_none=None,
    text_wrapping_glyph=None,
    text_wrapping_word_glyph=None,
    text_font_id=None,
    icon_font_id= None,
    icon=None,
    icon_size=None,
    icon_line_height=None,
    user_data=None, 
    show=true,
    disabled=None,
    style_id=None, 
    style_std=None,
    palette_id=None,
    gen_id=None, 
    ))] 
pub fn add_checkbox(
    parent_id: String,
    on_toggle: Option<PyObject>,
    is_checked: bool,
    label: Option<String>,
    width: Option<f32>,
    fill: Option<bool>,
    size: Option<f32>,
    spacing: Option<f32>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    text_wrapping_none: Option<bool>,
    text_wrapping_glyph: Option<bool>,
    text_wrapping_word_glyph: Option<bool>,
    text_font_id: Option<usize>,
    icon_font_id: Option<usize>,
    icon: Option<Icon>,
    icon_size: Option<f32>,
    icon_line_height: Option<f32>,
    user_data: Option<PyObject>,
    show: bool,
    disabled: Option<bool>,
    style_id: Option<usize>,
    style_std: Option<CheckboxStyleStd>,
    palette_id: Option<usize>,
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);
    
    if let Some(py) = on_toggle {
        add_callback_to_mutex(id, "on_toggle".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::CheckBox(
        CheckBox {
            id,
            show,
            is_checked,
            label,
            width,
            fill,
            size,
            spacing,
            text_size,
            line_height,
            text_wrapping_none,
            text_wrapping_glyph,
            text_wrapping_word_glyph,
            text_font_id,
            icon_font_id,
            icon,
            icon_size,
            icon_line_height,
            disabled,
            style_id,
            style_std,
            palette_id,
            }));

    drop(state);
    Ok(id)

}



/// Adds styling to container
/// 
/// Parameters
/// ----------
/// border_radius: float
///     The radius of the 4 corners.
/// border_width: float
///     The border width.
/// text_color: Color, Optional
///     The text color.
/// text_color_alpha: float, Optional
///     Sets the alpha of the Color.
/// text_rgba: list[float], Optional
///     The text color in rgba format.
///  gen_id : int,  Optional
///      Obtains an ID of a widget that have not been created, used for the gen_id parameter.
#[pyfunction]
#[pyo3(signature = ( 
    border_radius=None, 
    border_width=None,
    text_color=None,
    text_color_alpha=None,
    text_rgba=None,
    gen_id=None
    ))]
pub fn add_checkbox_style(
    border_radius: Option<f32>,
    border_width: Option<f32>,
    text_color: Option<Color>,
    text_color_alpha: Option<f32>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::CheckboxStyle(
        CheckboxStyle {
            id,
            border_radius, 
            border_width,
            text_color,
            text_color_alpha,
            text_rgba,
        }));

    drop(state);
    Ok(id)
}