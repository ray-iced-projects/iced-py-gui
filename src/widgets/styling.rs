//! Style standard definitions and shared styling helpers
use crate::py_api::helpers::get_radius;

use iced::{Background as IcedBackground, Border, Radians, Shadow, Vector};
use iced::gradient::{self, Gradient};

use pyo3::pyclass;

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum StyleStandard {
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
    color: Option<iced::Color>,
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

/// Apply optional shadow overrides using a combined `[x, y]` offset array.
pub fn apply_shadow_overrides_xy(
    shadow: &mut Shadow,
    color: Option<iced::Color>,
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

/// Build an `Option<Background>` from the user-supplied color/gradient fields.
///
/// * If `gradient_color_stop` is `Some`, a linear gradient background is built
///   from `background_color` (offset 0.0) → `gradient_color_stop` (offset 1.0)
///   using either `gradient_degrees` or `gradient_radians` for the angle
///   (degrees is checked first, then radians; defaults to 0.0).
/// * Otherwise, if `background_color` alone is `Some`, a solid color background
///   is returned.
/// * If neither is set the existing background is left untouched (`None`).
pub fn apply_background_color_overrides(
    background: &mut Option<IcedBackground>,
    background_color: Option<iced::Color>,
    gradient_color_stop: Option<iced::Color>,
    gradient_degrees: Option<f32>,
    gradient_radians: Option<f32>,
) {
    if let Some(stop_color) = gradient_color_stop {
        // Calculate angle: prefer degrees, fall back to radians, default 0.
        let angle: Radians = if let Some(deg) = gradient_degrees {
            Radians(deg.to_radians())
        } else if let Some(rad) = gradient_radians {
            Radians(rad)
        } else {
            Radians(0.0)
        };

        let start_color = background_color.unwrap_or(iced::Color::TRANSPARENT);

        let linear = gradient::Linear::new(angle)
            .add_stop(0.0, start_color)
            .add_stop(1.0, stop_color);

        *background = Some(IcedBackground::Gradient(Gradient::Linear(linear)));
    } else if let Some(color) = background_color {
        *background = Some(IcedBackground::Color(color));
    }
}
