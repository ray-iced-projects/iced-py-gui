//! ipg_color_picker
use std::collections::HashMap;

use crate::graphics::bootstrap_arrow::Arrow;
use crate::graphics::colors::Color;
use crate::state::Widgets;
use crate::widgets::ipg_button::{BtnStatus, ButtonStyleStd};
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};
use crate::IpgState;
use crate::app::Message;
use crate::py_api::helpers::{get_len, get_padding};
use super::callbacks::{invoke_callback, invoke_callback_with_args};

use iced::widget::{Button, button, text};
use iced::{Element, Theme};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct ColorPicker {
    pub id: usize,
    pub show: bool,
    pub color: Option<Color>,
    pub color_alpha: Option<f32>,
    pub color_rgba: Option<[f32; 4]>,
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
        
        // if !self.show {
        //     return Some(btn.map(move |message| Message::ColorPicker(self.id, message)));
        // }

        let color: iced::Color = if let Some(c) = 
            Color::rgba_ipg_color_to_iced(self.color_rgba, &self.color, self.color_alpha) {
                c
            } else {
                [0.5, 0.2, 0.7, 1.0].into()
            };

        let color_picker: Element<ColPikMessage> = crate::iced_aw_widgets::color_picker::ColorPicker::new(
                                        true,
                                        color,
                                        btn,
                                        ColPikMessage::OnCancel,
                                        ColPikMessage::OnSubmit,
                                    ).into();

        Some(color_picker.map(move |message| Message::ColorPicker(self.id, message)))

    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum ColPikMessage {
    OnPress,
    OnCancel,
    OnSubmit(iced::Color),
}

pub fn color_picker_callback(state: &mut IpgState, id: usize, message: ColPikMessage) {
    // Update widget state directly
    if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
        match &message {
            ColPikMessage::OnCancel => {
                cp.show = false;
            },
            ColPikMessage::OnSubmit(color) => {
                cp.show = false;
                cp.color_rgba = Some([color.r, color.g, color.b, color.a]);
            },
            ColPikMessage::OnPress => {
                cp.show = true;
            },
        }
    }

    // Invoke Python callback
    match message {
        ColPikMessage::OnCancel => {
            invoke_callback(id, "on_cancel", "ColorPicker");
        },
        ColPikMessage::OnSubmit(color) => {
            invoke_callback_with_args(id, "on_select", "ColorPicker", convert_color_to_list(color),
                "def cb(wid: int, color: list[float])");
        },
        ColPikMessage::OnPress => {
            invoke_callback(id, "on_press", "ColorPicker");
        },
    }
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
            ColorPickerParam::Color => set_t_value(&mut self.color, value, "ColorPickerParam::Color"),
            ColorPickerParam::ColorAlpha => set_t_value(&mut self.color_alpha, value, "ColorPickerParam::ColorAlpha"),
            ColorPickerParam::ColorRgba => set_t_value(&mut self.color_rgba, value, "ColorPickerParam::ColorRgba"),
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
        }
    }
}
