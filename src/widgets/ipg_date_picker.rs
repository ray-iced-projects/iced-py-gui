//! ipg_date_picker
use std::collections::HashMap;

use crate::app::{Message, self};
use crate::graphics::BOOTSTRAP_FONT;
use crate::state::IpgWidgets;
use crate::widgets::ipg_button::IpgButtonStyleStandard;
use crate::IpgState;
use super::callbacks::invoke_callback_with_args;
use crate::widgets::ipg_button::IpgButtonStyle;
use crate::py_api::helpers::{DATE_FORMATS, DAYS, 
    MONTH_NAMES, WEEKDAYS, get_padding, format_date};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,
    set_bool, set_opt_f32, set_opt_string, set_opt_vec_f32,
};
use iced::advanced::graphics::core::Element;
use iced::widget::{button, text};
use iced::{Length, Renderer, Theme};
use iced::alignment::{self, Alignment};
use iced::widget::{Button, Column, Container, PickList, 
    Row, Space, Text};

use chrono::prelude::*;
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub enum DPMessage {
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

#[derive(Debug, Clone, Default)]
pub struct IpgDatePicker {
    pub id: usize,
    pub parent_id: String,
    pub label: Option<String>,
    pub size_factor: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub show: bool,
    pub show_calendar: Option<bool>,
    pub is_submitted: bool,
    pub button_style_standard: Option<IpgButtonStyleStandard>,
    pub button_style_id: Option<usize>,

    // internal to the app
    pub selected_format: String,
    pub selected_year: i32,
    pub selected_month_index: usize,
    pub selected_day: usize,
    pub selected_date: String,

    pub show_width: f32,
    pub show_height: f32,
    pub hide_width: Option<Length>,
    pub hide_height: Option<Length>,
}

impl IpgDatePicker {
    pub fn new( 
        id: usize,
        parent_id: String,
        label: Option<String>,
        size_factor: Option<f32>,
        padding: Option<Vec<f32>>,
        show: bool,
        show_calendar: Option<bool>,
        button_style_standard: Option<IpgButtonStyleStandard>,
        button_style_id: Option<usize>,
        ) -> Self {
        Self {
            id,
            parent_id,
            label,
            size_factor,
            padding,
            show,
            show_calendar,

            selected_format: "YYYY-mm-dd".to_string(),
            selected_year: Utc::now().year(),
            selected_month_index: Utc::now().month() as usize,
            selected_day: Utc::now().day() as usize,
            selected_date: "".to_string(),

            show_width: 145.0,
            show_height: 180.0,
            hide_width: Some(Length::Shrink),
            hide_height: Some(Length::Shrink),
            is_submitted: false,
            button_style_standard,
            button_style_id,
        }
    }

    /// Get the selected month name from the index
    pub fn selected_month(&self) -> &'static str {
        MONTH_NAMES[self.selected_month_index]
    }

    /// Update selected_date from current format/year/month/day
    pub fn update_selected_date(&mut self) {
        self.selected_date = format_date(
            self.selected_format.clone(),
            self.selected_year,
            self.selected_month_index,
            self.selected_day
        );
    }

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self, 
        widgets: &HashMap<usize, IpgWidgets>,
        ) -> Option<Element<'a, Message, Theme, Renderer>> {
        
        if !self.show {
            return None;
        }

        let btn_style_opt = 
            self.lookup(widgets, self.button_style_id)
                .and_then(IpgWidgets::as_button_style).cloned();


        if self.show_calendar == Some(false) {
            let cal_show_btn: Element<'a, Message, Theme, Renderer> = 
                calendar_show_button(self, btn_style_opt);
            return Some(cal_show_btn)
        }

        let size = self.size_factor.unwrap_or(1.0).max(1.0);
        
        let col_content: Element<Message, Theme, Renderer> =
            Column::with_children(vec![
                create_first_row_arrows(self.id, 
                    self.selected_month(), 
                    self.selected_month_index, 
                    self.selected_year,
                    size),
                
                // Column titles S M T W T F S
                Row::with_children(
                    vec![Space::new().width(7.0*size).into(), 
                    create_day_row(size)]
                ).width(Length::Fill).into(),
                
                // days of the month
                Row::with_children(
                    vec![Space::new().width(5.0*size).into(), 
                    get_calendar_days(self.id, 
                        self.selected_year,
                        self.selected_month_index,
                        self.selected_day,
                        size),
                    ]).width(Length::Fill).into(),

                // close btn and format picklist
                Row::with_children(
                    vec![Space::new().width(5.0*size).into(), 
                    create_select_row(
                        self.id, 
                        self.selected_format.clone(), 
                        size),
                    ]).width(Length::Fill).into(),
                
                // bottom submit btn and selected date, if any
                Row::with_children(
                    vec![Space::new().width(5.0*size).into(),
                        create_submit_row(
                            self.id, 
                            size, 
                            self.selected_date.clone())
                    ]).width(Length::Fill).into(),
                
            ])
            .spacing(3.0*size)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into();

        let width = Length::Fixed(self.show_width * size);
        let height = Length::Fixed(self.show_height * size);

        Some(Container::new(col_content)
                .width(width)
                .height(height)
            // .style(theme::Container::Box)
            .into())

    }
}

fn icon(unicode: char, size: f32) -> Text<'static> {
    Text::new(unicode.to_string())
        .font(BOOTSTRAP_FONT)
        .size(size)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
}

fn left_arrow_icon(size: f32) -> Text<'static> {
    icon('\u{f12c}', size)
}

fn right_arrow_icon(size: f32) -> Text<'static> {
    icon('\u{f135}', size)
}

fn arrow_button(id: usize, icon: Text<'static>, message: DPMessage, width: f32, height: f32) -> Element<'static, Message, Theme, Renderer> {
    let btn: Element<DPMessage, Theme, Renderer> = 
        Button::new(icon)
            .on_press(message)
            .width(width)
            .height(height)
            .padding(0)
            .style(move |theme, status| button::text(theme, status))
            .into();
    btn.map(move |msg| Message::DatePicker(id, msg))
}

fn get_days_of_month(year: i32, month: u32) -> i64 {

    let mut mon: u32 = month;
    let mut yr: i32 = year;

    if month == 12 {
        mon = 1;
        yr += 1;
    } else {
        mon += 1;
    }

    let start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(yr, mon, 1).unwrap();
    let since = NaiveDate::signed_duration_since;
   
    since(end, start).num_days()
    
}

fn calendar_show_button<'a>(
        dp: &'a IpgDatePicker, 
        btn_style: Option<IpgButtonStyle>,
    ) -> Element<'a, Message, Theme, Renderer> {
    
    let label = if let Some(lb) = dp.label.clone() {
        lb
    } else { "Calendar".to_string() };

    let show_btn: Element<DPMessage, Theme, Renderer> = 
        Button::new(text(label))
            .on_press(DPMessage::ShowModal)
            .height(Length::Shrink)
            .width(Length::Shrink)
            .style(move|theme, status|
                if let Some(st) = &btn_style {
                        st.to_iced(theme, status, &dp.button_style_standard)
                    } else {
                       match &dp.button_style_standard {
                            Some(std) => std.to_iced(theme, status),
                            None => button::primary(theme, status),
                        }
                    }
            )
            .into();

    let s_btn = 
        show_btn.map(move |message| Message::DatePicker(dp.id, message));

    Container::new(s_btn)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .padding(get_padding(&dp.padding)).into()
                    
}


fn create_first_row_arrows(
    id: usize, 
    selected_month: &str, 
    selected_month_index: usize, 
    selected_year: i32,
    size_factor: f32) 
    -> Element<'static, Message, Theme, Renderer> 
{
    let w = 18.0 * size_factor;
    let h = 15.0 * size_factor;
    let arrow_size = 11.0 * size_factor;
    let month_container_width = 45.0 * size_factor;
    let text_size = 9.0 * size_factor;

    let selected_month_cont: Element<'static, Message, Theme, Renderer> = 
        Container::new(Text::new(selected_month.to_owned()).size(text_size))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .width(Length::Fixed(month_container_width))
            .into();

    Row::with_children(vec![
        arrow_button(id, left_arrow_icon(arrow_size), DPMessage::MonthLeftPressed(selected_month_index), w, h),
        selected_month_cont,
        arrow_button(id, right_arrow_icon(arrow_size), DPMessage::MonthRightPressed(selected_month_index), w, h),
        arrow_button(id, left_arrow_icon(arrow_size), DPMessage::YearLeftPressed, w, h),
        Text::new(selected_year.to_string()).size(text_size).into(),
        arrow_button(id, right_arrow_icon(arrow_size), DPMessage::YearRightPressed, w, h),
    ])
    .spacing(2)
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()

}


fn get_calendar_days(
        id: usize, selected_year: i32, 
        selected_month_index: usize, 
        selected_day: usize,
        size_factor: f32,
    ) -> Element<'static, Message, Theme, Renderer> 
{

    let days = get_days_of_month(selected_year, selected_month_index as u32) as f32;

    let first_day_index = NaiveDate::from_ymd_opt(selected_year, selected_month_index as u32, 1).unwrap().num_days_from_ce();
    let first_day = NaiveDate::from_ymd_opt(selected_year, selected_month_index as u32, 1).unwrap().weekday();
    
    let mut weeks: usize = (days/7.0).ceil() as usize;
    if weeks as f32 * 7.0 < days + first_day_index as f32 {
        weeks += 1;
    } 

    let mut calendar_days: Vec<Element<'static, Message, Theme, Renderer>> = vec![];

    let mut start_weekday = false;
    let mut start_correction = 0;

    for week in 0..weeks {

        let mut row: Vec<Element<'static, Message, Theme, Renderer>> = vec![];

        for d in 1..=7 {
            let mut day = week * 7 + d - start_correction;
            if !start_weekday {
            
                if *WEEKDAYS[d-1] == first_day.to_string() {
                    start_weekday = true;
                    start_correction = d-1;
                    day -= start_correction;
                    
                } else {
                    row.push(
                        Space::new()
                            .width(15.0*size_factor)
                            .height(15.0*size_factor)
                            .into());
                }
            }
            if day <= days as usize && start_weekday {

                let content = 
                    Text::new(day.to_string())
                        .size(8.0*size_factor)
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center);
                
                let btn: Element<DPMessage, Theme, Renderer> = 
                    Button::new(content)
                        .on_press(DPMessage::DayPressed(day)
                        )
                        .height(15.0*size_factor)
                        .width(15.0*size_factor)
                        .padding(0)
                        .style(move|theme: &Theme, status| {
                                if day == selected_day {
                                    button::success(theme, status)
                                } else {
                                    button::primary(theme, status)
                                }}
                            )
                        .into();
                row.push(btn.map(move |message| Message::DatePicker(id, message)));
                
            }
        }
        
        calendar_days.push(Row::with_children(row).spacing(5.0*size_factor).into());
    
    }

    Column::with_children(calendar_days)
        .align_x(Alignment::Start)
        .width(Length::Fill)
        .padding(0)
        .into()
    
}


fn create_day_row(size_factor: f32) -> Element<'static, Message, Theme, Renderer> {
    
    let days = 
        DAYS.into_iter().map(|x| 
            Text::new(x.to_string())
            .size(8.0*size_factor)
            .into())
            .collect::<Vec<Element<'static, Message, Theme, Renderer>>>();

    Row::with_children(days).spacing(15.0*size_factor).width(Length::Fill).into()
}

fn create_select_row(
    id: usize, 
    selected_format: String,
    size_factor: f32,
    ) -> Element<'static, Message, Theme, Renderer> 
{

    let date_formats = 
        DATE_FORMATS
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

    let close_text = 
        Text::new("Close").size(10.0*size_factor);
    
    let cl_button: Element<DPMessage, Theme, Renderer> = 
        Button::new(close_text)
            .on_press(DPMessage::HideModal)
            .style(move|theme, status| 
                button::primary(theme, status)
            )
            .into();
                                
    let close_button: Element<Message, Theme, Renderer> = 
        cl_button.map(move |message| Message::DatePicker(id, message));

    let picklist: Element<DPMessage, Theme, Renderer> = 
        PickList::new(
            date_formats,
            Some(selected_format),
            DPMessage::DatePickerFormat
        )
        .text_size(8.0*size_factor)
        .placeholder("Choose format...")
        .into();
    
    let mapped_pl: Element<Message, Theme, Renderer> = 
                    picklist.map(move |message| app::Message::DatePicker(id, message));

    Row::with_children(vec![
        Row::with_children(vec![
            close_button,
            mapped_pl,    
        ]).width(Length::Fill)
        .spacing(10.0*size_factor)
        .into(),

    ]).into()  
}


fn create_submit_row(id: usize, size_factor: f32, selected_date: String) -> Element<'static, Message, Theme, Renderer> 
{
    let submit_text: Element<DPMessage, Theme, Renderer> = Text::new("Submit").size(10.0*size_factor).into();

    let submit_btn: Element<DPMessage, Theme, Renderer> = 
        Button::new(submit_text)
            .on_press(DPMessage::OnSubmit)
            .style(move|theme, status| button::primary(theme, status))
            .into();
    
    let submit_btn_mapped: Element<Message, Theme, Renderer> = 
                                submit_btn.map(move |message| app::Message::DatePicker(id, message));

    Row::new()
        .push(submit_btn_mapped)
        .push(Text::new(selected_date).size(10.0*size_factor))
        .width(Length::Fill)
        .spacing(10.0*size_factor)
        .into()
}


pub fn date_picker_update(state: &mut IpgState, id: usize, message: DPMessage) {
    // Get the date picker widget
    let dp = match state.widgets.get_mut(&id) {
        Some(IpgWidgets::IpgDatePicker(dp)) => dp,
        _ => return,
    };

    match message {
        DPMessage::ShowModal => {
            dp.show_calendar = Some(true);
            dp.is_submitted = false;
        }
        DPMessage::HideModal => {
            dp.show_calendar = Some(false);
        }
        DPMessage::DayPressed(day) => {
            dp.selected_day = day;
            dp.update_selected_date();
        }
        DPMessage::DatePickerFormat(date_fmt) => {
            dp.selected_format = date_fmt;
            dp.update_selected_date();
        }
        DPMessage::MonthRightPressed(index) => {
            dp.selected_month_index = if index == 12 { 1 } else { index + 1 };
            dp.is_submitted = false;
            dp.update_selected_date();
        }
        DPMessage::MonthLeftPressed(index) => {
            dp.selected_month_index = if index == 1 { 12 } else { index - 1 };
            dp.is_submitted = false;
            dp.update_selected_date();
        }
        DPMessage::YearRightPressed => {
            dp.selected_year += 1;
            dp.is_submitted = false;
            dp.update_selected_date();
        }
        DPMessage::YearLeftPressed => {
            dp.selected_year -= 1;
            dp.is_submitted = false;
            dp.update_selected_date();
        }
        DPMessage::OnSubmit => {
            dp.is_submitted = true;
            let selected_date = dp.selected_date.clone();
            invoke_callback_with_args(id, "on_submit", "DatePicker", selected_date);
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq)]
pub enum IpgDatePickerParam {
    Label,
    Padding,
    SizeFactor,
    Show,
}



// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgDatePicker {
    type Param = IpgDatePickerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgDatePickerParam::Label      => set_opt_string(&mut self.label, value, name),
            IpgDatePickerParam::Padding    => set_opt_vec_f32(&mut self.padding, value, name),
            IpgDatePickerParam::SizeFactor => set_opt_f32(&mut self.size_factor, value, name),
            IpgDatePickerParam::Show       => set_bool(&mut self.show, value, name),
        }
    }
}