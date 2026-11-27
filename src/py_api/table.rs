//! Table module - provides add_table pyfunction or Table class

use pyo3::{Py, PyAny, PyResult, pyfunction};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_name_to_mutex, add_user_data_to_mutex, 
    state::{Containers, Widgets, get_id, set_state_cont_wnd_ids, set_state_of_container, 
        set_state_of_widget}, widgets::{callbacks::CallbackName, ipg_sash::SashStyleStd, 
            ipg_table::{Table, TableBasic, TableBody, TableFooter, TableHeader}}};


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
/// body : list of list of str
///     Sets the table body data as rows of any values.
/// column_widths : list of float
///     Sets the initial width of each column in logical pixels.
/// body_row_heights : float, Optional
///     Sets the height of the body rows, default 30.0.
/// body_row_highlight : bool, default False
///     Whether to highlight alternating body rows.
/// body_text_format_id: int, Optional
///     Sets the id via the add_text_format() method.
/// body_text_style_id: int, Optional
///     Sets the id via the add_text_style() method.
/// body_row_outline : bool, default True
///     Whether to show an outline on body rows.
/// footer : list of str, Optional
///     Sets the column footer labels.
/// footer_height : float, Optional
///     Sets the height of the footer row.
/// footer_text_format_id: int, Optional
///     Sets the id via the add_text_format() method.
/// footer_text_style_id: int, Optional
///     Sets the id via the add_text_style() method.
/// header : list of str, Optional
///     Sets the column header labels.
/// header_height : float, Optional
///     Sets the height of the header row.
/// header_text_format_id: int, Optional
///     Sets the id via the add_text_format() method.
/// header_text_style_id: int, Optional
///     Sets the id via the add_text_style() method.
/// header_body_divider_rule_height: float = 3.0
///     The dividing rule between header and body.
/// body_footer_divider_rule_height: float = 0.0
///     The dividing rule between body and footer.
/// rule_style_id: int, Optional
///     Style of the dividing rules.
/// min_size : float, default 0.0
///     Sets the minimum column width in logical pixels.
/// resize_columns_enabled : bool, default True
///     Whether column resizing is enabled.
/// resize_table_enabled : bool, default False
///     Whether table resizing is enabled.
/// sash_size : float, default 4.0
///     Sets the width of the column sash (drag handle).
/// sash_style_id : int, Optional
///     Sets the ID of a custom style for the sash.
/// scrollable_height : float, Optional
///     Sets the height of the scrollable area.
/// parent_id : str, Optional
///     Sets the parent container ID. Defaults to the window itself.
/// show : bool, default True
///     Whether the table is visible.
/// on_column_resize : callable, Optional
///     Sets the callback method to invoke when a column is being resized.
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
    column_widths,
    // above required
    body_row_heights=None,
    body_row_highlight=false,
    body_row_outline=true,
    body_text_format_id=None,
    body_text_style_id=None,
    footer_height=None,
    footer=None,
    footer_text_format_id=None,
    footer_text_style_id=None,
    gen_id=None,
    header_height=None,
    header=None,
    header_text_format_id=None,
    header_text_style_id=None,
    header_body_divider_rule_height=3.0,
    body_footer_divider_rule_height=0.0,
    rule_style_id=None,
    min_size=0.0,
    on_column_resize=None,
    resize_columns_enabled=true,
    resize_table_enabled=false,
    header_sash_size=4.0,
    body_sash_size=4.0,
    footer_sash_size=4.0,
    header_sash_style_id=None,
    body_sash_style_id=None,
    footer_sash_style_id=None,
    header_sash_style_std=None, 
    body_sash_style_std=None, 
    footer_sash_style_std=None, 
    scrollable_height=None,  
    show=true,
    user_data=None,
    ))]
pub fn add_table_basic(
    parent_id: String,
    body: Vec<Vec<String>>,
    column_widths: Vec<f32>,
    // above required
    body_row_heights: Option<f32>,
    body_row_highlight: bool,
    body_row_outline: bool,
    body_text_format_id: Option<usize>,
    body_text_style_id: Option<usize>,
    footer_height: Option<f32>,
    footer: Option<Vec<String>>,
    footer_text_format_id: Option<usize>,
    footer_text_style_id: Option<usize>,
    gen_id: Option<usize>,
    header_height: Option<f32>,
    header: Option<Vec<String>>,
    header_text_format_id: Option<usize>,
    header_text_style_id: Option<usize>,
    header_body_divider_rule_height: f32,
    body_footer_divider_rule_height: f32,
    rule_style_id: Option<usize>,
    min_size: f32,
    on_column_resize: Option<PyObject>,
    resize_columns_enabled: bool,
    resize_table_enabled: bool,
    header_sash_size: f32,
    body_sash_size: f32,
    footer_sash_size: f32,
    header_sash_style_id: Option<usize>,
    body_sash_style_id: Option<usize>,
    footer_sash_style_id: Option<usize>,
    header_sash_style_std: Option<SashStyleStd>, 
    body_sash_style_std: Option<SashStyleStd>, 
    footer_sash_style_std: Option<SashStyleStd>, 
    scrollable_height: Option<f32>,
    show: bool,
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
            body_text_format_id,
            body_text_style_id,
            column_widths,
            footer_height,
            footer,
            footer_text_format_id,
            footer_text_style_id,
            header_height,
            header,
            header_text_format_id,
            header_text_style_id,
            header_body_divider_rule_height,
            body_footer_divider_rule_height,
            rule_style_id,min_size,
            resize_columns_enabled,
            resize_table_enabled,
            header_sash_size,
            body_sash_size,
            footer_sash_size,
            header_sash_style_id,
            body_sash_style_id,
            footer_sash_style_id,
            scrollable_height,
            show, 
            header_sash_style_std, 
            body_sash_style_std, 
            footer_sash_style_std, 
        }));

    drop(state);
    Ok(id)

}

/// Add a Table widget.
///
/// An advanced table with that allow one to add widgets
/// to construct the header, body, and footer.
///
/// Parameters
/// ----------
/// window_id: str
///     Sets the window this column belongs to.
/// container_id: str
///     Sets the Unique string identifier for the column.
/// parent_id: str,  Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// col_widths: list[float]
///     Sets the number, length of the list, and sizes of the columns.
/// sash_size: float = 4.0
///     Sets the sizes of the column divider (sash).
/// min_size: float = 0.0
///     Sets the min_size of the column when resizing.
/// on_column_resize : callable, Optional
///     Sets the callback method to invoke when a column is being resized.
/// user_data : Any, Optional
///     Sets arbitrary data forwarded to callbacks.
/// show: bool = True
///     Whether to show the Table or not.
/// 
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created table.
#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    col_widths,
    parent_id=None,
    sash_size=4.0,
    min_size=0.0,
    on_column_resize=None,
    user_data=None,
    show=true,
    ))]
pub fn add_table(
        window_id: String,
        container_id: String,
        col_widths: Vec<f32>,
        parent_id: Option<String>,
        sash_size: f32,
        min_size: f32,
        on_column_resize: Option<PyObject>,
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

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_table".to_string());

    state.containers.insert(id, Containers::Table(
        Table { 
            id, 
            col_widths, 
            sash_size, 
            min_size, 
            show 
        }));

    drop(state);
    Ok(id)

}

/// Add a TableHeader widget.
///
/// An advanced table with that allow one to add widgets
/// to construct the header.  If the row parameters are different,
/// like row_height, one can add multiple headers and therefore
/// have different row hieghts, for example.
/// If the parameters are not different, then only one header is needed,
/// as long as the len of the list of widgets are in multiples of the needed
/// rows, i.e. 2 rows of 8 = 16 widgets. if a header column is not needed,
/// add an empty widget like add_text("") or add_space().  
///
/// Parameters
/// ----------
/// window_id: str
///     Sets the window this column belongs to.
/// container_id: str
///     Sets the Unique string identifier for the column.
/// parent_id: str,  Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// row_height: float = 30.0
///     Sets the height of the header row(s).
/// style_id: int, Optional,
///     Sets the id of the header style via add_table_style() method.
/// sash_style_id: int, Optional
///     Sets the id of the sash, divider, via add_sash_style() method.
/// sash_style_std: int, Optional
///     Sets the standard style method the the sash via SashStdStyle class,
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created table.
#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    parent_id=None,
    row_height=None,
    style_id=None,
    sash_style_id=None,
    sash_style_std=None,
    ))]
pub fn add_table_header(
        window_id: String,
        container_id: String,
        parent_id: Option<String>,
        row_height: Option<f32>,
        style_id: Option<usize>,
        sash_style_id: Option<usize>,
        sash_style_std: Option<SashStyleStd>,
    ) -> PyResult<usize> 
{

    let id = get_id(None);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_table".to_string());

    state.containers.insert(id, Containers::TableHeader(
        TableHeader{
            id,
            row_height,
            style_id,
            sash_style_id,
            sash_style_std,
        }));

    drop(state);
    Ok(id)

}

/// Add a TableBody widget.
///
/// An advanced table with that allow one to add widgets
/// to construct the body.
/// The len of the list of widgets are in multiples of the needed
/// rows and columns, i.e. 10 rows of 8 = 80 widgets. if a row item is blank,
/// add an empty widget like add_text("") or add_space().  
///
/// Parameters
/// ----------
/// window_id: str
///     Sets the window this column belongs to.
/// container_id: str
///     Sets the Unique string identifier for the column.
/// parent_id: str,  Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// row_height: float = 30.0
///     Sets the height of the header row(s).
/// body_width: float, Optional
///     If the body_width is set, a scrollable widget
///     is added and will show if the width exceeds the body_width value.
/// body_body_height: float, Optional
///     If the body_height is set, a scrollable widget
///     is added and will show if the height exceeds the body_height value.
/// style_id: int, Optional,
///     Sets the id of the header style via add_table_style() method.
/// sash_style_id: int, Optional
///     Sets the id of the sash, divider, via add_sash_style() method.
/// sash_style_std: int, Optional
///     Sets the standard style method the the sash via SashStdStyle class,
/// scroller_x_id: int, Optional
///     If set via add_scroller() method, allows one to modify the scroller parameters.
/// scroller_y_id: int, Optional
///     If set via add_scroller() method, allows one to modify the scroller parameters.
/// scroller_style_id=None,
///     If set via add_scroller_style() method, allows one to modify the scroller style.
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created table.
#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    parent_id=None,
    row_height=None,
    body_width=None,
    body_height=None,
    style_id=None,
    sash_style_id=None,
    sash_style_std=None,
    scroller_x_id=None,
    scroller_y_id=None,
    scroller_style_id=None,
    ))]
pub fn add_table_body(
        window_id: String,
        container_id: String,
        parent_id: Option<String>,
        row_height: Option<f32>,
        body_width: Option<f32>,
        body_height: Option<f32>,
        style_id: Option<usize>,
        sash_style_id: Option<usize>,
        sash_style_std: Option<SashStyleStd>,
        scroller_x_id: Option<usize>,
        scroller_y_id: Option<usize>,
        scroller_style_id: Option<usize>,
    ) -> PyResult<usize> 
{

    let id = get_id(None);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_table".to_string());

    state.containers.insert(id, Containers::TableBody(
        TableBody {
            id,
            row_height,
            body_width,
            body_height,
            style_id,
            sash_style_id,
            sash_style_std,
            scroller_x_id,
            scroller_y_id,
            scroller_style_id,
        }));

    drop(state);
    Ok(id)

}


/// Add a TableFooter widget.
///
/// An advanced table with that allow one to add widgets
/// to construct the footer.  If the row parameters are different,
/// like row_height, one can add multiple footers and therefore
/// have different row hieghts, for example.
/// If the parameters are not different, then only one footer is needed,
/// as long as the len of the list of widgets are in multiples of the needed
/// rows, i.e. 2 rows of 8 = 16 widgets. if a footer column is not needed,
/// add an empty widget like add_text("") or add_space().  
///
/// Parameters
/// ----------
/// window_id: str
///     Sets the window this column belongs to.
/// container_id: str
///     Sets the Unique string identifier for the column.
/// parent_id: str,  Optional
///     Sets the parent container ID.  Defaults to the window itself.
/// row_height: float = 30.0
///     Sets the height of the footer row(s).
/// style_id: int, Optional,
///     Sets the id of the footer style via add_table_style() method.
/// sash_style_id: int, Optional
///     Sets the id of the sash, divider, via add_sash_style() method.
/// sash_style_std: int, Optional
///     Sets the standard style method the the sash via SashStdStyle class,
/// 
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created table.
#[pyfunction]
#[pyo3(signature = (
    window_id,
    container_id,
    parent_id=None,
    height=None,
    style_id=None,
    sash_style_id=None,
    sash_style_std=None, 
    ))]
pub fn add_table_footer(
        window_id: String,
        container_id: String,
        parent_id: Option<String>,
        height: Option<f32>,
        style_id: Option<usize>,
        sash_style_id: Option<usize>,
        sash_style_std: Option<SashStyleStd>,
    ) -> PyResult<usize> 
{

    let id = get_id(None);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_table".to_string());

    state.containers.insert(id, Containers::TableFooter(
        TableFooter {
            id,height,
            style_id,
            sash_style_id, 
            sash_style_std, 
        }));

    drop(state);
    Ok(id)

}
