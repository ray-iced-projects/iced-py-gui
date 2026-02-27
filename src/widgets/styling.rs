//! Style standard definitions and shared styling helpers

use crate::py_api::helpers::get_radius;

use iced::{Border, Color, Shadow, Vector};
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

/// Apply optional border overrides to an existing `Border`.
pub fn apply_border_overrides(
    border: &mut Border,
    color: Option<Color>,
    radius: &Option<Vec<f32>>,
    width: Option<f32>,
    widget_name: &str,
) {
    if let Some(c) = color {
        border.color = c;
    }
    if let Some(r) = radius {
        border.radius = get_radius(r, widget_name.to_string());
    }
    if let Some(w) = width {
        border.width = w;
    }
}

/// Apply optional shadow overrides to an existing `Shadow`.
// pub fn apply_shadow_overrides(
//     shadow: &mut Shadow,
//     color: Option<Color>,
//     offset_x: Option<f32>,
//     offset_y: Option<f32>,
//     blur_radius: Option<f32>,
// ) {
//     if let Some(c) = color {
//         shadow.color = c;
//     }
//     if let Some(x) = offset_x {
//         shadow.offset = Vector::new(x, offset_y.unwrap_or(shadow.offset.y));
//     } else if let Some(y) = offset_y {
//         shadow.offset = Vector::new(shadow.offset.x, y);
//     }
//     if let Some(b) = blur_radius {
//         shadow.blur_radius = b;
//     }
// }

/// Apply optional shadow overrides using a combined `[x, y]` offset array.
pub fn apply_shadow_overrides_xy(
    shadow: &mut Shadow,
    color: Option<Color>,
    offset: Option<[f32; 2]>,
    blur_radius: Option<f32>,
) {
    if let Some(c) = color {
        shadow.color = c;
    }
    if let Some([x, y]) = offset {
        shadow.offset = Vector::new(x, y);
    }
    if let Some(b) = blur_radius {
        shadow.blur_radius = b;
    }
}
