//! ColorPicker module - provides add_button pyfunction

use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;

use crate::ipg_widgets::ipg_color_picker::color_picker::ColorValue;
use crate::widgets::ipg_button::ButtonStyleStd;
use crate::{add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{Widgets, access_state, get_id, set_state_of_widget};
use crate::widgets::ipg_color_picker::{ColorOutFormat, ColorPicker};
use crate::widgets::ipg_color_picker::rgb_to_hue;


/// Add a color_picker widget.
///
/// A color picker that opens from a button, allowing the user
/// to select a color interactively using HSV canvas, RGBA sliders,
/// a hue slider, and an optional palette panel showing tint/shade
/// variants of the selected color.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this color picker belongs to.
/// on_press : callable, optional
///     Callback invoked when the trigger button is pressed (picker opens/closes).
/// on_submit : callable, optional
///     Callback invoked when the user clicks Submit. Receives the selected
///     color formatted according to ``color_output_format``.
/// on_cancel : callable, optional
///     Callback invoked when the user clicks Cancel.
/// opened : bool, optional
///     Initial open/closed state of the picker. Default ``False``.
/// initial_color : ColorValue, optional
///     Starting color. Accepts ``ColorValue.Float``, ``ColorValue.Integer``,
///     ``ColorValue.Hex``, or ``ColorValue.Percent``. Defaults to a purple.
/// color_output_format : ColorOutFormat, optional
///     Format used when returning the color via ``on_submit`` and the
///     clipboard button. One of ``Integer`` (default), ``Float``, ``Hex``,
///     or ``Percent``.
/// gap : int, optional
///     Gap in pixels between the trigger button and the picker panel.
/// snap_within_viewport : bool, optional
///     When ``True``, clamps the picker panel inside the window bounds.
/// position_follow_cursor : bool, optional
///     Opens the picker at the cursor position.
/// position_bottom : bool, optional
///     Opens the picker below the trigger button.
/// position_left : bool, optional
///     Opens the picker to the left of the trigger button.
/// position_top : bool, optional
///     Opens the picker above the trigger button. This is the default.
/// position_right : bool, optional
///     Opens the picker to the right of the trigger button.
/// position_center : bool, optional
///     Centers the picker over the trigger button.
/// btn_label : str, optional
///     Text label displayed on the trigger button. Defaults to "Color Picker".
/// btn_style_id : int, optional
///     ID of a custom button style created with ``add_button_style``.
/// btn_style_std : ButtonStyleStd, optional
///     Predefined standard button style variant.
/// user_data : Any, optional
///     Arbitrary data forwarded unchanged to all callbacks.
/// gen_id : int, optional
///     Pre-reserved widget ID (from ``generate_id``).
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created color picker.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    on_press=None, 
    on_submit=None, 
    on_cancel=None,
    opened=false,
    initial_color=None,
    color_output_format=None,
    gap=None,
    snap_within_viewport=None,
    position_follow_cursor=None,
    position_bottom=None,
    position_left=None,
    position_top=None,
    position_right=None,
    position_center=None,
    btn_label=None,
    btn_style_id=None,
    btn_style_std=None,
    user_data=None,
    gen_id=None,
    ))]
pub fn add_color_picker(
    parent_id: String,
    on_press: Option<PyObject>,
    on_submit: Option<PyObject>,
    on_cancel: Option<PyObject>,
    opened: bool,
    initial_color: Option<PyObject>,
    color_output_format: Option<ColorOutFormat>,
    gap: Option<u32>,
    snap_within_viewport: Option<bool>,
    position_follow_cursor: Option<bool>,
    position_bottom: Option<bool>,
    position_left: Option<bool>,
    position_top: Option<bool>,
    position_right: Option<bool>,
    position_center: Option<bool>,
    btn_label: Option<String>,
    btn_style_id: Option<usize>,
    btn_style_std: Option<ButtonStyleStd>,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);


    let initial_color: Option<ColorValue> = match initial_color {
        Some(ic) => pyo3::Python::attach(|py| ic.extract::<ColorValue>(py).ok()),
        None => None,
    }.or(Some(ColorValue::Integer([109, 80, 203, 255])));

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
        add_callback_to_mutex(id, "on_open".to_string(), py);
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
            opened,
            initial_color,
            color_output_format, 
            gap,
            snap_within_viewport,
            position_follow_cursor,
            position_bottom,
            position_left,
            position_top,
            position_right,
            position_center,
            btn_label,
            btn_style_id,
            btn_style_std,
            
            r_value, 
            g_value, 
            b_value, 
            a_value, 
            hue_value,
            show_palette: false,
            }));

    drop(state);
    Ok(id)

}
