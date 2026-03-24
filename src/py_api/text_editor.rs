//! TextEditor module - provides add_text_editor pyfunction

use iced::widget::text_editor::{Content, Status};
use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::py_api::helpers::get_length;
use crate::state::{IpgWidgets, get_id, set_state_of_widget};
use crate::widgets::ipg_text::TextWrapping;
use crate::widgets::ipg_text_editor::IpgTextEditor;



#[pyfunction]
#[pyo3(signature = (
    parent_id,
    place_holder=None, 
    font_id=None,
    text_size=None,
    line_height=None,
    width=None,
    width_fill=false,
    height=None,
    height_fill=false,
    min_height=None,
    max_height=None,
    padding=None,
    wrapping=None,
))]
pub fn add_text_editor(
    parent_id: String,
    place_holder: Option<String>, 
    font_id: Option<usize>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    width: Option<f32>,
    width_fill: bool,
    height: Option<f32>,
    height_fill: bool,
    min_height: Option<f32>,
    max_height: Option<f32>,
    padding: Option<Vec<f32>>,
    wrapping: Option<TextWrapping>,
) ->PyResult<usize> {

    let id = get_id(None);
    
    // Calculate dimensions
    let width = get_length(width, width_fill);
    let height = get_length(height, height_fill);

    // Register widget with parent
    set_state_of_widget(id, parent_id.clone());

    // Create and store button
    let mut state = access_state();
    
    state.widgets.insert(
        id,
        IpgWidgets::IpgTextEditor(
            IpgTextEditor { 
                id, 
                content: Content::new(),
                place_holder, 
                font_id, 
                text_size, 
                line_height, 
                width, 
                height, 
                min_height, 
                max_height, 
                padding, 
                wrapping, 
                last_status: Status::Disabled
            }),
        );
    drop(state);

    Ok(id)
}