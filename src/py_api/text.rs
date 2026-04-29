//! Text module - provides add_text pyfunction
use pyo3::{pyfunction, PyResult};

use crate::state::{Widgets, get_id, set_state_of_widget}; 
use crate::access_state; 
use crate::graphics::colors::Color;
use crate::widgets::ipg_text::{Text, TextColorStd};

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
/// font_id : int, Optional
///     Sets the Font ID for the text.
/// size : float, Optional
///     Sets the font size for the text.
/// line_height : float, Optional
///     Sets the line height for the text.
/// shaping_advanced : bool, Optional
///     Sets the Text shaping strategy.
/// color : Color, Optional
///     Sets the text color using a predefined color variant.
/// color_alpha : float, Optional
///     Sets the alpha of the Color.
/// color_rgba : list of float, Optional
///     Sets the text color in rgba format as [r, g, b, a].
/// color_std: TextColorStd, Optional
///     Selected standard colors the the text.
/// wrapping_none: Optional bool
///     Whether to have no text wrapping, default=word wrap.
/// wrapping_glyph: bool, Optional
///     Whether to wrap on a glyph.
/// wrapping_word_glyph: bool, Optional
///     Whether to wrap on a word, then glyph
/// show : bool, default True
///     Whether the widget is visible.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created text.
#[pyfunction]
#[pyo3(signature = (
    parent_id, 
    content, 
    width=None, 
    width_fill=None, 
    height=None, 
    height_fill=None,
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
    font_id=None,
    size=None,
    line_height=None,  
    color=None,
    color_alpha=None,
    color_rgba=None,
    color_std=None,
    wrapping_none=None,
    wrapping_glyph=None,
    wrapping_word_glyph=None,
    show=true,
    gen_id=None, 
    ))]
pub fn add_text(
    parent_id: String,
    content: String,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
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
    font_id: Option<usize>,
    size: Option<f32>,
    line_height: Option<f32>,
    color: Option<Color>,
    color_alpha: Option<f32>,
    color_rgba: Option<[f32; 4]>,
    color_std: Option<TextColorStd>,
    wrapping_none: Option<bool>,
    wrapping_glyph: Option<bool>,
    wrapping_word_glyph: Option<bool>,
    show: bool,
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{

    let id = get_id(gen_id);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();
    
    state.widgets.insert(id, Widgets::Text(
        Text {
            id,
            content,
            width,
            width_fill,
            height,
            height_fill,
            fill,
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
            size,
            line_height,
            color,
            color_alpha,
            color_rgba,
            color_std,
            wrapping_none,
            wrapping_glyph,
            wrapping_word_glyph,
            show,
        }));

    drop(state);
    Ok(id)

}
