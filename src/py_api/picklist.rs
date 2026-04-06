//! PickList module - provides add_pick_list pyfunction

use iced::Color;

use pyo3::{Py, PyAny, PyResult, pyfunction};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, 
    add_user_data_to_mutex};
use crate::graphics::{colors::IpgColor, 
        bootstrap_arrow::Arrow}; 
use crate::py_api::helpers::get_length; 
use crate::state::{IpgWidgets, get_id, set_state_of_widget}; 
use crate::widgets::{ipg_text::TextShaping, 
        ipg_pick_list::{IpgPickList, IpgPickListHandle, 
        IpgPickListStyle, convert_pyobject_vec_string}};



/// Add a pick list widget.
///
/// A dropdown pick list that lets the user select one option
/// from a list of choices.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this pick list belongs to.
/// options : list of str
///     Sets the list of selectable options.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// on_select : callable, Optional
///     Sets the Callback method to invoke when an option is selected.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the pick list fills available width.
/// menu_height : float, Optional
///     Sets the Fixed height of the dropdown menu in logical pixels.
/// menu_height_fill : bool, default False
///     Whether the dropdown menu fills available height.
/// padding : list of float, Optional
///     Sets the Padding as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// placeholder : str, Optional
///     Sets the placeholder text shown when no option is selected.
/// selected : str, Optional
///     Sets the currently selected option.
/// text_size : float, Optional
///     Sets the Font size for the text.
/// text_line_height : float, Optional
///     Sets the Line height for the text.
/// text_shaping : TextShaping, Optional
///     Sets the Text shaping strategy.
/// handle : IpgPickListHandle, Optional
///     Sets the handle type for the pick list.
/// arrow_size : float, Optional
///     Sets the size of the arrow icon.
/// dynamic_closed : IpgArrow, Optional
///     Sets the arrow icon when the pick list is closed.
/// dynamic_open : IpgArrow, Optional
///     Sets the arrow icon when the pick list is open.
/// custom_static : IpgArrow, Optional
///     Sets the static custom arrow icon.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_pick_list_style``.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the pick list is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created pick list.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    options, 
    gen_id=None, 
    on_select=None, 
    width=None, 
    width_fill=false,
    menu_height=None,
    menu_height_fill=false, 
    padding=None,  
    placeholder=None, 
    selected=None, 
    text_size=None, 
    text_line_height=None, 
    text_shaping=None, 
    handle=None, 
    arrow_size=None, 
    dynamic_closed=None, 
    dynamic_open=None, 
    custom_static=None,
    style_id=None, 
    user_data=None, 
    show=true,
    ))]
pub fn add_pick_list(
    parent_id: String,
    options: PyObject,
    // **above required
    gen_id: Option<usize>,
    on_select: Option<PyObject>,
    width: Option<f32>,
    width_fill: bool,
    menu_height: Option<f32>,
    menu_height_fill: bool,
    padding: Option<Vec<f32>>,
    placeholder: Option<String>,
    selected: Option<String>,
    text_size: Option<f32>,
    text_line_height: Option<f32>,
    text_shaping: Option<TextShaping>,
    handle: Option<IpgPickListHandle>,
    arrow_size: Option<f32>,
    dynamic_closed: Option<Arrow>,
    dynamic_open: Option<Arrow>,
    custom_static: Option<Arrow>,
    style_id: Option<usize>,
    user_data: Option<PyObject>,
    show: bool,
) -> PyResult<usize>
{
    let id = get_id(gen_id);

    if let Some(py) = on_select {
        add_callback_to_mutex(id, "on_select".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let width = get_length(width, width_fill);
    let menu_height = get_length(menu_height, menu_height_fill);

    let options =  convert_pyobject_vec_string(options);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgPickList(
        IpgPickList { 
            id,
            parent_id,
            show,
            options,
            placeholder,
            selected,
            width,
            menu_height,
            padding,
            text_size,
            text_line_height,
            text_shaping,
            handle,
            arrow_size,
            dynamic_closed,
            dynamic_open,
            custom_static,
            style_id,
        }));


    drop(state);
    Ok(id)
}

/// Add styling to a pick list.
///
/// Creates a custom style that can be applied to a pick list
/// via its ``style_id`` parameter.
///
/// Parameters
/// ----------
/// background_color : IpgColor, Optional
///     Sets the background color using a predefined color variant.
/// background_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// background_rgba : list of float, Optional
///     Sets the background color in rgba format as [r, g, b, a].
/// text_color : IpgColor, Optional
///     Sets the text color using a predefined color variant.
/// text_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// text_rgba : list of float, Optional
///     Sets the text color in rgba format as [r, g, b, a].
/// handle_color : IpgColor, Optional
///     Sets the handle color using a predefined color variant.
/// handle_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// handle_rgba : list of float, Optional
///     Sets the handle color in rgba format as [r, g, b, a].
/// placeholder_color : IpgColor, Optional
///     Sets the placeholder text color using a predefined color variant.
/// placeholder_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// placeholder_rgba : list of float, Optional
///     Sets the placeholder text color in rgba format as [r, g, b, a].
/// border_color : IpgColor, Optional
///     Sets the border color using a predefined color variant.
/// border_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// border_rgba : list of float, Optional
///     Sets the border color in rgba format as [r, g, b, a].
/// border_color_hovered : IpgColor, Optional
///     Sets the border color when hovered using a predefined color variant.
/// border_color_hovered_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// border_rgba_hovered : list of float, Optional
///     Sets the border color when hovered in rgba format as [r, g, b, a].
/// border_radius : list of float, Optional
///     Sets the radius of the corners as [all] or
///     [top-left, top-right, bottom-right, bottom-left].
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a pick list's ``style_id``.
#[pyfunction]
#[pyo3(signature = (
    background_color=None,
    background_color_alpha=None,
    background_rgba=None,
    text_color=None,
    text_color_alpha=None,
    text_rgba=None,
    handle_color=None,
    handle_color_alpha=None,
    handle_rgba=None,
    placeholder_color=None,
    placeholder_color_alpha=None,
    placeholder_rgba=None,
    border_color=None,
    border_color_alpha=None,
    border_rgba=None,
    border_color_hovered=None,
    border_color_hovered_alpha=None,
    border_rgba_hovered=None,
    border_radius=None,
    border_width=None,
    gen_id=None
    ))]
pub fn add_pick_list_style(
    background_color: Option<IpgColor>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    text_color: Option<IpgColor>,
    text_color_alpha: Option<f32>,
    text_rgba: Option<[f32; 4]>,
    handle_color: Option<IpgColor>,
    handle_color_alpha: Option<f32>,
    handle_rgba: Option<[f32; 4]>,
    placeholder_color: Option<IpgColor>,
    placeholder_color_alpha: Option<f32>,
    placeholder_rgba: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_color_hovered: Option<IpgColor>,
    border_color_hovered_alpha: Option<f32>,
    border_rgba_hovered: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);
    
    let background_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, background_color_alpha);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, border_color_alpha);
    let border_color_hovered: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_hovered, border_color_hovered, border_color_hovered_alpha);
    let handle_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(handle_rgba, handle_color, handle_color_alpha);
    let placeholder_color = 
        IpgColor::rgba_ipg_color_to_iced(placeholder_rgba, placeholder_color, placeholder_color_alpha);
    let text_color = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, text_color_alpha);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgPickListStyle(
        IpgPickListStyle {
            id,
            background_color,
            text_color,
            handle_color,
            placeholder_color,
            border_color,
            border_color_hovered,
            border_radius,
            border_width,
        }));

    drop(state);
    Ok(id)
}
