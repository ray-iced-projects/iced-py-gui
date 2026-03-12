//! ipg_rule
use iced::widget::rule::{self, FillMode, Style};
use iced::{Color, Element, Theme};
use iced::widget::Container;
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app;

use crate::py_api::helpers::get_radius;
use crate::state::IpgWidgets;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_opt_bool, set_opt_f32, set_opt_iced_color, set_opt_iced_color_from_rgba, set_opt_u16, set_opt_u16_array_2, set_opt_u32, set_opt_usize, set_opt_vec_f32};

#[derive(Debug, Clone)]
pub struct IpgRule {
    pub id: usize,
    pub parent_id: String,
    pub is_vertical: Option<bool>,
    pub thickness: Option<u32>,
    pub style_id: Option<usize>,
    pub show: bool,
}

#[derive(Debug, Clone)]
pub struct IpgRuleStyle {
    pub id: usize,
    pub color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub fillmode_percent: Option<f32>,
    pub fillmode_padded: Option<u16>,
    pub fillmode_asymmetric_padding: Option<[u16; 2]>,
    pub snap: Option<bool>,
}

pub fn construct_rule<'a>(
    rl: &'a IpgRule, 
    style_opt: Option<&IpgWidgets>) 
    -> Option<Element<'a, app::Message>> {

    if !rl.show {
        return None
    }

    let style = style_opt.and_then(IpgWidgets::as_rule_style).cloned();

    let thickness = if let Some(th) = rl.thickness {
        th
    } else { 1 };

    let rul = if let Some(_) = rl.is_vertical {
        rule::vertical(thickness)
            .style(move|theme: &Theme| {   
                get_styling(theme, style.clone())  
            })
    } else {
        rule::horizontal(thickness)
            .style(move|theme: &Theme| {   
                get_styling(theme, style.clone())  
            })
    };


    Some(Container::new(rul).into())

}



fn get_styling(theme: &Theme,
                style_opt: Option<IpgRuleStyle>,
                ) -> Style {

    let mut base_style = rule::default(theme);

    let style = if let Some(st) = style_opt {
        st
    } else { return  base_style };

    if let Some(c) = style.color {
        base_style.color = c;
    }


    if let Some(br)  = style.border_radius {
        base_style.radius = 
            get_radius(&br, "Rule".to_string()); 
    }

    base_style.fill_mode = 
        if let Some(percent) = style.fillmode_percent {
            FillMode::Percent(percent)
        } else if let Some(pd) = style.fillmode_padded {
            FillMode::Padded(pd)
        } else if let Some(a_pd) = style.fillmode_asymmetric_padding {
            FillMode::AsymmetricPadding(a_pd[0], a_pd[1])
        } else {
            FillMode::Full
        };
    
    if let Some(sp) = style.snap { 
        base_style.snap = sp;
    };
    

    base_style

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRuleStyleParam {
    BorderRadius,
    FillModeAsymmetricPadding,
    FillModePadded,
    FillModePercent,
    IpgColor,
    RbgaColor,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRuleParam {
    IsVertical,
    Thickness,
    StyleId
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgRule {
    type Param = IpgRuleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgRuleParam::IsVertical => set_opt_bool(&mut self.is_vertical, value, "IsVertical"),
            IpgRuleParam::Thickness => set_opt_u32(&mut self.thickness, value, "Thickness"),
            IpgRuleParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
        }
    }
}

impl WidgetParamUpdate for IpgRuleStyle {
    type Param = IpgRuleStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgRuleStyleParam::BorderRadius => set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
            IpgRuleStyleParam::FillModeAsymmetricPadding => set_opt_u16_array_2(&mut self.fillmode_asymmetric_padding, value, "FillModeAsymmetricPadding"),
            IpgRuleStyleParam::FillModePadded => set_opt_u16(&mut self.fillmode_padded, value, "FillModePadded"),
            IpgRuleStyleParam::FillModePercent => set_opt_f32(&mut self.fillmode_percent, value, "FillModePercent"),
            IpgRuleStyleParam::IpgColor => set_opt_iced_color(&mut self.color, value, "IpgColor"),
            IpgRuleStyleParam::RbgaColor => set_opt_iced_color_from_rgba(&mut self.color, value, "RbgaColor"),
        }
    }
}
