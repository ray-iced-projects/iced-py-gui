//! Opague module - provides add_opaque_container pyfunction

use iced::Color;

use pyo3::{Py, PyAny, PyResult, pyfunction};
type PyObject = Py<PyAny>;

use crate::widgets::ipg_opaque::{IpgOpaque, IpgOpaqueStyle};
use crate::{add_callback_to_mutex, add_user_data_to_mutex};
use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::{get_height, get_width};
use crate::state::{IpgContainers, IpgWidgets, access_state, get_id, set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::enums::{AlignX, 
    AlignY};

/// Add a container widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    width=None, 
    width_fill=false,
    height=None, 
    height_fill=false,
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

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

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

#[pyfunction]
#[pyo3(signature = ( 
    background_color=None, 
    background_rgba=None,
    gen_id=None
    ))]
pub fn add_opaque_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    let background_color: Option<Color> = 
    IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);

    state.widgets.insert(id, IpgWidgets::IpgOpaqueStyle(
        IpgOpaqueStyle {
            id,
            background_color,
    }));

    drop(state);
    Ok(id)
}
