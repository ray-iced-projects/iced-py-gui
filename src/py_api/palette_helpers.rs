//! Palette Helpers module - provides various palette pyfunction

use pyo3::{PyResult, PyAny, Py, pyfunction, Python};
type PyObject = Py<PyAny>;

/// Parse YAML content string returned by the file system dialog.
/// Validates that the file has the correct file_type identifier.
fn load_yaml_parts(parts_file: &str) -> PyResult<serde_yaml::Value> {
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(parts_file).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Failed to parse YAML: {}", e))
    })?;

    // Validate it's the correct file type
    if let Some(file_type) = yaml_value.get("file_type").and_then(|v| v.as_str()) {
        if file_type != "widget_palette_parts" {
            return Err(pyo3::exceptions::PyValueError::new_err(
                format!("Wrong file type: '{}'. Expected 'widget_palette_parts'", file_type)
            ));
        }
    } else {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Not a valid widget_palette_parts file (missing file_type identifier)"
        ));
    }

    Ok(yaml_value)
}

/// Returns the top-level widget names from the parts file as a Python list.
///
/// Parameters
/// ----------
/// parts_file : str
///     YAML content string of the widget_palette_parts file, obtained
///     from add_file_system_dialog() results_callback with load_file=True.
///
/// Returns
/// -------
/// list[str]
///     Widget names found in the file
#[pyfunction]
pub fn get_widget_palette_list(parts_file: String) -> PyResult<PyObject> {
    let yaml_value = load_yaml_parts(&parts_file)?;

    Python::attach(|py| {
        use pyo3::types::PyList;

        let names: Vec<&str> = yaml_value
            .as_mapping()
            .map(|m| {
                m.keys()
                    .filter_map(|k| k.as_str())
                    .filter(|k| k != &"file_type" && k != &"version")  // Skip metadata
                    .collect()
            })
            .unwrap_or_default();

        let list = PyList::new(py, names)?;
        Ok(list.unbind().into())
    })
}

/// Returns the palette configuration for a specific widget from the parts file as a Python dict.
///
/// Parameters
/// ----------
/// widget : str
///     Widget name to look up (case-insensitive)
/// parts_file : str
///     YAML content string of the widget_palette_parts file, obtained
///     from add_file_system_dialog() results_callback with load_file=True.
///
/// Returns
/// -------
/// dict
///     Widget palette configuration
#[pyfunction]
pub fn get_widget_palette_part(widget: String, parts_file: String) -> PyResult<PyObject> {
    let widget_lower = widget.to_lowercase();
    let yaml_value = load_yaml_parts(&parts_file)?;

    let widget_config = yaml_value
        .as_mapping()
        .and_then(|m| {
            m.iter()
                .find(|(k, _)| k.as_str().map_or(false, |s| s.to_lowercase() == widget_lower))
                .map(|(_, v)| v.clone())
        })
        .ok_or_else(|| {
            pyo3::exceptions::PyKeyError::new_err(
                format!("Widget '{}' not found in '{}'", widget, parts_file)
            )
        })?;

    Python::attach(|py| {
        yaml_to_py_dict(py, &widget_config)
    })
}

/// Helper function to convert serde_yaml::Value to Python dict
fn yaml_to_py_dict<'py>(py: pyo3::Python<'py>, value: &serde_yaml::Value) -> PyResult<PyObject> {
    use pyo3::types::{PyDict, PyDictMethods};
    use pyo3::IntoPyObject;
    
    match value {
        serde_yaml::Value::Mapping(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                let key: PyObject = match k.as_str() {
                    Some(s) => s.into_pyobject(py)?.into_any().unbind(),
                    None => k.as_i64().unwrap_or(0).into_pyobject(py)?.into_any().unbind(),
                };
                let val = yaml_to_py_object(py, v)?;
                dict.set_item(key, val)?;
            }
            Ok(dict.unbind().into())
        }
        other => yaml_to_py_object(py, other),
    }
}

/// Helper function to convert serde_yaml::Value to Python object
fn yaml_to_py_object<'py>(py: pyo3::Python<'py>, value: &serde_yaml::Value) -> PyResult<PyObject> {
    use pyo3::types::{PyDict, PyDictMethods, PyList};
    use pyo3::IntoPyObject;

    match value {
        serde_yaml::Value::Null => Ok(py.None()),
        serde_yaml::Value::Bool(b) => {
            // Borrowed can't be moved; borrow through deref then clone_ref to get owned Py<T>
            let borrowed = (*b).into_pyobject(py)?;
            Ok(borrowed.as_unbound().clone_ref(py).into())
        },
        serde_yaml::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any().unbind())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.into_any().unbind())
            } else {
                Ok(n.to_string().into_pyobject(py)?.into_any().unbind())
            }
        }
        serde_yaml::Value::String(s) => Ok(s.as_str().into_pyobject(py)?.into_any().unbind()),
        serde_yaml::Value::Sequence(seq) => {
            let items: Vec<PyObject> = seq.iter()
                .map(|item| yaml_to_py_object(py, item))
                .collect::<PyResult<_>>()?;
            let list = PyList::new(py, items)?;
            Ok(list.unbind().into())
        }
        serde_yaml::Value::Mapping(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                let key: PyObject = match k.as_str() {
                    Some(s) => s.into_pyobject(py)?.into_any().unbind(),
                    None => k.as_i64().unwrap_or(0).into_pyobject(py)?.into_any().unbind(),
                };
                let val = yaml_to_py_object(py, v)?;
                dict.set_item(key, val)?;
            }
            Ok(dict.unbind().into())
        }
        serde_yaml::Value::Tagged(tagged) => {
            yaml_to_py_object(py, &tagged.value)
        }
    }
}
