//! ipg_color_picker
use crate::IpgState;
use crate::ipg_widgets::ipg_date_picker::lib::helpers::{convert_to_len_two, format_date, get_content};
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

use chrono::prelude::*;


#[derive(Debug, Clone)]
pub struct DatePicker {
    pub id: usize,
    pub opened: bool,
    pub size_factor: Option<f32>,
    pub gap: Option<u32>,
    pub position: Position,
    pub snap_within_viewport: Option<bool>,
    pub dp_content: DpContent,
}

impl DatePicker {

    pub fn construct<'a>(
        &'a self,
        mut opener_content: Vec<Element<'a, Message>>,
        ) -> Option<Element<'a, Message>> {

        let btn = if opener_content.is_empty() {
            return None;
        } else {
            opener_content.remove(0).map(|_| DatePikMessage::Noop)
        };

        let content = get_content(self.size_factor, &self.dp_content)
            .map(DatePikMessage::DatePicker);
        
        let dpk = DP::new(
            btn,
            content,
            self.dp_content.selected_date.clone(),
            self.position,
        )
        .opened(self.opened)
        .on_open(DatePikMessage::Opened)
        .gap(self.gap.unwrap_or(10))
        .style(container::rounded_box);

        let dpk: Element<'_, DatePikMessage> = dpk.into();
        Some(dpk.map(move |message| Message::DatePicker(self.id, message)))

    }

}

#[derive(Debug, Clone)]
pub struct DpContent {
    pub selected_format: String,
    pub selected_year: i32,
    pub selected_month_index:usize,
    pub selected_day: usize,
    pub selected_date: String,
    pub show_width: f32,
    pub show_height: f32,
}

impl DpContent {
    pub fn default() -> Self {
        let date = Utc::now().to_string();
        let date: Vec<&str> = date.split(" ").collect();
        Self {
            selected_format: "YYYY-mm-dd".to_string(),
            selected_year: Utc::now().year(),
            selected_month_index: Utc::now().month() as usize,
            selected_day: Utc::now().day() as usize,
            selected_date: date[0].to_string(),
            show_width: 145.0,
            show_height: 180.0,
        }
    } 
}

#[derive(Debug, Clone)]
pub enum DPMessage {
    CopyToClipBoard,
    DatePickerFormat(String),
    DayPressed(usize),
    HideModal,
    MonthLeftPressed(usize),
    MonthRightPressed(usize),
    OnSubmit,
    ShowModal,
    YearLeftPressed,
    YearRightPressed,
} 


#[derive(Debug, Clone)]
pub enum DatePikMessage {
    Noop,
    Opened(bool),
    DatePicker(DPMessage),
}

pub fn date_picker_callback(
    state: &mut IpgState, 
    id: usize, 
    message: DatePikMessage,
) -> Option<Task<Message>> {

    let dp = if let Some(Containers::DatePicker(dp)) = state.containers.get_mut(&id) {
        dp
    } else { panic!("Unable to get the DatePicker container using id{}", id)};
    
    let dpc = &dp.dp_content;
    
    match message {
        DatePikMessage::Noop => (),
        DatePikMessage::Opened(open) => {
            dp.opened = open;
            invoke_callback_with_args(id, "on_open", "DaterPicker", open,
                    "def cb(wid: int, opened: bool)");
            
        },
        DatePikMessage::DatePicker(dpmessage) => {
            match dpmessage {
                DPMessage::CopyToClipBoard => {
                    dp.opened = false;
                    let txt = dp.dp_content.selected_date.clone();
                    return Some(iced::clipboard::write(txt).discard());
                }
                DPMessage::DatePickerFormat(fmt) => {
                    let date = format_date(&fmt, dpc.selected_year, dpc.selected_month_index, dpc.selected_day);
                    dp.dp_content.selected_format = fmt;
                    dp.dp_content.selected_date = date;
                },
                DPMessage::DayPressed(day) => {
                    dp.dp_content.selected_date = insert_day(&dpc.selected_format, day, &dpc.selected_date);
                    dp.dp_content.selected_day = day;
                },
                DPMessage::HideModal => dp.opened = false,
                DPMessage::MonthLeftPressed(mut month) => {
                    month -= 1;
                    if month == 0 {
                        month = 12;
                        let selected_date = insert_month(&dpc.selected_format, month, &dpc.selected_date);
                        let selected_date = insert_year(&dpc.selected_format, &selected_date, true);
                        dp.dp_content.selected_date = insert_month(&dpc.selected_format, month, &selected_date);
                        dp.dp_content.selected_month_index = month;
                        dp.dp_content.selected_year -= 1;
                    } else {
                        dp.dp_content.selected_date = insert_month(&dpc.selected_format, month, &dpc.selected_date);
                        dp.dp_content.selected_month_index = month;
                    }
                },
                DPMessage::MonthRightPressed(mut month) => {
                    month += 1;
                    if month == 13 {
                        month = 1;
                        let selected_date = insert_month(&dpc.selected_format, month, &dpc.selected_date);
                        let selected_date = insert_year(&dpc.selected_format, &selected_date, false);
                        dp.dp_content.selected_date = insert_month(&dpc.selected_format, month, &selected_date);
                        dp.dp_content.selected_month_index = month;
                        dp.dp_content.selected_year += 1;
                    } else {
                        dp.dp_content.selected_date = insert_month(&dpc.selected_format, month, &dpc.selected_date);
                        dp.dp_content.selected_month_index = month;
                    }
                    
                },
                DPMessage::OnSubmit => {
                    dp.opened = false;
                    invoke_callback_with_args(id, "on_submit", "Calendar", 
                    dp.dp_content.selected_date.clone(), "def cb(wid: int, on_submit: str)");
                },
                DPMessage::ShowModal => dp.opened = true,
                DPMessage::YearLeftPressed => {
                    dp.dp_content.selected_date = insert_year(&dpc.selected_format, &dpc.selected_date, true);
                    dp.dp_content.selected_year -= 1;
                },
                DPMessage::YearRightPressed => {
                    dp.dp_content.selected_date = insert_year(&dpc.selected_format, &dpc.selected_date, false);
                    dp.dp_content.selected_year += 1;
                },
            }
        },
    }

    None
        
}

fn insert_day(format: &String, day: usize, date: &String) -> String {
    let day_str = convert_to_len_two(day);
    match format.as_str() {
        // YYYY-mm-dd  →  replace chars 8..10
        "YYYY-mm-dd" => format!("{}{}", &date[..8], day_str),
        // mm-dd-YYYY  →  replace chars 3..5
        "mm-dd-YYYY" => format!("{}{}{}", &date[..3], day_str, &date[5..]),
        // mm-dd-YY    →  replace chars 3..5
        "mm-dd-YY"   => format!("{}{}{}", &date[..3], day_str, &date[5..]),
        _ => panic!("Calendar Date format {} not found", format)
    }
}

fn insert_month(format: &String, month: usize, date: &String) -> String {
    let month_str = convert_to_len_two(month);
    match format.as_str() {
        // YYYY-mm-dd  →  replace chars 5..7
        "YYYY-mm-dd" => format!("{}{}{}", &date[..5], month_str, &date[7..]),
        // mm-dd-YYYY  →  replace chars 0..2
        "mm-dd-YYYY" => format!("{}{}", month_str, &date[2..]),
        // mm-dd-YY    →  replace chars 0..2
        "mm-dd-YY"   => format!("{}{}", month_str, &date[2..]),
        _ => panic!("Calendar Date format {} not found", format)
    }
}

fn insert_year(format: &String, date: &String, left: bool) -> String {
    let year: Vec<&str> = date.split("-").collect();
    let mut year_num = match year[0].parse::<i32>() {
        Ok(num) =>  num,
        Err(e) => panic!("Calendar: Failed to parse year: {}", e)
    };

    if left {
        year_num -= 1;
    } else {
        year_num += 1;
    }

    let year_str = year_num.to_string();

    match format.as_str() {
        // YYYY-mm-dd  →  replace chars 0..4 (4-digit year)
        "YYYY-mm-dd" => format!("{:04}{}", year_str, &date[4..]),
        // mm-dd-YYYY  →  replace chars 6..10 (4-digit year)
        "mm-dd-YYYY" => format!("{}{:04}", &date[..6], year_str),
        // mm-dd-YY    →  replace chars 6..8 (2-digit year)
        "mm-dd-YY"   => format!("{}{}", &date[..6], convert_to_len_two(year_num as usize % 100)),
        _ => panic!("Calendar: Date format {} not found", format)
    }
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
