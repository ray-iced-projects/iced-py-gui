//! Opague module - provides add_opaque_container pyfunction

use iced::Color;

use pyo3::{Py, PyAny, PyResult, pyfunction};
type PyObject = Py<PyAny>;

use crate::widgets::ipg_opaque::{IpgOpaque, IpgOpaqueStyle};
use crate::{add_callback_to_mutex, add_user_data_to_mutex};
use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::get_length;
use crate::state::{IpgContainers, IpgWidgets, access_state, 
    get_id, set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::enums::{AlignX, 
    AlignY};

/// Add an opaque container widget.
///
/// An opaque container blocks mouse events from passing through
/// to widgets underneath it, useful for overlay scenarios.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this opaque container belongs to.
/// container_id : str
///     Sets the Unique string identifier for the opaque container.
/// parent_id : str, Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the container fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the container fills available height.
/// fill : bool, Optional
///     Whether to fill both the available width and height
/// center : bool, Optional
///     Whether to center the child within the container.
/// align_x : AlignX, Optional
///     Sets the horizontal alignment of the child.
/// align_y : AlignY, Optional
///     Sets the vertical alignment of the child.
/// mouse_on_press : callable, Optional
///     Sets the Callback method to invoke when the mouse is pressed in the area.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the opaque container is visible.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_opaque_style``.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created opaque container.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    width=None, 
    width_fill=false,
    height=None, 
    height_fill=false,
    fill=None,
    center=None,
    align_x=None, 
    align_y=None,
    mouse_on_press=None,
    user_data=None,
    show=true, 
    style_id=None,
    gen_id=None,
    ))]
pub fn add_opaque_container(
    window_id: String,
    container_id: String,
    // required above
    parent_id: Option<String>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    fill: Option<bool>,
    center: Option<bool>,
    align_x: Option<AlignX>,
    align_y: Option<AlignY>,
    mouse_on_press: Option<PyObject>,
    user_data: Option<PyObject>,
    show: bool,
    style_id: Option<usize>,
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let (width, height) = if fill == Some(true) {
        (get_length(None, true), get_length(None, true))
    } else {
        (get_length(width, width_fill), get_length(height, height_fill))
    };

    let include_mouse_area = if let Some(py) = mouse_on_press {
        add_callback_to_mutex(id, "on_press".to_string(), py);
        true
    } else { false };

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let prt_id = if let Some(id) = parent_id {
        id
    } else { window_id.clone() };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_opaque".to_string());

    state.containers.insert(id, IpgContainers::IpgOpaque(
        IpgOpaque {
            id,  
            width, 
            height,
            center,
            align_x,
            align_y,
            include_mouse_area,
            show,
            style_id
        }));

    drop(state);         
    Ok(id)
}

/// Add styling to an opaque container.
///
/// Creates a custom style that can be applied to an opaque
/// container via its ``style_id`` parameter.
///
/// Parameters
/// ----------
/// background_color : IpgColor, Optional
///     Sets the background color using a predefined color variant.
/// background_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// background_rgba : list of float, Optional
///     Sets the background color in rgba format as [r, g, b, a].
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to an opaque container's ``style_id``.
#[pyfunction]
#[pyo3(signature = ( 
    background_color=None,
    background_color_alpha=None, 
    background_rgba=None,
    gen_id=None
    ))]
pub fn add_opaque_style(
    background_color: Option<IpgColor>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    let background_color: Option<Color> = 
    IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, background_color_alpha);

    state.widgets.insert(id, IpgWidgets::IpgOpaqueStyle(
        IpgOpaqueStyle {
            id,
            background_color,
    }));

    drop(state);
    Ok(id)
}
