//! Input float display fields that can be filled with a float.
#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::theme::palette::{self, Background};
use iced::widget;
use iced::widget::text_input;
use iced::widget::text_input::{Status, Style};
use iced::{Border, Element, Length, Theme, alignment};

use pyo3::pyclass;
use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;

use crate::IpgState;
use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::helpers::{get_len, get_padding};
use crate::state::Widgets;
use crate::widgets::callbacks::{CallbackName, invoke_callback, invoke_callback_with_args};
use crate::widgets::widget_param_update::{WidgetParamUpdate, extract_param, set_t_value};

#[derive(Debug, Clone)]
pub struct InputFloat {
    pub id: usize,
    pub value: String,
    pub placeholder: Option<String>,
    pub left_side: Option<bool>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub padding: Option<Vec<f32>>,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub align_left: Option<bool>,
    pub align_center: Option<bool>,
    pub align_right: Option<bool>,
    pub text_font_id: Option<usize>,
    pub style_id: Option<usize>,
    pub show: bool,
}

impl InputFloat {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &self,
        content: Vec<Element<'a, Message>>,
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message>> {
        
        if !self.show {
            return None;
        }

        let style_opt = self
            .lookup(widgets, self.style_id)
            .and_then(Widgets::as_input_float_style)
            .cloned();
        let wd = get_len(None, self.width_fill, self.width);

        let width = if wd == Length::Shrink { Length::Fill } else { wd };

        let pd = get_padding(&self.padding);

        let padding = if pd.left == 0.0 { pd.left(2.0) } else { pd };

        let placeholder = if let Some(ph) = &self.placeholder {
            ph.clone()
        } else {
            String::new()
        };

        let txt_input: widget::TextInput<'_, InputFloatMessage> =
            widget::TextInput::new(placeholder, self.value.as_str())
                .on_input(InputFloatMessage::OnInput)
                .on_submit(InputFloatMessage::OnSubmit(self.value.clone()))
                .on_paste(InputFloatMessage::OnPaste)
                .width(width)
                .padding(padding)
                .style(move |theme: &Theme, status| {
                    if let Some(ti) = &style_opt {
                        ti.to_iced(theme, status)
                    } else {
                        text_input::default(theme, status)
                    }
                });

        let txt_input = if let Some(sz) = self.size {
            txt_input.size(sz)
        } else {
            txt_input
        };

        let txt_input = if let Some(lh) = self.line_height {
            txt_input.line_height(lh)
        } else {
            txt_input
        };

        // default
        let txt_input = txt_input.align_x(alignment::Horizontal::Left);

        let txt_input = if self.align_center == Some(true) {
            txt_input.align_x(alignment::Horizontal::Center)
        } else {
            txt_input
        };

        let txt_input = if self.align_right == Some(true) {
            txt_input.align_x(alignment::Horizontal::Right)
        } else {
            txt_input
        };

        let txt_input: Element<'_, InputFloatMessage> = txt_input.into();

        let ti: Element<'a, Message> = txt_input.map(move |message| Message::InputFloat(self.id, message));

        let cnt = if self.left_side == Some(true) {
            let mut cnt = content;
            cnt.extend(vec![ti]);
            cnt
        } else {
            let mut cnt = vec![ti];
            cnt.extend(content);
            cnt
        };

        Some(iced::widget::Row::with_children(cnt).into())
    }
}

pub fn input_float_callback(state: &mut IpgState, id: usize, message: InputFloatMessage) {
    // During the input, the widget is assigned the value so that it shows
    // during typing.  On submit, the text box is cleared, so no value.
    // However, in both cases the value is passed to the callback.

    match message {
        InputFloatMessage::OnInput(value) => {
            if let Some(Widgets::InputInt(ii)) = state.widgets.get_mut(&id) {
                if value.parse::<i64>().is_ok() {
                    ii.value = value.clone();
                } else {
                    println!("ERROR: add_input_int value '{}' not an integer", value);
                }
            }
            invoke_callback_with_args(
                id,
                CallbackName::OnInput,
                "InputInt",
                value.to_string(),
                "def cb(wid: int, value: str)",
            );
        }
        InputFloatMessage::OnSubmit(value) => {
            if let Some(Widgets::InputInt(ii)) = state.widgets.get_mut(&id) {
                ii.value = String::new();
            }
            invoke_callback_with_args(
                id,
                CallbackName::OnSubmit,
                "InputInt",
                value,
                "def cb(wid: int, value: str)",
            );
        }
        InputFloatMessage::OnPaste(value) => {
            if let Some(Widgets::InputInt(ii)) = state.widgets.get_mut(&id) {
                ii.value = value.clone();
            }
            invoke_callback_with_args(
                id,
                CallbackName::OnPaste,
                "InputInt",
                value,
                "def cb(wid: int, value: str)",
            );
        }
        InputFloatMessage::OnPress => {
            invoke_callback(id, CallbackName::OnPress, "InputIntButton");
        }
    }
}

#[derive(Debug, Clone)]
pub enum InputFloatMessage {
    OnInput(String),
    OnSubmit(String),
    OnPaste(String),
    OnPress,
}

#[derive(Debug, Clone)]
pub struct InputFloatStyle {
    pub id: usize,

    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,

    pub text_color: Option<Color>,
    pub text_color_alpha: Option<f32>,
    pub text_rgba: Option<[f32; 4]>,

    pub primary_color: Option<Color>,
    pub primary_color_alpha: Option<f32>,
    pub primary_rgba: Option<[f32; 4]>,

    pub secondary_color: Option<Color>,
    pub secondary_color_alpha: Option<f32>,
    pub secondary_rgba: Option<[f32; 4]>,

    pub border_color_active: Option<Color>,
    pub border_color_alpha_active: Option<f32>,
    pub border_rgba_active: Option<[f32; 4]>,

    pub border_color_hovered: Option<Color>,
    pub border_color_alpha_hovered: Option<f32>,
    pub border_rgba_hovered: Option<[f32; 4]>,

    pub border_color_focused: Option<Color>,
    pub border_color_alpha_focused: Option<f32>,
    pub border_rgba_focused: Option<[f32; 4]>,

    pub border_color_disabled: Option<Color>,
    pub border_color_alpha_disabled: Option<f32>,
    pub border_rgba_disabled: Option<[f32; 4]>,

    pub border_width: Option<f32>,
    pub border_radius: Option<f32>,

    pub placeholder_color_active: Option<Color>,
    pub placeholder_color_alpha_active: Option<f32>,
    pub placeholder_rgba_active: Option<[f32; 4]>,

    pub placeholder_color_hovered: Option<Color>,
    pub placeholder_color_alpha_hovered: Option<f32>,
    pub placeholder_rgba_hovered: Option<[f32; 4]>,

    pub placeholder_color_focused: Option<Color>,
    pub placeholder_color_alpha_focused: Option<f32>,
    pub placeholder_rgba_focused: Option<[f32; 4]>,

    pub placeholder_color_disabled: Option<Color>,
    pub placeholder_color_alpha_disabled: Option<f32>,
    pub placeholder_rgba_disabled: Option<[f32; 4]>,

    pub value_color_active: Option<Color>,
    pub value_color_alpha_active: Option<f32>,
    pub value_rgba_active: Option<[f32; 4]>,

    pub value_color_hovered: Option<Color>,
    pub value_color_alpha_hovered: Option<f32>,
    pub value_rgba_hovered: Option<[f32; 4]>,

    pub value_color_focused: Option<Color>,
    pub value_color_alpha_focused: Option<f32>,
    pub value_rgba_focused: Option<[f32; 4]>,

    pub value_color_disabled: Option<Color>,
    pub value_color_alpha_disabled: Option<f32>,
    pub value_rgba_disabled: Option<[f32; 4]>,

    pub selection_color_active: Option<Color>,
    pub selection_color_alpha_active: Option<f32>,
    pub selection_rgba_active: Option<[f32; 4]>,

    pub selection_color_hovered: Option<Color>,
    pub selection_color_alpha_hovered: Option<f32>,
    pub selection_rgba_hovered: Option<[f32; 4]>,

    pub selection_color_focused: Option<Color>,
    pub selection_color_alpha_focused: Option<f32>,
    pub selection_rgba_focused: Option<[f32; 4]>,

    pub selection_color_disabled: Option<Color>,
    pub selection_color_alpha_disabled: Option<f32>,
    pub selection_rgba_disabled: Option<[f32; 4]>,
}

impl InputFloatStyle {
    fn to_iced(&self, theme: &Theme, status: Status) -> Style {
        let background_color = Color::rgba_ipg_color_to_iced(
            self.background_rgba,
            &self.background_color,
            self.background_color_alpha,
        );

        let text_color = Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha);

        let primary_color =
            Color::rgba_ipg_color_to_iced(self.primary_rgba, &self.primary_color, self.primary_color_alpha);

        let secondary_color =
            Color::rgba_ipg_color_to_iced(self.secondary_rgba, &self.secondary_color, self.secondary_color_alpha);

        let border_color_active = Color::rgba_ipg_color_to_iced(
            self.border_rgba_active,
            &self.border_color_active,
            self.border_color_alpha_active,
        );
        let border_color_hovered = Color::rgba_ipg_color_to_iced(
            self.border_rgba_hovered,
            &self.border_color_hovered,
            self.border_color_alpha_hovered,
        );
        let border_color_focused = Color::rgba_ipg_color_to_iced(
            self.border_rgba_focused,
            &self.border_color_focused,
            self.border_color_alpha_focused,
        );
        let border_color_disabled = Color::rgba_ipg_color_to_iced(
            self.border_rgba_disabled,
            &self.border_color_disabled,
            self.border_color_alpha_disabled,
        );

        let placeholder_color_active = Color::rgba_ipg_color_to_iced(
            self.placeholder_rgba_active,
            &self.placeholder_color_active,
            self.placeholder_color_alpha_active,
        );
        let placeholder_color_hovered = Color::rgba_ipg_color_to_iced(
            self.placeholder_rgba_hovered,
            &self.placeholder_color_hovered,
            self.placeholder_color_alpha_hovered,
        );
        let placeholder_color_focused = Color::rgba_ipg_color_to_iced(
            self.placeholder_rgba_focused,
            &self.placeholder_color_focused,
            self.placeholder_color_alpha_focused,
        );
        let placeholder_color_disabled = Color::rgba_ipg_color_to_iced(
            self.placeholder_rgba_disabled,
            &self.placeholder_color_disabled,
            self.placeholder_color_alpha_disabled,
        );

        let value_color_active = Color::rgba_ipg_color_to_iced(
            self.value_rgba_active,
            &self.value_color_active,
            self.value_color_alpha_active,
        );
        let value_color_hovered = Color::rgba_ipg_color_to_iced(
            self.value_rgba_hovered,
            &self.value_color_hovered,
            self.value_color_alpha_hovered,
        );
        let value_color_focused = Color::rgba_ipg_color_to_iced(
            self.value_rgba_focused,
            &self.value_color_focused,
            self.value_color_alpha_focused,
        );
        let value_color_disabled = Color::rgba_ipg_color_to_iced(
            self.value_rgba_disabled,
            &self.value_color_disabled,
            self.value_color_alpha_disabled,
        );

        let selection_color_active = Color::rgba_ipg_color_to_iced(
            self.selection_rgba_active,
            &self.selection_color_active,
            self.selection_color_alpha_active,
        );
        let selection_color_hovered = Color::rgba_ipg_color_to_iced(
            self.selection_rgba_hovered,
            &self.selection_color_hovered,
            self.selection_color_alpha_hovered,
        );
        let selection_color_focused = Color::rgba_ipg_color_to_iced(
            self.selection_rgba_focused,
            &self.selection_color_focused,
            self.selection_color_alpha_focused,
        );
        let selection_color_disabled = Color::rgba_ipg_color_to_iced(
            self.selection_rgba_disabled,
            &self.selection_color_disabled,
            self.selection_color_alpha_disabled,
        );

        // border
        let br = self.border_radius.unwrap_or(2.0);

        let bw = self.border_width.unwrap_or(1.0);

        let palette = theme.palette();

        // One can use the theme text color but the background and primary
        // are needed together to produce the correct colors
        let txt_color = if let Some(c) = text_color {
            c
        } else {
            theme.palette().background.base.text
        };

        let background_opt = background_color.map(|bkg| Background::new(bkg, txt_color));

        let pm_swatch_opt = primary_color.map(|c| palette::Swatch::derive(c, txt_color));

        let sec_swatch_opt = secondary_color.map(|c| palette::Swatch::derive(c, txt_color));

        let new_theme = background_opt.is_some() && pm_swatch_opt.is_some() && sec_swatch_opt.is_some();

        let bkg_base_color = if new_theme {
            background_opt.unwrap().base.color
        } else {
            palette.background.base.color
        };

        // border color
        let (bc_active, bc_hovered, bc_focused, bc_disabled) = if new_theme {
            let background = background_opt.unwrap();
            let primary = pm_swatch_opt.unwrap();
            (
                background.strong.color,
                background.base.text,
                primary.strong.color,
                background.base.text,
            )
        } else {
            (
                border_color_active.unwrap_or(palette.background.strong.color),
                border_color_hovered
                    .or(border_color_active)
                    .unwrap_or(palette.background.base.text),
                border_color_focused
                    .or(border_color_active)
                    .unwrap_or(palette.primary.strong.color),
                border_color_disabled
                    .or(border_color_active)
                    .unwrap_or(palette.background.strong.color),
            )
        };

        // placeholder
        let (ph_active, ph_hovered, ph_focused, ph_disabled) = if new_theme {
            let c = sec_swatch_opt.unwrap().base.color;
            (c, c, c, background_opt.unwrap().strongest.color)
        } else {
            let base = placeholder_color_active.unwrap_or(palette.secondary.base.color);
            (
                base,
                placeholder_color_hovered.unwrap_or(base),
                placeholder_color_focused.unwrap_or(base),
                placeholder_color_disabled
                    .or(placeholder_color_active)
                    .unwrap_or(palette.background.strongest.color),
            )
        };

        // value
        let (val_active, val_hovered, val_focused, val_disabled) = if new_theme {
            let c = background_opt.unwrap().base.text;
            (c, c, c, c)
        } else {
            let base = value_color_active.unwrap_or(palette.background.base.text);
            (
                base,
                value_color_hovered.unwrap_or(base),
                value_color_focused.unwrap_or(base),
                value_color_disabled.unwrap_or(base),
            )
        };

        // selection
        let (sel_active, sel_hovered, sel_focused, sel_disabled) = if new_theme {
            let c = pm_swatch_opt.unwrap().weak.color;
            (c, c, c, c)
        } else {
            let base = selection_color_active.unwrap_or(palette.primary.weak.color);
            (
                base,
                selection_color_hovered.unwrap_or(base),
                selection_color_focused.unwrap_or(base),
                selection_color_disabled.unwrap_or(base),
            )
        };

        let active = Style {
            background: bkg_base_color.into(),
            border: Border {
                radius: br.into(),
                width: bw,
                color: bc_active,
            },
            placeholder: ph_active,
            value: val_active,
            selection: sel_active,
        };

        let hovered = Style {
            background: bkg_base_color.into(),
            border: Border {
                radius: br.into(),
                width: bw,
                color: bc_hovered,
            },
            placeholder: ph_hovered,
            value: val_hovered,
            selection: sel_hovered,
        };

        let focused = Style {
            background: bkg_base_color.into(),
            border: Border {
                radius: br.into(),
                width: bw,
                color: bc_focused,
            },
            placeholder: ph_focused,
            value: val_focused,
            selection: sel_focused,
        };

        let disabled = Style {
            background: bkg_base_color.into(),
            border: Border {
                radius: br.into(),
                width: bw,
                color: bc_disabled,
            },
            placeholder: ph_disabled,
            value: val_disabled,
            selection: sel_disabled,
        };

        match status {
            Status::Active => active,
            Status::Hovered => hovered,
            Status::Focused { .. } => focused,
            Status::Disabled => disabled,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum InputFloatParam {
    LeftSide,
    LineHeight,
    Padding,
    Placeholder,
    Show,
    Size,
    StyleId,
    Value,
    Width,
    WidthFill,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum InputFloatStyleParam {
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRgba,

    TextColor,
    TextColorAlpha,
    TextRgba,

    PrimaryColor,
    PrimaryColorAlpha,
    PrimaryRgba,

    SecondaryColor,
    SecondaryColorAlpha,
    SecondaryRgba,

    BorderColorActive,
    BorderColorAlphaActive,
    BorderRgbaActive,

    BorderColorHovered,
    BorderColorAlphaHovered,
    BorderRgbaHovered,

    BorderColorFocused,
    BorderColorAlphaFocused,
    BorderRgbaFocused,

    BorderColorDisabled,
    BorderColorAlphaDisabled,
    BorderRgbaDisabled,

    BorderWidth,
    BorderRadius,

    PlaceholderColorActive,
    PlaceholderColorAlphaActive,
    PlaceholderRgbaActive,

    PlaceholderColorHovered,
    PlaceholderColorAlphaHovered,
    PlaceholderRgbaHovered,

    PlaceholderColorFocused,
    PlaceholderColorAlphaFocused,
    PlaceholderRgbaFocused,

    PlaceholderColorDisabled,
    PlaceholderColorAlphaDisabled,
    PlaceholderRgbaDisabled,

    ValueColorActive,
    ValueColorAlphaActive,
    ValueRgbaActive,

    ValueColorHovered,
    ValueColorAlphaHovered,
    ValueRgbaHovered,

    ValueColorFocused,
    ValueColorAlphaFocused,
    ValueRgbaFocused,

    ValueColorDisabled,
    ValueColorAlphaDisabled,
    ValueRgbaDisabled,

    SelectionColorActive,
    SelectionColorAlphaActive,
    SelectionRgbaActive,

    SelectionColorHovered,
    SelectionColorAlphaHovered,
    SelectionRgbaHovered,

    SelectionColorFocused,
    SelectionColorAlphaFocused,
    SelectionRgbaFocused,

    SelectionColorDisabled,
    SelectionColorAlphaDisabled,
    SelectionRgbaDisabled,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for InputFloat {
    type Param = InputFloatParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            InputFloatParam::LeftSide => set_t_value(&mut self.left_side, value, "InputFloatParam::LeftSide"),
            InputFloatParam::LineHeight => set_t_value(&mut self.line_height, value, "InputIntParam::LineHeight"),
            InputFloatParam::Padding => set_t_value(&mut self.padding, value, "InputIntParam::Padding"),
            InputFloatParam::Placeholder => set_t_value(&mut self.placeholder, value, "InputIntParam::Placeholder"),
            InputFloatParam::Show => set_t_value(&mut self.show, value, "InputIntParam::Show"),
            InputFloatParam::Size => set_t_value(&mut self.size, value, "InputIntParam::Size"),
            InputFloatParam::StyleId => set_t_value(&mut self.style_id, value, "InputIntParam::StyleId"),
            InputFloatParam::Value => {
                let v: f64 = extract_param(value);
                self.value = v.to_string();
            }
            InputFloatParam::Width => set_t_value(&mut self.width, value, "InputIntParam::Width"),
            InputFloatParam::WidthFill => set_t_value(&mut self.width_fill, value, "TextInputParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for InputFloatStyle {
    type Param = InputFloatStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            InputFloatStyleParam::BackgroundColor => {
                set_t_value(&mut self.background_color, value, "InputIntStyleParam::BackgroundColor")
            }
            InputFloatStyleParam::BackgroundColorAlpha => set_t_value(
                &mut self.background_color_alpha,
                value,
                "InputIntStyleParam::BackgroundColorAlpha",
            ),
            InputFloatStyleParam::BackgroundRgba => {
                set_t_value(&mut self.background_rgba, value, "InputIntStyleParam::BackgroundRgba")
            }
            InputFloatStyleParam::TextColor => {
                set_t_value(&mut self.text_color, value, "InputIntStyleParam::TextColor")
            }
            InputFloatStyleParam::TextColorAlpha => {
                set_t_value(&mut self.text_color_alpha, value, "InputIntStyleParam::TextColorAlpha")
            }
            InputFloatStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "InputIntStyleParam::TextRgba"),
            InputFloatStyleParam::PrimaryColor => {
                set_t_value(&mut self.primary_color, value, "InputIntStyleParam::PrimaryColor")
            }
            InputFloatStyleParam::PrimaryColorAlpha => set_t_value(
                &mut self.primary_color_alpha,
                value,
                "InputIntStyleParam::PrimaryColorAlpha",
            ),
            InputFloatStyleParam::PrimaryRgba => {
                set_t_value(&mut self.primary_rgba, value, "InputIntStyleParam::PrimaryRgba")
            }
            InputFloatStyleParam::SecondaryColor => {
                set_t_value(&mut self.secondary_color, value, "InputIntStyleParam::SecondaryColor")
            }
            InputFloatStyleParam::SecondaryColorAlpha => set_t_value(
                &mut self.secondary_color_alpha,
                value,
                "InputIntStyleParam::SecondaryColorAlpha",
            ),
            InputFloatStyleParam::SecondaryRgba => {
                set_t_value(&mut self.secondary_rgba, value, "InputIntStyleParam::SecondaryRgba")
            }
            InputFloatStyleParam::BorderColorActive => set_t_value(
                &mut self.border_color_active,
                value,
                "InputIntStyleParam::BorderColorActive",
            ),
            InputFloatStyleParam::BorderColorAlphaActive => set_t_value(
                &mut self.border_color_alpha_active,
                value,
                "InputIntStyleParam::BorderColorAlphaActive",
            ),
            InputFloatStyleParam::BorderRgbaActive => set_t_value(
                &mut self.border_rgba_active,
                value,
                "InputIntStyleParam::BorderRgbaActive",
            ),
            InputFloatStyleParam::BorderColorHovered => set_t_value(
                &mut self.border_color_hovered,
                value,
                "InputIntStyleParam::BorderColorHovered",
            ),
            InputFloatStyleParam::BorderColorAlphaHovered => set_t_value(
                &mut self.border_color_alpha_hovered,
                value,
                "InputIntStyleParam::BorderColorAlphaHovered",
            ),
            InputFloatStyleParam::BorderRgbaHovered => set_t_value(
                &mut self.border_rgba_hovered,
                value,
                "InputIntStyleParam::BorderRgbaHovered",
            ),
            InputFloatStyleParam::BorderColorFocused => set_t_value(
                &mut self.border_color_focused,
                value,
                "InputIntStyleParam::BorderColorFocused",
            ),
            InputFloatStyleParam::BorderColorAlphaFocused => set_t_value(
                &mut self.border_color_alpha_focused,
                value,
                "InputIntStyleParam::BorderColorAlphaFocused",
            ),
            InputFloatStyleParam::BorderRgbaFocused => set_t_value(
                &mut self.border_rgba_focused,
                value,
                "InputIntStyleParam::BorderRgbaFocused",
            ),
            InputFloatStyleParam::BorderColorDisabled => set_t_value(
                &mut self.border_color_disabled,
                value,
                "InputIntStyleParam::BorderColorDisabled",
            ),
            InputFloatStyleParam::BorderColorAlphaDisabled => set_t_value(
                &mut self.border_color_alpha_disabled,
                value,
                "InputIntStyleParam::BorderColorAlphaDisabled",
            ),
            InputFloatStyleParam::BorderRgbaDisabled => set_t_value(
                &mut self.border_rgba_disabled,
                value,
                "InputIntStyleParam::BorderRgbaDisabled",
            ),
            InputFloatStyleParam::BorderWidth => {
                set_t_value(&mut self.border_width, value, "InputIntStyleParam::BorderWidth")
            }
            InputFloatStyleParam::BorderRadius => {
                set_t_value(&mut self.border_radius, value, "InputIntStyleParam::BorderRadius")
            }
            InputFloatStyleParam::PlaceholderColorActive => set_t_value(
                &mut self.placeholder_color_active,
                value,
                "InputIntStyleParam::PlaceholderColorActive",
            ),
            InputFloatStyleParam::PlaceholderColorAlphaActive => set_t_value(
                &mut self.placeholder_color_alpha_active,
                value,
                "InputIntStyleParam::PlaceholderColorAlphaActive",
            ),
            InputFloatStyleParam::PlaceholderRgbaActive => set_t_value(
                &mut self.placeholder_rgba_active,
                value,
                "InputIntStyleParam::PlaceholderRgbaActive",
            ),
            InputFloatStyleParam::PlaceholderColorHovered => set_t_value(
                &mut self.placeholder_color_hovered,
                value,
                "InputIntStyleParam::PlaceholderColorHovered",
            ),
            InputFloatStyleParam::PlaceholderColorAlphaHovered => set_t_value(
                &mut self.placeholder_color_alpha_hovered,
                value,
                "InputIntStyleParam::PlaceholderColorAlphaHovered",
            ),
            InputFloatStyleParam::PlaceholderRgbaHovered => set_t_value(
                &mut self.placeholder_rgba_hovered,
                value,
                "InputIntStyleParam::PlaceholderRgbaHovered",
            ),
            InputFloatStyleParam::PlaceholderColorFocused => set_t_value(
                &mut self.placeholder_color_focused,
                value,
                "InputIntStyleParam::PlaceholderColorFocused",
            ),
            InputFloatStyleParam::PlaceholderColorAlphaFocused => set_t_value(
                &mut self.placeholder_color_alpha_focused,
                value,
                "InputIntStyleParam::PlaceholderColorAlphaFocused",
            ),
            InputFloatStyleParam::PlaceholderRgbaFocused => set_t_value(
                &mut self.placeholder_rgba_focused,
                value,
                "InputIntStyleParam::PlaceholderRgbaFocused",
            ),
            InputFloatStyleParam::PlaceholderColorDisabled => set_t_value(
                &mut self.placeholder_color_disabled,
                value,
                "InputIntStyleParam::PlaceholderColorDisabled",
            ),
            InputFloatStyleParam::PlaceholderColorAlphaDisabled => set_t_value(
                &mut self.placeholder_color_alpha_disabled,
                value,
                "InputIntStyleParam::PlaceholderColorAlphaDisabled",
            ),
            InputFloatStyleParam::PlaceholderRgbaDisabled => set_t_value(
                &mut self.placeholder_rgba_disabled,
                value,
                "InputIntStyleParam::PlaceholderRgbaDisabled",
            ),
            InputFloatStyleParam::ValueColorActive => set_t_value(
                &mut self.value_color_active,
                value,
                "InputIntStyleParam::ValueColorActive",
            ),
            InputFloatStyleParam::ValueColorAlphaActive => set_t_value(
                &mut self.value_color_alpha_active,
                value,
                "InputIntStyleParam::ValueColorAlphaActive",
            ),
            InputFloatStyleParam::ValueRgbaActive => set_t_value(
                &mut self.value_rgba_active,
                value,
                "InputIntStyleParam::ValueRgbaActive",
            ),
            InputFloatStyleParam::ValueColorHovered => set_t_value(
                &mut self.value_color_hovered,
                value,
                "InputIntStyleParam::ValueColorHovered",
            ),
            InputFloatStyleParam::ValueColorAlphaHovered => set_t_value(
                &mut self.value_color_alpha_hovered,
                value,
                "InputIntStyleParam::ValueColorAlphaHovered",
            ),
            InputFloatStyleParam::ValueRgbaHovered => set_t_value(
                &mut self.value_rgba_hovered,
                value,
                "InputIntStyleParam::ValueRgbaHovered",
            ),
            InputFloatStyleParam::ValueColorFocused => set_t_value(
                &mut self.value_color_focused,
                value,
                "InputIntStyleParam::ValueColorFocused",
            ),
            InputFloatStyleParam::ValueColorAlphaFocused => set_t_value(
                &mut self.value_color_alpha_focused,
                value,
                "InputIntStyleParam::ValueColorAlphaFocused",
            ),
            InputFloatStyleParam::ValueRgbaFocused => set_t_value(
                &mut self.value_rgba_focused,
                value,
                "InputIntStyleParam::ValueRgbaFocused",
            ),
            InputFloatStyleParam::ValueColorDisabled => set_t_value(
                &mut self.value_color_disabled,
                value,
                "InputIntStyleParam::ValueColorDisabled",
            ),
            InputFloatStyleParam::ValueColorAlphaDisabled => set_t_value(
                &mut self.value_color_alpha_disabled,
                value,
                "InputIntStyleParam::ValueColorAlphaDisabled",
            ),
            InputFloatStyleParam::ValueRgbaDisabled => set_t_value(
                &mut self.value_rgba_disabled,
                value,
                "InputIntStyleParam::ValueRgbaDisabled",
            ),
            InputFloatStyleParam::SelectionColorActive => set_t_value(
                &mut self.selection_color_active,
                value,
                "InputIntStyleParam::SelectionColorActive",
            ),
            InputFloatStyleParam::SelectionColorAlphaActive => set_t_value(
                &mut self.selection_color_alpha_active,
                value,
                "InputIntStyleParam::SelectionColorAlphaActive",
            ),
            InputFloatStyleParam::SelectionRgbaActive => set_t_value(
                &mut self.selection_rgba_active,
                value,
                "InputIntStyleParam::SelectionRgbaActive",
            ),
            InputFloatStyleParam::SelectionColorHovered => set_t_value(
                &mut self.selection_color_hovered,
                value,
                "InputIntStyleParam::SelectionColorHovered",
            ),
            InputFloatStyleParam::SelectionColorAlphaHovered => set_t_value(
                &mut self.selection_color_alpha_hovered,
                value,
                "InputIntStyleParam::SelectionColorAlphaHovered",
            ),
            InputFloatStyleParam::SelectionRgbaHovered => set_t_value(
                &mut self.selection_rgba_hovered,
                value,
                "InputIntStyleParam::SelectionRgbaHovered",
            ),
            InputFloatStyleParam::SelectionColorFocused => set_t_value(
                &mut self.selection_color_focused,
                value,
                "InputIntStyleParam::SelectionColorFocused",
            ),
            InputFloatStyleParam::SelectionColorAlphaFocused => set_t_value(
                &mut self.selection_color_alpha_focused,
                value,
                "InputIntStyleParam::SelectionColorAlphaFocused",
            ),
            InputFloatStyleParam::SelectionRgbaFocused => set_t_value(
                &mut self.selection_rgba_focused,
                value,
                "InputIntStyleParam::SelectionRgbaFocused",
            ),
            InputFloatStyleParam::SelectionColorDisabled => set_t_value(
                &mut self.selection_color_disabled,
                value,
                "InputIntStyleParam::SelectionColorDisabled",
            ),
            InputFloatStyleParam::SelectionColorAlphaDisabled => set_t_value(
                &mut self.selection_color_alpha_disabled,
                value,
                "InputIntStyleParam::SelectionColorAlphaDisabled",
            ),
            InputFloatStyleParam::SelectionRgbaDisabled => set_t_value(
                &mut self.selection_rgba_disabled,
                value,
                "InputIntStyleParam::SelectionRgbaDisabled",
            ),
        }
    }
}
