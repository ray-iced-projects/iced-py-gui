//! Scrollable module - Provides add_scrollable and 
//!                     add_scrollable_style to pyfunction

use iced::{Color, Rectangle};
use pyo3::{Py, PyAny, PyResult, pyfunction};

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, graphics::colors::IpgColor, py_api::helpers::{get_height, get_width}, state::{IpgContainers, IpgWidgets, get_id, set_state_cont_wnd_ids, set_state_of_container}, widgets::ipg_scrollable::{IpgScrollable, IpgScrollableAlignment, IpgScrollableDirection, IpgScrollableStyle}};
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
    direction=None, 
    h_bar_width=None, 
    h_bar_margin=None, 
    h_scroller_width=None,
    h_spacing=None, 
    h_bar_alignment=None,
    v_bar_width=None, 
    v_bar_margin=None, 
    v_scroller_width=None,
    v_spacing=None, 
    v_bar_alignment=None,
    on_scroll=None, 
    user_data=None,
    style_id=None,
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
    direction: Option<IpgScrollableDirection>,
    h_bar_width: Option<f32>,
    h_bar_margin: Option<f32>,
    h_scroller_width: Option<f32>,
    h_spacing: Option<f32>,
    h_bar_alignment: Option<IpgScrollableAlignment>,
    v_bar_width: Option<f32>,
    v_bar_margin: Option<f32>,
    v_scroller_width: Option<f32>,
    v_spacing: Option<f32>,
    v_bar_alignment: Option<IpgScrollableAlignment>,
    on_scroll: Option<PyObject>,
    user_data: Option<PyObject>,
    style_id: Option<usize>,
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
            direction,
            h_bar_width,
            h_bar_margin,
            h_scroller_width,
            h_spacing,
            h_bar_alignment,
            v_bar_width,
            v_bar_margin,
            v_scroller_width,
            v_spacing,
            v_bar_alignment,
            style_id,
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
    background_color=None, 
    background_rgba=None,
    border_color=None, 
    border_rgba=None,
    border_radius=None, 
    border_width=None,
    shadow_color=None, 
    shadow_rgba=None,
    shadow_offset_x=None, 
    shadow_offset_y=None,
    shadow_blur_radius=None,
    text_color=None, 
    text_rgba=None,
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
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    shadow_color: Option<IpgColor>,
    shadow_rgba: Option<[f32; 4]>,
    shadow_offset_x: Option<f32>,
    shadow_offset_y: Option<f32>,
    shadow_blur_radius: Option<f32>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
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

    let background_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0, false);
    let shadow_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(shadow_rgba, shadow_color, 1.0, false);
    let text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, 1.0, false);

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
            background_color,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_x,
            shadow_offset_y,
            shadow_blur_radius,
            text_color,
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
