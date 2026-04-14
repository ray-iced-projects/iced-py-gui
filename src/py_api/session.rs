//! Session module - provides start_session and generate_id pyfunctions

use std::panic;

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
    // Install a custom panic hook so errors stand out in the console
    panic::set_hook(Box::new(|info| {
        let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        let location = if let Some(loc) = info.location() {
            format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
        } else {
            "unknown location".to_string()
        };

        eprintln!("\n{}", "=".repeat(60));
        eprintln!("  IcedPyGui ERROR");
        eprintln!("{}", "=".repeat(60));
        eprintln!("  {msg}");
        eprintln!("  at {location}");
        eprintln!("{}\n", "=".repeat(60));
    }));

    let _ = iced::daemon(App::new, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .scale_factor(App::scale_factor)
        .title(App::title)
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
