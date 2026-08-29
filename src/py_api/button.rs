//! Button module - provides add_button pyfunction

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::{colors::Color, bootstrap::bootstrap_arrow::Arrow};
use crate::state::{Widgets, access_state, add_callback_name_to_mutex, get_id, set_state_of_widget};
use crate::widgets::callbacks::CallbackName;
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
/// disabled: bool, Optional
///     Whether the button is disabled.
/// font_id : int, Optional
///     Sets the ID of a custom style created with ``add_font_style``.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_button_style``.
/// style_std : ButtonStyleStd,  Optional
///     Sets the a predefined standard style variant.
/// style_arrow : Arrow,  Optional
///     Sets an arrow icon style for the button.
/// palette_id : int, Optional
///     Sets the ID of a custom palette created with ``custom_palette``.
/// user_data : Any,  Optional
///     Sets an arbitrary data forwarded to callbacks.
/// active: bool, Optional
///     Whether to set the button status as Active.
/// hovered: bool, Optional
///     Whether to set the button status as Hovered.
/// pressed: bool, Optional
///     Whether to set the button status as Pressed.
/// disabled: bool, Optional
///     Whether to set the button status as Disabled.
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
    font_id=None,
    style_id=None,
    style_std=None,
    style_arrow=None,
    palette_id=None,
    user_data=None,
    active=None,
    hovered=None,
    pressed=None,
    disabled=None,
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
    font_id: Option<usize>,
    style_id: Option<usize>,
    style_std: Option<ButtonStyleStd>,
    style_arrow: Option<Arrow>,
    palette_id: Option<usize>,
    user_data: Option<PyObject>,
    active: Option<bool>,
    hovered: Option<bool>,
    pressed: Option<bool>,
    disabled: Option<bool>,
    show: bool,
    gen_id: Option<usize>,
) -> PyResult<usize> {

    let id = get_id(gen_id);
    
    // Register widget with parent
    set_state_of_widget(id, parent_id.clone());

    // Store callback if provided
    if let Some(py) = on_press {
        add_callback_name_to_mutex(id, CallbackName::OnPress, py);
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
                font_id,
                style_id,
                style_std,
                style_arrow,
                palette_id,
                show,
                active,
                hovered,
                pressed,
                disabled,
            }),
        );
    drop(state);

    Ok(id)
}


/// """
/// Adds styling to a button
///
/// There are 5 ways to add styling to a widget.  Widgets vary a bit
/// because some have statues like checked, pressed, etc.
/// 1. Do nothing and the widgets default to one of the standard styles
///    which is usually primary but varies some from widget to widget.
/// 2. You select one of the available standard styles (see below for button)
/// 3. You create your own palette (see the palette example which gives a full explanation)
/// 4. You override the standard style colors.  Maybe you want a different background
///    color for danger.
/// 5. You add styling that is not set by any of the standard styles like gradient, 
///    shadow, text parameters, border width and color.  These are listed below.
/// 
/// The standard style colors will vary depending on the theme selected
/// for the window. This makes sure that there is enough contrast between the
/// colors and the theme background to be seen.
/// 
/// Standard styles are:
/// Background, Danger, Primary, Secondary,
/// Subtle, Success, Warning, Text,
///
/// Each widget has a status where the color changes based on the state of the widget.
/// The status type will vary among the widgets due of the actions they produce.  
/// For example, a button has a status of pressed and a checkbox has a status of checked.
/// 
/// The statuses are:
/// Active, Hovered, Pressed, Disabled
///
/// gradient — applied to the background instead of a solid color:
///   gradient_color_stops: list[Color, ≤8]
///   gradient_color_alpha_stops: list[float]
///   gradient_rgba_stops: list[list[float, 4], ≤8]
///     Up to 8 stops total (color + rgba combined).
///   gradient_offset_stops: list[float]
///     Offset positions for each stop (0.0–1.0).
///   gradient_degrees: float — angle in degrees
///   gradient_radians: float — angle in radians (takes priority over degrees)
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

    border_color = None,
    border_color_alpha = None,
    border_rgba = None,
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

    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
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

            border_color,
            border_color_alpha,
            border_rgba,
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
