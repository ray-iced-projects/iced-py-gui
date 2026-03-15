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
///Add a button widget.
///A clickable button used for some gui action.
/// 
///Parameters
///----------
///parent_id : str, Optional
///    The parent container ID that this button belongs to.
///label : str, optional
///    Text label displayed on the button.
///on_press : callable, optional
///    Callback invoked when the button is pressed.
///width : float, optional
///    Fixed width in logical pixels.
///height : float, optional
///    Fixed height in logical pixels.
///width_fill : bool, default False
///    Whether the button fills available width.
///height_fill : bool, default False
///    Whether the button fills available height.
///padding : list of float, optional
///    Padding as [all], [vertical, horizontal], or
///    [top, right, bottom, left].
///text_top_left : bool, optional
///    Align the label to the top-left.
///text_top_center : bool, optional
///    Align the label to the top-centre.
///text_top_right : bool, optional
///    Align the label to the top-right.
///text_center_left : bool, optional
///    Align the label to the centre-left.
///text_center : bool, optional
///    Align the label to the centre (default True).
///text_center_right : bool, optional
///    Align the label to the centre-right.
///text_bottom_left : bool, optional
///    Align the label to the bottom-left.
///text_bottom_center : bool, optional
///    Align the label to the bottom-centre.
///text_bottom_right : bool, optional
///    Align the label to the bottom-right.
///text_size : float, optional
///    Font size for the label text.
///clip : bool, optional
///    Whether to clip content that overflows the button.
///style_id : int, optional
///    ID of a custom style created with ``add_button_style``.
///style_std : IpgButtonStyleStd, optional
///    A predefined standard style variant.
///style_arrow : IpgArrow, optional
///    An arrow icon style for the button.
///user_data : Any, optional
///    Arbitrary data forwarded to callbacks.
///show : bool, default True
///    Whether the button is visible.
///gen_id : int, optional
///    Pre-generated numeric ID.  Used to assign ids to widgets that have not benn created yet.
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
///background_color: Optional[IpgColor]=None
///    Color of the background.
///background_rgba: Optional[list[float, 4]]=None
///    Color of the background in rgba format.
///border_color: Optional[IpgColor]=None
///    Color used for the border.
///border_rgba: list[float; 4]=None
///    Color of the border in rgba format.
///border_radius: Optional[list[float]]=None
///    The radius border, [float]=all corners, 
///    [float, 4]=[top-left, top-right, bottom-right, bottom-left].
///border_width: Optional[float]
///    Border width.
///shadow_color: Optional[IpgColor]
///    The color of the shadow.
///shadow_rgba: Optional[list]
///    The color in rgba format [float; 4] used as state above.
///shadow_offset_xy: Optional[float, 2]
///    Shadow offset in the horizontal direction.
///shadow_blur_radius: Optional[float]
///    The blur radius of the shadow.
///text_color: Optional[IpgColor]
///    The text color, if not defined, will either be a Black or White variation based on theme background.
///text_rgba: [list, 4]
///    The color in rgba used as state above.
/// gen_id : int, optional
///     Pre-generated numeric ID.  Used to assign ids to widgets that have not benn created yet.
///"""
#[pyfunction]
#[pyo3(signature = (
        background_color=None, 
        background_rgba=None,
        background_gradient_color_stop=None,
        background_gradient_rgba_stop=None,
        background_gradient_degrees=None,
        background_gradient_radians=None,
        background_gradient_alpha=None,
        border_color=None, 
        border_rgba=None,
        border_radius=None, 
        border_width=1.0,
        shadow_color=None, 
        shadow_rgba=None,
        shadow_offset_xy=None, 
        shadow_blur_radius=None,
        text_color=None, 
        text_rgba=None,
        gen_id=None
        ))]
pub fn add_button_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    background_gradient_color_stop: Option<IpgColor>,
    background_gradient_rgba_stop: Option<[f32; 4]>,
    background_gradient_degrees: Option<f32>,
    background_gradient_radians: Option<f32>,
    background_gradient_alpha: Option<f32>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    shadow_color: Option<IpgColor>,
    shadow_rgba: Option<[f32; 4]>,
    shadow_offset_xy: Option<[f32; 2]>,
    shadow_blur_radius: Option<f32>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let grad_a = background_gradient_alpha.unwrap_or(1.0);

    let background_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0, false);
    let background_gradient_color_stop: Option<Color> =
        IpgColor::rgba_ipg_color_to_iced(background_gradient_rgba_stop, background_gradient_color_stop, grad_a, false);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0, false);
    let shadow_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(shadow_rgba, shadow_color, 1.0, false);
    let text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, 1.0, false);

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
