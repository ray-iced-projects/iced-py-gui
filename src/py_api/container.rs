//! Container module - provides add_container pyfunction
use iced::Color;
use pyo3::{PyResult, pyfunction};

use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::get_length;
use crate::state::{IpgContainers, IpgWidgets, access_state, 
    get_id, set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::ipg_container::{IpgContainer, 
    IpgContainerStyle, IpgContainerStyleStd};


/// Add a container widget.
///
/// A container wraps a single child widget and provides alignment,
/// sizing, padding, and optional styling.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this container belongs to.
/// container_id : str
///     Sets the Unique string identifier for the container.
/// parent_id : str,  Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// width : float,  Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the container fills available width.
/// fill : bool, Optional
///     Whether to fill both the available width and height
/// height : float,  Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the container fills available height.
/// clip : bool,  Optional
///     Whether to clip content that overflows the container.
/// max_height : float,  Optional
///     Sets the Maximum height in logical pixels.
/// max_width : float,  Optional
///     Sets the Maximum width in logical pixels.
/// align_top_left : bool,  Optional
///     Whether to Align the child to the top-left corner.
/// align_top_center : bool,  Optional
///     Whether to Align the child to the top-centre.
/// align_top_right : bool,  Optional
///     Whether to Align the child to the top-right corner.
/// align_center_left : bool,  Optional
///     Whether to Align the child to the centre-left.
/// align_center : bool,  Optional
///     Whether to Align the child to the centre.
/// align_center_right : bool,  Optional
///     Whether to Align the child to the centre-right.
/// align_bottom_left : bool,  Optional
///     Whether to Align the child to the bottom-left corner.
/// align_bottom_center : bool,  Optional
///     Whether to Align the child to the bottom-centre.
/// align_bottom_right : bool,  Optional
///     Whether to Align the child to the bottom-right corner.
/// padding : list of float,  Optional
///     Sets the Padding as ``[all]``, ``[vertical, horizontal]``, or
///     ``[top, right, bottom, left]``.
/// show : bool, default True
///     Whether the container is visible.
/// style_id : int,  Optional
///     Sets the ID of a custom style created with ``add_container_style``.
/// style_std : IpgContainerStyleStd,  Optional
///     Sets the predefined standard style variant.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created container.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    width=None, 
    width_fill=false, 
    height=None, 
    height_fill=false,
    fill=None,
    clip=None, 
    max_height=None, 
    max_width=None,
    align_top_left=None,
    align_top_center=None,
    align_top_right=None,
    align_center_left=None,
    align_center=None,
    align_center_right=None,
    align_bottom_left=None,
    align_bottom_center=None,
    align_bottom_right=None,
    padding=None,
    show=true, 
    style_id=None,
    style_std=None, 
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
    fill: Option<bool>,
    clip: Option<bool>,
    max_height: Option<f32>,
    max_width: Option<f32>,
    align_top_left: Option<bool>,
    align_top_center: Option<bool>,
    align_top_right: Option<bool>,
    align_center_left: Option<bool>,
    align_center: Option<bool>,
    align_center_right: Option<bool>,
    align_bottom_left: Option<bool>,
    align_bottom_center: Option<bool>,
    align_bottom_right: Option<bool>, 
    padding: Option<Vec<f32>>, 
    show: bool,
    style_id: Option<usize>,
    style_std: Option<IpgContainerStyleStd>,
    ) -> PyResult<usize>
{
    let id = get_id(None);

    let (width, height) = if fill == Some(true) {
        (get_length(None, true), get_length(None, true))
    } else {
        (get_length(width, width_fill), get_length(height, height_fill))
    };
    
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
            align_top_left,
            align_top_center,
            align_top_right,
            align_center_left,
            align_center,
            align_center_right,
            align_bottom_left,
            align_bottom_center,
            align_bottom_right,
            clip,
            style_id,
            style_std,
        }));

    drop(state);
    Ok(id)

}


/// Add styling to a container.
///
/// Creates a custom style that can be applied to a container
/// via its ``style_id`` parameter.
///
/// Parameters
/// ----------
/// background_color : IpgColor, Optional
///     Sets the background color using a predefined color variant.
/// background_rgba : list of float, Optional
///     Sets the background color in rgba format as [r, g, b, a].
/// background_alpha : float, Optional
///     Sets the alpha transparency for the background color.
/// background_gradient_color_stop : IpgColor, Optional
///     Sets the stop color of the background gradient.
/// background_gradient_rgba_stop : list of float, Optional
///     Sets the stop color of the background gradient in rgba format as [r, g, b, a].
/// background_gradient_degrees : float, Optional
///     Sets the background gradient angle in degrees.
/// background_gradient_radians : float, Optional
///     Sets the background gradient angle in radians.
/// background_gradient_alpha : float, Optional
///     Sets the alpha transparency for the gradient stop color.
/// border_color : IpgColor, Optional
///     Sets the border color using a predefined color variant.
/// border_rgba : list of float, Optional
///     Sets the border color in rgba format as [r, g, b, a].
/// border_alpha : float, Optional
///     Sets the alpha transparency for the border color.
/// border_radius : list of float, Optional
///     Sets the radius of the corners as [all] or
///     [top-left, top-right, bottom-right, bottom-left].
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// shadow_color : IpgColor, Optional
///     Sets the shadow color using a predefined color variant.
/// shadow_rgba : list of float, Optional
///     Sets the shadow color in rgba format as [r, g, b, a].
/// shadow_alpha : float, Optional
///     Sets the alpha transparency for the shadow color.
/// shadow_offset_xy : list of float, Optional
///     Sets the shadow offset as [x, y] in logical pixels.
/// shadow_blur_radius : float, Optional
///     Sets the shadow blur radius in logical pixels.
/// text_color : IpgColor, Optional
///     Sets the text color using a predefined color variant.
/// text_rgba : list of float, Optional
///     Sets the text color in rgba format as [r, g, b, a].
/// text_alpha : float, Optional
///     Sets the alpha transparency for the text color.
/// snap : bool, Optional
///     Whether to snap the container to the pixel grid.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a container's ``style_id``.
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
