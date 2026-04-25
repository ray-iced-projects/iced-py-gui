//! Menu module - provides add_menu pyfunction

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};

use crate::graphics::colors::Color;
use crate::widgets::ipg_menu::{Menu, MenuBarItem, MenuStyle};
use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::{Containers, Widgets, get_id, set_state_cont_wnd_ids, set_state_of_container};
type PyObject = Py<PyAny>;


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
    parent_id=None,
    padding=None,
    spacing=None,
    width=None,
    width_fill=None,
    height=None,
    close_on_bar_item_click=None,
    close_on_bar_background_click=None,
    items_close_on_click_global=None,
    items_close_on_background_click_global=None,
    cursor_bounds_margin=None,
    scroll_speed_line=None,
    scroll_speed_pixel=None,
    on_select=None,
    style_id=None,
    style_primary=None,
    show=true, 
    user_data=None, 
    gen_id=None
    ))]
pub fn add_menu(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    padding: Option<Vec<f32>>,
    spacing: Option<f32>,
    width: Option<f32>,
    width_fill: Option<bool>,
    height: Option<f32>,
    close_on_bar_item_click: Option<bool>,
    close_on_bar_background_click: Option<bool>,
    items_close_on_click_global: Option<bool>,
    items_close_on_background_click_global: Option<bool>,
    cursor_bounds_margin: Option<f32>,
    scroll_speed_line: Option<f32>,
    scroll_speed_pixel: Option<f32>,
    on_select: Option<PyObject>,
    style_id: Option<usize>,
    style_primary: Option<bool>,
    show: bool,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    if let Some(py) = on_select {
        add_callback_to_mutex(id, "on_select".to_string(), py);
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

    state.containers.insert(id, Containers::Menu(
        Menu {
            id,
            padding,
            spacing,
            width,
            width_fill,
            height,
            close_on_bar_item_click,
            close_on_bar_background_click,
            items_close_on_click_global,
            items_close_on_background_click_global,
            style_id,
            style_primary,
            cursor_bounds_margin,
            scroll_speed_line,
            scroll_speed_pixel,
            show,
        }));

    drop(state);
    Ok(id)
}


/// Add styling to a menu.
///
/// Creates a custom style that can be applied to a menu via its
/// ``style_id`` parameter.  The style has three sections: **bar**
/// (the horizontal menu bar), **menu** (the dropdown panels), and
/// **path** (the highlighted trail from bar item to open menu).
///
/// Parameters
/// ----------
/// bar_background_color : Color, Optional
///     Sets the bar background color.
/// bar_background_rgba : list[float, 4], Optional
///     Sets the bar background color in rgba format.
/// bar_background_alpha : float, Optional
///     Sets the alpha transparency for the bar background color.
/// bar_border_color : Color, Optional
///     Sets the bar border color.
/// bar_border_rgba : list[float, 4], Optional
///     Sets the bar border color in rgba format.
/// bar_border_alpha : float, Optional
///     Sets the alpha transparency for the bar border color.
/// bar_border_radius : list of float, Optional
///     Sets the bar border radius, ``[float]`` = all corners,
///     ``[float, 4]`` = [top-left, top-right, bottom-right,
///     bottom-left].
/// bar_border_width : float, Optional
///     Sets the bar border width.
/// bar_shadow_color : Color, Optional
///     Sets the bar shadow color.
/// bar_shadow_rgba : list[float, 4], Optional
///     Sets the bar shadow color in rgba format.
/// bar_shadow_alpha : float, Optional
///     Sets the alpha transparency for the bar shadow color.
/// bar_shadow_offset_xy : list[float, 2], Optional
///     Sets the bar shadow offset as [x, y].
/// bar_shadow_blur_radius : float, Optional
///     Sets the bar shadow blur radius.
/// menu_background_color : Color, Optional
///     Sets the dropdown menu background color.
/// menu_background_rgba : list[float, 4], Optional
///     Sets the dropdown menu background color in rgba format.
/// menu_background_alpha : float, Optional
///     Sets the alpha transparency for the dropdown menu
///     background color.
/// menu_border_color : Color, Optional
///     Sets the dropdown menu border color.
/// menu_border_rgba : list[float, 4], Optional
///     Sets the dropdown menu border color in rgba format.
/// menu_border_alpha : float, Optional
///     Sets the alpha transparency for the dropdown menu border
///     color.
/// menu_border_radius : list of float, Optional
///     Sets the dropdown menu border radius, ``[float]`` = all
///     corners, ``[float, 4]`` = [top-left, top-right,
///     bottom-right, bottom-left].
/// menu_border_width : float, Optional
///     Sets the dropdown menu border width.
/// menu_shadow_color : Color, Optional
///     Sets the dropdown menu shadow color.
/// menu_shadow_rgba : list[float, 4], Optional
///     Sets the dropdown menu shadow color in rgba format.
/// menu_shadow_alpha : float, Optional
///     Sets the alpha transparency for the dropdown menu shadow
///     color.
/// menu_shadow_offset_xy : list[float, 2], Optional
///     Sets the dropdown menu shadow offset as [x, y].
/// menu_shadow_blur_radius : float, Optional
///     Sets the dropdown menu shadow blur radius.
/// path_background_color : Color, Optional
///     Sets the path highlight background color.
/// path_background_rgba : list[float, 4], Optional
///     Sets the path highlight background color in rgba format.
/// path_background_alpha : float, Optional
///     Sets the alpha transparency for the path background color.
/// path_border_color : Color, Optional
///     Sets the path highlight border color.
/// path_border_rgba : list[float, 4], Optional
///     Sets the path highlight border color in rgba format.
/// path_border_alpha : float, Optional
///     Sets the alpha transparency for the path border color.
/// path_border_radius : list of float, Optional
///     Sets the path highlight border radius, ``[float]`` = all
///     corners, ``[float, 4]`` = [top-left, top-right,
///     bottom-right, bottom-left].
/// path_border_width : float, Optional
///     Sets the path highlight border width.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used
///     for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass as ``style_id`` to
///     ``add_menu``.
#[pyfunction]
#[pyo3(signature = (
    bar_background_color=None,
    bar_background_color_alpha=None,
    bar_background_rgba=None,
    
    bar_border_color=None,
    bar_border_color_alpha=None,
    bar_border_rgba=None,
    bar_border_radius=None,
    bar_border_width=None,

    bar_shadow_color=None,
    bar_shadow_color_alpha=None,
    bar_shadow_rgba=None,
    bar_shadow_offset_xy=None,
    bar_shadow_blur_radius=None,

    menu_background_color=None,
    menu_background_color_alpha=None,
    menu_background_rgba=None,
    
    menu_border_color=None,
    menu_border_color_alpha=None,
    menu_border_rgba=None,
    menu_border_radius=None,
    menu_border_width=None,

    menu_shadow_color=None,
    menu_shadow_color_alpha=None,
    menu_shadow_rgba=None,
    menu_shadow_offset_xy=None,
    menu_shadow_blur_radius=None,

    path_background_color=None,
    path_background_color_alpha=None,
    path_background_rgba=None,
    
    path_border_color=None,
    path_border_color_alpha=None,
    path_border_rgba=None,
    path_border_radius=None,
    path_border_width=None,

    gen_id=None))]
pub fn add_menu_style(
    bar_background_color: Option<Color>,
    bar_background_color_alpha: Option<f32>,
    bar_background_rgba: Option<[f32; 4]>,
    
    bar_border_color: Option<Color>,
    bar_border_color_alpha: Option<f32>,
    bar_border_rgba: Option<[f32; 4]>,
    bar_border_radius: Option<Vec<f32>>,
    bar_border_width: Option<f32>,
    
    bar_shadow_color: Option<Color>,
    bar_shadow_color_alpha: Option<f32>,
    bar_shadow_rgba: Option<[f32; 4]>,
    bar_shadow_offset_xy: Option<[f32; 2]>,
    bar_shadow_blur_radius: Option<f32>,

    menu_background_color: Option<Color>,
    menu_background_color_alpha: Option<f32>,
    menu_background_rgba: Option<[f32; 4]>,
    
    menu_border_color: Option<Color>,
    menu_border_color_alpha: Option<f32>,
    menu_border_rgba: Option<[f32; 4]>,
    menu_border_radius: Option<Vec<f32>>,
    menu_border_width: Option<f32>,
    
    menu_shadow_color: Option<Color>,
    menu_shadow_color_alpha: Option<f32>,
    menu_shadow_rgba: Option<[f32; 4]>,
    menu_shadow_offset_xy: Option<[f32; 2]>,
    menu_shadow_blur_radius: Option<f32>,

    path_background_color: Option<Color>,
    path_background_color_alpha: Option<f32>,
    path_background_rgba: Option<[f32; 4]>,
    
    path_border_color: Option<Color>,
    path_border_color_alpha: Option<f32>,
    path_border_rgba: Option<[f32; 4]>,
    path_border_radius: Option<Vec<f32>>,
    path_border_width: Option<f32>,

    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    

    let mut state = access_state();

    state.widgets.insert(id, Widgets::MenuStyle(
        MenuStyle {
            id,
            bar_background_color,
            bar_background_color_alpha,
            bar_background_rgba,
            
            bar_border_color,
            bar_border_color_alpha,
            bar_border_rgba,
            bar_border_radius,
            bar_border_width,

            bar_shadow_color,
            bar_shadow_color_alpha,
            bar_shadow_rgba,
            bar_shadow_offset_xy,
            bar_shadow_blur_radius,

            menu_background_color,
            menu_background_color_alpha,
            menu_background_rgba,
            
            menu_border_color,
            menu_border_color_alpha,
            menu_border_rgba,
            menu_border_radius,
            menu_border_width,

            menu_shadow_color,
            menu_shadow_color_alpha,
            menu_shadow_rgba,
            menu_shadow_offset_xy,
            menu_shadow_blur_radius,

            path_background_color,
            path_background_color_alpha,
            path_background_rgba,
            
            path_border_color,
            path_border_color_alpha,
            path_border_rgba,
            path_border_radius,
            path_border_width,
        }));

    drop(state);
    Ok(id)
}


/// Add a menu bar item container.
///
/// Groups a bar-level widget with its dropdown items.  The first
/// child added inside the ``MenuBarItem`` is rendered on the menu
/// bar; all subsequent children become dropdown items.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this menu bar item belongs to.
/// container_id : str
///     Sets the unique string identifier for the menu bar item.
/// parent_id : str, Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// width : float, Optional
///     Sets the width of this dropdown menu in logical pixels.
/// spacing : float, Optional
///     Sets the vertical spacing between dropdown items.
/// offset : float, Optional
///     Sets the horizontal offset of the dropdown relative to its
///     bar item.
/// paddings : list of float, Optional
///     Sets the padding inside this dropdown as ``[all]``,
///     ``[vertical, horizontal]``, or
///     ``[top, right, bottom, left]``.
/// close_on_item_click : bool, Optional
///     Per-dropdown override for closing when an item is clicked.
///     Overrides the global default set on the Menu.
/// close_on_background_click : bool, Optional
///     Per-dropdown override for closing when the background is
///     clicked.  Overrides the global default set on the Menu.
/// show : bool, default True
///     Whether the menu bar item is visible.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used
///     for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created menu bar item.
#[pyfunction]
#[pyo3(signature = ( 
    window_id,
    container_id,
    parent_id=None,
    width=None,
    spacing=None,
    offset=None,
    padding=None,
    close_on_item_click=None,
    close_on_background_click=None,
    show=true,
    gen_id=None
    ))]
pub fn add_menu_bar_item(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    width: Option<f32>,
    spacing: Option<f32>,
    offset: Option<f32>,
    padding: Option<Vec<f32>>,
    close_on_item_click: Option<bool>,
    close_on_background_click: Option<bool>,
    show: bool,
    gen_id: Option<usize>,
) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };
    
    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_menu_bar_item".to_string());

    state.containers.insert(id, Containers::MenuBarItem(
        MenuBarItem {
            id,
            width,
            spacing,
            offset,
            padding,
            close_on_item_click,
            close_on_background_click,
            show,
        }));

    drop(state);
    Ok(id)
}
