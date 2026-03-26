//! Colors module - provides get_rgba_color and get_color_palette pyfunctions
use iced::theme::palette::readable;
use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::graphics::colors::IpgColor;


#[pyfunction]
#[pyo3(signature = (color, alpha=None))]
pub fn get_rgba_color(
    color: IpgColor,
    alpha: Option<f32>,
    ) -> PyResult<[f32; 4]>
{
    let rgba = if let Some(base) = 
        IpgColor::rgba_ipg_color_to_iced(None, Some(color), alpha) {
        base
    } else {
        panic!("Unable to get the rgba format of the color")
    };

    Ok([rgba.r, rgba.g, rgba.b, rgba.a])
}

#[pyfunction]
#[pyo3(signature = (
    base_color=None, 
    base_rgba=None,
    alpha=None))]
pub fn get_color_palette(
    base_color: Option<IpgColor>,
    base_rgba: Option<[f32; 4]>,
    alpha: Option<f32>,
    ) -> PyResult<([f32; 4], [f32; 4], [f32; 4])>
{
    
    let base = IpgColor::rgba_ipg_color_to_iced(base_rgba, base_color, alpha);

    let text_color = readable(base.unwrap(), iced::Color::WHITE);

    let palette = iced::theme::palette::Background::new(base.unwrap(), text_color);

    let color = palette.strong.color;
    let strong = [color.r, color.g, color.b, color.a];
    let color = palette.weak.color;
    let weak = [color.r, color.g, color.b, color.a];
    let color = text_color;
    let text = [color.r, color.g, color.b, color.a];

    Ok((strong, weak, text)) 
}