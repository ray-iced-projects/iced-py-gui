//! Minimal lib.rs - prototype for modular widget architecture
//!
//! This demonstrates using module-level pyfunctions instead of a monolithic IPG pyclass.
//! This is a self-contained prototype that doesn't depend on the complex existing modules.

use pyo3::prelude::*;

// Core modules for the minimal prototype
mod state;
mod app;
mod py_api;
mod graphics;

// Minimal widget definitions (self-contained)
mod widgets;

// Re-export for internal use
pub use state::{
    access_state, access_callbacks, access_user_data1,
    add_callback_to_mutex, add_user_data_to_mutex, clone_state_to_runtime,
    IpgIds, IpgState
};

// Import pyfunctions from py_api modules
use py_api::window::add_window;
use py_api::ipg_button::{add_button, add_button_style};
use py_api::ipg_column::add_column;
use py_api::ipg_container::add_container;
use py_api::ipg_row::add_row;
use py_api::session::{start_session, generate_id};
use py_api::update::update_widget;

// Import enums from widgets module
use widgets::enums::{IpgAlignment, IpgHorizontalAlignment, IpgVerticalAlignment};
use widgets::window::{IpgWindowLevel, IpgWindowMode, IpgWindowTheme};
use widgets::button::IpgButtonArrow;
use widgets::styling::IpgStyleStandard;

use crate::graphics::colors::IpgColor;
use crate::widgets::button::{IpgButtonParam, IpgButtonStyleParam};

/// Python module definition
#[pymodule]
fn icedpygui(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Session functions
    m.add_function(wrap_pyfunction!(start_session, m)?)?;
    m.add_function(wrap_pyfunction!(generate_id, m)?)?;
    
    // Widget functions
    m.add_function(wrap_pyfunction!(add_window, m)?)?;
    m.add_function(wrap_pyfunction!(add_button, m)?)?;
    m.add_function(wrap_pyfunction!(add_column, m)?)?;
    m.add_function(wrap_pyfunction!(add_container, m)?)?;
    m.add_function(wrap_pyfunction!(add_row, m)?)?;
    m.add_function(wrap_pyfunction!(update_widget, m)?)?;
    
    // styles
    m.add_function(wrap_pyfunction!(add_button_style, m)?)?;
    // Enums
    m.add_class::<IpgAlignment>()?;
    m.add_class::<IpgHorizontalAlignment>()?;
    m.add_class::<IpgVerticalAlignment>()?;
    m.add_class::<IpgWindowLevel>()?;
    m.add_class::<IpgWindowMode>()?;
    m.add_class::<IpgWindowTheme>()?;
    m.add_class::<IpgButtonArrow>()?;
    m.add_class::<IpgButtonParam>()?;
    m.add_class::<IpgButtonStyleParam>()?;
    m.add_class::<IpgColor>()?;
    m.add_class::<IpgStyleStandard>()?;
    
    Ok(())
}
