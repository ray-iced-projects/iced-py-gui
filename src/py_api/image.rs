//! Image module - provides add_image pyfunction
use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    py_api::helpers::get_length, 
    state::{IpgWidgets, get_id, set_state_of_widget}, 
    widgets::{enums::{IpgContentFit, IpgColorFilter, IpgRotation}, ipg_image::IpgImage, 
        ipg_mouse_area::IpgMousePointer}};


/// Add an image widget.
///
/// Displays an image from a file path with optional mouse interaction.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this image belongs to.
/// image_path : str
///     Sets the file path to the image.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the image fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the image fills available height.
/// padding : list of float, Optional
///     Sets the Padding as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// content_fit : IpgContentFit, Optional
///     Sets the content fit strategy for the image.
/// filter_method : IpgColorFilter, Optional
///     Sets the color filter method applied to the image.
/// rotation_method : IpgRotation, Optional
///     Sets the rotation method for the image.
/// rotation_radians : float, Optional
///     Sets the rotation angle in radians.
/// opacity : float, Optional
///     Sets the opacity of the image (0.0 to 1.0).
/// mouse_pointer : IpgMousePointer, Optional
///     Sets the mouse pointer style when hovering over the image.
/// on_press : callable, Optional
///     Sets the Callback method to invoke when the image is pressed.
/// on_release : callable, Optional
///     Sets the Callback method to invoke when the image is released.
/// on_right_press : callable, Optional
///     Sets the Callback method to invoke when the image is right-pressed.
/// on_right_release : callable, Optional
///     Sets the Callback method to invoke when the image is right-released.
/// on_middle_press : callable, Optional
///     Sets the Callback method to invoke when the image is middle-pressed.
/// on_middle_release : callable, Optional
///     Sets the Callback method to invoke when the image is middle-released.
/// on_enter : callable, Optional
///     Sets the Callback method to invoke when the mouse enters the image.
/// on_move : callable, Optional
///     Sets the Callback method to invoke when the mouse moves over the image.
/// on_exit : callable, Optional
///     Sets the Callback method to invoke when the mouse exits the image.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the image is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created image.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    image_path, 
    gen_id=None, 
    width=None, 
    width_fill=false, 
    height=None, 
    height_fill=false, 
    padding=None, 
    content_fit=None, 
    filter_method=None,
    rotation_method=None,
    rotation_radians=None, 
    opacity=None,
    mouse_pointer=None,
    on_press=None, 
    on_release=None,
    on_right_press=None, 
    on_right_release=None,
    on_middle_press=None, 
    on_middle_release=None,
    on_enter=None, 
    on_move=None, 
    on_exit=None,
    user_data=None,
    show=true,
    ))]
pub fn add_image(
    parent_id: String,
    image_path: String,
    // above required
    gen_id: Option<usize>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    padding: Option<Vec<f32>>,
    content_fit: Option<IpgContentFit>,
    filter_method: Option<IpgColorFilter>,
    rotation_method: Option<IpgRotation>,
    rotation_radians: Option<f32>,
    opacity: Option<f32>,
    mouse_pointer: Option<IpgMousePointer>,
    on_press: Option<PyObject>,
    on_release: Option<PyObject>,
    on_right_press: Option<PyObject>,
    on_right_release: Option<PyObject>,
    on_middle_press: Option<PyObject>,
    on_middle_release: Option<PyObject>,
    on_enter: Option<PyObject>,
    on_move: Option<PyObject>,
    on_exit: Option<PyObject>,
    user_data: Option<PyObject>,
    show: bool,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

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

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgImage(
        IpgImage {
            id,
            parent_id,
            image_path,
            width,
            height,
            padding,
            content_fit,
            filter_method,
            rotation_method,
            rotation_radians,
            opacity,
            mouse_pointer,
            show,
        }));

    drop(state);
    Ok(id)

}
