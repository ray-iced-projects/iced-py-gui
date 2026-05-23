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

use crate::IpgState;

use iced::border::Radius;
use iced::widget::{scrollable::Scrollbar};
use iced::{Border, Length, alignment, widget};
use iced::Length::Fill;
use iced::{Element, Renderer, Theme};
use iced::widget::{Space, center, column, container, row, rule, scrollable, stack, text};

use iced_sash::{Id, SashH};
pub use iced_sash::resize as sash_resize;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone, Default)]
pub struct TableBasic {
        pub id: usize,
        pub row_height: f32,
        pub col_widths: Vec<f32>,
        pub scrollable_height: Option<f32>,
        pub file_path: Option<String>,
        pub headers: Option<Vec<String>>,
        pub body: Option<Vec<Vec<String>>>,
        pub footers: Option<Vec<String>>,
        pub column_widths: Option<Vec<f32>>,
        pub sash_size: Option<f32>,
        pub header_enabled: Option<bool>,
        pub header_row_height: Option<f32>,
        pub header_row_spacing: Option<f32>,
        pub footer_height: Option<f32>,
        pub footer_spacing: Option<f32>,
        pub body_row_highlight: bool,
        pub row_spacing: Option<f32>,
        pub resize_columns_enabled: bool,
        pub min_size: f32,
        pub text_size: Option<f32>,
        pub style_id: Option<usize>,
        pub sash_style_id: Option<usize>,
        pub released: bool,
        pub show: bool,
}

impl TableBasic {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message, Theme, Renderer>>,
        widgets: &'a HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {

        if !self.show { return None }
        
        let wid = self.id.clone();

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

        let (header, body) = if let Some(fp) = &self.file_path {
            let (header_data, body_data) =
                load_csv(fp).unwrap_or_default();

            let header = 
                table_header(wid, &header_data, self.col_widths.clone(), self.row_height);
            let body = 
                table_body(wid, &body_data, self.col_widths.clone(), self.row_height);
            (header, body)
        } else {
            let header = 
                table_header(wid, &self.headers.clone().unwrap_or(vec![]), self.col_widths.clone(), self.row_height);
            let body = 
                table_body(wid, &self.body.clone().unwrap_or(vec![vec![]]), self.col_widths.clone(), self.row_height);
            (header, body)
        };

        let body = if let Some(sh) = self.scrollable_height {
            scrollable(body).height(sh).into()
        } else { body };

        let table = column![header, body];

        Some(container(table)
            .style(move|theme| {
                container::bordered_box(theme)
            }).into())

    }
}

fn table_header<'a>(id: usize, header: &[String], sizes: Vec<f32>, height: f32) -> Element<'a, Message> {

    let sash: Element<'a, Message> = SashH::new(
        header.iter().map(|col| center(text(col.clone())).center(Fill).into()).collect(),
        sizes.clone(),
        height,
        6.0,
    )
    .min_size(10.0)
    .on_resize(move |s_id, idx, val| Message::Table(id, TableMessage::ResizeH(s_id, idx, val)))
    .sync_sashes(sizes.clone())
    .style(|theme, status| iced_sash::subtle(theme, status))
    .clip(true)
    .into();

    container(column![sash, container(rule::horizontal(6)).width(Length::Fixed(sizes.iter().sum()))])
        .style(|theme| container::rounded_box(theme))
        .into()
}

fn table_body<'a>(id: usize, body: &[Vec<String>], sizes: Vec<f32>, height: f32) -> Element<'a, Message> {
    
    let rows: Vec<Element<'a, Message>> = body
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let sash: Element<'a, Message> = SashH::new(
                row.iter().map(|cell| center(text(cell.clone()).size(14.0)).center(Fill).into()).collect(),
                sizes.clone(),
                height,
                6.0,
            )
            .min_size(10.0)
            .on_resize(move |s_id, idx, val| Message::Table(id, TableMessage::ResizeH(s_id, idx, val)))
            .sync_sashes(sizes.clone())
            .style(|theme, status| iced_sash::subtle(theme, status))
            .clip(true)
            .into();

            if i % 2 == 1 {
                container(sash)
                    .style(|theme: &Theme| {
                        let base = theme.palette().background.base.color;
                        let weak = theme.palette().background.weak.color;
                        let mid = iced::Color {
                            r: (base.r + weak.r) / 2.0,
                            g: (base.g + weak.g) / 2.0,
                            b: (base.b + weak.b) / 2.0,
                            a: 1.0,
                        };
                        container::Style {
                            background: Some(mid.into()),
                            ..Default::default()
                        }
                    })
                    .into()
            } else {
                sash
            }
        })
        .collect();
    
    column(rows).into()
}

fn load_csv(path: &str) -> Result<(Vec<String>, Vec<Vec<String>>), csv::Error> {
    
    let mut reader = csv::Reader::from_path(path)?;

    let header: Vec<String> = reader
        .headers()?
        .iter()
        .map(|s| s.to_string())
        .collect();

    let body: Vec<Vec<String>> = reader
        .records()
        .map(|r| r.map(|rec| rec.iter().map(|s| s.to_string()).collect()))
        .collect::<Result<_, _>>()?;

    Ok((header, body))
}

#[derive(Clone, Debug, PartialEq)]
pub enum TableMessage {
    ResizeH(Id, usize, f32)
}

pub fn table_callback(
        state: &mut IpgState,  
        widget_id: usize,  
        message: TableMessage) 
{
    match message {
        TableMessage::ResizeH(_id, idx, size) => {
            let table = match state.containers.get_mut(&widget_id) {
                Some(Containers::TableBasic(t)) => t,
                _ => return,
            };
            sash_resize(&mut table.col_widths, idx, size, table.min_size);

            // Fire Python callback if registered: def cb(wid: int, data: tuple[int, float])
            invoke_callback_with_args(widget_id, "on_resize", "Table-SashH", (idx, size),
                "def cb(wid: int, data: tuple[int, float])");
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
    RowSpacing,
    RowHeight,
    ResizeColumnsEnabled,
    MinSize,
    TextSize,
    Show,
    StyleId,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for TableBasic {
    type Param = TableParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TableParam::Headers => set_t_value(&mut self.headers, value, "TableParam::Headers"),
            TableParam::Body => set_t_value(&mut self.body, value, "TableParam::Body"),
            TableParam::Footers => set_t_value(&mut self.footers, value, "TableParam::Footers"),
            TableParam::ColumnWidths => set_t_value(&mut self.column_widths, value, "TableParam::ColumnWidths"),
            TableParam::Height => set_t_value(&mut self.row_height, value, "TableParam::Height"),
            TableParam::Width => set_t_value(&mut self.col_widths, value, "TableParam::Width"),
            TableParam::SashSize => set_t_value(&mut self.sash_size, value, "TableParam::SashSize"),
            TableParam::HeaderEnabled => set_t_value(&mut self.header_enabled, value, "TableParam::HeaderEnabled"),
            TableParam::HeaderHeight => set_t_value(&mut self.header_row_height, value, "TableParam::HeaderHeight"),
            TableParam::HeaderRowSpacing => set_t_value(&mut self.header_row_spacing, value, "TableParam::HeaderRowSpacing"),
            TableParam::FooterHeight => set_t_value(&mut self.footer_height, value, "TableParam::FooterHeight"),
            TableParam::FooterSpacing => set_t_value(&mut self.footer_spacing, value, "TableParam::FooterSpacing"),
            TableParam::RowSpacing => set_t_value(&mut self.row_spacing, value, "TableParam::RowSpacing"),
            TableParam::RowHeight => set_t_value(&mut self.row_height, value, "TableParam::RowHeight"),
            TableParam::ResizeColumnsEnabled => set_t_value(&mut self.resize_columns_enabled, value, "TableParam::ResizeColumnsEnabled"),
            TableParam::MinSize => set_t_value(&mut self.min_size, value, "TableParam::MinSize"),
            TableParam::TextSize => set_t_value(&mut self.text_size, value, "TableParam::TextSize"),
            TableParam::Show => set_t_value(&mut self.show, value, "TableParam::Show"),
            TableParam::StyleId => set_t_value(&mut self.style_id, value, "TableParam::StyleId"),
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
