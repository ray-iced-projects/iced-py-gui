//! ipg_progress_bar
use iced::{Color, Element, Length, Theme};
use iced::widget::{progress_bar, ProgressBar};
use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use crate::app;
use crate::state::IpgWidgets;
use crate::widgets::styling::IpgStyleStandard;
use crate::widgets::widget_param_update::WidgetParamUpdate;

#[derive(Debug, Clone)]
pub struct IpgProgressBar {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    
    pub min: f32,
    pub max: f32,
    pub value: f32, 
    pub is_vertical: Option<bool>,
    pub width: Length,
    pub height: Length,
    pub style_standard: Option<IpgStyleStandard>,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct IpgProgressBarStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub bar_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
}

pub fn construct_progress_bar<'a>(bar: &'a IpgProgressBar, 
                            style_opt: Option<&'a IpgWidgets>) 
                            -> Option<Element<'a, app::Message>> {
    
    if !bar.show {
        return None
    }

    let style = get_progress_bar_style(style_opt);

    Some(ProgressBar::new(bar.min..=bar.max, bar.value)
                            .length(bar.width)
                            .girth(bar.height)
                            .style(move|theme: &Theme | {   
                                get_styling(theme, 
                                    bar.style_standard.clone(), 
                                    style.clone(), 
                                    )  
                                })
                            .into()
    )
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgProgressBarParam {
    Height,
    IsVertical,
    Min,
    Max,
    Show,
    StyleStandard,
    Style,
    Value,
    Width,
    WidthFill,
}

pub fn get_styling(theme: &Theme,
                    style_standard: Option<IpgStyleStandard>,
                    style_opt: Option<IpgProgressBarStyle>, 
                    ) -> progress_bar::Style 
{
    if style_standard.is_none() && style_opt.is_none() {
        return progress_bar::primary(theme)
    }

    if style_standard.is_some() {
        let style_std = style_standard.unwrap().clone();
        
        let mut std_style = match style_std {
            IpgStyleStandard::Primary => {
                progress_bar::primary(theme)
            },
            IpgStyleStandard::Success => {
                progress_bar::success(theme)
            },
            IpgStyleStandard::Danger => {
                progress_bar::danger(theme)
            },
            IpgStyleStandard::Text => panic!("IpgStandardStyle.Text is not valid for progress bar"),
        };

        if style_opt.is_some() {
            let custom = style_opt.unwrap();
            if custom.border_color.is_some() {
                std_style.border.color = custom.border_color.unwrap();
            }
            if custom.border_width.is_some() {
                 std_style.border.width = custom.border_width.unwrap();
            }
            if custom.border_radius.is_some() {
                std_style.border.radius = get_radius(custom.border_radius.clone().unwrap(),
                                            "ProgressBar".to_string());
            }
        }
        return std_style
    }


    let mut custom = progress_bar::primary(theme);

    //tested above so should unwrap()
    let style = style_opt.unwrap();
    
    if style.background_color.is_some() {
        custom.background = style.background_color.unwrap().into();
    }

    if style.bar_color.is_some() {
        custom.bar = style.bar_color.unwrap().into();
    }

    if style.border_color.is_some() {
        custom.border.color = style.border_color.unwrap();
    }
    if style.border_width.is_some() {
         custom.border.width = style.border_width.unwrap();
    }
    if style.border_radius.is_some() {
        custom.border.radius = get_radius(style.border_radius.clone().unwrap(),
                                    "ProgressBar".to_string());
    }

    custom
 
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgProgressBarStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BarIpgColor,
    BarRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
}

fn get_progress_bar_style(style: Option<&IpgWidgets>) -> Option<IpgProgressBarStyle>{
    match style {
        Some(IpgWidgets::IpgProgressBarStyle(style)) => {
            Some(style.clone())
        }
            _ => None,
        }
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgProgressBar {
    type Param = IpgProgressBarParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgProgressBarParam::Height      => set_height(&mut self.height, value, name),
            IpgProgressBarParam::HeightFill  => set_height_fill(&mut self.height, value, name),
            IpgProgressBarParam::Width       => set_width(&mut self.width, value, name),
            IpgProgressBarParam::WidthFill   => set_width_fill(&mut self.width, value, name),
            IpgProgressBarParam::Show        => set_bool(&mut self.show, value, name),
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
