//! Menu module - provides add_menu pyfunction

use iced::Length;
use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};

use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::get_length;
use crate::widgets::ipg_menu::{IpgMenu, IpgMenuBarStyle, IpgMenuStyle};
use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{IpgContainers, IpgWidgets, get_id, set_state_cont_wnd_ids, set_state_of_container};
type PyObject = Py<PyAny>;




#[pyfunction]
#[pyo3(signature = ( 
    window_id,
    container_id,
    bar_items,
    menu_items,
    parent_id=None,
    item_offset=None,
    item_padding=None,
    item_spacing=None,
    item_widths=None,
    bar_height=None,
    bar_padding=None,
    bar_spacing=None,
    bar_width=None,
    close_on_item_click=None,
    close_on_background_click=None,
    on_select=None,
    bar_style_id=None,
    style_id=None,
    show=true, 
    user_data=None, 
    gen_id=None
    ))]
pub fn add_menu(
    window_id: String,
    container_id: String,
    bar_items: usize,
    menu_items: Vec<usize>,
    parent_id: Option<String>,
    item_offset: Option<Vec<f32>>,
    item_padding: Option<Vec<f32>>,
    item_spacing: Option<Vec<f32>>,
    item_widths: Option<Vec<f32>>,
    bar_height: Option<f32>,
    bar_padding: Option<Vec<f32>>,
    bar_spacing: Option<f32>,
    bar_width: Option<f32>,
    close_on_item_click: Option<bool>,
    close_on_background_click: Option<bool>,
    on_select: Option<PyObject>,
    bar_style_id: Option<usize>,
    style_id: Option<usize>,
    show: bool,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    if let Some(py) = on_select {
        add_callback_to_mutex(id, "on_select".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let bar_height = get_length(bar_height, false);
    let bar_width = get_length(bar_width, false);

    let item_widths = if let Some(widths) = item_widths {
        let mut wds = vec![];
        for w in widths {
            wds.push(get_length(Some(w), false));
        }
        wds
    } else {
        vec![Length::Shrink]
    };

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };
    
    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_menu".to_string());

    state.containers.insert(id, IpgContainers::IpgMenu(
        IpgMenu {
            id,
            bar_items,
            menu_items,
            item_offset,
            item_padding,
            item_spacing,
            item_widths,
            bar_height,
            bar_padding,
            bar_spacing,
            bar_width,
            close_on_item_click,
            close_on_background_click,
            bar_style_id,
            style_id,
            show,
            is_checked: false,
            is_toggled: false,
        }));

    drop(state);
    Ok(id)
}


#[pyfunction]
#[pyo3(signature = (
    base_color=None,
    base_rgba=None,
    border_color=None,
    border_rgba=None,
    border_radius=None,
    border_width=None,
    shadow_color=None,
    shadow_rgba=None,
    shadow_offset_xy=None,
    shadow_blur_radius=None,
    gen_id=None))]
pub fn add_menu_bar_style(
    base_color: Option<IpgColor>,
    base_rgba: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    shadow_color: Option<IpgColor>,
    shadow_rgba: Option<[f32; 4]>,
    shadow_offset_xy: Option<[f32; 2]>,
    shadow_blur_radius: Option<f32>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let base_color = 
        IpgColor::rgba_ipg_color_to_iced(base_rgba, base_color, 1.0, false);
    let border_color = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0, false);
    let shadow_color = 
        IpgColor::rgba_ipg_color_to_iced(shadow_rgba, shadow_color, 1.0, false);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgMenuBarStyle(
        IpgMenuBarStyle {
            id,
            base_color,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_xy,
            shadow_blur_radius,
        }));

    drop(state);
    Ok(id)
}


#[pyfunction]
#[pyo3(signature = (
    base_color=None,
    base_rgba=None,
    border_color=None,
    border_rgba=None,
    border_radius=None,
    border_width=None,
    shadow_color=None,
    shadow_rgba=None,
    shadow_offset_xy=None,
    shadow_blur_radius=None,
    path_base_color=None,
    path_base_rgba=None,
    path_border_color=None,
    path_border_rgba=None,
    path_border_radius=None,
    path_border_width=None,
    gen_id=None))]
pub fn add_menu_style(
    base_color: Option<IpgColor>,
    base_rgba: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    shadow_color: Option<IpgColor>,
    shadow_rgba: Option<[f32; 4]>,
    shadow_offset_xy: Option<[f32; 2]>,
    shadow_blur_radius: Option<f32>,
    path_base_color: Option<IpgColor>,
    path_base_rgba: Option<[f32; 4]>,
    path_border_color: Option<IpgColor>,
    path_border_rgba: Option<[f32; 4]>,
    path_border_radius: Option<Vec<f32>>,
    path_border_width: Option<f32>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);
    
    let base_color = 
        IpgColor::rgba_ipg_color_to_iced(base_rgba, base_color, 1.0, false);
    let border_color = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0, false);
    let shadow_color = 
        IpgColor::rgba_ipg_color_to_iced(shadow_rgba, shadow_color, 1.0, false);
    let path_base_color = 
        IpgColor::rgba_ipg_color_to_iced(path_base_rgba, path_base_color, 1.0, false);
    let path_border_color = 
        IpgColor::rgba_ipg_color_to_iced(path_border_rgba, path_border_color, 1.0, false);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgMenuStyle(
        IpgMenuStyle {
            id,
            base_color,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_xy,
            shadow_blur_radius,
            path_base_color,
            path_border_color,
            path_border_radius,
            path_border_width,
        }));

    drop(state);

    Ok(id)
}