//! ipg_checkbox

use std::collections::HashMap;

use crate::graphics::colors::Color;
use crate::py_api::colors::{CustomPalette, PaletteKey, StateVariant, StylePart, WidgetStatus};
use crate::py_api::helpers::get_len;
use crate::state::IpgState;
use crate::app::Message;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::styling::apply_border_overrides;
use crate::state::Widgets;

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap::bootstrap_icon::Icon;

use iced::advanced::text;
use iced::{Background, Element, Theme};
use iced::widget::text::{LineHeight, Shaping, Wrapping};
use iced::widget::{Checkbox, checkbox};

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
            &CustomPalette { 
                id: 0, 
                palette: palette.background, 
                statuses: None, 
            }
         };

        let mut default_unchecked_statuses: HashMap<WidgetStatus, HashMap<StylePart, (PaletteKey, f32)>> = HashMap::new();
        
        let mut inner = HashMap::new();
        inner.insert(StylePart::Border, (PaletteKey::ThemeStrong,   1.0));
        inner.insert(StylePart::Background, (PaletteKey::ThemeBase, 1.0));
        inner.insert(StylePart::Icon, (PaletteKey::BaseText,       1.0));
        inner.insert(StylePart::Text, (PaletteKey::ThemeBaseText,  1.0));
        default_unchecked_statuses.insert(WidgetStatus::Active, inner);
                                                    
        let mut inner = HashMap::new();
        inner.insert(StylePart::Border, (PaletteKey::ThemeStrong,   1.0));
        inner.insert(StylePart::Background, (PaletteKey::ThemeWeak, 1.0));
        inner.insert(StylePart::Icon, (PaletteKey::BaseText,       1.0));
        inner.insert(StylePart::Text, (PaletteKey::ThemeBaseText,  1.0));
        
        default_unchecked_statuses.insert(WidgetStatus::Hovered, inner);
         
        let mut inner = HashMap::new();
        inner.insert(StylePart::Border, (PaletteKey::ThemeWeak,       1.0));
        inner.insert(StylePart::Background, (PaletteKey::ThemeWeaker, 1.0));
        inner.insert(StylePart::Icon, (PaletteKey::BaseText,     1.0));
        inner.insert(StylePart::Text, (PaletteKey::ThemeBaseText, 1.0));
        
        default_unchecked_statuses.insert(WidgetStatus::Disabled, inner);

        // Default checked overrides per interaction status.
        let mut default_checked_overrides: HashMap<WidgetStatus, HashMap<StylePart, (PaletteKey, f32)>> = HashMap::new();

        let mut inner = HashMap::new();
        inner.insert(StylePart::Border, (PaletteKey::Base, 1.0));
        inner.insert(StylePart::Background, (PaletteKey::Base, 1.0));
        inner.insert(StylePart::Text, (PaletteKey::ThemeBaseText, 1.0));
        default_checked_overrides.insert(WidgetStatus::Active, inner);

        let mut inner = HashMap::new();
        inner.insert(StylePart::Border, (PaletteKey::Strong, 1.0));
        inner.insert(StylePart::Background, (PaletteKey::Strong, 1.0));
        inner.insert(StylePart::Text, (PaletteKey::ThemeBaseText, 1.0));
        default_checked_overrides.insert(WidgetStatus::Hovered, inner);

        let mut inner = HashMap::new();
        inner.insert(StylePart::Border, (PaletteKey::ThemeStrong, 1.0));
        inner.insert(StylePart::Background, (PaletteKey::ThemeStrong, 1.0));
        inner.insert(StylePart::Text, (PaletteKey::ThemeBaseText, 1.0));
        default_checked_overrides.insert(WidgetStatus::Disabled, inner);
        
        let cust_color = custom_pal.palette;
        let theme_color = theme.palette().background;

        let custom_statuses: HashMap<(WidgetStatus, StateVariant), HashMap<StylePart, (PaletteKey, f32)>> =
            if let Some(status) = custom_pal.statuses.as_ref() {
                status
                    .iter()
                    .map(|((widget_status, variant), parts)| {
                        let part_map: HashMap<StylePart, (PaletteKey, f32)> = parts
                            .iter()
                            .map(|(style_part, palette_key, alpha)| {
                                (style_part.clone(), (palette_key.clone(), *alpha))
                            })
                            .collect();
                        ((widget_status.clone(), *variant), part_map)
                    })
                    .collect()
            } else {
                HashMap::new()
            };

        let mut resolved_statuses: HashMap<(WidgetStatus, StateVariant), HashMap<StylePart, (PaletteKey, f32)>> = HashMap::new();

        for ws in [WidgetStatus::Active, WidgetStatus::Hovered, WidgetStatus::Disabled] {
            let mut unchecked_parts = default_unchecked_statuses.get(&ws).unwrap().clone();

            if let Some(custom_unchecked) = custom_statuses.get(&(ws.clone(), StateVariant::Unchecked)) {
                for (part, pair) in custom_unchecked {
                    unchecked_parts.insert(part.clone(), pair.clone());
                }
            }

            resolved_statuses.insert((ws.clone(), StateVariant::Unchecked), unchecked_parts.clone());

            let mut checked_parts = unchecked_parts;

            if let Some(per_status_checked) = default_checked_overrides.get(&ws) {
                for (part, pair) in per_status_checked {
                    checked_parts.insert(part.clone(), pair.clone());
                }
            }

            // Backward compatibility: IsChecked acts as a checked override map.
            if let Some(checked_override) = custom_statuses
                .get(&(WidgetStatus::IsChecked, StateVariant::Checked))
                .or_else(|| custom_statuses.get(&(WidgetStatus::IsChecked, StateVariant::Unchecked)))
            {
                for (part, pair) in checked_override {
                    checked_parts.insert(part.clone(), pair.clone());
                }
            }

            if let Some(custom_checked) = custom_statuses.get(&(ws.clone(), StateVariant::Checked)) {
                for (part, pair) in custom_checked {
                    checked_parts.insert(part.clone(), pair.clone());
                }
            }

            resolved_statuses.insert((ws, StateVariant::Checked), checked_parts);
        }

        let (widget_status, variant) = match status {
            checkbox::Status::Active { is_checked } => {
                (WidgetStatus::Active, if is_checked { StateVariant::Checked } else { StateVariant::Unchecked })
            },
            checkbox::Status::Hovered { is_checked } => {
                (WidgetStatus::Hovered, if is_checked { StateVariant::Checked } else { StateVariant::Unchecked })
            },
            checkbox::Status::Disabled { is_checked } => {
                (WidgetStatus::Disabled, if is_checked { StateVariant::Checked } else { StateVariant::Unchecked })
            },
        };

        let resolve_status_parts = |ws: WidgetStatus, sv: StateVariant| {
            let parts = resolved_statuses.get(&(ws, sv)).unwrap();
            let (border_pal, bd_alpha) = parts.get(&StylePart::Border).unwrap();
            let (base_pal, base_alpha) = parts.get(&StylePart::Background).unwrap();
            let (icon_pal, ic_alpha) = parts.get(&StylePart::Icon).unwrap();
            let (text_pal, txt_alpha) = parts.get(&StylePart::Text).unwrap();

            let border_color = border_pal.pal_key_to_color(&theme_color, &cust_color).scale_alpha(*bd_alpha);
            let base_color = base_pal.pal_key_to_color(&theme_color, &cust_color).scale_alpha(*base_alpha);
            let icon_color = icon_pal.pal_key_to_color(&theme_color, &cust_color).scale_alpha(*ic_alpha);
            let status_text_color = text_pal.pal_key_to_color(&theme_color, &cust_color).scale_alpha(*txt_alpha);

            (border_color, base_color, icon_color, status_text_color)
        };

        let (border_color, background_color, icon_color, status_text_color) = resolve_status_parts(widget_status, variant);

        let text_color = Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha)
            .unwrap_or(status_text_color);

        let mut border = iced::Border {
            radius: 2.0.into(),
            width: 1.0,
            color: border_color,
        };

        apply_border_overrides(
            &mut border,
            None,
            &self.border_radius,
            self.border_width,
            "Checkbox",
        );

        checkbox::Style {
            background: Background::Color(background_color),
            icon_color,
            border,
            text_color: Some(text_color),
        }

        
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
