//! ipg_table
#![allow(clippy::unit_arg)]
use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::ipg_widgets::ipg_sash;
use crate::state::{Containers, Widgets};
use crate::widgets::callbacks::{CallbackName, invoke_callback_with_args};
use crate::widgets::ipg_rule::RuleStyle;
use crate::widgets::ipg_sash::{SashStyle, SashStyleStd};
use crate::widgets::ipg_text;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};

use crate::IpgState;

use iced::widget::{center, column, container, rule, scrollable};
use iced::{Element, Renderer, Theme};
use iced::{Length, Widget};

use ipg_sash::sash::resize as sash_resize;
use ipg_sash::sash::{Id, SashH};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone, Default)]
pub struct TableBasic {
    pub id: usize,
    pub body_row_heights: Option<f32>,
    pub body_row_highlight: bool,
    pub body_row_outline: bool,
    pub body: Vec<Vec<String>>,
    pub body_text_format_id: Option<usize>,
    pub body_text_style_id: Option<usize>,
    pub column_widths: Vec<f32>,
    pub footer_height: Option<f32>,
    pub footer: Option<Vec<String>>,
    pub footer_text_format_id: Option<usize>,
    pub footer_text_style_id: Option<usize>,
    pub header_height: Option<f32>,
    pub header: Option<Vec<String>>,
    pub header_text_format_id: Option<usize>,
    pub header_text_style_id: Option<usize>,
    pub header_body_divider_rule_height: f32,
    pub body_footer_divider_rule_height: f32,
    pub rule_style_id: Option<usize>,
    pub min_size: f32,
    pub resize_columns_enabled: bool,
    pub resize_table_enabled: bool,
    pub header_sash_size: f32,
    pub body_sash_size: f32,
    pub footer_sash_size: f32,
    pub header_sash_style_id: Option<usize>,
    pub body_sash_style_id: Option<usize>,
    pub footer_sash_style_id: Option<usize>,
    pub header_sash_style_std: Option<SashStyleStd>,
    pub body_sash_style_std: Option<SashStyleStd>,
    pub footer_sash_style_std: Option<SashStyleStd>,
    pub scrollable_height: Option<f32>,
    pub show: bool,
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

        let header_sash_style_opt = self
            .lookup(widgets, self.header_sash_style_id)
            .and_then(Widgets::as_sash_style)
            .cloned();

        let body_sash_style_opt = self
            .lookup(widgets, self.body_sash_style_id)
            .and_then(Widgets::as_sash_style)
            .cloned();

        let footer_sash_style_opt = self
            .lookup(widgets, self.footer_sash_style_id)
            .and_then(Widgets::as_sash_style)
            .cloned();

        let rule_style_opt = self
            .lookup(widgets, self.rule_style_id)
            .and_then(Widgets::as_rule_style)
            .cloned();

        let header = header_footer(
            wid,
            true,
            widgets,
            &self.header.clone().unwrap_or(vec![]),
            self.column_widths.clone(),
            self.header_height,
            self.header_sash_size,
            header_sash_style_opt,
            &self.header_sash_style_std,
            Some(self.min_size),
            self.header_text_style_id,
            self.header_text_format_id,
            self.header_body_divider_rule_height,
            rule_style_opt.clone(),
            if self.scrollable_height.is_some() { 10.0 } else { 0.0 },
        );

        let body = table_body(
            wid,
            &self.body.clone(),
            widgets,
            self.column_widths.clone(),
            self.body_row_heights.unwrap_or(30.0),
            self.body_row_highlight,
            self.body_row_outline,
            self.body_sash_size,
            body_sash_style_opt,
            &self.body_sash_style_std,
            Some(self.min_size),
            self.body_text_style_id,
            self.body_text_format_id,
        );

        let footer = header_footer(
            wid,
            false,
            widgets,
            &self.footer.clone().unwrap_or(vec![]),
            self.column_widths.clone(),
            self.footer_height,
            self.footer_sash_size,
            footer_sash_style_opt,
            &self.footer_sash_style_std,
            Some(self.min_size),
            self.footer_text_style_id,
            self.footer_text_format_id,
            self.body_footer_divider_rule_height,
            rule_style_opt,
            if self.scrollable_height.is_some() { 10.0 } else { 0.0 },
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
                .style(move |theme| {
                    if self.body_row_outline {
                        container::bordered_box(theme)
                    } else {
                        container::transparent(theme)
                    }
                })
                .boxed(),
        )
    }
}

fn header_footer<'a>(
    id: usize,
    header: bool,
    widgets: &HashMap<usize, Widgets>,
    hf: &[String],
    sizes: Vec<f32>,
    height: Option<f32>,
    sash_size: f32,
    sash_style_opt: Option<SashStyle>,
    sash_style_std: &'a Option<SashStyleStd>,
    min_size: Option<f32>,
    text_style_id: Option<usize>,
    text_format_id: Option<usize>,
    line_height: f32,
    rule_style_opt: Option<RuleStyle>,
    bar_width: f32,
) -> Option<Element<'a, Message>> {
    if hf.len() == 0 {
        return None;
    }

    let mut children = vec![];

    let mut txt = ipg_text::Text::default();

    txt.style_id = text_style_id;
    txt.format_id = text_format_id;
    txt.show = true;

    for (index, hf_text) in hf.iter().enumerate() {
        txt.content = hf_text.clone();
        let txt_element = txt.construct(widgets);
        if txt_element.is_none() {
            panic!("Unable to construct text for content {} with index {}", &hf_text, index)
        } else {
            children.push(txt_element.unwrap());
        }
    }

    let sash = SashH::new(children, sizes.clone(), height.unwrap_or(30.0), sash_size)
        .min_size(min_size.unwrap_or_default())
        .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
        .sync_sashes(sizes.clone())
        .style(move |theme, status| {
            if let Some(st) = sash_style_opt.clone() {
                st.to_iced(theme, status, sash_style_std)
            } else {
                ipg_sash::sash::subtle(theme, status)
            }
        })
        .clip(true)
        .boxed();

    let rl = rule::horizontal(line_height);
    let rl = if let Some(st) = rule_style_opt {
        rl.style(move |theme| st.to_iced(theme)).boxed()
    } else {
        rl.boxed()
    };

    let width: f32 = sizes.into_iter().sum();

    if header {
        Some(
            container(column![sash, rl])
                .width(width + bar_width)
                .style(container::rounded_box)
                .boxed(),
        )
    } else {
        Some(
            container(column![rl, sash])
                .width(width + bar_width)
                .style(container::rounded_box)
                .boxed(),
        )
    }
}

fn table_body<'a>(
    id: usize,
    body: &[Vec<String>],
    widgets: &HashMap<usize, Widgets>,
    sizes: Vec<f32>,
    height: f32,
    highlight: bool,
    outline: bool,
    sash_size: f32,
    sash_style_opt: Option<SashStyle>,
    sash_style_std: &'a Option<SashStyleStd>,
    min_size: Option<f32>,
    text_style_id: Option<usize>,
    text_format_id: Option<usize>,
) -> Element<'a, Message> {
    let mut txt = ipg_text::Text::default();

    txt.style_id = text_style_id;
    txt.format_id = text_format_id;
    txt.show = true;

    let rows: Vec<Element<'a, Message>> = body
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let sash: Element<'a, Message> = SashH::new(
                row.iter()
                    .map(|cell| {
                        txt.content = cell.clone();
                        let txt_element = txt.construct(widgets);
                        center(txt_element.unwrap().boxed()).boxed()
                    })
                    .collect(),
                sizes.clone(),
                height,
                sash_size,
            )
            .min_size(min_size.unwrap_or_default())
            .on_resize(move |s_id, idx, val| Message::Table(id, TableBasicMessage::ResizeH(s_id, idx, val)))
            .sync_sashes(sizes.clone())
            .style({
                let sash_opt = sash_style_opt.clone();
                move |theme, status| {
                    if let Some(st) = sash_opt.clone() {
                        st.to_iced(theme, status, &sash_style_std)
                    } else {
                        ipg_sash::sash::subtle(theme, status)
                    }
                }
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

// The three sections of content passed to `Table::construct`.
pub struct TableSections<'a> {
    pub header_content: HashMap<usize, Vec<Element<'a, Message>>>,
    pub header_child_ids: HashMap<usize, Vec<usize>>,
    pub header_params: Vec<(usize, &'a TableHeader)>,

    pub body_content: Vec<Element<'a, Message, Theme, Renderer>>,
    pub body_child_ids: Vec<usize>,
    pub body_params: Option<&'a TableBody>,

    pub footer_content: Vec<Element<'a, Message, Theme, Renderer>>,
    pub footer_child_ids: Vec<usize>,
    pub footer_params: Option<&'a TableFooter>,
}

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub id: usize,
    pub col_widths: Vec<f32>,
    pub sash_size: f32,
    pub min_size: f32,
    pub show: bool,
}

impl Table {
    // fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
    //     id.and_then(|id| widgets.get(&id))
    // }

    pub fn construct<'a>(
        &'a self,
        mut sections: TableSections<'a>,
        _widgets: &'a HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message, Theme, Renderer>> {
        if !self.show {
            return None;
        }

        let table_id = self.id;
        let sizes = self.col_widths.clone();
        let sash_size = self.sash_size;
        let min_size = self.min_size;

        let mut parts: Vec<Element<'a, Message, Theme, Renderer>> = vec![];

        for (id, th) in sections.header_params {
            if let Some(content) = sections.header_content.remove(&id) {
                let header_element = th.construct(table_id, content, &sizes, min_size, self.sash_size);
                parts.push(header_element);
            }
        }

        parts.push(
            container(rule::horizontal(6.0))
                .width(Length::Fixed(sizes.iter().sum()))
                .boxed(),
        );

        if !sections.body_content.is_empty() {
            parts.push(adv_body(
                table_id,
                sections.body_content,
                sizes.clone(),
                30.0,
                sash_size,
                min_size,
            ));
        }

        if !sections.footer_content.is_empty() {
            parts.push(adv_footer(
                table_id,
                sections.footer_content,
                sizes.clone(),
                30.0,
                sash_size,
                min_size,
            ));
        }

        Some(container(column(parts)).style(container::rounded_box).boxed())
    }
}

// Renders the body: flat cells chunked by col count, each chunk becomes a SashH row.
// The Python side must add cells flat (no Row wrapper) inside TableBody.
fn adv_body<'a>(
    table_id: usize,
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
        let sash: Element<'a, Message, Theme, Renderer> = SashH::new(cells, sizes.clone(), height, sash_size)
            .on_resize(move |s_id, idx, val| Message::Table(table_id, TableBasicMessage::ResizeH(s_id, idx, val)))
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

// Renders the footer as a single SashH row with a dividing rule above.
fn adv_footer<'a>(
    table_id: usize,
    content: Vec<Element<'a, Message, Theme, Renderer>>,
    sizes: Vec<f32>,
    height: f32,
    sash_size: f32,
    min_size: f32,
) -> Element<'a, Message, Theme, Renderer> {
    let total_width: f32 = sizes.iter().sum();
    let sash: Element<'a, Message, Theme, Renderer> = SashH::new(content, sizes.clone(), height, sash_size)
        .on_resize(move |s_id, idx, val| Message::Table(table_id, TableBasicMessage::ResizeH(s_id, idx, val)))
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
    pub sash_style_std: Option<SashStyleStd>,
    pub text_format_id: Option<usize>,
}

// Renders the header: cells are chunked by col count, each chunk becomes a SashH row.
// Adding 2×n_cols cells gives two header rows, etc.
impl TableHeader {
    fn construct<'a>(
        &self,
        table_id: usize,
        content: Vec<Element<'a, Message, Theme, Renderer>>,
        sizes: &Vec<f32>,
        min_size: f32,
        sash_size: f32,
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
                SashH::new(cells, sizes.clone(), self.height.unwrap_or(30.0), sash_size)
                    .min_size(min_size)
                    .on_resize(move |s_id, idx, val| {
                        Message::Table(table_id, TableBasicMessage::ResizeH(s_id, idx, val))
                    })
                    .sync_sashes(sizes.clone())
                    .style(ipg_sash::sash::subtle)
                    .clip(true)
                    .boxed();
            header_rows.push(sash);
        }
        header_rows.push(container(rule::horizontal(1)).width(Length::Fixed(total_width)).boxed());
        container(column(header_rows)).style(container::rounded_box).boxed()
    }
}
#[derive(Debug, Clone, Default)]
pub struct TableBody {
    pub id: usize,
    pub row_height: Option<f32>,
    pub style_id: Option<usize>,
    pub sash_style_id: Option<usize>,
    pub sash_style_std: Option<SashStyleStd>,
    pub text_format_id: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct TableFooter {
    pub id: usize,
    pub style_id: Option<usize>,
    pub sash_style_id: Option<usize>,
    pub show: bool,
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
            match state.widgets.get(&widget_id) {
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
    BodyFooterDividerRuleHeight,
    BodyRowHeights,
    BodyRowHighlight,
    BodyRowOutline,
    BodySashSize,
    BodySashStyleId,
    BodySashStyleStd,
    BodyTextFormatId,
    BodyTextStyleId,
    ColumnWidths,
    Footer,
    FooterHeight,
    FooterSashSize,
    FooterSashStyleId,
    FooterSashStyleStd,
    FooterTextFormatId,
    FooterTextStyleId,
    Header,
    HeaderBodyDividerRuleHeight,
    HeaderHeight,
    HeaderSashSize,
    HeaderSashStyleId,
    HeaderSashStyleStd,
    HeaderTextFormatId,
    HeaderTextStyleId,
    MinSize,
    ResizeColumnsEnabled,
    ResizeTableEnabled,
    RuleStyleId,
    ScrollableHeight,
    Show,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for TableBasic {
    type Param = TableBasicParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TableBasicParam::Body => set_t_value(&mut self.body, value, "TableParam::Body"),
            TableBasicParam::BodyFooterDividerRuleHeight => set_t_value(
                &mut self.body_footer_divider_rule_height,
                value,
                "TableBasicParam::BodyFooterDividerRuleHeight",
            ),
            TableBasicParam::BodyRowHeights => {
                set_t_value(&mut self.body_row_heights, value, "TableParam::BodyRowHeights")
            }
            TableBasicParam::BodyRowHighlight => {
                set_t_value(&mut self.body_row_highlight, value, "TableBasicParam::BodyRowHighlight")
            }
            TableBasicParam::BodyRowOutline => {
                set_t_value(&mut self.body_row_outline, value, "TableBasicParam::BodyRowOutline")
            }
            TableBasicParam::BodySashSize => {
                set_t_value(&mut self.body_sash_size, value, "TableBasicParam::BodySashSize")
            }
            TableBasicParam::BodySashStyleId => {
                set_t_value(&mut self.body_sash_style_id, value, "TableBasicParam::BodySashStyleId")
            }
            TableBasicParam::BodySashStyleStd => set_t_value(
                &mut self.body_sash_style_std,
                value,
                "TableBasicParam::BodySashStyleStd",
            ),
            TableBasicParam::BodyTextFormatId => set_t_value(
                &mut self.body_text_format_id,
                value,
                "TableBasicParam::BodyTextFormatId",
            ),
            TableBasicParam::BodyTextStyleId => {
                set_t_value(&mut self.body_text_style_id, value, "TableBasicParam::BodyTextStyleId")
            }
            TableBasicParam::ColumnWidths => set_t_value(&mut self.column_widths, value, "TableParam::ColumnWidths"),
            TableBasicParam::Footer => set_t_value(&mut self.footer, value, "TableParam::Footer"),
            TableBasicParam::FooterHeight => set_t_value(&mut self.footer_height, value, "TableParam::FooterHeight"),
            TableBasicParam::FooterSashSize => {
                set_t_value(&mut self.footer_sash_size, value, "TableBasicParam::FooterSashSize")
            }
            TableBasicParam::FooterSashStyleId => set_t_value(
                &mut self.footer_sash_style_id,
                value,
                "TableBasicParam::FooterSashStyleId",
            ),
            TableBasicParam::FooterSashStyleStd => set_t_value(
                &mut self.footer_sash_style_std,
                value,
                "TableBasicParam::FooterSashStyleStd",
            ),
            TableBasicParam::FooterTextFormatId => set_t_value(
                &mut self.footer_text_format_id,
                value,
                "TableBasicParam::FooterTextFormatId",
            ),
            TableBasicParam::FooterTextStyleId => set_t_value(
                &mut self.footer_text_style_id,
                value,
                "TableBasicParam::FooterTextStyleId",
            ),
            TableBasicParam::Header => set_t_value(&mut self.header, value, "TableParam::Header"),
            TableBasicParam::HeaderBodyDividerRuleHeight => set_t_value(
                &mut self.header_body_divider_rule_height,
                value,
                "TableBasicParam::HeaderBodyDividerRuleHeight",
            ),
            TableBasicParam::HeaderHeight => set_t_value(&mut self.header_height, value, "TableParam::HeaderHeight"),
            TableBasicParam::HeaderSashSize => {
                set_t_value(&mut self.header_sash_size, value, "TableBasicParam::HeaderSashSize")
            }
            TableBasicParam::HeaderSashStyleId => set_t_value(
                &mut self.header_sash_style_id,
                value,
                "TableBasicParam::HeaderSashStyleId",
            ),
            TableBasicParam::HeaderSashStyleStd => set_t_value(
                &mut self.header_sash_style_std,
                value,
                "TableBasicParam::HeaderSashStyleStd",
            ),
            TableBasicParam::HeaderTextFormatId => set_t_value(
                &mut self.header_text_format_id,
                value,
                "TableBasicParam::HeaderTextFormatId",
            ),
            TableBasicParam::HeaderTextStyleId => set_t_value(
                &mut self.header_text_style_id,
                value,
                "TableBasicParam::HeaderTextStyleId",
            ),
            TableBasicParam::MinSize => set_t_value(&mut self.min_size, value, "TableParam::MinSize"),
            TableBasicParam::ResizeColumnsEnabled => set_t_value(
                &mut self.resize_columns_enabled,
                value,
                "TableBasicParam::ResizeColumnsEnabled",
            ),
            TableBasicParam::ResizeTableEnabled => set_t_value(
                &mut self.resize_table_enabled,
                value,
                "TableBasicParam::ResizeTableEnabled",
            ),
            TableBasicParam::RuleStyleId => set_t_value(&mut self.rule_style_id, value, "TableBasicParam::RuleStyleId"),
            TableBasicParam::ScrollableHeight => {
                set_t_value(&mut self.scrollable_height, value, "TableBasicParam::ScrollableHeight")
            }
            TableBasicParam::Show => set_t_value(&mut self.show, value, "TableParam::Show"),
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
