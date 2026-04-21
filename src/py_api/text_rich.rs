//! Rich text module - provides add_rich_text and add_span pyfunctions
use pyo3::{pyfunction, PyResult, Py, PyAny};
type PyObject = Py<PyAny>;


use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex};
use crate::graphics::colors::Color;
use crate::state::{Containers, Widgets, get_id, set_state_cont_wnd_ids, set_state_of_container, set_state_of_widget};
use crate::widgets::ipg_text_rich::{RichText, Span};


/// Add a rich text widget.
///
/// A rich text widget that contains styled spans of text.
/// Use ``add_span`` to add text spans to this widget.
///
/// Parameters
/// ----------
/// parent_id : str
///     The id of the parent container.
/// size : float, Optional
///     The default text size for all spans.
/// line_height : float, Optional
///     The default line height for all spans.
/// color : Color, Optional
///     The default text color for all spans.
/// color_alpha : float, Optional
///     Sets the alpha of the Color.
/// rgba : list[float, 4], Optional
///     The default text color in rgba format.
/// show : bool
///     Whether the widget is visible.
/// gen_id : int, Optional
///     Obtains an ID of a widget that has not been created.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created rich text.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id,
    parent_id=None,
    size=None,
    line_height=None,
    width=None,
    width_fill=None,
    height=None,
    height_fill=None,
    fill=None,
    font_id=None,
    color=None,
    color_alpha=None,
    rgba=None,
    align_bottom_center=None,
    align_bottom_left=None,
    align_bottom_right=None,
    align_center_left=None,
    align_center_right=None,
    align_center=None,
    align_top_center=None,
    align_top_left=None,
    align_top_right=None,
    wrapping_none=None,
    wrapping_glyph=None,
    wrapping_word_glyph=None,
    on_link_click=None,
    user_data=None,
    show=true,
))]
pub fn add_rich_text(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    size: Option<f32>,
    line_height: Option<f32>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    font_id: Option<usize>,
    color: Option<Color>,
    color_alpha: Option<f32>,
    rgba: Option<[f32; 4]>,
    align_bottom_center: Option<bool>,
    align_bottom_left: Option<bool>,
    align_bottom_right: Option<bool>,
    align_center_left: Option<bool>,
    align_center_right: Option<bool>,
    align_center: Option<bool>,
    align_top_center: Option<bool>,
    align_top_left: Option<bool>,
    align_top_right: Option<bool>,
    wrapping_none: Option<bool>,
    wrapping_glyph: Option<bool>,
    wrapping_word_glyph: Option<bool>,
    on_link_click: Option<PyObject>,
    user_data: Option<PyObject>,
    show: bool,
) -> PyResult<usize> 
{
    let id = get_id(None);
    
    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    // Store callback if provided
    if let Some(py) = on_link_click {
        add_callback_to_mutex(id, "on_link_click".to_string(), py);
    }

    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_rich_text".to_string());

    state.containers.insert(id, Containers::RichText(
        RichText {
            id,
            size,
            line_height,
            width,
            width_fill,
            height,
            height_fill,
            fill,
            font_id,
            color,
            color_alpha,
            rgba,
            align_bottom_center,
            align_bottom_left,
            align_bottom_right,
            align_center_left,
            align_center_right,
            align_center,
            align_top_center,
            align_top_left,
            align_top_right,
            wrapping_none,
            wrapping_glyph,
            wrapping_word_glyph,
            show,
        }));

    drop(state);
    Ok(id)
}


/// Add a span to a rich text widget.
///
/// A span of styled text within a rich text widget.
///
/// Parameters
/// ----------
/// rich_text_id : int
///     The ID of the rich text widget to add the span to.
/// text : str
///     The text content of the span.
/// size : float, Optional
///     The text size for this span.
/// line_height : float, Optional
///     The line height for this span.
/// color : Color, Optional
///     The text color in Color format.
/// color_alpha : float, Optional
///     Sets the alpha of the Color.
/// rgba : list[float, 4], Optional
///     The text color in rgba format.
/// bold : bool
///     Whether the text is bold.
/// italic : bool
///     Whether the text is italic.
/// underline : bool
///     Whether the text is underlined.
/// strikethrough : bool
///     Whether the text has strikethrough.
/// link : int, Optional
///     The span link identifier passed to the rich text ``on_link_click`` callback.
/// gen_id : int, Optional
///     Obtains an ID for the span.
///
/// Returns
/// -------
/// int
///     The numeric ID of the newly created span.
#[pyfunction]
#[pyo3(signature = (
    parent_id,
    text,
    size=None,
    line_height=None,
    color=None,
    color_alpha=None,
    rgba=None,
    font_id=None,
    underline=None,
    strikethrough=None,
    link=None,
    background_color=None,
    background_color_alpha=None,
    background_rgba=None,
    background_gradient_color_stop=None,
    background_gradient_color_stop_alpha=None,
    background_gradient_rgba_stop=None,
    background_gradient_degrees=None,
    background_gradient_radians=None,
    border_color=None,
    border_color_alpha=None,
    border_rgba=None,
    border_radius=None,
    border_width=None,
    gen_id=None,
))]
pub fn add_span(
    parent_id: String,
    text: String,
    size: Option<f32>,
    line_height: Option<f32>,
    color: Option<Color>,
    color_alpha: Option<f32>,
    rgba: Option<[f32; 4]>,
    font_id: Option<usize>,
    underline: Option<bool>,
    strikethrough: Option<bool>,
    link: Option<usize>,
    background_color: Option<Color>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    background_gradient_color_stop: Option<Color>,
    background_gradient_color_stop_alpha: Option<f32>,
    background_gradient_rgba_stop: Option<[f32; 4]>,
    background_gradient_degrees: Option<f32>,
    background_gradient_radians: Option<f32>,
    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    gen_id: Option<usize>,
) -> PyResult<usize> 
{
    let id = get_id(gen_id);
    
    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::Span(
        Span {
            id,
            text,
            size,
            line_height,
            color,
            color_alpha,
            rgba,
            font_id,
            padding: None,
            underline,
            strikethrough,
            link,
            background_color,
            background_color_alpha,
            background_rgba,
            background_gradient_color_stop,
            background_gradient_color_stop_alpha,
            background_gradient_rgba_stop,
            background_gradient_degrees,
            background_gradient_radians,
            border_color,
            border_color_alpha,
            border_rgba,
            border_radius,
            border_width,
        }));

    drop(state);
    Ok(id)
}


