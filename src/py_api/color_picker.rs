//! ColorPicker module - provides add_button pyfunction

use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::ipg_widgets::ipg_color_picker::color_picker::ColorValue;
use crate::{add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{Widgets, access_state, get_id, set_state_of_widget};
use crate::widgets::ipg_button::{ButtonStyleStd};
use crate::widgets::ipg_color_picker::{ColorOutFormat, ColorPicker};
use crate::widgets::ipg_color_picker::rgb_to_hue;
use crate::graphics::bootstrap_arrow::Arrow;


/// Add a color_picker widget.
///
/// A color picker that opens from a button, allowing the user
/// to select a color interactively.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this color picker belongs to.
/// label : str,  Optional
///     Sets the Text label displayed on the button.
/// gen_id : int,  Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// on_press : callable,  Optional
///     Sets the Callback method to invoke when the button is pressed.
/// on_submit : callable,  Optional
///     Sets the Callback method to invoke when a color is submitted.
/// on_cancel : callable,  Optional
///     Sets the Callback method to invoke when the color selection is cancelled.
/// color : Color,  Optional
///     Sets the initial color using a predefined color variant.
/// color_alpha : float,  Optional
///     Sets the alpha of the Color.
/// color_rgba : list of float,  Optional
///     Sets the initial color in rgba format as [r, g, b, a].
/// width : float,  Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the button fills available width.
/// height : float,  Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the button fills available height.
/// padding : list of float,  Optional
///     Sets the Padding as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// clip : bool,  Optional
///     Whether to clip content that overflows the button.
/// style_id : int,  Optional
///     Sets the ID of a custom style created with ``add_button_style``.
/// style_std : ButtonStyleStd,  Optional
///     Sets the predefined standard style variant.
/// style_arrow : Arrow,  Optional
///     Sets the arrow icon style for the button.
/// user_data : Any,  Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default False
///     Whether the color picker is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created color picker.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    label=None, 
    gen_id=None, 
    on_press=None, 
    on_submit=None, 
    on_cancel=None,
    position_follow_cursor=None,
    position_bottom=None,
    position_left=None,
    position_top=None,
    position_right=None,
    text=None,
    gap=None,
    snap_within_viewport=None,
    delay_sec=None,
    width=None,
    width_fill=None,  
    height=None, 
    height_fill=None,
    fill=None, 
    padding=None, 
    clip=None, 
    style_id=None, 
    style_std=None, 
    style_arrow=None,
    user_data=None,
    show=false,

    opened=false,
    initial_color=None,
    color_output_format=None,
    ))]
pub fn add_color_picker(
    parent_id: String,
    label: Option<String>,
    gen_id: Option<usize>,
    on_press: Option<PyObject>,
    on_submit: Option<PyObject>,
    on_cancel: Option<PyObject>,
    position_follow_cursor: Option<bool>,
    position_bottom: Option<bool>,
    position_left: Option<bool>,
    position_top: Option<bool>,
    position_right: Option<bool>,
    text: Option<String>,
    gap: Option<u32>,
    snap_within_viewport: Option<bool>,
    delay_sec: Option<u64>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    padding: Option<Vec<f32>>,
    clip: Option<bool>,
    style_id: Option<usize>,
    style_std: Option<ButtonStyleStd>,
    style_arrow: Option<Arrow>,
    user_data: Option<PyObject>,
    show: bool,

    opened: bool,
    initial_color: Option<PyObject>,
    color_output_format: Option<ColorOutFormat>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);


    let initial_color: Option<ColorValue> = match initial_color {
        Some(ic) => pyo3::Python::attach(|py| ic.extract::<ColorValue>(py).ok()),
        None => None,
    }.or(Some(ColorValue::Integer([0, 0, 0, 255])));

    let [r_init, g_init, b_init, a_init] = initial_color
        .as_ref()
        .map(|ic| ic.to_normalized())
        .unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let r_value = (r_init * 255.0).round() as u8;
    let g_value = (g_init * 255.0).round() as u8;
    let b_value = (b_init * 255.0).round() as u8;
    let a_value = (a_init * 255.0).round() as u8;
    let hue_value = rgb_to_hue(r_value, g_value, b_value);

    let color_output_format = if let Some(cfs) = color_output_format {
        Some(cfs)
    } else {
        Some(ColorOutFormat::Integer)
    };

    if let Some(py) = on_press {
        add_callback_to_mutex(id, "on_press".to_string(), py);
    }

    if let Some(py) = on_submit {
        add_callback_to_mutex(id, "on_submit".to_string(), py);
    }

    if let Some(py) = on_cancel {
        add_callback_to_mutex(id, "on_cancel".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, Widgets::ColorPicker(
        ColorPicker {
            id,
            show,
            position_follow_cursor,
            position_bottom,
            position_left,
            position_top,
            position_right,
            text,
            gap,
            snap_within_viewport,
            delay_sec,
            // button related
            label,
            width,
            width_fill,  
            height, 
            height_fill,
            fill, 
            padding,
            clip,
            style_id,
            style_std,
            style_arrow,

            r_value, 
            g_value, 
            b_value, 
            a_value, 
            hue_value,
            opened,
            initial_color,
            color_output_format,                             
            }));

    drop(state);
    Ok(id)

}
