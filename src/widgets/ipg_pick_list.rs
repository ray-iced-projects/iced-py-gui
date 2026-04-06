//! ipg_pick_list
use crate::app;
use crate::IpgState;
use crate::graphics::bootstrap_arrow::Arrow;
use crate::py_api::helpers::{get_padding, get_radius};
use crate::state::IpgWidgets;
use crate::widgets::widget_param_update::set_opt_iced_color_from_rgba;
use crate::widgets::widget_param_update::set_opt_ipg_arrow;
use crate::widgets::widget_param_update::{WidgetParamUpdate,
    set_bool, set_height, set_height_fill,set_opt_iced_color,
    set_opt_f32, set_opt_string, set_opt_text_shaping,
    set_opt_usize, set_opt_vec_f32, set_vec_string,
    set_width};
use crate::widgets::ipg_text::TextShaping;
use super::callbacks::invoke_callback_with_args;

use iced::widget::pick_list::{self, Status};
use iced::{Color, Font, Pixels, Theme};
use iced::{Length, Element};
use iced::widget::PickList;
use iced::widget::pick_list::{Handle, Icon};

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgPickList {
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
    pub text_shaping: Option<TextShaping>,
    pub handle: Option<IpgPickListHandle>,
    pub arrow_size: Option<f32>,
    pub dynamic_closed: Option<Arrow>,
    pub dynamic_open: Option<Arrow>,
    pub custom_static: Option<Arrow>,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct IpgPickListStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub text_color: Option<Color>,
    pub handle_color: Option<Color>,
    pub placeholder_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_color_hovered: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum PLMessage {
    OnSelect(String),
}


pub fn construct_picklist<'a>(
    pick: &'a IpgPickList, 
    style_opt: Option<&IpgWidgets>) 
    -> Option<Element<'a, app::Message>> {
    
    if!pick.show {
        return None
    }
    let style = style_opt.and_then(IpgWidgets::as_pick_list_style).cloned();
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
        PickList::new(pick.options.clone(), 
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

    let pl: Element<'_, PLMessage> = if let Some(sh) = &pick.text_shaping {
        pl.text_shaping(sh.to_iced())
    } else { pl }.into();

    Some(pl.map(move |message| app::Message::PickList(pick.id, message)))

}
 

 pub fn pick_list_callback(state: &mut IpgState, id: usize, message: PLMessage) {
    match message {
        PLMessage::OnSelect(selected) => {
            // Update widget state directly
            if let Some(IpgWidgets::IpgPickList(pl)) = state.widgets.get_mut(&id) {
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


 #[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgPickListParam {
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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgPickListHandle {
    Default,
    Arrow,
    Dynamic,
    None,
    Static,
}

impl IpgPickListHandle {
    fn extract(value: &PyObject) -> Option<Self> {
        Some(Python::attach(|py| {

            let res = value.extract::<Self>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("Unable to extract python object for IpgPickListHandle"),
            }
        }))
    }
}

fn get_handle(ipg_handle: &IpgPickListHandle, 
                arrow_size: Option<f32>,
                closed: &Option<Arrow>,
                opened: &Option<Arrow>,
                custom: &Option<Arrow>,
            ) -> Handle<Font> 
{
    match ipg_handle {
        IpgPickListHandle::Default => Handle::default(),
        IpgPickListHandle::Arrow => {
            match arrow_size {
                Some(ars) => Handle::Arrow { size: Some(Pixels(ars)) },
                None => Handle::Arrow { size: None },
            }
        },
        IpgPickListHandle::Dynamic => {
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
        IpgPickListHandle::None => Handle::None,
        IpgPickListHandle::Static => {
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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgPickListStyleParam {
    BackgroundIpgColor,
    BackgroundRbgaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    HandleIpgColor,
    HandleRgbaColor,
    PlaceholderIpgColor,
    PlaceholderRgbaColor,
    TextIpgColor,
    TextRgbaColor,
}

pub fn get_styling(theme: &Theme, status: Status, 
                    style_opt: Option<IpgPickListStyle>, 
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

impl WidgetParamUpdate for IpgPickList {
    type Param = IpgPickListParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgPickListParam::ArrowSize => set_opt_f32(&mut self.arrow_size, value, "ArrowSize"),
            IpgPickListParam::CustomStatic => set_opt_ipg_arrow(&mut self.custom_static, value, "CustomStatic"),
            IpgPickListParam::DynamicClosed => set_opt_ipg_arrow(&mut self.dynamic_closed, value, "DynamicClosed"),
            IpgPickListParam::DynamicOpen => set_opt_ipg_arrow(&mut self.dynamic_open, value, "DynamicOpen"),
            IpgPickListParam::Handle => self.handle = IpgPickListHandle::extract(value),
            IpgPickListParam::MenuHeight => set_height(&mut self.menu_height, value, "MenuHeight"),
            IpgPickListParam::MenuHeightFill => set_height_fill(&mut self.menu_height, value, "MenuHeightFill"),
            IpgPickListParam::Options => set_vec_string(&mut self.options, value, "Options"),
            IpgPickListParam::Padding => set_opt_vec_f32(&mut self.padding, value, "Padding"),
            IpgPickListParam::Placeholder => set_opt_string(&mut self.placeholder, value, "Placeholder"),
            IpgPickListParam::Selected => set_opt_string(&mut self.selected, value, "Selected"),
            IpgPickListParam::Show => set_bool(&mut self.show, value, "Show"),
            IpgPickListParam::StyleId => set_opt_usize(&mut self.style_id, value, "StyleId"),
            IpgPickListParam::TextLineHeight => set_opt_f32(&mut self.text_line_height,value, "TextLineHeight"),
            IpgPickListParam::TextShaping => set_opt_text_shaping(&mut self.text_shaping, value, "TextShaping"),
            IpgPickListParam::TextSize => set_opt_f32(&mut self.text_size, value, "TextSize"),
            IpgPickListParam::Width => set_width(&mut self.width, value, "Width"),
        }
    }
}

impl WidgetParamUpdate for IpgPickListStyle {
    type Param = IpgPickListStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgPickListStyleParam::BackgroundIpgColor => 
                set_opt_iced_color(&mut self.background_color, value, "BackgroundIpgColor"),
            IpgPickListStyleParam::BackgroundRbgaColor => 
                set_opt_iced_color_from_rgba(&mut self.background_color, value, "BackgroundRbgaColor"),
            IpgPickListStyleParam::BorderIpgColor => 
                set_opt_iced_color(&mut self.border_color, value, "BorderIpgColor"),
            IpgPickListStyleParam::BorderRadius => 
                set_opt_vec_f32(&mut self.border_radius, value, "BorderRadius"),
            IpgPickListStyleParam::BorderRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.border_color, value, "BorderRgbaColor"),
            IpgPickListStyleParam::BorderWidth => 
                set_opt_f32(&mut self.border_width, value, "BorderWidth"),
            IpgPickListStyleParam::HandleIpgColor => 
                set_opt_iced_color(&mut self.handle_color, value, "HandleIpgColor"),
            IpgPickListStyleParam::HandleRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.handle_color, value, "HandleRgbaColor"),
            IpgPickListStyleParam::PlaceholderIpgColor => 
                set_opt_iced_color(&mut self.placeholder_color, value, "PlaceholderIpgColor"),
            IpgPickListStyleParam::PlaceholderRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.placeholder_color, value, "PlaceholderRgbaColor"),
            IpgPickListStyleParam::TextIpgColor => 
                set_opt_iced_color(&mut self.text_color, value, "TextIpgColor"),
            IpgPickListStyleParam::TextRgbaColor => 
                set_opt_iced_color_from_rgba(&mut self.text_color, value, "TextRgbaColor"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Length;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_picklist() -> IpgPickList {
        IpgPickList {
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
            text_shaping: None,
            handle: None,
            arrow_size: None,
            dynamic_closed: None,
            dynamic_open: None,
            custom_static: None,
            style_id: None,
        }
    }

    fn make_picklist_style() -> IpgPickListStyle {
        IpgPickListStyle::default()
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::attach(|py| py.None().into_py_any(py).unwrap())
    }

    // -- IpgPickList param tests --

    #[test]
    fn test_arrow_size() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::ArrowSize, &py_obj(16.0f32));
        assert_eq!(pl.arrow_size, Some(16.0));
        pl.param_update(IpgPickListParam::ArrowSize, &py_none());
        assert_eq!(pl.arrow_size, None);
    }

    #[test]
    fn test_options() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::Options, &py_obj(vec!["x".to_string(), "y".to_string()]));
        assert_eq!(pl.options, vec!["x", "y"]);
    }

    #[test]
    fn test_menu_height() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::MenuHeight, &py_obj(200.0f32));
        assert_eq!(pl.menu_height, Length::Fixed(200.0));
    }

    #[test]
    fn test_menu_height_fill() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::MenuHeightFill, &py_obj(true));
        assert_eq!(pl.menu_height, Length::Fill);
    }

    #[test]
    fn test_padding() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::Padding, &py_obj(vec![5.0f32, 10.0]));
        assert_eq!(pl.padding, Some(vec![5.0, 10.0]));
        pl.param_update(IpgPickListParam::Padding, &py_none());
        assert_eq!(pl.padding, None);
    }

    #[test]
    fn test_placeholder() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::Placeholder, &py_obj("Choose...".to_string()));
        assert_eq!(pl.placeholder, Some("Choose...".to_string()));
        pl.param_update(IpgPickListParam::Placeholder, &py_none());
        assert_eq!(pl.placeholder, None);
    }

    #[test]
    fn test_selected() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::Selected, &py_obj("a".to_string()));
        assert_eq!(pl.selected, Some("a".to_string()));
        pl.param_update(IpgPickListParam::Selected, &py_none());
        assert_eq!(pl.selected, None);
    }

    #[test]
    fn test_show() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::Show, &py_obj(false));
        assert!(!pl.show);
    }

    #[test]
    fn test_style_id() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::StyleId, &py_obj(42usize));
        assert_eq!(pl.style_id, Some(42));
        pl.param_update(IpgPickListParam::StyleId, &py_none());
        assert_eq!(pl.style_id, None);
    }

    #[test]
    fn test_text_line_height() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::TextLineHeight, &py_obj(1.5f32));
        assert_eq!(pl.text_line_height, Some(1.5));
        pl.param_update(IpgPickListParam::TextLineHeight, &py_none());
        assert_eq!(pl.text_line_height, None);
    }

    #[test]
    fn test_text_size() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::TextSize, &py_obj(18.0f32));
        assert_eq!(pl.text_size, Some(18.0));
        pl.param_update(IpgPickListParam::TextSize, &py_none());
        assert_eq!(pl.text_size, None);
    }

    #[test]
    fn test_width() {
        let mut pl = make_picklist();
        pl.param_update(IpgPickListParam::Width, &py_obj(250.0f32));
        assert_eq!(pl.width, Length::Fixed(250.0));
    }

    // -- IpgPickListStyle param tests --

    #[test]
    fn test_style_background_rgba() {
        let mut s = make_picklist_style();
        s.param_update(IpgPickListStyleParam::BackgroundRbgaColor, &py_obj(vec![1.0f32, 0.0, 0.0, 1.0]));
        assert!(s.background_color.is_some());
    }

    #[test]
    fn test_style_border_rgba() {
        let mut s = make_picklist_style();
        s.param_update(IpgPickListStyleParam::BorderRgbaColor, &py_obj(vec![0.0f32, 1.0, 0.0, 1.0]));
        assert!(s.border_color.is_some());
    }

    #[test]
    fn test_style_border_radius() {
        let mut s = make_picklist_style();
        s.param_update(IpgPickListStyleParam::BorderRadius, &py_obj(vec![4.0f32, 4.0, 4.0, 4.0]));
        assert_eq!(s.border_radius, Some(vec![4.0, 4.0, 4.0, 4.0]));
        s.param_update(IpgPickListStyleParam::BorderRadius, &py_none());
        assert_eq!(s.border_radius, None);
    }

    #[test]
    fn test_style_border_width() {
        let mut s = make_picklist_style();
        s.param_update(IpgPickListStyleParam::BorderWidth, &py_obj(2.0f32));
        assert_eq!(s.border_width, Some(2.0));
        s.param_update(IpgPickListStyleParam::BorderWidth, &py_none());
        assert_eq!(s.border_width, None);
    }

    #[test]
    fn test_style_handle_rgba() {
        let mut s = make_picklist_style();
        s.param_update(IpgPickListStyleParam::HandleRgbaColor, &py_obj(vec![0.5f32, 0.5, 0.5, 1.0]));
        assert!(s.handle_color.is_some());
    }

    #[test]
    fn test_style_placeholder_rgba() {
        let mut s = make_picklist_style();
        s.param_update(IpgPickListStyleParam::PlaceholderRgbaColor, &py_obj(vec![0.7f32, 0.7, 0.7, 1.0]));
        assert!(s.placeholder_color.is_some());
    }

    #[test]
    fn test_style_text_rgba() {
        let mut s = make_picklist_style();
        s.param_update(IpgPickListStyleParam::TextRgbaColor, &py_obj(vec![0.0f32, 0.0, 0.0, 1.0]));
        assert!(s.text_color.is_some());
    }
}
