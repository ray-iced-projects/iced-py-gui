//! Button widget definition
use crate::access_callbacks;
use crate::access_user_data1;
use crate::app::Message;
use crate::graphics::bootstrap_arrow::IpgArrow;
use crate::state::IpgWidgets;
use crate::widgets::enums::{IpgHorizontalAlignment, 
    IpgVerticalAlignment};
use crate::py_api::helpers::get_padding;
use crate::widgets::styling::{apply_border_overrides, apply_shadow_overrides};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_halign, set_height, set_height_fill, set_iced_color_from_rgba, set_opt_bool, set_opt_f32, set_opt_iced_color, set_opt_string, set_opt_usize, set_opt_vec_f32, set_valign, set_width, set_width_fill
};

use iced::widget::{button, text, Button};
use iced::{Color, Element, Length, Theme};

use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct IpgButton {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub label: Option<String>,
    pub width: Length,
    pub height: Length,
    pub padding: Option<Vec<f32>>,
    pub text_align_x: Option<IpgHorizontalAlignment>,
    pub text_align_y: Option<IpgVerticalAlignment>,
    pub text_size: Option<f32>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgButtonStyleStandard>,
    pub style_arrow: Option<IpgArrow>,
}

#[derive(Debug, Clone)]
pub enum BtnMessage {
    OnPress,
}

pub fn construct_button<'a>(
    ipg_btn: &'a IpgButton, 
    style_widget: Option<&IpgWidgets>,
    ) -> Option<Element<'a, Message>> {
    
    if !ipg_btn.show {
        return None;
    }

    let style_opt = style_widget.and_then(IpgWidgets::as_button_style).cloned();

    let txt = 
        if let Some(sa) = ipg_btn.style_arrow.clone() {
            let arrow = IpgArrow::to_string(&sa);
            text(arrow).font(iced::Font::with_name("bootstrap-icons"))
        } else {
            let label = if let Some(lb) = &ipg_btn.label {
                lb.clone()
            } else {
                String::new()
            };
            text(label.clone())
        };

    let txt = 
        if let Some(align) = &ipg_btn.text_align_x {
            txt.align_x(align.to_iced())
        } else {txt};

    let txt = 
        if let Some(align) = &ipg_btn.text_align_y {
            txt.align_y(align.to_iced())
        } else {txt};
    
    let txt = 
        if let Some(size) = ipg_btn.text_size {
            txt.size(size)
        } else {txt};
    
    let btn=
        Button::new(txt)
            .padding(get_padding(&ipg_btn.padding))
            .on_press(BtnMessage::OnPress)
            .width(ipg_btn.width)
            .height(ipg_btn.height)
            .style(move |theme: &Theme, status| {
                get_styling(theme, status, &style_opt, &ipg_btn.style_standard)
        });

    let btn: Element<'_, BtnMessage> = 
        if let Some(cp) = ipg_btn.clip {
            btn.clip(cp).into()
        } else { btn.into() };

    Some(btn.map(move |message| Message::Button(ipg_btn.id, message)))

}



pub fn button_callback(id: usize, message: BtnMessage) {
    match message {
        BtnMessage::OnPress => {
            process_callback(id, "on_press".to_string());
        }
    }
}

fn process_callback(id: usize, event_name: String) {
    
    let app_cbs = access_callbacks();
    
    // Retrieve the callback
    let callback = match app_cbs.get(id, &event_name) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };
    
    drop(app_cbs);
    
    // Check user data 1
    let user_data_1_lock = access_user_data1();
    let user_data_1_opt = user_data_1_lock.get(id).map(|ud| Python::attach(|py| ud.clone_ref(py)));
    drop(user_data_1_lock);
    
    // Call the callback
    Python::attach(|py| {
        let result = if let Some(user_data) = user_data_1_opt {
            callback.call1(py, (id, user_data))
        } else {
            callback.call1(py, (id,))
        };
        
        if let Err(err) = result {
            panic!("Button callback error: {err}");
        }
        return
    });

    // Check user data 2
    let user_data_2_lock = access_user_data1();
    let user_data_2_opt = user_data_2_lock.get(id).map(|ud| Python::attach(|py| ud.clone_ref(py)));
    drop(user_data_2_lock);

    // Call the callback
    Python::attach(|py| {
        let result = if let Some(user_data) = user_data_2_opt {
            callback.call1(py, (id, user_data))
        } else {
            callback.call1(py, (id,))
        };
        
        if let Err(err) = result {
            panic!("Button callback error: {err}");
        }
    });

}

#[derive(Debug, Clone, Default)]
pub struct IpgButtonStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    pub background_color_disabled: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<Color>,
    pub shadow_offset_x: Option<f32>,
    pub shadow_offset_y: Option<f32>,
    pub shadow_blur_radius: Option<f32>,
    pub text_color: Option<Color>,
}

impl IpgButtonStyle {
    /// Apply user-defined style overrides to an existing iced button::Style
    pub fn apply_to(&self, style: &mut button::Style, status: button::Status) {
        
        if let Some(color) = self.background_color {
            if status == button::Status::Active || status == button::Status::Pressed {
                style.background = Some(color.into());
            }
        }

        if let Some(color) = self.background_color_hovered {
            if status == button::Status::Hovered {
                style.background = Some(color.into());
            }
        }

        if let Some(color) = self.background_color_disabled {
            if status == button::Status::Disabled {
                style.background = Some(color.into());
            }
        }

        apply_border_overrides(
            &mut style.border, self.border_color,
            &self.border_radius, self.border_width, "Button",
        );

        apply_shadow_overrides(
            &mut style.shadow, self.shadow_color,
            self.shadow_offset_x, self.shadow_offset_y,
            self.shadow_blur_radius,
        );

        if let Some(color) = self.text_color {
            style.text_color = color;
        }
    }
}

pub fn get_styling(theme: &Theme, status: button::Status,
                    style_opt: &Option<IpgButtonStyle>,
                    style_standard: &Option<IpgButtonStyleStandard>,
                    ) -> button::Style 
{
    let mut style = match style_standard {
        Some(_) => get_button_standard_style(theme, status, style_standard),
        None => button::primary(theme, status),
    };

    if let Some(user_style) = style_opt {
        user_style.apply_to(&mut style, status);
    }

    style
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonStyleStandard {
    Background,
    Danger,
    Primary,
    Secondary,
    Subtle,
    Success,
    Warning,
    Text,
}

pub fn get_button_standard_style(
    theme: &Theme, 
    status: button::Status, 
    style_standard: &Option<IpgButtonStyleStandard>
    ) -> button::Style {
    
    match style_standard {
        Some(IpgButtonStyleStandard::Background) => {
            button::background(theme, status)
        },
        Some(IpgButtonStyleStandard::Danger) => {
            button::danger(theme, status)
        },
        Some(IpgButtonStyleStandard::Primary) => {
            button::primary(theme, status)
        },
        Some(IpgButtonStyleStandard::Secondary) => {
            button::secondary(theme, status)
        },
        Some(IpgButtonStyleStandard::Subtle) => {
            button::subtle(theme, status)
        },
        Some(IpgButtonStyleStandard::Success) => {
            button::success(theme, status)
        },
        Some(IpgButtonStyleStandard::Warning) => {
            button::warning(theme, status)
        },
        Some(IpgButtonStyleStandard::Text) => {
            button::text(theme, status)
        },
        None => button::primary(theme, status),
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonParam {
    ArrowStyle,
    Height,
    HeightFill,
    Label,
    Padding,
    Clip,
    Show,
    StyleId,
    StyleStandard,
    TextAlignX,
    TextAlignY,
    TextSize,
    Width,
    WidthFill,
}

pub fn extract_button_style_standard(
    value: &PyObject, 
    name: String,
    ) -> IpgButtonStyleStandard {
    
    Python::attach(|py| {

        let res = 
            value.extract::<IpgButtonStyleStandard>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for ButtonStyleStandard", name),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgButtonStyleParam {
    BackgroundIpgColor,
    BackgroundRbgaColor,
    BackgroundIpgColorHovered,
    BackgroundIpgRgbaHovered,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlurRadius,
    TextIpgColor,
    TextRgbaColor,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgButton {
    type Param = IpgButtonParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgButtonParam::ArrowStyle => {
                self.style_arrow = IpgArrow::extract(value);
            }
            IpgButtonParam::Clip => set_opt_bool(&mut self.clip, value, name),
            IpgButtonParam::Height => set_height(&mut self.height, value, name),
            IpgButtonParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgButtonParam::Label => set_opt_string(&mut self.label, value, name),
            IpgButtonParam::Padding => set_opt_vec_f32(&mut self.padding, value, name),
            IpgButtonParam::Show => set_bool(&mut self.show, value, name),
            IpgButtonParam::StyleId => set_opt_usize(&mut self.style_id, value, name),
            IpgButtonParam::StyleStandard => {
                self.style_standard = Some(extract_button_style_standard(value, name));
            },
            IpgButtonParam::TextAlignX => set_halign(&mut self.text_align_x, value, name),
            IpgButtonParam::TextAlignY => set_valign(&mut self.text_align_y, value, name),
            IpgButtonParam::TextSize => set_opt_f32(&mut self.text_size, value, name),
            IpgButtonParam::Width => set_width(&mut self.width, value, name),
            IpgButtonParam::WidthFill => set_width_fill(&mut self.width, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgButtonStyle {
    type Param = IpgButtonStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgButtonStyleParam::BackgroundIpgColor => 
                set_opt_iced_color(&mut self.background_color, value, name),
            IpgButtonStyleParam::BackgroundRbgaColor => 
                set_iced_color_from_rgba(&mut self.background_color, value, name),
            IpgButtonStyleParam::BackgroundIpgColorHovered => 
                set_opt_iced_color(&mut self.background_color_hovered, value, name),
            IpgButtonStyleParam::BackgroundIpgRgbaHovered => 
                set_iced_color_from_rgba(&mut self.background_color_hovered, value, name),
            IpgButtonStyleParam::BorderIpgColor => 
                set_opt_iced_color(&mut self.border_color, value, name),
            IpgButtonStyleParam::BorderRgbaColor => 
                set_iced_color_from_rgba(&mut self.border_color, value, name),
            IpgButtonStyleParam::BorderRadius => 
                set_opt_vec_f32(&mut self.border_radius, value, name),
            IpgButtonStyleParam::BorderWidth => 
                set_opt_f32(&mut self.border_width, value, name),
            IpgButtonStyleParam::ShadowIpgColor => 
                set_opt_iced_color(&mut self.shadow_color, value, name),
            IpgButtonStyleParam::ShadowRgbaColor => 
                set_iced_color_from_rgba(&mut self.shadow_color, value, name),
            IpgButtonStyleParam::ShadowOffsetX => 
                set_opt_f32(&mut self.shadow_offset_x, value, name),
            IpgButtonStyleParam::ShadowOffsetY => 
                set_opt_f32(&mut self.shadow_offset_y, value, name),
            IpgButtonStyleParam::ShadowBlurRadius => 
                set_opt_f32(&mut self.shadow_blur_radius, value, name),
            IpgButtonStyleParam::TextIpgColor => 
                set_opt_iced_color(&mut self.text_color, value, name),
            IpgButtonStyleParam::TextRgbaColor => 
                set_iced_color_from_rgba(&mut self.text_color, value, name),
        }
    }
}
