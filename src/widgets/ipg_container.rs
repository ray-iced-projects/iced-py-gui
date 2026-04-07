//! ipg_container

use std::collections::HashMap;

use crate::app::Message;
use crate::py_api::helpers::get_padding;
use crate::state::Widgets;

use crate::widgets::styling::{apply_background_overrides, 
    apply_border_overrides, apply_shadow_overrides_xy};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_height, set_height_fill, set_opt_bool, 
    set_opt_f32, set_opt_f32_array_2, set_opt_iced_color, set_opt_iced_color_from_rgba, 
    set_opt_usize, set_opt_vec_f32, set_width, set_width_fill, set_lengths_fill
};

use iced::{Element, Length, Theme, alignment};
use iced::widget::{self, Space};

use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct Container {
    pub id: usize,
    pub show: bool,

    pub padding: Option<Vec<f32>>,
    pub width: Length,
    pub height: Length,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub align_top_left: Option<bool>,
    pub align_top_center: Option<bool>,
    pub align_top_right: Option<bool>,
    pub align_center_left: Option<bool>,
    pub align_center: Option<bool>,
    pub align_center_right: Option<bool>,
    pub align_bottom_left: Option<bool>,
    pub align_bottom_center: Option<bool>,
    pub align_bottom_right: Option<bool>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_std: Option<ContainerStyleStd>,
}

impl Container {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        widgets: &HashMap<usize, Widgets>,
        ) -> Element<'a, Message> {
        
        if !self.show {return Space::new().into()}

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_container_style).cloned();

        // Since a container can have only one element and the 
        // the process sends a vec then if empty container, put in a
        // space or remove the element in the vec.
        let new_content: Element<Message> = if content.is_empty() {
            Space::new().into()
        } else {
            content.remove(0)
        };

        let cont = 
            widget::Container::new(new_content)
                .width(self.width)
                .height(self.height)
                .padding(get_padding(&self.padding))
                .style(move|theme|
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, &self.style_std)
                    } else {
                       match &self.style_std {
                            Some(std) => std.to_iced(theme),
                            None => widget::container::transparent(theme),
                        }
                    }
                );

        let cont = 
            if let Some(mw) = self.max_width {
                cont.max_width(mw)
            } else { cont };

        let cont = 
            if let Some(mh) = self.max_height {
                cont.max_width(mh)
            } else { cont };

        let cont = 
            if self.align_top_left == Some(true) {
                cont.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Top)
            } else { cont };

        let cont = 
            if self.align_top_center == Some(true) {
                cont.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Top)
            } else { cont };

        let cont = 
            if self.align_top_right == Some(true) {
                cont.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Top)
            } else { cont };
        
        let cont = 
            if self.align_center_left == Some(true) {
                cont.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Center)
            } else { cont };

        let cont = 
            if self.align_center == Some(true) {
                cont.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center)
            } else { cont };
        
        let cont = 
            if self.align_center_right == Some(true) {
                cont.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Center)
            } else { cont };

        let cont = 
            if self.align_bottom_left == Some(true) {
                cont.align_x(alignment::Horizontal::Left)
                    .align_y(alignment::Vertical::Bottom)
            } else { cont };

        let cont = 
            if self.align_bottom_center == Some(true) {
                cont.align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Bottom)
            } else { cont };

        let cont = 
            if self.align_bottom_right == Some(true) {
                cont.align_x(alignment::Horizontal::Right)
                    .align_y(alignment::Vertical::Bottom)
            } else { cont };

        let cont = 
            if self.clip == Some(true) {
                cont.clip(true)
            } else { cont };

        cont.into()            
        
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerStyle {
    pub id: usize,
    pub background_color: Option<iced::Color>,
    pub background_gradient_color_stop: Option<iced::Color>,
    pub background_gradient_degrees: Option<f32>,
    pub background_gradient_radians: Option<f32>,
    pub border_color: Option<iced::Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub shadow_color: Option<iced::Color>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,
    pub text_color: Option<iced::Color>,
    pub snap: Option<bool>,
}

impl ContainerStyle {
    pub fn to_iced(
        &self, 
        theme: &Theme, 
        std_style_opt: &Option<ContainerStyleStd>,  
        ) -> widget::container::Style {
        
        let mut style = 
            if let Some(st) = std_style_opt {
                st.to_iced(theme)
            } else { widget::container::transparent(theme) };

        

        // Apply remaining optional overrides
        apply_background_overrides(
            &mut style.background, self.background_color,
            self.background_gradient_color_stop,
            self.background_gradient_degrees,
            self.background_gradient_radians,
        );

        apply_border_overrides(
            &mut style.border, self.border_color,
            &self.border_radius, self.border_width, "Container",
        );

        apply_shadow_overrides_xy(
            &mut style.shadow, self.shadow_color, 
            self.shadow_offset_xy, self.shadow_blur_radius);
        
        style.text_color = self.text_color;

        if let Some(sp) = self.snap {
            style.snap = sp;
        }

        style
        
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ContainerStyleStd {
    BorderedBox,
    Danger,
    Dark,
    Primary,
    RoundedBox,
    Secondary,
    Success,
    Transparent,
    Warning,
}

impl ContainerStyleStd {
    pub fn to_iced(
        &self,
        theme: &Theme, 
        ) -> widget::container::Style {
        
        match self {
            ContainerStyleStd::BorderedBox => {
                widget::container::bordered_box(theme)
            },
            ContainerStyleStd::Danger => {
                widget::container::danger(theme)
            },
            ContainerStyleStd::Dark => {
                widget::container::dark(theme)
            },
            ContainerStyleStd::Primary => {
                widget::container::primary(theme)
            },
            ContainerStyleStd::RoundedBox => {
                widget::container::rounded_box(theme)
            },
            ContainerStyleStd::Secondary => {
                widget::container::secondary(theme)
            },
            ContainerStyleStd::Success => {
                widget::container::success(theme)
            },
            ContainerStyleStd::Warning => {
                widget::container::warning(theme)
            },
            ContainerStyleStd::Transparent => {
                widget::container::transparent(theme)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ContainerParam {
    AlignBottomCenter,
    AlignBottomLeft,
    AlignBottomRight,
    AlignCenter,
    AlignCenterLeft,
    AlignCenterRight,
    AlignTopCenter,
    AlignTopLeft,
    AlignTopRight,
    Clip,
    Fill,
    Height,
    HeightFill,
    MaxHeight,
    MaxWidth,
    Padding,
    Width,
    WidthFill,
    Show,
    StyleId,
    StyleStd,
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ContainerStyleParam {
    BackgroundColor,
    BackgroundRgbaColor,
    BorderColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    ShadowColor,
    ShadowRgbaColor,
    ShadowOffsetXY,
    ShadowBlurRadius,
    TextColor,
    TextRgbaColor,
}

pub fn try_extract_container_style_param(update_obj: &PyObject) -> ContainerStyleStd{

    Python::attach(|py| {
        let res = update_obj.extract::<ContainerStyleStd>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("ContainerStyleStd update extraction failed"),
        }
    })
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Container {
    type Param = ContainerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ContainerParam::AlignBottomCenter => set_opt_bool(&mut self.align_bottom_center, value, "AlignBottomCenter"),
            ContainerParam::AlignBottomLeft => set_opt_bool(&mut self.align_bottom_left, value, "AlignBottomLeft"),
            ContainerParam::AlignBottomRight => set_opt_bool(&mut self.align_bottom_right, value, "AlignBottomRight"),
            ContainerParam::AlignCenter => set_opt_bool(&mut self.align_center, value, "AlignCenter"),
            ContainerParam::AlignCenterLeft => set_opt_bool(&mut self.align_center_left, value, "AlignCenterLeft"),
            ContainerParam::AlignCenterRight => set_opt_bool(&mut self.align_center_right, value, "AlignCenterRight"),
            ContainerParam::AlignTopCenter => set_opt_bool(&mut self.align_top_center, value, "AlignTopCenter"),
            ContainerParam::AlignTopLeft => set_opt_bool(&mut self.align_top_left, value, "AlignTopLeft"),
            ContainerParam::AlignTopRight => set_opt_bool(&mut self.align_top_right, value, "AlignTopRight"),
            ContainerParam::Clip => set_opt_bool(&mut self.clip, value, "Clip"),
            ContainerParam::Fill => set_lengths_fill(&mut self.width, &mut self.height, value, "Fill"),
            ContainerParam::Height => set_height(&mut self.height, value, "Height"),
            ContainerParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            ContainerParam::MaxHeight => set_opt_f32(&mut self.max_height, value, "MaxHeight"),
            ContainerParam::MaxWidth => set_opt_f32(&mut self.max_width, value, "MaxWidth"),
            ContainerParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            ContainerParam::Width => set_width(&mut self.width, value, "Width"),
            ContainerParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
            ContainerParam::Show => set_bool(&mut self.show, value, "Show"),
            ContainerParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            ContainerParam::StyleStd => self.style_std = Some(try_extract_container_style_param(value)),
        }
    }
}

impl WidgetParamUpdate for ContainerStyle {
    type Param = ContainerStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ContainerStyleParam::BackgroundColor  => set_opt_iced_color(&mut self.background_color, value, "BackgroundColor"),
            ContainerStyleParam::BackgroundRgbaColor => set_opt_iced_color_from_rgba(&mut self.background_color, value, "BackgroundRgbaColor"),
            ContainerStyleParam::BorderColor => set_opt_iced_color(&mut self.border_color, value, "BorderColor"),
            ContainerStyleParam::BorderRgbaColor => set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgbaColor"),
            ContainerStyleParam::BorderRadius => set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
            ContainerStyleParam::BorderWidth => set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            ContainerStyleParam::ShadowColor => set_opt_iced_color(&mut self.shadow_color, value, "ShadowColor"),
            ContainerStyleParam::ShadowRgbaColor => set_opt_iced_color_from_rgba(&mut self.shadow_color, value, "ShadowRgbaColor"),
            ContainerStyleParam::ShadowOffsetXY => set_opt_f32_array_2(&mut self.shadow_offset_xy, value, "ShadowOffsetXY"),
            ContainerStyleParam::ShadowBlurRadius => set_opt_f32(&mut self.shadow_blur_radius, value, "ShadowBlurRadius"),
            ContainerStyleParam::TextColor => set_opt_iced_color(&mut self.text_color, value, "TextColor"),
            ContainerStyleParam::TextRgbaColor => set_opt_iced_color_from_rgba(&mut self.text_color, value, "TextRgbaColor"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{IntoPyObjectExt, Python};

    fn make_container() -> Container {
        Container {
            id: 0,
            show: true,
            padding: None,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: None,
            max_height: None,
            align_top_left: None,
            align_top_center: None,
            align_top_right: None,
            align_center_left: None,
            align_center: None,
            align_center_right: None,
            align_bottom_left: None,
            align_bottom_center: None,
            align_bottom_right: None,
            clip: None,
            style_id: None,
            style_std: None,
        }
    }

    fn make_container_style() -> ContainerStyle {
        ContainerStyle {
            id: 0,
            background_color: None,
            background_gradient_color_stop: None,
            background_gradient_degrees: None,
            background_gradient_radians: None,
            border_color: None,
            border_radius: None,
            border_width: None,
            shadow_color: None,
            shadow_offset_xy: None,
            shadow_blur_radius: None,
            text_color: None,
            snap: None,
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

    // -- Container param tests --

    #[test]
    fn test_align_bottom_center() {
        let mut c = make_container();
        c.param_update(ContainerParam::AlignBottomCenter, &py_obj(true));
        assert_eq!(c.align_bottom_center, Some(true));
        c.param_update(ContainerParam::AlignBottomCenter, &py_none());
        assert_eq!(c.align_bottom_center, None);
    }

    #[test]
    fn test_align_bottom_left() {
        let mut c = make_container();
        c.param_update(ContainerParam::AlignBottomLeft, &py_obj(true));
        assert_eq!(c.align_bottom_left, Some(true));
    }

    #[test]
    fn test_align_bottom_right() {
        let mut c = make_container();
        c.param_update(ContainerParam::AlignBottomRight, &py_obj(true));
        assert_eq!(c.align_bottom_right, Some(true));
    }

    #[test]
    fn test_align_center() {
        let mut c = make_container();
        c.param_update(ContainerParam::AlignCenter, &py_obj(true));
        assert_eq!(c.align_center, Some(true));
    }

    #[test]
    fn test_align_center_left() {
        let mut c = make_container();
        c.param_update(ContainerParam::AlignCenterLeft, &py_obj(true));
        assert_eq!(c.align_center_left, Some(true));
    }

    #[test]
    fn test_align_center_right() {
        let mut c = make_container();
        c.param_update(ContainerParam::AlignCenterRight, &py_obj(true));
        assert_eq!(c.align_center_right, Some(true));
    }

    #[test]
    fn test_align_top_center() {
        let mut c = make_container();
        c.param_update(ContainerParam::AlignTopCenter, &py_obj(true));
        assert_eq!(c.align_top_center, Some(true));
    }

    #[test]
    fn test_align_top_left() {
        let mut c = make_container();
        c.param_update(ContainerParam::AlignTopLeft, &py_obj(true));
        assert_eq!(c.align_top_left, Some(true));
    }

    #[test]
    fn test_align_top_right() {
        let mut c = make_container();
        c.param_update(ContainerParam::AlignTopRight, &py_obj(true));
        assert_eq!(c.align_top_right, Some(true));
    }

    #[test]
    fn test_clip() {
        let mut c = make_container();
        c.param_update(ContainerParam::Clip, &py_obj(true));
        assert_eq!(c.clip, Some(true));
        c.param_update(ContainerParam::Clip, &py_none());
        assert_eq!(c.clip, None);
    }

    #[test]
    fn test_fill() {
        let mut c = make_container();
        c.param_update(ContainerParam::Fill, &py_obj(Some(true)));
        assert_eq!(c.width, Length::Fill);
        assert_eq!(c.height, Length::Fill);
        c.param_update(ContainerParam::Fill, &py_obj(Some(false)));
        assert_eq!(c.width, Length::Shrink);
        assert_eq!(c.height, Length::Shrink);
        c.param_update(ContainerParam::Fill, &py_none());
        assert_eq!(c.width, Length::Shrink);
        assert_eq!(c.height, Length::Shrink);
    }

    #[test]
    fn test_height() {
        let mut c = make_container();
        c.param_update(ContainerParam::Height, &py_obj(50.0f32));
        assert_eq!(c.height, Length::Fixed(50.0));
    }

    #[test]
    fn test_height_fill() {
        let mut c = make_container();
        c.param_update(ContainerParam::HeightFill, &py_obj(true));
        assert_eq!(c.height, Length::Fill);
    }

    #[test]
    fn test_max_height() {
        let mut c = make_container();
        c.param_update(ContainerParam::MaxHeight, &py_obj(300.0f32));
        assert_eq!(c.max_height, Some(300.0));
        c.param_update(ContainerParam::MaxHeight, &py_none());
        assert_eq!(c.max_height, None);
    }

    #[test]
    fn test_max_width() {
        let mut c = make_container();
        c.param_update(ContainerParam::MaxWidth, &py_obj(500.0f32));
        assert_eq!(c.max_width, Some(500.0));
        c.param_update(ContainerParam::MaxWidth, &py_none());
        assert_eq!(c.max_width, None);
    }

    #[test]
    fn test_padding() {
        let mut c = make_container();
        c.param_update(ContainerParam::Padding, &py_obj(vec![5.0f32, 10.0]));
        assert_eq!(c.padding, Some(vec![5.0, 10.0]));
        c.param_update(ContainerParam::Padding, &py_none());
        assert_eq!(c.padding, None);
    }

    #[test]
    fn test_width() {
        let mut c = make_container();
        c.param_update(ContainerParam::Width, &py_obj(200.0f32));
        assert_eq!(c.width, Length::Fixed(200.0));
    }

    #[test]
    fn test_width_fill() {
        let mut c = make_container();
        c.param_update(ContainerParam::WidthFill, &py_obj(true));
        assert_eq!(c.width, Length::Fill);
    }

    #[test]
    fn test_show() {
        let mut c = make_container();
        c.param_update(ContainerParam::Show, &py_obj(false));
        assert!(!c.show);
    }

    #[test]
    fn test_style_id() {
        let mut c = make_container();
        c.param_update(ContainerParam::StyleId, &py_obj(42usize));
        assert_eq!(c.style_id, Some(42));
        c.param_update(ContainerParam::StyleId, &py_none());
        assert_eq!(c.style_id, None);
    }

    // -- ContainerStyle param tests --

    #[test]
    fn test_style_background_rgba() {
        let mut s = make_container_style();
        s.param_update(ContainerStyleParam::BackgroundRgbaColor, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.background_color.is_some());
    }

    #[test]
    fn test_style_border_rgba() {
        let mut s = make_container_style();
        s.param_update(ContainerStyleParam::BorderRgbaColor, &py_obj(vec![0.0f32, 1.0, 0.0, 1.0]));
        assert!(s.border_color.is_some());
    }

    #[test]
    fn test_style_border_radius() {
        let mut s = make_container_style();
        s.param_update(ContainerStyleParam::BorderRadius, &py_obj(vec![5.0f32, 5.0, 5.0, 5.0]));
        assert_eq!(s.border_radius, Some(vec![5.0, 5.0, 5.0, 5.0]));
        s.param_update(ContainerStyleParam::BorderRadius, &py_none());
        assert_eq!(s.border_radius, None);
    }

    #[test]
    fn test_style_border_width() {
        let mut s = make_container_style();
        s.param_update(ContainerStyleParam::BorderWidth, &py_obj(2.0f32));
        assert_eq!(s.border_width, Some(2.0));
        s.param_update(ContainerStyleParam::BorderWidth, &py_none());
        assert_eq!(s.border_width, None);
    }

    #[test]
    fn test_style_shadow_rgba() {
        let mut s = make_container_style();
        s.param_update(ContainerStyleParam::ShadowRgbaColor, &py_obj(vec![0.0f32, 0.0, 0.0, 0.5]));
        assert!(s.shadow_color.is_some());
    }

    #[test]
    fn test_style_shadow_offset_xy() {
        let mut s = make_container_style();
        s.param_update(ContainerStyleParam::ShadowOffsetXY, &py_obj(vec![2.0f32, 3.0]));
        assert_eq!(s.shadow_offset_xy, Some([2.0, 3.0]));
    }

    #[test]
    fn test_style_shadow_blur_radius() {
        let mut s = make_container_style();
        s.param_update(ContainerStyleParam::ShadowBlurRadius, &py_obj(4.0f32));
        assert_eq!(s.shadow_blur_radius, Some(4.0));
        s.param_update(ContainerStyleParam::ShadowBlurRadius, &py_none());
        assert_eq!(s.shadow_blur_radius, None);
    }

    #[test]
    fn test_style_text_rgba() {
        let mut s = make_container_style();
        s.param_update(ContainerStyleParam::TextRgbaColor, &py_obj(vec![1.0f32, 1.0, 1.0, 1.0]));
        assert!(s.text_color.is_some());
    }
}
