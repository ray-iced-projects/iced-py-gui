//! Colors module - provides get_rgba_color and get_color_palette pyfunctions
use std::collections::HashMap;
use iced::theme::palette::readable;
use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::graphics::colors::Color;


#[pyfunction]
#[pyo3(signature = (color, alpha=None))]
pub fn get_rgba_color(
    color: Color,
    alpha: Option<f32>,
    ) -> PyResult<[f32; 4]>
{
    let rgba = if let Some(base) = 
        Color::rgba_ipg_color_to_iced(None, &Some(color), alpha) {
        base
    } else {
        panic!("Unable to get the rgba format of the color")
    };

    Ok([rgba.r, rgba.g, rgba.b, rgba.a])
}

#[pyfunction]
#[pyo3(signature = (
    color=None, 
    rgba=None,
    color_alpha=None))]
pub fn get_color_palette(
    color: Option<Color>,
    rgba: Option<[f32; 4]>,
    color_alpha: Option<f32>,
) -> PyResult<HashMap<String, [f64; 4]>>
{
    let base = Color::rgba_ipg_color_to_iced(rgba, &color, color_alpha)
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(
            "get_color_palette: no color supplied — provide base_color or base_rgba"
        ))?;

    let text_color = readable(base, iced::Color::WHITE);
    let bkg = iced::theme::palette::Background::new(base, text_color);

    fn to_arr(c: iced::Color) -> [f64; 4] {
        let r = |v: f32| ((v as f64) * 100.0).round() / 100.0;
        [r(c.r), r(c.g), r(c.b), r(c.a)]
    }

    let mut map = HashMap::new();
    map.insert("base_color".into(),     to_arr(bkg.base.color));
    map.insert("base_text".into(),      to_arr(bkg.base.text));
    map.insert("weak_color".into(),     to_arr(bkg.weak.color));
    map.insert("weak_text".into(),      to_arr(bkg.weak.text));
    map.insert("weaker_color".into(),   to_arr(bkg.weaker.color));
    map.insert("weaker_text".into(),    to_arr(bkg.weaker.text));
    map.insert("weakest_color".into(),  to_arr(bkg.weakest.color));
    map.insert("weakest_text".into(),   to_arr(bkg.weakest.text));
    map.insert("neutral_color".into(),  to_arr(bkg.neutral.color));
    map.insert("neutral_text".into(),   to_arr(bkg.neutral.text));
    map.insert("strong_color".into(),   to_arr(bkg.strong.color));
    map.insert("strong_text".into(),    to_arr(bkg.strong.text));
    map.insert("stronger_color".into(), to_arr(bkg.stronger.color));
    map.insert("stronger_text".into(),  to_arr(bkg.stronger.text));
    map.insert("strongest_color".into(),to_arr(bkg.strongest.color));
    map.insert("strongest_text".into(), to_arr(bkg.strongest.text));

    Ok(map)
}