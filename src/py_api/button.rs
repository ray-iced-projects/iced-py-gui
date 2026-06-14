//! Button module - provides add_button pyfunction

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::{colors::Color, bootstrap::bootstrap_arrow::Arrow};
use crate::state::{Widgets, access_state, add_callback_to_mutex, 
    get_id, set_state_of_widget};
use crate::widgets::ipg_button::{Button,  
    ButtonStyle, ButtonStyleStd};


/// Adds a button widget.
/// A clickable button used for some gui action.
///  
/// Parameters
/// ----------
/// parent_id : str, Optional
///     Sets the parent container ID that this button belongs to.
/// label : str,  Optional
///     Sets the Text label displayed on the button.
/// label_style_id: int, Optional
///     Makes the label a text object that can be styled
/// on_press : callable,  Optional
///     Sets the Callback method to invoke when the button is pressed.
/// width : float,  Optional
///     Sets the Fixed Width in logical pixels.
/// width_fill : bool, default False
///     Whether the button fills available width.
/// height : float,  Optional
///     Sets the Fixed Height in logical pixels.
/// height_fill : bool, default False
///     Whether the button fills available height.
/// fill : bool, Optional
///     Whether the button fills available width and height.
/// padding : list of float,  Optional
///     Sets the Padding as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// clip : bool,  Optional
///     Whether to clip content that overflows the button.
/// style_id : int,  Optional
///     Stes the ID of a custom style created with ``add_button_style``.
/// style_std : ButtonStyleStd,  Optional
///     Sets the a predefined standard style variant.
/// style_arrow : Arrow,  Optional
///     Sets an arrow icon style for the button.
/// user_data : Any,  Optional
///     Sets an arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the button is visible.
/// gen_id : int,  Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// Returns
/// -------
/// int
///    The numeric widget ID of the newly created button.
#[pyfunction]
#[pyo3(signature = (
    parent_id,
    label=None,
    on_press=None,
    width=None,
    width_fill=None,
    height=None,
    height_fill=None,
    fill=None,
    padding=None,
    clip=None,
    status_active=None,
    status_hovered=None,
    status_pressed=None,
    status_disabled=None,
    font_id=None,
    style_id=None,
    style_std=None,
    style_arrow=None,
    palette_id=None,
    user_data=None,
    show=true,
    gen_id=None,
))]
pub fn add_button(
    parent_id: String,
    label: Option<String>,
    on_press: Option<PyObject>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    padding: Option<Vec<f32>>,
    clip: Option<bool>,
    status_active: Option<bool>,
    status_hovered: Option<bool>,
    status_pressed: Option<bool>,
    status_disabled: Option<bool>,
    font_id: Option<usize>,
    style_id: Option<usize>,
    style_std: Option<ButtonStyleStd>,
    style_arrow: Option<Arrow>,
    palette_id: Option<usize>,
    user_data: Option<PyObject>,
    show: bool,
    gen_id: Option<usize>,
) -> PyResult<usize> {

    let id = get_id(gen_id);
    
    // Register widget with parent
    set_state_of_widget(id, parent_id.clone());

    // Store callback if provided
    if let Some(py) = on_press {
        add_callback_to_mutex(id, "on_press".to_string(), py);
    }

    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    // Create and store button
    let mut state = access_state();
    state.widgets.insert(
        id,
        Widgets::Button(
            Button {
                id,
                label,
                width,
                width_fill,
                height,
                height_fill,
                fill,
                padding,
                clip,
                status_active,
                status_hovered,
                status_pressed,
                status_disabled,
                font_id,
                style_id,
                style_std,
                style_arrow,
                palette_id,
                show,
            }),
        );
    drop(state);

    Ok(id)
}


/// """
/// Adds styling to a button
///
/// The standard styles for the button are defined below which gives
/// you the approach to developing your own styles, if wanted.
/// 
/// if you want to produce your own styles from a new background,
/// then you will need to define the new background and optionally 
/// a text color.  The text color can be auto generated or defined.
/// 
/// You also have the ability to define all the colors individually or
/// define an active color which replaces all the colors for that parameter.
/// 
/// Below are the settings for the current styles.
/// 
/// Standard styles are:
/// Background,
/// Danger,
/// Primary,
/// Secondary,
/// Subtle (unique settings),
/// Success,
/// Warning,
/// Text,
///
/// Status    |  Standard Styles
/// Active    |  base
/// Hovered   |  strong
/// Pressed   |  base
/// Disabled  |  base => background scale_alpha(0.5)
///
/// Status    |  Text button
/// Active    |  base
/// Hovered   |  base text scale alpha(0.8)
/// Pressed   |  base
/// Disabled  |  base => background scale_alpha(0.5)
///
/// Status    |  Background Custom Colors
/// Active    |  base
/// Hovered   |  weak
/// Pressed   |  strong
/// Disabled  |  base => background scale_alpha(0.5)
///
/// Status    |  Standard Style Subtle (unique)
/// Active    |  base
/// Hovered   |  strong
/// Pressed   |  base
/// Disabled  |  base => background scale_alpha(0.5)
/// 
/// Parameters
/// ----------
/// **Color triplet** — Each color group accepts one of three formats:
///   <name>_color: Color         — named Color enum value
///   <name>_alpha: float         — alpha multiplier applied to _color (0.0–1.0)
///   <name>_rgba: list[float, 4] — raw [r, g, b, a] (takes priority over color + alpha)
///
/// bkg (background):
///   bkg_color, bkg_color_alpha, bkg_rgba
///     Background color. Defaults to the primary theme color.
///
/// text (global — all statuses inherit from this unless per-status override is set):
///   text_color, text_color_alpha, text_rgba
///     Overall text color. Defaults to the text color paired with the background.
///
/// text per-status (overrides the global text color for that status only):
///   active:   text_color_active,   text_color_alpha_active,   text_rgba_active
///   hovered:  text_color_hovered,  text_color_alpha_hovered,  text_rgba_hovered
///   pressed:  text_color_pressed,  text_color_alpha_pressed,  text_rgba_pressed
///   disabled: text_color_disabled, text_color_alpha_disabled, text_rgba_disabled
///
/// border per-status (defaults to the background color for that status):
///   active:   border_color_active,   border_color_alpha_active,   border_rgba_active
///   hovered:  border_color_hovered,  border_color_alpha_hovered,  border_rgba_hovered
///   pressed:  border_color_pressed,  border_color_alpha_pressed,  border_rgba_pressed
///   disabled: border_color_disabled, border_color_alpha_disabled, border_rgba_disabled
///
/// gradient — applied to the background instead of a solid color:
///   gradient_color_stops: list[Color, ≤8]
///   gradient_color_alpha_stops: list[float]
///   gradient_rgba_stops: list[list[float, 4], ≤8]
///     Up to 8 stops total (color + rgba combined).
///   gradient_offset_stops: list[float]
///     Offset positions for each stop (0.0–1.0).
///   gradient_degrees: float   — angle in degrees
///   gradient_radians: float   — angle in radians (takes priority over degrees)
///
/// shadow:
///   shadow_color, shadow_color_alpha, shadow_rgba
///     Shadow color.
///   shadow_offset_xy: list[float, 2]
///     Shadow offset [x, y].
///   shadow_blur_radius: float
///     Shadow blur radius.
///
/// text alignment (default: center):
///   text_top_left, text_top_center, text_top_right: bool
///   text_center_left, text_center, text_center_right: bool
///   text_bottom_left, text_bottom_center, text_bottom_right: bool
///
/// text_size: float — font size for the label
///
/// text wrapping (default: Word):
///   wrapping_none: bool — no wrapping
///   wrapping_glyph: bool — wrap at glyph boundary
///   wrapping_word_glyph: bool — wrap at word, fall back to glyph
///
/// border_radius: list[float] — [all] or [top-left, top-right, bottom-right, bottom-left]
/// border_width: float — border line width
///
/// snap: bool — snap rendering to pixel grid
///
/// gen_id: int, Optional
///     Obtain an ID for a widget not yet created; use for the gen_id parameter.
/// """
#[pyfunction]
#[pyo3(signature = (
        text_top_left = None,
        text_top_center = None,
        text_top_right = None,
        text_center_left = None,
        text_center = None,
        text_center_right = None,
        text_bottom_left = None,
        text_bottom_center = None,
        text_bottom_right = None,
        text_size = None,

        wrapping_none = None,
        wrapping_glyph = None,
        wrapping_word_glyph = None,

        gradient_color_stops = None,
        gradient_color_alpha_stops = None,
        gradient_rgba_stops = None,
        gradient_offset_stops = None,
        gradient_degrees = None,
        gradient_radians = None,

        border_radius = None,
        border_width = None,

        shadow_color = None,
        shadow_color_alpha = None,
        shadow_rgba = None,
        shadow_offset_xy = None,
        shadow_blur_radius = None,

        snap = None,
        gen_id=None
        ))]
pub fn add_button_style(
    text_top_left: Option<bool>,
    text_top_center: Option<bool>,
    text_top_right: Option<bool>,
    text_center_left: Option<bool>,
    text_center: Option<bool>,
    text_center_right: Option<bool>,
    text_bottom_left: Option<bool>,
    text_bottom_center: Option<bool>,
    text_bottom_right: Option<bool>,
    text_size: Option<f32>,

    wrapping_none: Option<bool>,
    wrapping_glyph: Option<bool>,
    wrapping_word_glyph: Option<bool>,

    gradient_color_stops: Option<Vec<Option<Color>>>,
    gradient_color_alpha_stops: Option<Vec<Option<f32>>>,
    gradient_rgba_stops: Option<Vec<Option<[f32; 4]>>>,
    gradient_offset_stops: Option<Vec<Option<f32>>>,
    gradient_degrees: Option<f32>,
    gradient_radians: Option<f32>,

    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,

    shadow_color: Option<Color>,
    shadow_color_alpha: Option<f32>,
    shadow_rgba: Option<[f32; 4]>,
    shadow_offset_xy: Option<[f32; 2]>,
    shadow_blur_radius: Option<f32>,

    snap: Option<bool>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::ButtonStyle(
        ButtonStyle {
            id,
            text_top_left,
            text_top_center,
            text_top_right,
            text_center_left,
            text_center,
            text_center_right,
            text_bottom_left,
            text_bottom_center,
            text_bottom_right,
            text_size,

            wrapping_none,
            wrapping_glyph,
            wrapping_word_glyph,

            gradient_color_stops,
            gradient_color_alpha_stops,
            gradient_rgba_stops,
            gradient_offset_stops,
            gradient_degrees,
            gradient_radians,

            border_radius,
            border_width,

            shadow_color,
            shadow_color_alpha,
            shadow_rgba,
            shadow_offset_xy,
            shadow_blur_radius,

            snap,
        }));

    drop(state);
    Ok(id)
}
