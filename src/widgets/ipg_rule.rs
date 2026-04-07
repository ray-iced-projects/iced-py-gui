//! ipg_rule

use iced::widget::rule::{self, FillMode, Style};
use iced::{Element, Theme};
use iced::widget::Container;
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app;

use crate::py_api::helpers::get_radius;
use crate::state::Widgets;
use crate::widgets::widget_param_update::{WidgetParamUpdate, 
    set_opt_bool, set_opt_f32, set_opt_iced_color, 
    set_opt_iced_color_from_rgba, set_opt_u16, set_opt_u16_array_2, 
    set_opt_u32, set_opt_usize, set_opt_vec_f32};

#[derive(Debug, Clone)]
pub struct Rule {
    pub id: usize,
    pub parent_id: String,
    pub is_vertical: Option<bool>,
    pub thickness: Option<u32>,
    pub style_id: Option<usize>,
    pub show: bool,
}

#[derive(Debug, Clone)]
pub struct RuleStyle {
    pub id: usize,
    pub color: Option<iced::Color>,
    pub border_radius: Option<Vec<f32>>,
    pub fillmode_percent: Option<f32>,
    pub fillmode_padded: Option<u16>,
    pub fillmode_asymmetric_padding: Option<[u16; 2]>,
    pub snap: Option<bool>,
}

pub fn construct_rule<'a>(
    rl: &'a Rule, 
    style_opt: Option<&Widgets>) 
    -> Option<Element<'a, app::Message>> {

    if !rl.show {
        return None
    }

    let style = style_opt.and_then(Widgets::as_rule_style).cloned();

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
                style_opt: Option<RuleStyle>,
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
pub enum RuleStyleParam {
    BorderRadius,
    FillModeAsymmetricPadding,
    FillModePadded,
    FillModePercent,
    Color,
    RbgaColor,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
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
            RuleParam::IsVertical => set_opt_bool(&mut self.is_vertical, value, "IsVertical"),
            RuleParam::Thickness => set_opt_u32(&mut self.thickness, value, "Thickness"),
            RuleParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
        }
    }
}

impl WidgetParamUpdate for RuleStyle {
    type Param = RuleStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            RuleStyleParam::BorderRadius => set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
            RuleStyleParam::FillModeAsymmetricPadding => set_opt_u16_array_2(&mut self.fillmode_asymmetric_padding, value, "FillModeAsymmetricPadding"),
            RuleStyleParam::FillModePadded => set_opt_u16(&mut self.fillmode_padded, value, "FillModePadded"),
            RuleStyleParam::FillModePercent => set_opt_f32(&mut self.fillmode_percent, value, "FillModePercent"),
            RuleStyleParam::Color => set_opt_iced_color(&mut self.color, value, "Color"),
            RuleStyleParam::RbgaColor => set_opt_iced_color_from_rgba(&mut self.color, value, "RbgaColor"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_rule() -> Rule {
        Rule {
            id: 0,
            parent_id: String::new(),
            is_vertical: None,
            thickness: None,
            style_id: None,
            show: true,
        }
    }

    fn make_rule_style() -> RuleStyle {
        RuleStyle {
            id: 0,
            color: None,
            border_radius: None,
            fillmode_percent: None,
            fillmode_padded: None,
            fillmode_asymmetric_padding: None,
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

    // -- Rule param tests --

    #[test]
    fn test_is_vertical() {
        let mut rl = make_rule();
        rl.param_update(RuleParam::IsVertical, &py_obj(true));
        assert_eq!(rl.is_vertical, Some(true));
        rl.param_update(RuleParam::IsVertical, &py_none());
        assert_eq!(rl.is_vertical, None);
    }

    #[test]
    fn test_thickness() {
        let mut rl = make_rule();
        rl.param_update(RuleParam::Thickness, &py_obj(3u32));
        assert_eq!(rl.thickness, Some(3));
    }

    #[test]
    fn test_style_id() {
        let mut rl = make_rule();
        rl.param_update(RuleParam::StyleId, &py_obj(10usize));
        assert_eq!(rl.style_id, Some(10));
        rl.param_update(RuleParam::StyleId, &py_none());
        assert_eq!(rl.style_id, None);
    }

    // -- RuleStyle param tests --

    #[test]
    fn test_style_border_radius() {
        let mut s = make_rule_style();
        s.param_update(RuleStyleParam::BorderRadius, &py_obj(vec![2.0f32, 2.0, 2.0, 2.0]));
        assert_eq!(s.border_radius, Some(vec![2.0, 2.0, 2.0, 2.0]));
        s.param_update(RuleStyleParam::BorderRadius, &py_none());
        assert_eq!(s.border_radius, None);
    }

    #[test]
    fn test_style_fillmode_asymmetric_padding() {
        let mut s = make_rule_style();
        s.param_update(RuleStyleParam::FillModeAsymmetricPadding, &py_obj(vec![5u16, 10]));
        assert_eq!(s.fillmode_asymmetric_padding, Some([5, 10]));
    }

    #[test]
    fn test_style_fillmode_padded() {
        let mut s = make_rule_style();
        s.param_update(RuleStyleParam::FillModePadded, &py_obj(8u16));
        assert_eq!(s.fillmode_padded, Some(8));
    }

    #[test]
    fn test_style_fillmode_percent() {
        let mut s = make_rule_style();
        s.param_update(RuleStyleParam::FillModePercent, &py_obj(0.5f32));
        assert_eq!(s.fillmode_percent, Some(0.5));
        s.param_update(RuleStyleParam::FillModePercent, &py_none());
        assert_eq!(s.fillmode_percent, None);
    }

    #[test]
    fn test_style_rgba_color() {
        let mut s = make_rule_style();
        s.param_update(RuleStyleParam::RbgaColor, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.color.is_some());
    }
}
