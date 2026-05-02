//! ipg_color_picker
use std::collections::HashMap;

use crate::graphics::bootstrap_arrow::Arrow;
use crate::state::Widgets;
use crate::widgets::ipg_button::ButtonStyleStd;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};
use crate::app::Message;
use crate::py_api::helpers::{get_len, get_padding};
use super::callbacks::{invoke_callback, invoke_callback_with_args};

use crate::ipg_widgets::ipg_color_picker::color_picker::{Position, Tooltip};

use iced::widget::{Button, button, center, container, text};
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

        let submit_row = submit_row().into();

        let r_sld_row = 
            rgba_slider("r", self.r_value, RGBA::R, ColPikMessage::RSldOnChange).into();
        let g_sld_row = 
            rgba_slider("g", self.g_value, RGBA::G, ColPikMessage::GSldOnChange).into();
        let b_sld_row = 
            rgba_slider("b", self.b_value, RGBA::B, ColPikMessage::BSldOnChange).into();
        let a_sld_row = 
            rgba_slider("a", self.a_value, RGBA::A, ColPikMessage::ASldOnChange).into();

        let rgba_col = 
            column(vec![r_sld_row, g_sld_row, b_sld_row, a_sld_row])
            .spacing(5.0)
            .into();

        let grad_cont: Element<Message> = Canvas::new(HsvSquare {
                hue: self.hue_value,
                r: self.r_value,
                g: self.g_value,
                b: self.b_value,
            })
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(100.0))
            .into();
        
        // Top row with gradient window and rgba sliders
        let grad_rgba_row = 
            row(vec![grad_cont, rgba_col])
            .width(Length::Fill)
            .spacing(5.0)
            .into();

        // contains a col of hue slider and radios
        // in a row with the selcted color container
        let hue_row = 
            hue_slider_row(
                self.hue_value, 
                self.color_format_selected,
                self.current_color()
            ).into();

        let col: Element<Message> =
            column(vec![grad_rgba_row, hue_row, submit_row])
            .spacing(10.0)
            .into();

        let content = 
            container(col)
                .width(370.0)
                .height(190.0)
                .padding(5.0);

        let cp = 
            ColorPicker::new(
                button("Color Picker").on_press(ColPikMessage::Noop),
                content,
                self.current_color(),
                Position::Top,
            )
            .opened(self.opened)
            .on_open(ColPikMessage::SetOpened)
            .gap(10)
            .style(container::rounded_box);

        
        Some(center(cp).into())

        // Some(color_picker.map(move |message| Message::ColorPicker(self.id, message)))


    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum ColPikMessage {
    Noop,
    Submit,
    Cancel,
    CopyClipBoard,
    SetOpened(bool),
    RSldOnChange(u8),
    GSldOnChange(u8),
    BSldOnChange(u8),
    ASldOnChange(u8),
    OnHueChange(u8),
    RgbaOnInput(RGBA, String),
    CanvasColorPicked(u8, u8, u8),
    ColorFormatSelected(ColorOutFormat)
}

pub fn color_picker_callback(id: usize, message: ColPikMessage) {
        match message {
            ColPikMessage::Submit => {
                self.opened = false;
                let val = 
                    vec![self.r_value, self.g_value, self.b_value, self.a_value];
                println!("Submitted {:?}", val)
            }
            ColPikMessage::Cancel => {
                self.opened = false;
                println!("Canceled")
            },
            ColPikMessage::CopyClipBoard => {
                let text = selected_color_format_to_text(
                    self.color_format_selected,
                    self.current_color(),
                );
                return iced::clipboard::write(text).discard();
            },
            ColPikMessage::SetOpened(opened) => {
                self.opened = opened;
            },
            ColPikMessage::RSldOnChange(value) => {
                self.r_value = value;
                self.hue_value = rgb_to_hue(self.r_value, self.g_value, self.b_value);
            },
            ColPikMessage::GSldOnChange(value) => {
                self.g_value = value;
                self.hue_value = rgb_to_hue(self.r_value, self.g_value, self.b_value);
            },
            ColPikMessage::BSldOnChange(value) => {
                self.b_value = value;
                self.hue_value = rgb_to_hue(self.r_value, self.g_value, self.b_value);
            },
            ColPikMessage::ASldOnChange(value) => {
                self.a_value = value;
            },
            ColPikMessage::OnHueChange(value) => {
                self.hue_value = value;
                let rgb = hue_to_rgb(value);
                self.r_value = (rgb.r * 255.0).round() as u8;
                self.g_value = (rgb.g * 255.0).round() as u8;
                self.b_value = (rgb.b * 255.0).round() as u8;
            },
            ColPikMessage::Noop => (),
            ColPikMessage::CanvasColorPicked(r, g, b) => {
                self.r_value = r;
                self.g_value = g;
                self.b_value = b;
                self.hue_value = rgb_to_hue(r, g, b);
            },
            ColPikMessage::RgbaOnInput(rgba, input) => {
                if let Ok(v) = input.parse::<u8>() {
                    match rgba {
                        RGBA::R => { self.r_value = v; self.hue_value = rgb_to_hue(self.r_value, self.g_value, self.b_value); }
                        RGBA::G => { self.g_value = v; self.hue_value = rgb_to_hue(self.r_value, self.g_value, self.b_value); }
                        RGBA::B => { self.b_value = v; self.hue_value = rgb_to_hue(self.r_value, self.g_value, self.b_value); }
                        RGBA::A => { self.a_value = v; }
                        RGBA::H => {
                            self.hue_value = v;
                            let rgb = hue_to_rgb(v);
                            self.r_value = (rgb.r * 255.0).round() as u8;
                            self.g_value = (rgb.g * 255.0).round() as u8;
                            self.b_value = (rgb.b * 255.0).round() as u8;
                        }
                    }
                }
            },
            ColPikMessage::ColorFormatSelected(format) => {
                self.color_format_selected = Some(format);
            }
        }

    }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColorOutFormat {
    Float,
    Hex,
    Integer,
    Percent,
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
