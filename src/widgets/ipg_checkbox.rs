//! ipg_checkbox

use std::collections::HashMap;

use crate::graphics::colors::Color;
use crate::py_api::colors::{CustomPalette, PaletteKey, WidgetStatus};
use crate::py_api::helpers::get_len;
use crate::state::IpgState;
use crate::app::Message;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::state::Widgets;

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap::bootstrap_icon::Icon;

use crate::widgets::styling::apply_border_overrides;

use iced::advanced::text;
use iced::{Background, Element, Theme};
use iced::widget::text::{LineHeight, Shaping, Wrapping};
use iced::widget::{Checkbox, checkbox};
use iced::theme::palette;

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
                    if style_opt.is_some() || pal_opt.is_some() {
                        let chk_st = CheckboxStyle::default();
                        let st = style_opt.as_ref().unwrap_or(&chk_st);
                        let is_label = self.label.is_some();
                        st.to_iced(theme, status, is_label, &pal_opt)
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
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
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
        is_label: bool,
        c_pal_opt: &Option<CustomPalette>,
        ) -> checkbox::Style {

        let border_color_override = 
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);

        let text_color_override =
            Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha);

        // Build the background palette — either from CustomPalette or theme default.
        let custom = if let Some(cp) = c_pal_opt {
            cp.clone()
        } else {
            CustomPalette {
                id: 0,
                background: theme.palette().background,
                statuses: None,
                alpha: None,
            }
        };

        let alpha = custom.alpha.unwrap_or(1.0);
        let bkg = custom.background;

        // Default status → PaletteKey mappings for checkbox background slot.
        // Active/Hovered use base/weak; Disabled uses weaker.
        let default_statuses: HashMap<PaletteKey, WidgetStatus> = [
            (PaletteKey::Base,      WidgetStatus::Active),
            (PaletteKey::Weak,      WidgetStatus::Hovered),
            (PaletteKey::Weaker,    WidgetStatus::Disabled),
            (PaletteKey::BaseAlpha, WidgetStatus::IsChecked),
        ].into_iter().collect();
        let statuses = custom.statuses.as_ref().unwrap_or(&default_statuses);

        let key_for = |ws: WidgetStatus| -> PaletteKey {
            statuses.iter()
                .find_map(|(k, v)| if *v == ws { Some(k.clone()) } else { None })
                .unwrap_or(PaletteKey::Base)
        };

        let txt_color = text_color_override
            .unwrap_or_else(|| bkg.base.text);

        // Border colour: use override if supplied, else derive from palette.
        let border = border_color_override.unwrap_or(bkg.strong.color);

        // Accent (checked fill) comes from the IsChecked palette key.
        let accent_pair = get_bkg_pair(&key_for(WidgetStatus::IsChecked), &bkg, alpha);
        // For checked+hovered, deviate the accent slightly (mirrors iced primary.base → primary.strong).
        let accent_hovered = palette::Pair {
            color: deviate_for_hover(accent_pair.color),
            text: accent_pair.text,
        };

        let mut style = match status {
            checkbox::Status::Active { is_checked } => {
                let base_pair = get_bkg_pair(&key_for(WidgetStatus::Active), &bkg, alpha);
                styled(border, base_pair, accent_pair.text, accent_pair, is_checked)
            },
            checkbox::Status::Hovered { is_checked } => {
                let base_pair = get_bkg_pair(&key_for(WidgetStatus::Hovered), &bkg, alpha);
                styled(border, base_pair, accent_hovered.text, accent_hovered, is_checked)
            },
            checkbox::Status::Disabled { is_checked } => {
                let base_pair = get_bkg_pair(&key_for(WidgetStatus::Disabled), &bkg, alpha);
                let accent_disabled = get_bkg_pair(&key_for(WidgetStatus::IsChecked), &bkg, 0.5);
                let border_disabled = get_bkg_pair(&key_for(WidgetStatus::Active), &bkg, alpha).color;
                styled(border_disabled, base_pair, accent_pair.text, accent_disabled, is_checked)
            },
        };

        apply_border_overrides(
            &mut style.border, border_color_override,
            &self.border_radius, self.border_width, "Checkbox",
        );

        style.text_color = if is_label { Some(txt_color) } else { None };

        style
        
    }
}

fn deviate_for_hover(color: iced::Color) -> iced::Color {
    // Lighten dark colours, darken light colours by ~10% — mirrors iced's deviate(color, 0.1).
    let amount = 0.1_f32;
    if palette::is_dark(color) {
        iced::Color {
            r: color.r + (1.0 - color.r) * amount,
            g: color.g + (1.0 - color.g) * amount,
            b: color.b + (1.0 - color.b) * amount,
            a: color.a,
        }
    } else {
        iced::Color {
            r: (color.r * (1.0 - amount)).max(0.0),
            g: (color.g * (1.0 - amount)).max(0.0),
            b: (color.b * (1.0 - amount)).max(0.0),
            a: color.a,
        }
    }
}

// fn get_bkg_pair(key: &PaletteKey, bkg: &iced::theme::palette::Background, alpha: f32) -> palette::Pair {
//     match key {
//         PaletteKey::Base      => bkg.base,
//         PaletteKey::BaseAlpha => palette::Pair { color: bkg.base.color.scale_alpha(alpha), text: bkg.base.text.scale_alpha(alpha) },
//         PaletteKey::Neutral   => bkg.neutral,
//         PaletteKey::NeutralAlpha => palette::Pair { color: bkg.neutral.color.scale_alpha(alpha), text: bkg.neutral.text.scale_alpha(alpha) },
//         PaletteKey::Strong    => bkg.strong,
//         PaletteKey::StrongAlpha => palette::Pair { color: bkg.strong.color.scale_alpha(alpha), text: bkg.strong.text.scale_alpha(alpha) },
//         PaletteKey::Stronger  => bkg.stronger,
//         PaletteKey::StrongerAlpha => palette::Pair { color: bkg.stronger.color.scale_alpha(alpha), text: bkg.stronger.text.scale_alpha(alpha) },
//         PaletteKey::Strongest => bkg.strongest,
//         PaletteKey::StrongestAlpha => palette::Pair { color: bkg.strongest.color.scale_alpha(alpha), text: bkg.strongest.text.scale_alpha(alpha) },
//         PaletteKey::Weak      => bkg.weak,
//         PaletteKey::WeakAlpha => palette::Pair { color: bkg.weak.color.scale_alpha(alpha), text: bkg.weak.text.scale_alpha(alpha) },
//         PaletteKey::Weaker    => bkg.weaker,
//         PaletteKey::WeakerAlpha => palette::Pair { color: bkg.weaker.color.scale_alpha(alpha), text: bkg.weaker.text.scale_alpha(alpha) },
//         PaletteKey::Weakest   => bkg.weakest,
//         PaletteKey::WeakestAlpha => palette::Pair { color: bkg.weakest.color.scale_alpha(alpha), text: bkg.weakest.text.scale_alpha(alpha) },
//     }
// }

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
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
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
            CheckboxStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "CheckboxStyleParam::BorderColor"),
            CheckboxStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "CheckboxStyleParam::BorderColorAlpha"),
            CheckboxStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "CheckboxStyleParam::BorderRadius"),
            CheckboxStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "CheckboxStyleParam::BorderRgbaColor"),
            CheckboxStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "CheckboxStyleParam::BorderWidth"),
            CheckboxStyleParam::TextColor => set_t_value(&mut self.text_color, value, "CheckboxStyleParam::TextColor"),
            CheckboxStyleParam::TextColorAlpha => set_t_value(&mut self.text_color_alpha, value, "CheckboxStyleParam::TextColorAlpha"),
            CheckboxStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "CheckboxStyleParam::TextRgbaColor"),
        }
    }
}
