//! ipg_color_picker
use crate::IpgState;
use crate::state::{Containers};
use crate::widgets::callbacks::{invoke_callback_with_args};
use crate::app::Message;

use crate::ipg_widgets::ipg_date_picker::{
    DatePicker as DP,
    Position,
};

use iced::widget::container;
use iced::{Element, Task};
use pyo3::pyclass;
#[derive(Debug, Clone)]
pub struct DatePicker {
    pub id: usize,
    pub opened: bool,
    pub gap: Option<u32>,
    pub position: Position,
    pub selected_date: Option<String>,
    pub snap_within_viewport: Option<bool>,
}

impl DatePicker {

    pub fn construct<'a>(
        &'a self,
        mut content: Vec<Element<'a, Message>>,
        ) -> Option<Element<'a, Message>> {

        let btn = if content.is_empty() {
            return None;
        } else {
            content.remove(0).map(|_| DatePikMessage::Noop)
        };

        let id = self.id;

        let content = iced::widget::void();

        let dpk = DP::new(
            btn,
            content,
            self.selected_date.clone(),
            self.position,
        )
        .opened(self.opened)
        .on_open(DatePikMessage::Opened)
        .gap(self.gap.unwrap_or(10))
        .style(container::rounded_box);

        let dpk: Element<'_, DatePikMessage> = dpk.into();
        Some(dpk.map(move |message| Message::DatePicker(id, message)))

    }

}


#[derive(Debug, Clone)]
pub enum DatePikMessage {
    Noop,
    Opened(bool),
}

pub fn date_picker_callback(
    state: &mut IpgState, 
    id: usize, 
    message: DatePikMessage,
) -> Option<Task<Message>> {

    match message {
        DatePikMessage::Noop => (),
        DatePikMessage::Opened(open) => {
            if let Some(Containers::DatePicker(cp)) = state.containers.get_mut(&id) {
                cp.opened = open;
                invoke_callback_with_args(id, "on_open", "DaterPicker", open,
                    "def cb(wid: int, opened: bool)");
            }
        },
    }

    None
        
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq)]
pub enum DatePickerParam {
    Label,
    Padding,
    SizeFactor,
    Show,
}



// // ---------------------------------------------------------------------------
// // WidgetParamUpdate implementation
// // ---------------------------------------------------------------------------

// impl WidgetParamUpdate for DatePicker {
//     type Param = DatePickerParam;

//     fn param_update(&mut self, param: Self::Param, value: &PyObject) {
//         match param {
//             DatePickerParam::Label      => set_t_value(&mut self.label, value, "DatePickerParam::Label"),
//             DatePickerParam::Padding    => set_t_value(&mut self.padding, value, "DatePickerParam::Padding"),
//             DatePickerParam::SizeFactor => set_t_value(&mut self.size_factor, value, "DatePickerParam::SizeFactor"),
//             DatePickerParam::Show       => set_t_value(&mut self.show, value, "DatePickerParam::Show"),
//         }
//     }
// }
