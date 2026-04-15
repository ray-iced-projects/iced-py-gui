//! Card module - provides add_card pyfunction

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::add_user_data_to_mutex;
use crate::graphics::colors::Color;
use crate::state::{Containers, Widgets, access_state, add_callback_to_mutex, get_id, set_state_cont_wnd_ids, set_state_of_container};
use crate::widgets::ipg_card::{Card, CardStyle, CardStyleStd};



/// Add a card container.
///
/// Card excepts the addition of 1 to 3 widgets, head, body, and optional foot.
/// if only 1 widget is added, then it's assumed to be the body
/// if 2 widgets are added, then they are head, body, respectively.
/// if 3 widgets are added, then they are head, body, foot, respectively. 
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this card belongs to.
/// is_open : bool, default True
///     Whether the card is open (expanded).
/// close_icon : bool, Optional
///     Whether to have a close icon.
/// close_icon_size : float, Optional
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
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created card.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id, 
    parent_id=None,     
    is_open=true,
    close_icon=None,
    close_icon_size=None,
    on_close=None,
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
    show=true, 
    user_data=None,
    gen_id=None,
    ))]
pub fn add_card(
    window_id: String,
    container_id: String,
    parent_id: Option<String>, 
    is_open: bool,
    close_icon: Option<bool>,
    close_icon_size: Option<f32>,
    on_close: Option<PyObject>,
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
    show: bool,
    user_data: Option<PyObject>, 
    gen_id: Option<usize>,
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    if let Some(py) = on_close {
        add_callback_to_mutex(id, "on_close".to_string(), py);
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
            close_icon_size,
            style_id,
            style_std,
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
/// body_background_color : Color, Optional
///     Sets the body background color using a predefined color variant.
/// body_background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// body_background_rgba : list of float, Optional
///     Sets the body background color in rgba format as [r, g, b, a].
/// foot_background_color : Color, Optional
///     Sets the footer background color using a predefined color variant.
/// foot_background_color_alpha : float, Optional
///     Sets the alpha of the Color.
/// foot_background_rgba : list of float, Optional
///     Sets the footer background color in rgba format as [r, g, b, a].
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
    body_background_color=None,
    body_background_color_alpha=None,
    body_background_rgba=None, 
    foot_background_color=None,
    foot_background_color_alpha=None,
    foot_background_rgba=None,
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
    body_background_color: Option<Color>,
    body_background_color_alpha: Option<f32>,
    body_background_rgba: Option<[f32; 4]>, 
    foot_background_color: Option<Color>,
    foot_background_color_alpha: Option<f32>,
    foot_background_rgba: Option<[f32; 4]>, 
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::CardStyle(
        CardStyle {
            id,
            background_color,
            background_color_alpha,
            background_rgba,
            border_radius, 
            border_width,
            border_color,
            border_color_alpha,
            border_rgba, 
            head_background_color,
            head_background_color_alpha,
            head_background_rgba, 
            body_background_color,
            body_background_color_alpha,
            body_background_rgba, 
            foot_background_color,
            foot_background_color_alpha,
            foot_background_rgba,
        }));


    drop(state);
    Ok(id)
}
