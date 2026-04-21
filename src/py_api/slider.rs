//! Provides Slider py function add_slidder

use pyo3::{pyfunction, Py, PyAny, PyResult};

use crate::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex, graphics::colors::Color, 
    state::{Widgets, 
        get_id, set_state_of_widget}, 
        widgets::ipg_slider::{Slider, SliderStyle}};
type PyObject = Py<PyAny>;


/// Add a slider widget.
///
/// A horizontal slider for selecting a numeric value within a range.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this slider belongs to.
/// min : float
///     Sets the minimum value of the slider.
/// max : float
///     Sets the maximum value of the slider.
/// step : float
///     Sets the step increment of the slider.
/// value : float
///     Sets the current value of the slider.
/// shift_step : float, Optional
///     Sets the step increment when shift is held.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// height : float, Optional
///     Sets the height of the slider track in logical pixels.
/// width_fill : bool, default False
///     Whether the slider fills available width.
/// on_change : callable, Optional
///     Sets the Callback method to invoke when the slider value changes.
/// on_release : callable, Optional
///     Sets the Callback method to invoke when the slider handle is released.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_slider_style``.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the slider is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created slider.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    min, 
    max, 
    step, 
    value,
    shift_step=None, 
    width=None,
    width_fill=None, 
    height=None,  
    on_change=None, 
    on_release=None, 
    style_id=None,
    user_data=None,
    show=true,
    gen_id=None,
    ))]
pub fn add_slider(
    parent_id: String,
    min: f32,
    max: f32,
    step: f32,
    value: f32,
    shift_step: Option<f32>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    on_change: Option<PyObject>,
    on_release: Option<PyObject>,
    style_id: Option<usize>,
    user_data: Option<PyObject>,
    show: bool,
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
    {

    let id = get_id(gen_id);

    if let Some(py) = on_change {
        add_callback_to_mutex(id, "on_change".to_string(), py);
    }
    if let Some(py) = on_release {
        add_callback_to_mutex(id, "on_release".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::Slider(
        Slider { 
            id,
            show,
            min,
            max,
            step,
            shift_step,
            value,
            width,
            width_fill,
            height,
            style_id,
        }));

    drop(state);
    Ok(id)

}

/// Add styling to a slider.
///
/// Creates a custom style that can be applied to a slider
/// via its ``style_id`` parameter.
///
/// Parameters
/// ----------
/// rail_color : Color, Optional
///     Sets the rail color using a predefined color variant.
/// rail_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// rail_rgba : list of float, Optional
///     Sets the rail color in rgba format as [r, g, b, a].
/// rail_color_hovered : Color, Optional
///     Sets the rail color when hovered using a predefined color variant.
/// rail_color_hovered_alpha : float, Optional
///     Sets the alpha of the Color.
/// rail_rgba_hovered : list of float, Optional
///     Sets the rail color when hovered in rgba format as [r, g, b, a].
/// rail_width : float, Optional
///     Sets the rail width in logical pixels.
/// rail_border_radius : list of float, Optional
///     Sets the radius of the rail corners as [all] or
///     [top-left, top-right, bottom-right, bottom-left].
/// handle_circle_radius : float, Optional
///     Sets the radius of a circular handle.
/// handle_rectangle_width : int, Optional
///     Sets the width of a rectangular handle.
/// handle_rectangle_border_radius : list of float, Optional
///     Sets the radius of the rectangular handle corners.
/// handle_color : Color, Optional
///     Sets the handle color using a predefined color variant.
/// handle_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// handle_rgba : list of float, Optional
///     Sets the handle color in rgba format as [r, g, b, a].
/// handle_border_width : float, Optional
///     Sets the handle border width in logical pixels.
/// handle_border_color : Color, Optional
///     Sets the handle border color using a predefined color variant.
/// handle_border_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// handle_border_rgba : list of float, Optional
///     Sets the handle border color in rgba format as [r, g, b, a].
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a slider's ``style_id``.
#[pyfunction]
#[pyo3(signature = (
    rail_color=None,
    rail_color_alpha=None,
    rail_rgba=None,
    rail_color_hovered=None,
    rail_color_hovered_alpha=None,
    rail_rgba_hovered=None,
    rail_width=None,
    rail_border_radius=None,
    handle_circle_radius=None,
    handle_rectangle_width=None,
    handle_rectangle_border_radius=None,
    handle_color=None,
    handle_color_alpha=None,
    handle_rgba=None,
    handle_border_width=None,
    handle_border_color=None,
    handle_border_color_alpha=None,
    handle_border_rgba=None,
    gen_id=None,
    ))]
pub fn add_slider_style(
    rail_color: Option<Color>,
    rail_color_alpha: Option<f32>,
    rail_rgba: Option<[f32; 4]>,
    rail_color_hovered: Option<Color>,
    rail_color_hovered_alpha: Option<f32>,
    rail_rgba_hovered: Option<[f32; 4]>,
    rail_width: Option<f32>,
    rail_border_radius: Option<Vec<f32>>,
    handle_circle_radius: Option<f32>,
    handle_rectangle_width: Option<u16>,
    handle_rectangle_border_radius: Option<Vec<f32>>,
    handle_color: Option<Color>,
    handle_color_alpha: Option<f32>,
    handle_rgba: Option<[f32; 4]>,
    handle_border_width: Option<f32>,
    handle_border_color: Option<Color>,
    handle_border_color_alpha: Option<f32>,
    handle_border_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    )  -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();
    
    state.widgets.insert(id, Widgets::SliderStyle(
        SliderStyle {
            id,
            rail_color,
            rail_color_alpha,
            rail_rgba,
            rail_color_hovered,
            rail_color_hovered_alpha,
            rail_rgba_hovered,
            rail_width,
            rail_border_radius,
            handle_circle_radius,
            handle_rectangle_width,
            handle_rectangle_border_radius,
            handle_color,
            handle_color_alpha,
            handle_rgba,
            handle_border_width,
            handle_border_color,
            handle_border_color_alpha,
            handle_border_rgba,
        }));

    drop(state);
    Ok(id)

}
