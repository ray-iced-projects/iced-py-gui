//! Colors module - provides get_rgba_color and get_color_palette pyfunctions

use std::collections::HashMap;

use iced::Theme;
use iced::theme::palette::{self, readable};
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
    theme_name,
    color=None, 
    rgba=None,
    color_alpha=None))]
pub fn get_color_palette(
    theme_name: String,
    color: Option<Color>,
    rgba: Option<[f32; 4]>,
    color_alpha: Option<f32>,
) -> PyResult<HashMap<PaletteKey, [f64; 4]>>
{
    let base = Color::rgba_ipg_color_to_iced(rgba, &color, color_alpha)
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(
            "get_color_palette: no color supplied — provide base_color or base_rgba"
        ))?;

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

    Ok(color_palette(&theme, base))
    
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

    // let bkg_pal = color_palette(pal.background.base.color);
    
    // hm.extend(bkg_pal);

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

fn color_palette(theme: &iced::Theme, base: iced::Color) -> HashMap<PaletteKey, [f64; 4]> {
    let text_color = readable(base, iced::Color::WHITE);
    let color_pal = iced::theme::palette::Background::new(base, text_color);
    let color_theme = theme.palette().background;

    fn to_arr(c: iced::Color) -> [f64; 4] {
        let r = |v: f32| ((v as f64) * 100.0).round() / 100.0;
        [r(c.r), r(c.g), r(c.b), r(c.a)]
    }
    
    let mut map = HashMap::new();
    
    map.insert(PaletteKey::Base,     to_arr(color_pal.base.color));
    map.insert(PaletteKey::BaseText, to_arr(color_pal.base.text));
    map.insert(PaletteKey::Weak,     to_arr(color_pal.weak.color));
    map.insert(PaletteKey::WeakText, to_arr(color_pal.weak.text));
    map.insert(PaletteKey::Weaker,   to_arr(color_pal.weaker.color));
    map.insert(PaletteKey::WeakerText, to_arr(color_pal.weaker.text));
    map.insert(PaletteKey::Weakest,  to_arr(color_pal.weakest.color));
    map.insert(PaletteKey::WeakestText, to_arr(color_pal.weakest.text));
    map.insert(PaletteKey::Neutral,  to_arr(color_pal.neutral.color));
    map.insert(PaletteKey::NeutralText,  to_arr(color_pal.neutral.text));
    map.insert(PaletteKey::Strong,   to_arr(color_pal.strong.color));
    map.insert(PaletteKey::StrongText, to_arr(color_pal.strong.text));
    map.insert(PaletteKey::Stronger, to_arr(color_pal.stronger.color));
    map.insert(PaletteKey::StrongerText, to_arr(color_pal.stronger.text));
    map.insert(PaletteKey::Strongest, to_arr(color_pal.strongest.color));
    map.insert(PaletteKey::StrongestText, to_arr(color_pal.strongest.text));

    map.insert(PaletteKey::ThemeBase,     to_arr(color_theme.base.color));
    map.insert(PaletteKey::ThemeBaseText, to_arr(color_theme.base.text));
    map.insert(PaletteKey::ThemeWeak,     to_arr(color_theme.weak.color));
    map.insert(PaletteKey::ThemeWeakText, to_arr(color_theme.weak.text));
    map.insert(PaletteKey::ThemeWeaker,   to_arr(color_theme.weaker.color));
    map.insert(PaletteKey::ThemeWeakerText, to_arr(color_theme.weaker.text));
    map.insert(PaletteKey::ThemeWeakest,  to_arr(color_theme.weakest.color));
    map.insert(PaletteKey::ThemeWeakestText, to_arr(color_theme.weakest.text));
    map.insert(PaletteKey::ThemeNeutral,  to_arr(color_theme.neutral.color));
    map.insert(PaletteKey::ThemeNeutralText,  to_arr(color_theme.neutral.text));
    map.insert(PaletteKey::ThemeStrong,   to_arr(color_theme.strong.color));
    map.insert(PaletteKey::ThemeStrongText, to_arr(color_theme.strong.text));
    map.insert(PaletteKey::ThemeStronger, to_arr(color_theme.stronger.color));
    map.insert(PaletteKey::ThemeStrongerText, to_arr(color_theme.stronger.text));
    map.insert(PaletteKey::ThemeStrongest, to_arr(color_theme.strongest.color));
    map.insert(PaletteKey::ThemeStrongestText, to_arr(color_theme.strongest.text));
    

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
    statuses: Option<Vec<((WidgetStatus, StateVariant), Vec<(StylePart, PaletteKey, f32)>)>>,
    gen_id: Option<usize>,
) -> PyResult<usize>
{
    let base = Color::rgba_ipg_color_to_iced(rgba, &color, None)
        .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(
            "get_color_palette: no color supplied — provide base_color or base_rgba"
        ))?;
        
    let text_color = readable(base, iced::Color::WHITE);
    let palette = iced::theme::palette::Background::new(base, text_color);
  
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::Palette(
        CustomPalette {
            id,
            palette,
            statuses: statuses.map(|v| v.into_iter().collect()),
        }));

    drop(state);
    Ok(id)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum StateVariant {
    NoVariant,
    Checked,
    Unchecked,
}

#[derive(Debug, Clone)]
pub struct CustomPalette{
    pub id: usize,
    pub palette: iced::theme::palette::Background,
    pub statuses: Option<HashMap<(WidgetStatus, StateVariant), Vec<(StylePart, PaletteKey, f32)>>>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum CustomPaletteParam {
    Background,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum PaletteKey {
    Transparent,
    Base,
    BaseText,
    Neutral,
    NeutralText,
    Strong,
    StrongText,
    Stronger,
    StrongerText,
    Strongest,
    StrongestText,
    Weak,
    WeakText,
    Weaker,
    WeakerText,
    Weakest,
    WeakestText,
    ThemeBase,
    ThemeBaseText,
    ThemeNeutral,
    ThemeNeutralText,
    ThemeStrong,
    ThemeStrongText,
    ThemeStronger,
    ThemeStrongerText,
    ThemeStrongest,
    ThemeStrongestText,
    ThemeWeak,
    ThemeWeakText,
    ThemeWeaker,
    ThemeWeakerText,
    ThemeWeakest,
    ThemeWeakestText,
}

impl PaletteKey {
    pub fn pal_key_to_color(&self, theme: &palette::Background, 
                            color: &palette::Background) -> iced::Color {
        match self {
            PaletteKey::Transparent => iced::Color::TRANSPARENT,
            PaletteKey::Base => color.base.color,
            PaletteKey::BaseText => color.base.text,
            PaletteKey::Neutral => color.neutral.color,
            PaletteKey::NeutralText => color.neutral.text,
            PaletteKey::Strong => color.strong.color,
            PaletteKey::StrongText => color.strong.text,
            PaletteKey::Stronger => color.stronger.color,
            PaletteKey::StrongerText => color.stronger.text,
            PaletteKey::Strongest => color.strongest.color,
            PaletteKey::StrongestText => color.strongest.text,
            PaletteKey::Weak => color.weak.color,
            PaletteKey::WeakText => color.weak.text,
            PaletteKey::Weaker => color.weaker.color,
            PaletteKey::WeakerText => color.weaker.text,
            PaletteKey::Weakest => color.weakest.color,
            PaletteKey::WeakestText => color.weakest.text,
            
            PaletteKey::ThemeBase => theme.base.color,
            PaletteKey::ThemeBaseText => theme.base.text,
            PaletteKey::ThemeNeutral => theme.neutral.color,
            PaletteKey::ThemeNeutralText => theme.neutral.text,
            PaletteKey::ThemeStrong => theme.strong.color,
            PaletteKey::ThemeStrongText => theme.strong.text,
            PaletteKey::ThemeStronger => theme.stronger.color,
            PaletteKey::ThemeStrongerText => theme.stronger.text,
            PaletteKey::ThemeStrongest => theme.strongest.color,
            PaletteKey::ThemeStrongestText => theme.strongest.text,
            PaletteKey::ThemeWeak => theme.weak.color,
            PaletteKey::ThemeWeakText => theme.weak.text,
            PaletteKey::ThemeWeaker => theme.weaker.color,
            PaletteKey::ThemeWeakerText => theme.weaker.text,
            PaletteKey::ThemeWeakest => theme.weakest.color,
            PaletteKey::ThemeWeakestText => theme.weakest.text,
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