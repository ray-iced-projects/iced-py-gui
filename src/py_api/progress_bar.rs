//! ProgressBar module - provides add_progress_bar pyfunction
use pyo3::{pyfunction, PyResult};

use crate::access_state;
use crate::graphics::colors::Color;
use crate::py_api::helpers::get_length;
use crate::state::{Widgets, get_id, set_state_of_widget};
use crate::widgets::ipg_progress_bar::{ProgressBar, ProgressBarStyle};
use crate::widgets::styling::StyleStandard;




/// Add a progress bar widget.
///
/// A horizontal or vertical bar that visually indicates progress
/// between a minimum and maximum value.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this progress bar belongs to.
/// min : float
///     Sets the minimum value of the progress bar.
/// max : float
///     Sets the maximum value of the progress bar.
/// value : float
///     Sets the current value of the progress bar.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// is_vertical : bool, Optional
///     Whether the progress bar is oriented vertically.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default True
///     Whether the progress bar fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the progress bar fills available height.
/// style_standard : StyleStandard, Optional
///     Sets the predefined standard style variant.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_progress_bar_style``.
/// show : bool, default True
///     Whether the progress bar is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created progress bar.
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
    gen_id: Option<usize>,
    is_vertical: Option<bool>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    style_standard: Option<StyleStandard>,
    style_id: Option<usize>,
    show: bool,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let width = get_length(width, width_fill);
    let height = get_length(height, height_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::ProgressBar(
        ProgressBar {   
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


/// Add styling to a progress bar.
///
/// Creates a custom style that can be applied to a progress bar
/// via its ``style_id`` parameter.
///
/// Parameters
/// ----------
/// background_color : Color, Optional
///     Sets the background color using a predefined color variant.
/// background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// background_rgba : list of float, Optional
///     Sets the background color in rgba format as [r, g, b, a].
/// bar_color : Color, Optional
///     Sets the bar fill color using a predefined color variant.
/// bar_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// bar_rgba : list of float, Optional
///     Sets the bar fill color in rgba format as [r, g, b, a].
/// border_color : Color, Optional
///     Sets the border color using a predefined color variant.
/// border_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// border_rgba : list of float, Optional
///     Sets the border color in rgba format as [r, g, b, a].
/// border_radius : list of float, Optional
///     Sets the radius of the corners as [all] or
///     [top-left, top-right, bottom-right, bottom-left].
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a progress bar's ``style_id``.
#[pyfunction]
#[pyo3(signature = (
    background_color=None,
    background_color_alpha=None,
    background_rgba=None,
    bar_color=None,
    bar_color_alpha=None,
    bar_rgba=None,
    border_color=None,
    border_color_alpha=None,
    border_rgba=None,
    border_radius=None, 
    border_width=None,
    gen_id=None
    ))]
pub fn add_progress_bar_style(
    background_color: Option<Color>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    bar_color: Option<Color>,
    bar_color_alpha: Option<f32>,
    bar_rgba: Option<[f32; 4]>,
    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color = 
        Color::rgba_ipg_color_to_iced(background_rgba, background_color, background_color_alpha);
    let bar_color = 
        Color::rgba_ipg_color_to_iced(bar_rgba, bar_color, bar_color_alpha);
    let border_color = 
        Color::rgba_ipg_color_to_iced(border_rgba, border_color, border_color_alpha);

    let mut state = access_state();

        state.widgets.insert(id, Widgets::ProgressBarStyle(
        ProgressBarStyle { 
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
