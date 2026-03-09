//! Radio adds add_radio to the pyfunction
use iced::Color;
use pyo3::{pyfunction, PyResult, Py, PyAny};
type PyObject = Py<PyAny>;
use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::IpgColor, py_api::helpers::{get_height, get_width}, 
    state::{IpgWidgets, get_id, set_state_of_widget}, 
    widgets::{ipg_text::TextShaping, ipg_radio::{IpgRadio, IpgRadioDirection, 
        IpgRadioStyle}, ipg_text::TextWrapping}};


#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    labels, 
    gen_id=None,
    direction=IpgRadioDirection::Vertical,
    spacing=None,
    radio_spacing=None, 
    padding=None, 
    width=None, 
    width_fill=false, 
    height=None, 
    height_fill=false,
    on_select=None, 
    selected_index=None, 
    size=None, 
    style_id=None,
    font_id=None,
    text_spacing=None, 
    text_size=None,
    text_line_height=None, 
    text_shaping=None,
    text_wrapping=None, 
    user_data=None, 
    show=true, 
    ))]
pub fn add_radio(
    parent_id: String,
    labels: Vec<String>,
    //**above required
    gen_id: Option<usize>,
    direction: IpgRadioDirection,
    spacing: Option<f32>,
    radio_spacing: Option<f32>,
    padding: Option<Vec<f32>>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    on_select: Option<PyObject>,
    selected_index: Option<usize>,
    size: Option<f32>,
    style_id: Option<usize>,
    font_id: Option<usize>,
    text_spacing: Option<f32>,
    text_size: Option<f32>,
    text_line_height: Option<f32>,
    text_shaping: Option<TextShaping>,
    text_wrapping: Option<TextWrapping>,
    user_data: Option<PyObject>,
    show: bool,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let is_selected = if let Some(val) = selected_index {
        if val > labels.len()-1 {
            panic!("Radio selected_index is greater than the size of the labels")
        } else { Some(val) }
    } else { None };

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

    if let Some(py) = on_select {
        add_callback_to_mutex(id, "on_select".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();
    
    state.widgets.insert(id, IpgWidgets::IpgRadio(
        IpgRadio {
            id,
            parent_id,
            labels,
            direction,
            spacing,
            radio_spacing,
            padding,
            show,
            is_selected,
            width,
            height,
            size,
            text_spacing,
            text_size,
            text_line_height,
            text_shaping,
            text_wrapping,
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
    background_color_hovered=None,
    background_rgba_hovered=None,
    border_color=None, 
    border_rgba=None,
    border_width=None,
    dot_color=None, 
    dot_rgba=None,
    dot_color_hovered=None, 
    dot_rgba_hovered=None,
    text_color=None, 
    text_rgba=None,
    gen_id=None
    ))]
pub fn add_radio_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    background_color_hovered: Option<IpgColor>,
    background_rgba_hovered: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_width: Option<f32>,
    dot_color: Option<IpgColor>,
    dot_rgba: Option<[f32; 4]>,
    dot_color_hovered: Option<IpgColor>,
    dot_rgba_hovered: Option<[f32; 4]>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let background_color_hovered = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba_hovered, background_color_hovered, 1.0, false);
    let dot_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(dot_rgba, dot_color, 1.0, false);
    let dot_color_hovered: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(dot_rgba_hovered, dot_color_hovered, 1.0, false);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0, false);
    let text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, 1.0, false);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgRadioStyle(
        IpgRadioStyle {
            id,
            background_color,
            background_color_hovered,
            dot_color,
            dot_color_hovered,
            border_color,
            border_width,
            text_color,
        }));

    drop(state);
    Ok(id)

}
