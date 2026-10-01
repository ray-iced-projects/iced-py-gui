//! PopUp widget definition

use crate::app::Message;
use crate::widgets::callbacks::{CallbackName, invoke_callback};
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};

use iced::widget::column;
use iced::{Element, Widget};

use crate::iced_widgets::popover::{Popover, Position};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct PopOver {
    pub id: usize,
    pub opened: bool,
    pub position_bottom: Option<bool>,
    pub position_center: Option<bool>,
    pub position_left: Option<bool>,
    pub position_top: Option<bool>,
    pub position_right: Option<bool>,
    pub gap: Option<f32>,
    pub padding: Option<f32>,
    pub snap_within_viewport: Option<bool>,
    pub focus_trap: Option<bool>,
}

impl PopOver {
    // fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
    //     id.and_then(|id| widgets.get(&id))
    // }

    pub fn construct<'a>(&'a self, content: Vec<Element<'a, Message>>) -> Option<Element<'a, Message>> {
        if content.len() == 1 && !self.opened {
            return None;
        }

        let position = match (
            self.position_bottom,
            self.position_center,
            self.position_left,
            self.position_top,
            self.position_right,
        ) {
            (Some(true), None, None, None, None) => Position::Bottom,
            (None, Some(true), None, None, None) => Position::Auto,
            (None, None, Some(true), None, None) => Position::Left,
            (None, None, None, Some(true), None) => Position::Top,
            (None, None, None, None, Some(true)) => Position::Right,
            _ => Position::Auto,
        };

        let mut iter = content.into_iter();

        let pu: Option<Element<'a, Message>> = if let Some(first) = iter.next() {
            if let Some(second) = iter.next() {
                // Two or more elements: first is base, rest are popup content
                let mut remaining = vec![second];
                remaining.extend(iter);
                let popup_content: Element<'a, Message> = column(remaining).boxed();
                let popup = self.opened.then_some(popup_content);
                Some(
                    Popover::new(first, popup)
                        .position(position)
                        .gap(self.gap.unwrap_or_default())
                        .snap_within_viewport(self.snap_within_viewport.unwrap_or(true))
                        .on_close(Message::Popover(self.id, PopOverMessage::ClickedOutside))
                        .boxed(),
                )
            } else {
                // One element with no popup: return it directly
                Some(first)
            }
        } else {
            None
        };

        pu
    }
}

#[derive(Debug, Clone)]
pub enum PopOverMessage {
    ClickedOutside,
    OnClose,
    OnOpen,
}

pub fn popover_callback(id: usize, message: PopOverMessage) {
    match message {
        PopOverMessage::ClickedOutside => {
            invoke_callback(id, CallbackName::OnClickOutside, "PopUp");
        }
        PopOverMessage::OnClose => {
            invoke_callback(id, CallbackName::OnClose, "PopUp");
        }
        PopOverMessage::OnOpen => {
            invoke_callback(id, CallbackName::OnOpen, "PopUp");
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum PopOverParam {
    FocusTrap,
    Gap,
    Opened,
    Padding,
    PositionBottom,
    PositionCenter,
    PositionLeft,
    PositionRight,
    PositionTop,
    SnapWithinViewport,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for PopOver {
    type Param = PopOverParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            PopOverParam::FocusTrap => set_t_value(&mut self.focus_trap, value, "PopOverParam::FocusTrap"),
            PopOverParam::Gap => set_t_value(&mut self.gap, value, "PopOverParam::Gap"),
            PopOverParam::Opened => set_t_value(&mut self.opened, value, "PopOverParam::Opened"),
            PopOverParam::Padding => set_t_value(&mut self.padding, value, "PopOverParam::Padding"),
            PopOverParam::PositionBottom => {
                set_t_value(&mut self.position_bottom, value, "PopOverParam::PositionBottom")
            }
            PopOverParam::PositionCenter => {
                set_t_value(&mut self.position_center, value, "PopOverParam::PositionCenter")
            }
            PopOverParam::PositionLeft => set_t_value(&mut self.position_left, value, "PopOverParam::PositionLeft"),
            PopOverParam::PositionRight => set_t_value(&mut self.position_right, value, "PopOverParam::PositionRight"),
            PopOverParam::PositionTop => set_t_value(&mut self.position_top, value, "PopOverParam::PositionTop"),
            PopOverParam::SnapWithinViewport => set_t_value(
                &mut self.snap_within_viewport,
                value,
                "PopOverParam::SnapWithinViewport",
            ),
        }
    }
}
