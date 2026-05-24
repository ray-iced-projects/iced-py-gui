//! ipg_table
#![allow(clippy::unit_arg)]
#![allow(unused)]
use std::collections::HashMap;
use std::num::NonZeroU16;

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
    .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
    .sync_sashes(sizes.clone())
    .style(|theme, status| iced_sash::subtle(theme, status))
    .clip(true)
    .into();

    let rl = container(rule::horizontal(6.0)).width(Length::Fixed(sizes.iter().sum()));
    
    container(column![sash, rl])
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
            .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
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

/// The three sections of content passed to `Table::construct`.
pub struct TableSections<'a> {
    pub header: Vec<Element<'a, Message, Theme, Renderer>>,
    pub body:   Vec<Element<'a, Message, Theme, Renderer>>,
    pub footer: Vec<Element<'a, Message, Theme, Renderer>>,
}

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub id: usize,
    pub col_widths: Vec<f32>,
    pub row_height: f32,
    pub sash_size: f32,
    pub min_size: f32,
    pub file_path: Option<String>,
    pub style_id: Option<usize>,
    pub sash_style_id: Option<usize>,
    pub show: bool,
}

impl Table {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        sections: TableSections<'a>,
        widgets: &'a HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {

        if !self.show { return None }

        let id = self.id;
        let sizes = self.col_widths.clone();
        let height = self.row_height;
        let sash_size = self.sash_size;
        let min_size = self.min_size;

        let mut parts: Vec<Element<'a, Message, Theme, Renderer>> = vec![];

        if !sections.header.is_empty() {
            parts.push(adv_header(id, sections.header, sizes.clone(), height, sash_size, min_size));
        }
        parts.push(container(rule::horizontal(6.0))
                    .width(Length::Fixed(sizes.iter().sum()
                    )).into());

        if !sections.body.is_empty() {
            parts.push(adv_body(id, sections.body, sizes.clone(), height, sash_size, min_size));
        }
        if !sections.footer.is_empty() {
            parts.push(adv_footer(id, sections.footer, sizes.clone(), height, sash_size, min_size));
        }

        Some(container(column(parts))
            .style(|theme| container::rounded_box(theme))
            .into())
    }

}

/// Renders the header: cells are chunked by col count, each chunk becomes a SashH row.
/// Adding 2×n_cols cells gives two header rows, etc.
fn adv_header<'a>(
    id: usize,
    content: Vec<Element<'a, Message, Theme, Renderer>>,
    sizes: Vec<f32>,
    height: f32,
    sash_size: f32,
    min_size: f32,
) -> Element<'a, Message, Theme, Renderer> {
    let total_width: f32 = sizes.iter().sum();
    let n_cols = sizes.len().max(1);
    let mut iter = content.into_iter();
    let mut header_rows: Vec<Element<'a, Message, Theme, Renderer>> = vec![];
    loop {
        let cells: Vec<Element<'a, Message, Theme, Renderer>> = iter.by_ref().take(n_cols).collect();
        if cells.is_empty() { break; }
        let sash: Element<'a, Message, Theme, Renderer> = SashH::new(cells, sizes.clone(), height, sash_size)
            .min_size(min_size)
            .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
            .sync_sashes(sizes.clone())
            .style(|theme, status| iced_sash::subtle(theme, status))
            .clip(true)
            .into();
        header_rows.push(sash);
    }
    header_rows.push(container(rule::horizontal(1)).width(Length::Fixed(total_width)).into());
    container(column(header_rows))
    .style(|theme| container::rounded_box(theme))
    .into()
}

/// Renders the body: flat cells chunked by col count, each chunk becomes a SashH row.
/// The Python side must add cells flat (no Row wrapper) inside TableBody.
fn adv_body<'a>(
    id: usize,
    content: Vec<Element<'a, Message, Theme, Renderer>>,
    sizes: Vec<f32>,
    height: f32,
    sash_size: f32,
    min_size: f32,
) -> Element<'a, Message, Theme, Renderer> {
    let n_cols = sizes.len().max(1);
    let mut iter = content.into_iter();
    let mut rows: Vec<Element<'a, Message, Theme, Renderer>> = vec![];
    let mut row_idx = 0usize;
    loop {
        let cells: Vec<Element<'a, Message, Theme, Renderer>> = iter.by_ref().take(n_cols).collect();
        if cells.is_empty() { break; }
        let sash: Element<'a, Message, Theme, Renderer> = SashH::new(cells, sizes.clone(), height, sash_size)
            .min_size(min_size)
            .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
            .sync_sashes(sizes.clone())
            .style(|theme, status| iced_sash::subtle(theme, status))
            .clip(true)
            .into();
        if row_idx % 2 == 1 {
            rows.push(container(sash)
                .style(|theme: &Theme| {
                    let base = theme.palette().background.base.color;
                    let weak = theme.palette().background.weak.color;
                    let mid = iced::Color {
                        r: (base.r + weak.r) / 2.0,
                        g: (base.g + weak.g) / 2.0,
                        b: (base.b + weak.b) / 2.0,
                        a: 1.0,
                    };
                    container::Style { background: Some(mid.into()), ..Default::default() }
                })
                .into());
        } else {
            rows.push(sash);
        }
        row_idx += 1;
    }
    scrollable(column(rows)).into()
}

/// Renders the footer as a single SashH row with a dividing rule above.
fn adv_footer<'a>(
    id: usize,
    content: Vec<Element<'a, Message, Theme, Renderer>>,
    sizes: Vec<f32>,
    height: f32,
    sash_size: f32,
    min_size: f32,
) -> Element<'a, Message, Theme, Renderer> {
    let total_width: f32 = sizes.iter().sum();
    let sash: Element<'a, Message, Theme, Renderer> = SashH::new(content, sizes.clone(), height, sash_size)
        .min_size(min_size)
        .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
        .sync_sashes(sizes)
        .style(|theme, status| iced_sash::subtle(theme, status))
        .clip(true)
        .into();
    container(column![
        container(rule::horizontal(1)).width(Length::Fixed(total_width)),
        sash,
    ])
    .style(|theme| container::rounded_box(theme))
    .into()
}

#[derive(Debug, Clone, Default)]
pub struct TableHeader {
    pub id: usize,
    pub style_id: Option<usize>,
    pub sash_style_id: Option<usize>,
    pub show: bool,
}


impl TableHeader {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message, Theme, Renderer>>,
        widgets: &'a HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {



        None
    }

}

#[derive(Debug, Clone, Default)]
pub struct TableBody {
    pub id: usize,
    pub style_id: Option<usize>,
    pub sash_style_id: Option<usize>,
    pub show: bool,
}

impl TableBody {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message, Theme, Renderer>>,
        widgets: &'a HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {



        None
    }

}

#[derive(Debug, Clone, Default)]
pub struct TableFooter {
    pub id: usize,
    pub style_id: Option<usize>,
    pub sash_style_id: Option<usize>,
    pub show: bool,
}

impl TableFooter {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message, Theme, Renderer>>,
        widgets: &'a HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {



        None
    }

}


#[derive(Clone, Debug, PartialEq)]
pub enum TableBasicMessage {
    ResizeH(Id, usize, f32)
}

pub fn table_callback(
        state: &mut IpgState,  
        widget_id: usize,  
        message: TableBasicMessage) 
{
    match message {
        TableBasicMessage::ResizeH(_id, idx, size) => {
            if let Some(Containers::TableBasic(t)) = state.containers.get_mut(&widget_id) {
                sash_resize(&mut t.col_widths, idx, size, t.min_size);
            } else if let Some(Containers::Table(t)) = state.containers.get_mut(&widget_id) {
                sash_resize(&mut t.col_widths, idx, size, t.min_size);
                return;
            } else {
                return;
            }
            let table = match state.containers.get(&widget_id) {
                Some(Containers::TableBasic(t)) => t,
                _ => return,
            };

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
pub enum TableBasicParam {
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
    type Param = TableBasicParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TableBasicParam::Headers => set_t_value(&mut self.headers, value, "TableParam::Headers"),
            TableBasicParam::Body => set_t_value(&mut self.body, value, "TableParam::Body"),
            TableBasicParam::Footers => set_t_value(&mut self.footers, value, "TableParam::Footers"),
            TableBasicParam::ColumnWidths => set_t_value(&mut self.column_widths, value, "TableParam::ColumnWidths"),
            TableBasicParam::Height => set_t_value(&mut self.row_height, value, "TableParam::Height"),
            TableBasicParam::Width => set_t_value(&mut self.col_widths, value, "TableParam::Width"),
            TableBasicParam::SashSize => set_t_value(&mut self.sash_size, value, "TableParam::SashSize"),
            TableBasicParam::HeaderEnabled => set_t_value(&mut self.header_enabled, value, "TableParam::HeaderEnabled"),
            TableBasicParam::HeaderHeight => set_t_value(&mut self.header_row_height, value, "TableParam::HeaderHeight"),
            TableBasicParam::HeaderRowSpacing => set_t_value(&mut self.header_row_spacing, value, "TableParam::HeaderRowSpacing"),
            TableBasicParam::FooterHeight => set_t_value(&mut self.footer_height, value, "TableParam::FooterHeight"),
            TableBasicParam::FooterSpacing => set_t_value(&mut self.footer_spacing, value, "TableParam::FooterSpacing"),
            TableBasicParam::RowSpacing => set_t_value(&mut self.row_spacing, value, "TableParam::RowSpacing"),
            TableBasicParam::RowHeight => set_t_value(&mut self.row_height, value, "TableParam::RowHeight"),
            TableBasicParam::ResizeColumnsEnabled => set_t_value(&mut self.resize_columns_enabled, value, "TableParam::ResizeColumnsEnabled"),
            TableBasicParam::MinSize => set_t_value(&mut self.min_size, value, "TableParam::MinSize"),
            TableBasicParam::TextSize => set_t_value(&mut self.text_size, value, "TableParam::TextSize"),
            TableBasicParam::Show => set_t_value(&mut self.show, value, "TableParam::Show"),
            TableBasicParam::StyleId => set_t_value(&mut self.style_id, value, "TableParam::StyleId"),
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
