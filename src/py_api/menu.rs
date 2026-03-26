//! Menu module - provides add_menu pyfunction

use iced::Length;
use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};

use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::get_length;
use crate::widgets::ipg_menu::{IpgMenu, IpgMenuBarItem, IpgMenuStyle};
use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{IpgContainers, IpgWidgets, get_id, set_state_cont_wnd_ids, set_state_of_container};
type PyObject = Py<PyAny>;




#[pyfunction]
#[pyo3(signature = ( 
    window_id,
    container_id,
    parent_id=None,
    item_offsets=None,
    item_paddings=None,
    item_spacings=None,
    item_widths=None,
    bar_height=None,
    bar_paddings=None,
    bar_spacing=None,
    bar_width=None,
    close_on_item_click=None,
    close_on_background_click=None,
    on_select=None,
    style_id=None,
    style_std_primary=None,
    show=true, 
    user_data=None, 
    gen_id=None
    ))]
pub fn add_menu(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    item_offsets: Option<Vec<f32>>,
    item_paddings: Option<Vec<f32>>,
    item_spacings: Option<Vec<f32>>,
    item_widths: Option<Vec<f32>>,
    bar_height: Option<f32>,
    bar_paddings: Option<Vec<f32>>,
    bar_spacing: Option<f32>,
    bar_width: Option<f32>,
    close_on_item_click: Option<bool>,
    close_on_background_click: Option<bool>,
    on_select: Option<PyObject>,
    style_id: Option<usize>,
    style_std_primary: Option<bool>,
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

    let item_offsets = if let Some(offsets) = item_offsets {
        offsets
    } else {
        vec![0.0]
    };

    let item_paddings = if let Some(pads) = item_paddings {
        pads
    } else {
        vec![0.0]
    };

    let item_spacings = if let Some(sps) = item_spacings {
        sps
    } else {
        vec![0.0]
    };

    let item_widths = if let Some(widths) = item_widths {
        let mut wds = vec![];
        for w in widths {
            wds.push(get_length(Some(w), false));
        }
        wds
    } else {
        vec![Length::Shrink]
    };

    let bar_paddings = if let Some(pads) = bar_paddings {
        pads
    } else {
        vec![0.0]
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
            item_offsets,
            item_paddings,
            item_spacings,
            item_widths,
            bar_height,
            bar_paddings,
            bar_spacing,
            bar_width,
            close_on_item_click,
            close_on_background_click,
            style_id,
            style_std_primary,
            check_bounds_width: None,
            show,
            is_checked: false,
            is_toggled: false,
        }));

    drop(state);
    Ok(id)
}


#[pyfunction]
#[pyo3(signature = (
    bar_background_color=None,
    bar_background_rgba=None,
    bar_background_alpha=None,
    bar_border_color=None,
    bar_border_rgba=None,
    bar_border_alpha=None,
    bar_border_radius=None,
    bar_border_width=None,
    bar_shadow_color=None,
    bar_shadow_rgba=None,
    bar_shadow_alpha=None,
    bar_shadow_offset_xy=None,
    bar_shadow_blur_radius=None,

    menu_background_color=None,
    menu_background_rgba=None,
    menu_background_alpha=None,
    menu_border_color=None,
    menu_border_rgba=None,
    menu_border_alpha=None,
    menu_border_radius=None,
    menu_border_width=None,
    menu_shadow_color=None,
    menu_shadow_rgba=None,
    menu_shadow_alpha=None,
    menu_shadow_offset_xy=None,
    menu_shadow_blur_radius=None,

    path_background_color=None,
    path_background_rgba=None,
    path_background_alpha=None,
    path_border_color=None,
    path_border_rgba=None,
    path_border_alpha=None,
    path_border_radius=None,
    path_border_width=None,

    gen_id=None))]
pub fn add_menu_style(
    bar_background_color: Option<IpgColor>,
    bar_background_rgba: Option<[f32; 4]>,
    bar_background_alpha: Option<f32>,
    bar_border_color: Option<IpgColor>,
    bar_border_rgba: Option<[f32; 4]>,
    bar_border_alpha: Option<f32>,
    bar_border_radius: Option<Vec<f32>>,
    bar_border_width: Option<f32>,
    bar_shadow_color: Option<IpgColor>,
    bar_shadow_rgba: Option<[f32; 4]>,
    bar_shadow_alpha: Option<f32>,
    bar_shadow_offset_xy: Option<[f32; 2]>,
    bar_shadow_blur_radius: Option<f32>,

    menu_background_color: Option<IpgColor>,
    menu_background_rgba: Option<[f32; 4]>,
    menu_background_alpha: Option<f32>,
    menu_border_color: Option<IpgColor>,
    menu_border_rgba: Option<[f32; 4]>,
    menu_border_alpha: Option<f32>,
    menu_border_radius: Option<Vec<f32>>,
    menu_border_width: Option<f32>,
    menu_shadow_color: Option<IpgColor>,
    menu_shadow_rgba: Option<[f32; 4]>,
    menu_shadow_alpha: Option<f32>,
    menu_shadow_offset_xy: Option<[f32; 2]>,
    menu_shadow_blur_radius: Option<f32>,

    path_background_color: Option<IpgColor>,
    path_background_rgba: Option<[f32; 4]>,
    path_background_alpha: Option<f32>,
    path_border_color: Option<IpgColor>,
    path_border_rgba: Option<[f32; 4]>,
    path_border_alpha: Option<f32>,
    path_border_radius: Option<Vec<f32>>,
    path_border_width: Option<f32>,

    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let bar_background_color = 
        IpgColor::rgba_ipg_color_to_iced(
            bar_background_rgba, 
            bar_background_color, 
            bar_background_alpha.unwrap_or(1.0));
    let bar_border_color = 
        IpgColor::rgba_ipg_color_to_iced(
            bar_border_rgba, 
            bar_border_color, 
            bar_border_alpha.unwrap_or(1.0));
    let bar_shadow_color = 
        IpgColor::rgba_ipg_color_to_iced(
            bar_shadow_rgba, 
            bar_shadow_color, 
            bar_shadow_alpha.unwrap_or(1.0));

    let menu_background_color = 
        IpgColor::rgba_ipg_color_to_iced(
            menu_background_rgba, 
            menu_background_color, 
            menu_background_alpha.unwrap_or(1.0));
    let menu_border_color = 
        IpgColor::rgba_ipg_color_to_iced(
            menu_border_rgba, 
            menu_border_color, 
            menu_border_alpha.unwrap_or(1.0));
    let menu_shadow_color = 
        IpgColor::rgba_ipg_color_to_iced(
            menu_shadow_rgba, 
            menu_shadow_color, 
            menu_shadow_alpha.unwrap_or(1.0));

    let path_background_color = 
        IpgColor::rgba_ipg_color_to_iced(
            path_background_rgba, 
            path_background_color, 
            path_background_alpha.unwrap_or(1.0));
    let path_border_color = 
        IpgColor::rgba_ipg_color_to_iced(
            path_border_rgba, 
            path_border_color, 
            path_border_alpha.unwrap_or(1.0));

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgMenuStyle(
        IpgMenuStyle {
            id,
            bar_background_color,
            bar_border_color,
            bar_border_radius,
            bar_border_width,
            bar_shadow_color,
            bar_shadow_offset_xy,
            bar_shadow_blur_radius,
            bar_background_alpha: None,
            bar_border_alpha: None,
            bar_shadow_alpha: None,

            menu_background_color,
            menu_border_color,
            menu_border_radius,
            menu_border_width,
            menu_shadow_color,
            menu_shadow_offset_xy,
            menu_shadow_blur_radius,
            menu_background_alpha: None,
            menu_border_alpha: None,
            menu_shadow_alpha: None,

            path_background_color,
            path_border_color,
            path_border_radius,
            path_border_width,
            path_background_alpha: None,
            path_border_alpha: None,
        }));

    drop(state);
    Ok(id)
}


#[pyfunction]
#[pyo3(signature = ( 
    window_id,
    container_id,
    parent_id=None,
    show=true,
    gen_id=None
    ))]
pub fn add_menu_bar_item(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    show: bool,
    gen_id: Option<usize>,
) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };
    
    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_menu_bar_item".to_string());

    state.containers.insert(id, IpgContainers::IpgMenuBarItem(
        IpgMenuBarItem {
            id,
            show,
        }));

    drop(state);
    Ok(id)
}
