//! ipg_table
#![allow(clippy::unit_arg)]
#![allow(unused)]
use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::helpers::get_radius;
use crate::state::{Containers, Widgets};
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::styling::apply_background_color_overrides;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value
};
use crate::ipg_widgets::ipg_sash::sash::{self, sash_horizontal};
use crate::IpgState;

use iced::border::Radius;
use iced::widget::{scrollable::Scrollbar};
use iced::{Border, Length, alignment, widget};
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
        pub width: Option<f32>,
        pub sash_size: f32,
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
        pub row_spacing: Option<f32>,
        pub row_height: Option<f32>,
        pub header_body_spacing: Option<f32>,
        pub body_footer_spacing: Option<f32>,
        pub resize_columns_enabled: bool,
        pub min_size: f32,
        pub text_size: Option<f32>,
        pub style_id: Option<usize>,
        pub sash_style_id: Option<usize>,
        pub scrollable_style_id: Option<usize>,
        pub released: bool,
        pub show: bool,
}

impl Table {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message, Theme, Renderer>>,
        widgets: &'a HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {

        if !self.show { return None }
        
        let scroll_style_header_opt  = self.lookup(widgets, self.style_id)
            .and_then(Widgets::as_scrollable_style).cloned();

        let scroll_style_body_opt  = self.lookup(widgets, self.style_id)
            .and_then(Widgets::as_scrollable_style).cloned();

        let scroll_style_footer_opt  = self.lookup(widgets, self.style_id)
            .and_then(Widgets::as_scrollable_style).cloned();
         
        let table_style_opt = self.lookup(widgets, self.style_id)
            .and_then(Widgets::as_table_style).cloned();

        let tbl_style = if let Some(tbl) = &table_style_opt {
            tbl.clone()
        } else { TableStyle::default() };

        let bkg_color = 
            Color::rgba_ipg_color_to_iced(tbl_style.bkg_rgba, &tbl_style.bkg_color, tbl_style.bkg_color_alpha);
        
        let brd_color = 
            Color::rgba_ipg_color_to_iced(tbl_style.border_rgba, &tbl_style.border_color, tbl_style.border_color_alpha);

        let text_color = 
            Color::rgba_ipg_color_to_iced(tbl_style.text_rgba, &tbl_style.text_color, tbl_style.text_color_alpha);

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
                                .clip(true)
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
                        if let Some(style) = &scroll_style_body_opt {
                            style.set_style(theme, status, widgets)
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
                            .clip(true)
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
                                if let Some(style) = &scroll_style_header_opt{
                                    style.set_style(theme, status, widgets)
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
                                if let Some(style) = &scroll_style_footer_opt {
                                    style.set_style(theme, status, widgets)
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

            let has_header = header.is_some();
            let mut main_col = vec![];

            if let Some(hdr) = header {
                if self.resize_columns_enabled {
                    let sash_height = header_height
                        + self.custom_header_rows.unwrap_or_default() as f32
                            * self.header_row_height.unwrap_or(20.0);
                    let sash_el = sash_horizontal(
                        self.id,
                        self.column_widths.clone(),
                        self.sash_size,
                        sash_height,
                        |(id, index, value)| Message::TableDividerChanged((id, index, value)),
                    )
                    .include_last_handle(false)
                    .on_release_fn(|(id, _)| Message::TableDividerReleased(id))
                    
                    ;
                    main_col.push(stack([hdr, sash_el.into()]).into());
                } else {
                    main_col.push(hdr);
                }
                main_col.push(Space::new().width(5.0).height(self.header_body_spacing.unwrap_or(5.0)).into());
            }

            // When there is no header and resize is enabled, fall back to sash on the body
            if !has_header && self.resize_columns_enabled {
                let sash_el = sash_horizontal(
                    self.id,
                    self.column_widths.clone(),
                    self.sash_size,
                    self.height,
                    |(id, index, value)| Message::TableDividerChanged((id, index, value)),
                )
                .include_last_handle(false)
                .on_release_fn(|(id, _)| Message::TableDividerReleased(id));
                main_col.push(stack([body.into(), sash_el.into()]).into());
            } else {
                main_col.push(body.into());
            }

            if let Some(ft) = footer {
                main_col.push(Space::new().width(5.0).height(self.body_footer_spacing.unwrap_or(5.0)).into());
                main_col.push(ft);
            }

            Some(column(main_col).into())

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

            if let Some(Containers::Table(tbl)) = state.containers.get_mut(&id) {
                let min = tbl.min_size;
                let len = tbl.column_widths.len();
                if index + 1 < len {
                    let diff = tbl.column_widths[index] - value;
                    let next_ideal = tbl.column_widths[index + 1] + diff;
                    let next_actual = next_ideal.max(min);
                    let excess = (next_actual - next_ideal).max(0.0);
                    tbl.column_widths[index] = (value - excess).max(min);
                    tbl.column_widths[index + 1] = next_actual;
                } else {
                    tbl.column_widths[index] = value.max(min);
                }
                invoke_callback_with_args(id, "dragging", "Table", (index, tbl.column_widths[index]),
                    "def cb(wid: int, data: tuple[int, float])");
            }
        },
        TableMessage::DivOnRelease=> {
            invoke_callback_with_args(id, "released", "Table", (),
                "def cb(wid: int)");
        },
    }
}


const ROW_COLOR: iced::Color = iced::Color::from_rgba(0.04, 0.35, 0.35, 0.2);
const ROW_CONTRAST_COLOR: iced::Color = iced::Color::from_rgba(0.25, 0.63, 0.67, 1.0);

#[derive(Debug, Clone, Default)]
pub struct TableStyle {
    pub id: usize,
    pub bkg_color: Option<Color>,
    pub bkg_color_alpha: Option<f32>,
    pub bkg_rgba: Option<[f32; 4]>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub text_color: Option<Color>,
    pub text_color_alpha: Option<f32>,
    pub text_rgba: Option<[f32; 4]>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TableStyleParam {
    BkgColor,
    BkgColorAlpha,
    BkgRgba,
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
    BorderRadius,
    BorderWidth,
    TextColor,
    TextColorAlpha,
    TextRgba,
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
    SashSize,
    HeaderEnabled,
    HeaderHeight,
    HeaderRowSpacing,
    FooterHeight,
    FooterSpacing,
    CustomHeaderRows,
    CustomFooterRows,
    ControlColumns,
    RowSpacing,
    RowHeight,
    HeaderBodySpacing,
    BodyFooterSpacing,
    ResizeColumnsEnabled,
    MinSize,
    TextSize,
    Show,
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
            TableParam::Headers => set_t_value(&mut self.headers, value, "TableParam::Headers"),
            TableParam::Body => set_t_value(&mut self.body, value, "TableParam::Body"),
            TableParam::Footers => set_t_value(&mut self.footers, value, "TableParam::Footers"),
            TableParam::ColumnWidths => set_t_value(&mut self.column_widths, value, "TableParam::ColumnWidths"),
            TableParam::Height => set_t_value(&mut self.height, value, "TableParam::Height"),
            TableParam::Width => set_t_value(&mut self.width, value, "TableParam::Width"),
            TableParam::SashSize => set_t_value(&mut self.sash_size, value, "TableParam::SashSize"),
            TableParam::HeaderEnabled => set_t_value(&mut self.header_enabled, value, "TableParam::HeaderEnabled"),
            TableParam::HeaderHeight => set_t_value(&mut self.header_row_height, value, "TableParam::HeaderHeight"),
            TableParam::HeaderRowSpacing => set_t_value(&mut self.header_row_spacing, value, "TableParam::HeaderRowSpacing"),
            TableParam::FooterHeight => set_t_value(&mut self.footer_height, value, "TableParam::FooterHeight"),
            TableParam::FooterSpacing => set_t_value(&mut self.footer_spacing, value, "TableParam::FooterSpacing"),
            TableParam::CustomHeaderRows => set_t_value(&mut self.custom_header_rows, value, "TableParam::CustomHeaderRows"),
            TableParam::CustomFooterRows => set_t_value(&mut self.custom_footer_rows, value, "TableParam::CustomFooterRows"),
            TableParam::ControlColumns => set_t_value(&mut self.control_columns, value, "TableParam::ControlColumns"),
            TableParam::RowSpacing => set_t_value(&mut self.row_spacing, value, "TableParam::RowSpacing"),
            TableParam::RowHeight => set_t_value(&mut self.row_height, value, "TableParam::RowHeight"),
            TableParam::HeaderBodySpacing => set_t_value(&mut self.header_body_spacing, value, "TableParam::HeaderBodySpacing"),
            TableParam::BodyFooterSpacing => set_t_value(&mut self.body_footer_spacing, value, "TableParam::BodyFooterSpacing"),
            TableParam::ResizeColumnsEnabled => set_t_value(&mut self.resize_columns_enabled, value, "TableParam::ResizeColumnsEnabled"),
            TableParam::MinSize => set_t_value(&mut self.min_size, value, "TableParam::MinSize"),
            TableParam::TextSize => set_t_value(&mut self.text_size, value, "TableParam::TextSize"),
            TableParam::Show => set_t_value(&mut self.show, value, "TableParam::Show"),
            TableParam::StyleId => set_t_value(&mut self.style_id, value, "TableParam::StyleId"),
            TableParam::ScrollableStyleId => set_t_value(&mut self.scrollable_style_id, value, "TableParam::ScrollableStyleId"),
        }
    }
}


impl WidgetParamUpdate for TableStyle {
    type Param = TableStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TableStyleParam::BkgColor => set_t_value(&mut self.bkg_color, value, "name"),
            TableStyleParam::BkgColorAlpha => set_t_value(&mut self.bkg_color_alpha, value, "name"),
            TableStyleParam::BkgRgba => set_t_value(&mut self.bkg_rgba, value, "name"),
            TableStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "name"),
            TableStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "name"),
            TableStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "name"),
            TableStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "name"),
            TableStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "name"),
            TableStyleParam::TextColor => set_t_value(&mut self.text_color, value, "name"),
            TableStyleParam::TextColorAlpha => set_t_value(&mut self.text_color_alpha, value, "name"),
            TableStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "name"),
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
            sash_size: 4.0,
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
            row_spacing: None,
            row_height: None,
            header_body_spacing: None,
            body_footer_spacing: None,
            resize_columns_enabled: false,
            min_size: 0.0,
            text_size: None,
            show: true,
            style_id: None,
            sash_style_id: None,
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
    fn test_sash_size() {
        let mut t = make_table();
        t.param_update(TableParam::SashSize, &py_obj(6.0f32));
        assert_eq!(t.sash_size, 6.0);
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
    fn test_min_size() {
        let mut t = make_table();
        t.param_update(TableParam::MinSize, &py_obj(50.0f32));
        assert_eq!(t.min_size, 50.0);
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
