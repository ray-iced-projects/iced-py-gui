//!ipg_image
#![allow(clippy::enum_variant_names)]
use std::collections::HashMap;

use crate::access_user_data1;
use crate::app;
use crate::access_callbacks;
use crate::py_api::helpers::get_padding;
use crate::widgets::widget_param_update::set_f32_opt;
use crate::widgets::widget_param_update::set_string;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, 
    set_bool, set_width, set_width_fill, set_height, 
    set_height_fill, set_vec_f32_opt,
};
use super::ipg_mousearea::IpgMousePointer;

use iced::mouse::Interaction;
use iced::widget::image::FilterMethod;
use iced::{Length, Element, Point, Radians, Rotation};
use iced::widget::{Container, Image, MouseArea};
use iced::advanced::image;

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgImage {
        pub id: usize,
        pub parent_id: String,
        pub image_path: String,
        pub width: Length,
        pub height: Length,
        pub padding: Option<Vec<f32>>,
        pub content_fit: Option<IpgImageContentFit>,
        pub filter_method: Option<IpgImageFilterMethod>,
        pub rotation_method: Option<IpgImageRotation>,
        pub rotation_radians: Option<f32>,
        pub opacity: Option<f32>,
        pub mouse_pointer: Option<IpgMousePointer>,
        pub show: bool,
}

#[derive(Debug, Clone)]
pub enum ImageMessage {
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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgImageContentFit {
    Contain,
    Cover,
    Fill,
    IpgNone,
    ScaleDown,
}

impl IpgImageContentFit {
    pub fn default() -> Self {
        IpgImageContentFit::Contain
    }

    pub fn to_iced(&self) -> iced::ContentFit {
        match self {
            IpgImageContentFit::Contain => iced::ContentFit::Contain,
            IpgImageContentFit::Cover => iced::ContentFit::Cover,
            IpgImageContentFit::Fill => iced::ContentFit::Fill,
            IpgImageContentFit::IpgNone => iced::ContentFit::None,
            IpgImageContentFit::ScaleDown => iced::ContentFit::ScaleDown,
        }
    }

    fn extract(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<IpgImageContentFit>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })  
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgImageFilterMethod {
    Linear,
    Nearest,
}

impl IpgImageFilterMethod {
    pub fn default() -> Self {
        IpgImageFilterMethod::Linear
    }

    pub fn to_iced(&self) -> FilterMethod {
        match self {
            IpgImageFilterMethod::Linear => FilterMethod::Linear,
            IpgImageFilterMethod::Nearest => FilterMethod::Nearest,
        }
    }

    fn extract(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<IpgImageFilterMethod>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })  
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgImageRotation {
    Floating,
    Solid,
}

impl IpgImageRotation {
    pub fn default() -> Rotation {
        Rotation::Floating(Radians(0.0))
    }

    pub fn to_iced(&self, rad: Option<f32>) -> Rotation {
        let rads = if let Some(rad) = rad {
            rad
        } else { 0.0 };
        match self {
            IpgImageRotation::Floating => Rotation::Floating(Radians(rads)),
            IpgImageRotation::Solid => Rotation::Solid(Radians(rads)),
        }
    }

    fn extract(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<IpgImageRotation>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })  
    }
}

pub fn construct_image(image: &IpgImage) 
                        -> Option<Element<'_, app::Message>> {

    if !image.show {
        return None
    }

    let fit = if let Some(f) = &image.content_fit {
        f.to_iced()
    } else {
        IpgImageContentFit::default().to_iced()
    };

    let filter = if let Some(f) = &image.filter_method {
        f.to_iced()
    } else {
        IpgImageFilterMethod::default().to_iced()
    };

    let rotation = if let Some(r) = &image.rotation_method {
        r.to_iced(image.rotation_radians)
    } else {
        IpgImageRotation::default()
    };

    let op = if let Some(op) = image.opacity {
        op
    } else { 1.0 };

    let img: Element<ImageMessage> = 
        Image::<image::Handle>::new(image.image_path.clone())
            .content_fit(fit)
            .filter_method(filter)
            .rotation(rotation)
            .opacity(op)
            .into();

    let cont: Element<ImageMessage> = Container::new(img)
                                                .width(image.width)
                                                .height(image.height)
                                                .padding(get_padding(&image.padding))
                                                .into();

    let pt = if let Some(pt) = &image.mouse_pointer{
        pt.to_iced()
    } else { Interaction::None };

    let ma: Element<ImageMessage> = 
                MouseArea::new(cont)
                    .on_press(ImageMessage::OnPress)
                    .on_release(ImageMessage::OnRelease)
                    .on_right_press(ImageMessage::OnRightPress)
                    .on_right_release(ImageMessage::OnRightRelease)
                    .on_middle_press(ImageMessage::OnMiddlePress)
                    .on_middle_release(ImageMessage::OnMiddleRelease)
                    .on_enter(ImageMessage::OnEnter)
                    .on_move(ImageMessage::OnMove)
                    .on_exit(ImageMessage::OnExit)
                    .interaction(pt)
                    .into();

    Some(ma.map(move |message| app::Message::Image(image.id, message)))

}

pub fn image_callback(id: usize, message: ImageMessage) {

    match message {
        ImageMessage::OnPress => {
            process_callback(id, "on_press".to_string(), None);
        },
        ImageMessage::OnRelease => {
            process_callback(id, "on_release".to_string(), None);
        },
        ImageMessage::OnRightPress => {
            process_callback(id, "on_right_press".to_string(), None);
        },
        ImageMessage::OnRightRelease => {
            process_callback(id, "on_right_release".to_string(), None);
        },
        ImageMessage::OnMiddlePress => {
            process_callback(id, "on_middle_press".to_string(), None);
        },
        ImageMessage::OnMiddleRelease => {
            process_callback(id, "on_middle_release".to_string(), None);
        },
        ImageMessage::OnEnter => {
            process_callback(id, "on_enter".to_string(), None);
        },
        ImageMessage::OnMove(point) => {
            let points: Option<HashMap<String, f32>> = Some(HashMap::from([
                ("x".to_string(), point.x),
                ("y".to_string(), point.y)
            ]));
            
            process_callback(id, "on_move".to_string(), points);
        },
        ImageMessage::OnExit => {
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
                Err(err) => panic!("Image callback error with user_data from ud1: {err}")
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
    //             Err(err) => panic!("Image callback error with user_data from ud2: {err}")
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
                Err(err) => panic!("Image callback error without user_data: {err}")
            }
    });

}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgImageParam {
    ContentFit,
    FilterMethod,
    Height,
    HeightFill,
    ImagePath,
    MousePointer,
    Opacity,
    Padding,
    RotationMethod,
    RotationRadians,
    Show,
    Width,
    WidthFill,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgImage {
    type Param = IpgImageParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgImageParam::ContentFit => self.content_fit = IpgImageContentFit::extract(value),
            IpgImageParam::FilterMethod => self.filter_method = IpgImageFilterMethod::extract(value),
            IpgImageParam::Height => set_height(&mut self.height, value, name),
            IpgImageParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgImageParam::ImagePath => set_string(&mut self.image_path, value, name),
            IpgImageParam::MousePointer => self.mouse_pointer = IpgMousePointer::extract(value),
            IpgImageParam::Opacity => set_f32_opt(&mut self.opacity, value, name),
            IpgImageParam::Padding => set_vec_f32_opt(&mut self.padding, value, name),
            IpgImageParam::RotationMethod => self.rotation_method = IpgImageRotation::extract(value),
            IpgImageParam::RotationRadians => set_f32_opt(&mut self.rotation_radians, value, name),
            IpgImageParam::Show => set_bool(&mut self.show, value, name),
            IpgImageParam::Width => set_width(&mut self.width, value, name),
            IpgImageParam::WidthFill => set_width_fill(&mut self.width, value, name),
        }
    }
}


