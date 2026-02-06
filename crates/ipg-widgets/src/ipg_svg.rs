//! ipg_svg
#![allow(clippy::enum_variant_names)]
use iced::{Length, Element, Radians, Rotation};
use iced::widget::{Svg, MouseArea};
use iced::mouse::Interaction;
use iced::advanced::svg;

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;

use ipg_helpers::{get_height, get_width, try_extract_boolean, 
    try_extract_f64, try_extract_string};
use ipg_types::{Message, SvgMessage};
use super::ipg_mousearea::{IpgMousePointer, get_interaction};


#[derive(Debug, Clone)]
pub struct IpgSvg {
        pub id: usize,
        pub parent_id: String,
        pub svg_path: String,
        pub width: Length,
        pub height: Length,
        pub content_fit: IpgSvgContentFit,
        pub rotation: IpgSvgRotation,
        pub rotation_radians: f32,
        pub opacity: f32,
        pub mouse_pointer: Option<IpgMousePointer>,
        pub show: bool,
}

impl IpgSvg {
    pub fn new( 
        id: usize,
        parent_id: String,
        svg_path: String,
        width: Length,
        height: Length,
        content_fit: IpgSvgContentFit,
        rotation: IpgSvgRotation,
        rotation_radians: f32,
        opacity: f32,
        mouse_pointer: Option<IpgMousePointer>,
        show: bool,
        ) -> Self {
        Self {
            id,
            parent_id,
            svg_path,
            width,
            height,
            content_fit,
            rotation,
            rotation_radians,
            opacity,
            mouse_pointer,
            show,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSvgContentFit {
    Contain,
    Cover,
    Fill,
    IpgNone,
    ScaleDown,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSvgRotation {
    Floating,
    Solid,
}

pub fn construct_svg(sg: &IpgSvg) 
                    -> Option<Element<'_, Message>> {

    if !sg.show {
        return None
    }

    let svg_handle = svg::Handle::from_path(sg.svg_path.clone());

    let svg_widget: Element<SvgMessage> = Svg::new(svg_handle)
                                                .width(sg.width)
                                                .height(sg.height)
                                                .content_fit(match_content_fit(sg.content_fit.clone()))
                                                .rotation(match_rotation(sg.rotation.clone(), Radians::from(sg.rotation_radians)))
                                                .opacity(sg.opacity)
                                                .into();

    let pointer: Interaction = get_interaction(&sg.mouse_pointer.clone());

    let widget: Element<SvgMessage> = 
                MouseArea::new(svg_widget)
                    .on_press(SvgMessage::OnPress)
                    .on_release(SvgMessage::OnRelease)
                    .on_right_press(SvgMessage::OnRightPress)
                    .on_right_release(SvgMessage::OnRightRelease)
                    .on_middle_press(SvgMessage::OnMiddlePress)
                    .on_middle_release(SvgMessage::OnMiddleRelease)
                    .on_enter(SvgMessage::OnEnter)
                    .on_move(SvgMessage::OnMove)
                    .on_exit(SvgMessage::OnExit)
                    //Need to add in the other Interactions
                    .interaction(pointer)
                    .into();

    Some(widget.map(move |message| Message::Svg(sg.id, message)))

}

fn match_content_fit(content: IpgSvgContentFit) -> iced::ContentFit {
    match content {
        IpgSvgContentFit::Contain => iced::ContentFit::Contain,
        IpgSvgContentFit::Cover => iced::ContentFit::Cover,
        IpgSvgContentFit::Fill => iced::ContentFit::Fill,
        IpgSvgContentFit::IpgNone => iced::ContentFit::None,
        IpgSvgContentFit::ScaleDown => iced::ContentFit::ScaleDown,
    }
}

fn match_rotation(rot: IpgSvgRotation, radians: Radians) -> Rotation {
    match rot {
        IpgSvgRotation::Floating => Rotation::Floating(radians),
        IpgSvgRotation::Solid => Rotation::Solid(radians),
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSvgParam {
    Height,
    HeightFill,
    ImagePath,
    Show,
    Width,
    WidthFill,
    RotationRadians,
    Opacity,
}

pub fn svg_item_update(img: &mut IpgSvg,
                            item: &PyObject,
                            value: &PyObject,
                            )
{
    let update = try_extract_svg_update(item);
    let name = "Svg".to_string();
    match update {
        IpgSvgParam::Height => {
            let val = try_extract_f64(value, name);
            img.height = get_height(Some(val as f32), false);
        },
        IpgSvgParam::HeightFill => {
            let val = try_extract_boolean(value, name);
            img.height = get_height(None, val);
        },
        IpgSvgParam::ImagePath => {
            img.svg_path = try_extract_string(value, name);
        },
        IpgSvgParam::Show => {
            img.show = try_extract_boolean(value, name);
        },
        IpgSvgParam::Width => {
            let val = try_extract_f64(value, name);
            img.width = get_width(Some(val as f32), false);
        },
        IpgSvgParam::WidthFill => {
            let val = try_extract_boolean(value, name);
            img.width = get_width(None, val);
        },
        IpgSvgParam::RotationRadians => {
            let val = try_extract_f64(value, name);
            img.rotation_radians = val as f32;
        },
        IpgSvgParam::Opacity => {
            let val = try_extract_f64(value, name);
            img.opacity = val as f32;
        },
    }
}

pub fn try_extract_svg_update(update_obj: &PyObject) -> IpgSvgParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgSvgParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Svg update extraction failed"),
        }
    })
}
