//! ipg_text_input
#![allow(clippy::enum_variant_names)]

use std::collections::HashMap;

use iced::widget::text_input;
use iced::widget::text_input::{Style, Status};
use iced::{Element, Length, Theme, alignment};
use iced::widget;
use iced::theme::palette;

use pyo3::pyclass;
use pyo3::{Py, PyAny};

use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::widgets::styling::create_custom_theme;
use crate::{IpgState};
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_string, set_opt_bool, set_width,
    set_opt_vec_f32, set_opt_f32, set_opt_usize, set_opt_iced_color,
};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct TextInput {
    pub id: usize,
    pub parent_id: String,
    pub placeholder: String,
    pub value: String,
    pub is_secure: Option<bool>,
    pub width: Length,
    pub padding: Option<Vec<f32>>,
    pub size: Option<f32>,
    pub line_height: Option<f32>,
    pub align_left: Option<bool>,
    pub align_center: Option<bool>,
    pub align_right: Option<bool>,
    // icon: Option<Message>,
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

        // default
        let txt = txt.align_x(alignment::Horizontal::Center);

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
            invoke_callback_with_args(id, "on_input", "TextInput", value);
        },
        TIMessage::OnSubmit(value) => {
            if let Some(Widgets::TextInput(ti)) = state.widgets.get_mut(&id) {
                ti.value = String::new();
            }
            invoke_callback_with_args(id, "on_submit", "TextInput", value);
        }
        TIMessage::OnPaste(value) => {
            if let Some(Widgets::TextInput(ti)) = state.widgets.get_mut(&id) {
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
pub struct TextInputStyle {
    pub id: usize,
    pub background_color: Option<iced::Color>,
    pub border_color_active: Option<iced::Color>,
    pub border_color_hovered: Option<iced::Color>,
    pub border_color_focused: Option<iced::Color>,
    pub border_color_disabled: Option<iced::Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<f32>,
    // pub icon_color: Option<iced::Color>,
    pub placeholder_color_active: Option<iced::Color>,
     pub placeholder_color_disabled: Option<iced::Color>,
    pub value_color: Option<iced::Color>,
    pub selection_color: Option<iced::Color>,
}

impl TextInputStyle {
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
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum TextInputStyleParam {
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

impl WidgetParamUpdate for TextInput {
    type Param = TextInputParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TextInputParam::Placeholder => set_string(&mut self.placeholder, value, "Placeholder"),
            TextInputParam::Value => set_string(&mut self.value, value, "Value"),
            TextInputParam::IsSecure => set_opt_bool(&mut self.is_secure, value, "IsSecure"),
            TextInputParam::Width => set_width(&mut self.width, value, "Width"),
            TextInputParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            TextInputParam::Size => set_opt_f32(&mut self.size, value, "Size"),
            TextInputParam::LineHeight => set_opt_f32(&mut self.line_height, value, "LineHeight"),
            TextInputParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
        }
    }
}

impl WidgetParamUpdate for TextInputStyle {
    type Param = TextInputStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            TextInputStyleParam::BackgroundColor =>
                set_opt_iced_color(&mut self.background_color, value, "BackgroundColor"),
            TextInputStyleParam::BorderColorActive =>
                set_opt_iced_color(&mut self.border_color_active, value, "BorderColorActive"),
            TextInputStyleParam::BorderColorHovered =>
                set_opt_iced_color(&mut self.border_color_hovered, value, "BorderColorHovered"),
            TextInputStyleParam::BorderColorFocused =>
                set_opt_iced_color(&mut self.border_color_focused, value, "BorderColorFocused"),
            TextInputStyleParam::BorderColorDisabled =>
                set_opt_iced_color(&mut self.border_color_disabled, value, "BorderColorDisabled"),
            TextInputStyleParam::BorderWidth =>
                set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            TextInputStyleParam::BorderRadius =>
                set_opt_f32(&mut self.border_radius, value, "BorderRadius"),
            TextInputStyleParam::PlaceholderColorActive =>
                set_opt_iced_color(&mut self.placeholder_color_active, value, "PlaceholderColorActive"),
            TextInputStyleParam::PlaceholderColorDisabled =>
                set_opt_iced_color(&mut self.placeholder_color_disabled, value, "PlaceholderColorDisabled"),
            TextInputStyleParam::ValueColor =>
                set_opt_iced_color(&mut self.value_color, value, "ValueColor"),
            TextInputStyleParam::SelectionColor =>
                set_opt_iced_color(&mut self.selection_color, value, "SelectionColor"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_text_input() -> TextInput {
        TextInput {
            id: 0,
            parent_id: String::new(),
            placeholder: "enter".to_string(),
            value: String::new(),
            is_secure: None,
            width: Length::Shrink,
            padding: None,
            size: None,
            line_height: None,
            align_left: None,
            align_center: None,
            align_right: None,
            font_id: None,
            style_id: None,
            show: true,
        }
    }

    fn make_text_input_style() -> TextInputStyle {
        TextInputStyle {
            id: 0,
            background_color: None,
            border_color_active: None,
            border_color_hovered: None,
            border_color_focused: None,
            border_color_disabled: None,
            border_width: None,
            border_radius: None,
            placeholder_color_active: None,
            placeholder_color_disabled: None,
            value_color: None,
            selection_color: None,
        }
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::attach(|py| py.None().into_py_any(py).unwrap())
    }

    // -- TextInput param tests --

    #[test]
    fn test_placeholder() {
        let mut t = make_text_input();
        t.param_update(TextInputParam::Placeholder, &py_obj("type here".to_string()));
        assert_eq!(t.placeholder, "type here");
    }

    #[test]
    fn test_value() {
        let mut t = make_text_input();
        t.param_update(TextInputParam::Value, &py_obj("hello".to_string()));
        assert_eq!(t.value, "hello");
    }

    #[test]
    fn test_is_secure() {
        let mut t = make_text_input();
        t.param_update(TextInputParam::IsSecure, &py_obj(true));
        assert_eq!(t.is_secure, Some(true));
        t.param_update(TextInputParam::IsSecure, &py_none());
        assert_eq!(t.is_secure, None);
    }

    #[test]
    fn test_width() {
        let mut t = make_text_input();
        t.param_update(TextInputParam::Width, &py_obj(300.0f32));
        assert_eq!(t.width, Length::Fixed(300.0));
    }

    #[test]
    fn test_padding() {
        let mut t = make_text_input();
        t.param_update(TextInputParam::Padding, &py_obj(vec![5.0f32, 10.0]));
        assert_eq!(t.padding, Some(vec![5.0, 10.0]));
        t.param_update(TextInputParam::Padding, &py_none());
        assert_eq!(t.padding, None);
    }

    #[test]
    fn test_size() {
        let mut t = make_text_input();
        t.param_update(TextInputParam::Size, &py_obj(16.0f32));
        assert_eq!(t.size, Some(16.0));
    }

    #[test]
    fn test_line_height() {
        let mut t = make_text_input();
        t.param_update(TextInputParam::LineHeight, &py_obj(1.5f32));
        assert_eq!(t.line_height, Some(1.5));
    }

    #[test]
    fn test_style_id() {
        let mut t = make_text_input();
        t.param_update(TextInputParam::StyleId, &py_obj(4usize));
        assert_eq!(t.style_id, Some(4));
        t.param_update(TextInputParam::StyleId, &py_none());
        assert_eq!(t.style_id, None);
    }

    // -- TextInputStyle param tests --

    #[test]
    fn test_style_border_width() {
        let mut s = make_text_input_style();
        s.param_update(TextInputStyleParam::BorderWidth, &py_obj(2.0f32));
        assert_eq!(s.border_width, Some(2.0));
    }

    #[test]
    fn test_style_border_radius() {
        let mut s = make_text_input_style();
        s.param_update(TextInputStyleParam::BorderRadius, &py_obj(5.0f32));
        assert_eq!(s.border_radius, Some(5.0));
    }
}
