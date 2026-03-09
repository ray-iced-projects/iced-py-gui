//! Toggler provide add_toggle and add_toggle_style to a python function
use pyo3::{pyfunction, Py, PyAny, PyResult};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex, graphics::colors::IpgColor, 
    py_api::helpers::get_width, state::{IpgWidgets, get_id, 
        set_state_of_widget}, widgets::{ 
        ipg_text::{TextShaping, TextWrapping}, 
        ipg_toggle::{IpgToggler, IpgTogglerStyle}}};



#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    label=None, 
    gen_id=None, 
    toggled=None, 
    width=None, 
    width_fill=false, 
    size=None, 
    text_size=None,
    text_line_height=None, 
    text_center=None,
    text_left=None,
    text_right=None,
    text_shaping=None,
    text_wrapping=None, 
    spacing=None, 
    user_data=None, 
    show=true,
    font_id=None, 
    style_id=None, 
    ))]
pub fn add_toggler(
    parent_id: String,
    // ** above required
    label: Option<String>,
    gen_id: Option<usize>,
    toggled: Option<PyObject>,
    width: Option<f32>,
    width_fill: bool,
    size: Option<f32>,
    text_size: Option<f32>,
    text_line_height: Option<f32>,
    text_center: Option<bool>,
    text_left: Option<bool>,
    text_right: Option<bool>,
    text_shaping: Option<TextShaping>,
    text_wrapping: Option<TextWrapping>,
    spacing: Option<f32>,
    user_data: Option<PyObject>,
    show: bool,
    font_id: Option<usize>,
    style_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    if let Some(py) = toggled {
        add_callback_to_mutex(id, "toggled".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let width = get_width(width, width_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgToggler(
        IpgToggler {
            id,
            parent_id,
            show,
            label,
            width,
            is_toggled: false,
            size,
            text_size,
            text_line_height,
            text_center,
            text_left,
            text_right,
            text_shaping,
            text_wrapping,
            spacing,
            font_id,
            style_id,                           
        }));

    drop(state);
    Ok(id)

}


#[pyfunction]
#[pyo3(signature = (
    background_color=None,
    background_rgba=None,
    background_border_color=None,
    background_border_rgba=None,
    background_border_width=None,
    foreground_color=None,
    foreground_rgba=None,
    foreground_border_color=None,
    foreground_border_rgba=None,
    foreground_border_width=None,
    text_ipg_color=None,
    text_rgba_color=None, 
    border_radius=None, 
    padding_ratio=None, 
    gen_id=None,
    ))]
pub fn add_toggler_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    background_border_color: Option<IpgColor>,
    background_border_rgba: Option<[f32; 4]>,
    background_border_width: Option<f32>,
    foreground_color: Option<IpgColor>,
    foreground_rgba: Option<[f32; 4]>,
    foreground_border_color: Option<IpgColor>,
    foreground_border_rgba: Option<[f32; 4]>,
    foreground_border_width: Option<f32>,
    text_ipg_color: Option<IpgColor>,
    text_rgba_color: Option<[f32; 4]>, 
    border_radius: Option<Vec<f32>>,
    padding_ratio: Option<f32>, 
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let background_border_color = 
        IpgColor::rgba_ipg_color_to_iced(background_border_rgba, background_border_color, 1.0, false);
    let foreground_color = 
        IpgColor::rgba_ipg_color_to_iced(foreground_rgba, foreground_color, 1.0, false);
    let foreground_border_color = 
        IpgColor::rgba_ipg_color_to_iced(foreground_border_rgba, foreground_border_color, 1.0, false);
    
    let text_color = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba_color, text_ipg_color, 1.0, false);

    let mut state = access_state();
    
    state.widgets.insert(id, IpgWidgets::IpgTogglerStyle(
        IpgTogglerStyle {
            id,
            background_color,
            background_border_color,
            background_border_width,
            foreground_color,
            foreground_border_color,
            foreground_border_width, 
            text_color, 
            border_radius, 
            padding_ratio, 
        }));

    drop(state);
    Ok(id)

}