//! ipg_progress_bar
use iced::{Color, Element, Length, Theme};
use iced::widget::{progress_bar, ProgressBar};

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app;
use crate::py_api::helpers::{get_radius, try_extract_style_standard};
use crate::state::IpgWidgets;
use crate::widgets::styling::IpgStyleStandard;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool, set_f32, set_height, set_height_fill, set_iced_color_from_rgba, set_opt_bool, set_opt_f32, set_opt_iced_color, set_opt_usize, set_opt_vec_f32, set_width, set_width_fill};

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

    let style = style_opt.and_then(IpgWidgets::as_progress_bar_style).cloned();

    Some(ProgressBar::new(
        bar.min..=bar.max, bar.value)
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
    HeightFill,
    IsVertical,
    Min,
    Max,
    Show,
    StyleStandard,
    StyleId,
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
            IpgStyleStandard::Secondary => {
                progress_bar::secondary(theme)
            },
            IpgStyleStandard::Success => {
                progress_bar::success(theme)
            },
            IpgStyleStandard::Danger => {
                progress_bar::danger(theme)
            },
            IpgStyleStandard::Text => {
                eprint!("[WARN] IpgStandardStyle.Text 
                is not valid for progress bar, defaulting to primary.");
                progress_bar::primary(theme)
            },
            IpgStyleStandard::Warning => {
                progress_bar::warning(theme)
            },
        };

        if let Some(custom) = style_opt {
            if let Some(bc) = custom.border_color {
                std_style.border.color = bc;
            }
            if let Some(bw) = custom.border_width {
                 std_style.border.width = bw;
            }
            if let Some(br) = custom.border_radius {
                std_style.border.radius = 
                    get_radius(&br, "ProgressBar".to_string());
            }
        }
        return std_style
    }


    let mut custom = progress_bar::primary(theme);

    //tested above so should unwrap()
    let style = style_opt.unwrap();
    
    if let Some(bc) = style.background_color {
        custom.background = bc.into();
    }

    if let Some(bc) = style.bar_color {
        custom.bar = bc.into();
    }

    if let Some(bc) = style.border_color {
        custom.border.color = bc;
    }
    if let Some(bw) = style.border_width {
         custom.border.width = bw;
    }
    if let Some(br) = style.border_radius {
        custom.border.radius = 
            get_radius(&br,"ProgressBar".to_string());
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



// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgProgressBar {
    type Param = IpgProgressBarParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgProgressBarParam::Height => set_height(&mut self.height, value, name),
            IpgProgressBarParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgProgressBarParam::IsVertical => set_opt_bool(&mut self.is_vertical, value, name),
            IpgProgressBarParam::Max => set_f32(&mut self.max, value, name),
            IpgProgressBarParam::Min => set_f32(&mut self.min, value, name),
            IpgProgressBarParam::Show => set_bool(&mut self.show, value, name),
            IpgProgressBarParam::StyleId => set_opt_usize(&mut self.style_id, value, name),
            IpgProgressBarParam::StyleStandard => self.style_standard = Some(try_extract_style_standard(value, name)),
            IpgProgressBarParam::Value => set_f32(&mut self.value, value, name),
            IpgProgressBarParam::Width => set_width(&mut self.width, value, name),
            IpgProgressBarParam::WidthFill => set_width_fill(&mut self.width, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgProgressBarStyle {
    type Param = IpgProgressBarStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgProgressBarStyleParam::BackgroundIpgColor => 
                set_opt_iced_color(&mut self.background_color, value, name),
            IpgProgressBarStyleParam::BackgroundRgbaColor => 
                set_iced_color_from_rgba(&mut self.background_color, value, name),
            IpgProgressBarStyleParam::BarIpgColor => 
                set_opt_iced_color(&mut self.bar_color, value, name),
            IpgProgressBarStyleParam::BarRgbaColor => 
                set_iced_color_from_rgba(&mut self.bar_color, value, name),
            IpgProgressBarStyleParam::BorderIpgColor => 
                set_opt_iced_color(&mut self.border_color, value, name),
            IpgProgressBarStyleParam::BorderRgbaColor => 
                set_iced_color_from_rgba(&mut self.border_color, value, name),
            IpgProgressBarStyleParam::BorderRadius => 
                set_opt_vec_f32(&mut self.border_radius, value, name),
            IpgProgressBarStyleParam::BorderWidth => 
                set_opt_f32(&mut self.border_width, value, name),
        }
    }
}
