//! ipg_progress_bar

use std::collections::HashMap;

use iced::{Element, Theme};
use iced::widget::{self, progress_bar};

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app;
use crate::graphics::colors::Color;
use crate::py_api::helpers::{get_len, get_radius};
use crate::state::Widgets;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};

#[derive(Debug, Clone)]
pub struct ProgressBar {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    
    pub min: f32,
    pub max: f32,
    pub value: f32, 
    pub is_vertical: Option<bool>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub style_std: Option<ProgressBarStyleStd>,
    pub style_id: Option<usize>,
}

impl ProgressBar {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self, 
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, app::Message>> {
        
        if !self.show { return None }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_progress_bar_style).cloned();

        Some(widget::ProgressBar::new(
            self.min..=self.max, self.value)
                .length(get_len(self.fill, self.width_fill, self.width))
                .girth(get_len(self.fill, self.height_fill, self.height))
                .style(move|theme: &Theme | {
                    if let Some(st) = style_opt.clone() {  
                        st.to_iced(theme, &self.style_std)
                        } else if let Some(std) = &self.style_std {
                            match_style_std(theme, std)
                        } else {
                            progress_bar::primary(theme)
                        }
                    })
                .into()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ProgressBarParam {
    Fill,
    Height,
    HeightFill,
    IsVertical,
    Max,
    Min,
    Show,
    StyleId,
    StyleStd,
    Value,
    Width,
    WidthFill,
}

#[derive(Debug, Clone, Default)]
pub struct ProgressBarStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,
    pub bar_color: Option<Color>,
    pub bar_color_alpha: Option<f32>,
    pub bar_rgba: Option<[f32; 4]>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
}

impl ProgressBarStyle {
    pub fn to_iced(
        &self,
        theme: &Theme,
        style_std: &Option<ProgressBarStyleStd>,
    ) -> progress_bar::Style {
        
        let mut style = 
            if let Some(std) = style_std {
                match_style_std(theme, std)
            } else {
                progress_bar::primary(theme)
            };

        let background_color = 
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
        let bar_color = 
            Color::rgba_ipg_color_to_iced(self.bar_rgba, &self.bar_color, self.bar_color_alpha);
        let border_color = 
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);

        if let Some(bc) = background_color {
            style.background = bc.into();
        }
        if let Some(bc) = bar_color {
            style.bar = bc.into();
        }
        if let Some(bc) = border_color {
            style.border.color = bc;
        }
        if let Some(bw) = self.border_width {
            style.border.width = bw;
        }
        if let Some(br) = &self.border_radius {
            style.border.radius = 
                get_radius(br, "ProgressBar".to_string());
        }
        
        style
        
    }
}

fn match_style_std(
    theme: &Theme, 
    std: &ProgressBarStyleStd
) -> progress_bar::Style {
    match std {
        ProgressBarStyleStd::Primary => {
            progress_bar::primary(theme)
        },
        ProgressBarStyleStd::Secondary => {
            progress_bar::secondary(theme)
        },
        ProgressBarStyleStd::Success => {
            progress_bar::success(theme)
        },
        ProgressBarStyleStd::Danger => {
            progress_bar::danger(theme)
        },
        
        ProgressBarStyleStd::Warning => {
            progress_bar::warning(theme)
        },
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ProgressBarStyleParam {
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRgba,
    BarColor,
    BarColorAlpha,
    BarRgba,
    BorderColor,
    BorderColorAlpha,
    BorderRadius,
    BorderRgba,
    BorderWidth,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ProgressBarStyleStd {
    Danger,
    Primary,
    Secondary,
    Success,
    Warning,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for ProgressBar {
    type Param = ProgressBarParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ProgressBarParam::Fill => set_t_value(&mut self.fill, value, "ProgressBarParam::Fill"),
            ProgressBarParam::Height => set_t_value(&mut self.height, value, "Height"),
            ProgressBarParam::HeightFill => set_t_value(&mut self.height_fill, value, "ProgressBarParam::HeightFill"),
            ProgressBarParam::IsVertical => set_t_value(&mut self.is_vertical, value, "ProgressBarParam::IsVertical"),
            ProgressBarParam::Max => set_t_value(&mut self.max, value, "ProgressBarParam::Max"),
            ProgressBarParam::Min => set_t_value(&mut self.min, value, "ProgressBarParam::Min"),
            ProgressBarParam::Show => set_t_value(&mut self.show, value, "ProgressBarParam::Show"),
            ProgressBarParam::StyleId => set_t_value(&mut self.style_id, value, "StyleId"),
            ProgressBarParam::StyleStd => set_t_value(&mut self.style_std, value, "ProgressBarParam::StyleStd"),
            ProgressBarParam::Value => set_t_value(&mut self.value, value, "ProgressBarParam::Value"),
            ProgressBarParam::Width => set_t_value(&mut self.width, value, "Width"),
            ProgressBarParam::WidthFill => set_t_value(&mut self.width, value, "ProgressBarParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for ProgressBarStyle {
    type Param = ProgressBarStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ProgressBarStyleParam::BackgroundColor => set_t_value(&mut self.background_color, value, "ProgressBarStyleParam::BackgroundColor"),
            ProgressBarStyleParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "ProgressBarStyleParam::BackgroundColorAlpha"),
            ProgressBarStyleParam::BackgroundRgba => set_t_value(&mut self.background_color, value, "ProgressBarStyleParam::BackgroundRgbaColor"),
            ProgressBarStyleParam::BarColor => set_t_value(&mut self.bar_color, value, "ProgressBarStyleParam::BarColor"),
            ProgressBarStyleParam::BarColorAlpha => set_t_value(&mut self.bar_color_alpha, value, "ProgressBarStyleParam::BarColorAlpha"),
            ProgressBarStyleParam::BarRgba => set_t_value(&mut self.bar_color, value, "ProgressBarStyleParam::BarRgbaColor"),
            ProgressBarStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "ProgressBarStyleParam::BorderColor"),
            ProgressBarStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "ProgressBarStyleParam::BorderColorAlpha"),
            ProgressBarStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "ProgressBarStyleParam::BorderRadius"),
            ProgressBarStyleParam::BorderRgba => set_t_value(&mut self.border_color, value, "ProgressBarStyleParam::BorderRgbaColor"),
            ProgressBarStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "ProgressBarStyleParam::BorderWidth"),
        }
    }
}
