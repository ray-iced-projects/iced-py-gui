//! ipg_progress_bar

use iced::{Element, Length, Theme};
use iced::widget::{self, progress_bar};

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::app;
use crate::py_api::helpers::{get_radius, try_extract_style_standard};
use crate::state::Widgets;
use crate::widgets::styling::StyleStandard;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool, set_f32, set_height, set_height_fill, set_opt_bool, set_opt_f32, set_opt_iced_color, set_opt_iced_color_from_rgba, set_opt_usize, set_opt_vec_f32, set_width, set_width_fill};

#[derive(Debug, Clone)]
pub struct ProgressBar {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    
    pub min: f32,
    pub max: f32,
    pub value: f32, 
    pub is_vertical: Option<bool>,
    pub width: Length,
    pub height: Length,
    pub style_standard: Option<StyleStandard>,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct ProgressBarStyle {
    pub id: usize,
    pub background_color: Option<iced::Color>,
    pub bar_color: Option<iced::Color>,
    pub border_color: Option<iced::Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
}

pub fn construct_progress_bar<'a>(bar: &'a ProgressBar, 
                            style_opt: Option<&'a Widgets>) 
                            -> Option<Element<'a, app::Message>> {
    
    if !bar.show {
        return None
    }

    let style = style_opt.and_then(Widgets::as_progress_bar_style).cloned();

    Some(widget::ProgressBar::new(
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


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ProgressBarParam {
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
                    style_standard: Option<StyleStandard>,
                    style_opt: Option<ProgressBarStyle>, 
                    ) -> progress_bar::Style 
{
    if style_standard.is_none() && style_opt.is_none() {
        return progress_bar::primary(theme)
    }

    if style_standard.is_some() {
        let style_std = style_standard.unwrap().clone();
        
        let mut std_style = match style_std {
            StyleStandard::Primary => {
                progress_bar::primary(theme)
            },
            StyleStandard::Secondary => {
                progress_bar::secondary(theme)
            },
            StyleStandard::Success => {
                progress_bar::success(theme)
            },
            StyleStandard::Danger => {
                progress_bar::danger(theme)
            },
            StyleStandard::Text => {
                eprint!("[WARN] StandardStyle.Text 
                is not valid for progress bar, defaulting to primary.");
                progress_bar::primary(theme)
            },
            StyleStandard::Warning => {
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

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ProgressBarStyleParam {
    BackgroundColor,
    BackgroundRgbaColor,
    BarColor,
    BarRgbaColor,
    BorderColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
}



// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for ProgressBar {
    type Param = ProgressBarParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ProgressBarParam::Height => set_height(&mut self.height, value, "Height"),
            ProgressBarParam::HeightFill => set_height_fill(&mut self.height, value, "HeightFill"),
            ProgressBarParam::IsVertical => set_opt_bool(&mut self.is_vertical, value, "IsVertical"),
            ProgressBarParam::Max => set_f32(&mut self.max, value, "Max"),
            ProgressBarParam::Min => set_f32(&mut self.min, value, "Min"),
            ProgressBarParam::Show => set_bool(&mut self.show, value, "Show"),
            ProgressBarParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            ProgressBarParam::StyleStandard => self.style_standard = Some(try_extract_style_standard(value, "StyleStandard")),
            ProgressBarParam::Value => set_f32(&mut self.value, value, "Value"),
            ProgressBarParam::Width => set_width(&mut self.width, value, "Width"),
            ProgressBarParam::WidthFill => set_width_fill(&mut self.width, value, "WidthFill"),
        }
    }
}

impl WidgetParamUpdate for ProgressBarStyle {
    type Param = ProgressBarStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ProgressBarStyleParam::BackgroundColor => 
                set_opt_iced_color(&mut self.background_color, value, "BackgroundColor"),
            ProgressBarStyleParam::BackgroundRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.background_color, value, "BackgroundRgbaColor"),
            ProgressBarStyleParam::BarColor => 
                set_opt_iced_color(&mut self.bar_color, value, "BarColor"),
            ProgressBarStyleParam::BarRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.bar_color, value, "BarRgbaColor"),
            ProgressBarStyleParam::BorderColor => 
                set_opt_iced_color(&mut self.border_color, value, "BorderColor"),
            ProgressBarStyleParam::BorderRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgbaColor"),
            ProgressBarStyleParam::BorderRadius => 
                set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
            ProgressBarStyleParam::BorderWidth => 
                set_opt_f32(&mut self.border_width, value, "BorderWidth"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_progress_bar() -> ProgressBar {
        ProgressBar {
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

    fn make_progress_bar_style() -> ProgressBarStyle {
        ProgressBarStyle::default()
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::attach(|py| py.None().into_py_any(py).unwrap())
    }

    // -- ProgressBar param tests --

    #[test]
    fn test_height() {
        let mut pb = make_progress_bar();
        pb.param_update(ProgressBarParam::Height, &py_obj(20.0f32));
        assert_eq!(pb.height, Length::Fixed(20.0));
    }

    #[test]
    fn test_height_fill() {
        let mut pb = make_progress_bar();
        pb.param_update(ProgressBarParam::HeightFill, &py_obj(true));
        assert_eq!(pb.height, Length::Fill);
    }

    #[test]
    fn test_is_vertical() {
        let mut pb = make_progress_bar();
        pb.param_update(ProgressBarParam::IsVertical, &py_obj(true));
        assert_eq!(pb.is_vertical, Some(true));
        pb.param_update(ProgressBarParam::IsVertical, &py_none());
        assert_eq!(pb.is_vertical, None);
    }

    #[test]
    fn test_min() {
        let mut pb = make_progress_bar();
        pb.param_update(ProgressBarParam::Min, &py_obj(10.0f32));
        assert_eq!(pb.min, 10.0);
    }

    #[test]
    fn test_max() {
        let mut pb = make_progress_bar();
        pb.param_update(ProgressBarParam::Max, &py_obj(200.0f32));
        assert_eq!(pb.max, 200.0);
    }

    #[test]
    fn test_show() {
        let mut pb = make_progress_bar();
        pb.param_update(ProgressBarParam::Show, &py_obj(false));
        assert!(!pb.show);
    }

    #[test]
    fn test_style_id() {
        let mut pb = make_progress_bar();
        pb.param_update(ProgressBarParam::StyleId, &py_obj(5usize));
        assert_eq!(pb.style_id, Some(5));
        pb.param_update(ProgressBarParam::StyleId, &py_none());
        assert_eq!(pb.style_id, None);
    }

    #[test]
    fn test_value() {
        let mut pb = make_progress_bar();
        pb.param_update(ProgressBarParam::Value, &py_obj(75.0f32));
        assert_eq!(pb.value, 75.0);
    }

    #[test]
    fn test_width() {
        let mut pb = make_progress_bar();
        pb.param_update(ProgressBarParam::Width, &py_obj(300.0f32));
        assert_eq!(pb.width, Length::Fixed(300.0));
    }

    #[test]
    fn test_width_fill() {
        let mut pb = make_progress_bar();
        pb.param_update(ProgressBarParam::WidthFill, &py_obj(true));
        assert_eq!(pb.width, Length::Fill);
    }

    // -- ProgressBarStyle param tests --

    #[test]
    fn test_style_background_rgba() {
        let mut s = make_progress_bar_style();
        s.param_update(ProgressBarStyleParam::BackgroundRgbaColor, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.background_color.is_some());
    }

    #[test]
    fn test_style_bar_rgba() {
        let mut s = make_progress_bar_style();
        s.param_update(ProgressBarStyleParam::BarRgbaColor, &py_obj(vec![0.0f32, 1.0, 0.0, 1.0]));
        assert!(s.bar_color.is_some());
    }

    #[test]
    fn test_style_border_rgba() {
        let mut s = make_progress_bar_style();
        s.param_update(ProgressBarStyleParam::BorderRgbaColor, &py_obj(vec![0.0f32, 0.0, 1.0, 1.0]));
        assert!(s.border_color.is_some());
    }

    #[test]
    fn test_style_border_radius() {
        let mut s = make_progress_bar_style();
        s.param_update(ProgressBarStyleParam::BorderRadius, &py_obj(vec![4.0f32, 4.0, 4.0, 4.0]));
        assert_eq!(s.border_radius, Some(vec![4.0, 4.0, 4.0, 4.0]));
        s.param_update(ProgressBarStyleParam::BorderRadius, &py_none());
        assert_eq!(s.border_radius, None);
    }

    #[test]
    fn test_style_border_width() {
        let mut s = make_progress_bar_style();
        s.param_update(ProgressBarStyleParam::BorderWidth, &py_obj(2.0f32));
        assert_eq!(s.border_width, Some(2.0));
        s.param_update(ProgressBarStyleParam::BorderWidth, &py_none());
        assert_eq!(s.border_width, None);
    }
}
