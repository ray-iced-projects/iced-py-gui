//! Checkbox module - provides add_checkbox pyfunction
use iced::Color;
use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::bootstrap_icon::Icon;
use crate::graphics::colors::{IpgColor, get_color};
use crate::py_api::helpers::get_width;
use crate::state::{IpgWidgets, access_state, 
    add_callback_to_mutex, get_id, set_state_of_widget};
use crate::widgets::enums::IpgShaping;
use crate::widgets::ipg_checkbox::{IpgCheckBox, IpgCheckboxStyle};
use crate::widgets::styling::IpgStyleStandard;

/// Add a checkbox widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    gen_id=None, 
    on_toggle=None, 
    is_checked=false, 
    label=None, 
    width=None, 
    width_fill=false, 
    size=None, 
    spacing=None, 
    text_size=None,
    text_line_height=None,
    text_shaping=None,
    text_font=None,
    icon_x= false,
    icon_font= None,
    icon=None,
    icon_size=None,
    icon_line_height=None,
    icon_shaping=None,
    user_data=None, 
    show=true, 
    style_id=None, 
    style_standard=None, 
    ))] 
fn add_checkbox(
    parent_id: String,
    // ** above required
    gen_id: Option<usize>,
    on_toggle: Option<PyObject>,
    is_checked: bool,
    label: Option<String>,
    width: Option<f32>,
    width_fill: bool,
    size: Option<f32>,
    spacing: Option<f32>,
    text_size: Option<f32>,
    text_line_height: Option<f32>,
    text_shaping: Option<IpgShaping>,
    text_font: Option<String>,
    icon_x: bool,
    icon_font: Option<String>,
    icon: Option<Icon>,
    icon_size: Option<f32>,
    icon_line_height: Option<f32>,
    icon_shaping: Option<IpgShaping>,
    user_data: Option<PyObject>,
    show: bool,
    style_id: Option<usize>,
    style_standard: Option<IpgStyleStandard>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);
    
    if let Some(py) = on_toggle {
        add_callback_to_mutex(id, "on_toggle".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    let width = get_width(width, width_fill);
    
    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgCheckBox(
        IpgCheckBox {
            id,
            parent_id,
            show,
            is_checked,
            label,
            width,
            size,
            spacing,
            text_size,
            text_line_height,
            text_shaping,
            text_font,
            icon_x,
            icon_font,
            icon,
            icon_size,
            icon_line_height,
            icon_shaping,
            style_id,
            style_standard,
            }));

    drop(state);
    Ok(id)

}

/// Add a checkbox style widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = ( 
    background_color=None, 
    background_rgba=None,
    background_color_hovered=None,
    background_rgba_hovered=None,
    accent_color=None,
    accent_rgba=None,
    accent_color_hovered=None,
    accent_rgba_hovered=None,
    border_color=None, 
    border_rgba=None,
    border_radius=None, 
    border_width=None,
    icon_color=None, 
    icon_rgba=None,
    text_color=None, 
    text_rgba=None,
    gen_id=None
    ))]
pub fn add_checkbox_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    background_color_hovered: Option<IpgColor>,
    background_rgba_hovered: Option<[f32; 4]>,
    accent_color: Option<IpgColor>,
    accent_rgba: Option<[f32; 4]>,
    accent_color_hovered: Option<IpgColor>,
    accent_rgba_hovered: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    icon_color: Option<IpgColor>,
    icon_rgba: Option<[f32; 4]>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color: Option<Color> = 
        get_color(background_rgba, background_color, 1.0, false);
    let background_color_hovered: Option<Color> = 
        get_color(background_rgba_hovered, background_color_hovered, 1.0, false);
    let accent_color: Option<Color> = 
        get_color(accent_rgba, accent_color, 1.0, false);
    let accent_color_hovered: Option<Color> = 
        get_color(accent_rgba_hovered, accent_color_hovered, 1.0, false);
    let border_color: Option<Color> = 
        get_color(border_rgba, border_color, 1.0, false);
    let icon_color: Option<Color> = 
        get_color(icon_rgba, icon_color, 1.0, false);
    let text_color: Option<Color> = 
        get_color(text_rgba, text_color, 1.0, false);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgCheckboxStyle(
        IpgCheckboxStyle {
            id,
            background_color,
            background_color_hovered,
            accent_color,
            accent_color_hovered,
            border_color,
            border_radius,
            border_width,
            icon_color,
            text_color,
        }));

    drop(state);
    Ok(id)
}