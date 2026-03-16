//! Window module - provides add_window pyfunction
use iced::Size;
use pyo3::{Py, PyAny, PyResult, pyfunction};

use crate::state::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex, IpgIds, IpgContainers};
use crate::widgets::ipg_window::{
    IpgWindow, IpgWindowLevel, IpgWindowTheme,
};

type PyObject = Py<PyAny>;

/// Add a window to the application.
///
/// This must be called before adding any widgets.
///
/// Parameters
/// ----------
/// window_id : str
///     Unique string identifier for this window.
/// title : str, optional
///     Title displayed in the window title bar.
/// size : tuple of (float, float), optional
///     Initial window size as ``(width, height)`` in logical pixels.
/// maximized : bool, optional
///     Whether the window starts maximized.
/// fullscreen : bool, optional
///     Whether the window starts in fullscreen mode.
/// center : bool, optional
///     Whether to centre the window on screen at startup.
/// position : tuple of (float, float), optional
///     Initial window position as ``(x, y)`` in logical pixels.
/// min_size : tuple of (float, float), optional
///     Minimum window size as ``(width, height)``.
/// max_size : tuple of (float, float), optional
///     Maximum window size as ``(width, height)``.
/// theme : IpgWindowTheme, optional
///     The colour theme for the window.
/// visible : bool, optional
///     Whether the window is visible on creation.
/// resizable : bool, optional
///     Whether the user can resize the window.
/// minimizable : bool, optional
///     Whether the user can minimise the window.
/// closeable : bool, optional
///     Whether the user can close the window.
/// decorations : bool, optional
///     Whether to show window decorations (title bar, borders).
/// transparent : bool, optional
///     Whether the window background is transparent.
/// blur : bool, optional
///     Whether to blur the window background.
/// level : IpgWindowLevel, optional
///     The stacking level of the window (normal, always-on-top, etc.).
/// icon_rgba : list of int, optional
///     Raw RGBA bytes for the window icon.
/// icon_width_height : list of [int, int], optional
///     Dimensions ``[width, height]`` of the icon in *icon_rgba*.
/// exit_on_close_request : bool, optional
///     Whether the application exits when this window is closed.
/// scale_factor : float, optional
///     DPI scale factor override for this window.
/// debug : bool, optional
///     Whether to enable the debug overlay for this window.
/// on_resize : callable, optional
///     Callback invoked when the window is resized.
/// user_data : Any, optional
///     Arbitrary data forwarded to callbacks.
/// gen_id : int, optional
///     Pre-generated numeric ID.  One is created automatically if omitted.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created window.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    title=None, 
    size=None, 
    maximized=None,
    fullscreen=None,
    center=None,
    position=None,
    min_size=None,
    max_size=None,
    theme=None,
    visible=None,
    resizable=None,
    minimizable=None,
    closeable=None,
    decorations=None,
    transparent=None,
    blur=None,
    level=None,
    icon_rgba=None,
    icon_width_height=None,
    exit_on_close_request=None,
    scale_factor=None,
    debug=None,
    on_resize=None,
    user_data=None,
    gen_id=None
))]
pub fn add_window(
    window_id: String,
    title: Option<String>,
    size: Option<(f32, f32)>,
    maximized: Option<bool>,
    fullscreen: Option<bool>,
    center: Option<bool>,
    position: Option<(f32, f32)>,
    min_size: Option<(f32, f32)>,
    max_size: Option<(f32, f32)>,
    theme: Option<IpgWindowTheme>,
    visible: Option<bool>,
    resizable: Option<bool>,
    minimizable: Option<bool>,
    closeable: Option<bool>,
    decorations: Option<bool>,
    transparent: Option<bool>,
    blur: Option<bool>,
    level: Option<IpgWindowLevel>,
    icon_rgba: Option<Vec<u8>>,
    icon_width_height: Option<[u32; 2]>,
    exit_on_close_request: Option<bool>,
    scale_factor: Option<f32>,
    debug: Option<bool>,
    on_resize: Option<PyObject>,
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

    let size = if let Some(wh) = size {
        Some(Size::new(wh.0, wh.1))
    } else { None };

    let min_size = if let Some(wh) = min_size {
        Some(Size::new(wh.0, wh.1))
    } else { None };

    let max_size = if let Some(wh) = max_size {
        Some(Size::new(wh.0, wh.1))
    } else { None };

    // Check for duplicate window IDs
    if state.windows_str_ids.get(&window_id).is_some() {
        panic!("Window id {} is not unique", window_id);
    }

    // Store window string -> usize mapping
    state.windows_str_ids.insert(window_id.clone(), id);

    // Register the window as a container so widgets can find their parent
    // The window's container_id maps to its own window_id
    state.container_wnd_str_ids.insert(window_id.clone(), window_id.clone());
    state.container_str_ids.insert(window_id.clone(), id);

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
    let window = 
        IpgWindow {
        id,
        title,
        size,
        maximized,
        fullscreen,
        center,
        position,
        min_size,
        max_size,
        theme,
        visible,
        resizable,
        minimizable,
        closeable,
        decorations,
        transparent,
        blur,
        level,
        icon_rgba,
        icon_width_height,
        exit_on_close_request,
        scale_factor,
        debug,
    };

    // Store in containers
    state.containers.insert(id, IpgContainers::IpgWindow(window.clone()));

    // Store in windows vec
    state.windows.push(window);

    drop(state);

    // Handle callbacks and user data outside of state lock
    if let Some(py) = on_resize {
        add_callback_to_mutex(id, "on_resize".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    Ok(id)
}
