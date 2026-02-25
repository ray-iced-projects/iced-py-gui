//! Scrollable module - Provides add_scrollable and 
//!                     add_scrollable_style to pyfunction

use iced::{Color, Rectangle};
use pyo3::{Py, PyAny, PyResult, pyfunction};

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::IpgColor, py_api::helpers::{get_height, get_width}, 
    state::{IpgContainers, IpgWidgets, get_id, set_state_cont_wnd_ids, 
        set_state_of_container, set_state_of_widget}, widgets::{ipg_scrollable::
        {IpgAnchor, IpgScrollable, IpgScrollableStyle, IpgScrollbar}}};
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
    style_id=None,
    container_style_id=None,
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
    style_id: Option<usize>,
    container_style_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(None);

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
            style_id,
            container_style_id,
            scroll_y_pos: None,
            scroll_x_pos: None,
            bounds: Rectangle::default(),
            content_bounds: Rectangle::default(),
        }));

    drop(state);
    Ok(id)

}

#[pyfunction]
#[pyo3(signature = ( 
    parent_id,
    x_direction,
    y_direction,
    width=None,
    margin=None,
    scroller_width=None,
    spacing=None,
    alignment=None,
    gen_id=None,
    ))]
pub fn add_scrollbar (
    parent_id: String,
    mut x_direction: bool,
    y_direction: bool,
    width: Option<f32>,
    margin: Option<f32>,
    scroller_width: Option<f32>,
    spacing: Option<f32>,
    alignment: Option<IpgAnchor>,
    gen_id: Option<usize>,
    )-> PyResult<usize>
{

    let id = get_id(gen_id);

    if !x_direction && !y_direction {
        x_direction = true;
    }

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgScrollbar (
        IpgScrollbar {
        id,
        x_direction,
        y_direction,
        width,
        margin,
        scroller_width,
        spacing,
        alignment,
    }));

    drop(state);
    Ok(id)
}

#[pyfunction]
#[pyo3(signature = ( 
    scrollbar_color=None,
    scrollbar_rgba=None,
    scrollbar_border_radius=None,
    scrollbar_border_width=None,
    scrollbar_border_color=None,
    scrollbar_border_rgba=None,
    scroller_color=None,
    scroller_rgba=None,
    scroller_color_hovered=None,
    scroller_rgba_hovered=None,
    scroller_color_dragged=None,
    scroller_rgba_dragged=None,
    gen_id=None
    ))]
pub fn add_scrollable_style(
    scrollbar_color: Option<IpgColor>,
    scrollbar_rgba: Option<[f32; 4]>,
    scrollbar_border_radius: Option<Vec<f32>>,
    scrollbar_border_width: Option<f32>,
    scrollbar_border_color: Option<IpgColor>,
    scrollbar_border_rgba: Option<[f32; 4]>,
    scroller_color: Option<IpgColor>,
    scroller_rgba: Option<[f32; 4]>,
    scroller_color_hovered: Option<IpgColor>,
    scroller_rgba_hovered: Option<[f32; 4]>,
    scroller_color_dragged: Option<IpgColor>,
    scroller_rgba_dragged: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let scrollbar_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(scrollbar_rgba, scrollbar_color, 1.0, false);
    let scrollbar_border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(scrollbar_border_rgba, scrollbar_border_color, 1.0, false);
    
    let scroller_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(scroller_rgba, scroller_color, 1.0, false);
    let scroller_color_hovered: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(scroller_rgba_hovered, scroller_color_hovered, 1.0, false);
    let scroller_color_dragged: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(scroller_rgba_dragged, scroller_color_dragged, 1.0, false);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgScrollableStyle(
        IpgScrollableStyle { 
            id,
            scrollbar_color,
            scrollbar_border_radius,
            scrollbar_border_width,
            scrollbar_border_color,
            scroller_color,
            scroller_color_hovered,
            scroller_color_dragged,
        }));

    drop(state);
    Ok(id)

}

