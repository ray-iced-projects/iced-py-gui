

use pyo3::{PyResult, pyfunction};

use crate::{access_state, graphics::colors::IpgColor, 
    py_api::helpers::get_length, state::{IpgWidgets, 
        get_id, set_state_of_widget}, widgets::ipg_separator::
        {IpgSeparator, IpgSeparatorStyle, IpgSeparatorType}};


/// Add a separator widget.
///
/// A visual separator using lines, dots, or a labelled divider.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this separator belongs to.
/// label : str, Optional
///     Sets the text label displayed in the separator.
/// separator_type : IpgSeparatorType, Optional
///     Sets the type of separator (line, dot, label, etc.).
/// label_left_width : float, Optional
///     Sets the width of the line to the left of the label.
/// label_right_width : float, Optional
///     Sets the width of the line to the right of the label.
/// dot_radius : float, Optional
///     Sets the radius of each dot in logical pixels.
/// dot_count : int, Optional
///     Sets the number of dots to display.
/// dot_fill : bool, default True
///     Whether the dots are filled.
/// dot_border_width : float, Optional
///     Sets the border width of each dot in logical pixels.
/// line_length : float, Optional
///     Sets the length of the separator line in logical pixels.
/// line_thickness : float, Optional
///     Sets the thickness of the separator line in logical pixels.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the separator fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the separator fills available height.
/// spacing : float, Optional
///     Sets the spacing between separator elements in logical pixels.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_separator_style``.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// show : bool, default True
///     Whether the separator is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created separator.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    label=None,
    separator_type=None, 
    label_left_width=None,
    label_right_width=None,
    dot_radius=None, 
    dot_count=None,
    dot_fill=true, 
    dot_border_width=None,
    line_length=None,
    line_thickness=None,
    width=None, 
    width_fill=false, 
    height=None, 
    height_fill=false,
    spacing=None, 
    style_id=None,
    gen_id=None, 
    show=true
    ))]
pub fn add_separator(
    parent_id: String,
    label: Option<String>,
    separator_type: Option<IpgSeparatorType>,
    label_left_width: Option<f32>,
    label_right_width: Option<f32>,
    dot_radius: Option<f32>,
    dot_count: Option<u32>,
    dot_fill: bool,
    dot_border_width: Option<f32>,
    line_length: Option<f32>,
    line_thickness: Option<f32>,
    width: Option<f32>, 
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    spacing: Option<f32>,
    style_id: Option<usize>,
    gen_id: Option<usize>,
    show: bool,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let width = get_length(width, width_fill);
    let height = get_length(height, height_fill);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgSeparator(
        IpgSeparator {
            id,
            parent_id,
            separator_type,
            label,
            label_left_width,
            label_right_width,
            dot_radius,
            dot_count,
            dot_fill,
            dot_border_width,
            line_length,
            line_thickness,
            width,
            height,
            spacing,
            style_id,
            show,
        }));

    drop(state);
    Ok(id)

}


/// Add styling to a separator.
///
/// Creates a custom style that can be applied to a separator
/// via its ``style_id`` parameter.
///
/// Parameters
/// ----------
/// ipg_color : IpgColor, Optional
///     Sets the separator color using a predefined color variant.
/// rgba_color : list of float, Optional
///     Sets the separator color in rgba format as [r, g, b, a].
/// border_ipg_color : IpgColor, Optional
///     Sets the border color using a predefined color variant.
/// border_rgba_color : list of float, Optional
///     Sets the border color in rgba format as [r, g, b, a].
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a separator's ``style_id``.
#[pyfunction]
#[pyo3(signature = (
    ipg_color=None,
    rgba_color=None,
    border_ipg_color=None,
    border_rgba_color=None,
    gen_id=None,
    ))]
pub fn add_separator_style(
    ipg_color: Option<IpgColor>,
    rgba_color: Option<[f32; 4]>,
    border_ipg_color: Option<IpgColor>,
    border_rgba_color: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let color = 
        IpgColor::rgba_ipg_color_to_iced(rgba_color, ipg_color, 1.0);
    let border_color = 
        IpgColor::rgba_ipg_color_to_iced(border_rgba_color, border_ipg_color, 1.0);

    let mut state = access_state();
    
    state.widgets.insert(id, IpgWidgets::IpgSeparatorStyle(
        IpgSeparatorStyle {
            id,
            color,
            border_color,
        }));

    drop(state);
    Ok(id)
}
