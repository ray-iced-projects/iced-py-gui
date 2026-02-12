//! Window module - provides add_window pyfunction

use iced::window::Position;
use iced::{Point, Size};
use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};

use crate::state::{access_state, add_callback, add_user_data, IpgIds, IpgContainers};
use crate::widgets::window::{
    IpgWindow, IpgWindowLevel, IpgWindowMode, IpgWindowTheme,
};

type PyObject = Py<PyAny>;

/// Add a window to the application.
/// 
/// This must be called before adding any widgets.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    title, 
    width, 
    height,
    max_width=None, 
    max_height=None,
    min_width=None, 
    min_height=None,
    pos_x=None, 
    pos_y=None,
    pos_centered=false, 
    resizable=true,
    decorations=true, 
    transparent=false,
    level=IpgWindowLevel::Normal,
    scale_factor=1.0,
    theme=IpgWindowTheme::Dark, 
    exit_on_close=false, 
    on_resize=None, 
    mode=IpgWindowMode::Windowed, 
    debug=false, 
    user_data=None,
    gen_id=None
))]
pub fn add_window(
    window_id: String,
    title: String,
    width: f32,
    height: f32,
    max_width: Option<f32>,
    max_height: Option<f32>,
    min_width: Option<f32>,
    min_height: Option<f32>,
    pos_x: Option<f32>,
    pos_y: Option<f32>,
    pos_centered: bool,
    resizable: bool,
    decorations: bool,
    transparent: bool,
    level: IpgWindowLevel,
    scale_factor: f64,
    theme: IpgWindowTheme,
    exit_on_close: bool,
    on_resize: Option<PyObject>,
    mode: IpgWindowMode,
    debug: bool,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
) -> PyResult<usize> {
    let mut state = access_state();
    
    // Get or generate ID
    let id = match gen_id {
        Some(gid) => gid,
        None => {
            state.last_id += 1;
            state.last_id
        }
    };

    // Build window position
    let mut window_position = Position::Default;
    let size = Size::new(width, height);

    let mut max_size = Size::INFINITY;
    if let Some(mw) = max_width {
        max_size.width = mw;
    }
    if let Some(mh) = max_height {
        max_size.height = mh;
    }

    let mut min_size = Size::ZERO;
    if let Some(mw) = min_width {
        min_size.width = mw;
    }
    if let Some(mh) = min_height {
        min_size.height = mh;
    }

    if pos_x.is_some() && pos_y.is_some() {
        let px = pos_x.unwrap_or(0.0);
        let py = pos_y.unwrap_or(0.0);
        window_position = Position::Specific(Point { x: px, y: py });
    }

    if pos_centered {
        window_position = Position::Centered;
    }

    let iced_theme = theme.to_iced();

    // Check for duplicate window IDs
    if state.windows_str_ids.get(&window_id).is_some() {
        panic!("Window id {} is not unique", window_id);
    }

    // Store window string -> usize mapping
    state.windows_str_ids.insert(window_id.clone(), id);

    // Initialize the IpgIds for this window
    state.ids.insert(
        id,
        vec![IpgIds {
            id,
            parent_uid: 0,
            container_id: Some(window_id.clone()),
            parent_id: "".to_string(),
            is_container: true,
        }],
    );

    // Initialize container tracking for this window
    state.container_ids.insert(id, vec![id]);

    // Create the window object
    let window = IpgWindow::new(
        id,
        title.clone(),
        size,
        Some(min_size),
        Some(max_size),
        window_position,
        exit_on_close,
        iced_theme.clone(),
        resizable,
        mode.clone(),
        decorations,
        transparent,
        level.clone(),
        scale_factor,
        debug,
    );

    // Store in containers
    state.containers.insert(id, IpgContainers::IpgWindow(window.clone()));

    // Store in windows vec
    state.windows.push(window);

    drop(state);

    // Handle callbacks and user data outside of state lock
    if let Some(py) = on_resize {
        add_callback(id, "on_resize".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data(id, py);
    }

    Ok(id)
}
