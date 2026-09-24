//! Text inputs display fields that can be filled with text.
#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use crate::graphics::bootstrap::bootstrap_arrow::Arrow;
use crate::graphics::bootstrap::bootstrap_icon::Icon;
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
pub struct InputInt {
    pub id: usize,
    pub parent_id: String,
    pub placeholder: String,
    pub value: String,
    pub icons: Option<Vec<Icon>>,
    pub arrows: Option<Vec<Arrow>>,
    pub button_outline: Option<bool>,
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

impl InputInt {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(&'a self, widgets: &HashMap<usize, Widgets>) -> Option<Element<'a, Message>> {
        if !self.show {
            return None;
        }

        let style_opt = self
            .lookup(widgets, self.style_id)
            .and_then(Widgets::as_input_int_style)
            .cloned();

        let wd = get_len(None, self.width_fill, self.width);

        let width = if wd == Length::Shrink { Length::Fill } else { wd };

        let pd = get_padding(&self.padding);

        let padding = if pd.left == 0.0 { pd.left(2.0) } else { pd };

        let txt_input: widget::TextInput<'_, InputIntMessage> =
            widget::TextInput::new(self.placeholder.as_str(), self.value.as_str())
                .on_input(InputIntMessage::OnInput)
                .on_submit(InputIntMessage::OnSubmit(self.value.clone()))
                .on_paste(InputIntMessage::OnPaste)
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

        let txt_input: Element<'_, InputIntMessage> = txt_input.into();

        let ti: Element<'a, Message> = txt_input.map(move |message| Message::InputInt(self.id, message));

        let arrows = if let Some(arrows) = &self.arrows {
            let mut many_arrows = vec![];
            for ar in arrows {
                let ar = ar.to_char().to_string();
                let txt = iced::widget::text(ar).font(iced::Font::new("bootstrap-icons")).size(13.5);
                many_arrows.push(txt);
            }
            many_arrows
        } else {
            vec![]
        };

        let btns = {
            let mut buttons = vec![];
            for arrow in arrows {
                buttons.push(
                    iced::widget::button(arrow)
                        .on_press(Message::InputInt(self.id, InputIntMessage::OnPress))
                        .padding(0.0)
                        .style(move |theme: &Theme, status| {
                            if self.button_outline == Some(true) {
                                iced::widget::button::primary(theme, status)
                            } else {
                                iced::widget::button::text(theme, status)
                            }
                        })
                        .into(),
                )
            }
            buttons
        };

        let mut content = vec![ti];
        content.extend(btns);

        Some(iced::widget::Row::with_children(content).into())
    }
}

pub fn input_int_callback(state: &mut IpgState, id: usize, message: InputIntMessage) {
    // During the input, the widget is assigned the value so that it shows
    // during typing.  On submit, the text box is cleared, so no value.
    // However, in both cases the value is passed to the callback.

    match message {
        InputIntMessage::OnInput(value) => {
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
        InputIntMessage::OnSubmit(value) => {
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
        InputIntMessage::OnPaste(value) => {
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
        InputIntMessage::OnPress => {
            invoke_callback(id, CallbackName::OnPress, "InputIntButton");
        }
    }
}

#[derive(Debug, Clone)]
pub enum InputIntMessage {
    OnInput(String),
    OnSubmit(String),
    OnPaste(String),
    OnPress,
}

#[derive(Debug, Clone)]
pub struct InputIntStyle {
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

impl InputIntStyle {
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
pub enum InputIntParam {
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
pub enum InputIntStyleParam {
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

impl WidgetParamUpdate for InputInt {
    type Param = InputIntParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            InputIntParam::LineHeight => set_t_value(&mut self.line_height, value, "InputIntParam::LineHeight"),
            InputIntParam::Padding => set_t_value(&mut self.padding, value, "InputIntParam::Padding"),
            InputIntParam::Placeholder => set_t_value(&mut self.placeholder, value, "InputIntParam::Placeholder"),
            InputIntParam::Show => set_t_value(&mut self.show, value, "InputIntParam::Show"),
            InputIntParam::Size => set_t_value(&mut self.size, value, "InputIntParam::Size"),
            InputIntParam::StyleId => set_t_value(&mut self.style_id, value, "InputIntParam::StyleId"),
            InputIntParam::Value => {
                let v: i64 = extract_param(value);
                self.value = v.to_string();
            }
            InputIntParam::Width => set_t_value(&mut self.width, value, "InputIntParam::Width"),
            InputIntParam::WidthFill => set_t_value(&mut self.width_fill, value, "TextInputParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for InputIntStyle {
    type Param = InputIntStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            InputIntStyleParam::BackgroundColor => {
                set_t_value(&mut self.background_color, value, "InputIntStyleParam::BackgroundColor")
            }
            InputIntStyleParam::BackgroundColorAlpha => set_t_value(
                &mut self.background_color_alpha,
                value,
                "InputIntStyleParam::BackgroundColorAlpha",
            ),
            InputIntStyleParam::BackgroundRgba => {
                set_t_value(&mut self.background_rgba, value, "InputIntStyleParam::BackgroundRgba")
            }
            InputIntStyleParam::TextColor => set_t_value(&mut self.text_color, value, "InputIntStyleParam::TextColor"),
            InputIntStyleParam::TextColorAlpha => {
                set_t_value(&mut self.text_color_alpha, value, "InputIntStyleParam::TextColorAlpha")
            }
            InputIntStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "InputIntStyleParam::TextRgba"),
            InputIntStyleParam::PrimaryColor => {
                set_t_value(&mut self.primary_color, value, "InputIntStyleParam::PrimaryColor")
            }
            InputIntStyleParam::PrimaryColorAlpha => set_t_value(
                &mut self.primary_color_alpha,
                value,
                "InputIntStyleParam::PrimaryColorAlpha",
            ),
            InputIntStyleParam::PrimaryRgba => {
                set_t_value(&mut self.primary_rgba, value, "InputIntStyleParam::PrimaryRgba")
            }
            InputIntStyleParam::SecondaryColor => {
                set_t_value(&mut self.secondary_color, value, "InputIntStyleParam::SecondaryColor")
            }
            InputIntStyleParam::SecondaryColorAlpha => set_t_value(
                &mut self.secondary_color_alpha,
                value,
                "InputIntStyleParam::SecondaryColorAlpha",
            ),
            InputIntStyleParam::SecondaryRgba => {
                set_t_value(&mut self.secondary_rgba, value, "InputIntStyleParam::SecondaryRgba")
            }
            InputIntStyleParam::BorderColorActive => set_t_value(
                &mut self.border_color_active,
                value,
                "InputIntStyleParam::BorderColorActive",
            ),
            InputIntStyleParam::BorderColorAlphaActive => set_t_value(
                &mut self.border_color_alpha_active,
                value,
                "InputIntStyleParam::BorderColorAlphaActive",
            ),
            InputIntStyleParam::BorderRgbaActive => set_t_value(
                &mut self.border_rgba_active,
                value,
                "InputIntStyleParam::BorderRgbaActive",
            ),
            InputIntStyleParam::BorderColorHovered => set_t_value(
                &mut self.border_color_hovered,
                value,
                "InputIntStyleParam::BorderColorHovered",
            ),
            InputIntStyleParam::BorderColorAlphaHovered => set_t_value(
                &mut self.border_color_alpha_hovered,
                value,
                "InputIntStyleParam::BorderColorAlphaHovered",
            ),
            InputIntStyleParam::BorderRgbaHovered => set_t_value(
                &mut self.border_rgba_hovered,
                value,
                "InputIntStyleParam::BorderRgbaHovered",
            ),
            InputIntStyleParam::BorderColorFocused => set_t_value(
                &mut self.border_color_focused,
                value,
                "InputIntStyleParam::BorderColorFocused",
            ),
            InputIntStyleParam::BorderColorAlphaFocused => set_t_value(
                &mut self.border_color_alpha_focused,
                value,
                "InputIntStyleParam::BorderColorAlphaFocused",
            ),
            InputIntStyleParam::BorderRgbaFocused => set_t_value(
                &mut self.border_rgba_focused,
                value,
                "InputIntStyleParam::BorderRgbaFocused",
            ),
            InputIntStyleParam::BorderColorDisabled => set_t_value(
                &mut self.border_color_disabled,
                value,
                "InputIntStyleParam::BorderColorDisabled",
            ),
            InputIntStyleParam::BorderColorAlphaDisabled => set_t_value(
                &mut self.border_color_alpha_disabled,
                value,
                "InputIntStyleParam::BorderColorAlphaDisabled",
            ),
            InputIntStyleParam::BorderRgbaDisabled => set_t_value(
                &mut self.border_rgba_disabled,
                value,
                "InputIntStyleParam::BorderRgbaDisabled",
            ),
            InputIntStyleParam::BorderWidth => {
                set_t_value(&mut self.border_width, value, "InputIntStyleParam::BorderWidth")
            }
            InputIntStyleParam::BorderRadius => {
                set_t_value(&mut self.border_radius, value, "InputIntStyleParam::BorderRadius")
            }
            InputIntStyleParam::PlaceholderColorActive => set_t_value(
                &mut self.placeholder_color_active,
                value,
                "InputIntStyleParam::PlaceholderColorActive",
            ),
            InputIntStyleParam::PlaceholderColorAlphaActive => set_t_value(
                &mut self.placeholder_color_alpha_active,
                value,
                "InputIntStyleParam::PlaceholderColorAlphaActive",
            ),
            InputIntStyleParam::PlaceholderRgbaActive => set_t_value(
                &mut self.placeholder_rgba_active,
                value,
                "InputIntStyleParam::PlaceholderRgbaActive",
            ),
            InputIntStyleParam::PlaceholderColorHovered => set_t_value(
                &mut self.placeholder_color_hovered,
                value,
                "InputIntStyleParam::PlaceholderColorHovered",
            ),
            InputIntStyleParam::PlaceholderColorAlphaHovered => set_t_value(
                &mut self.placeholder_color_alpha_hovered,
                value,
                "InputIntStyleParam::PlaceholderColorAlphaHovered",
            ),
            InputIntStyleParam::PlaceholderRgbaHovered => set_t_value(
                &mut self.placeholder_rgba_hovered,
                value,
                "InputIntStyleParam::PlaceholderRgbaHovered",
            ),
            InputIntStyleParam::PlaceholderColorFocused => set_t_value(
                &mut self.placeholder_color_focused,
                value,
                "InputIntStyleParam::PlaceholderColorFocused",
            ),
            InputIntStyleParam::PlaceholderColorAlphaFocused => set_t_value(
                &mut self.placeholder_color_alpha_focused,
                value,
                "InputIntStyleParam::PlaceholderColorAlphaFocused",
            ),
            InputIntStyleParam::PlaceholderRgbaFocused => set_t_value(
                &mut self.placeholder_rgba_focused,
                value,
                "InputIntStyleParam::PlaceholderRgbaFocused",
            ),
            InputIntStyleParam::PlaceholderColorDisabled => set_t_value(
                &mut self.placeholder_color_disabled,
                value,
                "InputIntStyleParam::PlaceholderColorDisabled",
            ),
            InputIntStyleParam::PlaceholderColorAlphaDisabled => set_t_value(
                &mut self.placeholder_color_alpha_disabled,
                value,
                "InputIntStyleParam::PlaceholderColorAlphaDisabled",
            ),
            InputIntStyleParam::PlaceholderRgbaDisabled => set_t_value(
                &mut self.placeholder_rgba_disabled,
                value,
                "InputIntStyleParam::PlaceholderRgbaDisabled",
            ),
            InputIntStyleParam::ValueColorActive => set_t_value(
                &mut self.value_color_active,
                value,
                "InputIntStyleParam::ValueColorActive",
            ),
            InputIntStyleParam::ValueColorAlphaActive => set_t_value(
                &mut self.value_color_alpha_active,
                value,
                "InputIntStyleParam::ValueColorAlphaActive",
            ),
            InputIntStyleParam::ValueRgbaActive => set_t_value(
                &mut self.value_rgba_active,
                value,
                "InputIntStyleParam::ValueRgbaActive",
            ),
            InputIntStyleParam::ValueColorHovered => set_t_value(
                &mut self.value_color_hovered,
                value,
                "InputIntStyleParam::ValueColorHovered",
            ),
            InputIntStyleParam::ValueColorAlphaHovered => set_t_value(
                &mut self.value_color_alpha_hovered,
                value,
                "InputIntStyleParam::ValueColorAlphaHovered",
            ),
            InputIntStyleParam::ValueRgbaHovered => set_t_value(
                &mut self.value_rgba_hovered,
                value,
                "InputIntStyleParam::ValueRgbaHovered",
            ),
            InputIntStyleParam::ValueColorFocused => set_t_value(
                &mut self.value_color_focused,
                value,
                "InputIntStyleParam::ValueColorFocused",
            ),
            InputIntStyleParam::ValueColorAlphaFocused => set_t_value(
                &mut self.value_color_alpha_focused,
                value,
                "InputIntStyleParam::ValueColorAlphaFocused",
            ),
            InputIntStyleParam::ValueRgbaFocused => set_t_value(
                &mut self.value_rgba_focused,
                value,
                "InputIntStyleParam::ValueRgbaFocused",
            ),
            InputIntStyleParam::ValueColorDisabled => set_t_value(
                &mut self.value_color_disabled,
                value,
                "InputIntStyleParam::ValueColorDisabled",
            ),
            InputIntStyleParam::ValueColorAlphaDisabled => set_t_value(
                &mut self.value_color_alpha_disabled,
                value,
                "InputIntStyleParam::ValueColorAlphaDisabled",
            ),
            InputIntStyleParam::ValueRgbaDisabled => set_t_value(
                &mut self.value_rgba_disabled,
                value,
                "InputIntStyleParam::ValueRgbaDisabled",
            ),
            InputIntStyleParam::SelectionColorActive => set_t_value(
                &mut self.selection_color_active,
                value,
                "InputIntStyleParam::SelectionColorActive",
            ),
            InputIntStyleParam::SelectionColorAlphaActive => set_t_value(
                &mut self.selection_color_alpha_active,
                value,
                "InputIntStyleParam::SelectionColorAlphaActive",
            ),
            InputIntStyleParam::SelectionRgbaActive => set_t_value(
                &mut self.selection_rgba_active,
                value,
                "InputIntStyleParam::SelectionRgbaActive",
            ),
            InputIntStyleParam::SelectionColorHovered => set_t_value(
                &mut self.selection_color_hovered,
                value,
                "InputIntStyleParam::SelectionColorHovered",
            ),
            InputIntStyleParam::SelectionColorAlphaHovered => set_t_value(
                &mut self.selection_color_alpha_hovered,
                value,
                "InputIntStyleParam::SelectionColorAlphaHovered",
            ),
            InputIntStyleParam::SelectionRgbaHovered => set_t_value(
                &mut self.selection_rgba_hovered,
                value,
                "InputIntStyleParam::SelectionRgbaHovered",
            ),
            InputIntStyleParam::SelectionColorFocused => set_t_value(
                &mut self.selection_color_focused,
                value,
                "InputIntStyleParam::SelectionColorFocused",
            ),
            InputIntStyleParam::SelectionColorAlphaFocused => set_t_value(
                &mut self.selection_color_alpha_focused,
                value,
                "InputIntStyleParam::SelectionColorAlphaFocused",
            ),
            InputIntStyleParam::SelectionRgbaFocused => set_t_value(
                &mut self.selection_rgba_focused,
                value,
                "InputIntStyleParam::SelectionRgbaFocused",
            ),
            InputIntStyleParam::SelectionColorDisabled => set_t_value(
                &mut self.selection_color_disabled,
                value,
                "InputIntStyleParam::SelectionColorDisabled",
            ),
            InputIntStyleParam::SelectionColorAlphaDisabled => set_t_value(
                &mut self.selection_color_alpha_disabled,
                value,
                "InputIntStyleParam::SelectionColorAlphaDisabled",
            ),
            InputIntStyleParam::SelectionRgbaDisabled => set_t_value(
                &mut self.selection_rgba_disabled,
                value,
                "InputIntStyleParam::SelectionRgbaDisabled",
            ),
        }
    }
}
