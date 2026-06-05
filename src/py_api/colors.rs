//! Colors module - provides get_rgba_color and get_color_palette pyfunctions

use std::collections::HashMap;

use iced::Theme;
use iced::theme::Palette;
use iced::theme::palette;
use iced::theme::palette::readable;
use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::graphics::colors::{Color, StdColorStyle};
use crate::widgets::ipg_window::name_to_window_theme;


#[pyfunction]
#[pyo3(signature = (color=None, alpha=None, name=None))]
pub fn get_rgba_color(
    color: Option<Color>,
    alpha: Option<f32>,
    name: Option<String>,
    ) -> PyResult<[f32; 4]>
{
    let rgba = match (color.is_some(), name.is_some()) {
        (true, false) => Color::rgba_ipg_color_to_iced(None, &color, alpha).unwrap(),
        (false, true) => Color::rgba_ipg_color_to_iced(None, &Color::from_combo_str(&name.unwrap()), alpha).unwrap(),
        _ => panic!("Either color or name is be a value")
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

/// Returns the palette used by the standard widget styles (button, container, etc.)
/// for a given color name and theme.
///
/// `color_name` : one of "Primary", "Secondary", "Success", "Warning", "Danger",
///                "Background", or "Subtle"  
/// `theme_name` : a theme name string as returned by `window_theme_names()`
///
/// Returns the same `{key_color / key_text}` dict as `get_color_palette`, drawn
/// directly from `theme.extended_palette()` so the colors match what the widgets
/// actually render.  "Background" and "Subtle" both map to the background section
/// of the extended palette (same source iced uses for `button::subtle()`).
/// The background section also includes "neutral_color"/"neutral_text" keys that
/// the other sections do not have.
#[pyfunction]
#[pyo3(signature = (theme_name, std_style_color))]
pub fn get_styling_palette(
    theme_name: String,
    std_style_color: StdColorStyle,
) -> PyResult<HashMap<String, ([f64; 4], [f64; 4])>>
{
    let theme: Theme = name_to_window_theme(&theme_name)
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(
            format!("get_styling_palette: unknown theme '{theme_name}'")
        ))?
        .to_iced();

    fn to_arr(c: iced::Color) -> [f64; 4] {
        let r = |v: f32| ((v as f64) * 100.0).round() / 100.0;
        [r(c.r), r(c.g), r(c.b), r(c.a)]
    }
    let seed = match theme {
        Theme::CatppuccinFrappe => palette::Seed::CATPPUCCIN_FRAPPE,
        Theme::CatppuccinLatte => palette::Seed::CATPPUCCIN_LATTE,
        Theme::CatppuccinMacchiato => palette::Seed::CATPPUCCIN_MACCHIATO,
        Theme::CatppuccinMocha => palette::Seed::CATPPUCCIN_MOCHA,
        Theme::Dark => palette::Seed::DARK,
        Theme::Dracula => palette::Seed::DRACULA,
        Theme::Ferra => palette::Seed::FERRA,
        Theme::GruvboxDark => palette::Seed::GRUVBOX_DARK,
        Theme::GruvboxLight => palette::Seed::GRUVBOX_LIGHT,
        Theme::KanagawaDragon => palette::Seed::KANAGAWA_DRAGON,
        Theme::KanagawaLotus => palette::Seed::KANAGAWA_LOTUS,
        Theme::KanagawaWave => palette::Seed::KANAGAWA_WAVE,
        Theme::Light => palette::Seed::LIGHT,
        Theme::Moonfly => palette::Seed::MOONFLY,
        Theme::Nightfly => palette::Seed::NIGHTFLY,
        Theme::Nord => palette::Seed::NORD,
        Theme::Oxocarbon => palette::Seed::OXOCARBON,
        Theme::SolarizedDark => palette::Seed::SOLARIZED_DARK,
        Theme::SolarizedLight => palette::Seed::SOLARIZED_LIGHT,
        Theme::TokyoNight => palette::Seed::TOKYO_NIGHT,
        Theme::TokyoNightLight => palette::Seed::TOKYO_NIGHT_LIGHT,
        Theme::TokyoNightStorm => palette::Seed::TOKYO_NIGHT_STORM,
        Theme::Custom(_custom) => todo!(),
    };
    
    let pal = Palette::generate(seed);

    let mut hm = HashMap::new();
    match std_style_color {
        StdColorStyle::Primary => {
            hm.insert("base".to_string(), (to_arr(pal.primary.base.color), to_arr(pal.primary.base.text)));
            hm.insert("weak".to_string(), (to_arr(pal.primary.weak.color), to_arr(pal.primary.weak.text)));
            hm.insert("strong".to_string(), (to_arr(pal.primary.strong.color), to_arr(pal.primary.strong.text)));
        },
        StdColorStyle::Secondary => {
            hm.insert("base".to_string(), (to_arr(pal.secondary.base.color), to_arr(pal.secondary.base.text)));
            hm.insert("weak".to_string(),  (to_arr(pal.secondary.weak.color), to_arr(pal.secondary.weak.text)));
            hm.insert("strong".to_string(),   (to_arr(pal.secondary.strong.color), to_arr(pal.secondary.strong.text)));
        },
        StdColorStyle::Success => {
            hm.insert("base".to_string(), (to_arr(pal.success.base.color), to_arr(pal.success.base.text)));
            hm.insert("weak".to_string(),  (to_arr(pal.success.weak.color), to_arr(pal.success.weak.text)));
            hm.insert("strong".to_string(),   (to_arr(pal.success.strong.color), to_arr(pal.success.strong.text)));
        },
        StdColorStyle::Danger => {
            hm.insert("base".to_string(), (to_arr(pal.danger.base.color), to_arr(pal.danger.base.text)));
            hm.insert("weak".to_string(),  (to_arr(pal.danger.weak.color), to_arr(pal.danger.weak.text)));
            hm.insert("strong".to_string(),   (to_arr(pal.danger.strong.color), to_arr(pal.danger.strong.text)));
        },
        StdColorStyle::Warning => {
            hm.insert("base".to_string(), (to_arr(pal.warning.base.color), to_arr(pal.warning.base.text)));
            hm.insert("weak".to_string(),  (to_arr(pal.warning.weak.color), to_arr(pal.warning.weak.text)));
            hm.insert("strong".to_string(),   (to_arr(pal.warning.strong.color), to_arr(pal.warning.strong.text)));
        },
    }
    hm.insert("bkg_base".to_string(), (to_arr(pal.background.base.color), to_arr(pal.background.base.text)));
    hm.insert("bkg_weak".to_string(), (to_arr(pal.background.weak.color), to_arr(pal.background.weak.text)));
    hm.insert("bkg_weaker".to_string(), (to_arr(pal.background.weaker.color), to_arr(pal.background.weaker.text)));
    hm.insert("bkg_weakest".to_string(), (to_arr(pal.background.weakest.color), to_arr(pal.background.weakest.text)));
    hm.insert("bkg_neutral".to_string(), (to_arr(pal.background.neutral.color), to_arr(pal.background.neutral.text)));
    hm.insert("bkg_strong".to_string(), (to_arr(pal.background.strong.color), to_arr(pal.background.strong.text)));
    hm.insert("bkg_stronger".to_string(), (to_arr(pal.background.stronger.color), to_arr(pal.background.stronger.text)));
    hm.insert("bkg_strongest".to_string(), (to_arr(pal.background.strongest.color), to_arr(pal.background.strongest.text)));
    
    Ok(hm)
    
}