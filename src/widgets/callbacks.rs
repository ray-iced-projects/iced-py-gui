//! callbacks
use crate::state::{access_callbacks, access_user_data2, USERDATA1};

use pyo3::{Py, PyAny, Python};
use pyo3::conversion::IntoPyObject;

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;

/// Invoke a widget callback with no additional arguments (like button's on_press).
/// 
/// The Python callback receives: `(id,)` or `(id, user_data)` if user_data is set.
pub fn invoke_callback(id: usize, event_name: &str, widget_name: &str) {
    let app_cbs = access_callbacks();
    
    let callback = match app_cbs.get(id, event_name) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };
    
    drop(app_cbs);
    
    let user_data_opt = get_user_data(id);
    let has_user_data = user_data_opt.is_some();

    Python::attach(|py| {
        let result = if let Some(user_data) = user_data_opt {
            callback.call1(py, (id, user_data))
        } else {
            callback.call1(py, (id,))
        };
        
        if let Err(err) = result {
            let hint = if has_user_data {
                format!("def callback(wid: int, user_data)")
            } else {
                format!("def callback(wid: int)")
            };
            panic!("{widget_name} '{event_name}' callback error: {err}\n  Expected signature: {hint}");
        }
    });
}

/// Invoke a widget callback with additional widget-specific arguments.
/// 
/// The Python callback receives: `(id, args)` or `(id, args, user_data)` if user_data is set.
/// Note: `args` is passed as a single argument. If it's a tuple like `(index, value)`,
/// the Python callback receives it as one tuple, not as separate arguments.
///
/// The `sig_hint` parameter describes the expected Python callback signature,
/// shown in error messages to help the user fix mismatches.
/// 
/// # Examples
/// - Checkbox: `invoke_callback_with_args(id, "on_toggle", "Checkbox", is_checked, "def cb(wid: int, is_checked: bool)")`
/// - Divider: `invoke_callback_with_args(id, "on_change", "Divider", (index, value), "def cb(wid: int, data: tuple[int, float])")`
pub fn invoke_callback_with_args<A>(id: usize, event_name: &str, widget_name: &str, args: A, sig_hint: &str)
where
    A: for<'py> IntoPyObject<'py> + Clone + Send,
{
    let app_cbs = access_callbacks();
    
    let callback = match app_cbs.get(id, event_name) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };
    
    drop(app_cbs);
    
    let user_data_opt = get_user_data(id);
    let has_user_data = user_data_opt.is_some();

    Python::attach(|py| {
        let result = if let Some(user_data) = user_data_opt {
            callback.call1(py, (id, args.clone(), user_data))
        } else {
            callback.call1(py, (id, args))
        };
        
        if let Err(err) = result {
            let hint = if has_user_data {
                format!("{sig_hint}, user_data)")
            } else {
                sig_hint.to_string()
            };
            panic!("{widget_name} '{event_name}' callback error: {err}\n  Expected signature: {hint}");
        }
    });
}

pub fn invoke_callback_with_two_args<A, B>(id: usize, event_name: &str, widget_name: &str, arg1: A, arg2: B, sig_hint: &str)
where
    A: for<'py> IntoPyObject<'py> + Clone + Send,
    B: for<'py> IntoPyObject<'py> + Clone + Send,
{
    let app_cbs = access_callbacks();
    
    let callback = match app_cbs.get(id, event_name) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };
    
    drop(app_cbs);
    
    let user_data_opt = get_user_data(id);
    let has_user_data = user_data_opt.is_some();

    Python::attach(|py| {
        let result = if let Some(user_data) = user_data_opt {
            callback.call1(py, (id, arg1.clone(), arg2.clone(), user_data))
        } else {
            callback.call1(py, (id, arg1, arg2))
        };
        
        if let Err(err) = result {
            let hint = if has_user_data {
                format!("{sig_hint}, user_data)")
            } else {
                sig_hint.to_string()
            };
            panic!("{widget_name} '{event_name}' callback error: {err}\n  Expected signature: {hint}");
        }
    });
}

/// Get user data for a widget, trying USERDATA1 first with fallback to USERDATA2.
fn get_user_data(id: usize) -> Option<PyObject> {
    let lock1 = USERDATA1.try_lock();
    if let Ok(ref ud1) = lock1 {
        let opt = ud1.get(id).map(|ud| Python::attach(|py| ud.clone_ref(py)));
        drop(lock1);
        opt
    } else {
        let ud2 = access_user_data2();
        let opt = ud2.get(id).map(|ud| Python::attach(|py| ud.clone_ref(py)));
        drop(ud2);
        opt
    }
}
