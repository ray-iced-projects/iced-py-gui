//! Radio adds add_radio to the pyfunction
use iced::Color;
use pyo3::{pyfunction, PyResult, Py, PyAny};
type PyObject = Py<PyAny>;
use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::IpgColor, py_api::helpers::get_length, 
    state::{IpgWidgets, get_id, set_state_of_widget}, 
    widgets::{ipg_text::TextShaping, ipg_radio::{IpgRadio, IpgRadioDirection, 
        IpgRadioStyle}, ipg_text::TextWrapping}};


/// Add a radio button group widget.
///
/// A group of radio buttons where the user can select one option
/// from a list of labels.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this radio group belongs to.
/// labels : list of str
///     Sets the list of labels for each radio button.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// direction : IpgRadioDirection, default Vertical
///     Sets the layout direction of the radio buttons.
/// spacing : float, Optional
///     Sets the spacing between radio buttons in logical pixels.
/// radio_spacing : float, Optional
///     Sets the spacing between the radio circle and its label.
/// padding : list of float, Optional
///     Sets the Padding as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the radio group fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the radio group fills available height.
/// on_select : callable, Optional
///     Sets the Callback method to invoke when a radio button is selected.
/// selected_index : int, Optional
///     Sets the index of the initially selected radio button.
/// size : float, Optional
///     Sets the size of the radio circle in logical pixels.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_radio_style``.
/// font_id : int, Optional
///     Sets the Font ID for the label text.
/// text_spacing : float, Optional
///     Sets the spacing between the radio circle and text.
/// text_size : float, Optional
///     Sets the Font size for the label text.
/// text_line_height : float, Optional
///     Sets the Line height for the label text.
/// text_shaping : TextShaping, Optional
///     Sets the Text shaping strategy for the labels.
/// text_wrapping : TextWrapping, Optional
///     Sets the Text wrapping strategy for the labels.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
/// show : bool, default True
///     Whether the radio group is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created radio group.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    labels, 
    gen_id=None,
    direction=IpgRadioDirection::Vertical,
    spacing=None,
    radio_spacing=None, 
    padding=None, 
    width=None, 
    width_fill=false, 
    height=None, 
    height_fill=false,
    on_select=None, 
    selected_index=None, 
    size=None, 
    style_id=None,
    font_id=None,
    text_spacing=None, 
    text_size=None,
    text_line_height=None, 
    text_shaping=None,
    text_wrapping=None, 
    user_data=None, 
    show=true, 
    ))]
pub fn add_radio(
    parent_id: String,
    labels: Vec<String>,
    //**above required
    gen_id: Option<usize>,
    direction: IpgRadioDirection,
    spacing: Option<f32>,
    radio_spacing: Option<f32>,
    padding: Option<Vec<f32>>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    on_select: Option<PyObject>,
    selected_index: Option<usize>,
    size: Option<f32>,
    style_id: Option<usize>,
    font_id: Option<usize>,
    text_spacing: Option<f32>,
    text_size: Option<f32>,
    text_line_height: Option<f32>,
    text_shaping: Option<TextShaping>,
    text_wrapping: Option<TextWrapping>,
    user_data: Option<PyObject>,
    show: bool,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let is_selected = if let Some(val) = selected_index {
        if val > labels.len()-1 {
            panic!("Radio selected_index is greater than the size of the labels")
        } else { Some(val) }
    } else { None };

    let width = get_length(width, width_fill);
    let height = get_length(height, height_fill);

    if let Some(py) = on_select {
        add_callback_to_mutex(id, "on_select".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();
    
    state.widgets.insert(id, IpgWidgets::IpgRadio(
        IpgRadio {
            id,
            parent_id,
            labels,
            direction,
            spacing,
            radio_spacing,
            padding,
            show,
            is_selected,
            width,
            height,
            size,
            text_spacing,
            text_size,
            text_line_height,
            text_shaping,
            text_wrapping,
            font_id,
            style_id,
        }));

    drop(state);                                      
    Ok(id)

}


/// Add styling to a radio button group.
///
/// Creates a custom style that can be applied to a radio group
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
/// background_color_hovered : IpgColor, Optional
///     Sets the background color when hovered using a predefined color variant.
/// background_color_hovered_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// background_rgba_hovered : list of float, Optional
///     Sets the background color when hovered in rgba format as [r, g, b, a].
/// border_color : IpgColor, Optional
///     Sets the border color using a predefined color variant.
/// border_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// border_rgba : list of float, Optional
///     Sets the border color in rgba format as [r, g, b, a].
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// dot_color : IpgColor, Optional
///     Sets the dot color using a predefined color variant.
/// dot_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// dot_rgba : list of float, Optional
///     Sets the dot color in rgba format as [r, g, b, a].
/// dot_color_hovered : IpgColor, Optional
///     Sets the dot color when hovered using a predefined color variant.
/// dot_color_hovered_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// dot_rgba_hovered : list of float, Optional
///     Sets the dot color when hovered in rgba format as [r, g, b, a].
/// text_color : IpgColor, Optional
///     Sets the text color using a predefined color variant.
/// text_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// text_rgba : list of float, Optional
///     Sets the text color in rgba format as [r, g, b, a].
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a radio group's ``style_id``.
#[pyfunction]
#[pyo3(signature = (
    background_color=None,
    background_color_alpha=None,
    background_rgba=None,
    background_color_hovered=None,
    background_color_hovered_alpha=None,
    background_rgba_hovered=None,
    border_color=None,
    border_color_alpha=None,
    border_rgba=None,
    border_width=None,
    dot_color=None,
    dot_color_alpha=None,
    dot_rgba=None,
    dot_color_hovered=None,
    dot_color_hovered_alpha=None,
    dot_rgba_hovered=None,
    text_color=None,
    text_color_alpha=None,
    text_rgba=None,
    gen_id=None
    ))]
pub fn add_radio_style(
    background_color: Option<IpgColor>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    background_color_hovered: Option<IpgColor>,
    background_color_hovered_alpha: Option<f32>,
    background_rgba_hovered: Option<[f32; 4]>,
    border_color: Option<IpgColor>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_width: Option<f32>,
    dot_color: Option<IpgColor>,
    dot_color_alpha: Option<f32>,
    dot_rgba: Option<[f32; 4]>,
    dot_color_hovered: Option<IpgColor>,
    dot_color_hovered_alpha: Option<f32>,
    dot_rgba_hovered: Option<[f32; 4]>,
    text_color: Option<IpgColor>,
    text_color_alpha: Option<f32>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background_color = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba, background_color, background_color_alpha);
    let background_color_hovered = 
        IpgColor::rgba_ipg_color_to_iced(background_rgba_hovered, background_color_hovered, background_color_hovered_alpha);
    let dot_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(dot_rgba, dot_color, dot_color_alpha);
    let dot_color_hovered: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(dot_rgba_hovered, dot_color_hovered, dot_color_hovered_alpha);
    let border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba, border_color, border_color_alpha);
    let text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, text_color_alpha);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgRadioStyle(
        IpgRadioStyle {
            id,
            background_color,
            background_color_hovered,
            dot_color,
            dot_color_hovered,
            border_color,
            border_width,
            text_color,
        }));

    drop(state);
    Ok(id)

}
