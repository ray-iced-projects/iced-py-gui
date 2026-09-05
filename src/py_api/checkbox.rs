//! Checkbox module - provides add_checkbox pyfunction

use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::bootstrap::bootstrap_icon::Icon;
use crate::graphics::colors::Color;
use crate::state::{Widgets, access_state, add_callback_name_to_mutex, get_id, set_state_of_widget};
use crate::widgets::callbacks::CallbackName;
use crate::widgets::ipg_checkbox::{CheckBox, CheckboxStyle, CheckboxStyleStd};


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
/// text_wrapping_none: Optional[bool]
///     Sets the wrapping mode of the label to no wrapping, default is on a word.
/// text_wrapping_glyph: Optional[bool]
///     Sets the wrapping mode of the label to wrap on a glyph.
/// text_wrapping_word_glyph: Optional[bool]
///     Sets the wrapping mode of the label to wrap on a glyph or glyph.
/// text_font_id : int,  Optional
///     Sets the Font ID for the label text.
/// icon_font_id : int,  Optional
///     Sets the Font ID for the checkbox icon.
/// icon : Icon,  Optional
///     Sets the Icon displayed inside the checkbox when checked.
/// icon_size : float,  Optional
///     Sets the Size of the checkbox icon.
/// icon_line_height : float,  Optional
///     Sets the Line height of the checkbox icon.
/// user_data : Any,  Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the checkbox is visible.
/// active : bool, Optional
///     Sets the checkbox to the Active status.
/// hovered : bool, Optional
///     Sets the checkbox to the Hovered status.
/// disabled : bool, Optional
///     Sets the checkbox to the Disabled status.
/// lock_is_checked: bool, Optional
///     Keeps the check or unchecked value to True or False.
/// style_id : int,  Optional
///     Sets the ID of a custom style created with ``add_checkbox_style``.
/// style_std : CheckboxStyleStd,  Optional
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
    fill=None, 
    size=None, 
    spacing=None, 
    text_size=None,
    line_height=None,
    text_wrapping_none=None,
    text_wrapping_glyph=None,
    text_wrapping_word_glyph=None,
    text_font_id=None,
    icon_font_id= None,
    icon=None,
    icon_size=None,
    icon_line_height=None,
    user_data=None, 
    show=true,
    active=None,
    hovered=None,
    disabled=None,
    lock_is_checked=None,
    style_id=None, 
    style_std=None,
    palette_id=None,
    gen_id=None, 
    ))] 
pub fn add_checkbox(
    parent_id: String,
    on_toggle: Option<PyObject>,
    is_checked: bool,
    label: Option<String>,
    width: Option<f32>,
    fill: Option<bool>,
    size: Option<f32>,
    spacing: Option<f32>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    text_wrapping_none: Option<bool>,
    text_wrapping_glyph: Option<bool>,
    text_wrapping_word_glyph: Option<bool>,
    text_font_id: Option<usize>,
    icon_font_id: Option<usize>,
    icon: Option<Icon>,
    icon_size: Option<f32>,
    icon_line_height: Option<f32>,
    user_data: Option<PyObject>,
    show: bool,
    active: Option<bool>,
    hovered: Option<bool>,
    disabled: Option<bool>,
    lock_is_checked: Option<bool>,
    style_id: Option<usize>,
    style_std: Option<CheckboxStyleStd>,
    palette_id: Option<usize>,
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);
    
    if let Some(py) = on_toggle {
        add_callback_name_to_mutex(id, CallbackName::OnToggle, py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }
    
    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::CheckBox(
        CheckBox {
            id,
            show,
            is_checked,
            label,
            width,
            fill,
            size,
            spacing,
            text_size,
            line_height,
            text_wrapping_none,
            text_wrapping_glyph,
            text_wrapping_word_glyph,
            text_font_id,
            icon_font_id,
            icon,
            icon_size,
            icon_line_height,
            active,
            hovered,
            disabled,
            lock_is_checked,
            style_id,
            style_std,
            palette_id,
            }));

    drop(state);
    Ok(id)

}



/// Adds styling to container
/// 
/// Parameters
/// ----------
/// bkg_color: Color, Optional
///     Sets the background color.
/// bkg_color_alpha: float, Optional
///     Sets the alpha value of the background Color.
/// bkg_rgba: list[float, 4], Optional
///     The background color in rgba format.
/// icon_color: Color, Optional
///     Sets the icon color.
/// icon_color_alpha: float, Optional
///     Sets the alpha value of the icon Color.
/// icon_rgba: list[float, 4], Optional
///     The icon color in rgba format.
/// border_color: Color, Optional
///     Sets the border color.
/// border_color_alpha: float, Optional
///     Sets the alpha value of the border Color.
/// border_rgba: list[float, 4], Optional
///     The border color in rgba format.
/// border_radius: list[float, 4], Optional
///     Sets the radius for all four corners as [top_left, top_right, bottom_right, bottom_left].
/// border_rounded: float, Optional
///     Sets the radius for all four corners to the same value.
/// border_radius_top: float, Optional
///     Sets the radius for top-left and top-right corners.
/// border_radius_top_left: float, Optional
///     Sets the radius for the top-left corner.
/// border_radius_top_right: float, Optional
///     Sets the radius for the top-right corner.
/// border_radius_bottom: float, Optional
///     Sets the radius for bottom-left and bottom-right corners.
/// border_radius_bottom_left: float, Optional
///     Sets the radius for the bottom-left corner.
/// border_radius_bottom_right: float, Optional
///     Sets the radius for the bottom-right corner.
/// border_radius_left: float, Optional
///     Sets the radius for top-left and bottom-left corners.
/// border_radius_right: float, Optional
///     Sets the radius for top-right and bottom-right corners.
/// border_width: float, Optional
///     Sets the border width.
/// text_color: Color, Optional
///     The text color.
/// text_color_alpha: float, Optional
///     Sets the alpha of the Color.
/// text_rgba: list[float], Optional
///     The text color in rgba format.
///  gen_id : int,  Optional
///      Obtains an ID of a widget that have not been created, used for the gen_id parameter.
#[pyfunction]
#[pyo3(signature = ( 
    bkg_color=None,
    bkg_color_alpha=None,
    bkg_rgba=None,

    icon_color=None,
    icon_color_alpha=None,
    icon_rgba=None,

    border_color=None,
    border_color_alpha=None,
    border_rgba=None,

    border_radius=None,
    border_rounded=None,

    border_radius_top=None,
    border_radius_top_left=None,
    border_radius_top_right=None,

    border_radius_bottom=None,
    border_radius_bottom_left=None,
    border_radius_bottom_right=None,
    
    border_radius_left=None,
    border_radius_right=None,

    border_width=None,

    text_color=None,
    text_color_alpha=None,
    text_rgba=None,
    gen_id=None
    ))]
pub fn add_checkbox_style(
    bkg_color: Option<Color>,
    bkg_color_alpha: Option<f32>,
    bkg_rgba: Option<[f32; 4]>,

    icon_color: Option<Color>,
    icon_color_alpha: Option<f32>,
    icon_rgba: Option<[f32; 4]>,

    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,

    border_radius: Option<[f32; 4]>,
    border_rounded: Option<f32>,
    border_radius_top: Option<f32>,
    border_radius_top_left: Option<f32>,
    border_radius_top_right: Option<f32>,

    border_radius_bottom: Option<f32>,
    border_radius_bottom_left: Option<f32>,
    border_radius_bottom_right: Option<f32>,

    border_radius_left: Option<f32>,
    border_radius_right: Option<f32>,

    border_width: Option<f32>,
    
    text_color: Option<Color>,
    text_color_alpha: Option<f32>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::CheckboxStyle(
        CheckboxStyle {
            id,

            bkg_color,
            bkg_color_alpha,
            bkg_rgba,

            icon_color,
            icon_color_alpha,
            icon_rgba,

            border_color,
            border_color_alpha,
            border_rgba,
            
            border_radius,
            border_rounded,
            
            border_radius_top,
            border_radius_top_left,
            border_radius_top_right,

            border_radius_bottom,
            border_radius_bottom_left,
            border_radius_bottom_right,
            
            border_radius_left,
            border_radius_right,

            border_width,

            text_color,
            text_color_alpha,
            text_rgba,
        }));

    drop(state);
    Ok(id)
}