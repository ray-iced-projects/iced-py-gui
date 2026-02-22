//! ipg_opaque
use iced::mouse::Interaction;
use iced::{Color, Element, Length};
use iced::widget::{Container, Space, mouse_area, opaque};
use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;


use crate::state::IpgWidgets;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_opt_iced_color, set_opt_bool, set_iced_color_from_rgba};
use crate::{access_callbacks, access_user_data1, IpgState};
use crate::app::Message;
use super::ipg_container::{self, get_cont_style};
use super::enums::{IpgHorizontalAlignment, IpgVerticalAlignment};


#[derive(Debug, Clone)]
pub struct IpgOpaque {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub center: Option<bool>,
    pub align_x: Option<IpgHorizontalAlignment>,
    pub align_y: Option<IpgVerticalAlignment>,
    pub include_mouse_area: bool,
    pub show: bool,
    pub style_id: Option<usize>, 
}

#[derive(Debug, Clone, Default)]
pub struct IpgOpaqueStyle {
    pub id: usize,
    pub background_color: Option<Color>,
}

pub fn construct_opaque<'a>(op: &'a IpgOpaque, 
                        mut content: Vec<Element<'a, Message>>, 
                        style_opt: Option<&'a IpgWidgets> ) 
                        -> Element<'a, Message> {

    if !op.show {return Space::new().into()}

    let new_content = content.remove(0);
    
    let mut align_h = if let Some(ha) = &op.align_x{
        IpgHorizontalAlignment::to_iced(ha)
    } else { iced::alignment::Horizontal::Left };
    
    let mut align_v = if let Some(va) = &op.align_y{
        IpgVerticalAlignment::to_iced(va)
    } else { iced::alignment::Vertical::Top};

    if let Some(cnt) = op.center {
        if cnt {
            align_h = iced::alignment::Horizontal::Center;
            align_v = iced::alignment::Vertical::Center;
        }
    }
    
    let style = get_cont_style(style_opt);

    let cont: Element<'a, Message> = Container::new(new_content)
                .width(op.width)
                .height(op.height)
                .align_x(align_h)
                .align_y(align_v)
                .style(move|theme|
                    ipg_container::get_styling(theme, 
                        style.clone(),
                        ))
                .into();
    
    if op.include_mouse_area {
        opaque(mouse_area(cont)
            .on_press(Message::OpaqueOnPress(op.id))
            .interaction(Interaction::Pointer))
    } else {
        opaque(cont)
    }

    
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgOpaqueParam {
    Show,
}

pub fn opaque_callback(_state: &mut IpgState, id: usize, event_name: String) {
    
    process_callback(id, event_name);
}


fn process_callback(id: usize, event_name: String) 
{
    let ud = access_user_data1();
    let user_data_opt = ud.user_data.get(&id);

    let app_cbs = access_callbacks();

    let callback_present = 
        app_cbs.callbacks.get(&(id, event_name));
    
    let callback = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let cb = 
        Python::attach(|py| {
            callback.clone_ref(py)
        });

    drop(app_cbs);
              
    Python::attach(|py| {
        if user_data_opt.is_some() {
            let res = cb.call1(py, (
                                                        id,
                                                        user_data_opt.unwrap()  
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Opaque: Only 2 parameter (id, user_data) 
                                    is required or a python error in this function. {er}"),
            }
        } else {
            let res = cb.call1(py, (
                                                        id,  
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Opaque: Only 1 parameter (id) 
                                    is required or a python error in this function. {er}"),
            }
        }
        
        
    });
    
    drop(ud);   

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgOpaqueStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgOpaque {
    type Param = IpgOpaqueParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgOpaqueParam::Show => set_opt_bool(&mut self.center, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgOpaqueStyle {
    type Param = IpgOpaqueStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgOpaqueStyleParam::BackgroundIpgColor => set_opt_iced_color(&mut self.background_color, value, name),
            IpgOpaqueStyleParam::BackgroundRgbaColor => set_iced_color_from_rgba(&mut self.background_color, value, name),
        }
    }
}
