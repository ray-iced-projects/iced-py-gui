//! ipg_toggler

use std::collections::HashMap;

use iced::advanced::text;
use iced::widget;
use iced::widget::text::Wrapping;
use iced::{Element, Theme};
use iced::theme::palette::{deviate, mix};

use pyo3::{pyclass, Py, PyAny};

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::helpers::{get_len, get_radius};
use crate::state::Widgets;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};
use crate::IpgState;
use crate::widgets::callbacks::invoke_callback_with_args;
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Toggler {
    pub id: usize,
    pub show: bool,
    pub is_toggled: bool,
    pub label: Option<String>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub size: Option<f32>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub text_center: Option<bool>,
    pub text_left: Option<bool>,
    pub text_right: Option<bool>,
    pub wrapping_none: Option<bool>,
    pub wrapping_glyph: Option<bool>,
    pub wrapping_word_glyph: Option<bool>,
    pub spacing: Option<f32>,
    pub font_id: Option<usize>,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum TOGMessage {
    Toggled(bool),
}

impl Toggler {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }
    
    pub fn construct<'a>(
        &'a self, 
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message>> {
        
        if !self.show { return None }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_toggler_style).cloned();

        let font_opt = 
            self.lookup(widgets, self.font_id)
                .and_then(Widgets::as_font).cloned();

        let mut tog =  
            widget::Toggler::new(self.is_toggled)
                .on_toggle(TOGMessage::Toggled)
                .width(get_len(None, self.width_fill, self.width))
                .style(move|theme: &Theme, status| {
                    if let Some(st) = &style_opt {
                        st.set_style(theme, status)
                    } else {
                        widget::toggler::default(theme, status)
                    } 
                });

        if let Some(lb)  = &self.label {
            tog = tog.label(lb);
        }

        if let Some(sz) = self.size {
            tog = tog.size(sz);
        }

        if self.text_center == Some(true){
            tog = tog.text_alignment(text::Alignment::Center);
        }

        if self.text_left == Some(true){
            tog = tog.text_alignment(text::Alignment::Left);
        }

        if self.text_right == Some(true){
            tog = tog.text_alignment(text::Alignment::Right);
        }

        if let Some(lh) = self.text_line_height {
            tog = tog.text_line_height(lh);
        }

        if let Some(ts) = self.text_size {
            tog = tog.text_size(ts);
        }

        if let Some(sp) = self.spacing {
            tog = tog.spacing(sp);
        }

        if let Some(f) = font_opt {
            tog = tog.font(f.to_iced())
        };
        

        // default is word so not checked
        let tog = 
            if self.wrapping_none.is_some() {
                tog.text_wrapping(Wrapping::None)
            } else if self.wrapping_glyph.is_some() {
                tog.text_wrapping(Wrapping::Glyph)
            } else if self.wrapping_word_glyph.is_some() {
                tog.text_wrapping(Wrapping::WordOrGlyph)
            } else { tog };

        let tog: Element<'_, TOGMessage> = tog.into();
        Some(tog.map(move |message| Message::Toggler(self.id, message)))

    }
}

pub fn toggle_callback(state: &mut IpgState, id: usize, message: TOGMessage) {
    match message {
        TOGMessage::Toggled(is_toggled) => {
            // Update widget state directly
            if let Some(Widgets::Toggler(tog)) = state.widgets.get_mut(&id) {
                tog.is_toggled = is_toggled;
            }
            invoke_callback_with_args(id, "toggled", "Toggler", is_toggled,
                "def cb(wid: int, is_toggled: bool)");
        }
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TogglerParam {
    FontId,
    Label,
    Show,
    Size,
    Spacing,
    StyleId,
    TextCenter,
    TextLeft,
    TextLineHeight,
    TextRight,
    TextSize,
    Width,
    WidthFill,
    WrappingGlyph,
    WrappingNone,
    WrappingWordGlyph,
}

#[derive(Debug, Clone)]
pub struct TogglerStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,
    pub background_border_color: Option<Color>,
    pub background_border_color_alpha: Option<f32>,
    pub background_border_rgba: Option<[f32; 4]>,
    pub background_border_width: Option<f32>,
    pub foreground_color: Option<Color>,
    pub foreground_color_alpha: Option<f32>,
    pub foreground_rgba: Option<[f32; 4]>,
    pub foreground_border_color: Option<Color>,
    pub foreground_border_color_alpha: Option<f32>,
    pub foreground_border_rgba: Option<[f32; 4]>,
    pub foreground_border_width: Option<f32>,
    pub text_color: Option<Color>,
    pub text_color_alpha: Option<f32>,
    pub text_rgba: Option<[f32; 4]>, 
    pub border_radius: Option<Vec<f32>>,
    pub padding_ratio: Option<f32>,
}

//      Status	            Background	       Foreground
// Active + toggled	        bg_base	            fg_base
// Active + untoggled	    deviate(bg, 0.15)	deviate(fg, 0.15)
// Hovered + toggled	    bg_base	            fg_base at 50% alpha
// Hovered + untoggled	    deviate(bg, 0.15)	deviate(fg, 0.1)
// Disabled + toggled	    deviate(bg, 0.1)	mix(fg, bg, 0.4)
// Disabled + untoggled	    deviate(bg, 0.2)	mix(fg, bg, 0.4)
impl TogglerStyle {
    fn set_style(
        &self, 
        theme: &Theme, 
        status: widget::toggler::Status, 
        ) -> widget::toggler::Style {

        let background_color = 
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
        let _background_border_color = 
            Color::rgba_ipg_color_to_iced(self.background_border_rgba, &self.background_border_color, self.background_border_color_alpha);
        let foreground_color = 
            Color::rgba_ipg_color_to_iced(self.foreground_rgba, &self.foreground_color, self.foreground_color_alpha);
        let foreground_border_color = 
            Color::rgba_ipg_color_to_iced(self.foreground_border_rgba, &self.foreground_border_color, self.foreground_border_color_alpha);
        
        let text_color = 
            Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha);
        
        // Use user-supplied base colors or fall back to theme palette colors
        let palette = theme.extended_palette();

        let bg_base = background_color
            .unwrap_or(palette.primary.base.color);
        let fg_base = foreground_color
            .unwrap_or(palette.primary.base.text);

        // Derive variants: deviate auto-lightens dark colors and darkens light ones
        let bg_untoggled = deviate(bg_base, 0.15);      // like "strong"
        let bg_disabled_on = deviate(bg_base, 0.1);
        let bg_disabled_off = deviate(bg_base, 0.2);

        let background = match status {
            widget::toggler::Status::Active { is_toggled } | widget::toggler::Status::Hovered { is_toggled } => {
                if is_toggled { bg_base } else { bg_untoggled }
            }
            widget::toggler::Status::Disabled { is_toggled } => {
                if is_toggled { bg_disabled_on } else { bg_disabled_off }
            }
        };

        let fg_untoggled = deviate(fg_base, 0.15);
        let fg_hovered_on = iced::Color { a: 0.5, ..fg_base };
        let fg_hovered_off = deviate(fg_base, 0.1);
        let fg_disabled = mix(fg_base, background, 0.4);

        let foreground = match status {
            widget::toggler::Status::Active { is_toggled } => {
                if is_toggled { fg_base } else { fg_untoggled }
            }
            widget::toggler::Status::Hovered { is_toggled } => {
                if is_toggled { fg_hovered_on } else { fg_hovered_off }
            }
            widget::toggler::Status::Disabled { .. } => fg_disabled,
        };

        let border_radius = if let Some(br) = &self.border_radius {
            Some(get_radius(&br, "toggle".to_string()))
        } else { None };

        widget::toggler::Style {
            background: background.into(),
            foreground: foreground.into(),
            foreground_border_width: self.foreground_border_width.unwrap_or(0.0),
            foreground_border_color: foreground_border_color.unwrap_or(iced::Color::TRANSPARENT),
            background_border_width: self.background_border_width.unwrap_or(0.0),
            background_border_color: iced::Color::TRANSPARENT,
            text_color: text_color,
            border_radius,
            padding_ratio: self.padding_ratio.unwrap_or(0.1),
        }

    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TogglerStyleParam {
    BackgroundBorderColor,
    BackgroundBorderColorAlpha,
    BackgroundBorderRgba,
    BackgroundBorderWidth,
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRgba,
    BorderRadius,
    ForegroundBorderColor,
    ForegroundBorderColorAlpha,
    ForegroundBorderRgba,
    ForegroundBorderWidth,
    ForegroundColor,
    ForegroundColorAlpha,
    ForegroundRgba,
    PaddingRatio,
    TextColor,
    TextColorAlpha,
    TextRgba,
}



// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Toggler {
    type Param = TogglerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TogglerParam::FontId => set_t_value(&mut self.font_id, value, "TogglerParam::FontId"),
            TogglerParam::Label => set_t_value(&mut self.label, value, "TogglerParam::Label"),
            TogglerParam::Show => set_t_value(&mut self.show, value, "TogglerParam::Show"),
            TogglerParam::Size => set_t_value(&mut self.size, value, "TogglerParam::Size"),
            TogglerParam::Spacing => set_t_value(&mut self.spacing, value, "TogglerParam::Spacing"),
            TogglerParam::StyleId => set_t_value(&mut self.style_id, value, "TogglerParam::StyleId"),
            TogglerParam::TextCenter => set_t_value(&mut self.text_center, value, "TogglerParam::TextCenter"),
            TogglerParam::TextLeft => set_t_value(&mut self.text_left, value, "TogglerParam::TextLeft"),
            TogglerParam::TextLineHeight => set_t_value(&mut self.text_line_height, value, "TogglerParam::TextLineHeight"),
            TogglerParam::TextRight => set_t_value(&mut self.text_right, value, "TogglerParam::TextRight"),
            TogglerParam::TextSize => set_t_value(&mut self.text_size, value, "TogglerParam::TextSize"),
            TogglerParam::Width => set_t_value(&mut self.width, value, "TogglerParam::Width"),
            TogglerParam::WidthFill => set_t_value(&mut self.width_fill, value, "TogglerParam::WidthFill"),
            TogglerParam::WrappingGlyph => set_t_value(&mut self.wrapping_glyph, value, "TogglerParam::WrappingGlyph"),
            TogglerParam::WrappingNone => set_t_value(&mut self.wrapping_none, value, "TogglerParam::WrappingNone"),
            TogglerParam::WrappingWordGlyph => set_t_value(&mut self.wrapping_word_glyph, value, "TogglerParam::WrappingWordGlyph"),
        }
    }
}

impl WidgetParamUpdate for TogglerStyle {
    type Param = TogglerStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TogglerStyleParam::BackgroundBorderColor => set_t_value(&mut self.background_border_color, value, "TogglerStyleParam:BackgroundBorderColor"),
            TogglerStyleParam::BackgroundBorderColorAlpha => set_t_value(&mut self.background_border_color_alpha, value, "TogglerStyleParam::BackgroundBorderColorAlpha"),
            TogglerStyleParam::BackgroundBorderRgba => set_t_value(&mut self.background_border_rgba, value, "TogglerStyleParam:BackgroundBorderRgba"),
            TogglerStyleParam::BackgroundBorderWidth => set_t_value(&mut self.background_border_width, value, "TogglerStyleParam:BackgroundBorderWidth"),
            TogglerStyleParam::BackgroundColor => set_t_value(&mut self.background_color, value, "TogglerStyleParam:BackgroundColor"),
            TogglerStyleParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "TogglerStyleParam::BackgroundColorAlpha"),
            TogglerStyleParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "TogglerStyleParam:BackgroundRgba"),
            TogglerStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "TogglerStyleParam:BorderRadius"),
            TogglerStyleParam::ForegroundBorderColor => set_t_value(&mut self.foreground_border_color, value, "TogglerStyleParam:ForegroundBorderColor"),
            TogglerStyleParam::ForegroundBorderColorAlpha => set_t_value(&mut self.foreground_border_color_alpha, value, "TogglerStyleParam::ForegroundBorderColorAlpha"),
            TogglerStyleParam::ForegroundBorderRgba => set_t_value(&mut self.foreground_border_rgba, value, "TogglerStyleParam:ForegroundBorderRgba"),
            TogglerStyleParam::ForegroundBorderWidth => set_t_value(&mut self.foreground_border_width, value, "TogglerStyleParam:ForegroundBorderWidth"),
            TogglerStyleParam::ForegroundColor => set_t_value(&mut self.foreground_color, value, "TogglerStyleParam:ForegroundColor"),
            TogglerStyleParam::ForegroundColorAlpha => set_t_value(&mut self.foreground_color_alpha, value, "TogglerStyleParam::ForegroundColorAlpha"),
            TogglerStyleParam::ForegroundRgba => set_t_value(&mut self.foreground_rgba, value, "TogglerStyleParam:ForegroundRgba"),
            TogglerStyleParam::PaddingRatio => set_t_value(&mut self.padding_ratio, value, "TogglerStyleParam:PaddingRatio"),
            TogglerStyleParam::TextColor => set_t_value(&mut self.text_color, value, "TogglerStyleParam:extColor"),
            TogglerStyleParam::TextColorAlpha => set_t_value(&mut self.text_color_alpha, value, "TogglerStyleParam::TextColorAlpha"),
            TogglerStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "TogglerStyleParam:TextRgba"),
        }
    }
}
