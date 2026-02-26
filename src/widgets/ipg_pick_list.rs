//! ipg_pick_list
use crate::access_callbacks;
use crate::access_user_data1;
use crate::app;
use crate::IpgState;
use crate::graphics::bootstrap_arrow::IpgArrow;
use crate::py_api::helpers::{get_padding, get_radius};
use crate::state::IpgWidgets;
use crate::widgets::enums::IpgShaping;
use crate::widgets::widget_param_update::set_opt_ipg_arrow;
use crate::widgets::widget_param_update::{WidgetParamUpdate,
    set_bool, set_height, set_height_fill,set_opt_iced_color,
    set_opt_f32, set_opt_string, set_opt_text_shaping,
    set_opt_usize, set_opt_vec_f32, set_vec_string,
    set_iced_color_from_rgba, set_width};
use super::callbacks::set_or_get_widget_callback_data;
use super::callbacks::WidgetCallbackIn;

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
    pub text_shaping: Option<IpgShaping>,
    pub handle: Option<IpgPickListHandle>,
    pub arrow_size: Option<f32>,
    pub dynamic_closed: Option<IpgArrow>,
    pub dynamic_open: Option<IpgArrow>,
    pub custom_static: Option<IpgArrow>,
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

    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};

    match message {
        PLMessage::OnSelect(selected) => {
            wci.value_str = Some(selected.clone());
            let _ = set_or_get_widget_callback_data(state, wci);
            
            process_callback(id, "on_select".to_string(), selected);
        },
    }
 }


 fn process_callback(
        id: usize, 
        event_name: String, 
        selected: String) 
 {
    let ud1 = access_user_data1();
    let app_cbs = access_callbacks();

    // Retrieve the callback
    let callback = match app_cbs.callbacks.get(&(id, event_name)) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_cbs);

    // Check user data from ud1
    if let Some(user_data) = ud1.user_data.get(&id) {
        Python::attach(|py| {
            if let Err(err) = callback.call1(py, (id, selected, user_data)) {
                panic!("PickList callback error: {err}");
            }
        });
        drop(ud1); // Drop ud1 before processing ud2
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Check user data from ud2
    // let ud2 = access_user_data2();
    // if let Some(user_data) = ud2.user_data.get(&id) {
    //     Python::attach(|py| {
    //         if let Err(err) = callback.call1(py, (id, selected, user_data)) {
    //             panic!("PickList callback error: {err}");
    //         }
    //     });
    //     drop(ud2); // Drop ud2 after processing
    //     return;
    // }
    // drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only the id and selected
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, selected)) {
            panic!("PickList callback error: {err}");
        }
    });

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
                closed: &Option<IpgArrow>,
                opened: &Option<IpgArrow>,
                custom: &Option<IpgArrow>,
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
                Some(cls) => IpgArrow::to_char(cls),
                None => IpgArrow::to_char(&IpgArrow::ArrowBarRight),
            };

            let arrow_opened = match opened {
                Some(op) => IpgArrow::to_char(op),
                None => IpgArrow::to_char(&IpgArrow::ArrowBarRight),
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
                    Some(cust) => IpgArrow::to_char(cust),
                    None => IpgArrow::to_char(&IpgArrow::ArrowBarRight),
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

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgPickListParam::ArrowSize => set_opt_f32(&mut self.arrow_size, value, name),
            IpgPickListParam::CustomStatic => set_opt_ipg_arrow(&mut self.custom_static, value, name),
            IpgPickListParam::DynamicClosed => set_opt_ipg_arrow(&mut self.dynamic_closed, value, name),
            IpgPickListParam::DynamicOpen => set_opt_ipg_arrow(&mut self.dynamic_open, value, name),
            IpgPickListParam::Handle => self.handle = IpgPickListHandle::extract(value),
            IpgPickListParam::MenuHeight => set_height(&mut self.menu_height, value, name),
            IpgPickListParam::MenuHeightFill => set_height_fill(&mut self.menu_height, value, name),
            IpgPickListParam::Options => set_vec_string(&mut self.options, value, name),
            IpgPickListParam::Padding => set_opt_vec_f32(&mut self.padding, value, name),
            IpgPickListParam::Placeholder => set_opt_string(&mut self.placeholder, value, name),
            IpgPickListParam::Selected => set_opt_string(&mut self.selected, value, name),
            IpgPickListParam::Show => set_bool(&mut self.show, value, name),
            IpgPickListParam::StyleId => set_opt_usize(&mut self.style_id, value, name),
            IpgPickListParam::TextLineHeight => set_opt_f32(&mut self.text_line_height,value, name),
            IpgPickListParam::TextShaping => set_opt_text_shaping(&mut self.text_shaping, value, name),
            IpgPickListParam::TextSize => set_opt_f32(&mut self.text_size, value, name),
            IpgPickListParam::Width => set_width(&mut self.width, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgPickListStyle {
    type Param = IpgPickListStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgPickListStyleParam::BackgroundIpgColor => 
                set_opt_iced_color(&mut self.background_color, value, name),
            IpgPickListStyleParam::BackgroundRbgaColor => 
                set_iced_color_from_rgba(&mut self.background_color, value, name),
            IpgPickListStyleParam::BorderIpgColor => 
                set_opt_iced_color(&mut self.border_color, value, name),
            IpgPickListStyleParam::BorderRadius => 
                set_opt_vec_f32(&mut self.border_radius, value, name),
            IpgPickListStyleParam::BorderRgbaColor => 
                set_iced_color_from_rgba(&mut self.border_color, value, name),
            IpgPickListStyleParam::BorderWidth => 
                set_opt_f32(&mut self.border_width, value, name),
            IpgPickListStyleParam::HandleIpgColor => 
                set_opt_iced_color(&mut self.handle_color, value, name),
            IpgPickListStyleParam::HandleRgbaColor => 
                set_iced_color_from_rgba(&mut self.handle_color, value, name),
            IpgPickListStyleParam::PlaceholderIpgColor => 
                set_opt_iced_color(&mut self.placeholder_color, value, name),
            IpgPickListStyleParam::PlaceholderRgbaColor => 
                set_iced_color_from_rgba(&mut self.placeholder_color, value, name),
            IpgPickListStyleParam::TextIpgColor => 
                set_opt_iced_color(&mut self.text_color, value, name),
            IpgPickListStyleParam::TextRgbaColor => 
                set_iced_color_from_rgba(&mut self.text_color, value, name),
        }
    }
}
