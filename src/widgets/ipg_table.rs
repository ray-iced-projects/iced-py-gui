//! ipg_table
#![allow(clippy::unit_arg)]
#![allow(unused)]
use std::collections::HashMap;
use std::num::NonZeroU16;

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::ipg_widgets::ipg_sash;
use crate::py_api::helpers::get_radius;
use crate::state::{Containers, Widgets};
use crate::widgets::callbacks::{CallbackName, invoke_callback_with_args};
use crate::widgets::styling::apply_background_color_overrides;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};

use crate::IpgState;

use iced::Length::Fill;
use iced::border::Radius;
use iced::widget::scrollable::Scrollbar;
use iced::widget::{Space, center, column, container, row, rule, scrollable, stack, text};
use iced::{Border, Length, Widget, alignment, widget};
use iced::{Element, Renderer, Theme};

use ipg_sash::sash::resize as sash_resize;
use ipg_sash::sash::{Id, SashH};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone, Default)]
pub struct TableBasic {
    pub id: usize,
    pub body_row_heights: f32,
    pub body_row_highlight: bool,
    pub body_row_outline: bool,
    pub body: Vec<Vec<String>>,
    pub column_widths: Vec<f32>,
    pub file_path: Option<String>,
    pub footer_height: Option<f32>,
    pub footer: Option<Vec<String>>,
    pub header_height: Option<f32>,
    pub header: Option<Vec<String>>,
    pub min_size: f32,
    pub released: bool,
    pub resize_columns_enabled: bool,
    pub resize_table_enabled: bool,
    pub sash_size: f32,
    pub sash_style_id: Option<usize>,
    pub scrollable_height: Option<f32>,
    pub show: bool,
    pub style_id: Option<usize>,
    pub text_size: Option<f32>,
}

impl TableBasic {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &'a HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {
        if !self.show {
            return None;
        }

        let wid = self.id;

        let scroll_style_header_opt = self
            .lookup(widgets, self.style_id)
            .and_then(Widgets::as_scrollable_style)
            .cloned();

        let scroll_style_body_opt = self
            .lookup(widgets, self.style_id)
            .and_then(Widgets::as_scrollable_style)
            .cloned();

        let scroll_style_footer_opt = self
            .lookup(widgets, self.style_id)
            .and_then(Widgets::as_scrollable_style)
            .cloned();

        let table_style_opt = self
            .lookup(widgets, self.style_id)
            .and_then(Widgets::as_table_style)
            .cloned();

        let tbl_style = if let Some(tbl) = &table_style_opt {
            tbl.clone()
        } else {
            TableStyle::default()
        };

        let bkg_color =
            Color::rgba_ipg_color_to_iced(tbl_style.bkg_rgba, &tbl_style.bkg_color, tbl_style.bkg_color_alpha);

        let brd_color = Color::rgba_ipg_color_to_iced(
            tbl_style.border_rgba,
            &tbl_style.border_color,
            tbl_style.border_color_alpha,
        );

        let text_color =
            Color::rgba_ipg_color_to_iced(tbl_style.text_rgba, &tbl_style.text_color, tbl_style.text_color_alpha);

        let (header, body) = if let Some(fp) = &self.file_path {
            let (header_data, body_data) = load_csv(fp).unwrap_or_default();

            let header = 
                header_footer(
                    wid, 
                    &header_data, 
                    self.column_widths.clone(), 
                    self.header_height,
                    self.sash_size,
                    Some(self.min_size),
                );

            let body = table_body(
                wid,
                &body_data,
                self.column_widths.clone(),
                self.body_row_heights,
                self.body_row_highlight,
                self.body_row_outline,
                self.sash_size,
                Some(self.min_size),
            );

            (header, body)
        } else {
            let header = header_footer(
                wid,
                &self.header.clone().unwrap_or(vec![]),
                self.column_widths.clone(),
                self.header_height,
                self.sash_size,
                Some(self.min_size),
            );

            let body = table_body(
                wid,
                &self.body.clone(),
                self.column_widths.clone(),
                self.body_row_heights,
                self.body_row_highlight,
                self.body_row_outline,
                self.sash_size,
                Some(self.min_size),
            );

            (header, body)
        };

        let footer = header_footer(
            wid,
            &self.footer.clone().unwrap_or(vec![]),
            self.column_widths.clone(),
            self.footer_height,
            self.sash_size,
            Some(self.min_size),
        );

        let body = if let Some(sh) = self.scrollable_height {
            scrollable(body).height(sh).boxed()
        } else {
            body
        };

        let mut table_vec = vec![];
        if header.is_some() {
            table_vec.push(header.unwrap())
        }
        table_vec.push(body);
        if footer.is_some() {
            table_vec.push(footer.unwrap())
        }

        let table = column(table_vec);

        Some(
            container(table)
                .style(move |theme| container::bordered_box(theme))
                .boxed(),
        )
    }
}

fn header_footer<'a>(
    id: usize, hf: &[String], 
    sizes: Vec<f32>, 
    height: Option<f32>,
    sash_size: f32,
    min_size: Option<f32>,
) -> Option<Element<'a, Message>> {
    if hf.len() == 0 {
        return None;
    }
    let sash: Element<'a, Message> = SashH::new(
        hf.iter()
            .map(|col| center(text(col.clone())).center(Fill).boxed())
            .collect(),
        sizes.clone(),
        height.unwrap_or(30.0),
        sash_size,
    )
    .min_size(min_size.unwrap_or_default())
    .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
    .sync_sashes(sizes.clone())
    .style(ipg_sash::sash::subtle)
    .clip(true)
    .boxed();

    let rl = container(rule::horizontal(6.0)).width(Length::Fixed(sizes.iter().sum()));

    Some(container(column![sash, rl]).style(container::rounded_box).boxed())
}

fn table_body<'a>(
    id: usize,
    body: &[Vec<String>],
    sizes: Vec<f32>,
    height: f32,
    highlight: bool,
    outline: bool,
    sash_size: f32,
    min_size: Option<f32>,
) -> Element<'a, Message> {

    let rows: Vec<Element<'a, Message>> = body
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let sash: Element<'a, Message> = SashH::new(
                row.iter()
                    .map(|cell| center(text(cell.clone()).size(14.0)).center(Fill).boxed())
                    .collect(),
                sizes.clone(),
                height,
                sash_size,
            )
            .min_size(min_size.unwrap_or_default())
            .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
            .sync_sashes(sizes.clone())
            .style(move|theme, status|{
                ipg_sash::sash::subtle(theme, status)
            })
            .clip(true)
            .boxed();

            if i % 2 == 1 && highlight {
                container(sash)
                    .style(move |theme: &Theme| {
                        let base = theme.palette().background.base.color;
                        let weak = theme.palette().background.weak.color;
                        let mid = iced::Color {
                            r: (base.r + weak.r) / 2.0,
                            g: (base.g + weak.g) / 2.0,
                            b: (base.b + weak.b) / 2.0,
                            a: 1.0,
                        };
                        if outline {
                            container::Style {
                                background: Some(mid.into()),
                                ..Default::default()
                            }
                        } else {
                            let mut st = container::bordered_box(theme);
                            st.background = Some(mid.into());
                            st
                        }
                    })
                    .boxed()
            } else {
                if outline {
                    container(sash).style(container::bordered_box).boxed()
                } else {
                    sash
                }
                
            }
        })
        .collect();

    column(rows).boxed()
}

fn load_csv(path: &str) -> Result<(Vec<String>, Vec<Vec<String>>), csv::Error> {
    let mut reader = csv::Reader::from_path(path)?;

    let header: Vec<String> = reader.headers()?.iter().map(|s| s.to_string()).collect();

    let body: Vec<Vec<String>> = reader
        .records()
        .map(|r| r.map(|rec| rec.iter().map(|s| s.to_string()).collect()))
        .collect::<Result<_, _>>()?;

    Ok((header, body))
}

/// The three sections of content passed to `Table::construct`.
pub struct TableSections<'a> {
    pub header: Vec<Element<'a, Message, Theme, Renderer>>,
    pub body: Vec<Element<'a, Message, Theme, Renderer>>,
    pub footer: Vec<Element<'a, Message, Theme, Renderer>>,
}

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub id: usize,
    pub col_widths: Vec<f32>,
    pub row_height: f32,
    pub header_height: Option<f32>,
    pub footer_height: Option<f32>,
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
        if !self.show {
            return None;
        }

        let id = self.id;
        let sizes = self.col_widths.clone();
        let sash_size = self.sash_size;
        let min_size = self.min_size;

        let mut parts: Vec<Element<'a, Message, Theme, Renderer>> = vec![];

        let header_height = if let Some(hh) = self.header_height {
            hh
        } else {
            self.row_height
        };

        if !sections.header.is_empty() {
            parts.push(adv_header(
                id,
                sections.header,
                sizes.clone(),
                header_height,
                sash_size,
                min_size,
            ));
        }
        parts.push(
            container(rule::horizontal(6.0))
                .width(Length::Fixed(sizes.iter().sum()))
                .boxed(),
        );

        if !sections.body.is_empty() {
            parts.push(adv_body(
                id,
                sections.body,
                sizes.clone(),
                self.row_height,
                sash_size,
                min_size,
            ));
        }

        let footer_height = if let Some(fh) = self.footer_height {
            fh
        } else {
            self.row_height
        };

        if !sections.footer.is_empty() {
            parts.push(adv_footer(
                id,
                sections.footer,
                sizes.clone(),
                footer_height,
                sash_size,
                min_size,
            ));
        }

        Some(container(column(parts)).style(container::rounded_box).boxed())
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
        if cells.is_empty() {
            break;
        }
        let sash: Element<'a, Message, Theme, Renderer> =
            SashH::new(cells, sizes.clone(), height, sash_size)
                .min_size(min_size)
                .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
                .sync_sashes(sizes.clone())
                .style(ipg_sash::sash::subtle)
                .clip(true)
                .boxed();
        header_rows.push(sash);
    }
    header_rows.push(container(rule::horizontal(1)).width(Length::Fixed(total_width)).boxed());
    container(column(header_rows)).style(container::rounded_box).boxed()
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
        if cells.is_empty() {
            break;
        }
        let sash: Element<'a, Message, Theme, Renderer> =
            SashH::new(cells, sizes.clone(), height, sash_size)
                .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
                .min_size(min_size)
                .sync_sashes(sizes.clone())
                .style(ipg_sash::sash::subtle)
                .clip(true)
                .boxed();
        if row_idx % 2 == 1 {
            rows.push(
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
                    .boxed(),
            );
        } else {
            rows.push(sash);
        }
        row_idx += 1;
    }
    scrollable(column(rows)).boxed()
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
    let sash: Element<'a, Message, Theme, Renderer> =
        SashH::new(content, sizes.clone(), height, sash_size)
            .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
            .min_size(min_size)
            .sync_sashes(sizes)
            .style(ipg_sash::sash::subtle)
            .clip(true)
            .boxed();
    container(column![
        container(rule::horizontal(1)).width(Length::Fixed(total_width)),
        sash,
    ])
    .style(container::rounded_box)
    .boxed()
}

#[derive(Debug, Clone, Default)]
pub struct TableHeader {
    pub id: usize,
    pub height: Option<f32>,
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
    pub row_height: Option<f32>,
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
    ResizeH(Id, usize, f32),
}

pub fn table_callback(state: &mut IpgState, widget_id: usize, message: TableBasicMessage) {
    match message {
        TableBasicMessage::ResizeH(_id, idx, size) => {
            if let Some(Widgets::TableBasic(t)) = state.widgets.get_mut(&widget_id) {
                sash_resize(&mut t.column_widths, idx, size, t.min_size);
            } else if let Some(Containers::Table(t)) = state.containers.get_mut(&widget_id) {
                sash_resize(&mut t.col_widths, idx, size, t.min_size);
                return;
            } else {
                return;
            }
            let table = match state.widgets.get(&widget_id) {
                Some(Widgets::TableBasic(t)) => t,
                _ => return,
            };

            // Fire Python callback if registered: def cb(wid: int, data: tuple[int, float])
            invoke_callback_with_args(
                widget_id,
                CallbackName::OnResize,
                "Table-SashH",
                (idx, size),
                "def cb(wid: int, data: tuple[int, float])",
            );
        }
    }
}

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
    Body,
    BodyRowHeights,
    ColumnWidths,
    Footer,
    FooterHeight,
    Header,
    HeaderHeight,
    MinSize,
    ResizeColumnsEnabled,
    SashSize,
    Show,
    StyleId,
    TextSize,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for TableBasic {
    type Param = TableBasicParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TableBasicParam::Body => set_t_value(&mut self.body, value, "TableParam::Body"),
            TableBasicParam::BodyRowHeights => {
                set_t_value(&mut self.body_row_heights, value, "TableParam::BodyRowHeights")
            }
            TableBasicParam::ColumnWidths => set_t_value(&mut self.column_widths, value, "TableParam::ColumnWidths"),
            TableBasicParam::Footer => set_t_value(&mut self.footer, value, "TableParam::Footer"),
            TableBasicParam::FooterHeight => set_t_value(&mut self.footer_height, value, "TableParam::FooterHeight"),
            TableBasicParam::Header => set_t_value(&mut self.header, value, "TableParam::Header"),
            TableBasicParam::HeaderHeight => set_t_value(&mut self.header_height, value, "TableParam::HeaderHeight"),
            TableBasicParam::MinSize => set_t_value(&mut self.min_size, value, "TableParam::MinSize"),
            TableBasicParam::SashSize => set_t_value(&mut self.sash_size, value, "TableParam::SashSize"),
            TableBasicParam::Show => set_t_value(&mut self.show, value, "TableParam::Show"),
            TableBasicParam::StyleId => set_t_value(&mut self.style_id, value, "TableParam::StyleId"),
            TableBasicParam::ResizeColumnsEnabled => set_t_value(
                &mut self.resize_columns_enabled,
                value,
                "TableParam::ResizeColumnsEnabled",
            ),
            TableBasicParam::TextSize => set_t_value(&mut self.text_size, value, "TableParam::TextSize"),
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
