//! Svg module - provides add_svg pyfunction
use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::graphics::colors::IpgColor;
use crate::state::{IpgWidgets, get_id, set_state_of_widget};
use crate::widgets::enums::{IpgContentFit, IpgRotation};
use crate::widgets::ipg_mouse_area::IpgMousePointer;
use crate::widgets::ipg_svg::IpgSvg;
use crate::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex}; 
use crate::py_api::helpers::get_length;


/// Add an SVG widget.
///
/// Displays a scalable vector graphic from a file path with
/// optional mouse interaction.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this SVG belongs to.
/// svg_path : str
///     Sets the file path to the SVG image.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the SVG fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the SVG fills available height.
/// ipg_color_filter : IpgColor, Optional
///     Sets the color filter using a predefined color variant.
/// rgba_filter : list of float, Optional
///     Sets the color filter in rgba format as [r, g, b, a].
/// content_fit : IpgContentFit, Optional
///     Sets the content fit strategy for the SVG.
/// rotation_type : IpgRotation, Optional
///     Sets the rotation method for the SVG.
/// rotation_radians : float, Optional
///     Sets the rotation angle in radians.
/// opacity : float, Optional
///     Sets the opacity of the SVG (0.0 to 1.0).
/// mouse_pointer : IpgMousePointer, Optional
///     Sets the mouse pointer style when hovering over the SVG.
/// show : bool, default True
///     Whether the SVG is visible.
/// on_press : callable, Optional
///     Sets the Callback method to invoke when the SVG is pressed.
/// on_release : callable, Optional
///     Sets the Callback method to invoke when the SVG is released.
/// on_right_press : callable, Optional
///     Sets the Callback method to invoke when the SVG is right-pressed.
/// on_right_release : callable, Optional
///     Sets the Callback method to invoke when the SVG is right-released.
/// on_middle_press : callable, Optional
///     Sets the Callback method to invoke when the SVG is middle-pressed.
/// on_middle_release : callable, Optional
///     Sets the Callback method to invoke when the SVG is middle-released.
/// on_enter : callable, Optional
///     Sets the Callback method to invoke when the mouse enters the SVG.
/// on_move : callable, Optional
///     Sets the Callback method to invoke when the mouse moves over the SVG.
/// on_exit : callable, Optional
///     Sets the Callback method to invoke when the mouse exits the SVG.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created SVG.
#[pyfunction]

#[pyo3(signature = (
    parent_id, 
    svg_path, 
    gen_id=None, 
    width=None, 
    width_fill=false, 
    height=None, 
    height_fill=false,
    ipg_color_filter=None,
    rgba_filter=None,
    content_fit=None,
    rotation_type=None,
    rotation_radians=None, 
    opacity=None,
    mouse_pointer=None, 
    show=true,
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
    ))]
pub fn add_svg(
    parent_id: String,
    svg_path: String,
    // above required
    gen_id: Option<usize>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    ipg_color_filter: Option<IpgColor>,
    rgba_filter: Option<[f32; 4]>,
    content_fit: Option<IpgContentFit>,
    rotation_type: Option<IpgRotation>,
    rotation_radians: Option<f32>,
    opacity: Option<f32>,
    mouse_pointer: Option<IpgMousePointer>,
    show: bool,
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
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let color_filter = 
        IpgColor::rgba_ipg_color_to_iced(rgba_filter, ipg_color_filter, 1.0, false);

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

    state.widgets.insert(id, IpgWidgets::IpgSvg(
        IpgSvg {
            id,
            parent_id,
            svg_path,
            width,
            height,
            color_filter,
            content_fit,
            rotation_type,
            rotation_radians,
            opacity,
            mouse_pointer,
            show,
        }));

    drop(state);
    Ok(id)

}