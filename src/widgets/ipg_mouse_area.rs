//! ipg_mousearea


use crate::widgets::callbacks::{CallbackName, invoke_callback, invoke_callback_with_args};
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};
use crate::app::Message;

use iced::{Element, Point};
use iced::widget::{self, Column};
use iced::mouse;

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone, Default)]
pub struct MouseArea {
        pub id: usize,
        pub mouse_pointer: Option<MousePointer>,
        pub enabled: Option<bool>,
        pub show: bool,
}

impl MouseArea {
    pub fn construct<'a>(
        &'a self, 
        content: Vec<Element<'a, Message>>,
    ) -> Option<Element<'a, Message>> {

        if !self.show { return None }

        let pt = if let Some(pt) = &self.mouse_pointer{
            pt
        } else { &MousePointer::None };
        
        let pointer: mouse::Interaction = MousePointer::to_iced(pt);

        let content: Element<Message> = Column::with_children(content).into();

        Some(widget::MouseArea::new(content)
            .on_press(Message::MouseArea(self.id, MaMessage::OnPress))
            .on_release(Message::MouseArea(self.id, MaMessage::OnRelease))
            .on_right_press(Message::MouseArea(self.id, MaMessage::OnRightPress))
            .on_right_release(Message::MouseArea(self.id, MaMessage::OnRightRelease))
            .on_middle_press(Message::MouseArea(self.id, MaMessage::OnMiddlePress))
            .on_middle_release(Message::MouseArea(self.id, MaMessage::OnMiddleRelease))
            .on_enter(Message::MouseArea(self.id, MaMessage::OnEnter))
            .on_move(move|p| Message::MouseArea(self.id, MaMessage::OnMove(p)))
            .on_exit(Message::MouseArea(self.id, MaMessage::OnExit))
            .interaction(pointer)
            .into())
    
    
    }
}

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum MaMessage {
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

pub fn mousearea_callback(id: usize, message: MaMessage) {
    match message {
        MaMessage::OnPress => invoke_callback(id, CallbackName::OnPress, "MouseArea"),
        MaMessage::OnRelease => invoke_callback(id, CallbackName::OnRelease, "MouseArea"),
        MaMessage::OnRightPress => invoke_callback(id, CallbackName::OnRightPress, "MouseArea"),
        MaMessage::OnRightRelease => invoke_callback(id, CallbackName::OnRightRelease, "MouseArea"),
        MaMessage::OnMiddlePress => invoke_callback(id, CallbackName::OnMiddlePress, "MouseArea"),
        MaMessage::OnMiddleRelease => invoke_callback(id, CallbackName::OnMiddleRelease, "MouseArea"),
        MaMessage::OnEnter => invoke_callback(id, CallbackName::OnEnter, "MouseArea"),
        MaMessage::OnMove(point) => invoke_callback_with_args(
            id, CallbackName::OnMove, "MouseArea",
            (point.x, point.y),
            "def on_move(ma_id: int, point: tuple[float, float])"),
        MaMessage::OnExit => invoke_callback(id, CallbackName::OnExit, "MouseArea"),
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
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
            MousePointer::Alias => mouse::Interaction::Alias,
            MousePointer::AllScroll => mouse::Interaction::AllScroll,
            MousePointer::Cell => mouse::Interaction::Cell,
            MousePointer::ContextMenu => mouse::Interaction::ContextMenu,
            MousePointer::Copy => mouse::Interaction::Copy,
            MousePointer::Crosshair => mouse::Interaction::Crosshair,
            MousePointer::Grab => mouse::Interaction::Grab,
            MousePointer::Grabbing => mouse::Interaction::Grabbing,
            MousePointer::Help => mouse::Interaction::Help,
            MousePointer::Hidden => mouse::Interaction::Hidden,
            MousePointer::Idle => mouse::Interaction::Idle,
            MousePointer::Move => mouse::Interaction::Move,
            MousePointer::NoDrop => mouse::Interaction::NoDrop,
            MousePointer::None => mouse::Interaction::None,
            MousePointer::NotAllowed => mouse::Interaction::NotAllowed,
            MousePointer::Pointer => mouse::Interaction::Pointer,
            MousePointer::Progress => mouse::Interaction::Progress,
            MousePointer::ResizingColumn => mouse::Interaction::ResizingColumn,
            MousePointer::ResizingDiagonallyDown => mouse::Interaction::ResizingDiagonallyDown,
            MousePointer::ResizingDiagonallyUp => mouse::Interaction::ResizingDiagonallyUp,
            MousePointer::ResizingHorizontally => mouse::Interaction::ResizingHorizontally,
            MousePointer::ResizingRow => mouse::Interaction::ResizingRow,
            MousePointer::ResizingVertically => mouse::Interaction::ResizingVertically,
            MousePointer::Text => mouse::Interaction::Text,
            MousePointer::Wait => mouse::Interaction::Wait,
            MousePointer::ZoomIn => mouse::Interaction::ZoomIn,
            MousePointer::ZoomOut => mouse::Interaction::ZoomOut,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MouseAreaParam {
    Enabled,
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
            MouseAreaParam::Enabled => set_t_value(&mut self.enabled, value, "MouseAreaParam::Enalbled"),
            MouseAreaParam::MousePointer => set_t_value(&mut self.mouse_pointer, value, "MouseAreaParam::MousePointer"),
            MouseAreaParam::Show => set_t_value(&mut self.show, value, "MouseAreaParam::Show"),
        }
    }
}
