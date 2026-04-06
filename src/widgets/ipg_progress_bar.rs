//! ipg_progress_bar

use iced::{Element, Length, Theme};
use iced::widget::{progress_bar, ProgressBar};

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app;
use crate::py_api::helpers::{get_radius, try_extract_style_standard};
use crate::state::IpgWidgets;
use crate::widgets::styling::IpgStyleStandard;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool, set_f32, set_height, set_height_fill, set_opt_bool, set_opt_f32, set_opt_iced_color, set_opt_iced_color_from_rgba, set_opt_usize, set_opt_vec_f32, set_width, set_width_fill};

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
    pub background_color: Option<iced::Color>,
    pub bar_color: Option<iced::Color>,
    pub border_color: Option<iced::Color>,
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

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgProgressBarParam::Height => set_height(&mut self.height, value, "Height"),
            IpgProgressBarParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            IpgProgressBarParam::IsVertical => set_opt_bool(&mut self.is_vertical, value, "IsVertical"),
            IpgProgressBarParam::Max => set_f32(&mut self.max, value, "Max"),
            IpgProgressBarParam::Min => set_f32(&mut self.min, value, "Min"),
            IpgProgressBarParam::Show => set_bool(&mut self.show, value, "Show"),
            IpgProgressBarParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            IpgProgressBarParam::StyleStandard => self.style_standard = Some(try_extract_style_standard(value, "StyleStandard")),
            IpgProgressBarParam::Value => set_f32(&mut self.value, value, "Value"),
            IpgProgressBarParam::Width => set_width(&mut self.width, value, "Width"),
            IpgProgressBarParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
        }
    }
}

impl WidgetParamUpdate for IpgProgressBarStyle {
    type Param = IpgProgressBarStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgProgressBarStyleParam::BackgroundIpgColor => 
                set_opt_iced_color(&mut self.background_color, value, "BackgroundIpgColor"),
            IpgProgressBarStyleParam::BackgroundRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.background_color, value, "BackgroundRgbaColor"),
            IpgProgressBarStyleParam::BarIpgColor => 
                set_opt_iced_color(&mut self.bar_color, value, "BarIpgColor"),
            IpgProgressBarStyleParam::BarRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.bar_color, value, "BarRgbaColor"),
            IpgProgressBarStyleParam::BorderIpgColor => 
                set_opt_iced_color(&mut self.border_color, value, "BorderIpgColor"),
            IpgProgressBarStyleParam::BorderRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgbaColor"),
            IpgProgressBarStyleParam::BorderRadius => 
                set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
            IpgProgressBarStyleParam::BorderWidth => 
                set_opt_f32(&mut self.border_width, value, "BorderWidth"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_progress_bar() -> IpgProgressBar {
        IpgProgressBar {
            id: 0,
            parent_id: String::new(),
            show: true,
            min: 0.0,
            max: 100.0,
            value: 50.0,
            is_vertical: None,
            width: Length::Shrink,
            height: Length::Shrink,
            style_standard: None,
            style_id: None,
        }
    }

    fn make_progress_bar_style() -> IpgProgressBarStyle {
        IpgProgressBarStyle::default()
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::attach(|py| py.None().into_py_any(py).unwrap())
    }

    // -- IpgProgressBar param tests --

    #[test]
    fn test_height() {
        let mut pb = make_progress_bar();
        pb.param_update(IpgProgressBarParam::Height, &py_obj(20.0f32));
        assert_eq!(pb.height, Length::Fixed(20.0));
    }

    #[test]
    fn test_height_fill() {
        let mut pb = make_progress_bar();
        pb.param_update(IpgProgressBarParam::HeightFill, &py_obj(true));
        assert_eq!(pb.height, Length::Fill);
    }

    #[test]
    fn test_is_vertical() {
        let mut pb = make_progress_bar();
        pb.param_update(IpgProgressBarParam::IsVertical, &py_obj(true));
        assert_eq!(pb.is_vertical, Some(true));
        pb.param_update(IpgProgressBarParam::IsVertical, &py_none());
        assert_eq!(pb.is_vertical, None);
    }

    #[test]
    fn test_min() {
        let mut pb = make_progress_bar();
        pb.param_update(IpgProgressBarParam::Min, &py_obj(10.0f32));
        assert_eq!(pb.min, 10.0);
    }

    #[test]
    fn test_max() {
        let mut pb = make_progress_bar();
        pb.param_update(IpgProgressBarParam::Max, &py_obj(200.0f32));
        assert_eq!(pb.max, 200.0);
    }

    #[test]
    fn test_show() {
        let mut pb = make_progress_bar();
        pb.param_update(IpgProgressBarParam::Show, &py_obj(false));
        assert!(!pb.show);
    }

    #[test]
    fn test_style_id() {
        let mut pb = make_progress_bar();
        pb.param_update(IpgProgressBarParam::StyleId, &py_obj(5usize));
        assert_eq!(pb.style_id, Some(5));
        pb.param_update(IpgProgressBarParam::StyleId, &py_none());
        assert_eq!(pb.style_id, None);
    }

    #[test]
    fn test_value() {
        let mut pb = make_progress_bar();
        pb.param_update(IpgProgressBarParam::Value, &py_obj(75.0f32));
        assert_eq!(pb.value, 75.0);
    }

    #[test]
    fn test_width() {
        let mut pb = make_progress_bar();
        pb.param_update(IpgProgressBarParam::Width, &py_obj(300.0f32));
        assert_eq!(pb.width, Length::Fixed(300.0));
    }

    #[test]
    fn test_width_fill() {
        let mut pb = make_progress_bar();
        pb.param_update(IpgProgressBarParam::WidthFill, &py_obj(true));
        assert_eq!(pb.width, Length::Fill);
    }

    // -- IpgProgressBarStyle param tests --

    #[test]
    fn test_style_background_rgba() {
        let mut s = make_progress_bar_style();
        s.param_update(IpgProgressBarStyleParam::BackgroundRgbaColor, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.background_color.is_some());
    }

    #[test]
    fn test_style_bar_rgba() {
        let mut s = make_progress_bar_style();
        s.param_update(IpgProgressBarStyleParam::BarRgbaColor, &py_obj(vec![0.0f32, 1.0, 0.0, 1.0]));
        assert!(s.bar_color.is_some());
    }

    #[test]
    fn test_style_border_rgba() {
        let mut s = make_progress_bar_style();
        s.param_update(IpgProgressBarStyleParam::BorderRgbaColor, &py_obj(vec![0.0f32, 0.0, 1.0, 1.0]));
        assert!(s.border_color.is_some());
    }

    #[test]
    fn test_style_border_radius() {
        let mut s = make_progress_bar_style();
        s.param_update(IpgProgressBarStyleParam::BorderRadius, &py_obj(vec![4.0f32, 4.0, 4.0, 4.0]));
        assert_eq!(s.border_radius, Some(vec![4.0, 4.0, 4.0, 4.0]));
        s.param_update(IpgProgressBarStyleParam::BorderRadius, &py_none());
        assert_eq!(s.border_radius, None);
    }

    #[test]
    fn test_style_border_width() {
        let mut s = make_progress_bar_style();
        s.param_update(IpgProgressBarStyleParam::BorderWidth, &py_obj(2.0f32));
        assert_eq!(s.border_width, Some(2.0));
        s.param_update(IpgProgressBarStyleParam::BorderWidth, &py_none());
        assert_eq!(s.border_width, None);
    }
}
