//! ipg_mousearea

use iced::widget::MouseArea;
use iced::Element;
use iced::widget::Column;
use iced::mouse::Interaction;

use pyo3::pyclass;
use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;

use ipg_helpers::try_extract_boolean;
use ipg_types::Message;


#[derive(Debug, Clone)]
pub struct IpgMouseArea {
        pub id: usize,
        pub mouse_pointer: Option<IpgMousePointer>,
        pub show: bool,
}

impl IpgMouseArea {
    pub fn new( 
        id: usize,
        mouse_pointer: Option<IpgMousePointer>,
        show: bool,
        ) -> Self {
        Self {
            id,
            mouse_pointer,
            show,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgMousePointer {
    IpgNone,
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

pub fn construct_mousearea<'a>(m_area: &'a IpgMouseArea, 
                            content: Vec<Element<'a, Message>>) 
                            -> Element<'a, Message> {

    let pointer: Interaction = get_interaction(&m_area.mouse_pointer);

    let cont: Element<Message> = Column::with_children(content).into();

    // Had to use the Message because the content already has Message.  Typical problem
    // with containers that are also like widgets with Message.
    
    MouseArea::new(cont)
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

pub fn get_interaction(pointer: &Option<IpgMousePointer>) -> Interaction {
    if pointer.is_none() {
        return Interaction::None
    }

    match pointer.clone().unwrap() {
        IpgMousePointer::IpgNone => Interaction::None,
        IpgMousePointer::Alias => Interaction::Alias,
        IpgMousePointer::AllScroll => Interaction::AllScroll,
        IpgMousePointer::Cell => Interaction::Cell,
        IpgMousePointer::ContextMenu => Interaction::ContextMenu,
        IpgMousePointer::Copy => Interaction::Copy,
        IpgMousePointer::Crosshair => Interaction::Crosshair,
        IpgMousePointer::Grab => Interaction::Grab,
        IpgMousePointer::Grabbing => Interaction::Grabbing,
        IpgMousePointer::Help => Interaction::Help,
        IpgMousePointer::Hidden => Interaction::Hidden,
        IpgMousePointer::Idle => Interaction::Idle,
        IpgMousePointer::Move => Interaction::Move,
        IpgMousePointer::NoDrop => Interaction::NoDrop,
        IpgMousePointer::NotAllowed => Interaction::NotAllowed,
        IpgMousePointer::Pointer => Interaction::Pointer,
        IpgMousePointer::Progress => Interaction::Progress,
        IpgMousePointer::ResizingColumn => Interaction::ResizingColumn,
        IpgMousePointer::ResizingDiagonallyDown => Interaction::ResizingDiagonallyDown,
        IpgMousePointer::ResizingDiagonallyUp => Interaction::ResizingDiagonallyUp,
        IpgMousePointer::ResizingHorizontally => Interaction::ResizingHorizontally,
        IpgMousePointer::ResizingRow => Interaction::ResizingRow,
        IpgMousePointer::ResizingVertically => Interaction::ResizingVertically,
        IpgMousePointer::Text => Interaction::Text,
        IpgMousePointer::Wait => Interaction::Wait,
        IpgMousePointer::ZoomIn => Interaction::ZoomIn,
        IpgMousePointer::ZoomOut => Interaction::ZoomOut,
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgMouseAreaParam {
    Show,
}


pub fn mousearea_item_update(img: &mut IpgMouseArea,
                                item: &PyObject,
                                value: &PyObject,
                                )
{

    let update = try_extract_mousearea_update(item);

    match update {
        IpgMouseAreaParam::Show => {
            img.show = try_extract_boolean(value, "MouseArea".to_string());
        },
    }
}

pub fn try_extract_mousearea_update(update_obj: &PyObject) -> IpgMouseAreaParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgMouseAreaParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("MouseArea update extraction failed"),
        }
    })
}