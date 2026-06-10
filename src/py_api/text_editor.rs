//! TextEditor module - provides add_text_editor pyfunction or TextEditor class

use pyo3::{pyfunction, PyResult, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex};
use crate::graphics::colors::Color;
use crate::state::{Widgets, get_id, set_state_of_widget};
use crate::widgets::ipg_text_editor::{TextEditorStyle, TextEditor};
use crate::widgets::ipg_text_editor::TxtEdStatus;


/// Add a text editor widget.
///
/// A multi-line text editing area that supports scrolling, placeholder text,
/// and optional edit callbacks.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this text editor belongs to.
/// content : str, Optional
///     Sets the initial text content displayed in the editor.
/// place_holder : str, Optional
///     Sets the placeholder text shown when the editor is empty.
/// font_id : int, Optional
///     Sets the Font ID for the editor text.
/// text_size : float, Optional
///     Sets the font size in logical pixels.
/// line_height : float, Optional
///     Sets the line height as a multiplier of the font size.
/// width : float, Optional
///     Sets a fixed width in logical pixels.
/// width_fill : bool, Optional
///     Whether the editor expands to fill the available width.
/// height : float, Optional
///     Sets a fixed height in logical pixels.
/// height_fill : bool, Optional
///     Whether the editor expands to fill the available height.
/// fill : bool, Optional
///     Whether the editor fills both width and height.
/// min_height : float, Optional
///     Sets the minimum height in logical pixels.
/// max_height : float, Optional
///     Sets the maximum height in logical pixels.
/// padding : list of float, Optional
///     Sets the padding as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// wrapping_none : bool, Optional
///     Disables text wrapping; text extends past the right edge.
/// wrapping_glyph : bool, Optional
///     Wraps at the glyph boundary (default).
/// wrapping_word_glyph : bool, Optional
///     Wraps at word boundaries, falling back to glyph boundaries.
/// on_edit : callable, Optional
///     Callback invoked on every edit action.
///     Signature: ``def cb(wid: int, content: str)``
/// user_data : Any, Optional
///     Arbitrary data forwarded to callbacks.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created text editor.



#[pyfunction]
#[pyo3(signature = (
    parent_id,
    content=None,
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
    content: Option<String>,
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

    let content = if let Some(text) = content {
        iced::widget::text_editor::Content::with_text(&text)
    } else { iced::widget::text_editor::Content::new() };
    
    // Register widget with parent
    set_state_of_widget(id, parent_id.clone());

    // Create and store button
    let mut state = access_state();
    
    state.widgets.insert(
        id,
        Widgets::TextEditor(
            TextEditor { 
                id, 
                content,
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



/// Add styling to a text editor.
///
/// Creates a custom style that can be applied to a text editor
/// via its ``style_id`` parameter.
///
/// Each colour parameter accepts three forms that are applied in priority order:
/// an rgba list ``[r, g, b, a]``, a named ``Color`` enum value, or an
/// ``_alpha`` modifier on the named colour.
///
/// Background and border colours can be set per-status (Active, Hovered,
/// Focused, Disabled) using the ``_<status>`` suffix.  When only the
/// base name is used it applies to all statuses.
///
/// Parameters
/// ----------
/// background_color : Color, Optional
///     Sets the background colour for all statuses using a predefined Color variant.
/// background_color_alpha : float, Optional
///     Sets the alpha of ``background_color``.
/// background_rgba : list of float, Optional
///     Sets the background colour for all statuses as ``[r, g, b, a]``.
/// background_color_hovered : Color, Optional
///     Overrides the background colour in the Hovered status.
/// background_color_alpha_hovered : float, Optional
///     Sets the alpha of ``background_color_hovered``.
/// background_rgba_hovered : list of float, Optional
///     Overrides the background colour in the Hovered status as ``[r, g, b, a]``.
/// background_color_focused : Color, Optional
///     Overrides the background colour in the Focused status.
/// background_color_alpha_focused : float, Optional
///     Sets the alpha of ``background_color_focused``.
/// background_rgba_focused : list of float, Optional
///     Overrides the background colour in the Focused status as ``[r, g, b, a]``.
/// background_color_disabled : Color, Optional
///     Overrides the background colour in the Disabled status.
/// background_color_alpha_disabled : float, Optional
///     Sets the alpha of ``background_color_disabled``.
/// background_rgba_disabled : list of float, Optional
///     Overrides the background colour in the Disabled status as ``[r, g, b, a]``.
/// border_color : Color, Optional
///     Sets the border colour for all statuses using a predefined Color variant.
/// border_color_alpha : float, Optional
///     Sets the alpha of ``border_color``.
/// border_rgba : list of float, Optional
///     Sets the border colour for all statuses as ``[r, g, b, a]``.
/// border_color_hovered : Color, Optional
///     Overrides the border colour in the Hovered status.
/// border_color_alpha_hovered : float, Optional
///     Sets the alpha of ``border_color_hovered``.
/// border_rgba_hovered : list of float, Optional
///     Overrides the border colour in the Hovered status as ``[r, g, b, a]``.
/// border_color_focused : Color, Optional
///     Overrides the border colour in the Focused status.
/// border_color_alpha_focused : float, Optional
///     Sets the alpha of ``border_color_focused``.
/// border_rgba_focused : list of float, Optional
///     Overrides the border colour in the Focused status as ``[r, g, b, a]``.
/// border_color_disabled : Color, Optional
///     Overrides the border colour in the Disabled status.
/// border_color_alpha_disabled : float, Optional
///     Sets the alpha of ``border_color_disabled``.
/// border_rgba_disabled : list of float, Optional
///     Overrides the border colour in the Disabled status as ``[r, g, b, a]``.
/// border_radius : list of float, Optional
///     Sets the corner radii as [all], [top-left, top-right, bottom-right, bottom-left].
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// placeholder_color : Color, Optional
///     Sets the placeholder text colour using a predefined Color variant.
/// placeholder_color_alpha : float, Optional
///     Sets the alpha of ``placeholder_color``.
/// placeholder_rgba : list of float, Optional
///     Sets the placeholder text colour as ``[r, g, b, a]``.
/// value_color : Color, Optional
///     Sets the editor text (value) colour using a predefined Color variant.
/// value_color_alpha : float, Optional
///     Sets the alpha of ``value_color``.
/// value_rgba : list of float, Optional
///     Sets the editor text colour as ``[r, g, b, a]``.
/// selection_color : Color, Optional
///     Sets the text selection highlight colour using a predefined Color variant.
/// selection_color_alpha : float, Optional
///     Sets the alpha of ``selection_color``.
/// selection_rgba : list of float, Optional
///     Sets the text selection highlight colour as ``[r, g, b, a]``.
/// gen_id : int, Optional
///     Obtains an ID of a widget that has not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a text editor's ``style_id``.
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
