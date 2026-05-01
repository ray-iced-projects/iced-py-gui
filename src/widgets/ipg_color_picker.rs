//! ipg_color_picker
use std::collections::HashMap;

use crate::graphics::bootstrap_arrow::Arrow;
use crate::state::Widgets;
use crate::widgets::ipg_button::ButtonStyleStd;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};
use crate::IpgState;
use crate::app::Message;
use crate::py_api::helpers::{get_len, get_padding};
use super::callbacks::{invoke_callback, invoke_callback_with_args};

use crate::ipg_widgets::ipg_color_picker::color_picker::{Position, Tooltip};

use iced::widget::{Button, button, text};
use iced::{Element, Theme};
use iced::time::seconds;

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct ColorPicker {
    pub id: usize,
    pub position_follow_cursor: Option<bool>,
    pub position_bottom: Option<bool>,
    pub position_left: Option<bool>,
    pub position_top: Option<bool>,
    pub position_right: Option<bool>,
    pub text: Option<String>,
    pub gap: Option<u32>,
    pub snap_within_viewport: Option<bool>,
    pub delay_sec: Option<u64>,
    pub show: bool,
    //button related
    pub label: Option<String>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub padding: Option<Vec<f32>>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_std: Option<ButtonStyleStd>,
    pub style_arrow: Option<Arrow>,
}

impl ColorPicker {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        widgets: &HashMap<usize, Widgets>,
        ) -> Option<Element<'a, Message>> {
        
        let label = 
            if let Some(lbl) = self.label.clone() {
                text(lbl)
            } else {
                text("Select Color".to_string())
            };
        

        let _style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_button_style).cloned();

        let btn: Element<ColPikMessage> = 
            Button::new(label)
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height))
                .padding(get_padding(&self.padding))
                .on_press(ColPikMessage::OnPress)  
                .style(move |theme: &Theme, status| {
                    match &self.style_std {
                        Some(std) => std.to_iced(theme, status),
                        None => button::primary(theme, status),
                    }
                    
                })
                .into();
        
        if !self.show {
            return Some(btn.map(move |message| Message::ColorPicker(self.id, message)));
        }

        let position: Position = 
        if self.position_follow_cursor == Some(true) {
            Position::FollowCursor
        } else if self.position_bottom == Some(true) {
            Position::Bottom
        } else if self.position_left == Some(true) {
            Position::Left
        } else if self.position_right == Some(true) {
            Position::Right
        } else {
            Position::Top
        };

        let tooltip: Element<'a, Message> = 
            if let Some(txt) = &self.text {
                    text(txt).into()
                } else {
                    if content.len() < 2 {
                        text("If you are not using the text parameter,
                            \nyou must use two widgets/containers").into()
                    } else {
                        content.remove(0)
                    }
                };

        // let color: iced::Color = if let Some(c) = 
        //     Color::rgba_ipg_color_to_iced(self.color_rgba, &self.color, self.color_alpha) {
        //         c
        //     } else {
        //         [0.5, 0.2, 0.7, 1.0].into()
        //     };

        let color_picker: Element<'a, Message> = Tooltip::new(
                content.remove(0),
                tooltip,
                position,
                )
                .gap(self.gap.unwrap_or(0))
                .snap_within_viewport(self.snap_within_viewport.unwrap_or(false))
                .delay(seconds(self.delay_sec.unwrap_or(0)))
                .into();

        // Some(color_picker.map(move |message| Message::ColorPicker(self.id, message)))
        Some(color_picker)
        

    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum ColPikMessage {
    OnPress,
    OnCancel,
    OnSubmit(iced::Color),
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ColorPickerParam {
    Clip,
    ColorAlpha,
    ColorRgba,
    Color,
    Fill, 
    HeightFill,
    Height, 
    Label,
    Padding,
    Show,
    StyleArrow,  
    StyleId,
    StyleStd,
    WidthFill,  
    Width,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ColorPickerStyleParam {
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRbga,
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
    BorderRadius,
    BorderWidth,
    ShadowColor,
    ShadowColorAlpha,
    ShadowRgba,
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlurRadius,
    TextColor,
    TextColorAlpha,
    TextRgba
}

fn convert_color_to_list(color: iced::Color) -> Vec<f64> {

    vec![
        rnd_2(color.r),
        rnd_2(color.g),
        rnd_2(color.b),
        rnd_2(color.a),
    ]
}

fn rnd_2(rgba: f32) -> f64 {
    let num = rgba as f64 * 100.0;
    num.round()/100.0
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for ColorPicker {
    type Param = ColorPickerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ColorPickerParam::Clip => set_t_value(&mut self.clip, value, "ColorPickerParam::Clip"),
            ColorPickerParam::Fill => set_t_value(&mut self.fill, value, "ColorPickerParam::Fill"),
            ColorPickerParam::Height => set_t_value(&mut self.height, value, "ColorPickerParam::Height"),
            ColorPickerParam::HeightFill => set_t_value(&mut self.height, value, "ColorPickerParam::HeightFill"),
            ColorPickerParam::Label => set_t_value(&mut self.label, value, "ColorPickerParam::Label"),
            ColorPickerParam::Padding => set_t_value(&mut self.padding, value, "ColorPickerParam::Padding"),
            ColorPickerParam::Show => set_t_value(&mut self.show, value, "ColorPickerParam::Show"),
            ColorPickerParam::StyleArrow => set_t_value(&mut self.style_arrow, value, "ColorPickerParam::StyleArrow"),
            ColorPickerParam::StyleId => set_t_value(&mut self.style_id, value, "ColorPickerParam::StyleId"),
            ColorPickerParam::StyleStd => set_t_value(&mut self.style_std, value, "ColorPickerParam::StyleStd"),
            ColorPickerParam::Width => set_t_value(&mut self.width, value, "ColorPickerParam::Width"),
            ColorPickerParam::WidthFill => set_t_value(&mut self.width, value, "ColorPickerParam::WidthFill"),
            ColorPickerParam::ColorAlpha => todo!(),
            ColorPickerParam::ColorRgba => todo!(),
            ColorPickerParam::Color => todo!(),
        }
    }
}
