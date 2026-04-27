//! Text inputs display fields that can be filled with text.
use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::Color, 
    state::{Widgets, get_id, set_state_of_widget}, 
    widgets::ipg_text_input::{TextInput, TextInputStyle}};

use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;


/// Add a text input widget.
///
/// A single-line text input field with placeholder text.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this text input belongs to.
/// placeholder : str
///     Sets the placeholder text shown when the input is empty.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// on_input : callable, Optional
///     Sets the Callback method to invoke when the input text changes.
/// on_submit : callable, Optional
///     Sets the Callback method to invoke when the user presses enter.
/// on_paste : callable, Optional
///     Sets the Callback method to invoke when text is pasted.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the text input fills available width.
/// padding : list of float, Optional
///     Sets the Padding as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// size : float, Optional
///     Sets the font size for the input text.
/// line_height : float, Optional
///     Sets the line height for the input text.
/// align_left : bool, Optional
///     Whether to set the horizontal alignment left (default).
/// align_center : bool, Optional
///     Whether to set the horizontal alignment center.
/// align_right : bool, Optional
///     Whether to set the horizontal alignment right.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// is_secure : bool, Optional
///     Whether the input text is obscured (password mode).
/// font_id : int, Optional
///     Sets the Font ID for the input text.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_text_input_style``.
/// show : bool, default True
///     Whether the text input is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created text input.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    placeholder, 
    gen_id=None,
    on_input=None, 
    on_submit=None, 
    on_paste=None, 
    width=None, 
    width_fill=None, 
    padding=None, 
    size=None, 
    line_height=None,
    align_left=None, 
    align_center=None,
    align_right=None,
    user_data=None,
    is_secure=None,
    font_id=None, 
    style_id=None, 
    show=true,
    ))]
pub fn add_text_input(
        parent_id: String,
        placeholder: String,
        gen_id: Option<usize>,
        on_input: Option<PyObject>,
        on_submit: Option<PyObject>,
        on_paste: Option<PyObject>,
        width: Option<f32>,
        width_fill: Option<bool>,
        padding: Option<Vec<f32>>,
        size: Option<f32>,
        line_height: Option<f32>,
        align_left: Option<bool>,
        align_center: Option<bool>,
        align_right: Option<bool>,
        user_data: Option<PyObject>,
        is_secure: Option<bool>,
        font_id: Option<usize>,
        style_id: Option<usize>,
        show: bool,
    ) -> PyResult<usize> 
{

    let id = get_id(gen_id);

    if let Some(py) = on_input {
        add_callback_to_mutex(id, "on_input".to_string(), py);
    }
    if let Some(py) = on_submit {
        add_callback_to_mutex(id, "on_submit".to_string(), py);
    }

    if let Some(py) = on_paste {
        add_callback_to_mutex(id, "on_paste".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();
    
    state.widgets.insert(id, Widgets::TextInput(
        TextInput {
            id,
            parent_id,
            placeholder,
            value: String::new(),
            is_secure,
            width,
            width_fill,
            padding,
            size,
            line_height,
            align_left,
            align_center,
            align_right,
            font_id,
            style_id,
            show,
        }));

    drop(state);
    Ok(id)

}

/// Add styling to a text input.
///
/// Creates a custom style that can be applied to a text input
/// via its ``style_id`` parameter.
/// 
/// The style are keyed to a background, text, primary, and secondary colors
/// Each color generates a weak, strong, etc. type
/// 
/// if you want to produce your own colors from a new background,
/// then you will need to define the new background, text, primary, 
/// and secondary colors.  Based on these, the color types will be 
/// generated for you based on the widget status.
/// 
/// You also have the ability to define all the colors individually or
/// define an active color which replaces all the colors for that parameter.
/// 
/// Active: background: background.base.color
///         border: background.strong.color,
///         icon: background.weak.text,
///         placeholder: secondary.base.color,
///         value: background.base.text,
///         selection: primary.weak.color,
/// 
/// Hovered: same as Active except
///          border: background.base.text
/// 
/// Focused: same as primary except
///          border: primary.strong.color
/// 
/// Disabled: same as primary except
///           background: background.weak.color
///           placeholder: background.strongest.color
///
/// Parameters
/// ----------
/// Colors for auto generate type background, text, primary, and secondary.
/// These color must all be defined to generate the status states.
/// background/_color/_rgba : Color/[float, 4], Optional
///     Sets the background color using a predefined color variant or as [r, g, b, a].
/// background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// text/_color/_rgba : Color/[float, 4], Optional
///     Sets the text color using a predefined color variant or as [r, g, b, a].
/// text_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// primary/_color/_rgba : Color/[float, 4], Optional
///     Sets the primary color using a predefined color variant or as [r, g, b, a].
/// primary_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// secondary/_color/_rgba : Color/[float, 4], Optional
///     Sets the secondary color using a predefined color variant or as [r, g, b, a].
/// secondary_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// border_color_<status> : Color, Optional
///     Status: active, hovered, focused, disabled
///     Sets the border color in status state using a predefined color variant.
/// border_color_<status>_alpha : float, Optional
///     Sets the alpha of the Color.
/// border_rgba_<status>: list of float, Optional
///     Sets the border color in <status> state in rgba format as [r, g, b, a].
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// border_radius : float, Optional
///     Sets the border radius in logical pixels.
/// icon_color_<status> : Color, Optional
///     Status: active, hovered, focused, disabled
///     Sets the icon text color in <status> state using a predefined color variant.
/// icon_color_<status>_alpha : float, Optional
///     Sets the alpha of the Color.
/// icon_rgba_<status> : list of float, Optional
///     Sets the icon text color in <status> state in rgba format as [r, g, b, a].
/// placeholder_color_<status> : Color, Optional
///     Status: active, hovered, focused, disabled
///     Sets the placeholder text color in <status> state using a predefined color variant.
/// placeholder_color_<status>_alpha : float, Optional
///     Sets the alpha of the Color.
/// placeholder_rgba_<status> : list of float, Optional
///     Sets the placeholder text color in <status> state in rgba format as [r, g, b, a].
/// value_color_<status> : Color, Optional
///     Status: active, hovered, focused, disabled
///     Sets the input value text color in <status> state using a predefined color variant.
/// value_color<status>_alpha : float, Optional
///     Sets the alpha of the Color.
/// value_rgba_<status> : list of float, Optional
///     Sets the input value text color in <status> state in rgba format as [r, g, b, a].
/// selection_color_<status> : Color, Optional
///     Status: active, hovered, focused, disabled
///     Sets the text selection highlight color <status> state using a predefined color variant.
/// selection_color_active_alpha : float, Optional
///     Sets the alpha of the Color.
/// selection_rgba_active : list of float, Optional
///     Sets the text selection highlight color in <status> state in rgba format as [r, g, b, a].
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a text input's ``style_id``.
#[pyfunction]
#[pyo3(signature = ( 
    background_color = None,
    background_color_alpha = None,
    background_rgba = None,

    text_color = None,
    text_color_alpha = None,
    text_rgba = None,

    primary_color = None,
    primary_color_alpha = None,
    primary_rgba = None,

    secondary_color = None,
    secondary_color_alpha = None,
    secondary_rgba = None,

    border_color_active = None,
    border_color_alpha_active = None,
    border_rgba_active = None,

    border_color_hovered = None,
    border_color_alpha_hovered = None,
    border_rgba_hovered = None,

    border_color_focused = None,
    border_color_alpha_focused = None,
    border_rgba_focused = None,

    border_color_disabled = None,
    border_color_alpha_disabled = None,
    border_rgba_disabled = None,

    border_width = None,
    border_radius = None,

    // overrides all other icon colors
    // if not defined
    icon_color_active = None,
    icon_color_alpha_active = None,
    icon_rgba_active = None,

    icon_color_hovered = None,
    icon_color_alpha_hovered = None,
    icon_rgba_hovered = None,

    icon_color_focused = None,
    icon_color_alpha_focused = None,
    icon_rgba_focused = None,

    icon_color_disabled = None,
    icon_color_alpha_disabled = None,
    icon_rgba_disabled = None,

    // overrides all other icon colors
    // if not defined
    placeholder_color_active = None,
    placeholder_color_alpha_active = None,
    placeholder_rgba_active = None,

    placeholder_color_hovered = None,
    placeholder_color_alpha_hovered = None,
    placeholder_rgba_hovered = None,

    placeholder_color_focused = None,
    placeholder_color_alpha_focused = None,
    placeholder_rgba_focused = None,

    placeholder_color_disabled = None,
    placeholder_color_alpha_disabled = None,
    placeholder_rgba_disabled = None,

    // overrides all other icon colors
    // if not defined
    value_color_active = None,
    value_color_alpha_active = None,
    value_rgba_active = None,

    value_color_hovered = None,
    value_color_alpha_hovered = None,
    value_rgba_hovered = None,

    value_color_focused = None,
    value_color_alpha_focused = None,
    value_rgba_focused = None,

    value_color_disabled = None,
    value_color_alpha_disabled = None,
    value_rgba_disabled = None,

    // overrides all other icon colors
    // if not defined
    selection_color_active = None,
    selection_color_alpha_active = None,
    selection_rgba_active = None,

    selection_color_hovered = None,
    selection_color_alpha_hovered = None,
    selection_rgba_hovered = None,

    selection_color_focused = None,
    selection_color_alpha_focused = None,
    selection_rgba_focused = None,

    selection_color_disabled = None,
    selection_color_alpha_disabled = None,
    selection_rgba_disabled = None,

    gen_id=None
))]
pub fn add_text_input_style(
        background_color: Option<Color>,
        background_color_alpha: Option<f32>,
        background_rgba: Option<[f32; 4]>,

        text_color: Option<Color>,
        text_color_alpha: Option<f32>,
        text_rgba: Option<[f32; 4]>,

        primary_color: Option<Color>,
        primary_color_alpha: Option<f32>,
        primary_rgba: Option<[f32; 4]>,

        secondary_color: Option<Color>,
        secondary_color_alpha: Option<f32>,
        secondary_rgba: Option<[f32; 4]>,

        border_color_active: Option<Color>,
        border_color_alpha_active: Option<f32>,
        border_rgba_active: Option<[f32; 4]>,

        border_color_hovered: Option<Color>,
        border_color_alpha_hovered: Option<f32>,
        border_rgba_hovered: Option<[f32; 4]>,

        border_color_focused: Option<Color>,
        border_color_alpha_focused: Option<f32>,
        border_rgba_focused: Option<[f32; 4]>,

        border_color_disabled: Option<Color>,
        border_color_alpha_disabled: Option<f32>,
        border_rgba_disabled: Option<[f32; 4]>,

        border_width: Option<f32>,
        border_radius: Option<f32>,

        // overrides all other icon colors
        // if not defined
        icon_color_active: Option<Color>,
        icon_color_alpha_active: Option<f32>,
        icon_rgba_active: Option<[f32; 4]>,

        icon_color_hovered: Option<Color>,
        icon_color_alpha_hovered: Option<f32>,
        icon_rgba_hovered: Option<[f32; 4]>,

        icon_color_focused: Option<Color>,
        icon_color_alpha_focused: Option<f32>,
        icon_rgba_focused: Option<[f32; 4]>,

        icon_color_disabled: Option<Color>,
        icon_color_alpha_disabled: Option<f32>,
        icon_rgba_disabled: Option<[f32; 4]>,

        // overrides all other icon colors
        // if not defined
        placeholder_color_active: Option<Color>,
        placeholder_color_alpha_active: Option<f32>,
        placeholder_rgba_active: Option<[f32; 4]>,

        placeholder_color_hovered: Option<Color>,
        placeholder_color_alpha_hovered: Option<f32>,
        placeholder_rgba_hovered: Option<[f32; 4]>,

        placeholder_color_focused: Option<Color>,
        placeholder_color_alpha_focused: Option<f32>,
        placeholder_rgba_focused: Option<[f32; 4]>,

        placeholder_color_disabled: Option<Color>,
        placeholder_color_alpha_disabled: Option<f32>,
        placeholder_rgba_disabled: Option<[f32; 4]>,

        // overrides all other icon colors
        // if not defined
        value_color_active: Option<Color>,
        value_color_alpha_active: Option<f32>,
        value_rgba_active: Option<[f32; 4]>,

        value_color_hovered: Option<Color>,
        value_color_alpha_hovered: Option<f32>,
        value_rgba_hovered: Option<[f32; 4]>,

        value_color_focused: Option<Color>,
        value_color_alpha_focused: Option<f32>,
        value_rgba_focused: Option<[f32; 4]>,

        value_color_disabled: Option<Color>,
        value_color_alpha_disabled: Option<f32>,
        value_rgba_disabled: Option<[f32; 4]>,

        // overrides all other icon colors
        // if not defined
        selection_color_active: Option<Color>,
        selection_color_alpha_active: Option<f32>,
        selection_rgba_active: Option<[f32; 4]>,

        selection_color_hovered: Option<Color>,
        selection_color_alpha_hovered: Option<f32>,
        selection_rgba_hovered: Option<[f32; 4]>,

        selection_color_focused: Option<Color>,
        selection_color_alpha_focused: Option<f32>,
        selection_rgba_focused: Option<[f32; 4]>,

        selection_color_disabled: Option<Color>,
        selection_color_alpha_disabled: Option<f32>,
        selection_rgba_disabled: Option<[f32; 4]>,
            
        gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();
    
    state.widgets.insert(id, Widgets::TextInputStyle(
        TextInputStyle { 
            id,
            background_color,
            background_color_alpha,
            background_rgba,

            text_color,
            text_color_alpha,
            text_rgba,

            primary_color,
            primary_color_alpha,
            primary_rgba,

            secondary_color,
            secondary_color_alpha,
            secondary_rgba,

            border_color_active,
            border_color_alpha_active,
            border_rgba_active,

            border_color_hovered,
            border_color_alpha_hovered,
            border_rgba_hovered,

            border_color_focused,
            border_color_alpha_focused,
            border_rgba_focused,

            border_color_disabled,
            border_color_alpha_disabled,
            border_rgba_disabled,

            border_width,
            border_radius,

            icon_color_active,
            icon_color_alpha_active,
            icon_rgba_active,

            icon_color_hovered,
            icon_color_alpha_hovered,
            icon_rgba_hovered,

            icon_color_focused,
            icon_color_alpha_focused,
            icon_rgba_focused,

            icon_color_disabled,
            icon_color_alpha_disabled,
            icon_rgba_disabled,

            placeholder_color_active,
            placeholder_color_alpha_active,
            placeholder_rgba_active,

            placeholder_color_hovered,
            placeholder_color_alpha_hovered,
            placeholder_rgba_hovered,

            placeholder_color_focused,
            placeholder_color_alpha_focused,
            placeholder_rgba_focused,

            placeholder_color_disabled,
            placeholder_color_alpha_disabled,
            placeholder_rgba_disabled,

            value_color_active,
            value_color_alpha_active,
            value_rgba_active,

            value_color_hovered,
            value_color_alpha_hovered,
            value_rgba_hovered,

            value_color_focused,
            value_color_alpha_focused,
            value_rgba_focused,

            value_color_disabled,
            value_color_alpha_disabled,
            value_rgba_disabled,

            selection_color_active,
            selection_color_alpha_active,
            selection_rgba_active,

            selection_color_hovered,
            selection_color_alpha_hovered,
            selection_rgba_hovered,

            selection_color_focused,
            selection_color_alpha_focused,
            selection_rgba_focused,

            selection_color_disabled,
            selection_color_alpha_disabled,
            selection_rgba_disabled,
        }));
                
    drop(state);
    Ok(id)

}
