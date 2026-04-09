//! Svg module - provides add_svg pyfunction

use pyo3::{pyfunction, PyResult};

use crate::graphics::colors::Color;
use crate::state::{Widgets, get_id, set_state_of_widget};
use crate::widgets::enums::{ContentFit, Rotation};
use crate::widgets::ipg_svg::Svg;
use crate::access_state; 
use crate::py_api::helpers::get_length;


/// Add an SVG widget.
///
/// Displays a scalable vector graphic from a file path with
/// optional mouse interaction.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this SVG belongs to.
/// svg_path : str
///     Sets the file path to the SVG image.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the SVG fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the SVG fills available height.
/// ipg_color_filter : Color, Optional
///     Sets the color filter using a predefined color variant.
/// ipg_color_filter_alpha : float, Optional
///     Sets the alpha of the Color.
/// rgba_filter : list of float, Optional
///     Sets the color filter in rgba format as [r, g, b, a].
/// content_fit : ContentFit, Optional
///     Sets the content fit strategy for the SVG.
/// rotation_type : Rotation, Optional
///     Sets the rotation method for the SVG.
/// rotation_radians : float, Optional
///     Sets the rotation angle in radians.
/// opacity : float, Optional
///     Sets the opacity of the SVG (0.0 to 1.0).
/// show : bool, default True
///     Whether the SVG is visible.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created SVG.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    svg_path, 
    width=None, 
    width_fill=false, 
    height=None, 
    height_fill=false,
    ipg_color_filter=None,
    ipg_color_filter_alpha=None,
    rgba_filter=None,
    content_fit=None,
    rotation_type=None,
    rotation_radians=None, 
    opacity=None,
    show=true,
    gen_id=None,
    ))]
pub fn add_svg(
    parent_id: String,
    svg_path: String,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    ipg_color_filter: Option<Color>,
    ipg_color_filter_alpha: Option<f32>,
    rgba_filter: Option<[f32; 4]>,
    content_fit: Option<ContentFit>,
    rotation_type: Option<Rotation>,
    rotation_radians: Option<f32>,
    opacity: Option<f32>,
    show: bool,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let color_filter = 
        Color::rgba_ipg_color_to_iced(rgba_filter, &ipg_color_filter, ipg_color_filter_alpha);

    let width = get_length(width, width_fill);
    let height = get_length(height, height_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::Svg(
        Svg {
            id,
            svg_path,
            width,
            height,
            color_filter,
            content_fit,
            rotation_type,
            rotation_radians,
            opacity,
            show,
        }));

    drop(state);
    Ok(id)

}