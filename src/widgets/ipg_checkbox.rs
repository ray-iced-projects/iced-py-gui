//! ipg_checkbox

use std::collections::HashMap;

use crate::graphics::colors::Color;
use crate::py_api::helpers::get_len;
use crate::state::IpgState;
use crate::app::Message;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value};
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::state::Widgets;

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap_icon::{Icon, icon_to_char};

use crate::widgets::styling::{apply_border_overrides, create_custom_theme};

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
    pub parent_id: String,
    pub show: bool,
    pub is_checked: bool,
    pub label: Option<String>,
    pub width: Option<f32>,
    pub fill: Option<bool>,
    pub size: Option<f32>,
    pub spacing: Option<f32>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
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
}

impl CheckBox {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message>> {

        if !self.show {
            return None
        };

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_checkbox_style).cloned();

        // Icon related
        let code_point = 
            if let Some(ic) = self.icon {
                icon_to_char(ic)
            } else {
                icon_to_char(Icon::Check)
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
        let text_line_height = 
            if let Some(lh) = self.text_line_height {
                text::LineHeight::Relative(lh)
            } else { text::LineHeight::default() };

        let chk = 
            Checkbox::new(self.is_checked)
                .on_toggle(ChkMessage::OnToggle)
                .width(get_len(self.fill, None, self.width))
                .text_line_height(text_line_height)
                .icon(icon)
                .style(move|theme: &Theme, status| {   
                    if let Some(st) = &style_opt {
                        let is_label = if self.label.is_some() { true } else { false };
                        st.to_iced(theme, status, &self.style_std, is_label)
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
                chk.text_wrapping(Wrapping::None)
            } else if self.text_wrapping_glyph.is_some() {
                chk.text_wrapping(Wrapping::Glyph)
            } else if self.text_wrapping_word_glyph.is_some() {
                chk.text_wrapping(Wrapping::WordOrGlyph)
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
            invoke_callback_with_args(id, "on_toggle", "Checkbox", is_checked);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CheckboxStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub icon_color: Option<Color>,
    pub icon_color_alpha: Option<f32>,
    pub icon_rgba: Option<[f32; 4]>,
    pub text_color: Option<Color>,
    pub text_color_alpha: Option<f32>,
    pub text_rgba: Option<[f32; 4]>,
}

impl CheckboxStyle {
    fn to_iced(
         &self, 
        theme: &Theme, 
        status: checkbox::Status,
        std_style_opt: &Option<CheckboxStyleStd>,
        is_label: bool,
        ) -> checkbox::Style {

        let mut style = if let Some(std) = 
            std_style_opt {
                std.to_iced(theme, status)
            } else { checkbox::primary(theme, status) };
        
        let background_color = 
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);

        let border_color = 
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);

        let icon_color = 
            Color::rgba_ipg_color_to_iced(self.icon_rgba, &self.icon_color, self.icon_color_alpha);
        
        if let Some(ic) = icon_color {
            style.icon_color = ic;
        }

        style.text_color = Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha);
    
        // custom style only depends on bkg and border color
        if background_color.is_none() && border_color.is_none() {
            return style
        }

        // if given a border_color then use border_color as is
        // if given a bkg_color then use bkg_color as is
        // the accent will be paired regardless

        let custom_theme;
        let text_color;

        let palette = if let Some(bkg) = background_color {
            let dark_mode = palette::is_dark(bkg);
            custom_theme = create_custom_theme(bkg, dark_mode);
            text_color = custom_theme.palette().text;
            custom_theme.extended_palette()
        } else {
            text_color = theme.palette().text;
            theme.extended_palette()
        };

        let mut style = match status {
            checkbox::Status::Active { is_checked } => styled(
                palette.background.strong.color,
                palette.background.base,
                palette.primary.base.text,
                palette.primary.base,
                is_checked,
            ),
            checkbox::Status::Hovered { is_checked } => styled(
                palette.background.strong.color,
                palette.background.weak,
                palette.primary.base.text,
                palette.primary.strong,
                is_checked,
            ),
            checkbox::Status::Disabled { is_checked } => styled(
                palette.background.weak.color,
                palette.background.weaker,
                palette.primary.base.text,
                palette.background.strong,
                is_checked,
            ),
        };

        apply_border_overrides(
            &mut style.border, border_color,
            &self.border_radius, self.border_width, "Checkbox",
        );

        if is_label && background_color.is_some() {
            let color = palette::readable(background_color.unwrap(), text_color);
            style.text_color = Some(color);
        }

        style
        
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
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRgba,
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
    BorderRadius,
    BorderWidth,
    IconColor,
    IconColorAlpha,
    IconRgba,
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
            CheckboxParam::TextFontId => set_t_value(&mut self.text_font_id, value, "CheckboxParam::TextFontId"),
            CheckboxParam::TextLineHeight => set_t_value(&mut self.text_line_height, value, "CheckboxParam::TextLineHeight"),
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
            CheckboxStyleParam::BackgroundColor => set_t_value(&mut self.background_color, value, "CheckboxStyleParam::BackgroundColor"),
            CheckboxStyleParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "CheckboxStyleParam::BackgroundColorAlpha"),
            CheckboxStyleParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "CheckboxStyleParam::BackgroundRgbaColor"),
            CheckboxStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "CheckboxStyleParam::BorderColor"),
            CheckboxStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "CheckboxStyleParam::BorderColorAlpha"),
            CheckboxStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "CheckboxStyleParam::BorderRadius"),
            CheckboxStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "CheckboxStyleParam::BorderRgbaColor"),
            CheckboxStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "CheckboxStyleParam::BorderWidth"),
            CheckboxStyleParam::IconColor => set_t_value(&mut self.icon_color, value, "CheckboxStyleParam::IconColor"),
            CheckboxStyleParam::IconColorAlpha => set_t_value(&mut self.icon_color_alpha, value, "CheckboxStyleParam::IconColorAlpha"),
            CheckboxStyleParam::IconRgba => set_t_value(&mut self.icon_rgba, value, "CheckboxStyleParam::IconRgbaColor"),
            CheckboxStyleParam::TextColor => set_t_value(&mut self.text_color, value, "CheckboxStyleParam::TextColor"),
            CheckboxStyleParam::TextColorAlpha => set_t_value(&mut self.text_color_alpha, value, "CheckboxStyleParam::TextColorAlpha"),
            CheckboxStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "CheckboxStyleParam::TextRgbaColor"),
        }
    }
}
