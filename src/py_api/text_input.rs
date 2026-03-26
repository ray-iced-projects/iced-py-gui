

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::IpgColor, py_api::helpers::get_length, 
    state::{IpgWidgets, get_id, set_state_of_widget}, 
    widgets::{enums::AlignX, ipg_text_input::{IpgTextInput, IpgTextInputStyle}}};

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
/// align_x : AlignX, Optional
///     Sets the horizontal alignment of the input text.
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
    width_fill=false, 
    padding=None, 
    size=None, 
    line_height=None,
    align_x=None, 
    user_data=None,
    is_secure=None,
    font_id=None, 
    style_id=None, 
    show=true,
    ))]
pub fn add_text_input(
        parent_id: String,
        placeholder: String,
        // **above required
        gen_id: Option<usize>,
        on_input: Option<PyObject>,
        on_submit: Option<PyObject>,
        on_paste: Option<PyObject>,
        width: Option<f32>,
        width_fill: bool,
        padding: Option<Vec<f32>>,
        size: Option<f32>,
        line_height: Option<f32>,
        align_x: Option<AlignX>,
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
    
    let width = get_length(width, width_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();
    
    state.widgets.insert(id, IpgWidgets::IpgTextInput(
        IpgTextInput {
            id,
            parent_id,
            placeholder,
            value: String::new(),
            is_secure,
            width,
            padding,
            size,
            line_height,
            align_x,
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
/// Parameters
/// ----------
/// background_color : IpgColor, Optional
///     Sets the background color using a predefined color variant.
/// background_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// background_rgba : list of float, Optional
///     Sets the background color in rgba format as [r, g, b, a].
/// border_color_active : IpgColor, Optional
///     Sets the border color in active state using a predefined color variant.
/// border_color_active_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// border_rgba_active : list of float, Optional
///     Sets the border color in active state in rgba format as [r, g, b, a].
/// border_color_hovered : IpgColor, Optional
///     Sets the border color when hovered using a predefined color variant.
/// border_color_hovered_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// border_rgba_hovered : list of float, Optional
///     Sets the border color when hovered in rgba format as [r, g, b, a].
/// border_color_focused : IpgColor, Optional
///     Sets the border color when focused using a predefined color variant.
/// border_color_focused_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// border_rgba_focused : list of float, Optional
///     Sets the border color when focused in rgba format as [r, g, b, a].
/// border_color_disabled : IpgColor, Optional
///     Sets the border color when disabled using a predefined color variant.
/// border_color_disabled_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// border_rgba_disabled : list of float, Optional
///     Sets the border color when disabled in rgba format as [r, g, b, a].
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// border_radius : float, Optional
///     Sets the border radius in logical pixels.
/// placeholder_color_active : IpgColor, Optional
///     Sets the placeholder text color in active state using a predefined color variant.
/// placeholder_color_active_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// placeholder_rgba_active : list of float, Optional
///     Sets the placeholder text color in active state in rgba format as [r, g, b, a].
/// placeholder_color_disabled : IpgColor, Optional
///     Sets the placeholder text color when disabled using a predefined color variant.
/// placeholder_color_disabled_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// placeholder_rgba_disabled : list of float, Optional
///     Sets the placeholder text color when disabled in rgba format as [r, g, b, a].
/// value_color : IpgColor, Optional
///     Sets the input value text color using a predefined color variant.
/// value_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// value_rgba : list of float, Optional
///     Sets the input value text color in rgba format as [r, g, b, a].
/// selection_color : IpgColor, Optional
///     Sets the text selection highlight color using a predefined color variant.
/// selection_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// selection_rgba : list of float, Optional
///     Sets the text selection highlight color in rgba format as [r, g, b, a].
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a text input's ``style_id``.
#[pyfunction]
#[pyo3(signature = ( 
    background_color=None,
    background_color_alpha=None,
    background_rgba=None,
    border_color_active=None,
    border_color_active_alpha=None,
    border_rgba_active=None,
    border_color_hovered=None,
    border_color_hovered_alpha=None,
    border_rgba_hovered=None,
    border_color_focused=None,
    border_color_focused_alpha=None,
    border_rgba_focused=None,
    border_color_disabled=None,
    border_color_disabled_alpha=None,
    border_rgba_disabled=None,
    border_width=None,
    border_radius=None,
    // icon_color=None,
    // icon_rgba=None,
    placeholder_color_active=None,
    placeholder_color_active_alpha=None,
    placeholder_rgba_active=None,
     placeholder_color_disabled=None,
    placeholder_color_disabled_alpha=None,
    placeholder_rgba_disabled=None,
    value_color=None,
    value_color_alpha=None,
    value_rgba=None,
    selection_color=None,
    selection_color_alpha=None,
    selection_rgba=None,
    gen_id=None))]
pub fn add_text_input_style(
        background_color: Option<IpgColor>,
        background_color_alpha: Option<f32>,
        background_rgba: Option<[f32; 4]>,
        border_color_active: Option<IpgColor>,
        border_color_active_alpha: Option<f32>,
        border_rgba_active: Option<[f32; 4]>,
        border_color_hovered: Option<IpgColor>,
        border_color_hovered_alpha: Option<f32>,
        border_rgba_hovered: Option<[f32; 4]>,
        border_color_focused: Option<IpgColor>,
        border_color_focused_alpha: Option<f32>,
        border_rgba_focused: Option<[f32; 4]>,
        border_color_disabled: Option<IpgColor>,
        border_color_disabled_alpha: Option<f32>,
        border_rgba_disabled: Option<[f32; 4]>,
        border_width: Option<f32>,
        border_radius: Option<f32>,
        // icon_color: Option<IpgColor>,
        // icon_rgba: Option<[f32; 4]>,
        placeholder_color_active: Option<IpgColor>,
        placeholder_color_active_alpha: Option<f32>,
        placeholder_rgba_active: Option<[f32; 4]>,
        placeholder_color_disabled: Option<IpgColor>,
        placeholder_color_disabled_alpha: Option<f32>,
        placeholder_rgba_disabled: Option<[f32; 4]>,
        value_color: Option<IpgColor>,
        value_color_alpha: Option<f32>,
        value_rgba: Option<[f32; 4]>,
        selection_color: Option<IpgColor>,
        selection_color_alpha: Option<f32>,
        selection_rgba: Option<[f32; 4]>,
        gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, background_color_alpha);
    let border_color_active = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_active, border_color_active, border_color_active_alpha);
    let border_color_hovered = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_hovered, border_color_hovered, border_color_hovered_alpha);
    let border_color_focused = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_focused, border_color_focused, border_color_focused_alpha);
    let border_color_disabled = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_disabled, border_color_disabled, border_color_disabled_alpha);
    
    // let icon_color = get_color(icon_rgba, icon_color, 1.0);
    let placeholder_color_active = 
        IpgColor::rgba_ipg_color_to_iced(placeholder_rgba_active, placeholder_color_active, placeholder_color_active_alpha);
    let placeholder_color_disabled = 
        IpgColor::rgba_ipg_color_to_iced(placeholder_rgba_disabled, placeholder_color_disabled, placeholder_color_disabled_alpha);

    let value_color = 
        IpgColor::rgba_ipg_color_to_iced(value_rgba, value_color, value_color_alpha);
    let selection_color = 
        IpgColor::rgba_ipg_color_to_iced(selection_rgba, selection_color, selection_color_alpha);

    let mut state = access_state();
    
    state.widgets.insert(id, IpgWidgets::IpgTextInputStyle(
        IpgTextInputStyle { 
            id,
            background_color,
            border_color_active,
            border_color_hovered,
            border_color_focused,
            border_color_disabled,
            border_width,
            border_radius,
            // icon_color,
            placeholder_color_active,
            placeholder_color_disabled,
            value_color,
            selection_color,
        }));
                
    drop(state);
    Ok(id)

}
