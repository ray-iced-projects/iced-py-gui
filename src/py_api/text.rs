//! Text module - provides add_text pyfunction
use pyo3::{Py, PyAny, pyfunction, PyResult};
type PyObject = Py<PyAny>;
use crate::state::{Widgets, get_id, set_state_of_widget}; 
use crate::{access_state, add_user_data_to_mutex}; 
use crate::graphics::colors::Color;
use crate::widgets::ipg_text::{Text, TextStyleStd, TextFormat, TextStyle};

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
/// format_id: int, Optional
///     Sets the id via add_text_format() method.
/// font_id : int, Optional
///     Sets the id via add_font() method.
/// selectable: bool, default False
///     Whether the text is selctable.
/// style_id: int, Optional
///     sets the id via add_text_style() method.
/// selection_style_id: int, Optional
///     Sets the id via add_text_style() method.
/// style_std: TextStyleStd, Optional
///     Sets a standard style of TextStylStd.
/// selection_style_std: TextStyleStd, Optional
///     Sets a standard style of TextStyleStd.
/// show : bool, default True
///     Whether the widget is visible.
/// user_data: any, Optional
///     attaches some user data to the method.
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
    format_id=None,
    font_id=None,
    selectable=false,
    style_id=None,
    selection_style_id=None,
    style_std=None,
    show=true,
    user_data=None,
    gen_id=None, 
    ))]
pub fn add_text(
    parent_id: String,
    content: String,
    format_id: Option<usize>,
    font_id: Option<usize>,
    selectable: bool,
    style_id: Option<usize>,
    selection_style_id: Option<usize>,
    style_std: Option<TextStyleStd>,
    show: bool,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{

    let id = get_id(gen_id);

    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();
    
    state.widgets.insert(id, Widgets::Text(
        Text {
            id,
            content,
            format_id,
            font_id,
            selectable,
            style_id,
            selection_style_id,
            style_std,
            show,
        }));

    drop(state);
    Ok(id)

}


/// Add styling to a text widget.
///
/// Styling the text content.
///
/// Parameters
/// ----------
/// color : Color, Optional
///     Sets the text color using a predefined color variant.
/// color_alpha : float, Optional
///     Sets the alpha of the Color.
/// color_rgba : list of float, Optional
///     Sets the text color in rgba format as [r, g, b, a].
/// selection_color : Color, Optional
///     Sets the text color using a predefined color variant.
/// selection_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// selection_color_rgba : list of float, Optional
///     Sets the text color in rgba format as [r, g, b, a].
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created text.
#[pyfunction]
#[pyo3(signature = (
    color=None,
    color_alpha=None,
    color_rgba=None,
    selection_color=None,
    selection_color_alpha=None,
    selection_color_rgba=None,
    ))]
pub fn add_text_style(
    color: Option<Color>,
    color_alpha: Option<f32>,
    color_rgba: Option<[f32; 4]>,
    selection_color: Option<Color>,
    selection_color_alpha: Option<f32>,
    selection_color_rgba: Option<[f32; 4]>,
    ) -> PyResult<usize> 
{

    let id = get_id(None);

    let mut state = access_state();
    
    state.widgets.insert(id, Widgets::TextStyle(
        TextStyle {
            id,
            color,
            color_alpha,
            color_rgba,
            selection_color,
            selection_color_alpha,
            selection_color_rgba,
        }));

    drop(state);
    Ok(id)

}

/// Add a text formatting.
///
/// A method for formatting a text widget.
///
/// Parameters
/// ----------
/// align_bottom_center : bool, Optional
///     Whether to Align the text to the bottom-centre.
/// align_bottom_left : bool, Optional
///     Whether to Align the text to the bottom-left.
/// align_bottom_right : bool, Optional
///     Whether to Align the text to the bottom-right.
/// align_center : bool, Optional
///     Whether to Align the text to the centre.
/// align_center_left : bool, Optional
///     Whether to Align the text to the centre-left.
/// align_center_right : bool, Optional
///     Whether to Align the text to the centre-right.
/// align_top_center : bool, Optional
///     Whether to Align the text to the top-centre.
/// align_top_left : bool, Optional
///     Whether to Align the text to the top-left.
/// align_top_right : bool, Optional
///     Whether to Align the text to the top-right.
/// font_id : int, Optional
///     Sets the Font ID for the text.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the text fills available height.
/// line_height : float, Optional
///     Sets the line height for the text.
/// size : float, Optional
///     Sets the font size for the text.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the text fills available width.
/// wrapping_glyph: bool, Optional
///     Whether to wrap on a glyph.
/// wrapping_none: Optional bool
///     Whether to have no text wrapping, default=word wrap.
/// wrapping_word_glyph: bool, Optional
///     Whether to wrap on a word, then glyph
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created text.
#[pyfunction]
#[pyo3(signature = (
    align_bottom_center=None,
    align_bottom_left=None,
    align_bottom_right=None,
    align_center_left=None,
    align_center_right=None,
    align_center=None,
    align_top_center=None,
    align_top_left=None,
    align_top_right=None,
    fill=None,
    height_fill=None,
    height=None, 
    line_height=None,  
    size=None,
    width_fill=None, 
    width=None, 
    wrapping_glyph=None,
    wrapping_none=None,
    wrapping_word=None,
    wrapping_word_glyph=None,
    ))]
pub fn add_text_format(
    align_bottom_center: Option<bool>,
    align_bottom_left: Option<bool>,
    align_bottom_right: Option<bool>,
    align_center_left: Option<bool>,
    align_center_right: Option<bool>,
    align_center: Option<bool>,
    align_top_center: Option<bool>,
    align_top_left: Option<bool>,
    align_top_right: Option<bool>,
    fill: Option<bool>,
    height_fill: Option<bool>,
    height: Option<f32>,
    line_height: Option<f32>,
    size: Option<f32>,
    width_fill: Option<bool>,
    width: Option<f32>,
    wrapping_glyph: Option<bool>,
    wrapping_none: Option<bool>,
    wrapping_word: Option<bool>,
    wrapping_word_glyph: Option<bool>,
    ) -> PyResult<usize> 
{

    let id = get_id(None);

    let mut state = access_state();
    
    state.widgets.insert(id, Widgets::TextFormat(
        TextFormat {
            id,
            align_bottom_center,
            align_bottom_left,
            align_bottom_right,
            align_center_left,
            align_center_right,
            align_center,
            align_top_center,
            align_top_left,
            align_top_right,
            fill,
            height_fill,
            height,
            line_height,
            size,
            width_fill,
            width,
            wrapping_glyph,
            wrapping_none,
            wrapping_word,
            wrapping_word_glyph,
        }));

    drop(state);
    Ok(id)

}
