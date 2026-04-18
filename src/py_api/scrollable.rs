//! Scrollable module - Provides add_scrollable, add_scrollable_style,
//!                     add_scrollbar, add_rail_style, add_autoscroll_style

use pyo3::{Py, PyAny, PyResult, pyfunction};

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::Color,
    state::{Containers, Widgets, get_id, set_state_cont_wnd_ids, 
        set_state_of_container}, widgets::{ipg_container::ContainerStyleStd, ipg_scrollable::{
            AutoScrollStyle, RailStyle, Scrollable, ScrollableStyle, Scroller}}};
type PyObject = Py<PyAny>;


/// Add a scrollable container widget.
///
/// A scrollable container that can scroll its children
/// vertically, horizontally, or both.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this scrollable belongs to.
/// container_id : str
///     Sets the Unique string identifier for the scrollable.
/// parent_id : str, Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the scrollable fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the scrollable fills available height.
/// fill : bool, Optional
///     Whether to fill both the available width and height
/// both_scrollers : bool, Optional
///     Whether to show both horizontal and vertical scrollers.
/// scroller_x_id : int, Optional
///     Sets the ID of the horizontal scroller parameters.
/// scroller_y_id : int, Optional
///     Sets the ID of the vertical scroller parameters.
/// on_scroll : callable, Optional
///     Sets the Callback method to invoke when the scrollable is scrolled.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_scrollable_style``.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created scrollable.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    width=None,
    width_fill=None, 
    height=None, 
    height_fill=None,
    fill=None,
    scroller_x_id=None,
    scroller_y_id=None,
    on_scroll=None, 
    user_data=None,
    style_id=None,
    ))]
pub fn add_scrollable(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    scroller_x_id: Option<usize>,
    scroller_y_id: Option<usize>,
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

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_scrollable".to_string());
    
    state.containers.insert(id, Containers::Scrollable(
        Scrollable { 
            id,
            width,
            width_fill,
            height,
            height_fill,
            fill,
            scroller_x_id,
            scroller_y_id,
            style_id,
        }));

    drop(state);
    Ok(id)

}

/// Add styling to a scrollable.
///
/// Creates a custom style that can be applied to a scrollable
/// via its ``style_id`` parameter.
///
/// Parameters
/// ----------
/// container_style_id : int, Optional
///     Sets the ID of a container style for the scrollable background.
/// container_style_std : ContainerStyleStd, Optional
///     Sets the predefined standard container style variant.
/// vertical_rail_style_id : int, Optional
///     Sets the ID of a rail style for the vertical scrollbar.
/// horizontal_rail_style_id : int, Optional
///     Sets the ID of a rail style for the horizontal scrollbar.
/// auto_scroll_style_id : int, Optional
///     Sets the ID of an autoscroll style.
/// gap_color : Color, Optional
///     Sets the gap color using a predefined color variant.
/// gap_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// gap_rgba : list of float, Optional
///     Sets the gap color in rgba format as [r, g, b, a].
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a scrollable's ``style_id``.
#[pyfunction]
#[pyo3(signature = ( 
    container_style_id=None,
    container_style_std=None,
    vertical_rail_style_id=None,
    horizontal_rail_style_id=None,
    auto_scroll_style_id=None,
    gap_color=None,
    gap_color_alpha=None,
    gap_rgba=None,
    gen_id=None
    ))]
pub fn add_scrollable_style(
    container_style_id: Option<usize>,
    container_style_std: Option<ContainerStyleStd>,
    vertical_rail_style_id: Option<usize>,
    horizontal_rail_style_id: Option<usize>,
    auto_scroll_style_id: Option<usize>,
    gap_color: Option<Color>,
    gap_color_alpha: Option<f32>,
    gap_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::ScrollableStyle(
        ScrollableStyle {
            container_style_id,
            container_style_std,
            vertical_rail_style_id,
            horizontal_rail_style_id,
            auto_scroll_style_id,
            gap_color,
            gap_color_alpha,
            gap_rgba,
        }));

    drop(state);
    Ok(id)
}

/// Add scroller parameters.
///
/// Creates scroller parameters that can be assigned to a
/// scrollable's ``scroller_x_id`` or ``scroller_y_id``.
///
/// Parameters
/// ----------
/// width : float, Optional
///     Sets the width of the scrollbar track.
/// margin : float, Optional
///     Sets the margin around the scrollbar.
/// scroller_width : float, Optional
///     Sets the width of the scroller thumb.
/// spacing : float, Optional
///     Sets the spacing between the scrollbar and content.
/// anchor : Anchor, Optional
///     Sets the anchor position of the scrollbar.
/// hidden : bool, Optional
///     Whether the scrollbar is hidden.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric ID to pass to a scrollable's scroller parameter.
#[pyfunction]
#[pyo3(signature = ( 
    width=None,
    margin=None,
    scroller_width=None,
    spacing=None,
    anchor_start=None,
    anchor_end=None,
    hidden=None,
    gen_id=None,
    ))]
pub fn add_scroller (
    width: Option<f32>,
    margin: Option<f32>,
    scroller_width: Option<f32>,
    spacing: Option<f32>,
    anchor_start: Option<bool>,
    anchor_end: Option<bool>,
    hidden: Option<bool>,
    gen_id: Option<usize>,
    )-> PyResult<usize>
{

    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::Scroller (
        Scroller {
        id,
        width,
        margin,
        scroller_width,
        spacing,
        anchor_start,
        anchor_end,
        hidden,
    }));

    drop(state);
    Ok(id)
}


/// Add styling to a scrollbar rail.
///
/// Creates a custom rail style that can be applied to a
/// scrollable style's ``vertical_rail_style_id`` or
/// ``horizontal_rail_style_id``.
///
/// Parameters
/// ----------
/// background_color : Color, Optional
///     Sets the rail background color using a predefined color variant.
/// background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// background_rgba : list of float, Optional
///     Sets the rail background color in rgba format as [r, g, b, a].
/// border_color : Color, Optional
///     Sets the rail border color using a predefined color variant.
/// border_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// border_rgba : list of float, Optional
///     Sets the rail border color in rgba format as [r, g, b, a].
/// border_width : float, Optional
///     Sets the rail border width in logical pixels.
/// border_radius : list of float, Optional
///     Sets the radius of the rail corners as [all] or
///     [top-left, top-right, bottom-right, bottom-left].
/// scroller_background_color : Color, Optional
///     Sets the scroller thumb background color using a predefined color variant.
/// scroller_background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// scroller_background_rgba : list of float, Optional
///     Sets the scroller thumb background color in rgba format as [r, g, b, a].
/// scroller_border_color : Color, Optional
///     Sets the scroller thumb border color using a predefined color variant.
/// scroller_border_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// scroller_border_rgba : list of float, Optional
///     Sets the scroller thumb border color in rgba format as [r, g, b, a].
/// scroller_border_width : float, Optional
///     Sets the scroller thumb border width in logical pixels.
/// scroller_border_radius : list of float, Optional
///     Sets the radius of the scroller thumb corners as [all] or
///     [top-left, top-right, bottom-right, bottom-left].
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a scrollable style's rail parameter.
#[pyfunction]
#[pyo3(signature = ( 
    background_color=None,
    background_color_alpha=None,
    background_rgba=None,
    border_color=None,
    border_color_alpha=None,
    border_rgba=None,
    border_width=None,
    border_radius=None,
    scroller_background_color=None,
    scroller_background_color_alpha=None,
    scroller_background_rgba=None,
    scroller_border_color=None,
    scroller_border_color_alpha=None,
    scroller_border_rgba=None,
    scroller_border_width=None,
    scroller_border_radius=None,
    gen_id=None
    ))]
pub fn add_rail_style(
    background_color: Option<Color>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_width: Option<f32>,
    border_radius: Option<Vec<f32>>,
    scroller_background_color: Option<Color>,
    scroller_background_color_alpha: Option<f32>,
    scroller_background_rgba: Option<[f32; 4]>,
    scroller_border_color: Option<Color>,
    scroller_border_color_alpha: Option<f32>,
    scroller_border_rgba: Option<[f32; 4]>,
    scroller_border_width: Option<f32>,
    scroller_border_radius: Option<Vec<f32>>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::RailStyle(
        RailStyle { 
            id,
            background_color,
            background_color_alpha,
            background_rgba,
            border_color,
            border_color_alpha,
            border_rgba,
            border_width,
            border_radius,
            scroller_background_color,
            scroller_background_color_alpha,
            scroller_background_rgba,
            scroller_border_color,
            scroller_border_color_alpha,
            scroller_border_rgba,
            scroller_border_width,
            scroller_border_radius,
        }));

    drop(state);
    Ok(id)

}

/// Add styling to an autoscroll indicator.\n///
/// Creates a custom autoscroll style that can be applied to a
/// scrollable style's ``auto_scroll_style_id``.
///
/// Parameters
/// ----------
/// background_color : Color, Optional
///     Sets the background color using a predefined color variant.
/// background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// background_rgba : list of float, Optional
///     Sets the background color in rgba format as [r, g, b, a].
/// border_color : Color, Optional
///     Sets the border color using a predefined color variant.
/// border_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// border_rgba : list of float, Optional
///     Sets the border color in rgba format as [r, g, b, a].
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// border_radius : list of float, Optional
///     Sets the radius of the corners as [all] or
///     [top-left, top-right, bottom-right, bottom-left].
/// shadow_color : Color, Optional
///     Sets the shadow color using a predefined color variant.
/// shadow_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// shadow_rgba : list of float, Optional
///     Sets the shadow color in rgba format as [r, g, b, a].
/// shadow_offset : list of float, Optional
///     Sets the shadow offset as [x, y] in logical pixels.
/// shadow_blur_radius : float, Optional
///     Sets the shadow blur radius in logical pixels.
/// shadow_icon_color : Color, Optional
///     Sets the shadow icon color using a predefined color variant.
/// shadow_icon_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// shadow_icon_rgba : list of float, Optional
///     Sets the shadow icon color in rgba format as [r, g, b, a].
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a scrollable style's ``auto_scroll_style_id``.
#[pyfunction]
#[pyo3(signature = ( 
    background_color=None,
    background_color_alpha=None,
    background_rgba=None,
    border_color=None,
    border_color_alpha=None,
    border_rgba=None,
    border_width=None,
    border_radius=None,
    shadow_color=None,
    shadow_color_alpha=None,
    shadow_rgba=None,
    shadow_offset=None,
    shadow_blur_radius=None,
    shadow_icon_color=None,
    shadow_icon_color_alpha=None,
    shadow_icon_rgba=None,
    gen_id=None
    ))]
pub fn add_autoscroll_style(
    background_color: Option<Color>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_width: Option<f32>,
    border_radius: Option<Vec<f32>>,
    shadow_color: Option<Color>,
    shadow_color_alpha: Option<f32>,
    shadow_rgba: Option<[f32; 4]>,
    shadow_offset: Option<[f32; 2]>,
    shadow_blur_radius: Option<f32>,
    shadow_icon_color: Option<Color>,
    shadow_icon_color_alpha: Option<f32>,
    shadow_icon_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::AutoScrollStyle(
        AutoScrollStyle { 
            id,
            background_color,
            background_color_alpha,
            background_rgba,
            border_color,
            border_color_alpha,
            border_rgba,
            border_width,
            border_radius,
            shadow_color,
            shadow_color_alpha,
            shadow_rgba,
            shadow_offset,
            shadow_blur_radius,
            shadow_icon_color,
            shadow_icon_color_alpha,
            shadow_icon_rgba,
        }));

    drop(state);
    Ok(id)

}

