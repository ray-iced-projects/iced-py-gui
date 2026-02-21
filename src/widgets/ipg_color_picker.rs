//! ipg_color_picker
use crate::graphics::bootstrap_arrow::IpgArrow;
use crate::state::IpgWidgets;
use crate::widgets::ipg_button::{IpgButtonStyleStandard, extract_btn_style, get_styling};
use crate::{access_callbacks, access_user_data1, IpgState};
use crate::app::Message;
use crate::py_api::helpers::get_padding;
use super::callbacks::{set_or_get_widget_callback_data, WidgetCallbackIn};

use iced::widget::{text, Button};
use iced::{Color, Element, Length, Theme};
use iced_aw::ColorPicker;

use pyo3::Python;


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
    pub style_standard: Option<IpgButtonStyleStandard>,
    pub style_arrow: Option<IpgArrow>,
}

// #[derive(Debug, Clone, Default)]
// pub struct IpgColorPickerStyle {
//     pub id: usize,
//     pub background_color: Option<Color>,
//     pub background_color_hovered: Option<Color>,
//     pub border_color: Option<Color>,
//     pub border_radius: Option<Vec<f32>>,
//     pub border_width: Option<f32>,
//     pub shadow_color: Option<Color>,
//     pub shadow_offset_x: Option<f32>,
//     pub shadow_offset_y: Option<f32>,
//     pub shadow_blur_radius: Option<f32>,
//     pub text_color: Option<Color>,
// }

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum ColPikMessage {
    OnPress,
    OnCancel,
    OnSubmit(Color),
}


pub fn construct_color_picker<'a>(cp: &'a IpgColorPicker,
                                style_opt: Option<&IpgWidgets>,
                                ) -> Option<Element<'a, Message>> {
    
    let label = 
        if let Some(lbl) = cp.label.clone() {
            text(lbl)
        } else {
            text("Select Color".to_string())
        };
    

    let style = extract_btn_style(style_opt);

    let btn: Element<ColPikMessage> = Button::new(label)
                                    .height(cp.height)
                                    .padding(get_padding(&cp.padding))
                                    .width(cp.width)
                                    .on_press(ColPikMessage::OnPress)
                                    .style(move|theme: &Theme, status| {   
                                        get_styling(theme, status,
                                            &style,
                                            &cp.style_standard)
                                        })
                                    .into();

    if !cp.show {
        return Some(btn.map(move |message| Message::ColorPicker(cp.id, message)));
    }

    let color_picker: Element<ColPikMessage> = ColorPicker::new(
                                    cp.show,
                                    cp.color,
                                    btn,
                                    ColPikMessage::OnCancel,
                                    ColPikMessage::OnSubmit,
                                ).into();

    Some(color_picker.map(move |message| Message::ColorPicker(cp.id, message)))

}

pub fn color_picker_callback(state: &mut IpgState, id: usize, message: ColPikMessage) {
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    match message {
        ColPikMessage::OnCancel => {
            wci.id = id;
            wci.value_bool = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(id, "on_cancel".to_string(), None);
        },
        ColPikMessage::OnSubmit(color) => {
            wci.id = id;
            wci.value_bool = Some(false);
            wci.color = Some(convert_color_to_list(color));
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(id, "on_select".to_string(), Some(convert_color_to_list(color)));
        },
        ColPikMessage::OnPress => {
            wci.id = id;
            wci.value_bool = Some(true);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(id, "on_press".to_string(), None);
        },
    }
}


pub fn process_callback(id: usize, event_name: String, color: Option<Vec<f64>>) 
{
    let ud1 = access_user_data1();
    let app_cbs = access_callbacks();

    // Retrieve the callback
    let callback = match app_cbs.callbacks.get(&(id, event_name.clone())) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_cbs);

    // Check user data from ud1
    if let Some(user_data) = ud1.user_data.get(&id) {
        Python::attach(|py| {
            if event_name == "on_select".to_string() {
                if let Err(err) = callback.call1(py, (id, color, user_data)) {
                    panic!("ColorPicker callback error: {err}");
                }
            } else {
                if let Err(err) = callback.call1(py, (id, user_data)) {
                    panic!("ColorPicker callback error: {err}");
                }
            }
            
        });
        drop(ud1); // Drop ud1 before processing ud2
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Check user data from ud2
    // let ud2 = access_user_data2();
    // if let Some(user_data) = ud2.user_data.get(&id) {
    //     Python::attach(|py| {
    //         if event_name == "on_submit".to_string() {
    //             if let Err(err) = callback.call1(py, (id, color, user_data)) {
    //                 panic!("ColorPicker callback error: {err}");
    //             }
    //         } else {
    //             if let Err(err) = callback.call1(py, (id, user_data)) {
    //                 panic!("ColorPicker callback error: {err}");
    //             }
    //         }
    //     });
    //     drop(ud2); // Drop ud2 after processing
    //     return;
    // }
    // drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the 
    // callback with only the id and color except for on_pressed
    // which has only an id.
    Python::attach(|py| {
        if event_name == "on_submit".to_string() {
            if let Err(err) = callback.call1(py, (id, color)) {
                panic!("ColorPicker callback error: {err}");
            }
        } else {
            if let Err(err) = callback.call1(py, (id,)) {
                panic!("ColorPicker callback error: {err}");
            }
        }
    });
}


// #[derive(Debug, Clone, PartialEq)]
// #[pyclass(eq, eq_int)]
// pub enum IpgColorPickerParam {
//     ArrowStyle,
//     Clip,
//     Color,
//     Height,
//     HeightFill,
//     Label,
//     Padding,
//     Show,
//     StyleId,
//     StyleStandard,
//     Width,
//     WidthFill,
// }

// pub fn color_picker_update(cp: &mut IpgColorPicker,
//                             item: &PyObject,
//                             value: &PyObject,
//                             ) 
// {
//     let update = try_extract_cp_update(item);
//     let name = "ColorPicker".to_string();
//     match update {
//        IpgColorPickerParam::ArrowStyle => {
//             cp.style_arrow = Some(try_extract_button_arrow(value));
//         },
//         IpgColorPickerParam::Color => {
//             let rgba = IpgColor::extract_rgba(value, name);
//             cp.color = Color::from(rgba);
//         },
//         IpgColorPickerParam::Label => {
//             cp.label = Some(try_extract_string(value, name));
//         },
//         IpgColorPickerParam::Height => {
//             let val = try_extract_f32(value, name);
//             cp.height = get_height(Some(val), false);
//         },
//         IpgColorPickerParam::HeightFill => {
//             let val = try_extract_boolean(value, name);
//             cp.height = get_height(None, val);
//         },
//         IpgColorPickerParam::Padding => {
//             cp.padding =  Some(try_extract_vec_f32(value, name));
//         },
//         IpgColorPickerParam::Clip => {
//             cp.clip = Some(try_extract_boolean(value, name));
//         }
//         IpgColorPickerParam::Show => {
//             cp.show = try_extract_boolean(value, name);
//         },
//         IpgColorPickerParam::StyleId => {
//             let val = try_extract_f32(value, name) as usize;
//             cp.style_id = Some(val);
//         },
//         IpgColorPickerParam::StyleStandard => {
//             let val = try_extract_button_style_standard(value, name);
//             cp.style_standard = Some(val);
//         },
//         IpgColorPickerParam::Width => {
//             let val = try_extract_f32(value, name);
//             cp.width = get_width(Some(val as f32), false);
//         },
//         IpgColorPickerParam::WidthFill => {
//             let val = try_extract_boolean(value, name);
//             cp.width = get_width(None, val);
//         },
//     }
// }

// pub fn try_extract_cp_update(update_obj: &PyObject) -> IpgColorPickerParam {

//     Python::attach(|py| {
//         let res = update_obj.extract::<IpgColorPickerParam>(py);
//         match res {
//             Ok(update) => update,
//             Err(_) => panic!("Color Picker update extraction failed"),
//         }
//     })
// }

// #[derive(Debug, Clone, PartialEq)]
// #[pyclass(eq, eq_int)]
// pub enum IpgColorPickerStyleParam {
//     BackgroundIpgColor,
//     BackgroundRbga,
//     BackgroundIpgColorHovered,
//     BackgroundIpgRgbaHovered,
//     BorderIpgColor,
//     BorderRgba,
//     BorderRadius,
//     BorderWidth,
//     ShadowIpgColor,
//     ShadowRgba,
//     ShadowOffsetX,
//     ShadowOffsetY,
//     ShadowBlurRadius,
//     TextIpgColor,
//     TextRgbaColor
// }

// pub fn color_picker_style_update_item(style: &mut IpgColorPickerStyle,
//                                         item: &PyObject,
//                                         value: &PyObject,) 
// {

//     let update = try_extract_color_picker_style_update(item);
//     let name = "ColorPickerStyle".to_string();
//     match update {
//         IpgColorPickerStyleParam::BackgroundIpgColor => {
//             let color = IpgColor::extract(value, name);
//             style.background_color = 
//             IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
//         },
//         IpgColorPickerStyleParam::BackgroundRbga => {
//             let rgba = IpgColor::extract_rgba(value, name);
//             style.background_color = 
//             IpgColor::rgba_ipg_color_to_iced(Some(rgba), None, 1.0, false);
//         },
//         IpgColorPickerStyleParam::BackgroundIpgColorHovered => {
//             let color = IpgColor::extract(value, name);
//             style.background_color_hovered = 
//             IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
//         },
//         IpgColorPickerStyleParam::BackgroundIpgRgbaHovered => {
//             let rgba = IpgColor::extract_rgba(value, name);
//             style.background_color_hovered = 
//             IpgColor::rgba_ipg_color_to_iced(Some(rgba), None, 1.0, false);
//         },
//         IpgColorPickerStyleParam::BorderIpgColor => {
//             let color = IpgColor::extract(value, name);
//             style.border_color = 
//             IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
//         },
//         IpgColorPickerStyleParam::BorderRgba => {
//             let rgba = IpgColor::extract_rgba(value, name);
//             style.border_color = 
//             IpgColor::rgba_ipg_color_to_iced(Some(rgba), None, 1.0, false);
//         },
//         IpgColorPickerStyleParam::BorderRadius => {
//             style.border_radius = Some(try_extract_vec_f32(value, name));
//         },
//         IpgColorPickerStyleParam::BorderWidth => {
//             style.border_width = Some(try_extract_f32(value, name));
//         },
//         IpgColorPickerStyleParam::ShadowIpgColor => {
//             let color = IpgColor::extract(value, name);
//             style.shadow_color = 
//             IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
//         },
//         IpgColorPickerStyleParam::ShadowRgba => {
//             let rgba = IpgColor::extract_rgba(value, name);
//             style.shadow_color = 
//             IpgColor::rgba_ipg_color_to_iced(Some(rgba), None, 1.0, false);
//         },
//         IpgColorPickerStyleParam::ShadowOffsetX => {
//             style.shadow_offset_x = Some(try_extract_f32(value, name));
//         },
//         IpgColorPickerStyleParam::ShadowOffsetY => {
//             style.shadow_offset_y = Some(try_extract_f32(value, name));
//         },
//         IpgColorPickerStyleParam::ShadowBlurRadius => {
//             style.shadow_blur_radius = Some(try_extract_f32(value, name));
//         },
//         IpgColorPickerStyleParam::TextIpgColor => {
//             let color = IpgColor::extract(value, name);
//             style.text_color = 
//             IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
//         },
//         IpgColorPickerStyleParam::TextRgbaColor => {
//             let rgba = IpgColor::extract_rgba(value, name);
//             style.text_color = 
//             IpgColor::rgba_ipg_color_to_iced(Some(rgba), None, 1.0, false);
//         },
//     }
// }

// pub fn try_extract_color_picker_style_update(update_obj: &PyObject) -> IpgColorPickerStyleParam {

//     Python::attach(|py| {
//         let res = update_obj.extract::<IpgColorPickerStyleParam>(py);
//         match res {
//             Ok(update) => update,
//             Err(_) => panic!("Color Picker style update extraction failed"),
//         }
//     })
// }

// fn get_cp_style(style: Option<&IpgWidgets>) -> Option<IpgColorPickerStyle>{
//     match style {
//         Some(IpgWidgets::IpgColorPickerStyle(style)) => {
//             Some(style.clone())
//         }
//         _ => None,
//     }
// }

// pub fn get_styling(theme: &Theme, status: button::Status,
//                     style_opt: &Option<IpgColorPickerStyle>,
//                     style_standard: &Option<IpgButtonStyleStandard>,
//                     ) -> button::Style 
// {
//     if style_standard.is_none() && style_opt.is_none() {
//         return button::primary(theme, status)
//     }

//     if style_opt.is_none() && style_standard.is_some() {
//             return get_standard_style(theme, status, &style_standard)
//     }

//     let mut border = Border::default();
//     let mut shadow = Shadow::default();

//     let mut base_style = button::primary(theme, status);
//     let mut hover_style = button::primary(theme, status);

//     let style = style_opt.clone().unwrap_or_default();

//     if style.border_color.is_some() {
//         border.color = style.border_color.unwrap();
//     }

//     if let Some(br) = style.border_radius {
//         border.radius = get_radius(br, "ColorPicker".to_string());
//     }

//     if let Some(bw) = style.border_width {
//         border.width = bw;
//     }

//     if let Some(sc) = style.shadow_color {
//         shadow.color = sc;
//     }

//     let offset_x = if let Some(x) = style.shadow_offset_x {
//         x
//     } else { 0.0 };

//     let offset_y = if let Some(y) = style.shadow_offset_x {
//         y
//     } else { 0.0 };

//     shadow.offset = Vector{ x: offset_x , y: offset_y };
    
//     if let Some(br) = style.shadow_blur_radius {
//         shadow.blur_radius = br;
//     }

//     if let Some(bc) = style.background_color {
//         base_style.background = Some(bc.into())
//     };

//     if let Some(bch) = style.background_color_hovered {
//         hover_style.background = Some(bch.into())
//     };

//     if let Some(tc) = style.text_color {
//         base_style.text_color = tc;
//         hover_style.text_color = tc;
//     }

//     base_style.border = border;
//     hover_style.border = border;

//     base_style.shadow = shadow;
//     hover_style.shadow = shadow;

//     match status {
//         button::Status::Active | button::Status::Pressed => base_style,
//         button::Status::Hovered => hover_style,
//         button::Status::Disabled => disabled(base_style),
//     }
    
// }

// fn disabled(style: button::Style) -> button::Style {
//     button::Style {
//         background: style
//             .background
//             .map(|background| background.scale_alpha(0.5)),
//         text_color: style.text_color.scale_alpha(0.5),
//         ..style
//     }
// }

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
