//! TextEditor module - provides add_text_editor pyfunction

use pyo3::{pyfunction, PyResult, Py, PyAny};
type PyObject = Py<PyAny>;


use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex};
use crate::graphics::colors::Color;
use crate::state::{Widgets, get_id, set_state_of_widget};
use crate::widgets::ipg_text_editor::{TextEditorStyle, TextEditor};
use crate::widgets::ipg_text_editor::TxtEdStatus;



#[pyfunction]
#[pyo3(signature = (
    parent_id,
    place_holder=None, 
    font_id=None,
    text_size=None,
    line_height=None,
    width=None,
    width_fill=None,
    height=None,
    height_fill=None,
    fill=None,
    min_height=None,
    max_height=None,
    padding=None,
    wrapping_none=None,
    wrapping_glyph=None,
    wrapping_word_glyph=None,
    on_edit = None,
    user_data = None,
))]
pub fn add_text_editor(
    parent_id: String,
    place_holder: Option<String>, 
    font_id: Option<usize>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    min_height: Option<f32>,
    max_height: Option<f32>,
    padding: Option<Vec<f32>>,
    wrapping_none: Option<bool>,
    wrapping_glyph: Option<bool>,
    wrapping_word_glyph: Option<bool>,
    on_edit: Option<PyObject>,
    user_data: Option<PyObject>,
) ->PyResult<usize> {

    let id = get_id(None);

    // Store callback if provided
    if let Some(py) = on_edit {
        add_callback_to_mutex(id, "on_edit".to_string(), py);
    }

    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    // Register widget with parent
    set_state_of_widget(id, parent_id.clone());

    // Create and store button
    let mut state = access_state();
    
    state.widgets.insert(
        id,
        Widgets::TextEditor(
            TextEditor { 
                id, 
                content: iced::widget::text_editor::Content::new(),
                place_holder, 
                font_id, 
                text_size, 
                line_height, 
                width,
                width_fill, 
                height,
                height_fill,
                fill,
                min_height, 
                max_height, 
                padding, 
                wrapping_none,
                wrapping_glyph,
                wrapping_word_glyph,
                last_status: TxtEdStatus::Disabled
            }),
        );
    drop(state);

    Ok(id)
}


/// The [`Background`] of the text input.
/// The [`Border`] of the text input.
/// The [`Color`] of the placeholder of the text input.
/// The [`Color`] of the value of the text input.
/// The [`Color`] of the selection of the text input.
#[pyfunction]
#[pyo3(signature = (
    background_color = None,
    background_color_alpha = None,
    background_rgba = None,
    background_color_hovered = None,
    background_color_alpha_hovered = None,
    background_rgba_hovered = None,
    background_color_focused = None,
    background_color_alpha_focused = None,
    background_rgba_focused = None,
    background_color_disabled = None,
    background_color_alpha_disabled = None,
    background_rgba_disabled = None,
    border_color = None,
    border_color_alpha = None,
    border_rgba = None,
    border_color_hovered = None,
    border_color_alpha_hovered = None,
    border_rgba_hovered = None,
    border_color_focused = None,
    border_color_alpha_focused = None,
    border_rgba_focused = None,
    border_color_disabled = None,
    border_color_alpha_disabled = None,
    border_rgba_disabled = None,
    border_radius = None,
    border_width = None,
    placeholder_color = None,
    placeholder_color_alpha = None,
    placeholder_rgba = None,
    value_color = None,
    value_color_alpha = None,
    value_rgba = None,
    selection_color = None,
    selection_color_alpha = None,
    selection_rgba = None,
    gen_id = None,
))]
pub fn add_text_editor_style(
    background_color: Option<Color>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    background_color_hovered: Option<Color>,
    background_color_alpha_hovered: Option<f32>,
    background_rgba_hovered: Option<[f32; 4]>,
    background_color_focused: Option<Color>,
    background_color_alpha_focused: Option<f32>,
    background_rgba_focused: Option<[f32; 4]>,
    background_color_disabled: Option<Color>,
    background_color_alpha_disabled: Option<f32>,
    background_rgba_disabled: Option<[f32; 4]>,
    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_color_hovered: Option<Color>,
    border_color_alpha_hovered: Option<f32>,
    border_rgba_hovered: Option<[f32; 4]>,
    border_color_focused: Option<Color>,
    border_color_alpha_focused: Option<f32>,
    border_rgba_focused: Option<[f32; 4]>,
    border_color_disabled: Option<Color>,
    border_color_alpha_disabled: Option<f32>,
    border_rgba_disabled: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    placeholder_color: Option<Color>,
    placeholder_color_alpha: Option<f32>,
    placeholder_rgba: Option<[f32; 4]>,
    value_color: Option<Color>,
    value_color_alpha: Option<f32>,
    value_rgba: Option<[f32; 4]>,
    selection_color: Option<Color>,
    selection_color_alpha: Option<f32>,
    selection_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::TextEditorStyle(
        TextEditorStyle {
            id,
            background_color,
            background_color_alpha,
            background_rgba,
            background_color_hovered,
            background_color_alpha_hovered,
            background_rgba_hovered,
            background_color_focused,
            background_color_alpha_focused,
            background_rgba_focused,
            background_color_disabled,
            background_color_alpha_disabled,
            background_rgba_disabled,
            border_color,
            border_color_alpha,
            border_rgba,
            border_color_hovered,
            border_color_alpha_hovered,
            border_rgba_hovered,
            border_color_focused,
            border_color_alpha_focused,
            border_rgba_focused,
            border_color_disabled,
            border_color_alpha_disabled,
            border_rgba_disabled,
            border_radius,
            border_width,
            placeholder_color,
            placeholder_color_alpha,
            placeholder_rgba,
            value_color,
            value_color_alpha,
            value_rgba,
            selection_color,
            selection_color_alpha,
            selection_rgba,
        }));

    drop(state);
    Ok(id)
}
