//! Colors module - provides get_rgba_color and get_color_palette pyfunctions

use std::collections::HashMap;

use iced::Theme;
use iced::theme::palette::readable;
use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::graphics::colors::{Color, StdColorStyle};
use crate::state::Widgets;
use crate::state::get_id;
use crate::widgets::ipg_window::name_to_window_theme;
use crate::widgets::widget_param_update::WidgetParamUpdate;
use crate::ButtonStyleStd;


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
) -> PyResult<HashMap<PaletteKey, ([f64; 4], [f64; 4])>>
{
    let base = Color::rgba_ipg_color_to_iced(rgba, &color, color_alpha)
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(
            "get_color_palette: no color supplied — provide base_color or base_rgba"
        ))?;

    Ok(color_palette(base))
    
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
) -> PyResult<HashMap<PaletteKey, ([f64; 4], [f64; 4])>>
{
    // Resolve built-in themes via WindowTheme enum; fall back to custom theme store.
    let theme: Theme = if let Some(wt) = name_to_window_theme(&theme_name) {
        wt.to_iced()
    } else if let Some(ct) = crate::widgets::ipg_window::get_custom_theme(&theme_name) {
        ct
    } else {
        return Err(pyo3::exceptions::PyValueError::new_err(
            format!("get_styling_palette: unknown theme '{theme_name}'")
        ));
    };

    fn to_arr(c: iced::Color) -> [f64; 4] {
        let r = |v: f32| ((v as f64) * 100.0).round() / 100.0;
        [r(c.r), r(c.g), r(c.b), r(c.a)]
    }

    // theme.palette() works for both built-in and Custom themes.
    let pal = theme.palette();

    let mut hm = HashMap::new();
    match std_style_color {
        StdColorStyle::Primary => {
            hm.insert(PaletteKey::Base, (to_arr(pal.primary.base.color), to_arr(pal.primary.base.text)));
            hm.insert(PaletteKey::Weak, (to_arr(pal.primary.weak.color), to_arr(pal.primary.weak.text)));
            hm.insert(PaletteKey::Strong, (to_arr(pal.primary.strong.color), to_arr(pal.primary.strong.text)));
        },
        StdColorStyle::Secondary => {
            hm.insert(PaletteKey::Base, (to_arr(pal.secondary.base.color), to_arr(pal.secondary.base.text)));
            hm.insert(PaletteKey::Weak,  (to_arr(pal.secondary.weak.color), to_arr(pal.secondary.weak.text)));
            hm.insert(PaletteKey::Strong,   (to_arr(pal.secondary.strong.color), to_arr(pal.secondary.strong.text)));
        },
        StdColorStyle::Success => {
            hm.insert(PaletteKey::Base, (to_arr(pal.success.base.color), to_arr(pal.success.base.text)));
            hm.insert(PaletteKey::Weak,  (to_arr(pal.success.weak.color), to_arr(pal.success.weak.text)));
            hm.insert(PaletteKey::Strong,   (to_arr(pal.success.strong.color), to_arr(pal.success.strong.text)));
        },
        StdColorStyle::Danger => {
            hm.insert(PaletteKey::Base, (to_arr(pal.danger.base.color), to_arr(pal.danger.base.text)));
            hm.insert(PaletteKey::Weak,  (to_arr(pal.danger.weak.color), to_arr(pal.danger.weak.text)));
            hm.insert(PaletteKey::Strong,   (to_arr(pal.danger.strong.color), to_arr(pal.danger.strong.text)));
        },
        StdColorStyle::Warning => {
            hm.insert(PaletteKey::Base, (to_arr(pal.warning.base.color), to_arr(pal.warning.base.text)));
            hm.insert(PaletteKey::Weak,  (to_arr(pal.warning.weak.color), to_arr(pal.warning.weak.text)));
            hm.insert(PaletteKey::Strong,   (to_arr(pal.warning.strong.color), to_arr(pal.warning.strong.text)));
        },
    }

    let bkg_pal = color_palette(pal.background.base.color);
    
    hm.extend(bkg_pal);

    Ok(hm)
    
}

#[pyfunction]
#[pyo3(signature = (theme_name, std_color))]
pub fn get_button_palette(
    theme_name: String,
    std_color: ButtonStyleStd,
) -> PyResult<HashMap<PaletteKey, ([f64; 4], [f64; 4])>>
{
    // Resolve built-in themes via WindowTheme enum; fall back to custom theme store.
    let theme: Theme = if let Some(wt) = name_to_window_theme(&theme_name) {
        wt.to_iced()
    } else if let Some(ct) = crate::widgets::ipg_window::get_custom_theme(&theme_name) {
        ct
    } else {
        return Err(pyo3::exceptions::PyValueError::new_err(
            format!("get_styling_palette: unknown theme '{theme_name}'")
        ));
    };

    fn to_arr(c: iced::Color) -> [f64; 4] {
        let r = |v: f32| ((v as f64) * 100.0).round() / 100.0;
        [r(c.r), r(c.g), r(c.b), r(c.a)]
    }

    // theme.palette() works for both built-in and Custom themes.
    let pal = theme.palette();

    let mut hm = HashMap::new();
    match std_color {
        ButtonStyleStd::Primary => {
            hm.insert(PaletteKey::Base, (to_arr(pal.primary.base.color), to_arr(pal.primary.base.text)));
            hm.insert(PaletteKey::Weak, (to_arr(pal.primary.weak.color), to_arr(pal.primary.weak.text)));
            hm.insert(PaletteKey::Strong, (to_arr(pal.primary.strong.color), to_arr(pal.primary.strong.text)));
        },
        ButtonStyleStd::Secondary => {
            hm.insert(PaletteKey::Base, (to_arr(pal.secondary.base.color), to_arr(pal.secondary.base.text)));
            hm.insert(PaletteKey::Weak,  (to_arr(pal.secondary.weak.color), to_arr(pal.secondary.weak.text)));
            hm.insert(PaletteKey::Strong,   (to_arr(pal.secondary.strong.color), to_arr(pal.secondary.strong.text)));
        },
        ButtonStyleStd::Success => {
            hm.insert(PaletteKey::Base, (to_arr(pal.success.base.color), to_arr(pal.success.base.text)));
            hm.insert(PaletteKey::Weak,  (to_arr(pal.success.weak.color), to_arr(pal.success.weak.text)));
            hm.insert(PaletteKey::Strong,   (to_arr(pal.success.strong.color), to_arr(pal.success.strong.text)));
        },
        ButtonStyleStd::Danger => {
            hm.insert(PaletteKey::Base, (to_arr(pal.danger.base.color), to_arr(pal.danger.base.text)));
            hm.insert(PaletteKey::Weak,  (to_arr(pal.danger.weak.color), to_arr(pal.danger.weak.text)));
            hm.insert(PaletteKey::Strong,   (to_arr(pal.danger.strong.color), to_arr(pal.danger.strong.text)));
        },
        ButtonStyleStd::Warning => {
            hm.insert(PaletteKey::Base, (to_arr(pal.warning.base.color), to_arr(pal.warning.base.text)));
            hm.insert(PaletteKey::Weak,  (to_arr(pal.warning.weak.color), to_arr(pal.warning.weak.text)));
            hm.insert(PaletteKey::Strong,   (to_arr(pal.warning.strong.color), to_arr(pal.warning.strong.text)));
        },
        ButtonStyleStd::Background => {
            hm.insert(PaletteKey::Base, (to_arr(pal.background.base.color), to_arr(pal.background.base.text)));
            hm.insert(PaletteKey::Weak,  (to_arr(pal.background.weak.color), to_arr(pal.background.weak.text)));
            hm.insert(PaletteKey::Strong,   (to_arr(pal.background.strong.color), to_arr(pal.background.strong.text)));
        },
        ButtonStyleStd::Subtle => {
            hm.insert(PaletteKey::Weaker, (to_arr(pal.background.weaker.color), to_arr(pal.background.weaker.text)));
            hm.insert(PaletteKey::Weakest,  (to_arr(pal.background.weakest.color), to_arr(pal.background.weakest.text)));
            hm.insert(PaletteKey::Strong,   (to_arr(pal.background.strong.color), to_arr(pal.background.strong.text)));
        },
        ButtonStyleStd::Text => {
            hm.insert(PaletteKey::Base, (to_arr(pal.background.base.text), to_arr(pal.background.base.text.scale_alpha(0.8))));
        },
            }
    
    // let bkg_pal = color_palette(pal.background.base.color);
    
    // hm.extend(bkg_pal);
    
    Ok(hm)
    
}

fn color_palette(base: iced::Color) -> HashMap<PaletteKey, ([f64; 4], [f64; 4])> {
    let text_color = readable(base, iced::Color::WHITE);
    let bkg = iced::theme::palette::Background::new(base, text_color);

    fn to_arr(c: iced::Color) -> [f64; 4] {
        let r = |v: f32| ((v as f64) * 100.0).round() / 100.0;
        [r(c.r), r(c.g), r(c.b), r(c.a)]
    }
    
    let mut map = HashMap::new();
    
    map.insert(PaletteKey::Base,     (to_arr(bkg.base.color), to_arr(bkg.base.text)));
    map.insert(PaletteKey::Weak,     (to_arr(bkg.weak.color), to_arr(bkg.weak.text)));
    map.insert(PaletteKey::Weaker,   (to_arr(bkg.weaker.color), to_arr(bkg.weaker.text)));
    map.insert(PaletteKey::Weakest,  (to_arr(bkg.weakest.color), to_arr(bkg.weakest.text)));
    map.insert(PaletteKey::Neutral,  (to_arr(bkg.neutral.color), to_arr(bkg.neutral.text)));
    map.insert(PaletteKey::Strong,   (to_arr(bkg.strong.color), to_arr(bkg.strong.text)));
    map.insert(PaletteKey::Stronger, (to_arr(bkg.stronger.color), to_arr(bkg.stronger.text)));
    map.insert(PaletteKey::Strongest, (to_arr(bkg.strongest.color), to_arr(bkg.strongest.text)));
    
    map
}

#[pyfunction]
#[pyo3(signature = (
    color=None, 
    rgba=None,
    statuses=None,
    gen_id=None))]
pub fn custom_palette(
    color: Option<Color>,
    rgba: Option<[f32; 4]>,
    statuses: Option<Vec<(WidgetStatus, Vec<(StylePart, PaletteKey, f32)>)>>,
    gen_id: Option<usize>,
) -> PyResult<usize>
{
    let base = Color::rgba_ipg_color_to_iced(rgba, &color, None)
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(
            "get_color_palette: no color supplied — provide base_color or base_rgba"
        ))?;

    let text_color = readable(base, iced::Color::WHITE);
    let background = iced::theme::palette::Background::new(base, text_color);
  
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::Palette(
        CustomPalette {
            id,
            background,
            statuses: statuses.map(|v| v.into_iter().collect()),
        }));

    drop(state);
    Ok(id)
}

#[derive(Debug, Clone)]
pub struct CustomPalette{
    pub id: usize,
    pub background: iced::theme::palette::Background,
    pub statuses: Option<HashMap<WidgetStatus, Vec<(StylePart, PaletteKey, f32)>>>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum CustomPaletteParam {
    Background,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum PaletteKey {
    Base,
    Neutral,
    Strong,
    Stronger,
    Strongest,
    Weak,
    Weaker,
    Weakest,
}

impl PaletteKey {
    pub fn color_key_to_color(&self, bkg: &iced::theme::palette::Background) -> iced::Color {
        match self {
            PaletteKey::Base => bkg.base.color,
            PaletteKey::Neutral => bkg.neutral.color,
            PaletteKey::Strong => bkg.strong.color,
            PaletteKey::Stronger => bkg.stronger.color,
            PaletteKey::Strongest => bkg.strongest.color,
            PaletteKey::Weak => bkg.weak.color,
            PaletteKey::Weaker => bkg.weaker.color,
            PaletteKey::Weakest => bkg.weakest.color,
        }
    }

    pub fn text_key_to_color(&self, bkg: &iced::theme::palette::Background) -> iced::Color {
        match self {
            PaletteKey::Base => bkg.base.text,
            PaletteKey::Neutral => bkg.neutral.text,
            PaletteKey::Strong => bkg.strong.text,
            PaletteKey::Stronger => bkg.stronger.text,
            PaletteKey::Strongest => bkg.strongest.text,
            PaletteKey::Weak => bkg.weak.text,
            PaletteKey::Weaker => bkg.weaker.text,
            PaletteKey::Weakest => bkg.weakest.text,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum PaletteWidget {
    Button,
    Checkbox,
    PickList,
    Radio,
    Sash,
    Slider,
    TextEditor,
    TextInput,
    Toggler,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum StylePart {
    Accent,
    Background,
    Base,
    Border,
    Icon,
    Text,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum WidgetStatus {
    Active,
    Disabled,
    Dragged,
    Focused,
    Hovered,
    IsChecked,
    IsToggled,
    Opened,
    Pressed,
}

use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;
// / ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for CustomPalette {
    type Param = CustomPaletteParam;

    fn param_update(&mut self, param: Self::Param, _value: &PyObject) {
        match param {
            CustomPaletteParam::Background => todo!(),
        }
    }
}

// Widget	    Statuses
// Button	    Active, Hovered, Pressed, Disabled
// Checkbox	    Active {is_checked}, Hovered {is_checked}, Disabled {is_checked}
// PickList	    Active, Hovered, Opened {..}, Disabled
// Radio	    Active {..}, Hovered {..}
// Sash	        Active, Hovered, Dragged, Disabled
// Scrollable	Passes the full scrollable::Status through — no explicit status match, applies to rails/scroller directly
// Slider	    Active, Hovered, Dragged
// TextEditor	Active, Hovered, Focused {..}, Disabled
// TextInput	Active, Hovered, Focused {..}, Disabled
// Toggler	    Active {is_toggled}, Hovered {is_toggled}, Disabled {is_toggled}