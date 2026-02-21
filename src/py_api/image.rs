//! Button module - provides add_button pyfunction

use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;


use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    py_api::helpers::{get_height, get_width}, 
    state::{IpgWidgets, get_id, set_state_of_widget}, 
    widgets::{ipg_image::{IpgImage, IpgImageContentFit, 
        IpgImageFilterMethod, IpgImageRotation}, 
        ipg_mousearea::IpgMousePointer}};


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
    rotation=None,
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
    content_fit: Option<IpgImageContentFit>,
    filter_method: Option<IpgImageFilterMethod>,
    rotation: Option<IpgImageRotation>,
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
    
    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

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
