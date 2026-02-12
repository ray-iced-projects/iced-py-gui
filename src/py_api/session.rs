//! Session module - provides start_session and generate_id pyfunctions

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::app::App;
use crate::state::access_state;

/// Start the Iced GUI session.
/// 
/// This should be called after all windows and widgets have been added.
/// It will block until all windows are closed.
#[pyfunction]
pub fn start_session() -> PyResult<()> {
    let _ = iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .font(include_bytes!("../fonts/Roboto.ttf"))
        .scale_factor(App::scale_factor)
        .run();
    
    Ok(())
}

/// Generate a unique ID for later use.
/// 
/// This can be used when you need to reference a widget before creating it.
#[pyfunction]
pub fn generate_id() -> PyResult<usize> {
    let mut state = access_state();
    state.last_id += 1;
    let id = state.last_id;
    state.gen_ids.push(id);
    drop(state);
    Ok(id)
}
