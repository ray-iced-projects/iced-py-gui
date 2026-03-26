//! Checkbox module - provides add_checkbox pyfunction
use iced::Color;
use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::bootstrap_icon::IpgIcon;
use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::get_length;
use crate::state::{IpgWidgets, access_state, 
    add_callback_to_mutex, get_id, set_state_of_widget};
use crate::widgets::ipg_checkbox::{IpgCheckBox, IpgCheckboxStyle, IpgCheckboxStyleStd};
use crate::widgets::ipg_text::{TextShaping, TextWrapping};


/// Add a checkbox widget.
///
/// A checkbox with a text label that can be toggled on and off.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this checkbox belongs to.
/// on_toggle : callable,  Optional
///     Sets the Callback method to invoke when the checkbox is toggled.
/// is_checked : bool, default False
///     Whether the checkbox starts in the checked state.
/// label : str,  Optional
///     Sets the Text label displayed next to the checkbox.
/// width : float,  Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the checkbox fills available width.
/// size : float,  Optional
///     Sets the size of the checkbox square in logical pixels.
/// spacing : float,  Optional
///     Sets the Spacing between the checkbox square and the label.
/// text_size : float,  Optional
///     Sets the Font size for the label text.
/// text_line_height : float,  Optional
///     Sets the Line height for the label text.
/// text_shaping : TextShaping,  Optional
///     Sets the Text shaping strategy for the label.
/// text_wrapping : TextWrapping,  Optional
///     Sets the Text wrapping strategy for the label.
/// text_font_id : int,  Optional
///     Sets the Font ID for the label text.
/// icon_font_id : int,  Optional
///     Sets the Font ID for the checkbox icon.
/// icon : IpgIcon,  Optional
///     Sets the Icon displayed inside the checkbox when checked.
/// icon_size : float,  Optional
///     Sets the Size of the checkbox icon.
/// icon_line_height : float,  Optional
///     Sets the Line height of the checkbox icon.
/// icon_shaping : TextShaping,  Optional
///     Sets the Text shaping strategy for the icon.
/// user_data : Any,  Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the checkbox is visible.
/// style_id : int,  Optional
///     Sets the ID of a custom style created with ``add_checkbox_style``.
/// style_std : IpgCheckboxStyleStd,  Optional
///     Sets the predefined standard style variant.
/// gen_id : int,  Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created checkbox.
/// 
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    on_toggle=None, 
    is_checked=false, 
    label=None, 
    width=None, 
    width_fill=false, 
    size=None, 
    spacing=None, 
    text_size=None,
    text_line_height=None,
    text_shaping=None,
    text_wrapping=None,
    text_font_id=None,
    icon_font_id= None,
    icon=None,
    icon_size=None,
    icon_line_height=None,
    icon_shaping=None,
    user_data=None, 
    show=true, 
    style_id=None, 
    style_std=None,
    gen_id=None, 
    ))] 
pub fn add_checkbox(
    parent_id: String,
    on_toggle: Option<PyObject>,
    is_checked: bool,
    label: Option<String>,
    width: Option<f32>,
    width_fill: bool,
    size: Option<f32>,
    spacing: Option<f32>,
    text_size: Option<f32>,
    text_line_height: Option<f32>,
    text_shaping: Option<TextShaping>,
    text_wrapping: Option<TextWrapping>,
    text_font_id: Option<usize>,
    icon_font_id: Option<usize>,
    icon: Option<IpgIcon>,
    icon_size: Option<f32>,
    icon_line_height: Option<f32>,
    icon_shaping: Option<TextShaping>,
    user_data: Option<PyObject>,
    show: bool,
    style_id: Option<usize>,
    style_std: Option<IpgCheckboxStyleStd>,
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);
    
    if let Some(py) = on_toggle {
        add_callback_to_mutex(id, "on_toggle".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    let width = get_length(width, width_fill);
    
    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgCheckBox(
        IpgCheckBox {
            id,
            parent_id,
            show,
            is_checked,
            label,
            width,
            size,
            spacing,
            text_size,
            text_line_height,
            text_shaping,
            text_wrapping,
            text_font_id,
            icon_font_id,
            icon,
            icon_size,
            icon_line_height,
            icon_shaping,
            style_id,
            style_std,
            }));

    drop(state);
    Ok(id)

}


///"""
///Adds styling to container
///
///Parameters
///----------
///background_color: IpgColor, Optional
///    The background color of the box.
///background_rgba: list, Optional,
///    The background color of the box in rgba format.
///dark_mode: bool, Optional
///    Whether the custom bkg color is dark.
///    Will adjust the border and text color from the bkg color
///    unless the border color is set.
///border_color: IpgColor, Optional
///    The color for the border.
///border_rgba: list[float]
///    The color of the border in rgba format used as state above.
///border_radius: list[float]
///    The radius of the 4 corners, [float]=all corners, 
///    [float] top-left, top-right, bottom-right, bottom-left.
///border_width: float
///    The border width.
///text_color: IpgColor, Optional
///    The text color, if not defined, will either be a Black or White variation based on theme background.
///text_rgba: list[float], Optional
///    The text color in rgba format.
/// /// gen_id : int,  Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///    """
/// 
#[pyfunction]
#[pyo3(signature = ( 
    background_color=None, 
    background_rgba=None,
    border_color=None, 
    border_rgba=None,
    border_radius=None, 
    border_width=None,
    icon_color=None, 
    icon_rgba=None,
    text_color=None, 
    text_rgba=None,
    gen_id=None
    ))]
pub fn add_checkbox_style(
    background_color: Option<IpgColor>,
    background_rgba: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    icon_color: Option<IpgColor>,
    icon_rgba: Option<[f32; 4]>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, 1.0);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, 1.0);
    let icon_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(icon_rgba, icon_color, 1.0);
    let text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, 1.0);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgCheckboxStyle(
        IpgCheckboxStyle {
            id,
            background_color,
            border_color,
            border_radius,
            border_width,
            icon_color,
            text_color,
        }));

    drop(state);
    Ok(id)
}