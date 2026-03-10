

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::IpgColor, py_api::helpers::get_length, 
    state::{IpgWidgets, get_id, set_state_of_widget}, 
    widgets::{enums::AlignX, ipg_text_input::{IpgTextInput, IpgTextInputStyle}}};

use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;


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

#[pyfunction]
#[pyo3(signature = ( 
    background_color=None,
    background_rgba=None,
    border_color_active=None,
    border_rgba_active=None,
    border_color_hovered=None,
    border_rgba_hovered=None,
    border_color_focused=None,
    border_rgba_focused=None,
    border_color_disabled=None,
    border_rgba_disabled=None,
    border_width=None,
    border_radius=None,
    // icon_color=None,
    // icon_rgba=None,
    placeholder_color_active=None,
    placeholder_rgba_active=None,
     placeholder_color_disabled=None,
    placeholder_rgba_disabled=None,
    value_color=None,
    value_rgba=None,
    selection_color=None,
    selection_rgba=None,
    gen_id=None))]
pub fn add_text_input_style(
        background_color: Option<IpgColor>,
        background_rgba: Option<[f32; 4]>,
        border_color_active: Option<IpgColor>,
        border_rgba_active: Option<[f32; 4]>,
        border_color_hovered: Option<IpgColor>,
        border_rgba_hovered: Option<[f32; 4]>,
        border_color_focused: Option<IpgColor>,
        border_rgba_focused: Option<[f32; 4]>,
        border_color_disabled: Option<IpgColor>,
        border_rgba_disabled: Option<[f32; 4]>,
        border_width: Option<f32>,
        border_radius: Option<f32>,
        // icon_color: Option<IpgColor>,
        // icon_rgba: Option<[f32; 4]>,
        placeholder_color_active: Option<IpgColor>,
        placeholder_rgba_active: Option<[f32; 4]>,
        placeholder_color_disabled: Option<IpgColor>,
        placeholder_rgba_disabled: Option<[f32; 4]>,
        value_color: Option<IpgColor>,
        value_rgba: Option<[f32; 4]>,
        selection_color: Option<IpgColor>,
        selection_rgba: Option<[f32; 4]>,
        gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let border_color_active = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_active, border_color_active, 1.0, false);
    let border_color_hovered = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_hovered, border_color_hovered, 1.0, false);
    let border_color_focused = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_focused, border_color_focused, 1.0, false);
    let border_color_disabled = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_disabled, border_color_disabled, 1.0, false);
    
    // let icon_color = get_color(icon_rgba, icon_color, 1.0, false);
    let placeholder_color_active = 
        IpgColor::rgba_ipg_color_to_iced(placeholder_rgba_active, placeholder_color_active, 1.0, false);
    let placeholder_color_disabled = 
        IpgColor::rgba_ipg_color_to_iced(placeholder_rgba_disabled, placeholder_color_disabled, 1.0, false);

    let value_color = 
        IpgColor::rgba_ipg_color_to_iced(value_rgba, value_color, 1.0, false);
    let selection_color = 
        IpgColor::rgba_ipg_color_to_iced(selection_rgba, selection_color, 1.0, false);

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
