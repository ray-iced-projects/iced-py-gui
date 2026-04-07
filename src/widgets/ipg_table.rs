//! ipg_table
#![allow(clippy::unit_arg)]
#![allow(unused)]
use std::collections::HashMap;

use crate::app::Message;
use crate::state::{Containers, Widgets};
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::divider::{self, divider_horizontal};
use crate::py_api::helpers::try_extract_vec_usize;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_f32, set_opt_f32, set_opt_usize,
    set_vec_f32, set_vec_string, set_vec_vec_f32
};
use crate::IpgState;

use iced::border::Radius;
use iced::widget::{scrollable::Scrollbar};
use iced::{Color, Length, alignment, widget};
use iced::Length::Fill;
use iced::{Element, Renderer, Theme};
use iced::widget::{Space, column, container, row, scrollable, stack, text};

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone, Default)]
pub struct Table {
        pub id: usize,
        pub headers: Vec<String>,
        pub body: Vec<Vec<f32>>,
        pub footers: Vec<String>,
        pub column_widths: Vec<f32>,
        pub height: f32,
        // above required
        pub width: Option<f32>,
        pub resizer_width: Option<f32>,
        pub header_enabled: bool,
        pub header_row_height: Option<f32>,
        pub header_scrollbar_height: Option<f32>,
        pub header_scrollbar_margin: Option<f32>,
        pub header_scroller_height: Option<f32>,
        pub header_scrollbar_spacing: Option<f32>,
        pub header_row_spacing: Option<f32>,
        pub footer_height: Option<f32>,
        pub footer_scrollbar_height: Option<f32>,
        pub footer_scrollbar_margin: Option<f32>,
        pub footer_scroller_height: Option<f32>,
        pub footer_scrollbar_spacing: Option<f32>,
        pub footer_spacing: Option<f32>,
        pub body_scrollbar_width: Option<f32>,
        pub body_scrollbar_margin: Option<f32>,
        pub body_scroller_width: Option<f32>,
        pub body_scrollbar_spacing: Option<f32>,
        pub body_row_highlight: bool,
        pub custom_header_rows: Option<usize>,
        pub custom_footer_rows: Option<usize>,
        pub control_columns: Vec<usize>,
        pub column_proportional_resize: bool,
        pub row_spacing: Option<f32>,
        pub row_height: Option<f32>,
        pub header_body_spacing: Option<f32>,
        pub body_footer_spacing: Option<f32>,
        pub resize_columns_enabled: bool,
        pub min_column_width: Option<f32>,
        pub text_size: Option<f32>,
        pub show: bool,
        pub table_width_fixed: bool,
        pub style_id: Option<usize>,
        pub scrollable_style_id: Option<usize>,
        pub released: bool, 
}

impl Table {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message, Theme, Renderer>>,
        widgets: &'a HashMap<usize, Widgets>,
    ) -> Element<'a, Message, Theme, Renderer> {
        
        let ipg_scroll_style_header  = self.lookup(widgets, self.style_id)
            .and_then(Widgets::as_scrollable_style).cloned();
         let ipg_scroll_style_body  = self.lookup(widgets, self.style_id)
            .and_then(Widgets::as_scrollable_style).cloned();
         let ipg_scroll_style_footer  = self.lookup(widgets, self.style_id)
            .and_then(Widgets::as_scrollable_style).cloned();

        let mut body_rows = vec![];
            for idx in 0..self.body.len() {
                    let mut rw = vec![];
                    for (i, item) in self.body[idx].iter().enumerate() {
                        let cell = if !self.control_columns.contains(&i) {
                                let txt = 
                                    text(item.to_string())
                                    .align_x(alignment::Horizontal::Center)
                                    .align_y(alignment::Vertical::Center)
                                    .width(self.column_widths[i]);
                                let txt = match self.text_size {
                                    Some(sz) if sz > 0.0 => txt.size(sz),
                                    _ => txt,
                                };
                                txt.into()
                        } else {
                            content.remove(0)
                        };
                        rw.push(Element::from(container(cell)
                                .width(self.column_widths[i])
                                .center_x(self.column_widths[i])
                            ));
                    }
                
                body_rows.push(row(rw).into());
            }

            let body_column = column(body_rows)
                                                    .spacing(self.row_spacing.unwrap_or_default());
            let (table_width, scroller_needed) = self.width.map_or_else(
                || (self.column_widths.iter().sum(), false),
                |width| {
                    let total_width: f32 = self.column_widths.iter().sum();
                    if width < total_width {
                        (width, true)
                    } else {
                        (width, false)
                    }
                },
            );
            
            let body: Element<Message> = 
                scrollable(body_column)
                    .height(self.height)
                    .width(table_width)
                    .on_scroll(move|vp|Message::TableScrolled(
                                    vp, self.id))
                    .direction({
                        let scrollbar = Scrollbar::new()
                            .scroller_width(self.body_scroller_width.unwrap_or(5.0))
                            .width(self.body_scrollbar_width.unwrap_or(5.0))
                            .margin(self.body_scrollbar_margin.unwrap_or_default());
                        scrollable::Direction::Both {
                            horizontal: scrollbar,
                            vertical: scrollbar,
                        }
                    })
                    .style(move|theme, status| {
                        if let Some(ipg_style) = &ipg_scroll_style_body {
                            ipg_style.set_style(theme, status, widgets)
                        } else {
                            scrollable::default(theme, status)
                        }
                        
                    })
                    .into();
            
            let header_height = if self.header_enabled {
                self.header_row_height.unwrap_or(20.0)
            } else {
                0.0
            };

            let custom_header_height = if let Some(c_rows) = self.custom_header_rows {
                self.header_row_height.unwrap_or(c_rows as f32 * 20.0)
            } else {
                0.0
            };

            let mut header_column = vec![];

            // add the header if enabled
            if self.header_enabled {
                let mut rw = vec![];
                for (i, hd) in self.headers.iter().enumerate() {
                        let txt = 
                            text(hd)
                            // .size(self.text_size.unwrap_or_default())
                            .align_x(alignment::Horizontal::Center)
                            .align_y(alignment::Vertical::Center)
                            .width(Fill)
                            .height(Fill);
                        let txt = match self.text_size {
                            Some(sz) if sz > 0.0 => txt.size(sz),
                            _ => txt,
                        };
                    rw.push(Element::from(
                        container(txt)
                            .width(self.column_widths[i])
                            .height(header_height)
                        ));
                }
                header_column.push(Element::from(row(rw)));
            }
                
            // add any custom header rows
            if self.custom_header_rows.is_some() {
                for _ in 0..self.custom_header_rows.unwrap() {
                    let mut custom_rw = vec![];
                    for i in 0..self.column_widths.len() {
                        custom_rw.push(Element::from(
                            container(content.remove(0))
                                .width(self.column_widths[i])
                                .height(custom_header_height)
                                .center_x(self.column_widths[i])
                            ));
                    }
                    header_column.push(Element::from(row(custom_rw)));
                }
            }

            let header = if header_column.len() > 0 {
                let hd_col = column(header_column)
                                                    .spacing(self.header_row_spacing.unwrap_or_default()).into();
                if scroller_needed {
                    Some(Element::from(
                        scrollable(hd_col)
                            .id(widget::Id::unique())
                            .width(table_width)
                            .direction({
                                let scrollbar = scrollable::Scrollbar::new()
                                    .scroller_width(self.header_scroller_height.unwrap_or(5.0))
                                    .width(self.header_scrollbar_height.unwrap_or(5.0))
                                    .margin(self.header_scrollbar_margin.unwrap_or_default())
                                    .spacing(self.header_scrollbar_spacing.unwrap_or_default());
                                scrollable::Direction::Horizontal(scrollbar)
                                })
                            .on_scroll(move|vp| Message::TableScrolled(
                                                vp, self.id))
                            .style(move|theme, status| {
                                if let Some(ipg_style) = &ipg_scroll_style_header{
                                    ipg_style.set_style(theme, status, widgets)
                                } else {
                                    scrollable::default(theme, status)
                                }
                            })
                        ))
                } else {
                    Some(hd_col)
                }
            } else {
                None
            };

            let footer = if self.custom_footer_rows.is_some() {
                let mut footer_column= vec![];
                for _ in 0..self.custom_footer_rows.unwrap_or_default() {
                    let mut rw = vec![];
                    for i in 0..self.column_widths.len() {
                        rw.push(Element::from(
                            container(content.remove(0))
                                .width(self.column_widths[i])
                                .height(Length::Fixed(self.footer_height.unwrap_or(20.0)))
                                .center_x(self.column_widths[i])
                            ));
                    }
                    footer_column.push(Element::from(row(rw)));
                }
                let ft_col = column(footer_column)
                                                    .spacing(self.footer_spacing.unwrap_or_default()).into();
                if scroller_needed {
                    Some(Element::from(
                        scrollable(ft_col)
                            .id(widget::Id::unique())
                            .width(table_width)
                            .direction({
                                let scrollbar = scrollable::Scrollbar::new()
                                    .scroller_width(self.footer_scroller_height.unwrap_or(5.0))
                                    .width(self.footer_scrollbar_height.unwrap_or(5.0))
                                    .margin(self.footer_scrollbar_margin.unwrap_or_default())
                                    .spacing(self.footer_scrollbar_spacing.unwrap_or_default());
                                scrollable::Direction::Horizontal(scrollbar)
                                })
                            .on_scroll(move|vp| Message::TableScrolled(
                                                vp, self.id))
                            .style(move|theme, status| {
                                if let Some(ipg_style) = &ipg_scroll_style_footer {
                                    ipg_style.set_style(theme, status, widgets)
                                } else {
                                    scrollable::default(theme, status)
                                }
                            })
                    ))
                } else {
                    Some(ft_col)
                }
            } else {
                None
            };

            let div_body = 
                divider_horizontal(
                    self.id,
                    self.column_widths.clone(),
                    self.resizer_width.unwrap_or(4.0),
                    self.height,
                    Message::TableDividerChanged,
                )
                .include_last_handle(!self.resize_columns_enabled)
                .on_release(Message::TableDividerReleased(self.id));
            
            let handle_height = header_height + self.custom_header_rows.unwrap_or_default() as f32 * 
                self.header_row_height.unwrap_or(20.0);
            let div_header = 
                divider_horizontal(
                    self.id,
                    self.column_widths.clone(),
                    self.resizer_width.unwrap_or(4.0),
                    handle_height,
                    Message::TableDividerChanged,
                )
                .include_last_handle(!self.resize_columns_enabled)
                .on_release(Message::TableDividerReleased(self.id))
                .style(move|theme, status| default_divider_style(theme, status));

            let handle_height = self.custom_footer_rows.unwrap_or_default() as f32 * self.footer_height.unwrap_or(20.0);
            let div_footer = 
                divider_horizontal(
                    self.id,
                    self.column_widths.clone(),
                    self.resizer_width.unwrap_or(4.0),
                    handle_height,
                    Message::TableDividerChanged,
                )
                .include_last_handle(!self.resize_columns_enabled)
                .on_release(Message::TableDividerReleased(self.id))
                .style(move|theme, status| default_divider_style(theme, status));

            let mut main_col = vec![];

            if header.is_some() && self.resize_columns_enabled {
                let header_stk = 
                    stack([header.unwrap(), div_header.into()]).into();
                main_col.push(header_stk);
                main_col.push(Space::new().width(5.0).height(self.header_body_spacing.unwrap_or(5.0)).into());

            } else if header.is_some() && !self.resize_columns_enabled {
                main_col.push(header.unwrap());
                main_col.push(Space::new().width(5.0).height(self.header_body_spacing.unwrap_or(5.0)).into());
            }
            
            if self.resize_columns_enabled {
                main_col.push(stack([body.into(), div_body.into()]).into())
            } else {
                main_col.push(body.into());
            }

            if footer.is_some() {
                main_col.push(Space::new().width(5.0).height(self.body_footer_spacing.unwrap_or(5.0)).into());
            }

            if footer.is_some() && self.resize_columns_enabled {
                let stk = stack([footer.unwrap().into(), div_footer.into()]).into();
                main_col.push(stk);
            } else if footer.is_some() && !self.resize_columns_enabled {
                main_col.push(footer.unwrap());
            }

            column(main_col).into()

    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TableMessage {
    DivDragging((usize, f32)),
    DivOnRelease,
}

pub fn table_callback(
        state: &mut IpgState,  
        id: usize,  
        message: TableMessage) 
{

    match message {
        TableMessage::DivDragging((index, value)) => {
            dbg!(&index, &value);
            if let Some(Containers::Table(tbl)) = state.containers.get_mut(&id) {

                let value = if value < tbl.min_column_width.unwrap_or_default() {
                    tbl.min_column_width.unwrap_or_default()
                } else {
                    value
                };

                if tbl.table_width_fixed && index == tbl.column_widths.len()-1 {
                    // don't change width
                } else {

                    // get diff
                    let diff = tbl.column_widths[index] - value;

                    // change the widths porportionally if enabled
                    if !tbl.table_width_fixed && index == tbl.column_widths.len()-1 {
                        if tbl.column_proportional_resize {
                            let table_width: f32 = tbl.column_widths.iter().sum();
                            let percent = 1.0 - diff/table_width;
                            
                            let mut new_widths = vec![];
                            for width in tbl.column_widths.iter() {
                                new_widths.push(width * percent)
                            }
                            
                            tbl.column_widths = new_widths.clone();
                        }
                    }
                    
                    // # Adjust the left side
                    tbl.column_widths[index] = value;
                    
                    // # Adjust the right side unless at end
                    if index < tbl.column_widths.len()-1 {
                            tbl.column_widths[index+1] += diff
                    }
                }
                    
                invoke_callback_with_args(id, "dragging", "Table", (index, value));
            }
        },
        TableMessage::DivOnRelease=> {
            invoke_callback_with_args(id, "released", "Table", ());
        },
    }
}

// // Table Divider dragging
// pub fn process_callback1(
//         id: usize, 
//         event_name: String, 
//         index: usize, 
//         value: Vec<f32>) 
// {
//     let ud1 = access_user_data1();
//     let app_cbs = access_callbacks();

//     // Retrieve the callback
//     let callback = match app_cbs.callbacks.get(&(id, event_name)) {
//         Some(cb) => Python::attach(|py| cb.clone_ref(py)),
//         None => return,
//     };

//     drop(app_cbs);

//     // Check user data from ud1
//     if let Some(user_data) = ud1.user_data.get(&id) {
//         Python::attach(|py| {
//             if let Err(err) = callback.call1(py, (id, index, value, user_data)) {
//                 panic!("Table callback error: {err}");
//             }
//         });
//         drop(ud1); // Drop ud1 before processing ud2
//         return;
//     }
//     drop(ud1); // Drop ud1 if no user data is found

//     // Check user data from ud2
//     let ud2 = access_user_data2();
//     if let Some(user_data) = ud2.user_data.get(&id) {
//         Python::attach(|py| {
//             if let Err(err) = callback.call1(py, (id, index, value, user_data)) {
//                 panic!("Table callback error: {err}");
//             }
//         });
//         drop(ud2); // Drop ud2 after processing
//         return;
//     }
//     drop(ud2); // Drop ud2 if no user data is found

//     // If no user data is found in both ud1 and ud2, call the callback with only the id, index, and value
//     Python::attach(|py| {
//         if let Err(err) = callback.call1(py, (id, index, value)) {
//             panic!("Table callback error: {err}");
//         }
//     });

// }

// // Table Divider released
// pub fn process_callback2(
//         id: usize, 
//         event_name: String) 
// {
//      let ud1 = access_user_data1();
//     let app_cbs = access_callbacks();

//     // Retrieve the callback
//     let callback = match app_cbs.callbacks.get(&(id, event_name)) {
//         Some(cb) => Python::attach(|py| cb.clone_ref(py)),
//         None => return,
//     };

//     drop(app_cbs);

//     // Check user data from ud1
//     if let Some(user_data) = ud1.user_data.get(&id) {
//         Python::attach(|py| {
//             if let Err(err) = callback.call1(py, (id, user_data)) {
//                 panic!("Table Divider release callback error: {err}");
//             }
//         });
//         drop(ud1); // Drop ud1 before processing ud2
//         return;
//     }
//     drop(ud1); // Drop ud1 if no user data is found

//     // Check user data from ud2
//     let ud2 = access_user_data2();
//     if let Some(user_data) = ud2.user_data.get(&id) {
//         Python::attach(|py| {
//             if let Err(err) = callback.call1(py, (id, user_data)) {
//                 panic!("Table Divider release callback error: {err}");
//             }
//         });
//         drop(ud2); // Drop ud2 after processing
//         return;
//     }
//     drop(ud2); // Drop ud2 if no user data is found

//     // If no user data is found in both ud1 and ud2, call the callback with only the id
//     Python::attach(|py| {
//         if let Err(err) = callback.call1(py, (id,)) {
//             panic!("Table Divider release callback error: {err}");
//         }
//     });

// }


#[derive(Debug, Clone, PartialEq)]
struct DividerStyle {
    background: Option<Color>,
    hover: Option<Color>,
}

const ROW_COLOR: Color = Color::from_rgba(0.04, 0.35, 0.35, 0.2);
const ROW_CONTRAST_COLOR: Color = Color::from_rgba(0.25, 0.63, 0.67, 1.0);


pub fn default_divider_style(_theme: &Theme, status: divider::Status) -> divider::Style {
    let background = match status {
        divider::Status::Active => ROW_COLOR.into(),
       _ => ROW_CONTRAST_COLOR.into(),
    };
    divider::Style {
        background,
        border_width: 0.0,
        border_color: Color::TRANSPARENT,
        border_radius: Radius::from(0.0),
    }
}

fn get_divider_style(
        style_opt: &Option<DividerStyle>, 
        theme: &Theme, 
        status: divider::Status) 
        -> divider::Style {
    
    let mut style = default_divider_style(theme, status);

    if style_opt.is_none() {
        return style
    }
    
    let style_opt = style_opt.clone().unwrap();

    style.background = 
    match (style_opt.background.is_some(),  
            style_opt.hover.is_some()) {
        (true, true) => {
            match status {
                divider::Status::Active => style_opt.background.unwrap().into(),
                _ => style_opt.hover.unwrap().into(),
            }
        },
        (true, false) => {
            match status {
                divider::Status::Active => style_opt.background.unwrap().into(),
                _ => style.background,
            }
        },
        (false, true,) => {
            match status {
                divider::Status::Active => style.background,
                _ => style_opt.hover.unwrap().into(),
            }
        },
        _ => style.background,
    };

    
    style
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TableParam {
    Headers,
    Body,
    Footers,
    ColumnWidths,
    Height,
    Width,
    ResizerWidth,
    HeaderEnabled,
    HeaderHeight,
    HeaderRowSpacing,
    FooterHeight,
    FooterSpacing,
    CustomHeaderRows,
    CustomFooterRows,
    ControlColumns,
    ColumnProportionalResize,
    RowSpacing,
    RowHeight,
    HeaderBodySpacing,
    BodyFooterSpacing,
    ResizeColumnsEnabled,
    MinColumnWidth,
    TextSize,
    Show,
    TableWidthFixed,
    StyleId,
    ScrollableStyleId,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Table {
    type Param = TableParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TableParam::Headers => set_vec_string(&mut self.headers, value, "Headers"),
            TableParam::Body => set_vec_vec_f32(&mut self.body, value, "Body"),
            TableParam::Footers => set_vec_string(&mut self.footers, value, "Footers"),
            TableParam::ColumnWidths => set_vec_f32(&mut self.column_widths, value, "ColumnWidths"),
            TableParam::Height => set_f32(&mut self.height, value, "Height"),
            TableParam::Width => set_opt_f32(&mut self.width, value, "Width"),
            TableParam::ResizerWidth => set_opt_f32(&mut self.resizer_width, value, "ResizerWidth"),
            TableParam::HeaderEnabled => set_bool(&mut self.header_enabled, value, "HeaderEnabled"),
            TableParam::HeaderHeight => set_opt_f32(&mut self.header_row_height, value, "HeaderHeight"),
            TableParam::HeaderRowSpacing => set_opt_f32(&mut self.header_row_spacing, value, "HeaderRowSpacing"),
            TableParam::FooterHeight => set_opt_f32(&mut self.footer_height, value, "FooterHeight"),
            TableParam::FooterSpacing => set_opt_f32(&mut self.footer_spacing, value, "FooterSpacing"),
            TableParam::CustomHeaderRows => set_opt_usize(&mut self.custom_header_rows, value, "CustomHeaderRows"),
            TableParam::CustomFooterRows => set_opt_usize(&mut self.custom_footer_rows, value, "CustomFooterRows"),
            TableParam::ControlColumns => {
                self.control_columns = try_extract_vec_usize(value, "ControlColumns");
            }
            TableParam::ColumnProportionalResize => set_bool(&mut self.column_proportional_resize, value, "ColumnProportionalResize"),
            TableParam::RowSpacing => set_opt_f32(&mut self.row_spacing, value, "RowSpacing"),
            TableParam::RowHeight => set_opt_f32(&mut self.row_height, value, "RowHeight"),
            TableParam::HeaderBodySpacing => set_opt_f32(&mut self.header_body_spacing, value, "HeaderBodySpacing"),
            TableParam::BodyFooterSpacing => set_opt_f32(&mut self.body_footer_spacing, value, "BodyFooterSpacing"),
            TableParam::ResizeColumnsEnabled => set_bool(&mut self.resize_columns_enabled, value, "ResizeColumnsEnabled"),
            TableParam::MinColumnWidth => set_opt_f32(&mut self.min_column_width, value, "MinColumnWidth"),
            TableParam::TextSize => set_opt_f32(&mut self.text_size, value, "TextSize"),
            TableParam::Show => set_bool(&mut self.show, value, "Show"),
            TableParam::TableWidthFixed => set_bool(&mut self.table_width_fixed, value, "TableWidthFixed"),
            TableParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            TableParam::ScrollableStyleId => set_opt_usize(&mut self.scrollable_style_id, value, "ScrollableStyleId"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_table() -> Table {
        Table {
            id: 0,
            headers: vec!["A".into(), "B".into()],
            body: vec![vec![1.0, 2.0]],
            footers: vec![],
            column_widths: vec![100.0, 100.0],
            height: 200.0,
            width: None,
            resizer_width: None,
            header_enabled: true,
            header_row_height: None,
            header_scrollbar_height: None,
            header_scrollbar_margin: None,
            header_scroller_height: None,
            header_scrollbar_spacing: None,
            header_row_spacing: None,
            footer_height: None,
            footer_scrollbar_height: None,
            footer_scrollbar_margin: None,
            footer_scroller_height: None,
            footer_scrollbar_spacing: None,
            footer_spacing: None,
            body_scrollbar_width: None,
            body_scrollbar_margin: None,
            body_scroller_width: None,
            body_scrollbar_spacing: None,
            body_row_highlight: false,
            custom_header_rows: None,
            custom_footer_rows: None,
            control_columns: vec![],
            column_proportional_resize: false,
            row_spacing: None,
            row_height: None,
            header_body_spacing: None,
            body_footer_spacing: None,
            resize_columns_enabled: false,
            min_column_width: None,
            text_size: None,
            show: true,
            table_width_fixed: false,
            style_id: None,
            scrollable_style_id: None,
            released: false,
        }
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::attach(|py| py.None().into_py_any(py).unwrap())
    }

    #[test]
    fn test_headers() {
        let mut t = make_table();
        t.param_update(TableParam::Headers, &py_obj(vec!["X".to_string(), "Y".to_string()]));
        assert_eq!(t.headers, vec!["X", "Y"]);
    }

    #[test]
    fn test_body() {
        let mut t = make_table();
        t.param_update(TableParam::Body, &py_obj(vec![vec![3.0f32, 4.0]]));
        assert_eq!(t.body, vec![vec![3.0, 4.0]]);
    }

    #[test]
    fn test_footers() {
        let mut t = make_table();
        t.param_update(TableParam::Footers, &py_obj(vec!["total".to_string()]));
        assert_eq!(t.footers, vec!["total"]);
    }

    #[test]
    fn test_column_widths() {
        let mut t = make_table();
        t.param_update(TableParam::ColumnWidths, &py_obj(vec![50.0f32, 75.0]));
        assert_eq!(t.column_widths, vec![50.0, 75.0]);
    }

    #[test]
    fn test_height() {
        let mut t = make_table();
        t.param_update(TableParam::Height, &py_obj(300.0f32));
        assert_eq!(t.height, 300.0);
    }

    #[test]
    fn test_width() {
        let mut t = make_table();
        t.param_update(TableParam::Width, &py_obj(500.0f32));
        assert_eq!(t.width, Some(500.0));
        t.param_update(TableParam::Width, &py_none());
        assert_eq!(t.width, None);
    }

    #[test]
    fn test_resizer_width() {
        let mut t = make_table();
        t.param_update(TableParam::ResizerWidth, &py_obj(5.0f32));
        assert_eq!(t.resizer_width, Some(5.0));
    }

    #[test]
    fn test_header_enabled() {
        let mut t = make_table();
        t.param_update(TableParam::HeaderEnabled, &py_obj(false));
        assert!(!t.header_enabled);
    }

    #[test]
    fn test_header_height() {
        let mut t = make_table();
        t.param_update(TableParam::HeaderHeight, &py_obj(30.0f32));
        assert_eq!(t.header_row_height, Some(30.0));
    }

    #[test]
    fn test_header_row_spacing() {
        let mut t = make_table();
        t.param_update(TableParam::HeaderRowSpacing, &py_obj(4.0f32));
        assert_eq!(t.header_row_spacing, Some(4.0));
    }

    #[test]
    fn test_footer_height() {
        let mut t = make_table();
        t.param_update(TableParam::FooterHeight, &py_obj(25.0f32));
        assert_eq!(t.footer_height, Some(25.0));
    }

    #[test]
    fn test_footer_spacing() {
        let mut t = make_table();
        t.param_update(TableParam::FooterSpacing, &py_obj(2.0f32));
        assert_eq!(t.footer_spacing, Some(2.0));
    }

    #[test]
    fn test_custom_header_rows() {
        let mut t = make_table();
        t.param_update(TableParam::CustomHeaderRows, &py_obj(3usize));
        assert_eq!(t.custom_header_rows, Some(3));
        t.param_update(TableParam::CustomHeaderRows, &py_none());
        assert_eq!(t.custom_header_rows, None);
    }

    #[test]
    fn test_custom_footer_rows() {
        let mut t = make_table();
        t.param_update(TableParam::CustomFooterRows, &py_obj(2usize));
        assert_eq!(t.custom_footer_rows, Some(2));
    }

    #[test]
    fn test_column_proportional_resize() {
        let mut t = make_table();
        t.param_update(TableParam::ColumnProportionalResize, &py_obj(true));
        assert!(t.column_proportional_resize);
    }

    #[test]
    fn test_row_spacing() {
        let mut t = make_table();
        t.param_update(TableParam::RowSpacing, &py_obj(3.0f32));
        assert_eq!(t.row_spacing, Some(3.0));
    }

    #[test]
    fn test_row_height() {
        let mut t = make_table();
        t.param_update(TableParam::RowHeight, &py_obj(20.0f32));
        assert_eq!(t.row_height, Some(20.0));
    }

    #[test]
    fn test_header_body_spacing() {
        let mut t = make_table();
        t.param_update(TableParam::HeaderBodySpacing, &py_obj(5.0f32));
        assert_eq!(t.header_body_spacing, Some(5.0));
    }

    #[test]
    fn test_body_footer_spacing() {
        let mut t = make_table();
        t.param_update(TableParam::BodyFooterSpacing, &py_obj(5.0f32));
        assert_eq!(t.body_footer_spacing, Some(5.0));
    }

    #[test]
    fn test_resize_columns_enabled() {
        let mut t = make_table();
        t.param_update(TableParam::ResizeColumnsEnabled, &py_obj(true));
        assert!(t.resize_columns_enabled);
    }

    #[test]
    fn test_min_column_width() {
        let mut t = make_table();
        t.param_update(TableParam::MinColumnWidth, &py_obj(50.0f32));
        assert_eq!(t.min_column_width, Some(50.0));
    }

    #[test]
    fn test_text_size() {
        let mut t = make_table();
        t.param_update(TableParam::TextSize, &py_obj(14.0f32));
        assert_eq!(t.text_size, Some(14.0));
    }

    #[test]
    fn test_show() {
        let mut t = make_table();
        t.param_update(TableParam::Show, &py_obj(false));
        assert!(!t.show);
    }

    #[test]
    fn test_table_width_fixed() {
        let mut t = make_table();
        t.param_update(TableParam::TableWidthFixed, &py_obj(true));
        assert!(t.table_width_fixed);
    }

    #[test]
    fn test_style_id() {
        let mut t = make_table();
        t.param_update(TableParam::StyleId, &py_obj(5usize));
        assert_eq!(t.style_id, Some(5));
        t.param_update(TableParam::StyleId, &py_none());
        assert_eq!(t.style_id, None);
    }

    #[test]
    fn test_scrollable_style_id() {
        let mut t = make_table();
        t.param_update(TableParam::ScrollableStyleId, &py_obj(7usize));
        assert_eq!(t.scrollable_style_id, Some(7));
    }
}
