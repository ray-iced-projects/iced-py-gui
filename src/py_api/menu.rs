//! Menu module - provides add_menu pyfunction

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::widgets::ipg_container::ContainerStyleStd;
use crate::widgets::ipg_menu::get_alignment;
use crate::widgets::ipg_menu::{Menu, MenuBar, MenuItem, SubMenu, get_position};
use crate::state::{Containers, get_id, set_state_cont_wnd_ids, set_state_of_container};


/// Add a menu widget.
///
/// Creates a root menu container that holds menu bars. Use ``add_menu_bar``
/// within the menu's context to add top-level menu items (e.g., "File", "Edit").
///
/// Usage pattern:
/// ```python
/// with Menu():
///     with MenuBar(label="File", ...):
///         # Add menu items and widgets here
/// ```
///
/// Parameters
/// ----------
/// window_id : str
///     The window this menu belongs to.
/// container_id : str
///     Unique identifier for this menu.
/// parent_id : str, Optional
///     Parent container ID. Defaults to the window itself.
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
    parent_id,
    bar_item_spacing=None,
    padding=None,
    ))]
pub fn add_menu(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    bar_item_spacing: Option<f32>,
    padding: Option<Vec<f32>>,
) -> PyResult<usize> 
{
    let id = get_id(None);

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
            bar_item_spacing,
            padding,
            is_open: vec![],
            open_submenus: vec![],
            pending_submenu: None,
            pending_timer: None,
            pending_close: None,
            close_timer: None,
        }));

    drop(state);
    Ok(id)
}


/// Add a menu bar (top-level menu item).
///
/// Creates a top-level menu bar item with a label (e.g., "File", "Edit").
/// The dropdown panel opens when the item is hovered. Add menu items and
/// widgets within this menu bar's context.
///
/// Usage pattern:
/// ```python
/// with Menu():
///     with MenuBar(label="File", width=60, dropdown_width=150):
///         with MenuItem(...):
///             with MenuButton(on_press=item_pressed):
///                 ipg.add_text(...)
/// ```
///
/// Parameters
/// ----------
/// window_id : str
///     The window this menu bar belongs to.
/// container_id : str
///     Unique identifier for this menu bar.
/// label : str
///     Text displayed on the menu bar (e.g., "File", "Edit").
/// width : float
///     Width of the menu bar item in logical pixels.
/// dropdown_width : float
///     Width of the dropdown panel in logical pixels.
/// str_id : str, Optional
///     String ID for debugging/tracking.
/// label_size : float, Optional
///     Font size of the label text.
/// parent_id : str, Optional
///     Parent container ID. Defaults to the menu.
/// spacing : float, Optional
///     Vertical spacing between dropdown items.
/// gap : float, Optional
///     Gap between items.
/// padding : list of float, Optional
///     Padding inside the dropdown panel.
/// dropdown_open_auto : bool, Optional
///     Whether to auto-position the dropdown.
/// dropdown_open_top : bool, Optional
///     Open dropdown above the bar item.
/// dropdown_open_bottom : bool, Optional
///     Open dropdown below the bar item.
/// dropdown_open_left : bool, Optional
///     Align dropdown to the left.
/// dropdown_open_right : bool, Optional
///     Align dropdown to the right.
/// container_style_id : int, Optional
///     Custom container style ID.
/// container_style_std : ContainerStyleStd, Optional
///     Standard container styling.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created menu bar.
///
#[pyfunction]
#[pyo3(signature = ( 
    window_id,
    container_id,
    label,
    width,
    dropdown_width,
    str_id=None,
    label_size=None,
    parent_id=None,
    spacing=None,
    gap=None,
    padding=None,
    dropdown_open_auto=None,
    dropdown_open_top=None,
    dropdown_open_bottom=None,
    dropdown_open_left=None,
    dropdown_open_right=None,
    dropdown_align_start=None,
    dropdown_align_center=None,
    dropdown_align_end=None,
    container_style_id=None,
    container_style_std=None,
    snap_within_viewport=None,
    ))]
pub fn add_menu_bar(
    window_id: String,
    container_id: String,
    label: String,
    width: f32,
    dropdown_width: f32,
    str_id: Option<String>,
    label_size: Option<f32>,
    parent_id: Option<String>,
    spacing: Option<f32>,
    gap: Option<f32>,
    padding: Option<Vec<f32>>,
    dropdown_open_auto: Option<bool>,
    dropdown_open_top: Option<bool>,
    dropdown_open_bottom: Option<bool>,
    dropdown_open_left: Option<bool>,
    dropdown_open_right: Option<bool>,
    dropdown_align_start: Option<bool>,
    dropdown_align_center: Option<bool>,
    dropdown_align_end: Option<bool>,
    container_style_id: Option<usize>,
    container_style_std: Option<ContainerStyleStd>,
    snap_within_viewport: Option<bool>,
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

    let position = get_position(positions, true);

    let alignments = [
        dropdown_align_start,
        dropdown_align_center,
        dropdown_align_end,
    ];

    let alignment = get_alignment(alignments);
    
    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_menu_bar".to_string());

    state.containers.insert(id, Containers::MenuBar(
        MenuBar {
            id,
            label,
            label_size,
            width,
            dropdown_width,
            str_id,
            position,
            alignment,
            spacing,
            gap,
            padding,
            container_style_id,
            container_style_std,
            snap_within_viewport,
        }));

    drop(state);
    Ok(id)
}


/// Add a nested submenu.
///
/// Creates a submenu that opens within a parent menu item (on hover).
/// Add menu items and widgets within this submenu's context to create
/// nested dropdown levels.
///
/// Usage pattern:
/// ```python
/// with Menu():
///     with MenuBar(label="File", width=50.0):
///         with MenuItem():
///             # The MenuItem before a SubMenu DOES NOT use MenuButtob
///             add_text(content="Open Recent")
///             # SubMenu must be nested within the MenuIten
///             # The MenuItem when pressed will open the SubMenu
///             with SubMenu(width=200):
///                 with MenuItem():
///                     add_text(content="~/document/iced_menu")
/// ```
///
/// Parameters
/// ----------
/// window_id : str
///     The window this submenu belongs to.
/// container_id : str
///     Unique identifier for this submenu.
/// width : float
///     Width of the submenu panel in logical pixels.
/// parent_id : str, Optional
///     Parent container ID. Defaults to the window.
/// str_id : str, Optional
///     String ID for debugging/tracking.
/// spacing : float, Optional
///     Vertical spacing between submenu items.
/// gap : float, Optional
///     Gap between items.
/// padding : list of float, Optional
///     Padding inside the submenu panel.
/// dropdown_open_auto : bool, Optional
///     Whether to auto-position the submenu.
/// dropdown_open_top : bool, Optional
///     Open submenu above the trigger.
/// dropdown_open_bottom : bool, Optional
///     Open submenu below the trigger.
/// dropdown_open_left : bool, Optional
///     Align submenu to the left.
/// dropdown_open_right : bool, Optional
///     Align submenu to the right.
/// container_style_id : int, Optional
///     Custom container style ID.
/// container_style_std : ContainerStyleStd, Optional
///     Standard container styling.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created submenu.
///
#[pyfunction]
#[pyo3(signature = ( 
    window_id,
    container_id,
    width,
    parent_id=None,
    str_id=None,
    spacing=None,
    gap=None,
    padding=None,
    dropdown_open_auto=None,
    dropdown_open_top=None,
    dropdown_open_bottom=None,
    dropdown_open_left=None,
    dropdown_open_right=None,
    dropdown_align_start=None,
    dropdown_align_center=None,
    dropdown_align_end=None,
    container_style_id=None,
    container_style_std=None,
    snap_within_viewport=None,
    ))]
pub fn add_sub_menu(
    window_id: String,
    container_id: String,
    width: f32,
    parent_id: Option<String>,
    str_id: Option<String>,
    spacing: Option<f32>,
    gap: Option<f32>,
    padding: Option<Vec<f32>>,
    dropdown_open_auto: Option<bool>,
    dropdown_open_top: Option<bool>,
    dropdown_open_bottom: Option<bool>,
    dropdown_open_left: Option<bool>,
    dropdown_open_right: Option<bool>,
    dropdown_align_start: Option<bool>,
    dropdown_align_center: Option<bool>,
    dropdown_align_end: Option<bool>,
    container_style_id: Option<usize>,
    container_style_std: Option<ContainerStyleStd>,
    snap_within_viewport: Option<bool>,
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

    let position = get_position(positions, false);
    
    let alignments = [
        dropdown_align_start,
        dropdown_align_center,
        dropdown_align_end,
    ];

    let alignment = get_alignment(alignments);

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_menu_bar".to_string());

    state.containers.insert(id, Containers::SubMenu(
        SubMenu {
            id,
            width,
            position,
            alignment,
            str_id,
            spacing,
            gap,
            padding,
            container_style_id,
            container_style_std,
            snap_within_viewport,
        }));

    drop(state);
    Ok(id)
}

/// Add a menu item container.
///
/// Creates a container for content within a menu bar or submenu. Add widgets
/// (text, buttons, etc.) within this item's context to create individual
/// menu entries. Optionally add a submenu for nested dropdowns.
///
/// Usage pattern:
/// ```python
/// with Menu():
///     with MenuBar(label="File", width=50.0):
///         with MenuItem():
///             # A MenuItem must use the special MenuButton, 
///             # which is a container that accepts widgets.
///             # except in the case where a SubMenu follows.
///             MenuButton(on_press=on_item_press):
///                 add_text(content="Open Recent")
/// ```
///
/// Parameters
/// ----------
/// window_id : str
///     The window this menu item belongs to.
/// container_id : str
///     Unique identifier for this menu item.
/// parent_id : str, Optional
///     Parent container ID (menu bar or submenu). Defaults to the window.
/// str_id : str, Optional
///     String ID for debugging/tracking.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created menu item.
///
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id,
    parent_id=None,
    str_id=None,
    ))]
pub fn add_menu_item(
    window_id: String,
    container_id: String,
    parent_id: Option<String>,
    str_id: Option<String>,
) -> PyResult<usize>
{
    let id = get_id(None);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_menu_item".to_string());

    state.containers.insert(id, Containers::MenuItem(
        MenuItem {
            id,
            str_id,
        }));

    drop(state);
    Ok(id)
}
