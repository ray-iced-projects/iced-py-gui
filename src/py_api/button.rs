//! Button module - provides add_button pyfunction

use iced::Color;
use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::{colors::IpgColor, bootstrap_arrow::IpgArrow};
use crate::py_api::helpers::get_length;
use crate::state::{IpgWidgets, access_state, add_callback_to_mutex, 
    get_id, set_state_of_widget};
use crate::widgets::ipg_button::{IpgButton,  
    IpgButtonStyle, IpgButtonStyleStd};


///"""
///Adds a button widget.
///A clickable button used for some gui action.
/// 
///Parameters
///----------
///parent_id : str, Optional
///    Sets the parent container ID that this button belongs to.
///label : str,  Optional
///    Sets the Text label displayed on the button.
///on_press : callable,  Optional
///    Sets the Callback method to invoke when the button is pressed.
///width : float,  Optional
///    Sets the Fixed Width in logical pixels.
///height : float,  Optional
///    Sets the Fixed Height in logical pixels.
///width_fill : bool, default False
///    Whether the button fills available width.
///height_fill : bool, default False
///    Whether the button fills available height.
///padding : list of float,  Optional
///    Sets the Padding as [all], [vertical, horizontal], or
///    [top, right, bottom, left].
///text_top_left : bool,  Optional
///    Whether to Align the label to the top-left.
///text_top_center : bool,  Optional
///    Whether to Align the label to the top-centre.
///text_top_right : bool,  Optional
///    Whether to Align the label to the top-right.
///text_center_left : bool,  Optional
///    Whether to Align the label to the centre-left.
///text_center : bool,  Optional
///    Whether to Align the label to the centre (default True).
///text_center_right : bool,  Optional
///    Whether to Align the label to the centre-right.
///text_bottom_left : bool,  Optional
///    Whether to Align the label to the bottom-left.
///text_bottom_center : bool,  Optional
///    Whether to Align the label to the bottom-centre.
///text_bottom_right : bool,  Optional
///    Whether to Align the label to the bottom-right.
///text_size : float,  Optional
///    Sets the Font size for the label text.
///clip : bool,  Optional
///    Whether to clip content that overflows the button.
///style_id : int,  Optional
///    Stes the ID of a custom style created with ``add_button_style``.
///style_std : IpgButtonStyleStd,  Optional
///    Sets the a predefined standard style variant.
///style_arrow : IpgArrow,  Optional
///    Sets an arrow icon style for the button.
///user_data : Any,  Optional
///    Sets an arbitrary data forwarded to callbacks.
///show : bool, default True
///    Whether the button is visible.
///gen_id : int,  Optional
///    Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///Returns
///-------
///int
///    The numeric widget ID of the newly created button.
///"""
#[pyfunction]
#[pyo3(signature = (
    parent_id,
    label=None,
    on_press=None,
    width=None,
    width_fill=false,
    height=None,
    height_fill=false,
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
    menu=None,
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
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
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
    menu: Option<bool>,
    clip: Option<bool>,
    style_id: Option<usize>,
    style_std: Option<IpgButtonStyleStd>,
    style_arrow: Option<IpgArrow>,
    user_data: Option<PyObject>,
    show: bool,
    gen_id: Option<usize>,
) -> PyResult<usize> {

    let id = get_id(gen_id);
    
    // Calculate dimensions
    let width = get_length(width, width_fill);
    let height = get_length(height, height_fill);

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
        IpgWidgets::IpgButton(
            IpgButton {
                id,
                parent_id,
                show,
                label,
                width,
                height,
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
                menu,
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
///Parameters
///----------
///background_color: IpgColor, Optional
///    Sets the Color of the background.
///background_color_alpha: float, Optional
///    Sets the alpha of the IpgColor.
///background_rgba: list, Optional
///    Sets the Color of the background in rgba format, 4 values.
/// background_gradient_color_stop: IpgColor, Optional
///    Sets the stop Color of the background gradient.
/// background_gradient_color_stop_alpha: float, Optional
///    Sets the alpha of the IpgColor.
/// background_gradient_rgba_stop: list, Optional
///    Sets the stop rgba color of the background gradient, 4 values.
/// background_gradient_degrees: float, Optional,
///    Sets the gradient degrees
/// background_gradient_radians: float, Optional,
///    Sets the gradient radians
/// background_gradient_alpha: float, Optional,
///    Sets the alpha color parameter.
///border_color: IpgColor, Optional
///    Sets the Color used for the border.
///border_color_alpha: float, Optional
///    Sets the alpha of the IpgColor.
///border_rgba: list[float], Optional
///    Sets the Color of the border in rgba format, 4 values.
///border_radius: list[float], Optional
///    Sets the radius of the border, [float]=all corners, 
///    [float]=[top-left, top-right, bottom-right, bottom-left].
///border_width: float, Optional
///    Sets the border width.
///shadow_color: IpgColor, Optional
///    Sets the color of the shadow.
///shadow_color_alpha: float, Optional
///    Sets the alpha of the IpgColor.
///shadow_rgba: list[float], Optional
///    Sets the color in rgba format [float; 4] used as state above.
///shadow_offset_xy: list[float], Optional
///    Sets the Shadow offset in the horizontal direction [x, y].
///shadow_blur_radius: float, Optional
///    Sets the blur radius of the shadow.
///text_color: IpgColor, Optional
///    Sets the text color, if not defined, will either be a Black or White variation based on theme background.
///text_color_alpha: float, Optional
///    Sets the alpha of the IpgColor.
///text_rgba: list[float], Optional
///    Sets the color in rgba used as state above, 4 values.
/// gen_id : int,  Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///"""
#[pyfunction]
#[pyo3(signature = (
        background_color=None,
        background_color_alpha=None, 
        background_rgba=None,
        background_gradient_color_stop=None,
        background_gradient_color_stop_alpha=None,
        background_gradient_rgba_stop=None,
        background_gradient_degrees=None,
        background_gradient_radians=None,
        border_color=None,
        border_color_alpha=None,
        border_rgba=None,
        border_radius=None, 
        border_width=1.0,
        shadow_color=None,
        shadow_color_alpha=None,
        shadow_rgba=None,
        shadow_offset_xy=None, 
        shadow_blur_radius=None,
        text_color=None,
        text_color_alpha=None,
        text_rgba=None,
        gen_id=None
        ))]
pub fn add_button_style(
    background_color: Option<IpgColor>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    background_gradient_color_stop: Option<IpgColor>,
    background_gradient_color_stop_alpha: Option<f32>,
    background_gradient_rgba_stop: Option<[f32; 4]>,
    background_gradient_degrees: Option<f32>,
    background_gradient_radians: Option<f32>,
    border_color: Option<IpgColor>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    shadow_color: Option<IpgColor>,
    shadow_color_alpha: Option<f32>,
    shadow_rgba: Option<[f32; 4]>,
    shadow_offset_xy: Option<[f32; 2]>,
    shadow_blur_radius: Option<f32>,
    text_color: Option<IpgColor>,
    text_color_alpha: Option<f32>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, background_color_alpha);
    let background_gradient_color_stop: Option<Color> =
        IpgColor::rgba_ipg_color_to_iced(background_gradient_rgba_stop, 
            background_gradient_color_stop, background_gradient_color_stop_alpha);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, border_color_alpha);
    let shadow_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(shadow_rgba, shadow_color, shadow_color_alpha);
    let text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, text_color_alpha);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgButtonStyle(
        IpgButtonStyle {
            id,
            background_color,
            background_gradient_color_stop,
            background_gradient_degrees,
            background_gradient_radians,
            border_color,
            border_radius,
            border_width,
            shadow_color,
            shadow_offset_xy,
            shadow_blur_radius,
            text_color,
        }));

    drop(state);
    Ok(id)
}
