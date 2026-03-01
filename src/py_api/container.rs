//! Container module - provides add_container pyfunction
use iced::Color;
use pyo3::{PyResult, pyfunction};

use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::{get_height, get_width};
use crate::state::{IpgContainers, IpgWidgets, access_state, 
    get_id, set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::ipg_container::{IpgContainer, 
    IpgContainerStyle, IpgContainerStyleStd};
use crate::widgets::enums::{IpgAlignmentX, 
    IpgAlignmentY};

/// Add a container widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    width=None, 
    width_fill=false, 
    height=None, 
    height_fill=false, 
    clip=None, 
    max_height=None, 
    max_width=None,
    align_x=None, 
    align_y=None,
    center_x=None,
    center_y=None,
    center=None,
    align_left=None,
    align_right=None,
    align_top=None,
    align_botton=None,
    padding=None, 
    show=true, 
    style_id=None,
    style_standard=None, 
    ))]
pub fn add_container(
    window_id: String,
    container_id: String,
    // **above required
    parent_id: Option<String>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    clip: Option<bool>,
    max_height: Option<f32>,
    max_width: Option<f32>,
    align_x: Option<IpgAlignmentX>,
    align_y: Option<IpgAlignmentY>,
    center_x: Option<bool>,
    center_y: Option<bool>,
    center: Option<bool>,
    align_left: Option<bool>,
    align_right: Option<bool>,
    align_top: Option<bool>,
    align_botton: Option<bool>, 
    padding: Option<Vec<f32>>, 
    show: bool,
    style_id: Option<usize>,
    style_standard: Option<IpgContainerStyleStd>,
    ) -> PyResult<usize>
{
    let id = get_id(None);

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);
    
    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_container".to_string());

    state.containers.insert(id, IpgContainers::IpgContainer(
        IpgContainer {
            id,
            show,
            padding,
            width,
            height,
            max_width,
            max_height,
            align_x,
            align_y,
            center_x,
            center_y,
            center,
            align_left,
            align_right,
            align_top,
            align_botton,
            clip,
            style_id,
            style_standard,
        }));

    drop(state);
    Ok(id)

}

#[pyfunction]
#[pyo3(signature = (
    background_color=None, 
    background_rgba=None,
    background_alpha=None,
    background_gradient_color_stop=None,
    background_gradient_rgba_stop=None,
    background_gradient_degrees=None,
    background_gradient_radians=None,
    background_gradient_alpha=None,
    border_color=None, 
    border_rgba=None,
    border_alpha=None,
    border_radius=None, 
    border_width=None,
    shadow_color=None, 
    shadow_rgba=None,
    shadow_alpha=None,
    shadow_offset_xy=None,
    shadow_blur_radius=None,
    text_color=None, 
    text_rgba=None,
    text_alpha=None,
    snap=None,
    gen_id=None
    ))]
pub fn add_container_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    background_alpha: Option<f32>,
    background_gradient_color_stop: Option<IpgColor>,
    background_gradient_rgba_stop: Option<[f32; 4]>,
    background_gradient_degrees: Option<f32>,
    background_gradient_radians: Option<f32>,
    background_gradient_alpha: Option<f32>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_alpha: Option<f32>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    shadow_color: Option<IpgColor>,
    shadow_rgba: Option<[f32; 4]>,
    shadow_alpha: Option<f32>,
    shadow_offset_xy: Option<[f32; 2]>,
    shadow_blur_radius: Option<f32>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    text_alpha: Option<f32>,
    snap: Option<bool>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let bkg_a = background_alpha.unwrap_or(1.0);
    let grad_a = background_gradient_alpha.unwrap_or(1.0);
    let border_a = border_alpha.unwrap_or(1.0);
    let shadow_a = shadow_alpha.unwrap_or(1.0);
    let text_a = text_alpha.unwrap_or(1.0);

    let background_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, bkg_a, false);
    let background_gradient_color_stop = 
        IpgColor::rgba_ipg_color_to_iced(background_gradient_rgba_stop, background_gradient_color_stop, grad_a, false);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, border_a, false);
    let shadow_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(shadow_rgba, shadow_color, shadow_a, false);
    let text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, text_a, false);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgContainerStyle(
        IpgContainerStyle {
            id,
            background_color,
            background_gradient_color_stop,
            background_gradient_degrees,
            background_gradient_radians,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_xy,
            shadow_blur_radius,
            text_color,
            snap,
        }));

    drop(state);
    Ok(id)
}
