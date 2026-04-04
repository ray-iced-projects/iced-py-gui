//! Text module - provides add_text pyfunction
use pyo3::{pyfunction, PyResult};

use crate::py_api::helpers::get_length;
use crate::state::{IpgWidgets, get_id, set_state_of_widget}; 
use crate::widgets::ipg_text::{IpgText, TextShaping, TextWrapping};
use crate::access_state; 
use crate::graphics::colors::IpgColor;

/// Add a text widget.
///
/// A static text label for displaying content.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this text belongs to.
/// content : str
///     Sets the text content to display.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the text fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the text fills available height.
/// align_bottom_center : bool, Optional
///     Whether to Align the text to the bottom-centre.
/// align_bottom_left : bool, Optional
///     Whether to Align the text to the bottom-left.
/// align_bottom_right : bool, Optional
///     Whether to Align the text to the bottom-right.
/// align_center_left : bool, Optional
///     Whether to Align the text to the centre-left.
/// align_center_right : bool, Optional
///     Whether to Align the text to the centre-right.
/// align_center : bool, Optional
///     Whether to Align the text to the centre.
/// align_top_center : bool, Optional
///     Whether to Align the text to the top-centre.
/// align_top_left : bool, Optional
///     Whether to Align the text to the top-left.
/// align_top_right : bool, Optional
///     Whether to Align the text to the top-right.
/// line_height : float, Optional
///     Sets the line height for the text.
/// size : float, Optional
///     Sets the font size for the text.
/// font_id : int, Optional
///     Sets the Font ID for the text.
/// text_shaping : TextShaping, Optional
///     Sets the Text shaping strategy.
/// text_color : IpgColor, Optional
///     Sets the text color using a predefined color variant.
/// text_color_alpha : float, Optional
///     Sets the alpha of the IpgColor.
/// text_rgba : list of float, Optional
///     Sets the text color in rgba format as [r, g, b, a].
/// text_wrapping : TextWrapping, Optional
///     Sets the Text wrapping strategy.
/// show : bool, default True
///     Whether the text is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created text.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    content, 
    gen_id=None, 
    width=None, 
    width_fill=false, 
    height=None, 
    height_fill=false,
    fill=None,
    align_bottom_center=None,
    align_bottom_left=None,
    align_bottom_right=None,
    align_center_left=None,
    align_center_right=None,
    align_center=None,
    align_top_center=None,
    align_top_left=None,
    align_top_right=None,
    line_height=None, 
    size=None,
    font_id=None, 
    text_shaping=None, 
    text_color=None,
    text_color_alpha=None,
    text_rgba=None,
    text_wrapping=None,
    show=true,
    ))]
pub fn add_text(
    parent_id: String,
    content: String,
    gen_id: Option<usize>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    fill: Option<bool>,
    align_bottom_center: Option<bool>,
    align_bottom_left: Option<bool>,
    align_bottom_right: Option<bool>,
    align_center_left: Option<bool>,
    align_center_right: Option<bool>,
    align_center: Option<bool>,
    align_top_center: Option<bool>,
    align_top_left: Option<bool>,
    align_top_right: Option<bool>,
    line_height: Option<f32>,
    size: Option<f32>,
    font_id: Option<usize>,
    text_shaping: Option<TextShaping>,
    text_color: Option<IpgColor>,
    text_color_alpha: Option<f32>,
    text_rgba: Option<[f32; 4]>,
    text_wrapping: Option<TextWrapping>,
    show: bool,
    ) -> PyResult<usize> 
{

    let id = get_id(gen_id);

    let color= 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, text_color_alpha);

    let (width, height) = if fill == Some(true) {
        (get_length(None, true), get_length(None, true))
    } else {
        (get_length(width, width_fill), get_length(height, height_fill))
    };

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();
    
    state.widgets.insert(id, IpgWidgets::IpgText(
        IpgText {
            id,
            parent_id,
            content,
            size,
            line_height,
            width,
            height,
            align_bottom_center,
            align_bottom_left,
            align_bottom_right,
            align_center_left,
            align_center_right,
            align_center,
            align_top_center,
            align_top_left,
            align_top_right,
            font_id,
            shaping: text_shaping,
            show,
            wrapping: text_wrapping,
            color,
        }));

    drop(state);
    Ok(id)

}