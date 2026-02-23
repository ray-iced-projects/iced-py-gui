//! ProgressBar module - provides add_progress_bar pyfunction
use pyo3::{pyfunction, PyResult};

use crate::access_state;
use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::{get_height, get_width};
use crate::state::{IpgWidgets, get_id, set_state_of_widget};
use crate::widgets::ipg_progress_bar::{IpgProgressBar, IpgProgressBarStyle};
use crate::widgets::styling::IpgStyleStandard;




#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    min, 
    max, 
    value,
    gen_id=None,
    is_vertical=None,
    width=None,
    width_fill=true,  
    height=None, 
    height_fill=false,
    style_standard=None, 
    style_id=None, 
    show=true, 
    ))]
pub fn add_progress_bar(
    parent_id: String,
    min: f32,
    max: f32,
    value: f32,
    // **above required
    gen_id: Option<usize>,
    is_vertical: Option<bool>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    style_standard: Option<IpgStyleStandard>,
    style_id: Option<usize>,
    show: bool,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgProgressBar(
        IpgProgressBar {   
            id,
            parent_id,
            show,
            min,
            max,
            value,
            is_vertical,
            width,
            height,
            style_standard,
            style_id,
        }));

    drop(state);
    Ok(id)

}


#[pyfunction]
#[pyo3(signature = (
    background_color=None, 
    background_rgba=None,
    bar_color=None, 
    bar_rgba=None,
    border_color=None, 
    border_rgba=None,
    border_radius=None, 
    border_width=None,
    gen_id=None
    ))]
pub fn add_progress_bar_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    bar_color: Option<IpgColor>,
    bar_rgba: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let bar_color = 
        IpgColor::rgba_ipg_color_to_iced(bar_rgba, bar_color, 1.0, false);
    let border_color = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0, false);

    let mut state = access_state();

        state.widgets.insert(id, IpgWidgets::IpgProgressBarStyle(
        IpgProgressBarStyle { 
            id,
            background_color,
            bar_color,
            border_color,
            border_radius,
            border_width,
        }));

    drop(state);
    Ok(id)
}
