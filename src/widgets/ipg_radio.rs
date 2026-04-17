//!ipg_radio

use std::collections::HashMap;

use crate::graphics::colors::Color;
use crate::py_api::helpers::{get_len, get_padding};
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};

use crate::IpgState;
use crate::app;
use crate::state::Widgets;

use iced::widget::radio::{self, Status};
use iced::widget::text::Wrapping;
use iced::{Element, Theme};
use iced::widget::{self, Column, Row};

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Radio {
    pub id: usize,
    pub labels: Vec<String>,
    pub direction: RadioDirection,
    pub spacing: Option<f32>,
    pub radio_spacing: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub selected_index: Option<usize>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub size: Option<f32>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub text_wrapping_none: Option<bool>,
    pub text_wrapping_glyph: Option<bool>,
    pub text_wrapping_word_glyph: Option<bool>,
    pub font_id: Option<usize>,
    pub style_id: Option<usize>,
    pub show: bool,
}

impl Radio {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self, 
        widgets: &HashMap<usize, Widgets>
    ) -> Option<Element<'a, app::Message>> {
        
        if !self.show { return None }

        let font_opt = 
            self.lookup(widgets, self.font_id)
                .and_then(Widgets::as_font).cloned();

        let mut radio_elements = vec![];

        for (i, label) in  self.labels.iter().enumerate() {
            
            let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_radio_style).cloned();

            let selected = if self.selected_index == Some(i) {
                    Some(i)
                } else {
                    None
                };

            let mut rd = widget::Radio::new(
                        label.clone(),
                        i,                        // value = index
                        selected,                 // selected = Option<usize>
                        RDMessage::OnSelected, // f: usize -> RDMessage
                    )
                .width(get_len(self.fill, self.width_fill, self.width))
                .style(move|theme: &Theme, status| {
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, status)
                    } else {
                        radio::default(theme, status)
                    }
                });

            if let Some(sz) = self.size {
                rd = rd.size(sz);
            }

            if let Some(sp) = self.spacing {
                rd = rd.spacing(sp);
            }

            if let Some(ts) = self.text_size {
                rd = rd.text_size(ts);
            }

            if let Some(lh) = self.text_line_height {
                rd = rd.text_line_height(lh);
            }

            let rd = if let Some(font) = &font_opt {
                rd.font(font.to_iced())
            } else { rd };

            // default is word so not checked
            let rd = 
                if self.text_wrapping_none.is_some() {
                    rd.text_wrapping(Wrapping::None)
                } else if self.text_wrapping_glyph.is_some() {
                    rd.text_wrapping(Wrapping::Glyph)
                } else if self.text_wrapping_word_glyph.is_some() {
                    rd.text_wrapping(Wrapping::WordOrGlyph)
                } else { rd };

            radio_elements.push(rd);
            
        }

        let elements: Vec<Element<'_, RDMessage>> = 
            radio_elements.into_iter().map(|r| r.into()).collect();

        let rd: Element<RDMessage> = match self.direction {
                RadioDirection::Horizontal =>{
                    let mut rw: Row<'_, RDMessage> = 
                        Row::with_children(elements)
                            .width(get_len(self.fill, self.width_fill, self.width))
                            .height(get_len(self.fill, self.height_fill, self.height))
                            .padding(get_padding(&self.padding));
                    
                    if let Some(rd_sp) = self.radio_spacing {
                        rw = rw.spacing(rd_sp);
                    }
                    rw.into()
                },
                RadioDirection::Vertical => {
                    let mut col: Column<'_, RDMessage> = 
                        Column::with_children(elements)
                            .padding(get_padding(&self.padding))
                            .width(get_len(self.fill, self.width_fill, self.width))
                            .height(get_len(self.fill, self.height_fill, self.height));

                    if let Some(rd_sp) = self.radio_spacing {
                        col = col.spacing(rd_sp);
                    }                                    
                    col.into()                                                               
                },
        };

        Some(rd.map(move |message| app::Message::Radio(self.id, message)))

    }
}

#[derive(Debug, Clone)]
pub enum RDMessage {
    OnSelected(usize),
}

pub fn radio_callback(state: &mut IpgState, id: usize, message: RDMessage) {
    match message {
        RDMessage::OnSelected(selected) => {
            // Update widget state directly
            if let Some(Widgets::Radio(rd)) = state.widgets.get_mut(&id) {
                rd.selected_index = Some(selected);
            }
            invoke_callback_with_args(id, "on_selected", "Radio", selected,
                "def cb(wid: int, on_selected: int)");
        },
    }
 }


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum RadioDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub struct RadioStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,
    pub background_color_hovered: Option<Color>,
    pub background_color_hovered_alpha: Option<f32>,
    pub background_rgba_hovered: Option<[f32; 4]>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_width: Option<f32>,
    pub dot_color: Option<Color>,
    pub dot_color_alpha: Option<f32>,
    pub dot_rgba: Option<[f32; 4]>,
    pub dot_color_hovered: Option<Color>,
    pub dot_color_hovered_alpha: Option<f32>,
    pub dot_rgba_hovered: Option<[f32; 4]>,
    pub text_color: Option<Color>,
    pub text_color_alpha: Option<f32>,
    pub text_rgba: Option<[f32; 4]>,
}

impl RadioStyle {
    pub fn to_iced(
        &self,
        theme: &Theme, 
        status: Status, 
    ) -> radio::Style {
        
        let mut style = radio::default(theme, status);

        let background_color = 
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
        let background_color_hovered = 
            Color::rgba_ipg_color_to_iced(self.background_rgba_hovered, &self.background_color_hovered, self.background_color_hovered_alpha);
        let dot_color = 
            Color::rgba_ipg_color_to_iced(self.dot_rgba, &self.dot_color, self.dot_color_alpha);
        let dot_color_hovered = 
            Color::rgba_ipg_color_to_iced(self.dot_rgba_hovered, &self.dot_color_hovered, self.dot_color_hovered_alpha);
        let border_color = 
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);
        let text_color = 
            Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha);

        style.text_color = text_color;
        
        if let Some(bkg) = background_color {
            style.background = bkg.into();
        }

        if let Some(dc) = dot_color {
            style.dot_color = dc;
        }
        
        // border color changes to inner color during hover
        if let Some(bc) = border_color {
            style.border_color = bc;
        }
        
        if let Some(bw) = self.border_width {
            style.border_width = bw;
        }
            
        match status {
            Status::Active{..} => style,
            Status::Hovered{..} => {
                if let Some(bch) = background_color_hovered {
                    style.background = bch.into();
                }
                if let Some(dch) = dot_color_hovered {
                    style.dot_color = dch;
                }
                style
            },
        }

    }
}
#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum RadioStyleParam {
    BackgroundColor,
    BackgroundRgbaColor,
    BorderColor,
    BorderRgbaColor,
    BorderWidth,
    DotColor,
    DotRgbaColor,
    DotColorHovered,
    DotRgbaColorHovered,
    TextColor,
    TextRgbaColor,
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum RadioParam {
    Direction,
    Fill,
    FontId,
    Height,
    HeightFill,
    Labels,
    Padding,
    SelectedIndex,
    Show,
    Size,
    Spacing,
    RadioSpacing,
    StyleId,
    TextLineHeight,
    TextSize,
    TextWrappingGlyph,
    TextWrappingNone,
    TextWrappingWordGlyph,
    Width,
    WidthFill,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Radio {
    type Param = RadioParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RadioParam::Direction => set_t_value(&mut self.direction, value, "RadioParam::Direction"),
            RadioParam::Fill => set_t_value(&mut self.fill, value, "RadioParam::Fill"),
            RadioParam::FontId => set_t_value(&mut self.font_id, value, "RadioParam::FontId"),
            RadioParam::Height => set_t_value(&mut self.height, value, "RadioParam::Height"),
            RadioParam::HeightFill => set_t_value(&mut self.height_fill, value, "RadioParam::HeightFill"),
            RadioParam::Labels => set_t_value(&mut self.labels, value, "RadioParam::Labels"),
            RadioParam::Padding => set_t_value(&mut self.padding, value, "RadioParam::Padding"),
            RadioParam::SelectedIndex => set_t_value(&mut self.selected_index, value, "RadioParam::SelectedIndex"),
            RadioParam::Show => set_t_value(&mut self.show, value, "RadioParam::Show"),
            RadioParam::Size => set_t_value(&mut self.size, value, "RadioParam::Size"),
            RadioParam::Spacing => set_t_value(&mut self.spacing, value, "RadioParam::Spacing"),
            RadioParam::RadioSpacing => set_t_value(&mut self.radio_spacing, value, "RadioParam::RadioSpacing"),
            RadioParam::StyleId => set_t_value(&mut self.style_id, value, "RadioParam::StyleId"),
            RadioParam::TextLineHeight => set_t_value(&mut self.text_line_height, value, "RadioParam::TextLineHeight"),
            RadioParam::TextSize => set_t_value(&mut self.text_size, value, "RadioParam::TextSize"),
            RadioParam::TextWrappingNone => set_t_value(&mut self.text_wrapping_none, value, "RadioParam::TextWrappingNone"),
            RadioParam::TextWrappingGlyph => set_t_value(&mut self.text_wrapping_glyph, value, "RadioParam::TextWrappingGlyph"),
            RadioParam::TextWrappingWordGlyph => set_t_value(&mut self.text_wrapping_word_glyph, value, "RadioParam::TextWrappingWordGlyph"),
            RadioParam::Width => set_t_value(&mut self.width, value, "RadioParam::Width"),
            RadioParam::WidthFill => set_t_value(&mut self.width_fill, value, "RadioParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for RadioStyle {
    type Param = RadioStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RadioStyleParam::BackgroundColor => 
                set_t_value(&mut self.background_color, value, "RadioStyleParam::BackgroundColor"),
            RadioStyleParam::BackgroundRgbaColor => 
                set_t_value(&mut self.background_color, value, "RadioStyleParam::BackgroundRgbaColor"),
            RadioStyleParam::BorderColor => 
                set_t_value(&mut self.border_color, value, "RadioStyleParam::BorderColor"),
            RadioStyleParam::BorderRgbaColor => 
                set_t_value(&mut self.border_color, value, "RadioStyleParam::BorderRgbaColor"),
            RadioStyleParam::BorderWidth => 
                set_t_value(&mut self.border_width, value, "RadioStyleParam::BorderWidth"),
            RadioStyleParam::DotColor => 
                set_t_value(&mut self.dot_color, value, "RadioStyleParam::DotColor"),
            RadioStyleParam::DotRgbaColor => 
                set_t_value(&mut self.dot_color, value, "RadioStyleParam::DotRgbaColor"),
            RadioStyleParam::DotColorHovered => 
                set_t_value(&mut self.dot_color_hovered, value, "RadioStyleParam::DotColorHovered"),
            RadioStyleParam::DotRgbaColorHovered => 
                set_t_value(&mut self.dot_color_hovered, value, "RadioStyleParam::DotRgbaColorHovered"),
            RadioStyleParam::TextColor => 
                set_t_value(&mut self.text_color, value, "RadioStyleParam::TextColor"),
            RadioStyleParam::TextRgbaColor => 
                set_t_value(&mut self.text_color, value, "RadioStyleParam::TextRgbaColor"),
        }
    }
}
