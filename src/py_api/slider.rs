//! Provides Slider py function add_slidder
use pyo3::{pyfunction, Py, PyAny, PyResult};

use crate::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex, graphics::colors::IpgColor, 
    py_api::helpers::get_length, state::{IpgWidgets, 
        get_id, set_state_of_widget}, 
        widgets::ipg_slider::{IpgSlider, IpgSliderStyle}};
type PyObject = Py<PyAny>;



#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    min, 
    max, 
    step, 
    value,
    shift_step=None, 
    gen_id=None, 
    width=None, 
    height=None, 
    width_fill=false, 
    on_change=None, 
    on_release=None, 
    style_id=None,
    user_data=None,
    show=true, 
    ))]
pub fn add_slider(
    parent_id: String,
    min: f32,
    max: f32,
    step: f32,
    value: f32,
    shift_step: Option<f32>,
    gen_id: Option<usize>,
    width: Option<f32>,
    height: Option<f32>,
    width_fill: bool,
    on_change: Option<PyObject>,
    on_release: Option<PyObject>,
    style_id: Option<usize>,
    user_data: Option<PyObject>,
    show: bool,
    ) -> PyResult<usize> 
    {

    let id = get_id(gen_id);

    if let Some(py) = on_change {
        add_callback_to_mutex(id, "on_change".to_string(), py);
    }
    if let Some(py) = on_release {
        add_callback_to_mutex(id, "on_release".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    let width = get_length(width, width_fill);
    let height = height.unwrap_or(16.0);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgSlider(
        IpgSlider { 
            id,
            parent_id,
            show,
            min,
            max,
            step,
            shift_step,
            value,
            width,
            height,
            style_id,
        }));

    drop(state);
    Ok(id)

}

#[pyfunction]
#[pyo3(signature = (
    rail_color=None,
    rail_rgba=None,
    rail_color_hovered=None,
    rail_rgba_hovered=None,
    rail_width=None,
    rail_border_radius=None,
    handle_circle_radius=None,
    handle_rectangle_width=None,
    handle_rectangle_border_radius=None,
    handle_color=None,
    handle_rgba=None,
    handle_border_width=None,
    handle_border_color=None,
    handle_border_rgba=None,
    gen_id=None,
    ))]
pub fn add_slider_style(
    rail_color: Option<IpgColor>,
    rail_rgba: Option<[f32; 4]>,
    rail_color_hovered: Option<IpgColor>,
    rail_rgba_hovered: Option<[f32; 4]>,
    rail_width: Option<f32>,
    rail_border_radius: Option<Vec<f32>>,
    handle_circle_radius: Option<f32>,
    handle_rectangle_width: Option<u16>,
    handle_rectangle_border_radius: Option<Vec<f32>>,
    handle_color: Option<IpgColor>,
    handle_rgba: Option<[f32; 4]>,
    handle_border_width: Option<f32>,
    handle_border_color: Option<IpgColor>,
    handle_border_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    )  -> PyResult<usize>
{
    let id = get_id(gen_id);

    let rail_color = 
        IpgColor::rgba_ipg_color_to_iced(rail_rgba, rail_color, 1.0, false);
    let rail_color_hovered = 
        IpgColor::rgba_ipg_color_to_iced(rail_rgba_hovered, rail_color_hovered, 1.0, false);
    let handle_color = 
        IpgColor::rgba_ipg_color_to_iced(handle_rgba, handle_color, 1.0, false);
    let handle_border_color = 
        IpgColor::rgba_ipg_color_to_iced(handle_border_rgba,handle_border_color,1.0, false);

    let mut state = access_state();
    
    state.widgets.insert(id, IpgWidgets::IpgSliderStyle(
        IpgSliderStyle {
            id,
            rail_color,
            rail_color_hovered,
            rail_width,
            rail_border_radius,
            handle_circle_radius,
            handle_rectangle_width,
            handle_rectangle_border_radius,
            handle_color,
            handle_border_width,
            handle_border_color,
        }));

    drop(state);
    Ok(id)

}
