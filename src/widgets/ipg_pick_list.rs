//! ipg_pick_list
use crate::app;
use crate::IpgState;
use crate::graphics::bootstrap_arrow::Arrow;
use crate::py_api::helpers::{get_padding, get_radius};
use crate::state::Widgets;
use crate::widgets::widget_param_update::set_opt_iced_color_from_rgba;
use crate::widgets::widget_param_update::set_opt_ipg_arrow;
use crate::widgets::widget_param_update::{WidgetParamUpdate,
    set_bool, set_height, set_height_fill,set_opt_iced_color,
    set_opt_f32, set_opt_string,
    set_opt_usize, set_opt_vec_f32, set_vec_string,
    set_width};
use super::callbacks::invoke_callback_with_args;

use iced::widget::pick_list::{self, Status};
use iced::widget::text::Shaping;
use iced::{Font, Pixels, Theme};
use iced::{Length, Element};
use iced::widget;
use iced::widget::pick_list::{Handle, Icon};

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct PickList {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub options: Vec<String>,
    pub placeholder: Option<String>,
    pub selected: Option<String>,
    pub width: Length,
    pub menu_height: Length,
    pub padding: Option<Vec<f32>>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub text_shaping_advanced: Option<bool>,
    pub text_shaping_basic: Option<bool>,
    pub handle: Option<PickListHandle>,
    pub arrow_size: Option<f32>,
    pub dynamic_closed: Option<Arrow>,
    pub dynamic_open: Option<Arrow>,
    pub custom_static: Option<Arrow>,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct PickListStyle {
    pub id: usize,
    pub background_color: Option<iced::Color>,
    pub text_color: Option<iced::Color>,
    pub handle_color: Option<iced::Color>,
    pub placeholder_color: Option<iced::Color>,
    pub border_color: Option<iced::Color>,
    pub border_color_hovered: Option<iced::Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum PLMessage {
    OnSelect(String),
}


pub fn construct_picklist<'a>(
    pick: &'a PickList, 
    style_opt: Option<&Widgets>) 
    -> Option<Element<'a, app::Message>> {
    
    if!pick.show {
        return None
    }
    let style = style_opt.and_then(Widgets::as_pick_list_style).cloned();
    let placeholder = pick.placeholder.clone().unwrap_or("".to_string());

    let handle = if let Some(hd) = &pick.handle {
        get_handle(
            &hd, 
            pick.arrow_size, 
            &pick.dynamic_closed,
            &pick.dynamic_open,
            &pick.custom_static)
    } else { Handle::None };

    let pl = 
        widget::PickList::new(pick.options.clone(), 
            pick.selected.clone(), 
            PLMessage::OnSelect,
        )
        .placeholder(placeholder)
        .width(pick.width)
        .menu_height(pick.menu_height)
        .padding(get_padding(&pick.padding))
        .handle(handle)
        .style(move|theme: &Theme, status| {   
            get_styling(theme, status, 
                style.clone(),
            )  
            });

    let pl = if let Some(ts) = pick.text_size {
        pl.text_size(ts)
    } else { pl };

    let pl = if let Some(lh) = pick.text_line_height {
        pl.text_line_height(lh)
    } else { pl };

    // default is auto so not checked
    let pl = 
        if pick.text_shaping_advanced.is_some() {
            pl.text_shaping(Shaping::Advanced)
        } else if pick.text_shaping_basic.is_some() {
            pl.text_shaping(Shaping::Basic)
        } else { pl };

    
    let pl: Element<'_, PLMessage> = pl.into();
    Some(pl.map(move |message| app::Message::PickList(pick.id, message)))

}
 

 pub fn pick_list_callback(state: &mut IpgState, id: usize, message: PLMessage) {
    match message {
        PLMessage::OnSelect(selected) => {
            // Update widget state directly
            if let Some(Widgets::PickList(pl)) = state.widgets.get_mut(&id) {
                pl.selected = Some(selected.clone());
            }
            invoke_callback_with_args(id, "on_select", "PickList", selected);
        },
    }
 }


 pub fn convert_pyobject_vec_string(options: PyObject) -> Vec<String> {

    let items: Vec<String> = vec![];

    Python::attach(|py| {

        let res = options.extract::<Vec<bool>>(py);
        if res.is_ok() {
            return match res {
                Ok(res) => {
                    res.iter().map(|v| {
                        if *v {
                            "True".to_string()
                        } else {
                           "False".to_string()
                        }
                    }).collect()
                },
                Err(_) => panic!("Picklist could not extract List[bool]"),
            }
        }

        let res = options.extract::<Vec<String>>(py);
        if res.is_ok() {
            return match res {
                Ok(res) => res,
                Err(_) => panic!("Picklist could not extract List[String]"),
            } 
        }

        let res = options.extract::<Vec<i64>>(py);
        if res.is_ok() { 
            return match res {
                Ok(res) => res.iter().map(|v| v.to_string()).collect(),
                Err(_) => panic!("Picklist could not extract List[int]"),
            } 
        } 
        
        let res = options.extract::<Vec<f64>>(py);
        if res.is_ok() { 
            return match res {
                Ok(res) => res.iter().map(|v| v.to_string()).collect(),
                Err(_) => panic!("Picklist could not extract List[float]"),
            } 
        }
        items
    })

 }


 #[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum PickListParam {
    ArrowSize,
    CustomStatic,
    DynamicClosed,
    DynamicOpen,
    Handle,
    Options,
    MenuHeight,
    MenuHeightFill,
    Padding,
    Placeholder,
    Selected,
    Show,
    StyleId,
    TextLineHeight,
    TextShaping,
    TextSize,
    Width,
    // WidthFill,  see comment below
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum PickListHandle {
    Default,
    Arrow,
    Dynamic,
    None,
    Static,
}

impl PickListHandle {
    fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("Unable to extract python object for PickListHandle"),
            }
        }))
    }
}

fn get_handle(ipg_handle: &PickListHandle, 
                arrow_size: Option<f32>,
                closed: &Option<Arrow>,
                opened: &Option<Arrow>,
                custom: &Option<Arrow>,
            ) -> Handle<Font> 
{
    match ipg_handle {
        PickListHandle::Default => Handle::default(),
        PickListHandle::Arrow => {
            match arrow_size {
                Some(ars) => Handle::Arrow { size: Some(Pixels(ars)) },
                None => Handle::Arrow { size: None },
            }
        },
        PickListHandle::Dynamic => {
            let arrow_closed = match closed {
                Some(cls) => Arrow::to_char(cls),
                None => Arrow::to_char(&Arrow::ArrowBarRight),
            };

            let arrow_opened = match opened {
                Some(op) => Arrow::to_char(op),
                None => Arrow::to_char(&Arrow::ArrowBarRight),
            };

            let size = arrow_size.map(Pixels);

            Handle::Dynamic { 
                closed: Icon { 
                    code_point: arrow_closed, 
                    font: iced::Font::with_name("bootstrap-icons"), 
                    size, 
                    line_height: Default::default(), 
                    shaping: Default::default()
                }, 
                open: Icon {
                    code_point: arrow_opened,
                    font: iced::Font::with_name("bootstrap-icons"), 
                    size, 
                    line_height: Default::default(), 
                    shaping: Default::default()} 
                }
        },
        PickListHandle::None => Handle::None,
        PickListHandle::Static => {
                let custom_type = match custom {
                    Some(cust) => Arrow::to_char(cust),
                    None => Arrow::to_char(&Arrow::ArrowBarRight),
                };

                let size = arrow_size.map(Pixels);

                Handle::Static(Icon { 
                    code_point: custom_type, 
                    font: iced::Font::with_name("bootstrap-icons"), 
                    size, 
                    line_height: Default::default(), 
                    shaping: Default::default()
                }
            )
        },
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum PickListStyleParam {
    BackgroundColor,
    BackgroundRbgaColor,
    BorderColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    HandleColor,
    HandleRgbaColor,
    PlaceholderColor,
    PlaceholderRgbaColor,
    TextColor,
    TextRgbaColor,
}

pub fn get_styling(theme: &Theme, status: Status, 
                    style_opt: Option<PickListStyle>, 
                    ) -> pick_list::Style {
    
    let mut active_style = pick_list::default(theme, Status::Active);

    if style_opt.is_none() {
        return pick_list::default(theme, status)
    }

    let style = style_opt.unwrap();

    if let Some(bc) = style.background_color {
        active_style.background = bc.into();
    }
    
    if let Some(hc) = style.handle_color {
        active_style.handle_color = hc;
    }

    if let Some(pc) = style.placeholder_color {
        active_style.placeholder_color = pc;
    }

    if let Some(tc) = style.text_color {
        active_style.text_color = tc;
    }

    if let Some(br) = style.border_radius {
     active_style.border.radius = 
        get_radius(&br, "PickList".to_string());
    }

    if let Some(bw) = style.border_width {
        active_style.border.width = bw;
    };

    
    if let Some(bc) = style.border_color && status == Status::Active {
        active_style.border.color = bc;
    }

    let mut hover_opened_style = active_style;
    
    if let Some(bch) = style.border_color_hovered {
        hover_opened_style.border.color = bch;
    }
    
    match status {
        Status::Active => active_style,
        Status::Hovered | Status::Opened { .. } => hover_opened_style,
    }

}



// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for PickList {
    type Param = PickListParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            PickListParam::ArrowSize => set_opt_f32(&mut self.arrow_size, value, "ArrowSize"),
            PickListParam::CustomStatic => set_opt_ipg_arrow(&mut self.custom_static, value, "CustomStatic"),
            PickListParam::DynamicClosed => set_opt_ipg_arrow(&mut self.dynamic_closed, value, "DynamicClosed"),
            PickListParam::DynamicOpen => set_opt_ipg_arrow(&mut self.dynamic_open, value, "DynamicOpen"),
            PickListParam::Handle => self.handle = PickListHandle::extract(value),
            PickListParam::MenuHeight => set_height(&mut self.menu_height, value, "MenuHeight"),
            PickListParam::MenuHeightFill => set_height_fill(&mut self.menu_height, value, "MenuHeightFill"),
            PickListParam::Options => set_vec_string(&mut self.options, value, "Options"),
            PickListParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            PickListParam::Placeholder => set_opt_string(&mut self.placeholder, value, "Placeholder"),
            PickListParam::Selected => set_opt_string(&mut self.selected, value, "Selected"),
            PickListParam::Show => set_bool(&mut self.show, value, "Show"),
            PickListParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            PickListParam::TextLineHeight => set_opt_f32(&mut self.text_line_height,value, "TextLineHeight"),
            PickListParam::TextSize => set_opt_f32(&mut self.text_size, value, "TextSize"),
            PickListParam::Width => set_width(&mut self.width, value, "Width"),
            PickListParam::TextShaping => todo!(),
        }
    }
}

impl WidgetParamUpdate for PickListStyle {
    type Param = PickListStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            PickListStyleParam::BackgroundColor => 
                set_opt_iced_color(&mut self.background_color, value, "BackgroundColor"),
            PickListStyleParam::BackgroundRbgaColor => 
                set_opt_iced_color_from_rgba(&mut self.background_color, value, "BackgroundRbgaColor"),
            PickListStyleParam::BorderColor => 
                set_opt_iced_color(&mut self.border_color, value, "BorderColor"),
            PickListStyleParam::BorderRadius => 
                set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
            PickListStyleParam::BorderRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgbaColor"),
            PickListStyleParam::BorderWidth => 
                set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            PickListStyleParam::HandleColor => 
                set_opt_iced_color(&mut self.handle_color, value, "HandleColor"),
            PickListStyleParam::HandleRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.handle_color, value, "HandleRgbaColor"),
            PickListStyleParam::PlaceholderColor => 
                set_opt_iced_color(&mut self.placeholder_color, value, "PlaceholderColor"),
            PickListStyleParam::PlaceholderRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.placeholder_color, value, "PlaceholderRgbaColor"),
            PickListStyleParam::TextColor => 
                set_opt_iced_color(&mut self.text_color, value, "TextColor"),
            PickListStyleParam::TextRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.text_color, value, "TextRgbaColor"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_picklist() -> PickList {
        PickList {
            id: 0,
            parent_id: String::new(),
            show: true,
            options: vec!["a".into(), "b".into()],
            placeholder: None,
            selected: None,
            width: Length::Shrink,
            menu_height: Length::Shrink,
            padding: None,
            text_size: None,
            text_line_height: None,
            text_shaping_advanced: None,
            text_shaping_basic: None,
            handle: None,
            arrow_size: None,
            dynamic_closed: None,
            dynamic_open: None,
            custom_static: None,
            style_id: None,
        }
    }

    fn make_picklist_style() -> PickListStyle {
        PickListStyle::default()
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::attach(|py| py.None().into_py_any(py).unwrap())
    }

    // -- PickList param tests --

    #[test]
    fn test_arrow_size() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::ArrowSize, &py_obj(16.0f32));
        assert_eq!(pl.arrow_size, Some(16.0));
        pl.param_update(PickListParam::ArrowSize, &py_none());
        assert_eq!(pl.arrow_size, None);
    }

    #[test]
    fn test_options() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::Options, &py_obj(vec!["x".to_string(), "y".to_string()]));
        assert_eq!(pl.options, vec!["x", "y"]);
    }

    #[test]
    fn test_menu_height() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::MenuHeight, &py_obj(200.0f32));
        assert_eq!(pl.menu_height, Length::Fixed(200.0));
    }

    #[test]
    fn test_menu_height_fill() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::MenuHeightFill, &py_obj(true));
        assert_eq!(pl.menu_height, Length::Fill);
    }

    #[test]
    fn test_padding() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::Padding, &py_obj(vec![5.0f32, 10.0]));
        assert_eq!(pl.padding, Some(vec![5.0, 10.0]));
        pl.param_update(PickListParam::Padding, &py_none());
        assert_eq!(pl.padding, None);
    }

    #[test]
    fn test_placeholder() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::Placeholder, &py_obj("Choose...".to_string()));
        assert_eq!(pl.placeholder, Some("Choose...".to_string()));
        pl.param_update(PickListParam::Placeholder, &py_none());
        assert_eq!(pl.placeholder, None);
    }

    #[test]
    fn test_selected() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::Selected, &py_obj("a".to_string()));
        assert_eq!(pl.selected, Some("a".to_string()));
        pl.param_update(PickListParam::Selected, &py_none());
        assert_eq!(pl.selected, None);
    }

    #[test]
    fn test_show() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::Show, &py_obj(false));
        assert!(!pl.show);
    }

    #[test]
    fn test_style_id() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::StyleId, &py_obj(42usize));
        assert_eq!(pl.style_id, Some(42));
        pl.param_update(PickListParam::StyleId, &py_none());
        assert_eq!(pl.style_id, None);
    }

    #[test]
    fn test_text_line_height() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::TextLineHeight, &py_obj(1.5f32));
        assert_eq!(pl.text_line_height, Some(1.5));
        pl.param_update(PickListParam::TextLineHeight, &py_none());
        assert_eq!(pl.text_line_height, None);
    }

    #[test]
    fn test_text_size() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::TextSize, &py_obj(18.0f32));
        assert_eq!(pl.text_size, Some(18.0));
        pl.param_update(PickListParam::TextSize, &py_none());
        assert_eq!(pl.text_size, None);
    }

    #[test]
    fn test_width() {
        let mut pl = make_picklist();
        pl.param_update(PickListParam::Width, &py_obj(250.0f32));
        assert_eq!(pl.width, Length::Fixed(250.0));
    }

    // -- PickListStyle param tests --

    #[test]
    fn test_style_background_rgba() {
        let mut s = make_picklist_style();
        s.param_update(PickListStyleParam::BackgroundRbgaColor, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.background_color.is_some());
    }

    #[test]
    fn test_style_border_rgba() {
        let mut s = make_picklist_style();
        s.param_update(PickListStyleParam::BorderRgbaColor, &py_obj(vec![0.0f32, 1.0, 0.0, 1.0]));
        assert!(s.border_color.is_some());
    }

    #[test]
    fn test_style_border_radius() {
        let mut s = make_picklist_style();
        s.param_update(PickListStyleParam::BorderRadius, &py_obj(vec![4.0f32, 4.0, 4.0, 4.0]));
        assert_eq!(s.border_radius, Some(vec![4.0, 4.0, 4.0, 4.0]));
        s.param_update(PickListStyleParam::BorderRadius, &py_none());
        assert_eq!(s.border_radius, None);
    }

    #[test]
    fn test_style_border_width() {
        let mut s = make_picklist_style();
        s.param_update(PickListStyleParam::BorderWidth, &py_obj(2.0f32));
        assert_eq!(s.border_width, Some(2.0));
        s.param_update(PickListStyleParam::BorderWidth, &py_none());
        assert_eq!(s.border_width, None);
    }

    #[test]
    fn test_style_handle_rgba() {
        let mut s = make_picklist_style();
        s.param_update(PickListStyleParam::HandleRgbaColor, &py_obj(vec![0.5f32, 0.5, 0.5, 1.0]));
        assert!(s.handle_color.is_some());
    }

    #[test]
    fn test_style_placeholder_rgba() {
        let mut s = make_picklist_style();
        s.param_update(PickListStyleParam::PlaceholderRgbaColor, &py_obj(vec![0.7f32, 0.7, 0.7, 1.0]));
        assert!(s.placeholder_color.is_some());
    }

    #[test]
    fn test_style_text_rgba() {
        let mut s = make_picklist_style();
        s.param_update(PickListStyleParam::TextRgbaColor, &py_obj(vec![0.0f32, 0.0, 0.0, 1.0]));
        assert!(s.text_color.is_some());
    }
}
