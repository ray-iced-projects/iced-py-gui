//! ipg_opaque
use iced::mouse::Interaction;
use iced::{Color, Element, Length};
use iced::widget::{horizontal_space, mouse_area, opaque, Container};

use ipg_styling::colors::get_color;
use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use ipg_alignment::{get_horizontal_alignment, get_vertical_alignment, IpgHorizontalAlignment, IpgVerticalAlignment};
use ipg_helpers::try_extract_boolean;
use ipg_types::Message;
use ipg_styling::{try_extract_ipg_color, try_extract_rgba_color};
use super::ipg_container::{self, get_cont_style};
use super::ipg_enums::IpgWidgets;


#[derive(Debug, Clone)]
pub struct IpgOpaque {
    pub id: usize,
    pub width: Length,
    pub height: Length,
    pub align_x: IpgHorizontalAlignment,
    pub align_y: IpgVerticalAlignment,
    pub include_mouse_area: bool,
    pub show: bool,
    pub style_id: Option<usize>, 
}

impl IpgOpaque {
    pub fn new(
        id: usize,
        width: Length,
        height: Length,
        align_x: IpgHorizontalAlignment,
        align_y: IpgVerticalAlignment,
        include_mouse_area: bool,
        show: bool,
        style_id: Option<usize>,
    ) -> Self {
        Self {
            id,
            width,
            height,
            align_x,
            align_y,
            include_mouse_area,
            show,
            style_id, 
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpgOpaqueStyle {
    pub id: usize,
    pub background_color: Option<Color>,
}

impl IpgOpaqueStyle {
    pub fn new(
        id: usize,
        background_color: Option<Color>,
    ) -> Self {
        Self {
            id,
            background_color,
        }
    }
}

pub fn construct_opaque<'a>(op: &'a IpgOpaque, 
                        mut content: Vec<Element<'a, Message>>, 
                        style_opt: Option<&'a IpgWidgets> ) 
                        -> Element<'a, Message> {

    if !op.show {return horizontal_space().into()}

    let new_content = content.remove(0);
    
    let align_h = get_horizontal_alignment(&op.align_x);
    let align_v = get_vertical_alignment(&op.align_y);
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

pub fn opaque_item_update(op: &mut IpgOpaque,
                            item: &PyObject,
                            value: &PyObject,) {

    let update = try_extract_stack_update(item);
    let name = "OpaqueContainer".to_string();
    match update {
        IpgOpaqueParam::Show => {
            op.show = try_extract_boolean(value, name);
        },
    }
}

pub fn try_extract_stack_update(update_obj: &PyObject) -> IpgOpaqueParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgOpaqueParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Opaque update extraction failed"),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgOpaqueStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
}

pub fn opaque_style_update_item(style: &mut IpgOpaqueStyle, 
                                item: &PyObject, 
                                value: &PyObject) 
{
    let update = try_extract_opaque_style_update(item);
    let name = "ContainerStyle".to_string();
    match update {
        IpgOpaqueStyleParam::BackgroundIpgColor => {
            let color = try_extract_ipg_color(value, name);
            style.background_color = get_color(None, Some(color), 1.0, false);
        },
        IpgOpaqueStyleParam::BackgroundRgbaColor => {
            style.background_color = Some(Color::from(try_extract_rgba_color(value, name)));
        },
    }
}

pub fn try_extract_opaque_style_update(update_obj: &PyObject) -> IpgOpaqueStyleParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgOpaqueStyleParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Opaque style parameter update extraction failed"),
        }
    })
}

