//! ipg_container
#![allow(unused)]

use iced::{Border, Color, Element, Length, Padding, Shadow, Theme, Vector};
use iced::widget::{container, Space, Container};
use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use crate::app::Message;
use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::{get_height, get_padding, get_radius, get_width, try_extract_array_2, try_extract_boolean, try_extract_f32, try_extract_vec_f32};
use crate::state::IpgWidgets;
use crate::widgets::enums::{IpgHorizontalAlignment, IpgVerticalAlignment};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,
    set_bool, set_opt_bool, set_opt_f32, set_opt_vec_f32, set_opt_array_2,
    set_width, set_width_fill, set_height, set_height_fill,
    set_halign, set_valign,
    set_opt_iced_color, set_iced_color_from_rgba,
};


#[derive(Debug, Clone)]
pub struct IpgContainer {
    pub id: usize,
    pub show: bool,

    pub padding: Option<Vec<f32>>,
    pub width: Length,
    pub height: Length,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub align_x: Option<IpgHorizontalAlignment>,
    pub align_y: Option<IpgVerticalAlignment>,
    pub center_x: Option<bool>,
    pub center_y: Option<bool>,
    pub center: Option<bool>,
    pub align_left: Option<bool>,
    pub align_right: Option<bool>,
    pub align_top: Option<bool>,
    pub align_botton: Option<bool>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>, 
}
#[derive(Debug, Clone)]
pub struct IpgContainerStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<Color>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,
    pub text_color: Option<Color>,
}

pub fn construct_container<'a>(
    ipg_cont: &'a IpgContainer, 
    mut content: Vec<Element<'a, Message>>,
    style_opt: Option<&'a IpgWidgets>,
    ) -> Element<'a, Message> {
    
    if !ipg_cont.show {return Space::new().into()}

    let style = get_cont_style(style_opt);

    // Since a container can have only one element and the 
    // the process sends a vec then if empty container, put in a
    // space or remove the element in the vec.
    let new_content: Element<Message> = if content.is_empty() {
        Space::new().into()
    } else {
        content.remove(0)
    };

    let cont = 
        Container::new(new_content)
            .width(ipg_cont.width)
            .height(ipg_cont.height)
            .style(move|theme|
                get_styling(theme, 
                    style.clone(),
                )
            );

    let cont = 
        if let Some(mw) = ipg_cont.max_width {
            cont.max_width(mw)
        } else { cont };

    let cont = 
        if let Some(mh) = ipg_cont.max_height {
            cont.max_width(mh)
        } else { cont };

    let cont = 
        if let Some(align) = &ipg_cont.align_x {
            cont.align_x(align.to_iced())
        } else { cont };

    let cont = 
        if let Some(align) = &ipg_cont.align_y {
            cont.align_y(align.to_iced())
        } else { cont };

    let cont = 
        if ipg_cont.center == Some(true) {
            cont.center(ipg_cont.width)
        } else { cont };

    let cont = 
        if ipg_cont.center_x == Some(true) {
            cont.center_x(ipg_cont.width)
        } else { cont };

    let cont = 
        if ipg_cont.center_y == Some(true) {
            cont.center_y(ipg_cont.height)
        } else { cont };

    let cont = 
        if ipg_cont.align_left == Some(true) {
            cont.align_left(ipg_cont.width)
        } else { cont };

    let cont = 
        if ipg_cont.align_right == Some(true) {
            cont.align_right(ipg_cont.width)
        } else { cont };

    let cont = 
        if ipg_cont.align_top == Some(true) {
            cont.align_top(ipg_cont.height)
        } else { cont };

    let cont = 
        if ipg_cont.align_botton == Some(true) {
            cont.align_bottom(ipg_cont.height)
        } else { cont };
    
    let cont = 
        if ipg_cont.clip == Some(true) {
            cont.clip(true)
        } else { cont };

    cont.into()            
    
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgContainerParam {
    AlignBotton,
    AlignLeft,
    AlignRight,
    AlignTop,
    AlignX,
    AlignY,
    Center,
    CenterX,
    CenterY,
    Clip,
    Height,
    HeightFill,
    MaxHeight,
    MaxWidth,
    Padding,
    Width,
    WidthFill,
    Show,
}

pub fn container_item_update(cont: &mut IpgContainer,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_container_update(item);
    let name = "Container".to_string();
    match update {
        IpgContainerParam::AlignBotton => {
            cont.align_botton = Some(try_extract_boolean(value, name));
        },
        IpgContainerParam::AlignLeft => {
            cont.align_left = Some(try_extract_boolean(value, name));
        },
        IpgContainerParam::AlignRight => {
            cont.align_right = Some(try_extract_boolean(value, name));
        },
        IpgContainerParam::AlignTop => {
            cont.align_top = Some(try_extract_boolean(value, name));
        },
        IpgContainerParam::AlignX => {
            cont.align_x = IpgHorizontalAlignment::extract(value);
        },
        IpgContainerParam::AlignY => {
            cont.align_y = IpgVerticalAlignment::extract(value);
        },
        IpgContainerParam::Center => {
            cont.center = Some(try_extract_boolean(value, name));
        },
        IpgContainerParam::CenterX => {
            cont.center_x = Some(try_extract_boolean(value, name));
        },
        IpgContainerParam::CenterY => {
            cont.center_y = Some(try_extract_boolean(value, name));
        },
        IpgContainerParam::Clip => {
            cont.clip = Some(try_extract_boolean(value, name));
        },
        IpgContainerParam::Height => {
            let h = Some(try_extract_f32(value, name));
            cont.height = get_height(h, false)
        },
        IpgContainerParam::HeightFill => {
            cont.height = get_height(None, try_extract_boolean(value, name));
        },
        IpgContainerParam::MaxHeight => {
            cont.max_height = Some(try_extract_f32(value, name));
        },
        IpgContainerParam::MaxWidth => {
            cont.max_width = Some(try_extract_f32(value, name));
        },
        IpgContainerParam::Padding => {
            cont.padding = Some(try_extract_vec_f32(value, name));
        },
        IpgContainerParam::Width => {
            let w = Some(try_extract_f32(value, name));
            cont.width = get_width(w, false)
        },
        IpgContainerParam::WidthFill => {
            cont.width = get_width(None, try_extract_boolean(value, name));
        },
        IpgContainerParam::Show => {
            cont.show = try_extract_boolean(value, name);
        }
    }
}

pub fn try_extract_container_update(update_obj: &PyObject) -> IpgContainerParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgContainerParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Container update extraction failed"),
        }
    })
}

pub fn get_cont_style(style: Option<&IpgWidgets>) -> Option<IpgContainerStyle>{
    match style {
        Some(IpgWidgets::IpgContainerStyle(style)) => {
            Some(style.clone())
        }
        _ => None,
    }
}

pub fn get_styling(theme: &Theme,
                style_opt: Option<IpgContainerStyle>,  
                ) -> container::Style {
    
    if style_opt.is_none() {
        return container::transparent(theme);
    }

    let style = style_opt.unwrap();

    let background_color = if style.background_color.is_some() {
        style.background_color.unwrap()
    } else {
        Color::TRANSPARENT
    };

    let mut border = Border::default();
    let mut shadow = Shadow::default();

    if style.border_color.is_some() {
        border.color = style.border_color.unwrap();
    }

    if let Some(radius) = style.border_radius {
        border.radius = get_radius(radius, "Container".to_string());
    }

    if let Some(width) = style.border_width {
        border.width = width;
    }

    if style.shadow_color.is_some() {
        shadow.color = style.shadow_color.unwrap();

        if let Some(blur) = style.shadow_blur_radius {
            shadow.blur_radius = blur;
        }

        if let Some(offset) = style.shadow_offset_xy {
            shadow.offset = Vector{ x: offset[0], y: offset[1] }
        }
    }

    container::Style {
        background: Some(background_color.into()),
        border,
        shadow,
        text_color: style.text_color,
        snap: false,
    }
    
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgContainerStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowIpgColor,
    ShadowRgbaColor,
    ShadowOffsetXY,
    ShadowBlurRadius,
    TextIpgColor,
    TextRgbaColor,
}

pub fn container_style_update_item(style: &mut IpgContainerStyle,
                            item: &PyObject,
                            value: &PyObject,) 
{
    let update = try_extract_container_style_update(item);
    let name = "ContainerStyle".to_string();
    match update {
        IpgContainerStyleParam::BackgroundIpgColor => {
            let color = IpgColor::extract(value, name);
            style.background_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgContainerStyleParam::BackgroundRgbaColor => {
            style.background_color = Some(Color::from(IpgColor::extract_rgba(value, name)));
        },
        IpgContainerStyleParam::BorderIpgColor => {
            let color = IpgColor::extract(value, name);
            style.border_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgContainerStyleParam::BorderRgbaColor => {
            style.border_color = Some(Color::from(IpgColor::extract_rgba(value, name)));
        },
        IpgContainerStyleParam::BorderRadius => {
            style.border_radius = Some(try_extract_vec_f32(value, name));
        },
        IpgContainerStyleParam::BorderWidth => {
            style.border_width = Some(try_extract_f32(value, name));
        },
        IpgContainerStyleParam::ShadowIpgColor => {
            let color = IpgColor::extract(value, name);
            style.shadow_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgContainerStyleParam::ShadowRgbaColor => {
            style.border_color = Some(Color::from(IpgColor::extract_rgba(value, name)));
        },
        IpgContainerStyleParam::ShadowOffsetXY => {
            style.shadow_offset_xy = Some(try_extract_array_2(value, name));
        },
        IpgContainerStyleParam::ShadowBlurRadius => {
            style.shadow_blur_radius = Some(try_extract_f32(value, name));
        },
        IpgContainerStyleParam::TextIpgColor => {
            let color = IpgColor::extract(value, name);
            style.text_color = 
                IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
        },
        IpgContainerStyleParam::TextRgbaColor => {
            style.text_color = Some(Color::from(IpgColor::extract_rgba(value, name)));
        },
    }

}

pub fn try_extract_container_style_update(update_obj: &PyObject) -> IpgContainerStyleParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgContainerStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Container style parameter update extraction failed"),
        }
    })
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgContainer {
    type Param = IpgContainerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgContainerParam::AlignBotton => set_opt_bool(&mut self.align_botton, value, name),
            IpgContainerParam::AlignLeft   => set_opt_bool(&mut self.align_left, value, name),
            IpgContainerParam::AlignRight  => set_opt_bool(&mut self.align_right, value, name),
            IpgContainerParam::AlignTop    => set_opt_bool(&mut self.align_top, value, name),
            IpgContainerParam::AlignX      => set_halign(&mut self.align_x, value, name),
            IpgContainerParam::AlignY      => set_valign(&mut self.align_y, value, name),
            IpgContainerParam::Center      => set_opt_bool(&mut self.center, value, name),
            IpgContainerParam::CenterX     => set_opt_bool(&mut self.center_x, value, name),
            IpgContainerParam::CenterY     => set_opt_bool(&mut self.center_y, value, name),
            IpgContainerParam::Clip        => set_opt_bool(&mut self.clip, value, name),
            IpgContainerParam::Height      => set_height(&mut self.height, value, name),
            IpgContainerParam::HeightFill  => set_height_fill(&mut self.height, value, name),
            IpgContainerParam::MaxHeight   => set_opt_f32(&mut self.max_height, value, name),
            IpgContainerParam::MaxWidth    => set_opt_f32(&mut self.max_width, value, name),
            IpgContainerParam::Padding     => set_opt_vec_f32(&mut self.padding, value, name),
            IpgContainerParam::Width       => set_width(&mut self.width, value, name),
            IpgContainerParam::WidthFill   => set_width_fill(&mut self.width, value, name),
            IpgContainerParam::Show        => set_bool(&mut self.show, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgContainerStyle {
    type Param = IpgContainerStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgContainerStyleParam::BackgroundIpgColor  => set_opt_iced_color(&mut self.background_color, value, name),
            IpgContainerStyleParam::BackgroundRgbaColor => set_iced_color_from_rgba(&mut self.background_color, value, name),
            IpgContainerStyleParam::BorderIpgColor      => set_opt_iced_color(&mut self.border_color, value, name),
            IpgContainerStyleParam::BorderRgbaColor     => set_iced_color_from_rgba(&mut self.border_color, value, name),
            IpgContainerStyleParam::BorderRadius        => set_opt_vec_f32(&mut self.border_radius, value, name),
            IpgContainerStyleParam::BorderWidth         => set_opt_f32(&mut self.border_width, value, name),
            IpgContainerStyleParam::ShadowIpgColor      => set_opt_iced_color(&mut self.shadow_color, value, name),
            IpgContainerStyleParam::ShadowRgbaColor     => set_iced_color_from_rgba(&mut self.shadow_color, value, name),
            IpgContainerStyleParam::ShadowOffsetXY      => set_opt_array_2(&mut self.shadow_offset_xy, value, name),
            IpgContainerStyleParam::ShadowBlurRadius    => set_opt_f32(&mut self.shadow_blur_radius, value, name),
            IpgContainerStyleParam::TextIpgColor        => set_opt_iced_color(&mut self.text_color, value, name),
            IpgContainerStyleParam::TextRgbaColor       => set_iced_color_from_rgba(&mut self.text_color, value, name),
        }
    }
}
