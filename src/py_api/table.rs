


use pyo3::{Py, PyAny, PyResult, pyfunction};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, graphics::colors::Color, state::{Containers, Widgets, get_id, set_state_cont_wnd_ids, set_state_of_container}, widgets::ipg_table::{TableBasic, TableStyle}};


/// Add a table basic widget.
///
/// A basic table with headers, body rows, footers, and resizable columns.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this table belongs to.
/// container_id : str
///     Sets the Unique string identifier for the table.
/// headers : list of str
///     Sets the column header labels.
/// body : list of list of float
///     Sets the table body data as rows of float values.
/// footers : list of str
///     Sets the column footer labels.
/// column_widths : list of float
///     Sets the initial width of each column in logical pixels.
/// height : float
///     Sets the height of the table in logical pixels.
/// parent_id : str, Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// width : float, Optional
///     Sets the Fixed width in logical pixels.
/// sash_size : float, default 4.0
///     Sets the width of the column sash (drag handle).
/// header_enabled : bool, default True
///     Whether the header row is displayed.
/// header_row_height : float, Optional
///     Sets the height of the header row.
/// header_scrollbar_height : float, Optional
///     Sets the height of the header scrollbar.
/// header_scrollbar_margin : float, Optional
///     Sets the margin around the header scrollbar.
/// header_scroller_height : float, Optional
///     Sets the height of the header scroller thumb.
/// header_scrollbar_spacing : float, Optional
///     Sets the spacing around the header scrollbar.
/// header_row_spacing : float, Optional
///     Sets the spacing between header rows.
/// footer_height : float, Optional
///     Sets the height of the footer row.
/// footer_scrollbar_height : float, Optional
///     Sets the height of the footer scrollbar.
/// footer_scrollbar_margin : float, Optional
///     Sets the margin around the footer scrollbar.
/// footer_scroller_height : float, Optional
///     Sets the height of the footer scroller thumb.
/// footer_scrollbar_spacing : float, Optional
///     Sets the spacing around the footer scrollbar.
/// footer_spacing : float, Optional
///     Sets the spacing between footer rows.
/// body_scrollbar_width : float, Optional
///     Sets the width of the body scrollbar.
/// body_scrollbar_margin : float, Optional
///     Sets the margin around the body scrollbar.
/// body_scroller_width : float, Optional
///     Sets the width of the body scroller thumb.
/// body_scrollbar_spacing : float, Optional
///     Sets the spacing around the body scrollbar.
/// body_row_highlight : bool, default True
///     Whether to highlight body rows on hover.
/// custom_header_rows : int, Optional
///     Sets the number of custom header rows.
/// custom_footer_rows : int, Optional
///     Sets the number of custom footer rows.
/// control_columns : list of int, default []
///     Sets the indices of columns that contain control widgets.
/// min_size : float, default 0.0
///     Sets the minimum column width in logical pixels.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// style_id : int, Optional
///     Sets the ID of a custom style.
/// scrollable_style_id : int, Optional
///     Sets the ID of a scrollable style for the table.
/// show : bool, default True
///     Whether the table is visible.
/// on_column_resize : callable, Optional
///     Sets the Callback method to invoke when a column is being resized.
/// on_column_resize_release : callable, Optional
///     Sets the Callback method to invoke when a column resize is released.
/// user_data : Any, Optional
///     Sets the Arbitrary data forwarded to callbacks.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created table.
#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id,
    row_height,
    col_widths,
    scrollable_height=None,
    parent_id=None,
    file_path=None,
    headers=None,
    body=None,
    footers=None,
    column_widths=None,
    sash_size=4.0,
    header_enabled=true,
    header_row_height=None,
    header_row_spacing=None,
    footer_height=None,
    row_spacing=None,
    resize_columns_enabled=true,
    min_size=0.0,
    text_size=None,
    gen_id=None,
    style_id=None,
    sash_style_id=None,  
    on_column_resize=None,
    on_column_resize_release=None,
    user_data=None,
    show=true,
    ))]
pub fn add_table_basic(
        window_id: String,
        container_id: String,
        row_height: f32,
        col_widths: Vec<f32>,
        scrollable_height: Option<f32>,
        parent_id: Option<String>,
        file_path: Option<String>,
        headers: Option<Vec<String>>,
        body: Option<Vec<Vec<String>>>,
        footers: Option<Vec<String>>,
        column_widths: Option<Vec<f32>>,
        sash_size: Option<f32>,
        header_enabled: Option<bool>,
        header_row_height: Option<f32>,
        header_row_spacing: Option<f32>,
        footer_height: Option<f32>,
        row_spacing: Option<f32>,
        resize_columns_enabled: bool,
        min_size: f32,
        text_size: Option<f32>,
        gen_id: Option<usize>,
        style_id: Option<usize>,
        sash_style_id: Option<usize>,
        on_column_resize: Option<PyObject>,
        on_column_resize_release: Option<PyObject>,
        user_data: Option<PyObject>,
        show: bool,
    ) -> PyResult<usize> 
{

    let id = get_id(gen_id);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    if let Some(py) = on_column_resize {
        add_callback_to_mutex(id, "on_resize".to_string(), py);
    }

    let released = if let Some(py) = on_column_resize_release {
        add_callback_to_mutex(id, "released".to_string(), py);
        true
    } else {
        false
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_table".to_string());

    state.containers.insert(id, Containers::TableBasic(
        TableBasic {
            id,
            row_height,
            col_widths,
            scrollable_height,
            file_path,
            headers,
            body,
            footers,
            column_widths,
            sash_size,
            header_enabled,
            header_row_height,
            header_row_spacing,
            footer_height,
            row_spacing,
            resize_columns_enabled,
            min_size,
            text_size,
            style_id,
            sash_style_id,
            released,
            show,
            ..Default::default()
        }));

    drop(state);
    Ok(id)

}


#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    parent_id=None,
    file_path=None,
    style_id=None,
    sash_style_id=None,
    on_column_resize=None,
    on_column_resize_release=None,
    user_data=None,
    show=true,
    ))]
pub fn add_table(
        window_id: String,
        container_id: String,
        parent_id: Option<String>,
        file_path: Option<String>,
        style_id: Option<usize>,
        sash_style_id: Option<usize>,
        on_column_resize: Option<PyObject>,
        on_column_resize_release: Option<PyObject>,
        user_data: Option<PyObject>,
        show: bool,
    ) -> PyResult<usize> 
{

    let id = get_id(None);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    if let Some(py) = on_column_resize {
        add_callback_to_mutex(id, "on_resize".to_string(), py);
    }

    let _released = if let Some(py) = on_column_resize_release {
        add_callback_to_mutex(id, "released".to_string(), py);
        true
    } else {
        false
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_table".to_string());

    state.containers.insert(id, Containers::TableBasic(
        TableBasic {
            id,
            file_path,
            style_id,
            sash_style_id,
            show,
            ..Default::default()
        }));

    drop(state);
    Ok(id)

}





#[pyfunction]
#[pyo3(signature = (
    bkg_color=None,
    bkg_color_alpha=None, 
    bkg_rgba=None,
    border_color=None,
    border_color_alpha=None, 
    border_rgba=None,
    border_radius=None, 
    border_width=None,
    text_color=None,
    text_color_alpha=None, 
    text_rgba=None,
    gen_id=None
    ))]
pub fn add_table_style(
    bkg_color: Option<Color>,
    bkg_color_alpha: Option<f32>,
    bkg_rgba: Option<[f32; 4]>,
    border_color: Option<Color>,
    border_color_alpha: Option<f32>,
    border_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    border_width: Option<f32>,
    text_color: Option<Color>,
    text_color_alpha: Option<f32>,
    text_rgba: Option<[f32; 4]>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::TableStyle(
        TableStyle {
            id,
            bkg_color,
            bkg_color_alpha, 
            bkg_rgba,
            border_color,
            border_color_alpha, 
            border_rgba,
            border_radius, 
            border_width,
            text_color,
            text_color_alpha, 
            text_rgba,
        }));

    drop(state);
    Ok(id)
}
