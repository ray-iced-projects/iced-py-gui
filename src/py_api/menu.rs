//! Menu module - provides add_menu pyfunction

use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};
type PyObject = Py<PyAny>;

use crate::widgets::callbacks::CallbackName;
use crate::widgets::ipg_button::ButtonStyleStd;
use crate::widgets::ipg_container::ContainerStyleStd;
use crate::widgets::ipg_menu::{Menu, MenuBarItem, MenuSubItem, get_position};
use crate::{access_state, add_callback_name_to_mutex, add_user_data_to_mutex};
use crate::state::{Containers, get_id, set_state_cont_wnd_ids, set_state_of_container};



/// Add a menu widget.
///
/// A horizontal menu bar with dropdown menus.  Each top-level bar
/// widget and its dropdown items are grouped inside a ``MenuBarItem``
/// context manager.  The first child of each ``MenuBarItem`` is
/// rendered on the bar; the remaining children become dropdown items.
///
/// Per-dropdown settings (width, spacing, offset, padding,
/// close_on_item_click, close_on_background_click) are now set on
/// each ``MenuBarItem`` instead of as vectors here.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this menu belongs to.
/// container_id : str
///     Sets the unique string identifier for the menu.
/// parent_id : str, Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// items_close_on_click_global : bool, Optional
///     Global default for closing dropdowns on item click.  Used
///     when neither the per-Item nor per-dropdown value is set.
///     Defaults to ``False``.
/// items_close_on_background_click_global : bool, Optional
///     Global default for closing dropdowns on background click.
///     Used when neither the per-Item nor per-dropdown value is
///     set.  Defaults to ``False``.
/// height : float, Optional
///     Sets the fixed height of the menu bar in logical pixels.
/// padding : list of float, Optional
///     Sets the padding inside the menu bar as ``[all]``,
///     ``[vertical, horizontal]``, or
///     ``[top, right, bottom, left]``.
/// spacing : float, Optional
///     Sets the horizontal spacing between bar items.
/// width : float, Optional
///     Sets the fixed width of the menu bar in logical pixels.
/// width_fill : bool, Optional
///     Whether to fill the width of a container holding the menu bar.
/// close_on_bar_item_click : bool, Optional
///     Whether the dropdown closes when a menu bar item is clicked.
/// close_on_bar_background_click : bool, Optional
///     Whether the dropdown closes when clicking outside the menu bar.
/// cursor_bounds_margin: float, Optional
///     Sets the margin where, if the cursor moves outside this area,
///     the menu will be closed.
/// scroll_speed_line: float, Optional
///     The speed of the scrolling when items are out of the screen or container.
///     The default is 60 lines which is 1 notch of the mouse wheel.
/// scroll_speed_pixel: float, Optional
///     The scroll_speed_pixels is only for Trackpads and high-precision scroll
///     wheels that report exact pixel deltas.
///     Laptops with trackpads typically produce this. The pixel multiplier
///     (default 1.0) is usually used but you can change if you want.
/// on_select : callable, Optional
///     Sets the callback method to invoke when a menu item is
///     selected.
/// style_id : int, Optional
///     Sets the ID of a custom style created with
///     ``add_menu_style``.
/// style_std_primary : bool, Optional
///     Whether to use the primary standard style.
/// show : bool, default True
///     Whether the menu is visible.
/// user_data : Any, Optional
///     Sets arbitrary data forwarded to callbacks.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used
///     for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created menu.
///
#[pyfunction]
#[pyo3(signature = ( 
    window_id,
    container_id,
    bar_labels,
    bar_widths,
    parent_id=None,
    bar_container_style_id=None,
    bar_container_style_std=None,
    bar_container_palette_id=None,
    bar_labels_text_style_id=None,
    bar_labels_text_font_id=None,
    bar_btn_style_id=None,
    bar_btn_style_std=None,
    bar_btn_palette_id=None,
    bar_btn_font_id=None,
    padding=None,
    spacing=None,
    height=None,
    dropdown_open_auto=None,
    dropdown_open_top=None,
    dropdown_open_bottom=None,
    dropdown_open_left=None,
    dropdown_open_right=None,
    close_on_bar_item_click=None,
    close_on_bar_background_click=None,
    items_close_on_click_global=None,
    items_close_on_background_click_global=None,
    on_bar_item_press=None,
    on_bar_item_enter=None,
    on_bar_item_exit=None,
    show=true, 
    user_data=None, 
    gen_id=None
    ))]
pub fn add_menu(
    window_id: String,
    container_id: String,
    bar_labels: Vec<String>,
    bar_widths: Vec<f32>,
    parent_id: Option<String>,
    bar_container_style_id: Option<usize>,
    bar_container_style_std: Option<ContainerStyleStd>,
    bar_container_palette_id: Option<usize>,
    bar_labels_text_style_id: Option<usize>,
    bar_labels_text_font_id: Option<usize>,
    bar_btn_style_id: Option<usize>,
    bar_btn_style_std: Option<ButtonStyleStd>,
    bar_btn_palette_id: Option<usize>,
    bar_btn_font_id: Option<usize>,
    padding: Option<Vec<f32>>,
    spacing: Option<f32>,
    height: Option<f32>,
    dropdown_open_auto: Option<bool>,
    dropdown_open_top: Option<bool>,
    dropdown_open_bottom: Option<bool>,
    dropdown_open_left: Option<bool>,
    dropdown_open_right: Option<bool>,
    close_on_bar_item_click: Option<bool>,
    close_on_bar_background_click: Option<bool>,
    items_close_on_click_global: Option<bool>,
    items_close_on_background_click_global: Option<bool>,
    on_bar_item_press: Option<PyObject>,
    on_bar_item_enter: Option<PyObject>,
    on_bar_item_exit: Option<PyObject>,
    show: bool,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    if let Some(py) = on_bar_item_press {
        add_callback_name_to_mutex(id, CallbackName::OnBarPress, py);
    }

    if let Some(py) = on_bar_item_enter {
        add_callback_name_to_mutex(id, CallbackName::OnBarEnter, py);
    }

    if let Some(py) = on_bar_item_exit {
        add_callback_name_to_mutex(id, CallbackName::OnBarExit, py);
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

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_menu".to_string());

    let items = bar_labels.len();

    state.containers.insert(id, Containers::Menu(
        Menu {
            id,
            bar_labels,
            bar_widths,
            bar_container_style_id,
            bar_container_style_std,
            bar_container_palette_id,
            bar_labels_text_style_id,
            bar_labels_text_font_id,
            bar_btn_style_id,
            bar_btn_style_std,
            bar_btn_palette_id,
            bar_btn_font_id,
            padding,
            spacing,
            height,
            close_on_bar_item_click,
            close_on_bar_background_click,
            items_close_on_click_global,
            items_close_on_background_click_global,
            show,
            is_open: vec![false; items],
            dropdown_is_open: vec![HashMap::new(); items],
            dropdown_open_auto,
            dropdown_open_top,
            dropdown_open_bottom,
            dropdown_open_left,
            dropdown_open_right,
        }));

    drop(state);
    Ok(id)
}


#[pyfunction]
#[pyo3(signature = ( 
    window_id,
    container_id,
    parent_id=None,
    width=None,
    width_fill=None,
    height=None,
    height_fill=None,
    spacing=None,
    gap=None,
    padding=None,
    dropdown_open_auto=None,
    dropdown_open_top=None,
    dropdown_open_bottom=None,
    dropdown_open_left=None,
    dropdown_open_right=None,
    container_style_id=None,
    container_style_std=None,
    ))]
pub fn add_menu_bar_item(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    height_fill: Option<bool>,
    spacing: Option<f32>,
    gap: Option<f32>,
    padding: Option<Vec<f32>>,
    dropdown_open_auto: Option<bool>,
    dropdown_open_top: Option<bool>,
    dropdown_open_bottom: Option<bool>,
    dropdown_open_left: Option<bool>,
    dropdown_open_right: Option<bool>,
    container_style_id: Option<usize>,
    container_style_std: Option<ContainerStyleStd>,
) -> PyResult<usize> 
{
    let id = get_id(None);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    let positions = 
        [
            dropdown_open_auto,
            dropdown_open_top,
            dropdown_open_bottom,
            dropdown_open_left,
            dropdown_open_right,
        ];

    let position = get_position(positions);
    
    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_menu_bar_item".to_string());

    state.containers.insert(id, Containers::MenuBarItem(
        MenuBarItem {
            id,
            position,
            width,
            width_fill,
            height,
            height_fill,
            spacing,
            gap,
            padding,
            container_style_id,
            container_style_std,
        }));

    drop(state);
    Ok(id)
}


/// Add a menu sub-item container.
///
/// Used inside a ``MenuBarItem`` (or another ``MenuSubItem``) to create
/// a nested sub-menu.  The first child added to ``MenuSubItem`` is the
/// trigger widget shown in the parent dropdown; all subsequent children
/// become the items of the child menu that opens on hover.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this sub-item belongs to.
/// container_id : str
///     Sets the unique string identifier for the sub-item.
/// parent_id : str, Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// width : float, Optional
///     Sets the width of this sub-menu panel in logical pixels.
/// spacing : float, Optional
///     Sets the vertical spacing between sub-menu items.
/// offset : float, Optional
///     Sets the offset of the sub-menu panel relative to its trigger.
/// padding : list of float, Optional
///     Sets the padding inside the sub-menu panel.
/// close_on_item_click : bool, Optional
///     Whether the sub-menu closes when an item is clicked.
/// close_on_background_click : bool, Optional
///     Whether the sub-menu closes when the background is clicked.
/// show : bool, default True
///     Whether the sub-item is visible.
/// gen_id : int, Optional
///     Obtains an ID of a widget that has not been created.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created menu sub-item.
#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    label,
    parent_id=None,
    width=None,
    spacing=None,
    gap=None,
    padding=None,
    container_style_id=None,
    container_style_std=None,
    btn_style_id=None,
    btn_style_std=None,
    btn_palette_id=None,
    btn_font_id=None,
    text_style_id=None,
    text_font_id=None,
    ))]
pub fn add_menu_sub_item(
    window_id: String,
    container_id: String,
    label: String,
    parent_id: Option<String>,
    width: Option<f32>,
    spacing: Option<f32>,
    gap: Option<f32>,
    padding: Option<Vec<f32>>,
    container_style_id: Option<usize>,
    container_style_std: Option<ContainerStyleStd>,
    btn_style_id: Option<usize>,
    btn_style_std: Option<ButtonStyleStd>,
    btn_palette_id: Option<usize>,
    btn_font_id: Option<usize>,
    text_style_id: Option<usize>,
    text_font_id: Option<usize>,
) -> PyResult<usize>
{
    let id = get_id(None);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_menu_sub_item".to_string());

    state.containers.insert(id, Containers::MenuSubItem(
        MenuSubItem {
            id,
            label,
            width,
            spacing,
            gap,
            padding,
            container_style_id,
            container_style_std,
            btn_style_id,
            btn_style_std,
            btn_palette_id,
            btn_font_id,
            text_style_id,
            text_font_id,
        }));

    drop(state);
    Ok(id)
}
