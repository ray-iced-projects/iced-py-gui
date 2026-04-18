//! ipg_rule

use std::collections::HashMap;

use iced::widget::rule::{self, FillMode, Style};
use iced::{Element, Theme};
use iced::widget::Container;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app;

use crate::graphics::colors::Color;
use crate::py_api::helpers::get_radius;
use crate::state::Widgets;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};

#[derive(Debug, Clone)]
pub struct Rule {
    pub id: usize,
    pub is_vertical: Option<bool>,
    pub thickness: Option<u32>,
    pub style_id: Option<usize>,
    pub show: bool,
}

impl Rule {

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
                .and_then(Widgets::as_rule_style).cloned();

        let thickness = if let Some(th) = self.thickness {
            th
        } else { 1 };

        let rul = if self.is_vertical == Some(true) {
            rule::vertical(thickness)
                .style(move|theme: &Theme| {  
                    if let Some(st) = &style_opt {
                        st.to_iced(theme)
                    } else {
                        rule::default(theme)
                    }
                })
        } else {
            rule::horizontal(thickness)
                .style(move|theme: &Theme| {   
                    if let Some(st) = &style_opt {
                        st.to_iced(theme)
                    } else {
                        rule::default(theme)
                    } 
                })
        };


        Some(Container::new(rul).into())

    }
}
#[derive(Debug, Clone)]
pub struct RuleStyle {
    pub id: usize,
    pub color: Option<Color>,
    pub color_alpha: Option<f32>,
    pub rgba: Option<[f32; 4]>,
    pub border_radius: Option<Vec<f32>>,
    pub fillmode_percent: Option<f32>,
    pub fillmode_padded: Option<u16>,
    pub fillmode_asymmetric_padding: Option<[u16; 2]>,
    pub snap: Option<bool>,
}

impl RuleStyle {
    fn to_iced(
        &self,
        theme: &Theme,
    ) -> Style {

        let mut style = rule::default(theme);

        let color = Color::rgba_ipg_color_to_iced(self.rgba, &self.color, self.color_alpha);

        if let Some(c) = color {
            style.color = c;
        }

        if let Some(br)  = &self.border_radius {
            style.radius = 
                get_radius(br, "Rule".to_string()); 
        }

        style.fill_mode = 
            if let Some(percent) = self.fillmode_percent {
                FillMode::Percent(percent)
            } else if let Some(pd) = self.fillmode_padded {
                FillMode::Padded(pd)
            } else if let Some(a_pd) = self.fillmode_asymmetric_padding {
                FillMode::AsymmetricPadding(a_pd[0], a_pd[1])
            } else {
                FillMode::Full
            };
        
        if let Some(sp) = self.snap { 
            style.snap = sp;
        };
        
        style

    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum RuleStyleParam {
    BorderRadius,
    Color,
    ColorAlpha,
    FillModeAsymmetricPadding,
    FillModePadded,
    FillModePercent,
    Rgba,
    Snap,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum RuleParam {
    IsVertical,
    Thickness,
    StyleId
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Rule {
    type Param = RuleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RuleParam::IsVertical => set_t_value(&mut self.is_vertical, value, "RuleParam::IsVertical"),
            RuleParam::Thickness => set_t_value(&mut self.thickness, value, "RuleParam::Thickness"),
            RuleParam::StyleId => set_t_value(&mut self.style_id, value, "RuleParam::StyleId"),
        }
    }
}

impl WidgetParamUpdate for RuleStyle {
    type Param = RuleStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RuleStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "RuleStyleParam::BorderRadius"),
            RuleStyleParam::Color => set_t_value(&mut self.color, value, "RuleStyleParam::Color"),
            RuleStyleParam::ColorAlpha => set_t_value(&mut self.color_alpha, value, "RuleStyleParam::ColorAlpha"),
            RuleStyleParam::FillModeAsymmetricPadding => set_t_value(&mut self.fillmode_asymmetric_padding, value, "RuleStyleParam::FillModeAsymmetricPadding"),
            RuleStyleParam::FillModePadded => set_t_value(&mut self.fillmode_padded, value, "RuleStyleParam::FillModePadded"),
            RuleStyleParam::FillModePercent => set_t_value(&mut self.fillmode_percent, value, "RuleStyleParam::FillModePercent"),
            RuleStyleParam::Rgba => set_t_value(&mut self.color, value, "RuleStyleParam::Rbga"),
            RuleStyleParam::Snap => set_t_value(&mut self.snap, value, "RuleStyleParam::Snap"),
        }
    }
}
