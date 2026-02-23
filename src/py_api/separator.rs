

use pyo3::{PyResult, pyfunction};

use crate::{access_state, graphics::colors::IpgColor, py_api::helpers::
    {get_height, get_width}, state::{IpgWidgets, 
        get_id, set_state_of_widget}, widgets::ipg_separator::{IpgSeparator, IpgSeparatorStyle, IpgSeparatorType}};


#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    label=None,
    separator_type=None, 
    label_left_width=None,
    label_right_width=None,
    dot_radius=4.0, 
    dot_count=1,
    dot_fill=true, 
    dot_border_width=0.0,
    width=None, 
    width_fill=false, 
    height=None, 
    height_fill=false,
    spacing=None, 
    style_id=None,
    gen_id=None, 
    show=true
    ))]
fn add_separator(
    parent_id: String,
    label: Option<String>,
    separator_type: Option<IpgSeparatorType>,
    label_left_width: Option<f32>,
    label_right_width: Option<f32>,
    dot_radius: Option<f32>,
    dot_count: Option<usize>,
    dot_fill: bool,
    dot_border_width: Option<f32>,
    width: Option<f32>, 
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    spacing: Option<f32>,
    style_id: Option<usize>,
    gen_id: Option<usize>,
    show: bool,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgSeparator(
        IpgSeparator {
            id,
            parent_id,
            separator_type,
            label,
            label_left_width,
            label_right_width,
            dot_radius,
            dot_count,
            dot_fill,
            dot_border_width,
            width,
            height,
            spacing,
            style_id,
            show,
        }));

    drop(state);
    Ok(id)

}


#[pyfunction]
#[pyo3(signature = (
    ipg_color=None,
    rgba_color=None,
    border_ipg_color=None,
    border_rgba_color=None,
    gen_id=None,
    ))]
fn add_separator_style(
    ipg_color: Option<IpgColor>,
    rgba_color: Option<[f32; 4]>,
    border_ipg_color: Option<IpgColor>,
    border_rgba_color: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let color = 
        IpgColor::rgba_ipg_color_to_iced(rgba_color, ipg_color, 1.0, false);
    let border_color = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_color, border_ipg_color, 1.0, false);

    let mut state = access_state();
    
    state.widgets.insert(id, IpgWidgets::IpgSeparatorStyle(
        IpgSeparatorStyle {
            id,
            color,
            border_color,
        }));

    drop(state);
    Ok(id)
}
