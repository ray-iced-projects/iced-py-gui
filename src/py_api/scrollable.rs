//! Scrollable module - Provides add_scrollable and 
//!                     add_scrollable_style to pyfunction

use iced::Color;
use pyo3::{Py, PyAny, PyResult, pyfunction};

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::IpgColor, py_api::helpers::{get_height, get_width}, 
    state::{IpgContainers, IpgWidgets, get_id, set_state_cont_wnd_ids, 
        set_state_of_container}, widgets::ipg_scrollable::{
            IpgAnchor, IpgAutoScrollStyle, IpgRailStyle, IpgScrollable, IpgScrollbar}};
type PyObject = Py<PyAny>;


#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    width=None,
    width_fill=false, 
    height=None, 
    height_fill=false,
    scrollbar_x_id=None,
    scrollbar_y_id=None,
    on_scroll=None, 
    user_data=None,
    container_style_id=None,
    rail_x_style_id=None,
    rail_y_style_id=None,
    auto_scroll_style_id=None,
    gap_background_color=None,
    gap_background_rgba=None,
    ))]
pub fn add_scrollable(
    window_id: String,
    container_id: String,
    // above required
    parent_id: Option<String>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    scrollbar_x_id: Option<usize>,
    scrollbar_y_id: Option<usize>,
    on_scroll: Option<PyObject>,
    user_data: Option<PyObject>,
    container_style_id: Option<usize>,
    rail_x_style_id: Option<usize>,
    rail_y_style_id: Option<usize>,
    auto_scroll_style_id: Option<usize>,
    gap_background_color: Option<IpgColor>,
    gap_background_rgba: Option<[f32; 4]>,
    ) -> PyResult<usize>
{
    let id = get_id(None);

    let gap_background_color = 
        IpgColor::rgba_ipg_color_to_iced(gap_background_rgba, gap_background_color, 1.0, false);

    if let Some(py) = on_scroll {
        add_callback_to_mutex(id, "on_scroll".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_scrollable".to_string());
    
    state.containers.insert(id, IpgContainers::IpgScrollable(
        IpgScrollable { 
            id,
            width,
            height,
            scrollbar_x_id,
            scrollbar_y_id,
            container_style_id,
            rail_x_style_id,
            rail_y_style_id,
            auto_scroll_style_id,
            gap_background_color,
        }));

    drop(state);
    Ok(id)

}

#[pyfunction]
#[pyo3(signature = ( 
    width=None,
    margin=None,
    scroller_width=None,
    spacing=None,
    anchor=None,
    hidden=None,
    gen_id=None,
    ))]
pub fn add_scrollbar (
    width: Option<f32>,
    margin: Option<f32>,
    scroller_width: Option<f32>,
    spacing: Option<f32>,
    anchor: Option<IpgAnchor>,
    hidden: Option<bool>,
    gen_id: Option<usize>,
    )-> PyResult<usize>
{

    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgScrollbar (
        IpgScrollbar {
        id,
        width,
        margin,
        scroller_width,
        spacing,
        anchor,
        hidden,
    }));

    drop(state);
    Ok(id)
}


#[pyfunction]
#[pyo3(signature = ( 
    background_color=None,
    background_rgba=None,
    border_color=None,
    border_rgba=None,
    border_width=None,
    border_radius=None,
    gen_id=None
    ))]
pub fn add_rail_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_width: Option<f32>,
    border_radius: Option<Vec<f32>>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0, false);
    
    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgRailStyle(
        IpgRailStyle { 
            id,
            background,
            border_color,
            border_width,
            border_radius,
        }));

    drop(state);
    Ok(id)

}

#[pyfunction]
#[pyo3(signature = ( 
    background_color=None,
    background_rgba=None,
    border_color=None,
    border_rgba=None,
    border_width=None,
    border_radius=None,
    shadow_color=None,
    shadow_rgba=None,
    shadow_offset=None,
    shadow_blur_radius=None,
    shadow_icon_color=None,
    shadow_icon_rgba=None,
    gen_id=None
    ))]
pub fn add_autoscroll_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_width: Option<f32>,
    border_radius: Option<Vec<f32>>,
    shadow_color: Option<IpgColor>,
    shadow_rgba: Option<[f32; 4]>,
    shadow_offset: Option<[f32; 2]>,
    shadow_blur_radius: Option<f32>,
    shadow_icon_color: Option<IpgColor>,
    shadow_icon_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0, false);
    let shadow_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(shadow_rgba, shadow_color, 1.0, false);
    let shadow_icon_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(shadow_icon_rgba, shadow_icon_color, 1.0, false);
    
    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgAutoScrollStyle(
        IpgAutoScrollStyle { 
            id,
            background,
            border_color,
            border_width,
            border_radius,
            shadow_color,
            shadow_offset,
            shadow_blur_radius,
            shadow_icon_color,
        }));

    drop(state);
    Ok(id)

}