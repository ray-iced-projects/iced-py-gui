//! Text module - provides add_text pyfunction
use pyo3::{pyfunction, PyResult};

use crate::py_api::helpers::{get_height, get_width};
use crate::state::{IpgWidgets, get_id, set_state_of_widget}; 
use crate::widgets::ipg_text::{IpgText, IpgWrapping};
use crate::widgets::enums::{IpgAlignmentX, IpgShaping, IpgAlignmentY};
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
    align_x=None, 
    align_y=None,
    line_height=None, 
    size=None,
    font_id=None, 
    shaping=None, 
    text_color=None, 
    text_rgba=None,
    wrapping=None,
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
    align_x: Option<IpgAlignmentX>,
    align_y: Option<IpgAlignmentY>,
    line_height: Option<f32>,
    size: Option<f32>,
    font_id: Option<usize>,
    shaping: Option<IpgShaping>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    wrapping: Option<IpgWrapping>,
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
            align_x,
            align_y,
            font_id,
            shaping,
            show,
            wrapping,
            color,
        }));

    drop(state);
    Ok(id)

}