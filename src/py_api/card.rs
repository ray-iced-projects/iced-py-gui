//! Card module - provides add_card pyfunction

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::colors::Color;
use crate::state::{Containers, Widgets, access_state, add_callback_to_mutex, get_id, set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::ipg_card::{Card, CardStyle, CardStyleStd};


/// Add a card widget.
///
/// A card with a head, body, and optional foot section.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this card belongs to.
/// head : str, Optional
///     Sets the Text displayed in the card header.
/// body : str, Optional
///     Sets the Text displayed in the card body.
/// is_open : bool, default True
///     Whether the card is open (expanded).
/// min_max_id : int, Optional
///     Sets the Widget ID of an external button used to toggle the card open/closed.
/// foot : str, Optional
///     Sets the Text displayed in the card footer.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// close_size : float, Optional
///     Sets the Size of the close button in logical pixels.
/// on_close : callable, Optional
///     Sets the Callback method to invoke when the card is closed.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// width_fill : bool, default False
///     Whether the card fills available width.
/// height : float, Optional
///     Sets the Fixed height in logical pixels.
/// height_fill : bool, default False
///     Whether the card fills available height.
/// max_width : float, Optional
///     Sets the Maximum width in logical pixels.
/// max_height : float, Optional
///     Sets the Maximum height in logical pixels.
/// padding : list of float, Optional
///     Sets the Padding for all sections as [all], [vertical, horizontal], or
///     [top, right, bottom, left].
/// padding_head : list of float, Optional
///     Sets the Padding for the header section.
/// padding_body : list of float, Optional
///     Sets the Padding for the body section.
/// padding_foot : list of float, Optional
///     Sets the Padding for the footer section.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_card_style``.
/// style_std : CardStyleStd, Optional
///     Sets a predefined standard style variant.
/// style_button : int, Optional
///     Sets the ID of a button style for the close button.
/// show : bool, default True
///     Whether the card is visible.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created card.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,
    head=None, 
    body=None,      
    is_open=true,
    foot=None, 
    close_icon=None,
    close_size=None,
    on_close=None,
    on_open=None, 
    width=None, 
    width_fill=None, 
    height=None, 
    height_fill=None,
    fill=None,
    max_width=None, 
    max_height=None,
    padding=None, 
    // padding_head=None, 
    padding_body=None, 
    padding_foot=None,
    style_id=None,
    style_std=None,
    style_button=None,
    show=true, 
    user_data=None,
    gen_id=None,
    ))]
pub fn add_card(
    window_id: String,
    container_id: String,
    parent_id: Option<String>, 
    head: Option<String>,
    body: Option<String>,
    is_open: bool,
    foot: Option<String>,
    close_icon: Option<bool>,
    close_size: Option<f32>,
    on_close: Option<PyObject>,
    on_open: Option<PyObject>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    fill: Option<bool>,
    max_width: Option<f32>,
    max_height: Option<f32>,
    padding: Option<Vec<f32>>,
    // padding_head: Option<Vec<f32>>,
    padding_body: Option<Vec<f32>>,
    padding_foot: Option<Vec<f32>>,
    style_id: Option<usize>,
    style_std: Option<CardStyleStd>,
    style_button: Option<usize>,
    show: bool,
    user_data: Option<PyObject>, 
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    if let Some(py) = on_close {
        add_callback_to_mutex(id, "on_close".to_string(), py);
    }

    if let Some(py) = on_open {
        add_callback_to_mutex(id, "on_open".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_container".to_string());

    state.containers.insert(id, Containers::Card(
        Card {
            id,
            is_open,
            width,
            width_fill,
            height,
            height_fill,
            fill,
            max_width,
            max_height,
            padding,
            // padding_head,
            padding_body,
            padding_foot,
            close_icon,
            close_size,
            head,
            body,
            foot,
            style_id,
            style_std,
            style_button,
            show,
        }));

    drop(state);
    Ok(id)

}


/// Add styling to a card.
///
/// Creates a custom style that can be applied to a card
/// via its ``style_id`` parameter.
///
/// Parameters
/// ----------
/// background_color : Color, Optional
///     Sets the background color using a predefined color variant.
/// background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// background_rgba : list of float, Optional
///     Sets the background color in rgba format as [r, g, b, a].
/// border_radius : float, Optional
///     Sets the border radius in logical pixels.
/// border_width : float, Optional
///     Sets the border width in logical pixels.
/// border_color : Color, Optional
///     Sets the border color using a predefined color variant.
/// border_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// border_rgba : list of float, Optional
///     Sets the border color in rgba format as [r, g, b, a].
/// head_background_color : Color, Optional
///     Sets the header background color using a predefined color variant.
/// head_background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// head_background_rgba : list of float, Optional
///     Sets the header background color in rgba format as [r, g, b, a].
/// head_text_color : Color, Optional
///     Sets the header text color using a predefined color variant.
/// head_text_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// head_text_rgba : list of float, Optional
///     Sets the header text color in rgba format as [r, g, b, a].
/// body_background_color : Color, Optional
///     Sets the body background color using a predefined color variant.
/// body_background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// body_background_rgba : list of float, Optional
///     Sets the body background color in rgba format as [r, g, b, a].
/// body_text_color : Color, Optional
///     Sets the body text color using a predefined color variant.
/// body_text_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// body_text_rgba : list of float, Optional
///     Sets the body text color in rgba format as [r, g, b, a].
/// foot_background_color : Color, Optional
///     Sets the footer background color using a predefined color variant.
/// foot_background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// foot_background_rgba : list of float, Optional
///     Sets the footer background color in rgba format as [r, g, b, a].
/// foot_text_color : Color, Optional
///     Sets the footer text color using a predefined color variant.
/// foot_text_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// foot_text_rgba : list of float, Optional
///     Sets the footer text color in rgba format as [r, g, b, a].
/// close_color : Color, Optional
///     Sets the close button color using a predefined color variant.
/// close_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// close_rgba : list of float, Optional
///     Sets the close button color in rgba format as [r, g, b, a].
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a card's ``style_id``.
#[pyfunction]
#[pyo3(signature = ( 
    background_color=None,
    background_color_alpha=None,
    background_rgba=None,
    border_radius=None, 
    border_width=None,
    border_color=None,
    border_color_alpha=None,
    border_rgba=None, 
    head_background_color=None,
    head_background_color_alpha=None,
    head_background_rgba=None, 
    head_text_color=None,
    head_text_color_alpha=None,
    head_text_rgba=None,
    body_background_color=None,
    body_background_color_alpha=None,
    body_background_rgba=None, 
    body_text_color=None,
    body_text_color_alpha=None,
    body_text_rgba=None, 
    foot_background_color=None,
    foot_background_color_alpha=None,
    foot_background_rgba=None, 
    foot_text_color=None,
    foot_text_color_alpha=None,
    foot_text_rgba=None, 
    close_color=None,
    close_color_alpha=None,
    close_rgba=None,
    gen_id=None
    ))]
pub fn add_card_style(
    background_color: Option<Color>,
    background_color_alpha: Option<f32>,
    background_rgba: Option<[f32; 4]>,
    border_radius: Option<f32>, 
    border_width: Option<f32>, 
    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>, 
    head_background_color: Option<Color>,
    head_background_color_alpha: Option<f32>,
    head_background_rgba: Option<[f32; 4]>, 
    head_text_color: Option<Color>,
    head_text_color_alpha: Option<f32>,
    head_text_rgba: Option<[f32; 4]>,
    body_background_color: Option<Color>,
    body_background_color_alpha: Option<f32>,
    body_background_rgba: Option<[f32; 4]>, 
    body_text_color: Option<Color>,
    body_text_color_alpha: Option<f32>,
    body_text_rgba: Option<[f32; 4]>, 
    foot_background_color: Option<Color>,
    foot_background_color_alpha: Option<f32>,
    foot_background_rgba: Option<[f32; 4]>, 
    foot_text_color: Option<Color>,
    foot_text_color_alpha: Option<f32>,
    foot_text_rgba: Option<[f32; 4]>, 
    close_color: Option<Color>,
    close_color_alpha: Option<f32>,
    close_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let background = 
        Color::rgba_ipg_color_to_iced(background_rgba, &background_color, background_color_alpha);
    let border_color = 
        Color::rgba_ipg_color_to_iced(border_rgba, &border_color, border_color_alpha);
    let head_background = 
        Color::rgba_ipg_color_to_iced(head_background_rgba, &head_background_color, head_background_color_alpha);
    let body_background = 
        Color::rgba_ipg_color_to_iced(body_background_rgba, &body_background_color, body_background_color_alpha);
    let foot_background = 
        Color::rgba_ipg_color_to_iced(foot_background_rgba, &foot_background_color, foot_background_color_alpha);
    let head_text_color = 
        Color::rgba_ipg_color_to_iced(head_text_rgba, &head_text_color, head_text_color_alpha);
    let body_text_color = 
        Color::rgba_ipg_color_to_iced(body_text_rgba, &body_text_color, body_text_color_alpha);
    let foot_text_color = 
        Color::rgba_ipg_color_to_iced(foot_text_rgba, &foot_text_color, foot_text_color_alpha);
    let close_color = 
        Color::rgba_ipg_color_to_iced(close_rgba, &close_color, close_color_alpha);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::CardStyle(
        CardStyle {
            id,
            background,
            border_radius,
            border_width,
            border_color,
            head_background, 
            head_text_color, 
            body_background, 
            body_text_color, 
            foot_background, 
            foot_text_color, 
            close_color,
        }));


    drop(state);
    Ok(id)
}
