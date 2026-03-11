//! ipg_svg
#![allow(clippy::enum_variant_names)]
use std::collections::HashMap;

use crate::access_user_data1;
use crate::app;
use crate::access_callbacks;
use crate::IpgState;
use crate::app::Message;
use crate::widgets::enums::IpgContentFit;
use crate::widgets::enums::IpgRotation;
use crate::widgets::widget_param_update::WidgetParamUpdate;
use crate::widgets::widget_param_update::set_bool;
use crate::widgets::widget_param_update::set_height;
use crate::widgets::widget_param_update::set_opt_f32;
use crate::widgets::widget_param_update::set_opt_iced_color;
use crate::widgets::widget_param_update::set_string;
use crate::widgets::widget_param_update::set_width;
use super::ipg_mousearea::IpgMousePointer;

use iced::Color;
use iced::{Length, Element, Point};
use iced::widget::{Svg, MouseArea};
use iced::mouse::Interaction;
use iced::advanced::svg;

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgSvg {
        pub id: usize,
        pub parent_id: String,
        pub svg_path: String,
        pub width: Length,
        pub height: Length,
        pub color_filter: Option<Color>,
        pub content_fit: Option<IpgContentFit>,
        pub rotation_type: Option<IpgRotation>,
        pub rotation_radians: Option<f32>,
        pub opacity: Option<f32>,
        pub mouse_pointer: Option<IpgMousePointer>,
        pub show: bool,
}

#[derive(Debug, Clone)]
pub enum SvgMessage {
    OnPress,
    OnRelease,
    OnRightPress,
    OnRightRelease,
    OnMiddlePress,
    OnMiddleRelease,
    OnEnter,
    OnMove(Point),
    OnExit,
}

pub fn construct_svg(
    ipg_svg: &IpgSvg) 
    -> Option<Element<'_, Message>> {

    if !ipg_svg.show {
        return None
    }

    let svg_handle = svg::Handle::from_path(ipg_svg.svg_path.clone());

    let svg = Svg::new(svg_handle)
                .width(ipg_svg.width)
                .height(ipg_svg.height);

    let svg = if let Some(cf) = &ipg_svg.content_fit {
        svg.content_fit(cf.to_iced())
    } else { svg };

    let svg = if let Some(rt) = &ipg_svg.rotation_type {
        svg.rotation(rt.to_iced(ipg_svg.rotation_radians))
    } else { svg };

    let svg = if let Some(op) = ipg_svg.opacity {
        svg.opacity(op)
    } else { svg };

    let pt = if let Some(pt) = &ipg_svg.mouse_pointer{
        pt.to_iced()
    } else { Interaction::None };

    let widget: Element<SvgMessage> = 
                MouseArea::new(svg)
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
                    .interaction(pt)
                    .into();

    Some(widget.map(move |message| app::Message::Svg(ipg_svg.id, message)))

}

pub fn svg_callback(_state: &mut IpgState, id: usize, message: SvgMessage) {

    match message {
        SvgMessage::OnPress => {
            process_callback(id, "on_press".to_string(), None);
        },
        SvgMessage::OnRelease => {
            process_callback(id, "on_release".to_string(), None);
        },
        SvgMessage::OnRightPress => {
            process_callback(id, "on_right_press".to_string(), None);
        },
        SvgMessage::OnRightRelease => {
            process_callback(id, "on_right_release".to_string(), None);
        },
        SvgMessage::OnMiddlePress => {
            process_callback(id, "on_middle_press".to_string(), None);
        },
        SvgMessage::OnMiddleRelease => {
            process_callback(id, "on_middle_release".to_string(), None);
        },
        SvgMessage::OnEnter => {
            process_callback(id, "on_enter".to_string(), None);
        },
        SvgMessage::OnMove(point) => {
            let points: Option<HashMap<String, f32>> = Some(HashMap::from([
                ("x".to_string(), point.x),
                ("y".to_string(), point.y)
            ]));
            
            process_callback(id, "on_move".to_string(), points);
        },
        SvgMessage::OnExit => {
            process_callback(id, "on_exit".to_string(), None);
        },
    }
}


fn process_callback(
    id: usize,
    event_name: String,
    points_opt: Option<HashMap<String, f32>>,
) {
    let ud1 = access_user_data1();
    let ud_opt = ud1.user_data.get(&id);

    let app_cbs = access_callbacks();
    let callback = match app_cbs.callbacks.get(&(id, event_name)) {
        Some(cb) => cb,
        None => return,
    };

    let cb = Python::attach(|py| callback.clone_ref(py));
    drop(app_cbs);

    // Execute the callback with user data from ud1
    if let Some(user_data) = ud_opt {
        Python::attach(|py| {
            let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone(), user_data)),
                None => cb.call1(py, (id, user_data)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("SVG callback error with user_data from ud1: {err}")
            }
        });
        drop(ud1); // Drop ud1 after processing
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Execute the callback with user data from ud2
    // let ud2 = access_user_data2();
    
    // if let Some(user_data) = ud2.user_data.get(&id) {
    //     Python::attach(|py| {
    //         let res = match points_opt {
    //             Some(ref points) => cb.call1(py, (id, points.clone(), user_data)),
    //             None => cb.call1(py, (id, user_data)),
    //         };

    //         match res {
    //             Ok(_) => (),
    //             Err(err) => panic!("SVG callback error with user_data from ud2: {err}")
    //         }
    //     });
    //     drop(ud2); // Drop ud2 after processing
    //     return;
    // }
    // drop(ud2); // Drop ud2 if no user data is found

    // Execute the callback without user data
    Python::attach(|py| {
        let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone())),
                None => cb.call1(py, (id,)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("SVG callback error without user_data: {err}")
            }
    });

}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSvgParam {
    ColorFilter,
    ContentFit,
    Height,
    MousePointer,
    Opacity,
    RotationRadians,
    RotationType,
    Show,
    SvgPath,
    Width,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgSvg {
    type Param = IpgSvgParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        let name = String::new();
        match param {
            IpgSvgParam::ColorFilter => set_opt_iced_color(&mut self.color_filter, value, name),
            IpgSvgParam::ContentFit => self.content_fit = IpgContentFit::extract(value),
            IpgSvgParam::Height => set_height(&mut self.height, value, name),
            IpgSvgParam::MousePointer => self.mouse_pointer = IpgMousePointer::extract(value),
            IpgSvgParam::Opacity => set_opt_f32(&mut self.opacity, value, name),
            IpgSvgParam::RotationRadians => set_opt_f32(&mut self.rotation_radians, value, name),
            IpgSvgParam::RotationType => self.rotation_type = IpgRotation::extract(value),
            IpgSvgParam::Show => set_bool(&mut self.show, value, name),
            IpgSvgParam::SvgPath => set_string(&mut self.svg_path, value, "SvgPath"),
            IpgSvgParam::Width => set_width(&mut self.width, value, name),
        }
    }
}

