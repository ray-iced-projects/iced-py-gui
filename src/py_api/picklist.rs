//! PickList module - provides add_pick_list pyfunction

use iced::Color;

use pyo3::{Py, PyAny, PyResult, pyfunction};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex};
use crate::graphics::{colors::IpgColor, 
        bootstrap_arrow::IpgArrow}; 
use crate::py_api::helpers::{get_height, get_width}; 
use crate::state::{IpgWidgets, get_id, set_state_of_widget}; 
use crate::widgets::{ipg_text::TextShaping, 
        ipg_pick_list::{IpgPickList, IpgPickListHandle, 
        IpgPickListStyle, convert_pyobject_vec_string}};



/// Add a cpick list widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    options, 
    gen_id=None, 
    on_select=None, 
    width=None, 
    width_fill=false,
    menu_height=None,
    menu_height_fill=false, 
    padding=None,  
    placeholder=None, 
    selected=None, 
    text_size=None, 
    text_line_height=None, 
    text_shaping=None, 
    handle=None, 
    arrow_size=None, 
    dynamic_closed=None, 
    dynamic_open=None, 
    custom_static=None,
    style_id=None, 
    user_data=None, 
    show=true,
    ))]
pub fn add_pick_list(
    parent_id: String,
    options: PyObject,
    // **above required
    gen_id: Option<usize>,
    on_select: Option<PyObject>,
    width: Option<f32>,
    width_fill: bool,
    menu_height: Option<f32>,
    menu_height_fill: bool,
    padding: Option<Vec<f32>>,
    placeholder: Option<String>,
    selected: Option<String>,
    text_size: Option<f32>,
    text_line_height: Option<f32>,
    text_shaping: Option<TextShaping>,
    handle: Option<IpgPickListHandle>,
    arrow_size: Option<f32>,
    dynamic_closed: Option<IpgArrow>,
    dynamic_open: Option<IpgArrow>,
    custom_static: Option<IpgArrow>,
    style_id: Option<usize>,
    user_data: Option<PyObject>,
    show: bool,
) -> PyResult<usize>
{
    let id = get_id(gen_id);

    if let Some(py) = on_select {
        add_callback_to_mutex(id, "on_select".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let width = get_width(width, width_fill);
    let menu_height = get_height(menu_height, menu_height_fill);

    let options =  convert_pyobject_vec_string(options);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgPickList(
        IpgPickList { 
            id,
            parent_id,
            show,
            options,
            placeholder,
            selected,
            width,
            menu_height,
            padding,
            text_size,
            text_line_height,
            text_shaping,
            handle,
            arrow_size,
            dynamic_closed,
            dynamic_open,
            custom_static,
            style_id,
        }));


    drop(state);
    Ok(id)
}

/// Add a pick list style widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    background_color=None,
    background_rgba=None,
    text_color=None,
    text_rgba=None,
    handle_color=None,
    handle_rgba=None,
    placeholder_color=None,
    placeholder_rgba=None,
    border_color=None,
    border_rgba=None,
    border_color_hovered=None,
    border_rgba_hovered=None,
    border_radius=None,
    border_width=None,
    gen_id=None
    ))]
pub fn add_pick_list_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    handle_color: Option<IpgColor>,
    handle_rgba: Option<[f32; 4]>,
    placeholder_color: Option<IpgColor>,
    placeholder_rgba: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_color_hovered: Option<IpgColor>,
    border_rgba_hovered: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);
    
    let background_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0, false);
    let border_color_hovered: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_hovered, border_color_hovered, 1.0, false);
    let handle_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(handle_rgba, handle_color, 1.0, false);
    let placeholder_color = 
        IpgColor::rgba_ipg_color_to_iced(placeholder_rgba, placeholder_color, 1.0, false);
    let text_color = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, 1.0, false);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgPickListStyle(
        IpgPickListStyle {
            id,
            background_color,
            text_color,
            handle_color,
            placeholder_color,
            border_color,
            border_color_hovered,
            border_radius,
            border_width,
        }));

    drop(state);
    Ok(id)
}
