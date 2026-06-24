//! ipg_checkbox

use std::collections::HashMap;

use crate::graphics::colors::Color;
use crate::py_api::colors::{CustomPalette, PaletteKey, StylePart, WidgetStatus};
use crate::py_api::helpers::get_len;
use crate::state::IpgState;
use crate::app::Message;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::state::Widgets;

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap::bootstrap_icon::Icon;

use iced::advanced::text;
use iced::{Background, Element, Theme};
use iced::widget::text::{LineHeight, Shaping, Wrapping};
use iced::widget::{Checkbox, checkbox};
use iced::theme::palette::{self, Pair};

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct CheckBox {
    pub id: usize,
    pub show: bool,
    pub is_checked: bool,
    pub label: Option<String>,
    pub width: Option<f32>,
    pub fill: Option<bool>,
    pub size: Option<f32>,
    pub spacing: Option<f32>,
    pub text_size: Option<f32>,
    pub line_height: Option<f32>,
    pub text_wrapping_none: Option<bool>,
    pub text_wrapping_glyph: Option<bool>,
    pub text_wrapping_word_glyph: Option<bool>,
    pub text_font_id: Option<usize>,
    pub icon_font_id: Option<usize>,
    pub icon: Option<Icon>,
    pub icon_size: Option<f32>,
    pub icon_line_height: Option<f32>,
    pub disabled: Option<bool>,
    pub style_id: Option<usize>,
    pub style_std: Option<CheckboxStyleStd>,
    pub palette_id: Option<usize>,
}

impl CheckBox {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message>> {

        if !self.show { return None };

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_checkbox_style).cloned();

        let pal_opt =
            self.lookup(widgets, self.palette_id)
                .and_then(Widgets::as_palette).cloned();

        // Icon related
        let code_point = 
            if let Some(ic) = self.icon {
                ic.to_char()
            } else {
                Icon::Check.to_char()
            };

        let size = 
            if let Some(sz) = self.icon_size {
                Some(iced::Pixels(sz))
            } else { None };

        let line_height = 
            if let Some(lh) = self.icon_line_height {
                LineHeight::Relative(lh)
            } else { LineHeight::default() };

        let icon = 
            checkbox::Icon {
                font: BOOTSTRAP_FONT,
                code_point,
                size,
                line_height,
                shaping: Shaping::Auto
            };
        
        // Text related
        let line_height = 
            if let Some(lh) = self.line_height {
                text::LineHeight::Relative(lh)
            } else { text::LineHeight::default() };

        let chk = 
            Checkbox::new(self.is_checked)
                .on_toggle(ChkMessage::OnToggle)
                .width(get_len(self.fill, None, self.width))
                .line_height(line_height)
                .icon(icon)
                .style(move|theme: &Theme, status| {
                    let status = if self.disabled == Some(true) {
                        let is_checked = match status {
                            checkbox::Status::Active { is_checked }
                            | checkbox::Status::Hovered { is_checked }
                            | checkbox::Status::Disabled { is_checked } => is_checked,
};
                        checkbox::Status::Disabled{is_checked}
                    } else { status };
                    if style_opt.is_some() || pal_opt.is_some() {
                        let chk_st = CheckboxStyle::default();
                        let st = style_opt.as_ref().unwrap_or(&chk_st);
                        st.to_iced(theme, status, &pal_opt)
                    } else {
                       match &self.style_std {
                            Some(std) => std.to_iced(theme, status),
                            None => checkbox::primary(theme, status),
                        }
                    }
                }
            );
        
        let chk = 
            if let Some(lb) = &self.label {
                chk.label(lb.clone())
            } else { chk };

        let chk = 
            if let Some(sz) = self.size {
                chk.size(sz)
            } else { chk };

        let chk = 
            if let Some(sp) = self.spacing {
                chk.spacing(sp)
            } else { chk };

        // default is word so not checked
        let chk = 
            if self.text_wrapping_none.is_some() {
                chk.wrapping(Wrapping::None)
            } else if self.text_wrapping_glyph.is_some() {
                chk.wrapping(Wrapping::Glyph)
            } else if self.text_wrapping_word_glyph.is_some() {
                chk.wrapping(Wrapping::WordOrGlyph)
            } else { chk };

        let chk: Element<'_, ChkMessage> = chk.into();
        Some(chk.map(move |message| Message::CheckBox(self.id, message)))

    }
}

#[derive(Debug, Clone)]
pub enum ChkMessage {
    OnToggle(bool),
}

pub fn checkbox_callback(state: &mut IpgState, id: usize, message: ChkMessage) {

    match message {
        ChkMessage::OnToggle(is_checked) => {
            if let Some(Widgets::CheckBox(cb)) = state.widgets.get_mut(&id) {
                cb.is_checked = is_checked;
            }
            invoke_callback_with_args(id, "on_toggle", "Checkbox", is_checked,
                "def cb(wid: int, is_checked: bool)");
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CheckboxStyle {
    pub id: usize,

    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    
    pub text_color: Option<Color>,
    pub text_color_alpha: Option<f32>,
    pub text_rgba: Option<[f32; 4]>,
}

impl CheckboxStyle {
    fn to_iced(
         &self, 
        theme: &Theme, 
        status: checkbox::Status,
        c_pal_opt: &Option<CustomPalette>,
        ) -> checkbox::Style {
        
        // Build the background palette — either from CustomPalette or theme default.
        let custom_pal = if let Some(cp) = c_pal_opt {
            cp
        } else {
            let palette = theme.palette();
            let background = palette.background;
            &CustomPalette { 
                id: 0, 
                background, 
                statuses: None, 
            }
         };

        let mut default_statuses: HashMap<WidgetStatus, HashMap<StylePart, (PaletteKey, f32)>> = HashMap::new();
        
        let mut inner = HashMap::new();
        inner.insert(StylePart::Border, (PaletteKey::Strong, 1.0));
        inner.insert(StylePart::Base, (PaletteKey::Base,     1.0));
        inner.insert(StylePart::Icon, (PaletteKey::Base,     1.0));
        inner.insert(StylePart::Accent, (PaletteKey::Base,   1.0));
        inner.insert(StylePart::Text, (PaletteKey::Base,     1.0));
        
        default_statuses.insert(WidgetStatus::Active, inner);
                                                    
        let mut inner = HashMap::new();
        inner.insert(StylePart::Border, (PaletteKey::Strong, 1.0));
        inner.insert(StylePart::Base, (PaletteKey::Weak,     1.0));
        inner.insert(StylePart::Icon, (PaletteKey::Base,     1.0));
        inner.insert(StylePart::Accent, (PaletteKey::Strong, 1.0));
        inner.insert(StylePart::Text, (PaletteKey::Base,     1.0));
        
        default_statuses.insert(WidgetStatus::Hovered, inner);
         
        let mut inner = HashMap::new();
        inner.insert(StylePart::Border, (PaletteKey::Weak,   1.0));
        inner.insert(StylePart::Base, (PaletteKey::Weaker,   1.0));
        inner.insert(StylePart::Icon, (PaletteKey::Base,     1.0));
        inner.insert(StylePart::Accent, (PaletteKey::Strong, 1.0));
        inner.insert(StylePart::Text, (PaletteKey::Base,     1.0));
        
        default_statuses.insert(WidgetStatus::Disabled, inner);
        
        let statuses = if let Some(status) = custom_pal.statuses.as_ref() {
            let mut merged: HashMap<WidgetStatus, HashMap<StylePart, (PaletteKey, f32)>> = status
                .iter()
                .map(|(widget_status, parts)| {
                    let part_map: HashMap<StylePart, (PaletteKey, f32)> = parts
                        .iter()
                        .map(|(style_part, palette_key, alpha)| {
                            (style_part.clone(), (palette_key.clone(), *alpha))
                        })
                        .collect();
                    (widget_status.clone(), part_map)
                })
                .collect();

            for (default_widget_status, default_parts) in &default_statuses {
                let merged_parts = merged
                    .entry(default_widget_status.clone())
                    .or_insert_with(|| default_parts.clone());

                for (default_style_part, default_pair) in default_parts {
                    merged_parts
                        .entry(default_style_part.clone())
                        .or_insert_with(|| default_pair.clone());
                }
            }

            merged
        } else { default_statuses };

        let bkg = custom_pal.background;
        
        match status {
            checkbox::Status::Active { is_checked } => {
                let active = statuses.get(&WidgetStatus::Active).unwrap();
                let (border_pal, bd_alpha) = active.get(&StylePart::Border).unwrap();
                let (base_pal, base_alpha) = active.get(&StylePart::Base).unwrap();
                let (icon_pal, ic_alpha) = active.get(&StylePart::Icon).unwrap();
                let (accent_pal, ac_alpha) = active.get(&StylePart::Accent).unwrap();
                let border_color = border_pal.color_key_to_color(&bkg).scale_alpha(*bd_alpha);
                let base_pair = Pair::new(base_pal.color_key_to_color(&bkg).scale_alpha(*base_alpha),
                                    base_pal.text_key_to_color(&bkg).scale_alpha(*base_alpha));
                let icon_color = icon_pal.text_key_to_color(&bkg).scale_alpha(*ic_alpha);
                let accent_pair = Pair::new(accent_pal.color_key_to_color(&bkg).scale_alpha(*ac_alpha),
                                    accent_pal.text_key_to_color(&bkg).scale_alpha(*ac_alpha));
                styled(border_color, base_pair, icon_color, accent_pair, is_checked)

            },
            checkbox::Status::Hovered { is_checked } => {
                let hovered = statuses.get(&WidgetStatus::Hovered).unwrap();
                let (border_pal, bd_alpha) = hovered.get(&StylePart::Border).unwrap();
                let (base_pal, base_alpha) = hovered.get(&StylePart::Base).unwrap();
                let (icon_pal, ic_alpha) = hovered.get(&StylePart::Icon).unwrap();
                let (accent_pal, ac_alpha) = hovered.get(&StylePart::Accent).unwrap();
                let border_color = border_pal.color_key_to_color(&bkg).scale_alpha(*bd_alpha);
                let base_pair = Pair::new(base_pal.color_key_to_color(&bkg).scale_alpha(*base_alpha),
                                    base_pal.text_key_to_color(&bkg).scale_alpha(*base_alpha));
                let icon_color = icon_pal.text_key_to_color(&bkg).scale_alpha(*ic_alpha);
                let accent_pair = Pair::new(accent_pal.color_key_to_color(&bkg).scale_alpha(*ac_alpha),
                                    accent_pal.text_key_to_color(&bkg).scale_alpha(*ac_alpha));
                styled(border_color, base_pair, icon_color, accent_pair, is_checked)
            },
            checkbox::Status::Disabled { is_checked } => {
                let disabled = statuses.get(&WidgetStatus::Disabled).unwrap();
                let (border_pal, bd_alpha) = disabled.get(&StylePart::Border).unwrap();
                let (base_pal, base_alpha) = disabled.get(&StylePart::Base).unwrap();
                let (icon_pal, ic_alpha) = disabled.get(&StylePart::Icon).unwrap();
                let (accent_pal, ac_alpha) = disabled.get(&StylePart::Accent).unwrap();
                let border_color = border_pal.color_key_to_color(&bkg).scale_alpha(*bd_alpha);
                let base_pair = Pair::new(base_pal.color_key_to_color(&bkg).scale_alpha(*base_alpha),
                                    base_pal.text_key_to_color(&bkg).scale_alpha(*base_alpha));
                let icon_color = icon_pal.text_key_to_color(&bkg).scale_alpha(*ic_alpha);
                let accent_pair = Pair::new(accent_pal.color_key_to_color(&bkg).scale_alpha(*ac_alpha),
                                    accent_pal.text_key_to_color(&bkg).scale_alpha(*ac_alpha));
                styled(border_color, base_pair, icon_color, accent_pair, is_checked)
            },
        }

        
    }
}


fn styled(
    border_color: iced::Color,
    base: palette::Pair,
    icon_color: iced::Color,
    accent: palette::Pair,
    is_checked: bool,
) -> checkbox::Style {
    let (background, border) = if is_checked {
        (accent, accent.color)
    } else {
        (base, border_color)
    };

    checkbox::Style {
        background: Background::Color(background.color),
        icon_color,
        border: iced::Border {
            radius: 2.0.into(),
            width: 1.0,
            color: border,
        },
        text_color: None,
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum CheckboxStyleStd {
    Danger,
    Primary,
    Secondary,
    Success,
}

impl CheckboxStyleStd {
    pub fn to_iced (
        &self,
        theme: &Theme, 
        status: checkbox::Status, 
        ) -> checkbox::Style {
        
        match self {
            CheckboxStyleStd::Danger => {
                checkbox::danger(theme, status)
            },
            CheckboxStyleStd::Primary => {
                checkbox::primary(theme, status)
            },
            CheckboxStyleStd::Secondary => {
                checkbox::secondary(theme, status)
            },
            CheckboxStyleStd::Success => {
                checkbox::success(theme, status)
            },
        }
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum CheckboxParam {
    Fill,
    IconFontId,
    IconLineHeight,
    IconSize,
    Icon,
    IsChecked,
    Label,
    Show,
    Size,
    Spacing,
    StyleId,
    StyleStd,
    PaletteId,
    TextFontId,
    TextLineHeight,
    TextSize,
    TextWrappingGlyph,
    TextWrappingNone,
    TextWrappingWordGlyph,
    Width,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum CheckboxStyleParam {
    BorderRadius,
    BorderWidth,
    TextColor,
    TextColorAlpha,
    TextRgba,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for CheckBox {
    type Param = CheckboxParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {

        match param {
            CheckboxParam::Fill => set_t_value(&mut self.fill, value, "CheckboxParam::WidthFill"),
            CheckboxParam::Icon => set_t_value(&mut self.icon, value, "CheckboxParam::Icon"),
            CheckboxParam::IconFontId => set_t_value(&mut self.icon_font_id, value, "CheckboxParam::IconFontId"),
            CheckboxParam::IconLineHeight => set_t_value(&mut self.icon_line_height, value, "CheckboxParam::IconLineHeight"),
            CheckboxParam::IconSize => set_t_value(&mut self.icon_size, value, "CheckboxParam::IconSize"),
            CheckboxParam::IsChecked => set_t_value(&mut self.is_checked, value, "CheckboxParam::IsChecked"),
            CheckboxParam::Label => set_t_value(&mut self.label, value, "CheckboxParam::Label"),
            CheckboxParam::Show => set_t_value(&mut self.show, value, "CheckboxParam::Show"),
            CheckboxParam::Size => set_t_value(&mut self.size, value, "CheckboxParam::Size"),
            CheckboxParam::Spacing => set_t_value(&mut self.spacing, value, "CheckboxParam::Spacing"),
            CheckboxParam::StyleId => set_t_value(&mut self.style_id, value, "CheckboxParam::StyleId"),
            CheckboxParam::StyleStd => set_t_value(&mut self.style_std, value, "CheckboxParam::StyleStd"),
            CheckboxParam::PaletteId => set_t_value(&mut self.palette_id, value, "CheckboxParam::PaletteId"),
            CheckboxParam::TextFontId => set_t_value(&mut self.text_font_id, value, "CheckboxParam::TextFontId"),
            CheckboxParam::TextLineHeight => set_t_value(&mut self.line_height, value, "CheckboxParam::TextLineHeight"),
            CheckboxParam::TextSize => set_t_value(&mut self.text_size, value, "CheckboxParam::TextSize"),
            CheckboxParam::TextWrappingGlyph => set_t_value(&mut self.text_wrapping_glyph, value, "CheckboxParam::TextWrappingGlyph"),
            CheckboxParam::TextWrappingNone => set_t_value(&mut self.text_wrapping_none, value, "CheckboxParam::TextWrappingNone"),
            CheckboxParam::TextWrappingWordGlyph => set_t_value(&mut self.text_wrapping_word_glyph, value, "CheckboxParam::TextWrappingWordGlyph"),
            CheckboxParam::Width => set_t_value(&mut self.width, value, "CheckboxParam::Width"),
        }
    }
}

impl WidgetParamUpdate for CheckboxStyle {
    type Param = CheckboxStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            CheckboxStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "CheckboxStyleParam::BorderRadius"),
            CheckboxStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "CheckboxStyleParam::BorderWidth"),
            CheckboxStyleParam::TextColor => set_t_value(&mut self.text_color, value, "CheckboxStyleParam::TextColor"),
            CheckboxStyleParam::TextColorAlpha => set_t_value(&mut self.text_color_alpha, value, "CheckboxStyleParam::TextColorAlpha"),
            CheckboxStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "CheckboxStyleParam::TextRgbaColor"),
        }
    }
}
