//! ipg_text_input
#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::widget::text_input;
use iced::widget::text_input::{Style, Status};
use iced::{Color, Element, Length, Theme};
use iced::widget::TextInput;
use iced::theme::palette;

use pyo3::pyclass;
use pyo3::{Py, PyAny};

use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::widgets::enums::AlignX;
use crate::widgets::styling::create_custom_theme;
use crate::{IpgState};
use crate::state::IpgWidgets;
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_string, set_opt_bool, set_width,
    set_opt_vec_f32, set_opt_f32, set_opt_usize, set_opt_iced_color,
};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgTextInput {
    pub id: usize,
    pub parent_id: String,
    pub placeholder: String,
    pub value: String,
    pub is_secure: Option<bool>,
    pub width: Length,
    pub padding: Option<Vec<f32>>,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub align_x: Option<AlignX>,
    // icon: Option<Message>,
    pub font_id: Option<usize>,
    pub style_id: Option<usize>,
    pub show: bool,
}

impl IpgTextInput { 

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }
    
    pub fn construct<'a>( 
        &'a self,
        widgets: &HashMap<usize, IpgWidgets>,
    ) -> Option<Element<'a, Message>> {
       
        if !self.show {
            return None
        }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(IpgWidgets::as_text_input_style).cloned();

        let txt: TextInput<'_, TIMessage> =  
            TextInput::new(
                    self.placeholder.as_str(), 
                    self.value.as_str()
                )
                .on_input(TIMessage::OnInput)
                .on_submit(TIMessage::OnSubmit(self.value.clone()))
                .on_paste(TIMessage::OnPaste)
                .secure(self.is_secure.unwrap_or(false))
                .width(self.width)
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

        let txt = if let Some(align) = &self.align_x {
            txt.align_x(align.to_iced())
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
            if let Some(IpgWidgets::IpgTextInput(ti)) = state.widgets.get_mut(&id) {
                ti.value = value.clone();
            }
            invoke_callback_with_args(id, "on_input", "TextInput", value);
        },
        TIMessage::OnSubmit(value) => {
            if let Some(IpgWidgets::IpgTextInput(ti)) = state.widgets.get_mut(&id) {
                ti.value = String::new();
            }
            invoke_callback_with_args(id, "on_submit", "TextInput", value);
        }
        TIMessage::OnPaste(value) => {
            if let Some(IpgWidgets::IpgTextInput(ti)) = state.widgets.get_mut(&id) {
                ti.value = value.clone();
            }
            invoke_callback_with_args(id, "on_paste", "TextInput", value);
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
pub struct IpgTextInputStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub border_color_active: Option<Color>,
    pub border_color_hovered: Option<Color>,
    pub border_color_focused: Option<Color>,
    pub border_color_disabled: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<f32>,
    // pub icon_color: Option<Color>,
    pub placeholder_color_active: Option<Color>,
     pub placeholder_color_disabled: Option<Color>,
    pub value_color: Option<Color>,
    pub selection_color: Option<Color>,
}

impl IpgTextInputStyle {
    fn to_iced(
        &self,
        theme: &Theme, 
        status: Status, 
    ) -> Style {

    let custom_theme;
    
    let palette = if let Some(bkg) = self.background_color {
        let dark_mode = palette::is_dark(bkg);
        custom_theme = create_custom_theme(bkg, dark_mode);
        custom_theme.extended_palette()
    } else {
        theme.extended_palette()
    };

    let br = if let Some(br) = self.border_radius {
        br
    } else { 2.0 };

    let bw = if let Some(bw) = self.border_width {
        bw
    } else { 1.0 };

    let bc_active = if let Some(bc) = self.border_color_active {
        bc
    } else { palette.background.strong.color };

    let bc_hovered = if let Some(bc) = self.border_color_hovered {
        bc
    } else { palette.background.base.text };

    let bc_focused = if let Some(bc) = self.border_color_focused {
        bc
    } else { palette.primary.strong.color };

    let pc_active = if let Some(pc) = self.placeholder_color_active {
        pc
    } else { palette.secondary.base.color };

     let pc_disabled = if let Some(pc) = self.placeholder_color_disabled {
        pc
    } else { palette.background.strongest.color };

    let active = Style {
        background: iced::Background::Color(palette.background.base.color),
        border: iced::Border {
            radius: br.into(),
            width: bw,
            color: bc_active,
        },
        icon: palette.background.weak.text,
        placeholder: pc_active,
        value: palette.background.base.text,
        selection: palette.primary.weak.color,
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            border: iced::Border {
                color: bc_hovered,
                ..active.border
            },
            ..active
        },
        Status::Focused { .. } => Style {
            border: iced::Border {
                color: bc_focused,
                ..active.border
            },
            ..active
        },
        Status::Disabled => Style {
            background: iced::Background::Color(palette.background.weak.color),
            value: active.placeholder,
            placeholder: pc_disabled,
            ..active
        },
    }
}}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTextInputParam {
    IsSecure,
    LineHeight,
    Padding,
    Placeholder,
    Size,
    StyleId,
    Value,
    Width,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTextInputStyleParam {
    BackgroundColor,
    BorderColorActive,
    BorderColorHovered,
    BorderColorFocused,
    BorderColorDisabled,
    BorderWidth,
    BorderRadius,
    PlaceholderColorActive,
    PlaceholderColorDisabled,
    ValueColor,
    SelectionColor
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgTextInput {
    type Param = IpgTextInputParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgTextInputParam::Placeholder => set_string(&mut self.placeholder, value, "Placeholder"),
            IpgTextInputParam::Value => set_string(&mut self.value, value, "Value"),
            IpgTextInputParam::IsSecure => set_opt_bool(&mut self.is_secure, value, "IsSecure"),
            IpgTextInputParam::Width => set_width(&mut self.width, value, "Width"),
            IpgTextInputParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            IpgTextInputParam::Size => set_opt_f32(&mut self.size, value, "Size"),
            IpgTextInputParam::LineHeight => set_opt_f32(&mut self.line_height, value, "LineHeight"),
            IpgTextInputParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
        }
    }
}

impl WidgetParamUpdate for IpgTextInputStyle {
    type Param = IpgTextInputStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgTextInputStyleParam::BackgroundColor =>
                set_opt_iced_color(&mut self.background_color, value, "BackgroundColor"),
            IpgTextInputStyleParam::BorderColorActive =>
                set_opt_iced_color(&mut self.border_color_active, value, "BorderColorActive"),
            IpgTextInputStyleParam::BorderColorHovered =>
                set_opt_iced_color(&mut self.border_color_hovered, value, "BorderColorHovered"),
            IpgTextInputStyleParam::BorderColorFocused =>
                set_opt_iced_color(&mut self.border_color_focused, value, "BorderColorFocused"),
            IpgTextInputStyleParam::BorderColorDisabled =>
                set_opt_iced_color(&mut self.border_color_disabled, value, "BorderColorDisabled"),
            IpgTextInputStyleParam::BorderWidth =>
                set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            IpgTextInputStyleParam::BorderRadius =>
                set_opt_f32(&mut self.border_radius, value, "BorderRadius"),
            IpgTextInputStyleParam::PlaceholderColorActive =>
                set_opt_iced_color(&mut self.placeholder_color_active, value, "PlaceholderColorActive"),
            IpgTextInputStyleParam::PlaceholderColorDisabled =>
                set_opt_iced_color(&mut self.placeholder_color_disabled, value, "PlaceholderColorDisabled"),
            IpgTextInputStyleParam::ValueColor =>
                set_opt_iced_color(&mut self.value_color, value, "ValueColor"),
            IpgTextInputStyleParam::SelectionColor =>
                set_opt_iced_color(&mut self.selection_color, value, "SelectionColor"),
        }
    }
}
