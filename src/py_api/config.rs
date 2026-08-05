//! Python API for configuration management

use pyo3::prelude::*;
use crate::config_creator::{load_file_filters, reload_file_filters, get_config_directory};

/// Get file filters from configuration
/// 
/// Returns a list of tuples: [(filter_name, extensions), ...]
/// Filters are loaded from user config or defaults if not found.
#[pyfunction]
#[pyo3(text_signature = "()")]
pub fn get_file_filters() -> PyResult<Vec<(String, String)>> {
    load_file_filters()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
}

/// Reload file filters from disk
/// 
/// Useful if the user has edited the configuration file.
/// Returns a list of tuples: [(filter_name, extensions), ...]
#[pyfunction]
#[pyo3(text_signature = "()")]
pub fn reload_filters() -> PyResult<Vec<(String, String)>> {
    reload_file_filters()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
}

/// Get the configuration directory path
/// 
/// Returns the path where the file_filters.json configuration file is located.
#[pyfunction]
#[pyo3(text_signature = "()")]
pub fn get_config_path() -> PyResult<String> {
    Ok(get_config_directory())
}
