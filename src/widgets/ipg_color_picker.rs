//! ipg_color_picker
use crate::IpgState;
use crate::state::{Containers};
use crate::widgets::callbacks::{invoke_callback, invoke_callback_with_args};
use crate::app::Message;

use crate::ipg_widgets::ipg_color_picker::{
    ColorPicker as CP,
    Position,
    ColorPickerState,
    ContentMsg,
    ColorPickerEvent,
};

use iced::widget::container;
use iced::{Element, Task};
#[derive(Debug, Clone)]
pub struct ColorPicker {
    pub id: usize,
    pub opened: bool,
    pub gap: Option<u32>,
    pub position: Position,
    pub snap_within_viewport: Option<bool>,
    pub cp: ColorPickerState,
}

impl ColorPicker {

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        ) -> Option<Element<'a, Message>> {

        let btn = if content.is_empty() {
            return None;
        } else {
            content.remove(0).map(|_| ColorPikMessage::Noop)
        };

        let panel = 
            self.cp.view(move |msg| ColorPikMessage::ColorPicker(msg));

        let id = self.id;

        let cpk: Element<'_, ColorPikMessage> = CP::new(
            btn,
            panel,
            self.cp.current_color(),
            self.position,
        )
        .opened(self.opened)
        .on_open(ColorPikMessage::SetOpened)
        .gap(self.gap.unwrap_or(10))
        .style(container::rounded_box)
        .into();

        Some(cpk.map(move |message| Message::ColorPicker(id, message)))

    }

}


#[derive(Debug, Clone)]
pub enum ColorPikMessage {
    Noop,
    SetOpened(bool),
    ColorPicker(ContentMsg),
}

pub fn color_picker_callback(
    state: &mut IpgState, 
    id: usize, 
    message: ColorPikMessage,
) -> Option<Task<Message>> {

    match message {
        ColorPikMessage::Noop => (),
        ColorPikMessage::SetOpened(open) => {
            if let Some(Containers::ColorPicker(cp)) = state.containers.get_mut(&id) {
                cp.opened = open;
                invoke_callback_with_args(id, "on_open", "ColorPicker", open,
                    "def cb(wid: int, opened: bool)");
            }
        },
        ColorPikMessage::ColorPicker(content) => {
            if let Some(Containers::ColorPicker(cp)) = state.containers.get_mut(&id) {
                let event = cp.cp.update(content);

                match event {
                    Some(ColorPickerEvent::Submitted(_)) => {
                        cp.opened = false;
                        invoke_callback_with_args(id, "on_submit", "ColorPicker", cp.cp.current_color_text(),
                            "def cb(wid: int, color: str)");
                    }
                    Some(ColorPickerEvent::Cancelled) => {
                        cp.opened = false;
                        invoke_callback(id, "on_cancel", "ColorPicker");
                    }
                    Some(ColorPickerEvent::Copy(text)) => {
                        return Some(iced::clipboard::write(text).discard());
                    }
                    None => {}
                }
            }
        },
        
    }

    None
        
}
