

use iced::Color;
use polars::frame::DataFrame;
use pyo3_polars::PyDataFrame;
use pyo3::{Py, PyAny, PyResult, pyfunction};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex, 
    graphics::colors::IpgColor, state::{IpgContainers, 
        IpgWidgets, get_id, set_state_cont_wnd_ids, set_state_of_container}, 
        widgets::ipg_table::{IpgTable, IpgTableStyle}};


#[pyfunction]
#[pyo3(signature = (
    window_id, 
    table_id, 
    polars_df,
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
        polars_df: PyDataFrame,
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

    let df: DataFrame = polars_df.into();
    
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
            df,
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


#[pyfunction]
#[pyo3(signature = ( 
    header_background_color=None, 
    header_background_rgba=None,
    header_border_color=None, 
    header_border_rgba=None,
    header_border_radius = 0.0, 
    header_border_width=0.0,
    header_text_color=None, 
    header_text_rgba=None,

    body_background_color=None, 
    body_background_rgba=None,
    body_border_color=None, 
    body_border_rgba=None,
    body_border_radius = 0.0, 
    body_border_width=0.0,
    body_text_color=None, 
    body_text_rgba=None,
    body_row_highlight_color=None,
    body_row_highlight_rgba=None,

    footer_background_color=None, 
    footer_background_rgba=None,
    footer_border_color=None, 
    footer_border_rgba=None,
    footer_border_radius = 0.0, 
    footer_border_width=0.0,
    footer_text_color=None, 
    footer_text_rgba=None,

    divider_color=None,
    divider_rgba=None,
    divider_hover_color=None,
    divider_hover_rgba=None,

    scroller_rail_color=None,
    scroller_rail_rgba=None,
    scroller_color=None,
    scroller_rgba=None,
    scroller_hover_color=None,
    scroller_hover_rgba=None,

    gen_id=None
    ))]
pub fn add_table_style(
    header_background_color: Option<IpgColor>,
    header_background_rgba: Option<[f32; 4]>,
    header_border_color: Option<IpgColor>,
    header_border_rgba: Option<[f32; 4]>,
    header_border_radius: f32,
    header_border_width: f32,
    header_text_color: Option<IpgColor>,
    header_text_rgba: Option<[f32; 4]>,

    body_background_color: Option<IpgColor>,
    body_background_rgba: Option<[f32; 4]>,
    body_border_color: Option<IpgColor>,
    body_border_rgba: Option<[f32; 4]>,
    body_border_radius: f32,
    body_border_width: f32,
    body_text_color: Option<IpgColor>,
    body_text_rgba: Option<[f32; 4]>,
    body_row_highlight_color: Option<IpgColor>,
    body_row_highlight_rgba: Option<[f32; 4]>,

    footer_background_color: Option<IpgColor>,
    footer_background_rgba: Option<[f32; 4]>,
    footer_border_color: Option<IpgColor>,
    footer_border_rgba: Option<[f32; 4]>,
    footer_border_radius: f32,
    footer_border_width: f32,
    footer_text_color: Option<IpgColor>,
    footer_text_rgba: Option<[f32; 4]>,

    divider_color: Option<IpgColor>,
    divider_rgba: Option<[f32; 4]>,
    divider_hover_color: Option<IpgColor>,
    divider_hover_rgba: Option<[f32; 4]>,

    scroller_rail_color: Option<IpgColor>,
    scroller_rail_rgba: Option<[f32; 4]>,
    scroller_color: Option<IpgColor>,
    scroller_rgba: Option<[f32; 4]>,
    scroller_hover_color: Option<IpgColor>,
    scroller_hover_rgba: Option<[f32; 4]>,

    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let header_background =  
        IpgColor::rgba_ipg_color_to_iced(header_background_rgba, header_background_color, 1.0, false);
    let header_border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(header_border_rgba, header_border_color, 1.0, false);
    let header_text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(header_text_rgba, header_text_color, 1.0, false);
    
    let body_background: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(body_background_rgba, body_background_color, 1.0, false);
    let body_border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(body_border_rgba, body_border_color, 1.0, false);
    let body_text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(body_text_rgba, body_text_color, 1.0, false);
    let body_row_highlight: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(body_row_highlight_rgba, body_row_highlight_color, 1.0, false);

    let footer_background: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(footer_background_rgba, footer_background_color, 1.0, false);
    let footer_border_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(footer_border_rgba, footer_border_color, 1.0, false);
    let footer_text_color: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(footer_text_rgba, footer_text_color, 1.0, false);

    let divider_background: Option<Color> = 
        IpgColor::rgba_ipg_color_to_iced(divider_rgba, divider_color, 1.0, false);
    let divider_hover_color = 
        IpgColor::rgba_ipg_color_to_iced(divider_hover_rgba, divider_hover_color, 1.0, false);

    let rail = 
        IpgColor::rgba_ipg_color_to_iced(scroller_rail_rgba, scroller_rail_color, 1.0, false);
    let scroller = 
        IpgColor::rgba_ipg_color_to_iced(scroller_rgba, scroller_color, 1.0, false);
    let scroller_hover = 
        IpgColor::rgba_ipg_color_to_iced(scroller_hover_rgba, scroller_hover_color, 1.0, false);
    
    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgTableStyle(
        IpgTableStyle {
            id,
            header_background,
            header_border_color,
            header_border_radius,
            header_border_width,
            header_text_color,
            
            body_background,
            body_border_color,
            body_border_radius,
            body_border_width,
            body_text_color,
            body_row_highlight,
            
            footer_background,
            footer_border_color,
            footer_border_radius,
            footer_border_width,
            footer_text_color,
            
            divider_background,
            divider_hover_color,

            rail,
            scroller,
            scroller_hover,
        }));

    drop(state);
    Ok(id)
}