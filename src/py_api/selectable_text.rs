//!SelectableText - Provides add_selectable_text to the pyfunction
use pyo3::{Py, PyAny, PyResult, pyfunction};

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::IpgColor, py_api::helpers::get_length, 
    state::{IpgWidgets, get_id, set_state_of_widget}, widgets::{enums::{AlignX, AlignY}, 
    ipg_selectable_text::IpgSelectableText, ipg_text::{TextShaping, TextWrapping}}};
type PyObject = Py<PyAny>;

/// Add a selectable text widget.
///
/// A text widget that supports mouse interaction and text selection.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this selectable text belongs to.
/// text : str
///     Sets the text content to display.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// on_press : callable, Optional
///     Sets the Callback method to invoke when the text is pressed.
/// on_release : callable, Optional
///     Sets the Callback method to invoke when the text is released.
/// on_right_press : callable, Optional
///     Sets the Callback method to invoke when the text is right-pressed.
/// on_right_release : callable, Optional
///     Sets the Callback method to invoke when the text is right-released.
/// on_middle_press : callable, Optional
///     Sets the Callback method to invoke when the text is middle-pressed.
/// on_middle_release : callable, Optional
///     Sets the Callback method to invoke when the text is middle-released.
/// on_move : callable, Optional
///     Sets the Callback method to invoke when the mouse moves over the text.
/// on_enter : callable, Optional
///     Sets the Callback method to invoke when the mouse enters the text.
/// on_exit : callable, Optional
///     Sets the Callback method to invoke when the mouse exits the text.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the text fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the text fills available height.
/// center : bool, Optional
///     Whether to center the text.
/// align_x : AlignX, Optional
///     Sets the horizontal alignment of the text.
/// align_y : AlignY, Optional
///     Sets the vertical alignment of the text.
/// line_height : float, Optional
///     Sets the line height for the text.
/// size : float, Optional
///     Sets the font size for the text.
/// text_color : IpgColor, Optional
///     Sets the text color using a predefined color variant.
/// text_rgba : list of float, Optional
///     Sets the text color in rgba format as [r, g, b, a].
/// show : bool, default True
///     Whether the selectable text is visible.
/// font_id : int, Optional
///     Sets the Font ID for the text.
/// shaping : TextShaping, Optional
///     Sets the Text shaping strategy.
/// wrapping : TextWrapping, Optional
///     Sets the Text wrapping strategy.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created selectable text.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    text, 
    gen_id=None, 
    on_press=None, 
    on_release=None, 
    on_right_press=None, 
    on_right_release=None, 
    on_middle_press=None, 
    on_middle_release=None, 
    on_move=None, 
    on_enter=None, 
    on_exit=None, 
    width=None, 
    width_fill=false,
    height=None, 
    height_fill=false,
    center=None,
    align_x=None, 
    align_y=None,
    line_height=None, 
    size=None,
    text_color=None,
    text_color_alpha=None, 
    text_rgba=None, 
    show=true,
    font_id=None, 
    shaping=None,
    wrapping=None, 
    user_data=None,
    ))]
pub fn add_selectable_text(
    parent_id: String,
    text: String,
    // ** above required
    gen_id: Option<usize>,
    on_press: Option<PyObject>,
    on_release: Option<PyObject>,
    on_right_press: Option<PyObject>,
    on_right_release: Option<PyObject>,
    on_middle_press: Option<PyObject>,
    on_middle_release: Option<PyObject>,
    on_move: Option<PyObject>,
    on_enter: Option<PyObject>,
    on_exit: Option<PyObject>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    center: Option<bool>,
    align_x: Option<AlignX>,
    align_y: Option<AlignY>,
    line_height: Option<f32>,
    size: Option<f32>,
    text_color: Option<IpgColor>,
    text_color_alpha: Option<f32>,
    text_rgba: Option<[f32; 4]>,
    show: bool,
    font_id: Option<usize>,
    shaping: Option<TextShaping>,
    wrapping: Option<TextWrapping>,
    user_data: Option<PyObject>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let content = text.clone();

    if let Some(py) = on_press {
    add_callback_to_mutex(id, "on_press".to_string(), py);
    }
    
    if let Some(py) = on_release {
        add_callback_to_mutex(id, "event_name".to_string(), py);
    }
    
    if let Some(py) = on_right_press {
        add_callback_to_mutex(id, "on_right_press".to_string(), py);
    }
    
    if let Some(py) = on_right_release {
        add_callback_to_mutex(id, "on_right_release".to_string(), py);
    }
    
    if let Some(py) = on_middle_press {
        add_callback_to_mutex(id, "on_middle_press".to_string(), py);
    }
    
    if let Some(py) = on_middle_release {
        add_callback_to_mutex(id, "on_middle_release".to_string(), py);
    }
    
    if let Some(py) = on_enter {
        add_callback_to_mutex(id, "on_enter".to_string(), py);
    }
    
    if let Some(py) = on_move {
        add_callback_to_mutex(id, "on_move".to_string(), py);
    }
    
    if let Some(py) = on_exit {
        add_callback_to_mutex(id, "on_exit".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    let width = get_length(width, width_fill);
    let height = get_length(height, height_fill);

    let text_color = IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, text_color_alpha);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();
    
    state.widgets.insert(id, IpgWidgets::IpgSelectableText(
        IpgSelectableText {
            id,
            parent_id,
            content,
            width,
            height,
            center,
            align_x,
            align_y,
            line_height,
            size,
            show,
            font_id,
            shaping,
            wrapping,
            text_color,
        }));

    drop(state);
    Ok(id)

}