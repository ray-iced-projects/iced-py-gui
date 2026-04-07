//! ipg_mousearea

use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool};
use crate::{access_callbacks, access_user_data1, IpgState};
use crate::app::Message;

use iced::{Element, Point};
use iced::widget::{self, Column};
use iced::mouse;

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone, Default)]
pub struct MouseArea {
        pub id: usize,
        pub mouse_pointer: Option<MousePointer>,
        pub show: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum MousePointer {
    None,
    Alias,
    AllScroll,
    Cell,
    ContextMenu,
    Copy,
    Crosshair,
    Grab,
    Grabbing,
    Help,
    Hidden,
    Idle,
    Move,
    NoDrop,
    NotAllowed,
    Pointer,
    Progress,
    ResizingColumn,
    ResizingDiagonallyDown,
    ResizingDiagonallyUp,
    ResizingHorizontally,
    ResizingRow,
    ResizingVertically,
    Text,
    Wait,
    ZoomIn,
    ZoomOut,
}

impl MousePointer {
    pub fn default() -> Self {
        MousePointer::None
    }

    pub fn to_iced(&self) -> mouse::Interaction {
        match self {
            MousePointer::Alias =>  mouse::Interaction::Alias,
            MousePointer::AllScroll =>  mouse::Interaction::AllScroll,
            MousePointer::Cell =>  mouse::Interaction::Cell,
            MousePointer::ContextMenu =>  mouse::Interaction::ContextMenu,
            MousePointer::Copy =>  mouse::Interaction::Copy,
            MousePointer::Crosshair =>  mouse::Interaction::Crosshair,
            MousePointer::Grab =>  mouse::Interaction::Grab,
            MousePointer::Grabbing =>  mouse::Interaction::Grabbing,
            MousePointer::Help =>  mouse::Interaction::Help,
            MousePointer::Hidden =>  mouse::Interaction::Hidden,
            MousePointer::Idle =>  mouse::Interaction::Idle,
            MousePointer::Move =>  mouse::Interaction::Move,
            MousePointer::NoDrop =>  mouse::Interaction::NoDrop,
            MousePointer::None => mouse::Interaction::None,
            MousePointer::NotAllowed =>  mouse::Interaction::NotAllowed,
            MousePointer::Pointer =>  mouse::Interaction::Pointer,
            MousePointer::Progress =>  mouse::Interaction::Progress,
            MousePointer::ResizingColumn =>  mouse::Interaction::ResizingColumn,
            MousePointer::ResizingDiagonallyDown =>  mouse::Interaction::ResizingDiagonallyDown,
            MousePointer::ResizingDiagonallyUp =>  mouse::Interaction::ResizingDiagonallyUp,
            MousePointer::ResizingHorizontally =>  mouse::Interaction::ResizingHorizontally,
            MousePointer::ResizingRow =>  mouse::Interaction::ResizingRow,
            MousePointer::ResizingVertically =>  mouse::Interaction::ResizingVertically,
            MousePointer::Text =>  mouse::Interaction::Text,
            MousePointer::Wait =>  mouse::Interaction::Wait,
            MousePointer::ZoomIn =>  mouse::Interaction::ZoomIn,
            MousePointer::ZoomOut =>  mouse::Interaction::ZoomOut,
        }
    }

    pub fn extract(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<MousePointer>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })  
    }
}

pub fn construct_mousearea<'a>(m_area: &'a MouseArea, 
                            content: Vec<Element<'a, Message>>) 
                            -> Element<'a, Message> {

    let pt = if let Some(pt) = &m_area.mouse_pointer{
        pt
    } else { &MousePointer::None };
    
    let pointer: mouse::Interaction = MousePointer::to_iced(pt);

    let cont: Element<Message> = Column::with_children(content).into();

    // Had to use the Message because the content already has Message.  Typical problem
    // with containers that are also like widgets with Message.
    
    widget::MouseArea::new(cont)
        .on_press(Message::MouseAreaOnPress(m_area.id))
        .on_release(Message::MouseAreaOnRelease(m_area.id))
        .on_right_press(Message::MouseAreaOnRightPress(m_area.id))
        .on_right_release(Message::MouseAreaOnRightRelease(m_area.id))
        .on_middle_press(Message::MouseAreaOnMiddlePress(m_area.id))
        .on_middle_release(Message::MouseAreaOnMiddleRelease(m_area.id))
        .on_enter(Message::MouseAreaOnEnter(m_area.id))
        .on_move(move|p| Message::MouseAreaOnMove(p, m_area.id))
        .on_exit(Message::MouseAreaOnExit(m_area.id))
        .interaction(pointer)
        .into()
}

pub fn mousearea_callback(_state: &mut IpgState, id: usize, event_name: String) {
    
    process_callback(id, event_name, None);

}

pub fn mousearea_callback_point(_state: &mut IpgState, 
                                id: usize, 
                                point: Point, 
                                event_name: String,
                                ) {

    let points: Option<(String, f32, String, f32)> = Some(
                ("x".to_string(), point.x,
                "y".to_string(), point.y));

    process_callback(id, event_name, points);
}


fn process_callback(
    id: usize, 
    event_name: String, 
    points_opt: Option<(String, f32, String, f32)>) 
{
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
                Err(err) => panic!("MouseArea callback error with user_data from ud1: {err}")
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
    //             Err(err) => panic!("MouseArea callback error with user_data from ud2: {err}")
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
                Err(err) => panic!("MouseArea callback error without user_data: {err}")
            }
    });

}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum MouseAreaParam {
    MousePointer,
    Show,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for MouseArea {
    type Param = MouseAreaParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            MouseAreaParam::MousePointer => self.mouse_pointer = MousePointer::extract(value),
            MouseAreaParam::Show => set_bool(&mut self.show, value, "Show"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_mousearea() -> MouseArea {
        MouseArea {
            id: 0,
            mouse_pointer: None,
            show: true,
        }
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    #[test]
    fn test_show() {
        let mut ma = make_mousearea();
        ma.param_update(MouseAreaParam::Show, &py_obj(false));
        assert!(!ma.show);
    }
}
