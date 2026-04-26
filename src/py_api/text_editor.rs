//! TextEditor module - provides add_text_editor pyfunction

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::state::{Widgets, get_id, set_state_of_widget};
use crate::widgets::ipg_text_editor::TextEditor;
use crate::widgets::ipg_text_editor::TxtEdStatus;



#[pyfunction]
#[pyo3(signature = (
    parent_id,
    place_holder=None, 
    font_id=None,
    text_size=None,
    line_height=None,
    width=None,
    width_fill=None,
    height=None,
    height_fill=None,
    fill=None,
    min_height=None,
    max_height=None,
    padding=None,
    wrapping_none=None,
    wrapping_glyph=None,
    wrapping_word_glyph=None,
))]
pub fn add_text_editor(
    parent_id: String,
    place_holder: Option<String>, 
    font_id: Option<usize>,
    text_size: Option<f32>,
    line_height: Option<f32>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    min_height: Option<f32>,
    max_height: Option<f32>,
    padding: Option<Vec<f32>>,
    wrapping_none: Option<bool>,
    wrapping_glyph: Option<bool>,
    wrapping_word_glyph: Option<bool>,
) ->PyResult<usize> {

    let id = get_id(None);
    
    // Register widget with parent
    set_state_of_widget(id, parent_id.clone());

    // Create and store button
    let mut state = access_state();
    
    state.widgets.insert(
        id,
        Widgets::TextEditor(
            TextEditor { 
                id, 
                content: iced::widget::text_editor::Content::new(),
                place_holder, 
                font_id, 
                text_size, 
                line_height, 
                width,
                width_fill, 
                height,
                height_fill,
                fill,
                min_height, 
                max_height, 
                padding, 
                wrapping_none,
                wrapping_glyph,
                wrapping_word_glyph,
                last_status: TxtEdStatus::Disabled
            }),
        );
    drop(state);

    Ok(id)
}