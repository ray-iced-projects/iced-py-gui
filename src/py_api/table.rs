


use pyo3::{Py, PyAny, PyResult, pyfunction};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    state::{IpgContainers, get_id, set_state_cont_wnd_ids, set_state_of_container}, 
        widgets::ipg_table::{IpgTable}};


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

    state.containers.insert(id, IpgContainers::IpgTable(
        IpgTable {
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

