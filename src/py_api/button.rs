//! Button module - provides add_button pyfunction

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::{colors::Color, bootstrap_arrow::Arrow};
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
/// text_top_left : bool,  Optional
///     Whether to Align the label to the top-left.
/// text_top_center : bool,  Optional
///     Whether to Align the label to the top-centre.
/// text_top_right : bool,  Optional
///     Whether to Align the label to the top-right.
/// text_center_left : bool,  Optional
///     Whether to Align the label to the centre-left.
/// text_center : bool,  Optional
///     Whether to Align the label to the centre (default True).
/// text_center_right : bool,  Optional
///     Whether to Align the label to the centre-right.
/// text_bottom_left : bool,  Optional
///     Whether to Align the label to the bottom-left.
/// text_bottom_center : bool,  Optional
///     Whether to Align the label to the bottom-centre.
/// text_bottom_right : bool,  Optional
///     Whether to Align the label to the bottom-right.
/// text_size : float,  Optional
///     Sets the Font size for the label text.
///  if_menu_btn: bool, Optional
///      Whether the button is used in the menu widget, effects the alignment.
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
    text_top_left=None,
    text_top_center=None,
    text_top_right=None,
    text_center_left=None,
    text_center=true,
    text_center_right=None,
    text_bottom_left=None,
    text_bottom_center=None,
    text_bottom_right=None,
    text_size=None,
    if_menu_btn=None,
    clip=None,
    style_id=None,
    style_std=None,
    style_arrow=None,
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
    if_menu_btn: Option<bool>,
    clip: Option<bool>,
    style_id: Option<usize>,
    style_std: Option<ButtonStyleStd>,
    style_arrow: Option<Arrow>,
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
                show,
                label,
                width,
                width_fill,
                height,
                height_fill,
                fill,
                padding,
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
                if_menu_btn,
                clip,
                style_id,
                style_std,
                style_arrow,
            }),
        );
    drop(state);

    Ok(id)
}


///"""
///Adds styling to a button
///
/// The style are keyed to a background, text, primary, and secondary colors
/// Each color generates a weak, strong, etc. type
/// 
/// if you want to produce your own colors from a new background,
/// then you will need to define the new background, text, primary, 
/// and secondary colors.  Based on these, the color types will be 
/// generated for you based on the widget status.
/// 
/// You also have the ability to define all the colors individually or
/// define an active color which replaces all the colors for that parameter.
/// 
/// Active: background: palette.background.weakest.color
///         border: background.strong.color
/// 
/// pressed: same as Active except
///          palette.background.strong.color
/// 
/// Hovered: same as Active except
///          background: palette.background.weaker.color
///          border: background.base.text
/// 
/// Disabled: same as primary except
///           background: palette.background.weakest.color
///                         with bkg_color_alpha = 0.5
///           text: background.base.text with alpha=0.5
///Parameters
///----------
///background_color: Color, Optional
///    Sets the Color of the background.
///background_color_alpha: float, Optional
///    Sets the alpha of the Color.
///background_rgba: list[float, 4], Optional
///    Sets the Color of the background in rgba format.
///gradient_color_stops: list[Color, 8], Optional
///    Sets the stop Color of the background gradient.
///    A total of 8 stops allowed counting the rgba also.
///gradient_color_alpha_stops: list[float], Optional
///    Sets the alpha of the Color.
///gradient_rgba_stops: list[list[float, 4], 8], Optional
///    Sets the stops rgba color of the background gradient.
///    A total of 8 stops allowed counting the colors also.
///gradient_offset_stops: list[float], Optional
///    The offsets for the gradient stops.
///gradient_degrees: float, Optional,
///    Sets the gradient degrees
///gradient_radians: float, Optional,
///    Sets the gradient radians
///border_color: Color, Optional
///    Sets the Color used for the border.
///border_color_alpha: float, Optional
///    Sets the alpha of the Color.
///border_rgba: list[float, 4], Optional
///    Sets the Color of the border in rgba format.
///border_radius: list[float | float, 4], Optional
///    Sets the radius of the border, [float]=all corners, 
///    [float]=[top-left, top-right, bottom-right, bottom-left].
///border_width: float, Optional
///    Sets the border width.
///shadow_color: Color, Optional
///    Sets the color of the shadow.
///shadow_color_alpha: float, Optional
///    Sets the alpha of the Color.
///shadow_rgba: list[float, 4], Optional
///    Sets the color in rgba format.
///shadow_offset_xy: list[float, 2], Optional
///    Sets the Shadow offset in the horizontal direction [x, y].
///shadow_blur_radius: float, Optional
///    Sets the blur radius of the shadow.
///text_color: Color, Optional
///    Sets the text color.
///text_color_alpha: float, Optional
///    Sets the alpha of the Color.
///text_rgba: list[float], Optional
///    Sets the color in rgba used as state above, 4 values.
///gen_id : int,  Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///"""
#[pyfunction]
#[pyo3(signature = (
        background_color = None,
        background_color_alpha = None,
        background_rgba = None,

        text_color = None,
        text_color_alpha = None,
        text_rgba = None,

        text_color_active = None,
        text_color_alpha_active = None,
        text_rgba_active = None,

        text_color_hovered = None,
        text_color_alpha_hovered = None,
        text_rgba_hovered = None,

        text_color_pressed = None,
        text_color_alpha_pressed = None,
        text_rgba_pressed = None,

        text_color_disabled = None,
        text_color_alpha_disabled = None,
        text_rgba_disabled = None,

        gradient_color_stops = None,
        gradient_color_alpha_stops = None,
        gradient_rgba_stops = None,
        gradient_offset_stops = None,
        gradient_degrees = None,
        gradient_radians = None,

        border_color_active = None,
        border_color_alpha_active = None,
        border_rgba_active = None,

        border_color_hovered = None,
        border_color_alpha_hovered = None,
        border_rgba_hovered = None,

        border_color_pressed = None,
        border_color_alpha_pressed = None,
        border_rgba_pressed = None,

        border_color_disabled = None,
        border_color_alpha_disabled = None,
        border_rgba_disabled = None,

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
    background_color: Option<Color>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,

    text_color: Option<Color>,
    text_color_alpha: Option<f32>,
    text_rgba: Option<[f32; 4]>,

    text_color_active: Option<Color>,
    text_color_alpha_active: Option<f32>,
    text_rgba_active: Option<[f32; 4]>,

    text_color_hovered: Option<Color>,
    text_color_alpha_hovered: Option<f32>,
    text_rgba_hovered: Option<[f32; 4]>,

    text_color_pressed: Option<Color>,
    text_color_alpha_pressed: Option<f32>,
    text_rgba_pressed: Option<[f32; 4]>,

    text_color_disabled: Option<Color>,
    text_color_alpha_disabled: Option<f32>,
    text_rgba_disabled: Option<[f32; 4]>,

    gradient_color_stops: Option<Vec<Option<Color>>>,
    gradient_color_alpha_stops: Option<Vec<Option<f32>>>,
    gradient_rgba_stops: Option<Vec<Option<[f32; 4]>>>,
    gradient_offset_stops: Option<Vec<Option<f32>>>,
    gradient_degrees: Option<f32>,
    gradient_radians: Option<f32>,

    border_color_active: Option<Color>,
    border_color_alpha_active: Option<f32>,
    border_rgba_active: Option<[f32; 4]>,

    border_color_hovered: Option<Color>,
    border_color_alpha_hovered: Option<f32>,
    border_rgba_hovered: Option<[f32; 4]>,

    border_color_pressed: Option<Color>,
    border_color_alpha_pressed: Option<f32>,
    border_rgba_pressed: Option<[f32; 4]>,

    border_color_disabled: Option<Color>,
    border_color_alpha_disabled: Option<f32>,
    border_rgba_disabled: Option<[f32; 4]>,

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
            background_color,
            background_color_alpha,
            background_rgba,

            text_color,
            text_color_alpha,
            text_rgba,

            text_color_active,
            text_color_alpha_active,
            text_rgba_active,

            text_color_hovered,
            text_color_alpha_hovered,
            text_rgba_hovered,

            text_color_pressed,
            text_color_alpha_pressed,
            text_rgba_pressed,

            text_color_disabled,
            text_color_alpha_disabled,
            text_rgba_disabled,

            gradient_color_stops,
            gradient_color_alpha_stops,
            gradient_rgba_stops,
            gradient_offset_stops,
            gradient_degrees,
            gradient_radians,

            border_color_active,
            border_color_alpha_active,
            border_rgba_active,

            border_color_hovered,
            border_color_alpha_hovered,
            border_rgba_hovered,

            border_color_pressed,
            border_color_alpha_pressed,
            border_rgba_pressed,

            border_color_disabled,
            border_color_alpha_disabled,
            border_rgba_disabled,

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
