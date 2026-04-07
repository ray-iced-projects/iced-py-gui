


use pyo3::{Py, PyAny, PyResult, pyfunction};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    state::{Containers, get_id, set_state_cont_wnd_ids, set_state_of_container}, 
        widgets::ipg_table::{Table}};


/// Add a table widget.
///
/// A table with headers, body rows, footers, and resizable columns.
///
/// Parameters
/// ----------
/// window_id : str
///     Sets the window this table belongs to.
/// table_id : str
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
/// resizer_width : float, Optional
///     Sets the width of the column resizer handle.
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
/// column_proportional_resize : bool, default True
///     Whether columns resize proportionally.
/// row_spacing : float, Optional
///     Sets the spacing between body rows.
/// row_height : float, Optional
///     Sets the height of each body row.
/// header_body_spacing : float, Optional
///     Sets the spacing between the header and body.
/// body_footer_spacing : float, Optional
///     Sets the spacing between the body and footer.
/// resize_columns_enabled : bool, default True
///     Whether the user can resize columns.
/// min_column_width : float, Optional
///     Sets the minimum column width in logical pixels.
/// text_size : float, Optional
///     Sets the font size for table text.
/// table_width_fixed : bool, default True
///     Whether the table has a fixed width.
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
    table_id, 
    headers,
    body,
    footers,
    column_widths,
    height,
    parent_id=None,
    width=None,
    resizer_width=None,
    header_enabled=true,
    header_row_height=None,
    header_scrollbar_height=None,
    header_scrollbar_margin=None,
    header_scroller_height=None,
    header_scrollbar_spacing=None,
    header_row_spacing=None,
    footer_height=None,
    footer_scrollbar_height=None,
    footer_scrollbar_margin=None,
    footer_scroller_height=None,
    footer_scrollbar_spacing=None,
    footer_spacing=None,
    body_scrollbar_width=None,
    body_scrollbar_margin=None,
    body_scroller_width=None,
    body_scrollbar_spacing=None,
    body_row_highlight=true,
    custom_header_rows=None,
    custom_footer_rows=None,
    control_columns=vec![],
    column_proportional_resize=true,
    row_spacing=None,
    row_height=None,
    header_body_spacing=None,
    body_footer_spacing=None,
    resize_columns_enabled=true,
    min_column_width=None,
    text_size=None,
    table_width_fixed=true,
    gen_id=None,
    style_id=None,
    scrollable_style_id=None,  
    show=true,
    on_column_resize=None,
    on_column_resize_release=None,
    user_data=None,
    ))]
pub fn add_table(
        window_id: String,
        table_id: String,
        headers: Vec<String>,
        body: Vec<Vec<f32>>,
        footers: Vec<String>,
        column_widths: Vec<f32>,
        height: f32,
        // above required
        parent_id: Option<String>,
        width: Option<f32>,
        resizer_width: Option<f32>,
        header_enabled: bool,
        header_row_height: Option<f32>,
        header_scrollbar_height: Option<f32>,
        header_scrollbar_margin: Option<f32>,
        header_scroller_height: Option<f32>,
        header_scrollbar_spacing: Option<f32>,
        header_row_spacing: Option<f32>,
        footer_height: Option<f32>,
        footer_scrollbar_height: Option<f32>,
        footer_scrollbar_margin: Option<f32>,
        footer_scroller_height: Option<f32>,
        footer_scrollbar_spacing: Option<f32>,
        footer_spacing: Option<f32>,
        body_scrollbar_width: Option<f32>,
        body_scrollbar_margin: Option<f32>,
        body_scroller_width: Option<f32>,
        body_scrollbar_spacing: Option<f32>,
        body_row_highlight: bool,
        custom_header_rows: Option<usize>,
        custom_footer_rows: Option<usize>,
        control_columns: Vec<usize>,
        column_proportional_resize: bool,
        row_spacing: Option<f32>,
        row_height: Option<f32>,
        header_body_spacing: Option<f32>,
        body_footer_spacing: Option<f32>,
        resize_columns_enabled: bool,
        min_column_width: Option<f32>,
        text_size: Option<f32>,
        table_width_fixed: bool,
        gen_id: Option<usize>,
        style_id: Option<usize>,
        scrollable_style_id: Option<usize>,
        show: bool,
        on_column_resize: Option<PyObject>,
        on_column_resize_release: Option<PyObject>,
        user_data: Option<PyObject>,
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
        add_callback_to_mutex(id, "dragging".to_string(), py);
    }

    let released = if let Some(py) = on_column_resize_release {
        add_callback_to_mutex(id, "released".to_string(), py);
        true
    } else {
        false
    };

    set_state_of_container(id, window_id.clone(), Some(table_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, table_id, id, "add_table".to_string());

    state.containers.insert(id, Containers::Table(
        Table {
            id,
            headers,
            body,
            footers,
            column_widths,
            height,
            width,
            resizer_width,
            header_enabled,
            header_row_height,
            header_scrollbar_height,
            header_scrollbar_margin,
            header_scroller_height,
            header_scrollbar_spacing,
            header_row_spacing,
            footer_height,
            footer_scrollbar_height,
            footer_scrollbar_margin,
            footer_scroller_height,
            footer_scrollbar_spacing,
            footer_spacing,
            body_scrollbar_width,
            body_scrollbar_margin,
            body_scroller_width,
            body_scrollbar_spacing,
            body_row_highlight,
            custom_header_rows,
            custom_footer_rows,
            control_columns,
            column_proportional_resize,
            row_spacing,
            row_height,
            header_body_spacing,
            body_footer_spacing,
            resize_columns_enabled,
            min_column_width,
            text_size,
            show,
            table_width_fixed,
            style_id,
            scrollable_style_id,
            released,
            ..Default::default()
        }));

    drop(state);
    Ok(id)

}

