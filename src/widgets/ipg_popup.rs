//! PopUp widget definition

use crate::app::Message;
use crate::widgets::callbacks::invoke_callback;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};

use iced::{Element, Pixels};
use iced::widget::column;

use crate::ipg_widgets::ipg_popup::popup::{Popup, Position};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;



#[derive(Debug, Clone)]
pub struct PopUp {
    pub id: usize,
    pub opened: Option<bool>,
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

impl PopUp {

    // fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
    //     id.and_then(|id| widgets.get(&id))
    // }
    
    pub fn construct<'a>(
        &'a self,
        content: Vec<Element<'a, Message>>
        ) -> Option<Element<'a, Message>> {
        dbg!(&self.opened, &content.len());

        if content.len() == 1 && (self.opened == Some(false) || self.opened == None) { return None }
    
        let position = match (self.position_bottom, self.position_center, self.position_left,
                                        self.position_top, self.position_right) {
                (Some(true), None, None, None, None) => Position::Bottom,
                (None, Some(true), None, None, None) => Position::Center,
                (None, None, Some(true), None, None) => Position::Left,
                (None, None, None, Some(true), None) => Position::Right,
                (None, None, None, None, Some(true)) => Position::Top,
                _ => Position::Center
            };
        
        let mut iter = content.into_iter();
        
        let pu: Option<Element<'a, Message>> = 
            if let Some(first) = iter.next() {
                dbg!("first");
                if let Some(second) = iter.next() {
                    // Two or more elements: first is widget, rest are content
                    let mut remaining = vec![second];
                    remaining.extend(iter);
                    let popup_content: Element<'a, Message> = column(remaining).into();
                    dbg!("second");
                    Some(Popup::new(first, popup_content, false)
                        .position(position)
                        .gap(self.gap.unwrap_or_default())
                        .padding(Pixels(self.padding.unwrap_or_default()))
                        .snap_within_viewport(self.snap_within_viewport.unwrap_or_default())
                        .focus_trap(self.focus_trap.unwrap_or_default())
                        .on_click_outside(|_| Message::PopUp(self.id, PopUpMessage::ClickedOutside))
                        .on_open(|| Message::PopUp(self.id, PopUpMessage::OnOpen))
                        .on_close(|| Message::PopUp(self.id, PopUpMessage::OnClose)).into())
                } else {
                    dbg!("Only one element");
                    // One element: use as popup content only
                    Some(Popup::without_widget(first, self.opened.unwrap_or_default())
                        .position(position)
                        .gap(self.gap.unwrap_or_default())
                        .padding(Pixels(self.padding.unwrap_or_default()))
                        .snap_within_viewport(self.snap_within_viewport.unwrap_or_default())
                        .focus_trap(self.focus_trap.unwrap_or_default())
                        .on_click_outside(|_| Message::PopUp(self.id, PopUpMessage::ClickedOutside))
                        .on_open(|| Message::PopUp(self.id, PopUpMessage::OnOpen))
                        .on_close(|| Message::PopUp(self.id, PopUpMessage::OnClose)).into())
                }
            } else {
                None
            };

        pu

    }
}

#[derive(Debug, Clone)]
pub enum PopUpMessage {
    ClickedOutside,
    OnClose,
    OnOpen,
}


pub fn popup_callback(id: usize, message: PopUpMessage) {
    match message {
        PopUpMessage::ClickedOutside => {
            invoke_callback(id, "on_click_outside", "PopUp");
        },
        PopUpMessage::OnClose => {
            invoke_callback(id, "on_close", "PopUp");
        },
        PopUpMessage::OnOpen => {
            invoke_callback(id, "on_open", "PopUp");
        },
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum PopUpParam {
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

impl WidgetParamUpdate for PopUp {
    type Param = PopUpParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            PopUpParam::FocusTrap => set_t_value(&mut self.focus_trap, value, "PopUpParam::FocusTrap"),
            PopUpParam::Gap => set_t_value(&mut self.gap, value, "PopUpParam::Gap"),
            PopUpParam::Opened => set_t_value(&mut self.opened, value, "PopUpParam::Opened"),
            PopUpParam::Padding => set_t_value(&mut self.padding, value, "PopUpParam::Padding"),
            PopUpParam::PositionBottom => set_t_value(&mut self.position_bottom, value, "PopUpParam::PositionBottom"),
            PopUpParam::PositionCenter => set_t_value(&mut self.position_center, value, "PopUpParam::PositionCenter"),
            PopUpParam::PositionLeft => set_t_value(&mut self.position_left, value, "PopUpParam::PositionLeft"),
            PopUpParam::PositionRight => set_t_value(&mut self.position_right, value, "PopUpParam::PositionRight"),
            PopUpParam::PositionTop => set_t_value(&mut self.position_top, value, "PopUpParam::PositionTop"),
            PopUpParam::SnapWithinViewport => set_t_value(&mut self.snap_within_viewport, value, "PopUpParam::SnapWithinViewport"),
        }
    }
}
