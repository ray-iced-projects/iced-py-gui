//! ipg_color_picker
use std::collections::HashMap;

use crate::graphics::bootstrap_arrow::Arrow;
use crate::state::Widgets;
use crate::widgets::ipg_button::{ButtonStyleStd, extract_button_style_standard};
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool, set_height, set_height_fill, set_iced_color, set_opt_bool, set_opt_ipg_arrow, set_opt_string, set_opt_usize, set_opt_vec_f32, set_width, set_width_fill};
use crate::IpgState;
use crate::app::Message;
use crate::py_api::helpers::get_padding;
use super::callbacks::{invoke_callback, invoke_callback_with_args};

use iced::widget::{Button, button, text};
use iced::{Element, Length, Theme};
use iced_aw;

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct ColorPicker {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub color: iced::Color,
    //button related
    pub label: Option<String>,
    pub width: Length,
    pub height: Length,
    pub padding: Option<Vec<f32>>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_standard: Option<ButtonStyleStd>,
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
        

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_button_style).cloned();


        let btn: Element<ColPikMessage> = 
            Button::new(label)
                .height(self.height)
                .padding(get_padding(&self.padding))
                .width(self.width)
                .on_press(ColPikMessage::OnPress)
                .style(move|theme: &Theme, status| {   
                    if let Some(st) = &style_opt {
                            st.to_iced(theme, status, &self.style_standard)
                        } else {
                        match &self.style_standard {
                                Some(std) => std.to_iced(theme, status),
                                None => button::primary(theme, status),
                            }
                        }
                    }
                )
                .into();

        if !self.show {
            return Some(btn.map(move |message| Message::ColorPicker(self.id, message)));
        }

        let color_picker: Element<ColPikMessage> = iced_aw::ColorPicker::new(
                                        self.show,
                                        self.color,
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
                cp.color = *color;
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
            invoke_callback_with_args(id, "on_select", "ColorPicker", convert_color_to_list(color));
        },
        ColPikMessage::OnPress => {
            invoke_callback(id, "on_press", "ColorPicker");
        },
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ColorPickerParam {
    ArrowStyle,
    Clip,
    Color,
    Height,
    HeightFill,
    Label,
    Padding,
    Show,
    StyleId,
    StyleStandard,
    Width,
    WidthFill,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ColorPickerStyleParam {
    BackgroundColor,
    BackgroundRbga,
    BackgroundColorHovered,
    BackgroundRgbaHovered,
    BorderColor,
    BorderRgba,
    BorderRadius,
    BorderWidth,
    ShadowColor,
    ShadowRgba,
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlurRadius,
    TextColor,
    TextRgbaColor
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
            ColorPickerParam::ArrowStyle => set_opt_ipg_arrow(&mut self.style_arrow, value, "ArrowStyle"),
            ColorPickerParam::Clip => set_opt_bool(&mut self.clip, value, "Clip"),
            ColorPickerParam::Color => set_iced_color(&mut self.color, value, "Color"),
            ColorPickerParam::Height => set_height(&mut self.height, value, "Height"),
            ColorPickerParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            ColorPickerParam::Label => set_opt_string(&mut self.label, value, "Label"),
            ColorPickerParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            ColorPickerParam::Show => set_bool(&mut self.show, value, "Show"),
            ColorPickerParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            ColorPickerParam::StyleStandard => {
                self.style_standard = Some(extract_button_style_standard(value, "StyleStandard"));
            },
            ColorPickerParam::Width => set_width(&mut self.width, value, "Width"),
            ColorPickerParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_color_picker() -> ColorPicker {
        ColorPicker {
            id: 0,
            parent_id: String::new(),
            show: false,
            color: iced::Color::BLACK,
            label: None,
            width: Length::Shrink,
            height: Length::Shrink,
            padding: None,
            clip: None,
            style_id: None,
            style_standard: None,
            style_arrow: None,
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

    #[test]
    fn test_clip() {
        let mut cp = make_color_picker();
        cp.param_update(ColorPickerParam::Clip, &py_obj(true));
        assert_eq!(cp.clip, Some(true));
        cp.param_update(ColorPickerParam::Clip, &py_none());
        assert_eq!(cp.clip, None);
    }

    #[test]
    fn test_color() {
        let mut cp = make_color_picker();
        cp.param_update(ColorPickerParam::Color, &py_obj(vec![1.0f32, 0.5, 0.25, 1.0]));
        assert_eq!(cp.color, iced::Color::from_rgba(1.0, 0.5, 0.25, 1.0));
    }

    #[test]
    fn test_height() {
        let mut cp = make_color_picker();
        cp.param_update(ColorPickerParam::Height, &py_obj(50.0f32));
        assert_eq!(cp.height, Length::Fixed(50.0));
    }

    #[test]
    fn test_height_fill() {
        let mut cp = make_color_picker();
        cp.param_update(ColorPickerParam::HeightFill, &py_obj(true));
        assert_eq!(cp.height, Length::Fill);
    }

    #[test]
    fn test_label() {
        let mut cp = make_color_picker();
        cp.param_update(ColorPickerParam::Label, &py_obj("Pick".to_string()));
        assert_eq!(cp.label, Some("Pick".to_string()));
        cp.param_update(ColorPickerParam::Label, &py_none());
        assert_eq!(cp.label, None);
    }

    #[test]
    fn test_padding() {
        let mut cp = make_color_picker();
        cp.param_update(ColorPickerParam::Padding, &py_obj(vec![5.0f32, 10.0]));
        assert_eq!(cp.padding, Some(vec![5.0, 10.0]));
        cp.param_update(ColorPickerParam::Padding, &py_none());
        assert_eq!(cp.padding, None);
    }

    #[test]
    fn test_show() {
        let mut cp = make_color_picker();
        cp.param_update(ColorPickerParam::Show, &py_obj(true));
        assert!(cp.show);
    }

    #[test]
    fn test_style_id() {
        let mut cp = make_color_picker();
        cp.param_update(ColorPickerParam::StyleId, &py_obj(42usize));
        assert_eq!(cp.style_id, Some(42));
        cp.param_update(ColorPickerParam::StyleId, &py_none());
        assert_eq!(cp.style_id, None);
    }

    #[test]
    fn test_width() {
        let mut cp = make_color_picker();
        cp.param_update(ColorPickerParam::Width, &py_obj(200.0f32));
        assert_eq!(cp.width, Length::Fixed(200.0));
    }

    #[test]
    fn test_width_fill() {
        let mut cp = make_color_picker();
        cp.param_update(ColorPickerParam::WidthFill, &py_obj(true));
        assert_eq!(cp.width, Length::Fill);
    }
}

