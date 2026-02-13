//! Style standard definitions

use pyo3::pyclass;

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgStyleStandard {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Text,
}
