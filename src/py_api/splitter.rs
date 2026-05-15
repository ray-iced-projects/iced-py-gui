//! Splitter module — provides add_splitter_h, add_splitter_v, add_splitter_style pyfunctions

use pyo3::{Py, PyAny, pyfunction, PyResult};

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex,
    graphics::colors::Color,
    state::{Containers, Widgets, get_id, set_state_of_container, set_state_cont_wnd_ids},
    widgets::ipg_splitter::{
        SplitterH, SplitterV, SplitterStyle, SplitterResizeTrigger,
    },
};
type PyObject = Py<PyAny>;


/// Add a horizontal splitter container.
///
/// A SplitterH lays N panels side by side with draggable vertical sash handles
/// between them. No external callback or Stack is required — resizing is
/// managed internally.
///
/// Parameters
/// ----------
/// window_id : str
///     Window this container belongs to.
/// container_id : str
///     Unique string identifier for the splitter.
/// sizes : list of float
///     Initial pixel width of each panel (left → right).
/// height : float
///     Fixed pixel height shared by all panels.
/// parent_id : str, Optional
///     Parent container. Defaults to the window.
/// min_size : float, default 20.0
///     Minimum pixel width of any panel.
/// max_size : float, Optional
///     Maximum total pixel width of all panels combined.  If the sum of
///     ``sizes`` exceeds this value the panels are scaled proportionally.
/// sash_size : float, default 8.0
///     Visual thickness of each drag handle in pixels.
/// on_resize : callable, Optional
///     Called on every drag tick with ``(wid, (panel_index, new_size, all_sizes))``.
/// on_release : callable, Optional
///     Called when the mouse button is released.
/// style_id : int, Optional
///     Style created with ``add_splitter_style``.
/// gen_id : int, Optional
///     Pre-reserved widget ID.
/// user_data : Any, Optional
///     Forwarded verbatim to every callback.
/// show : bool, default True
///     Whether the splitter is visible.
///
/// Returns
/// -------
/// int
///     Numeric widget ID.
#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    sizes,
    height,
    parent_id=None,
    min_size=20.0,
    max_size=None,
    sash_size=8.0,
    on_resize=None,
    on_release=None,
    style_id=None,
    gen_id=None,
    user_data=None,
    show=true,
))]
pub fn add_splitter_h(
    window_id: String,
    container_id: String,
    sizes: Vec<f32>,
    height: f32,
    parent_id: Option<String>,
    min_size: f32,
    max_size: Option<f32>,
    sash_size: f32,
    on_resize: Option<PyObject>,
    on_release: Option<PyObject>,
    style_id: Option<usize>,
    gen_id: Option<usize>,
    user_data: Option<PyObject>,
    show: bool,
) -> PyResult<usize>
{
    let id = get_id(gen_id);

    if let Some(py) = on_resize {
        add_callback_to_mutex(id, "on_resize".to_string(), py);
    }
    if let Some(py) = on_release {
        add_callback_to_mutex(id, "on_release".to_string(), py);
    }
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let prt_id = parent_id.unwrap_or_else(|| window_id.clone());

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_splitter_h".to_string());

    state.containers.insert(id, Containers::SplitterH(
        SplitterH {
            id,
            sizes,
            height,
            min_size,
            max_size,
            sash_size,
            on_resize_trigger: SplitterResizeTrigger::OnDrag,
            style_id,
            show,
            index_in_use: 0,
            value_in_use: 0.0,
        }
    ));

    drop(state);
    Ok(id)
}


/// Add a vertical splitter container.
///
/// A SplitterV stacks N panels from top to bottom with draggable horizontal
/// sash handles between them.
///
/// Parameters
/// ----------
/// window_id : str
///     Window this container belongs to.
/// container_id : str
///     Unique string identifier for the splitter.
/// sizes : list of float
///     Initial pixel height of each panel (top → bottom).
/// width : float
///     Fixed pixel width shared by all panels.
/// parent_id : str, Optional
///     Parent container. Defaults to the window.
/// min_size : float, default 20.0
///     Minimum pixel height of any panel.
/// max_size : float, Optional
///     Maximum total pixel height of all panels combined.  If the sum of
///     ``sizes`` exceeds this value the panels are scaled proportionally.
/// sash_size : float, default 8.0
///     Visual thickness of each drag handle in pixels.
/// on_resize : callable, Optional
///     Called on every drag tick with ``(wid, (panel_index, new_size, all_sizes))``.
/// on_release : callable, Optional
///     Called when the mouse button is released.
/// style_id : int, Optional
///     Style created with ``add_splitter_style``.
/// gen_id : int, Optional
///     Pre-reserved widget ID.
/// user_data : Any, Optional
///     Forwarded verbatim to every callback.
/// show : bool, default True
///     Whether the splitter is visible.
///
/// Returns
/// -------
/// int
///     Numeric widget ID.
#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    sizes,
    width,
    parent_id=None,
    min_size=20.0,
    max_size=None,
    sash_size=8.0,
    on_resize=None,
    on_release=None,
    style_id=None,
    gen_id=None,
    user_data=None,
    show=true,
))]
pub fn add_splitter_v(
    window_id: String,
    container_id: String,
    sizes: Vec<f32>,
    width: f32,
    parent_id: Option<String>,
    min_size: f32,
    max_size: Option<f32>,
    sash_size: f32,
    on_resize: Option<PyObject>,
    on_release: Option<PyObject>,
    style_id: Option<usize>,
    gen_id: Option<usize>,
    user_data: Option<PyObject>,
    show: bool,
) -> PyResult<usize>
{
    let id = get_id(gen_id);

    if let Some(py) = on_resize {
        add_callback_to_mutex(id, "on_resize".to_string(), py);
    }
    if let Some(py) = on_release {
        add_callback_to_mutex(id, "on_release".to_string(), py);
    }
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let prt_id = parent_id.unwrap_or_else(|| window_id.clone());

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_splitter_v".to_string());

    state.containers.insert(id, Containers::SplitterV(
        SplitterV {
            id,
            sizes,
            width,
            min_size,
            max_size,
            sash_size,
            on_resize_trigger: SplitterResizeTrigger::OnDrag,
            style_id,
            show,
            index_in_use: 0,
            value_in_use: 0.0,
        }
    ));

    drop(state);
    Ok(id)
}


/// Add styling for a SplitterH or SplitterV.
///
/// Parameters
/// ----------
/// background_color : Color, Optional
///     Handle background color.
/// background_color_alpha : float, Optional
///     Alpha of the background color.
/// background_rgba : list of float, Optional
///     Background color in ``[r, g, b, a]`` format (0.0–1.0 each).
/// background_color_hovered : Color, Optional
///     Handle color when the cursor hovers over it.
/// background_color_hovered_alpha : float, Optional
///     Alpha of the hovered color.
/// background_rgba_hovered : list of float, Optional
///     Hovered color in rgba format.
/// background_transparent : bool, Optional
///     Render handle as fully transparent.
/// border_color : Color, Optional
/// border_color_alpha : float, Optional
/// border_rgba : list of float, Optional
/// border_radius : list of float, Optional
///     Corner radii as ``[all]`` or ``[tl, tr, br, bl]``.
/// border_width : float, Optional
/// gen_id : int, Optional
///     Pre-reserved style ID.
///
/// Returns
/// -------
/// int
///     Numeric style ID to pass to a splitter's ``style_id``.
#[pyfunction]
#[pyo3(signature = (
    background_color=None,
    background_color_alpha=None,
    background_rgba=None,
    background_color_hovered=None,
    background_color_hovered_alpha=None,
    background_rgba_hovered=None,
    background_transparent=None,
    border_color=None,
    border_color_alpha=None,
    border_rgba=None,
    border_radius=None,
    border_width=None,
    gen_id=None,
))]
pub fn add_splitter_style(
    background_color: Option<Color>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    background_color_hovered: Option<Color>,
    background_color_hovered_alpha: Option<f32>,
    background_rgba_hovered: Option<[f32; 4]>,
    background_transparent: Option<bool>,
    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    gen_id: Option<usize>,
) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::SplitterStyle(
        SplitterStyle {
            id,
            background_color,
            background_color_alpha,
            background_rgba,
            background_color_hovered,
            background_color_hovered_alpha,
            background_rgba_hovered,
            background_transparent,
            border_color,
            border_color_alpha,
            border_rgba,
            border_radius,
            border_width,
        }
    ));

    drop(state);
    Ok(id)
}
