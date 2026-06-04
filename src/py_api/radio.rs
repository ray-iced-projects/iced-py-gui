//! Radio adds add_radio to the pyfunction

use pyo3::{pyfunction, PyResult, Py, PyAny};
type PyObject = Py<PyAny>;
use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::Color,
    state::{Widgets, get_id, set_state_of_widget}, 
    widgets::{ipg_radio::{Radio, RadioStyle}}};


/// Add a radio button group widget.
///
/// A group of radio buttons where the user can select one option
/// from a list of labels.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this radio group belongs to.
/// labels : list of str
///     Sets the list of labels for each radio button.
/// horizontal : bool, default false
///     Whether the layout direction is horizontal (default vertical).
/// spacing : float, Optional
///     Sets the spacing between the radio circle and its label.
/// radio_spacing: float, Optional
///     Sets the spacing between radio buttons.
/// padding : list of float, Optional
///     Sets the Padding as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the radio group fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the radio group fills available height.
/// on_selected : callable, Optional
///     Sets the Callback method to invoke when a radio button is selected.
/// selected_index : int, Optional
///     Sets the index of the initially selected radio button.
/// size : float, Optional
///     Sets the size of the radio circle in logical pixels.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_radio_style``.
/// font_id : int, Optional
///     Sets the Font ID for the label text.
/// text_spacing : float, Optional
///     Sets the spacing between the radio circle and text.
/// text_size : float, Optional
///     Sets the Font size for the label text.
/// line_height : float, Optional
///     Sets the Line height for the label text.
/// wrapping : TextWrapping, Optional
///     Sets the Text wrapping strategy for the labels.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the radio group is visible.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created radio group.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    labels, 
    horizontal=None,
    spacing=None,
    radio_spacing=None,
    radio_wrap=None,
    radio_wrap_spacing=None,
    radio_wrap_align_start=None,
    radio_wrap_align_center=None,
    radio_wrap_align_end=None,
    padding=None, 
    width=None, 
    width_fill=None, 
    height=None, 
    height_fill=None,
    fill=None,
    on_selected=None, 
    selected_index=None, 
    size=None, 
    style_id=None,
    font_id=None,
    text_size=None,
    line_height=None, 
    text_wrapping_none=None,
    text_wrapping_glyph=None,
    text_wrapping_word_glyph=None,
    user_data=None, 
    show=true, 
    gen_id=None,
    ))]
pub fn add_radio(
    parent_id: String,
    labels: Vec<String>,
    horizontal: Option<bool>,
    spacing: Option<f32>,
    radio_spacing: Option<f32>,
    radio_wrap: Option<bool>,
    radio_wrap_spacing: Option<f32>,
    radio_wrap_align_start: Option<bool>,
    radio_wrap_align_center: Option<bool>,
    radio_wrap_align_end: Option<bool>,
    padding: Option<Vec<f32>>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    on_selected: Option<PyObject>,
    selected_index: Option<usize>,
    size: Option<f32>,
    style_id: Option<usize>,
    font_id: Option<usize>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    text_wrapping_none: Option<bool>,
    text_wrapping_glyph: Option<bool>,
    text_wrapping_word_glyph: Option<bool>,
    user_data: Option<PyObject>,
    show: bool,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let selected_index = if let Some(val) = selected_index {
        if val > labels.len()-1 {
            panic!("Radio selected_index is greater than the size of the labels")
        } else { Some(val) }
    } else { None };

    if let Some(py) = on_selected {
        add_callback_to_mutex(id, "on_selected".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();
    
    state.widgets.insert(id, Widgets::Radio(
        Radio {
            id,
            labels,
            horizontal,
            spacing,
            radio_spacing,
            radio_wrap,
            radio_wrap_spacing,
            radio_wrap_align_start,
            radio_wrap_align_center,
            radio_wrap_align_end,
            padding,
            selected_index,
            width,
            width_fill,
            height,
            height_fill,
            fill,
            size,
            text_size,
            line_height,
            text_wrapping_none,
            text_wrapping_glyph,
            text_wrapping_word_glyph,
            font_id,
            style_id,
            show,
        }));

    drop(state);                                      
    Ok(id)

}


/// Add styling to a radio button group.
///
/// Creates a custom style that can be applied to a radio group
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
/// background_color_hovered : Color, Optional
///     Sets the background color when hovered using a predefined color variant.
/// background_color_hovered_alpha : float, Optional
///     Sets the alpha of the Color.
/// background_rgba_hovered : list of float, Optional
///     Sets the background color when hovered in rgba format as [r, g, b, a].
/// border_color : Color, Optional
///     Sets the border color using a predefined color variant.
/// border_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// border_rgba : list of float, Optional
///     Sets the border color in rgba format as [r, g, b, a].
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// dot_color : Color, Optional
///     Sets the dot color using a predefined color variant.
/// dot_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// dot_rgba : list of float, Optional
///     Sets the dot color in rgba format as [r, g, b, a].
/// dot_color_hovered : Color, Optional
///     Sets the dot color when hovered using a predefined color variant.
/// dot_color_hovered_alpha : float, Optional
///     Sets the alpha of the Color.
/// dot_rgba_hovered : list of float, Optional
///     Sets the dot color when hovered in rgba format as [r, g, b, a].
/// text_color : Color, Optional
///     Sets the text color using a predefined color variant.
/// text_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// text_rgba : list of float, Optional
///     Sets the text color in rgba format as [r, g, b, a].
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a radio group's ``style_id``.
#[pyfunction]
#[pyo3(signature = (
    background_color=None,
    background_color_alpha=None,
    background_rgba=None,
    background_color_hovered=None,
    background_color_hovered_alpha=None,
    background_rgba_hovered=None,
    border_color=None,
    border_color_alpha=None,
    border_rgba=None,
    border_width=None,
    dot_color=None,
    dot_color_alpha=None,
    dot_rgba=None,
    dot_color_hovered=None,
    dot_color_hovered_alpha=None,
    dot_rgba_hovered=None,
    text_color=None,
    text_color_alpha=None,
    text_rgba=None,
    gen_id=None
    ))]
pub fn add_radio_style(
    background_color: Option<Color>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    background_color_hovered: Option<Color>,
    background_color_hovered_alpha: Option<f32>,
    background_rgba_hovered: Option<[f32; 4]>,
    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_width: Option<f32>,
    dot_color: Option<Color>,
    dot_color_alpha: Option<f32>,
    dot_rgba: Option<[f32; 4]>,
    dot_color_hovered: Option<Color>,
    dot_color_hovered_alpha: Option<f32>,
    dot_rgba_hovered: Option<[f32; 4]>,
    text_color: Option<Color>,
    text_color_alpha: Option<f32>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::RadioStyle(
        RadioStyle {
            id,
            background_color,
            background_color_alpha,
            background_rgba,
            background_color_hovered,
            background_color_hovered_alpha,
            background_rgba_hovered,
            border_color,
            border_color_alpha,
            border_rgba,
            border_width,
            dot_color,
            dot_color_alpha,
            dot_rgba,
            dot_color_hovered,
            dot_color_hovered_alpha,
            dot_rgba_hovered,
            text_color,
            text_color_alpha,
            text_rgba,
        }));

    drop(state);
    Ok(id)

}
