//! ipg_color_picker
use std::collections::HashMap;

use crate::graphics::bootstrap_arrow::IpgArrow;
use crate::state::IpgWidgets;
use crate::widgets::ipg_button::{IpgButtonStyleStd, extract_button_style_standard};
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool, set_height, set_height_fill, set_iced_color, set_opt_bool, set_opt_ipg_arrow, set_opt_string, set_opt_usize, set_opt_vec_f32, set_width, set_width_fill};
use crate::IpgState;
use crate::app::Message;
use crate::py_api::helpers::get_padding;
use super::callbacks::{invoke_callback, invoke_callback_with_args};

use iced::widget::{Button, button, text};
use iced::{Color, Element, Length, Theme};
use iced_aw::ColorPicker;

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct IpgColorPicker {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub color: Color,
    //button related
    pub label: Option<String>,
    pub width: Length,
    pub height: Length,
    pub padding: Option<Vec<f32>>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgButtonStyleStd>,
    pub style_arrow: Option<IpgArrow>,
}

impl IpgColorPicker {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, IpgWidgets>,
        ) -> Option<Element<'a, Message>> {
        
        let label = 
            if let Some(lbl) = self.label.clone() {
                text(lbl)
            } else {
                text("Select Color".to_string())
            };
        

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(IpgWidgets::as_button_style).cloned();


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

        let color_picker: Element<ColPikMessage> = ColorPicker::new(
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
    OnSubmit(Color),
}

pub fn color_picker_callback(state: &mut IpgState, id: usize, message: ColPikMessage) {
    // Update widget state directly
    if let Some(IpgWidgets::IpgColorPicker(cp)) = state.widgets.get_mut(&id) {
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


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgColorPickerParam {
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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgColorPickerStyleParam {
    BackgroundIpgColor,
    BackgroundRbga,
    BackgroundIpgColorHovered,
    BackgroundIpgRgbaHovered,
    BorderIpgColor,
    BorderRgba,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgba,
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlurRadius,
    TextIpgColor,
    TextRgbaColor
}

fn convert_color_to_list(color: Color) -> Vec<f64> {

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

impl WidgetParamUpdate for IpgColorPicker {
    type Param = IpgColorPickerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgColorPickerParam::ArrowStyle => set_opt_ipg_arrow(&mut self.style_arrow, value, "ArrowStyle"),
            IpgColorPickerParam::Clip => set_opt_bool(&mut self.clip, value, "Clip"),
            IpgColorPickerParam::Color => set_iced_color(&mut self.color, value, "Color"),
            IpgColorPickerParam::Height => set_height(&mut self.height, value, "Height"),
            IpgColorPickerParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            IpgColorPickerParam::Label => set_opt_string(&mut self.label, value, "Label"),
            IpgColorPickerParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            IpgColorPickerParam::Show => set_bool(&mut self.show, value, "Show"),
            IpgColorPickerParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            IpgColorPickerParam::StyleStandard => {
                self.style_standard = Some(extract_button_style_standard(value, "StyleStandard"));
            },
            IpgColorPickerParam::Width => set_width(&mut self.width, value, "Width"),
            IpgColorPickerParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_color_picker() -> IpgColorPicker {
        IpgColorPicker {
            id: 0,
            parent_id: String::new(),
            show: false,
            color: Color::BLACK,
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
        cp.param_update(IpgColorPickerParam::Clip, &py_obj(true));
        assert_eq!(cp.clip, Some(true));
        cp.param_update(IpgColorPickerParam::Clip, &py_none());
        assert_eq!(cp.clip, None);
    }

    #[test]
    fn test_color() {
        let mut cp = make_color_picker();
        cp.param_update(IpgColorPickerParam::Color, &py_obj(vec![1.0f32, 0.5, 0.25, 1.0]));
        assert_eq!(cp.color, Color::from_rgba(1.0, 0.5, 0.25, 1.0));
    }

    #[test]
    fn test_height() {
        let mut cp = make_color_picker();
        cp.param_update(IpgColorPickerParam::Height, &py_obj(50.0f32));
        assert_eq!(cp.height, Length::Fixed(50.0));
    }

    #[test]
    fn test_height_fill() {
        let mut cp = make_color_picker();
        cp.param_update(IpgColorPickerParam::HeightFill, &py_obj(true));
        assert_eq!(cp.height, Length::Fill);
    }

    #[test]
    fn test_label() {
        let mut cp = make_color_picker();
        cp.param_update(IpgColorPickerParam::Label, &py_obj("Pick".to_string()));
        assert_eq!(cp.label, Some("Pick".to_string()));
        cp.param_update(IpgColorPickerParam::Label, &py_none());
        assert_eq!(cp.label, None);
    }

    #[test]
    fn test_padding() {
        let mut cp = make_color_picker();
        cp.param_update(IpgColorPickerParam::Padding, &py_obj(vec![5.0f32, 10.0]));
        assert_eq!(cp.padding, Some(vec![5.0, 10.0]));
        cp.param_update(IpgColorPickerParam::Padding, &py_none());
        assert_eq!(cp.padding, None);
    }

    #[test]
    fn test_show() {
        let mut cp = make_color_picker();
        cp.param_update(IpgColorPickerParam::Show, &py_obj(true));
        assert!(cp.show);
    }

    #[test]
    fn test_style_id() {
        let mut cp = make_color_picker();
        cp.param_update(IpgColorPickerParam::StyleId, &py_obj(42usize));
        assert_eq!(cp.style_id, Some(42));
        cp.param_update(IpgColorPickerParam::StyleId, &py_none());
        assert_eq!(cp.style_id, None);
    }

    #[test]
    fn test_width() {
        let mut cp = make_color_picker();
        cp.param_update(IpgColorPickerParam::Width, &py_obj(200.0f32));
        assert_eq!(cp.width, Length::Fixed(200.0));
    }

    #[test]
    fn test_width_fill() {
        let mut cp = make_color_picker();
        cp.param_update(IpgColorPickerParam::WidthFill, &py_obj(true));
        assert_eq!(cp.width, Length::Fill);
    }
}

