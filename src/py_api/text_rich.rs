//! Rich text module - provides add_rich_text and add_span pyfunctions
use pyo3::{pyfunction, PyResult};

use iced::{Font, Padding};

use crate::access_state;
use crate::graphics::colors::IpgColor;
use crate::state::{IpgWidgets, get_id, set_state_of_widget};
use crate::widgets::ipg_text_rich::{IpgRichText, IpgSpan};


/// Add a rich text widget.
///
/// A rich text widget that contains styled spans of text.
/// Use ``add_span`` to add text spans to this widget.
///
/// Parameters
/// ----------
/// parent_id : str
///     The id of the parent container.
/// size : float, Optional
///     The default text size for all spans.
/// line_height : float, Optional
///     The default line height for all spans.
/// text_color : IpgColor, Optional
///     The default text color for all spans.
/// text_rgba : list[float, 4], Optional
///     The default text color in rgba format.
/// show : bool
///     Whether the widget is visible.
/// gen_id : int, Optional
///     Obtains an ID of a widget that has not been created.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created rich text.
#[pyfunction]
#[pyo3(signature = (
    parent_id,
    size=None,
    line_height=None,
    text_color=None,
    text_rgba=None,
    show=true,
    gen_id=None,
))]
pub fn add_rich_text(
    parent_id: String,
    size: Option<f32>,
    line_height: Option<f32>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    show: bool,
    gen_id: Option<usize>,
) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let color = IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, 1.0, false);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgRichText(
        IpgRichText {
            id,
            parent_id,
            spans: vec![],
            size,
            line_height,
            color,
            padding: Padding::ZERO,
            show,
        }));

    drop(state);
    Ok(id)
}


/// Add a span to a rich text widget.
///
/// A span of styled text within a rich text widget.
///
/// Parameters
/// ----------
/// rich_text_id : int
///     The ID of the rich text widget to add the span to.
/// text : str
///     The text content of the span.
/// size : float, Optional
///     The text size for this span.
/// line_height : float, Optional
///     The line height for this span.
/// text_color : IpgColor, Optional
///     The text color in IpgColor format.
/// text_rgba : list[float, 4], Optional
///     The text color in rgba format.
/// bold : bool
///     Whether the text is bold.
/// italic : bool
///     Whether the text is italic.
/// underline : bool
///     Whether the text is underlined.
/// strikethrough : bool
///     Whether the text has strikethrough.
/// gen_id : int, Optional
///     Obtains an ID for the span.
///
/// Returns
/// -------
/// int
///     The numeric ID of the newly created span.
#[pyfunction]
#[pyo3(signature = (
    rich_text_id,
    text,
    size=None,
    line_height=None,
    text_color=None,
    text_rgba=None,
    bold=false,
    italic=false,
    underline=false,
    strikethrough=false,
    gen_id=None,
))]
pub fn add_span(
    rich_text_id: usize,
    text: String,
    size: Option<f32>,
    line_height: Option<f32>,
    text_color: Option<IpgColor>,
    text_rgba: Option<[f32; 4]>,
    bold: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
    gen_id: Option<usize>,
) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let color = IpgColor::rgba_ipg_color_to_iced(text_rgba, text_color, 1.0, false);

    let font = if bold || italic {
        let weight = if bold { 
            iced::font::Weight::Bold 
        } else { 
            iced::font::Weight::Normal 
        };
        let style = if italic { 
            iced::font::Style::Italic 
        } else { 
            iced::font::Style::Normal 
        };
        Some(Font { weight, style, ..Font::default() })
    } else {
        None
    };

    let mut state = access_state();

    let rt = state.widgets.get_mut(&rich_text_id)
        .expect("add_span: rich_text_id not found in widgets");

    let rt = rt.as_rich_text_mut()
        .expect("add_span: widget is not an IpgRichText");

    rt.spans.push(IpgSpan {
        id,
        rich_text_id,
        text,
        size,
        line_height,
        color,
        font,
        highlight: None,
        padding: None,
        underline,
        strikethrough,
    });

    drop(state);
    Ok(id)
}


