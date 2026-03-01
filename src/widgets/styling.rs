//! Style standard definitions and shared styling helpers
use crate::{graphics::colors::IpgColor, py_api::helpers::get_radius};

use iced::{Background as IcedBackground, Border, Color, Radians, Shadow, Theme, Vector, theme::Palette};
use iced::gradient::{self, Gradient};
use iced::theme::palette::{Extended, Background, Primary, Secondary, 
    Success, Warning, Danger, is_dark};

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

/// Build an `Option<Background>` from the user-supplied color/gradient fields.
///
/// * If `gradient_color_stop` is `Some`, a linear gradient background is built
///   from `background_color` (offset 0.0) → `gradient_color_stop` (offset 1.0)
///   using either `gradient_degrees` or `gradient_radians` for the angle
///   (degrees is checked first, then radians; defaults to 0.0).
/// * Otherwise, if `background_color` alone is `Some`, a solid color background
///   is returned.
/// * If neither is set the existing background is left untouched (`None`).
pub fn apply_background_overrides(
    background: &mut Option<IcedBackground>,
    background_color: Option<Color>,
    gradient_color_stop: Option<Color>,
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

        let start_color = background_color.unwrap_or(Color::TRANSPARENT);

        let linear = gradient::Linear::new(angle)
            .add_stop(0.0, start_color)
            .add_stop(1.0, stop_color);

        *background = Some(IcedBackground::Gradient(Gradient::Linear(linear)));
    } else if let Some(color) = background_color {
        *background = Some(IcedBackground::Color(color));
    }
}

pub fn get_theme_palette_color(theme: &iced::Theme) -> Color {
    let r = theme.palette().background.r;
    let g = theme.palette().background.g;
    let b = theme.palette().background.b;
    let a = theme.palette().background.a;
    let rgba = [r, b, g, a];
    IpgColor::rgba_ipg_color_to_iced(Some(rgba), None, 1.0, false).unwrap()
}


pub fn create_custom_theme(base_color: Color, dark_mode: bool) -> Theme {
    // Build a Palette from your one color
    let background = if dark_mode { Color::from_rgb(0.15, 0.15, 0.18) } 
                     else { Color::from_rgb(0.95, 0.95, 0.93) };
    let text = if dark_mode { Color::from_rgb(0.9, 0.9, 0.9) } 
               else { Color::from_rgb(0.1, 0.1, 0.1) };

    let palette = Palette {
        background,
        text,
        primary: base_color,
        success: base_color,  // or hue-rotated
        warning: base_color,  // or hue-rotated
        danger: base_color,   // or hue-rotated
    };

    Theme::custom_with_fn("Custom", palette, |p| {
        // Here you control ALL the deviation factors
        Extended {
            background: Background::new(p.background, p.text),  // uses iced's defaults
            primary: Primary::generate(p.primary, p.background, p.text),
            secondary: Secondary::generate(p.background, p.text),
            success: Success::generate(p.success, p.background, p.text),
            warning: Warning::generate(p.warning, p.background, p.text),
            danger: Danger::generate(p.danger, p.background, p.text),
            is_dark: is_dark(p.background),
        }
    })
}

pub fn get_custom_palette(bkg_color: Color) -> (Extended, Color) {
    let dark_mode = is_dark(bkg_color);
    let custom_theme = create_custom_theme(bkg_color, dark_mode);
    let text_color = custom_theme.palette().text;
    (custom_theme.extended_palette().to_owned(), text_color)
}
