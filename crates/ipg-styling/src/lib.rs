//! styling
use pyo3::{pyclass, Py, PyAny, Python};
use pyo3::types::PyAnyMethods;

type PyObject = Py<PyAny>;

pub mod colors;
use crate::colors::IpgColor;

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgStyleStandard {
    Primary,
    Success,
    Danger,
    Text,
    Warning,
}

// pub fn get_theme_color(wnd_theme: &Theme) -> Color {
//     let palette = Theme::palette(wnd_theme);

//     palette.background
// }

// pub fn is_dark(color: Color) -> bool {
//     to_hsl(color).lightness < 0.6
// }

// pub fn darken(color: Color, amount: f32) -> Color {
//     let mut hsl = to_hsl(color);

//     hsl.lightness = if hsl.lightness - amount < 0.0 {
//         0.0
//     } else {
//         hsl.lightness - amount
//     };

//     from_hsl(hsl)
// }

// pub fn lighten(color: Color, amount: f32) -> Color {
//     let mut hsl = to_hsl(color);

//     hsl.lightness = if hsl.lightness + amount > 1.0 {
//         1.0
//     } else {
//         hsl.lightness + amount
//     };

//     from_hsl(hsl)
// }

// fn to_hsl(color: Color) -> Hsl {
//     Hsl::from_color(Rgb::from(color))
// }

// fn from_hsl(hsl: Hsl) -> Color {
//     Rgb::from_color(hsl).into()
// }

// pub fn readable(background: Color, text: Color) -> Color {
//     if is_readable(background, text) {
//         text
//     } else {
//         let white_contrast = relative_contrast(background, Color::WHITE);
//         let black_contrast = relative_contrast(background, Color::BLACK);

//         if white_contrast >= black_contrast {
//             Color::WHITE
//         } else {
//             Color::BLACK
//         }
//     }
// }

// fn is_readable(a: Color, b: Color) -> bool {
//     let a_srgb = Rgb::from(a);
//     let b_srgb = Rgb::from(b);

//     a_srgb.has_enhanced_contrast_text(b_srgb)
// }

// fn relative_contrast(a: Color, b: Color) -> f32 {
//     let a_srgb = Rgb::from(a);
//     let b_srgb = Rgb::from(b);

//     a_srgb.relative_contrast(b_srgb)
// }

pub fn try_extract_style_standard(value: &PyObject, name: String) -> IpgStyleStandard {
    Python::attach(|py| {

        let res = value.bind(py).extract::<IpgStyleStandard>();
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for StyleStandard", name),
        }
    })
}

pub fn try_extract_ipg_color(value: &PyObject, name: String) -> IpgColor {
    Python::attach(|py| {

        let res = value.bind(py).extract::<IpgColor>();
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for IpgColor", name),
        }
    })
}

pub fn try_extract_rgba_color(value: &PyObject, name: String) -> [f32; 4] {
    Python::attach(|py| {

        let res = value.bind(py).extract::<[f32; 4]>();
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for RGBA color", name),
        }
    })
}
