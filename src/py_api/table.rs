//! Table module - provides add_table pyfunction or Table class

use pyo3::{Py, PyAny, PyResult, pyfunction};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_name_to_mutex, add_user_data_to_mutex, graphics::colors::Color, state::{Containers, Widgets, get_id, set_state_cont_wnd_ids, set_state_of_container, set_state_of_widget}, widgets::{callbacks::CallbackName, ipg_table::{Table, TableBasic, TableBody, TableFooter, TableHeader, TableStyle}}};


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
/// body : list of list of any
///     Sets the table body data as rows of any values.
/// body_row_heights : float
///     Sets the height of the body rows.
/// column_widths : list of float
///     Sets the initial width of each column in logical pixels.
/// body_row_highlight : bool, default False
///     Whether to highlight alternating body rows.
/// body_row_outline : bool, default True
///     Whether to show an outline on body rows.
/// file_path : str, Optional
///     Sets the path to load table data from a CSV file.
/// footer : list of str, Optional
///     Sets the column footer labels.
/// footer_height : float, Optional
///     Sets the height of the footer row.
/// header : list of str, Optional
///     Sets the column header labels.
/// header_height : float, Optional
///     Sets the height of the header row.
/// min_size : float, default 0.0
///     Sets the minimum column width in logical pixels.
/// resize_columns_enabled : bool, default True
///     Whether column resizing is enabled.
/// sash_size : float, default 4.0
///     Sets the width of the column sash (drag handle).
/// sash_style_id : int, Optional
///     Sets the ID of a custom style for the sash.
/// scrollable_height : float, Optional
///     Sets the height of the scrollable area.
/// text_size : float, Optional
///     Sets the font size for table text.
/// style_id : int, Optional
///     Sets the ID of a custom style.
/// parent_id : str, Optional
///     Sets the parent container ID. Defaults to the window itself.
/// show : bool, default True
///     Whether the table is visible.
/// on_column_resize : callable, Optional
///     Sets the callback method to invoke when a column is being resized.
/// on_column_resize_release : callable, Optional
///     Sets the callback method to invoke when a column resize is released.
/// gen_id : int, Optional
///     Obtains an ID of a widget that has not been created; used for the gen_id parameter.
/// user_data : Any, Optional
///     Sets arbitrary data forwarded to callbacks.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created table.
#[pyfunction]
#[pyo3(signature = (
    parent_id,
    body,
    body_row_heights,
    column_widths,
    // above required
    body_row_highlight=false,
    body_row_outline=true,
    file_path=None,
    footer_height=None,
    footer=None,
    gen_id=None,
    header_height=None,
    header=None,
    min_size=0.0,
    on_column_resize_release=None,
    on_column_resize=None,
    resize_columns_enabled=true,
    resize_table_enabled=false,
    sash_size=4.0,
    sash_style_id=None,  
    scrollable_height=None,
    show=true,
    style_id=None,
    text_size=None,
    user_data=None,
    ))]
pub fn add_table_basic(
    parent_id: String,
    body: Vec<Vec<String>>,
    body_row_heights: f32,
    column_widths: Vec<f32>,
    // above required
    body_row_highlight: bool,
    body_row_outline: bool,
    file_path: Option<String>,
    footer_height: Option<f32>,
    footer: Option<Vec<String>>,
    gen_id: Option<usize>,
    header_height: Option<f32>,
    header: Option<Vec<String>>,
    min_size: f32,
    on_column_resize_release: Option<PyObject>,
    on_column_resize: Option<PyObject>,
    resize_columns_enabled: bool,
    resize_table_enabled: bool,
    sash_size: f32,
    sash_style_id: Option<usize>,
    scrollable_height: Option<f32>,
    show: bool,
    style_id: Option<usize>,
    text_size: Option<f32>,
    user_data: Option<PyObject>,
    ) -> PyResult<usize> 
{

    let id = get_id(gen_id);

    // Register widget with parent
    set_state_of_widget(id, parent_id.clone());

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    if let Some(py) = on_column_resize {
        add_callback_name_to_mutex(id, CallbackName::OnResize, py);
    }

    let released = if let Some(py) = on_column_resize_release {
        add_callback_name_to_mutex(id, CallbackName::OnReleased, py);
        true
    } else {
        false
    };

    

    let mut state = access_state();
    state.widgets.insert(
        id,
        Widgets::TableBasic(
        TableBasic {
            id,
            body_row_heights,
            body_row_highlight,
            body_row_outline,
            body,
            column_widths,
            file_path,
            footer_height,
            footer,
            header,
            header_height,
            min_size,
            released,
            resize_columns_enabled,
            resize_table_enabled,
            sash_size,
            sash_style_id,
            scrollable_height,
            show,
            style_id,
            text_size,
        }));

    drop(state);
    Ok(id)

}


#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    col_widths,
    row_height,
    header_height=None,
    footer_height=None,
    parent_id=None,
    sash_size=4.0,
    min_size=10.0,
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
        col_widths: Vec<f32>,
        row_height: f32,
        header_height: Option<f32>,
        footer_height: Option<f32>,
        parent_id: Option<String>,
        sash_size: f32,
        min_size: f32,
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
        add_callback_name_to_mutex(id, CallbackName::OnResize, py);
    }

    let _released = if let Some(py) = on_column_resize_release {
        add_callback_name_to_mutex(id, CallbackName::OnReleased, py);
        true
    } else {
        false
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_table".to_string());

    state.containers.insert(id, Containers::Table(
        Table { 
            id, 
            col_widths, 
            row_height, 
            header_height, 
            footer_height, 
            sash_size, 
            min_size, 
            file_path, 
            style_id, 
            sash_style_id, 
            show }));

    drop(state);
    Ok(id)

}

#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    parent_id=None,
    height=None,
    style_id=None,
    sash_style_id=None,
    on_column_resize=None,
    on_column_resize_release=None,
    user_data=None,
    show=true,
    ))]
pub fn add_table_header(
        window_id: String,
        container_id: String,
        parent_id: Option<String>,
        height: Option<f32>,
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
        add_callback_name_to_mutex(id, CallbackName::OnResize, py);
    }

    let _released = if let Some(py) = on_column_resize_release {
        add_callback_name_to_mutex(id, CallbackName::OnReleased, py);
        true
    } else {
        false
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_table".to_string());

    state.containers.insert(id, Containers::TableHeader(
        TableHeader{
            id,
            height,
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
    window_id,
    container_id,
    parent_id=None,
    row_height=None,
    style_id=None,
    sash_style_id=None,
    on_column_resize=None,
    on_column_resize_release=None,
    user_data=None,
    show=true,
    ))]
pub fn add_table_body(
        window_id: String,
        container_id: String,
        parent_id: Option<String>,
        row_height: Option<f32>,
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
        add_callback_name_to_mutex(id, CallbackName::OnResize, py);
    }

    let _released = if let Some(py) = on_column_resize_release {
        add_callback_name_to_mutex(id, CallbackName::OnReleased, py);
        true
    } else {
        false
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_table".to_string());

    state.containers.insert(id, Containers::TableBody(
        TableBody {
            id,
            row_height,
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
    window_id,
    container_id,
    parent_id=None,
    style_id=None,
    sash_style_id=None,
    on_column_resize=None,
    on_column_resize_release=None,
    user_data=None,
    show=true,
    ))]
pub fn add_table_footer(
        window_id: String,
        container_id: String,
        parent_id: Option<String>,
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
        add_callback_name_to_mutex(id, CallbackName::OnResize, py);
    }

    let _released = if let Some(py) = on_column_resize_release {
        add_callback_name_to_mutex(id, CallbackName::OnReleased, py);
        true
    } else {
        false
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_table".to_string());

    state.containers.insert(id, Containers::TableFooter(
        TableFooter {
            id,
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
