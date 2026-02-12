//! Minimal lib.rs - prototype for modular widget architecture
//!
//! This demonstrates using module-level pyfunctions instead of a monolithic IPG pyclass.
//! This is a self-contained prototype that doesn't depend on the complex existing modules.

use pyo3::prelude::*;

// Core modules for the minimal prototype
mod state;
mod app;
mod py_api;

// Minimal widget definitions (self-contained)
mod widgets;

// Re-export for internal use
pub use state::{
    access_state, access_callbacks, access_user_data,
    add_callback, add_user_data, clone_state_to_runtime,
    IpgIds, IpgState,
};

// Import pyfunctions from py_api modules
use py_api::window::add_window;
use py_api::button::add_button;
use py_api::session::{start_session, generate_id};

// Import enums from widgets module
use widgets::enums::{IpgAlignment, IpgHorizontalAlignment, IpgVerticalAlignment};
use widgets::window::{IpgWindowLevel, IpgWindowMode, IpgWindowTheme};
use widgets::button::IpgButtonArrow;
use widgets::colors::IpgColor;
use widgets::styling::IpgStyleStandard;

/// Python module definition
#[pymodule]
fn icedpygui(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Session functions
    m.add_function(wrap_pyfunction!(start_session, m)?)?;
    m.add_function(wrap_pyfunction!(generate_id, m)?)?;
    
    // Widget functions
    m.add_function(wrap_pyfunction!(add_window, m)?)?;
    m.add_function(wrap_pyfunction!(add_button, m)?)?;
    
    // Enums
    m.add_class::<IpgAlignment>()?;
    m.add_class::<IpgHorizontalAlignment>()?;
    m.add_class::<IpgVerticalAlignment>()?;
    m.add_class::<IpgWindowLevel>()?;
    m.add_class::<IpgWindowMode>()?;
    m.add_class::<IpgWindowTheme>()?;
    m.add_class::<IpgButtonArrow>()?;
    m.add_class::<IpgColor>()?;
    m.add_class::<IpgStyleStandard>()?;
    
    Ok(())
}
