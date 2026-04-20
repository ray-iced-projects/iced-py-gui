//! ipg_text_input
#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::widget::text_input;
use iced::widget::text_input::{Style, Status};
use iced::{Element, Theme, alignment};
use iced::widget;
use iced::theme::palette;

use pyo3::pyclass;
use pyo3::{Py, PyAny};

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::helpers::{get_len, get_padding};
use crate::widgets::styling::create_custom_theme;
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

    pub border_color_active: Option<Color>,
    pub border_color_active_alpha: Option<f32>,
    pub border_rgba_active: Option<[f32; 4]>,

    pub border_color_hovered: Option<Color>,
    pub border_color_hovered_alpha: Option<f32>,
    pub border_rgba_hovered: Option<[f32; 4]>,

    pub border_color_focused: Option<Color>,
    pub border_color_focused_alpha: Option<f32>,
    pub border_rgba_focused: Option<[f32; 4]>,

    pub border_color_disabled: Option<Color>,
    pub border_color_disabled_alpha: Option<f32>,
    pub border_rgba_disabled: Option<[f32; 4]>,

    pub border_width: Option<f32>,
    pub border_radius: Option<f32>,

    pub placeholder_color_active: Option<Color>,
    pub placeholder_color_active_alpha: Option<f32>,
    pub placeholder_rgba_active: Option<[f32; 4]>,

    pub placeholder_color_hovered: Option<Color>,
    pub placeholder_color_hovered_alpha: Option<f32>,
    pub placeholder_rgba_hovered: Option<[f32; 4]>,

    pub placeholder_color_focused: Option<Color>,
    pub placeholder_color_focused_alpha: Option<f32>,
    pub placeholder_rgba_focused: Option<[f32; 4]>,
    
    pub placeholder_color_disabled: Option<Color>,
    pub placeholder_color_disabled_alpha: Option<f32>,
    pub placeholder_rgba_disabled: Option<[f32; 4]>,

    pub value_color_active: Option<Color>,
    pub value_color_active_alpha: Option<f32>,
    pub value_rgba_active: Option<[f32; 4]>,

    pub value_color_hovered: Option<Color>,
    pub value_color_hovered_alpha: Option<f32>,
    pub value_rgba_hovered: Option<[f32; 4]>,

    pub value_color_focused: Option<Color>,
    pub value_color_focused_alpha: Option<f32>,
    pub value_rgba_focused: Option<[f32; 4]>,

    pub value_color_disabled: Option<Color>,
    pub value_color_disabled_alpha: Option<f32>,
    pub value_rgba_disabled: Option<[f32; 4]>,

    pub selection_color_active: Option<Color>,
    pub selection_color_active_alpha: Option<f32>,
    pub selection_rgba_active: Option<[f32; 4]>,

    pub selection_color_hovered: Option<Color>,
    pub selection_color_hovered_alpha: Option<f32>,
    pub selection_rgba_hovered: Option<[f32; 4]>,

    pub selection_color_focused: Option<Color>,
    pub selection_color_focused_alpha: Option<f32>,
    pub selection_rgba_focused: Option<[f32; 4]>,

    pub selection_color_disabled: Option<Color>,
    pub selection_color_disabled_alpha: Option<f32>,
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
    
    let border_color_active = 
        Color::rgba_ipg_color_to_iced(self.border_rgba_active, &self.border_color_active, self.border_color_active_alpha);
    let border_color_hovered = 
        Color::rgba_ipg_color_to_iced(self.border_rgba_hovered, &self.border_color_hovered, self.border_color_hovered_alpha);
    let border_color_focused = 
        Color::rgba_ipg_color_to_iced(self.border_rgba_focused, &self.border_color_focused, self.border_color_focused_alpha);
    let border_color_disabled = 
        Color::rgba_ipg_color_to_iced(self.border_rgba_disabled, &self.border_color_disabled, self.border_color_disabled_alpha);
    
    let placeholder_color_active = 
        Color::rgba_ipg_color_to_iced(self.placeholder_rgba_active, &self.placeholder_color_active, self.placeholder_color_active_alpha);
    let placeholder_color_hovered = 
        Color::rgba_ipg_color_to_iced(self.placeholder_rgba_hovered, &self.placeholder_color_hovered, self.placeholder_color_hovered_alpha);
    let placeholder_color_focused = 
        Color::rgba_ipg_color_to_iced(self.placeholder_rgba_focused, &self.placeholder_color_focused, self.placeholder_color_focused_alpha);
    let placeholder_color_disabled = 
        Color::rgba_ipg_color_to_iced(self.placeholder_rgba_disabled, &self.placeholder_color_disabled, self.placeholder_color_disabled_alpha);

    let value_color_active = 
        Color::rgba_ipg_color_to_iced(self.value_rgba_active, &self.value_color_active, self.value_color_active_alpha);
    let value_color_hovered = 
        Color::rgba_ipg_color_to_iced(self.value_rgba_hovered, &self.value_color_hovered, self.value_color_hovered_alpha);
    let value_color_focused = 
        Color::rgba_ipg_color_to_iced(self.value_rgba_focused, &self.value_color_focused, self.value_color_focused_alpha);
    let value_color_disabled = 
        Color::rgba_ipg_color_to_iced(self.value_rgba_disabled, &self.value_color_disabled, self.value_color_disabled_alpha);
    
    
    let selection_color_active = 
        Color::rgba_ipg_color_to_iced(self.selection_rgba_active, &self.selection_color_active, self.selection_color_active_alpha);
    let selection_color_hovered = 
        Color::rgba_ipg_color_to_iced(self.selection_rgba_hovered, &self.selection_color_hovered, self.selection_color_hovered_alpha);
    let selection_color_focused = 
        Color::rgba_ipg_color_to_iced(self.selection_rgba_focused, &self.selection_color_focused, self.selection_color_focused_alpha);
    let selection_color_disabled = 
        Color::rgba_ipg_color_to_iced(self.selection_rgba_disabled, &self.selection_color_disabled, self.selection_color_disabled_alpha);

    let custom_theme;
    
    let palette = if let Some(bkg) = background_color {
        let dark_mode = palette::is_dark(bkg);
        custom_theme = create_custom_theme(bkg, dark_mode);
        custom_theme.extended_palette()
    } else {
        theme.extended_palette()
    };

    // border
    let br = if let Some(br) = self.border_radius {
        br
    } else { 2.0 };

    let bw = if let Some(bw) = self.border_width {
        bw
    } else { 1.0 };

    let bc_active = if let Some(bc) = border_color_active {
        bc
    } else { palette.background.strong.color };

    let bc_hovered = if let Some(bc) = border_color_hovered {
        bc
    } else { palette.background.base.text };

    let bc_focused = if let Some(bc) = border_color_focused {
        bc
    } else { palette.primary.strong.color };

    let bc_disabled = if let Some(bc) = border_color_disabled {
        bc
    } else { palette.background.strong.color };

    // placeholder
    let pc_active = if let Some(pc) = placeholder_color_active {
        pc
    } else { palette.secondary.base.color };

    let pc_hovered = if let Some(pc) = placeholder_color_hovered {
        pc
    } else { palette.secondary.base.color };

    let pc_focused = if let Some(pc) = placeholder_color_focused {
        pc
    } else { palette.secondary.base.color };

     let pc_disabled = if let Some(pc) = placeholder_color_disabled {
        pc
    } else { palette.background.strongest.color };

    // value
    let vc_active = if let Some(vc) = value_color_active {
        vc
    } else { palette.background.base.text };

    let vc_hovered = if let Some(vc) = value_color_hovered {
        vc
    } else { palette.background.base.text };

    let vc_focused = if let Some(vc) = value_color_focused {
        vc
    } else { palette.background.base.text };

    let vc_disabled = if let Some(vc) = value_color_disabled {
        vc
    } else { palette.secondary.base.color };

    // selection
    let sc_active = if let Some(sc) = selection_color_active {
        sc
    } else { palette.primary.weak.color };

    let sc_hovered = if let Some(sc) = selection_color_hovered {
        sc
    } else { palette.primary.weak.color };

    let sc_focused = if let Some(sc) = selection_color_focused {
        sc
    } else { palette.primary.weak.color };

    let sc_disabled = if let Some(sc) = selection_color_disabled {
        sc
    } else { palette.primary.weak.color };


    let active = Style {
        background: iced::Background::Color(palette.background.base.color),
        border: iced::Border {
            radius: br.into(),
            width: bw,
            color: bc_active,
        },
        icon: palette.background.weak.text,
        placeholder: pc_active,
        value: vc_active,
        selection: sc_active,
    };

    let hovered = Style {
        background: iced::Background::Color(palette.background.base.color),
        border: iced::Border {
            radius: br.into(),
            width: bw,
            color: bc_hovered,
        },
        icon: palette.background.weak.text,
        placeholder: pc_hovered,
        value: vc_hovered,
        selection: sc_hovered,
    };

    let focused = Style {
        background: iced::Background::Color(palette.background.base.color),
        border: iced::Border {
            radius: br.into(),
            width: bw,
            color: bc_focused,
        },
        icon: palette.background.weak.text,
        placeholder: pc_focused,
        value: vc_focused,
        selection: sc_focused,
    };

    let disabled = Style {
        background: iced::Background::Color(palette.background.base.color),
        border: iced::Border {
            radius: br.into(),
            width: bw,
            color: bc_disabled,
        },
        icon: palette.background.weak.text,
        placeholder: pc_disabled,
        value: vc_disabled,
        selection: sc_disabled,
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
    
    BorderColorActive,
    BorderColorActiveAlpha,
    BorderColorDisabled,
    BorderColorDisabledAlpha,
    BorderColorFocused,
    BorderColorFocusedAlpha,
    BorderColorHovered,
    BorderColorHoveredAlpha,
    BorderRgbaActive,
    BorderRgbaDisabled,
    BorderRgbaFocused,
    BorderRgbaHovered,
    
    BorderRadius,
    BorderWidth,

    PlaceholderColorActive,
    PlaceholderColorActiveAlpha,
    PlaceholderColorDisabled,
    PlaceholderColorDisabledAlpha,
    PlaceholderColorFocused,
    PlaceholderColorFocusedAlpha,
    PlaceholderColorHovered,
    PlaceholderColorHoveredAlpha,
    PlaceholderRgbaActive,
    PlaceholderRgbaDisabled,
    PlaceholderRgbaFocused,
    PlaceholderRgbaHovered,

    SelectionColorActive,
    SelectionColorActiveAlpha,
    SelectionColorDisabled,
    SelectionColorDisabledAlpha,
    SelectionColorFocused,
    SelectionColorFocusedAlpha,
    SelectionColorHovered,
    SelectionColorHoveredAlpha,
    SelectionRgbaActive,
    SelectionRgbaDisabled,
    SelectionRgbaFocused,
    SelectionRgbaHovered,

    ValueColorActive,
    ValueColorActiveAlpha,
    ValueColorDisabled,
    ValueColorDisabledAlpha,
    ValueColorFocused,
    ValueColorFocusedAlpha,
    ValueColorHovered,
    ValueColorHoveredAlpha,
    ValueRgbaActive,
    ValueRgbaDisabled,
    ValueRgbaFocused,
    ValueRgbaHovered,
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
            
            TextInputStyleParam::BorderColorActive => set_t_value(&mut self.border_color_active, value, "TextInputStyleParam::BorderColorActive"),
            TextInputStyleParam::BorderColorActiveAlpha => set_t_value(&mut self.border_color_active_alpha, value, "TextInputStyleParam::BorderColorActiveAlpha"),
            TextInputStyleParam::BorderColorDisabled => set_t_value(&mut self.border_color_disabled, value, "TextInputStyleParam::BorderColorDisabled"),
            TextInputStyleParam::BorderColorDisabledAlpha => set_t_value(&mut self.border_color_disabled_alpha, value, "TextInputStyleParam::BorderColorDisabledAlpha"),
            TextInputStyleParam::BorderColorFocused => set_t_value(&mut self.border_color_focused, value, "TextInputStyleParam::BorderColorFocused"),
            TextInputStyleParam::BorderColorFocusedAlpha => set_t_value(&mut self.border_color_focused_alpha, value, "TextInputStyleParam::BorderColorFocusedAlpha"),
            TextInputStyleParam::BorderColorHovered => set_t_value(&mut self.border_color_hovered, value, "TextInputStyleParam::BorderColorHovered"),
            TextInputStyleParam::BorderColorHoveredAlpha => set_t_value(&mut self.border_color_hovered_alpha, value, "TextInputStyleParam::BorderColorHoveredAlpha"),
            TextInputStyleParam::BorderRgbaActive => set_t_value(&mut self.border_rgba_active, value, "TextInputStyleParam::BorderRgbaActive"),
            TextInputStyleParam::BorderRgbaDisabled => set_t_value(&mut self.border_rgba_disabled, value, "TextInputStyleParam::BorderRgbaDisabled"),
            TextInputStyleParam::BorderRgbaFocused => set_t_value(&mut self.border_rgba_focused, value, "TextInputStyleParam::BorderRgbaFocused"),
            TextInputStyleParam::BorderRgbaHovered => set_t_value(&mut self.border_rgba_hovered, value, "TextInputStyleParam::BorderRgbaHovered"),

            TextInputStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "TextInputStyleParam::BorderRadius"),
            TextInputStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "TextInputStyleParam::BorderWidth"),
            
            TextInputStyleParam::PlaceholderColorActive => set_t_value(&mut self.placeholder_color_active, value, "TextInputStyleParam::PlaceholderColorActive"),
            TextInputStyleParam::PlaceholderColorActiveAlpha => set_t_value(&mut self.placeholder_color_active_alpha, value, "TextInputStyleParam::PlaceholderColorActiveAlpha"),
            TextInputStyleParam::PlaceholderColorDisabled => set_t_value(&mut self.placeholder_color_disabled, value, "TextInputStyleParam::PlaceholderColorDisabled"),
            TextInputStyleParam::PlaceholderColorDisabledAlpha => set_t_value(&mut self.placeholder_color_disabled_alpha, value, "TextInputStyleParam::PlaceholderColorDisabledAlpha"),
            TextInputStyleParam::PlaceholderColorFocused => set_t_value(&mut self.placeholder_color_focused, value, "TextInputStyleParam::PlaceholderColorFocused"),
            TextInputStyleParam::PlaceholderColorFocusedAlpha => set_t_value(&mut self.placeholder_color_focused_alpha, value, "TextInputStyleParam::PlaceholderColorFocusedAlpha"),
            TextInputStyleParam::PlaceholderColorHovered => set_t_value(&mut self.placeholder_color_hovered, value, "TextInputStyleParam::PlaceholderColorHovered"),
            TextInputStyleParam::PlaceholderColorHoveredAlpha => set_t_value(&mut self.placeholder_color_hovered_alpha, value, "TextInputStyleParam::PlaceholderColorHoveredAlpha"),
            TextInputStyleParam::PlaceholderRgbaActive => set_t_value(&mut self.placeholder_rgba_active, value, "TextInputStyleParam::PlaceholderRgbaActive"),
            TextInputStyleParam::PlaceholderRgbaDisabled => set_t_value(&mut self.placeholder_rgba_disabled, value, "TextInputStyleParam::PlaceholderRgbaDisabled"),
            TextInputStyleParam::PlaceholderRgbaFocused => set_t_value(&mut self.placeholder_rgba_focused, value, "TextInputStyleParam::PlaceholderRgbaFocused"),
            TextInputStyleParam::PlaceholderRgbaHovered => set_t_value(&mut self.placeholder_rgba_hovered, value, "TextInputStyleParam::PlaceholderRgbaHovered"),

            TextInputStyleParam::SelectionColorActive => set_t_value(&mut self.selection_color_active, value, "TextInputStyleParam::SelectionColorActive"),
            TextInputStyleParam::SelectionColorActiveAlpha => set_t_value(&mut self.selection_color_active_alpha, value, "TextInputStyleParam::SelectionColorActiveAlpha"),
            TextInputStyleParam::SelectionRgbaActive => set_t_value(&mut self.selection_rgba_active, value, "TextInputStyleParam::SelectionRgbaActive"),
            TextInputStyleParam::SelectionColorDisabled => set_t_value(&mut self.selection_color_disabled, value, "TextInputStyleParam::SelectionColorDisabled"),
            TextInputStyleParam::SelectionColorDisabledAlpha => set_t_value(&mut self.selection_color_disabled_alpha, value, "TextInputStyleParam::SelectionColorDisabledAlpha"),
            TextInputStyleParam::SelectionColorFocused => set_t_value(&mut self.selection_color_focused, value, "TextInputStyleParam::SelectionColorFocused"),
            TextInputStyleParam::SelectionColorFocusedAlpha => set_t_value(&mut self.selection_color_focused_alpha, value, "TextInputStyleParam::SelectionColorFocusedAlpha"),
            TextInputStyleParam::SelectionColorHovered => set_t_value(&mut self.selection_color_hovered, value, "TextInputStyleParam::SelectionColorHovered"),
            TextInputStyleParam::SelectionColorHoveredAlpha => set_t_value(&mut self.selection_color_hovered_alpha, value, "TextInputStyleParam::SelectionColorHoveredAlpha"),
            TextInputStyleParam::SelectionRgbaDisabled => set_t_value(&mut self.selection_rgba_disabled, value, "TextInputStyleParam::SelectionRgbaDisabled"),
            TextInputStyleParam::SelectionRgbaFocused => set_t_value(&mut self.selection_rgba_focused, value, "TextInputStyleParam::SelectionRgbaFocused"),
            TextInputStyleParam::SelectionRgbaHovered => set_t_value(&mut self.selection_rgba_hovered, value, "TextInputStyleParam::SelectionRgbaHovered"),

            TextInputStyleParam::ValueColorActive => set_t_value(&mut self.value_color_active, value, "TextInputStyleParam::ValueColorActive"),
            TextInputStyleParam::ValueColorActiveAlpha => set_t_value(&mut self.value_color_active_alpha, value, "TextInputStyleParam::ValueColorActiveAlpha"),
            TextInputStyleParam::ValueRgbaActive => set_t_value(&mut self.value_rgba_active, value, "TextInputStyleParam::ValueRgbaActive"),
            TextInputStyleParam::ValueColorDisabled => set_t_value(&mut self.value_color_disabled, value, "TextInputStyleParam::ValueColorDisabled"),
            TextInputStyleParam::ValueColorDisabledAlpha => set_t_value(&mut self.value_color_disabled_alpha, value, "TextInputStyleParam::ValueColorDisabledAlpha"),
            TextInputStyleParam::ValueColorFocused => set_t_value(&mut self.value_color_focused, value, "TextInputStyleParam::ValueColorFocused"),
            TextInputStyleParam::ValueColorFocusedAlpha => set_t_value(&mut self.value_color_focused_alpha, value, "TextInputStyleParam::ValueColorFocusedAlpha"),
            TextInputStyleParam::ValueColorHovered => set_t_value(&mut self.value_color_hovered, value, "TextInputStyleParam::ValueColorHovered"),
            TextInputStyleParam::ValueColorHoveredAlpha => set_t_value(&mut self.value_color_hovered_alpha, value, "TextInputStyleParam::ValueColorHoveredAlpha"),
            TextInputStyleParam::ValueRgbaDisabled => set_t_value(&mut self.value_rgba_disabled, value, "TextInputStyleParam::ValueRgbaDisabled"),
            TextInputStyleParam::ValueRgbaFocused => set_t_value(&mut self.value_rgba_focused, value, "TextInputStyleParam::ValueRgbaFocused"),
            TextInputStyleParam::ValueRgbaHovered => set_t_value(&mut self.value_rgba_hovered, value, "TextInputStyleParam::ValueRgbaHovered"),
        }
    }
}
