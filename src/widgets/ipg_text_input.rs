//! Text inputs display fields that can be filled with text.
#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::widget::text_input;
use iced::widget::text_input::{Style, Status};
use iced::{Border, Element, Theme, alignment};
use iced::widget;
use iced::theme::palette::{self, Background};

use pyo3::pyclass;
use pyo3::{Py, PyAny};

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::helpers::{get_len, get_padding};
use crate::{IpgState};
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,  set_t_value};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct TextInput {
    pub id: usize,
    pub parent_id: String,
    pub placeholder: String,
    pub value: String,
    pub is_secure: Option<bool>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub padding: Option<Vec<f32>>,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub align_left: Option<bool>,
    pub align_center: Option<bool>,
    pub align_right: Option<bool>,
    pub font_id: Option<usize>,
    pub style_id: Option<usize>,
    pub show: bool,
}

impl TextInput { 

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }
    
    pub fn construct<'a>( 
        &'a self,
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, Message>> {
       
        if !self.show {
            return None
        }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_text_input_style).cloned();


        let txt: widget::TextInput<'_, TIMessage> =  
            widget::TextInput::new(
                    self.placeholder.as_str(), 
                    self.value.as_str()
                )
                .on_input(TIMessage::OnInput)
                .on_submit(TIMessage::OnSubmit(self.value.clone()))
                .on_paste(TIMessage::OnPaste)
                .secure(self.is_secure.unwrap_or(false))
                .width(get_len(None, self.width_fill, self.width))
                .padding(get_padding(&self.padding))
                .style(move|theme: &Theme, status| {   
                    if let Some(ti) = &style_opt {
                        ti.to_iced(theme, status)
                    } else {
                        text_input::default(theme, status)
                    }
                })
            ;

        let txt = if let Some(sz) = self.size {
            txt.size(sz)
        } else { txt };

        let txt = if let Some(lh) = self.line_height {
            txt.line_height(lh)
        } else { txt };

        // default
        let txt = txt.align_x(alignment::Horizontal::Left);

        let txt = if self.align_center == Some(true) {
            txt.align_x(alignment::Horizontal::Center)
        } else { txt };

        let txt = if self.align_right == Some(true) {
            txt.align_x(alignment::Horizontal::Right)
        } else { txt };

        let txt: Element<'_, TIMessage> = txt.into();

        Some(txt.map(move |message| Message::TextInput(self.id, message)))

    }

}


pub fn text_input_callback(state: &mut IpgState, id: usize, message: TIMessage) {
    // During the input, the widget is assigned the value so that it shows
    // during typing.  On submit, the text box is cleared, so no value.
    // However, in both cases the value is passed to the callback.

    match message {
        TIMessage::OnInput(value) => {
            if let Some(Widgets::TextInput(ti)) = state.widgets.get_mut(&id) {
                ti.value = value.clone();
            }
            invoke_callback_with_args(id, "on_input", "TextInput", value,
                "def cb(wid: int, value: str)");
        },
        TIMessage::OnSubmit(value) => {
            if let Some(Widgets::TextInput(ti)) = state.widgets.get_mut(&id) {
                ti.value = String::new();
            }
            invoke_callback_with_args(id, "on_submit", "TextInput", value,
                "def cb(wid: int, value: str)");
        }
        TIMessage::OnPaste(value) => {
            if let Some(Widgets::TextInput(ti)) = state.widgets.get_mut(&id) {
                ti.value = value.clone();
            }
            invoke_callback_with_args(id, "on_paste", "TextInput", value,
                "def cb(wid: int, value: str)");
        }
            
    }
}

#[derive(Debug, Clone)]
pub enum TIMessage {
    OnInput(String),
    OnSubmit(String),
    OnPaste(String),
}

#[derive(Debug, Clone)]
pub struct TextInputStyle {
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

    // overrides all other icon colors
    // if not defined
    pub icon_color_active: Option<Color>,
    pub icon_color_alpha_active: Option<f32>,
    pub icon_rgba_active: Option<[f32; 4]>,

    pub icon_color_hovered: Option<Color>,
    pub icon_color_alpha_hovered: Option<f32>,
    pub icon_rgba_hovered: Option<[f32; 4]>,

    pub icon_color_focused: Option<Color>,
    pub icon_color_alpha_focused: Option<f32>,
    pub icon_rgba_focused: Option<[f32; 4]>,

    pub icon_color_disabled: Option<Color>,
    pub icon_color_alpha_disabled: Option<f32>,
    pub icon_rgba_disabled: Option<[f32; 4]>,

    // overrides all other icon colors
    // if not defined
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

    // overrides all other icon colors
    // if not defined
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

    // overrides all other icon colors
    // if not defined
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

impl TextInputStyle {
    fn to_iced(
        &self,
        theme: &Theme, 
        status: Status, 
    ) -> Style {

    let background_color = 
        Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);

    let text_color = 
        Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha);

    let primary_color = 
        Color::rgba_ipg_color_to_iced(self.primary_rgba, &self.primary_color, self.primary_color_alpha);

    let secondary_color = 
        Color::rgba_ipg_color_to_iced(self.secondary_rgba, &self.secondary_color, self.secondary_color_alpha);
    
    let icon_color_active = 
        Color::rgba_ipg_color_to_iced(self.icon_rgba_active, &self.icon_color_active, self.icon_color_alpha_active);
    let icon_color_hovered = 
        Color::rgba_ipg_color_to_iced(self.icon_rgba_hovered, &self.icon_color_hovered, self.icon_color_alpha_hovered);
    let icon_color_focused = 
        Color::rgba_ipg_color_to_iced(self.icon_rgba_focused, &self.icon_color_focused, self.icon_color_alpha_focused);
    let icon_color_disabled = 
        Color::rgba_ipg_color_to_iced(self.icon_rgba_disabled, &self.icon_color_disabled, self.icon_color_alpha_disabled);

    let border_color_active = 
        Color::rgba_ipg_color_to_iced(self.border_rgba_active, &self.border_color_active, self.border_color_alpha_active);
    let border_color_hovered = 
        Color::rgba_ipg_color_to_iced(self.border_rgba_hovered, &self.border_color_hovered, self.border_color_alpha_hovered);
    let border_color_focused = 
        Color::rgba_ipg_color_to_iced(self.border_rgba_focused, &self.border_color_focused, self.border_color_alpha_focused);
    let border_color_disabled = 
        Color::rgba_ipg_color_to_iced(self.border_rgba_disabled, &self.border_color_disabled, self.border_color_alpha_disabled);
    
    let placeholder_color_active = 
        Color::rgba_ipg_color_to_iced(self.placeholder_rgba_active, &self.placeholder_color_active, self.placeholder_color_alpha_active);
    let placeholder_color_hovered = 
        Color::rgba_ipg_color_to_iced(self.placeholder_rgba_hovered, &self.placeholder_color_hovered, self.placeholder_color_alpha_hovered);
    let placeholder_color_focused = 
        Color::rgba_ipg_color_to_iced(self.placeholder_rgba_focused, &self.placeholder_color_focused, self.placeholder_color_alpha_focused);
    let placeholder_color_disabled = 
        Color::rgba_ipg_color_to_iced(self.placeholder_rgba_disabled, &self.placeholder_color_disabled, self.placeholder_color_alpha_disabled);

    let value_color_active = 
        Color::rgba_ipg_color_to_iced(self.value_rgba_active, &self.value_color_active, self.value_color_alpha_active);
    let value_color_hovered = 
        Color::rgba_ipg_color_to_iced(self.value_rgba_hovered, &self.value_color_hovered, self.value_color_alpha_hovered);
    let value_color_focused = 
        Color::rgba_ipg_color_to_iced(self.value_rgba_focused, &self.value_color_focused, self.value_color_alpha_focused);
    let value_color_disabled = 
        Color::rgba_ipg_color_to_iced(self.value_rgba_disabled, &self.value_color_disabled, self.value_color_alpha_disabled);
    
    
    let selection_color_active = 
        Color::rgba_ipg_color_to_iced(self.selection_rgba_active, &self.selection_color_active, self.selection_color_alpha_active);
    let selection_color_hovered = 
        Color::rgba_ipg_color_to_iced(self.selection_rgba_hovered, &self.selection_color_hovered, self.selection_color_alpha_hovered);
    let selection_color_focused = 
        Color::rgba_ipg_color_to_iced(self.selection_rgba_focused, &self.selection_color_focused, self.selection_color_alpha_focused);
    let selection_color_disabled = 
        Color::rgba_ipg_color_to_iced(self.selection_rgba_disabled, &self.selection_color_disabled, self.selection_color_alpha_disabled);

    // border
    let br = if let Some(br) = self.border_radius {
        br
    } else { 2.0 };

    let bw = if let Some(bw) = self.border_width {
        bw
    } else { 1.0 };

    let palette = theme.palette();
    
    // One can use the theme text color but the background and primary 
    // are needed together to produce the correct colors
    let txt_color = if let Some(c) = text_color {
        c
    } else { theme.palette().background.base.text};

    let background_opt = if let Some(bkg) = background_color {
        Some(Background::new(bkg, txt_color))
    } else { None };

    let pm_swatch_opt = if let Some(c) = primary_color {
        Some(palette::Swatch::derive(c, txt_color))
    } else { None };

    let sec_swatch_opt = if let Some(c) = secondary_color {
        Some(palette::Swatch::derive(c, txt_color))
    } else { None };

    let new_theme = background_opt.is_some() && pm_swatch_opt.is_some() && sec_swatch_opt.is_some();

    let bkg_base_color = if new_theme {
        background_opt.unwrap().base.color
    } else { palette.background.base.color };

    // border color
    let (bc_active, bc_hovered, bc_focused, bc_disabled) = if new_theme {
        let background = background_opt.unwrap();
        let primary = pm_swatch_opt.unwrap();
        (background.strong.color, background.base.text,
        primary.strong.color, background.base.text)
    } else {
        (
            border_color_active.unwrap_or(palette.background.strong.color),
            border_color_hovered.or(border_color_active).unwrap_or(palette.background.base.text),
            border_color_focused.or(border_color_active).unwrap_or(palette.primary.strong.color),
            border_color_disabled.or(border_color_active).unwrap_or(palette.background.strong.color),
        )
    };
    
    // icon
    let (ic_active, ic_hovered, ic_focused, ic_disabled) = if new_theme {
        let c = background_opt.unwrap().weak.text;
        (c, c, c, c)
    } else {
        let base = icon_color_active.unwrap_or(palette.background.weak.text);
        (
            base,
            icon_color_hovered.unwrap_or(base),
            icon_color_focused.unwrap_or(base),
            icon_color_disabled.unwrap_or(base),
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
            placeholder_color_disabled.or(placeholder_color_active).unwrap_or(palette.background.strongest.color),
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
        icon: ic_active,
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
        icon: ic_hovered,
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
        icon: ic_focused,
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
        icon: ic_disabled,
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

}}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TextInputParam {
    IsSecure,
    LineHeight,
    Padding,
    Placeholder,
    Size,
    StyleId,
    Value,
    Width,
    WidthFill,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TextInputStyleParam {
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

    IconColorActive,
    IconColorAlphaActive,
    IconRgbaActive,

    IconColorHovered,
    IconColorAlphaHovered,
    IconRgbaHovered,

    IconColorFocused,
    IconColorAlphaFocused,
    IconRgbaFocused,

    IconColorDisabled,
    IconColorAlphaDisabled,
    IconRgbaDisabled,

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

impl WidgetParamUpdate for TextInput {
    type Param = TextInputParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TextInputParam::IsSecure => set_t_value(&mut self.is_secure, value, "TextInputParam::IsSecure"),
            TextInputParam::LineHeight => set_t_value(&mut self.line_height, value, "TextInputParam::LineHeight"),
            TextInputParam::Padding => set_t_value(&mut self.padding, value, "TextInputParam::Padding"),
            TextInputParam::Placeholder => set_t_value(&mut self.placeholder, value, "TextInputParam::Placeholder"),
            TextInputParam::Size => set_t_value(&mut self.size, value, "TextInputParam::Size"),
            TextInputParam::StyleId => set_t_value(&mut self.style_id, value, "TextInputParam::StyleId"),
            TextInputParam::Value => set_t_value(&mut self.value, value, "TextInputParam::Value"),
            TextInputParam::Width => set_t_value(&mut self.width, value, "TextInputParam::Width"),
            TextInputParam::WidthFill => set_t_value(&mut self.width_fill, value, "TextInputParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for TextInputStyle {
    type Param = TextInputStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TextInputStyleParam::BackgroundColor => set_t_value(&mut self.background_color, value, "TextInputStyleParam::BackgroundColor"),
            TextInputStyleParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "TextInputStyleParam::BackgroundColorAlpha"),
            TextInputStyleParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "TextInputStyleParam::BackgroundRgba"),
            TextInputStyleParam::TextColor => set_t_value(&mut self.text_color, value, "TextInputStyleParam::TextColor"),
            TextInputStyleParam::TextColorAlpha => set_t_value(&mut self.text_color_alpha, value, "TextInputStyleParam::TextColorAlpha"),
            TextInputStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "TextInputStyleParam::TextRgba"),
            TextInputStyleParam::PrimaryColor => set_t_value(&mut self.primary_color, value, "TextInputStyleParam::PrimaryColor"),
            TextInputStyleParam::PrimaryColorAlpha => set_t_value(&mut self.primary_color_alpha, value, "TextInputStyleParam::PrimaryColorAlpha"),
            TextInputStyleParam::PrimaryRgba => set_t_value(&mut self.primary_rgba, value, "TextInputStyleParam::PrimaryRgba"),
            TextInputStyleParam::SecondaryColor => set_t_value(&mut self.secondary_color, value, "TextInputStyleParam::SecondaryColor"),
            TextInputStyleParam::SecondaryColorAlpha => set_t_value(&mut self.secondary_color_alpha, value, "TextInputStyleParam::SecondaryColorAlpha"),
            TextInputStyleParam::SecondaryRgba => set_t_value(&mut self.secondary_rgba, value, "TextInputStyleParam::SecondaryRgba"),
            TextInputStyleParam::BorderColorActive => set_t_value(&mut self.border_color_active, value, "TextInputStyleParam::BorderColorActive"),
            TextInputStyleParam::BorderColorAlphaActive => set_t_value(&mut self.border_color_alpha_active, value, "TextInputStyleParam::BorderColorAlphaActive"),
            TextInputStyleParam::BorderRgbaActive => set_t_value(&mut self.border_rgba_active, value, "TextInputStyleParam::BorderRgbaActive"),
            TextInputStyleParam::BorderColorHovered => set_t_value(&mut self.border_color_hovered, value, "TextInputStyleParam::BorderColorHovered"),
            TextInputStyleParam::BorderColorAlphaHovered => set_t_value(&mut self.border_color_alpha_hovered, value, "TextInputStyleParam::BorderColorAlphaHovered"),
            TextInputStyleParam::BorderRgbaHovered => set_t_value(&mut self.border_rgba_hovered, value, "TextInputStyleParam::BorderRgbaHovered"),
            TextInputStyleParam::BorderColorFocused => set_t_value(&mut self.border_color_focused, value, "TextInputStyleParam::BorderColorFocused"),
            TextInputStyleParam::BorderColorAlphaFocused => set_t_value(&mut self.border_color_alpha_focused, value, "TextInputStyleParam::BorderColorAlphaFocused"),
            TextInputStyleParam::BorderRgbaFocused => set_t_value(&mut self.border_rgba_focused, value, "TextInputStyleParam::BorderRgbaFocused"),
            TextInputStyleParam::BorderColorDisabled => set_t_value(&mut self.border_color_disabled, value, "TextInputStyleParam::BorderColorDisabled"),
            TextInputStyleParam::BorderColorAlphaDisabled => set_t_value(&mut self.border_color_alpha_disabled, value, "TextInputStyleParam::BorderColorAlphaDisabled"),
            TextInputStyleParam::BorderRgbaDisabled => set_t_value(&mut self.border_rgba_disabled, value, "TextInputStyleParam::BorderRgbaDisabled"),
            TextInputStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "TextInputStyleParam::BorderWidth"),
            TextInputStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "TextInputStyleParam::BorderRadius"),
            TextInputStyleParam::IconColorActive => set_t_value(&mut self.icon_color_active, value, "TextInputStyleParam::IconColorActive"),
            TextInputStyleParam::IconColorAlphaActive => set_t_value(&mut self.icon_color_alpha_active, value, "TextInputStyleParam::IconColorAlphaActive"),
            TextInputStyleParam::IconRgbaActive => set_t_value(&mut self.icon_rgba_active, value, "TextInputStyleParam::IconRgbaActive"),
            TextInputStyleParam::IconColorHovered => set_t_value(&mut self.icon_color_hovered, value, "TextInputStyleParam::IconColorHovered"),
            TextInputStyleParam::IconColorAlphaHovered => set_t_value(&mut self.icon_color_alpha_hovered, value, "TextInputStyleParam::IconColorAlphaHovered"),
            TextInputStyleParam::IconRgbaHovered => set_t_value(&mut self.icon_rgba_hovered, value, "TextInputStyleParam::IconRgbaHovered"),
            TextInputStyleParam::IconColorFocused => set_t_value(&mut self.icon_color_focused, value, "TextInputStyleParam::IconColorFocused"),
            TextInputStyleParam::IconColorAlphaFocused => set_t_value(&mut self.icon_color_alpha_focused, value, "TextInputStyleParam::IconColorAlphaFocused"),
            TextInputStyleParam::IconRgbaFocused => set_t_value(&mut self.icon_rgba_focused, value, "TextInputStyleParam::IconRgbaFocused"),
            TextInputStyleParam::IconColorDisabled => set_t_value(&mut self.icon_color_disabled, value, "TextInputStyleParam::IconColorDisabled"),
            TextInputStyleParam::IconColorAlphaDisabled => set_t_value(&mut self.icon_color_alpha_disabled, value, "TextInputStyleParam::IconColorAlphaDisabled"),
            TextInputStyleParam::IconRgbaDisabled => set_t_value(&mut self.icon_rgba_disabled, value, "TextInputStyleParam::IconRgbaDisabled"),
            TextInputStyleParam::PlaceholderColorActive => set_t_value(&mut self.placeholder_color_active, value, "TextInputStyleParam::PlaceholderColorActive"),
            TextInputStyleParam::PlaceholderColorAlphaActive => set_t_value(&mut self.placeholder_color_alpha_active, value, "TextInputStyleParam::PlaceholderColorAlphaActive"),
            TextInputStyleParam::PlaceholderRgbaActive => set_t_value(&mut self.placeholder_rgba_active, value, "TextInputStyleParam::PlaceholderRgbaActive"),
            TextInputStyleParam::PlaceholderColorHovered => set_t_value(&mut self.placeholder_color_hovered, value, "TextInputStyleParam::PlaceholderColorHovered"),
            TextInputStyleParam::PlaceholderColorAlphaHovered => set_t_value(&mut self.placeholder_color_alpha_hovered, value, "TextInputStyleParam::PlaceholderColorAlphaHovered"),
            TextInputStyleParam::PlaceholderRgbaHovered => set_t_value(&mut self.placeholder_rgba_hovered, value, "TextInputStyleParam::PlaceholderRgbaHovered"),
            TextInputStyleParam::PlaceholderColorFocused => set_t_value(&mut self.placeholder_color_focused, value, "TextInputStyleParam::PlaceholderColorFocused"),
            TextInputStyleParam::PlaceholderColorAlphaFocused => set_t_value(&mut self.placeholder_color_alpha_focused, value, "TextInputStyleParam::PlaceholderColorAlphaFocused"),
            TextInputStyleParam::PlaceholderRgbaFocused => set_t_value(&mut self.placeholder_rgba_focused, value, "TextInputStyleParam::PlaceholderRgbaFocused"),
            TextInputStyleParam::PlaceholderColorDisabled => set_t_value(&mut self.placeholder_color_disabled, value, "TextInputStyleParam::PlaceholderColorDisabled"),
            TextInputStyleParam::PlaceholderColorAlphaDisabled => set_t_value(&mut self.placeholder_color_alpha_disabled, value, "TextInputStyleParam::PlaceholderColorAlphaDisabled"),
            TextInputStyleParam::PlaceholderRgbaDisabled => set_t_value(&mut self.placeholder_rgba_disabled, value, "TextInputStyleParam::PlaceholderRgbaDisabled"),
            TextInputStyleParam::ValueColorActive => set_t_value(&mut self.value_color_active, value, "TextInputStyleParam::ValueColorActive"),
            TextInputStyleParam::ValueColorAlphaActive => set_t_value(&mut self.value_color_alpha_active, value, "TextInputStyleParam::ValueColorAlphaActive"),
            TextInputStyleParam::ValueRgbaActive => set_t_value(&mut self.value_rgba_active, value, "TextInputStyleParam::ValueRgbaActive"),
            TextInputStyleParam::ValueColorHovered => set_t_value(&mut self.value_color_hovered, value, "TextInputStyleParam::ValueColorHovered"),
            TextInputStyleParam::ValueColorAlphaHovered => set_t_value(&mut self.value_color_alpha_hovered, value, "TextInputStyleParam::ValueColorAlphaHovered"),
            TextInputStyleParam::ValueRgbaHovered => set_t_value(&mut self.value_rgba_hovered, value, "TextInputStyleParam::ValueRgbaHovered"),
            TextInputStyleParam::ValueColorFocused => set_t_value(&mut self.value_color_focused, value, "TextInputStyleParam::ValueColorFocused"),
            TextInputStyleParam::ValueColorAlphaFocused => set_t_value(&mut self.value_color_alpha_focused, value, "TextInputStyleParam::ValueColorAlphaFocused"),
            TextInputStyleParam::ValueRgbaFocused => set_t_value(&mut self.value_rgba_focused, value, "TextInputStyleParam::ValueRgbaFocused"),
            TextInputStyleParam::ValueColorDisabled => set_t_value(&mut self.value_color_disabled, value, "TextInputStyleParam::ValueColorDisabled"),
            TextInputStyleParam::ValueColorAlphaDisabled => set_t_value(&mut self.value_color_alpha_disabled, value, "TextInputStyleParam::ValueColorAlphaDisabled"),
            TextInputStyleParam::ValueRgbaDisabled => set_t_value(&mut self.value_rgba_disabled, value, "TextInputStyleParam::ValueRgbaDisabled"),
            TextInputStyleParam::SelectionColorActive => set_t_value(&mut self.selection_color_active, value, "TextInputStyleParam::SelectionColorActive"),
            TextInputStyleParam::SelectionColorAlphaActive => set_t_value(&mut self.selection_color_alpha_active, value, "TextInputStyleParam::SelectionColorAlphaActive"),
            TextInputStyleParam::SelectionRgbaActive => set_t_value(&mut self.selection_rgba_active, value, "TextInputStyleParam::SelectionRgbaActive"),
            TextInputStyleParam::SelectionColorHovered => set_t_value(&mut self.selection_color_hovered, value, "TextInputStyleParam::SelectionColorHovered"),
            TextInputStyleParam::SelectionColorAlphaHovered => set_t_value(&mut self.selection_color_alpha_hovered, value, "TextInputStyleParam::SelectionColorAlphaHovered"),
            TextInputStyleParam::SelectionRgbaHovered => set_t_value(&mut self.selection_rgba_hovered, value, "TextInputStyleParam::SelectionRgbaHovered"),
            TextInputStyleParam::SelectionColorFocused => set_t_value(&mut self.selection_color_focused, value, "TextInputStyleParam::SelectionColorFocused"),
            TextInputStyleParam::SelectionColorAlphaFocused => set_t_value(&mut self.selection_color_alpha_focused, value, "TextInputStyleParam::SelectionColorAlphaFocused"),
            TextInputStyleParam::SelectionRgbaFocused => set_t_value(&mut self.selection_rgba_focused, value, "TextInputStyleParam::SelectionRgbaFocused"),
            TextInputStyleParam::SelectionColorDisabled => set_t_value(&mut self.selection_color_disabled, value, "TextInputStyleParam::SelectionColorDisabled"),
            TextInputStyleParam::SelectionColorAlphaDisabled => set_t_value(&mut self.selection_color_alpha_disabled, value, "TextInputStyleParam::SelectionColorAlphaDisabled"),
            TextInputStyleParam::SelectionRgbaDisabled => set_t_value(&mut self.selection_rgba_disabled, value, "TextInputStyleParam::SelectionRgbaDisabled"),
        }
    }
}
