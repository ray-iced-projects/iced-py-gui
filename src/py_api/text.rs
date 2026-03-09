//! Text module - provides add_text pyfunction
use pyo3::{pyfunction, PyResult};

use crate::py_api::helpers::{get_height, get_width};
use crate::state::{IpgWidgets, get_id, set_state_of_widget}; 
use crate::widgets::ipg_text::{IpgText, TextShaping, TextWrapping};
use crate::access_state; 
use crate::graphics::colors::IpgColor;

/// Add a text widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    content, 
    gen_id=None, 
    width=None, 
    width_fill=false, 
    height=None, 
    height_fill=false,
    center=true,
    align_top_left=None,
    align_top_center=None,
    align_top_right=None,
    align_center_left=None,
    align_center=None,
    align_center_right=None,
    align_bottom_left=None,
    align_bottom_center=None,
    align_bottom_right=None,
    line_height=None, 
    size=None,
    font_id=None, 
    text_shaping=None, 
    text_color=None, 
    text_rgba=None,
    text_wrapping=None,
    show=true,
    ))]
pub fn add_text(
    parent_id: String,
    content: String,
    // ** above required
    gen_id: Option<usize>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    center: Option<bool>,
    align_top_left: Option<bool>,
    align_top_center: Option<bool>,
    align_top_right: Option<bool>,
    align_center_left: Option<bool>,
    align_center: Option<bool>,
    align_center_right: Option<bool>,
    align_bottom_left: Option<bool>,
    align_bottom_center: Option<bool>,
    align_bottom_right: Option<bool>,
    line_height: Option<f32>,
    size: Option<f32>,
    font_id: Option<usize>,
    text_shaping: Option<TextShaping>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    text_wrapping: Option<TextWrapping>,
    show: bool,
    ) -> PyResult<usize> 
{

    let id = get_id(gen_id);

    let color= 
        IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, 1.0, false);

    let width = get_width(width, width_fill);
    let height = get_height(height, height_fill);

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
            center,
            align_top_left,
            align_top_center,
            align_top_right,
            align_center_left,
            align_center,
            align_center_right,
            align_bottom_left,
            align_bottom_center,
            align_bottom_right,
            font_id,
            shaping: text_shaping,
            show,
            wrapping: text_wrapping,
            color,
        }));

    drop(state);
    Ok(id)

}