//! ipg_date_picker
use crate::app::{Message, self};
use crate::graphics::BOOTSTRAP_FONT;
use crate::state::{IpgWidgets, access_user_data2};
use crate::widgets::ipg_button::IpgButtonStyleStandard;
use crate::{access_callbacks, access_user_data1, IpgState};
use super::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn};
use crate::widgets::ipg_button::{self, IpgButtonStyle};
use crate::py_api::helpers::{DATE_FORMATS, DAYS, 
    MONTH_NAMES, WEEKDAYS, get_padding};
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
use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;


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
    pub selected_month: String,
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
            selected_month: MONTH_NAMES[Utc::now().month() as usize].to_string(),
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
}


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

pub fn construct_date_picker<'a>(
    dp: &'a IpgDatePicker, 
    btn_style_opt: Option<&'a IpgWidgets>) 
    -> Option<Element<'a, Message, Theme, Renderer>> {
    
    let btn_style  = btn_style_opt.and_then(IpgWidgets::as_button_style).cloned();

    if !dp.show {
        return None;
    }

    if dp.show_calendar == Some(false) {
        let cal_show_btn: Element<'a, Message, Theme, Renderer> = 
            calendar_show_button(dp, btn_style);
        return Some(cal_show_btn)
    }

    let size = dp.size_factor.unwrap_or(1.0).max(1.0);
    
    let col_content: Element<Message, Theme, Renderer> =
        Column::with_children(vec![
            create_first_row_arrows(dp.id, 
                &dp.selected_month, 
                dp.selected_month_index, 
                dp.selected_year,
                size),
            
            // Column titles S M T W T F S
            Row::with_children(
                vec![Space::new().width(7.0*size).into(), 
                create_day_row(size)]
            ).width(Length::Fill).into(),
            
            // days of the month
            Row::with_children(
                vec![Space::new().width(5.0*size).into(), 
                get_calendar_days(dp.id, 
                    dp.selected_year,
                    dp.selected_month_index,
                    dp.selected_day,
                    size),
                ]).width(Length::Fill).into(),

            // close btn and format picklist
            Row::with_children(
                vec![Space::new().width(5.0*size).into(), 
                create_select_row(
                    dp.id, 
                    dp.selected_format.clone(), 
                    size),
                ]).width(Length::Fill).into(),
            
            // bottom submit btn and selected date, if any
            Row::with_children(
                vec![Space::new().width(5.0*size).into(),
                    create_submit_row(
                        dp.id, 
                        size, 
                        dp.selected_date.clone())
                ]).width(Length::Fill).into(),
            
        ])
        .spacing(3.0*size)
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(Alignment::Center)
        .into();

    let width = Length::Fixed(dp.show_width * size);
    let height = Length::Fixed(dp.show_height * size);

    Some(Container::new(col_content)
            .width(width)
            .height(height)
        // .style(theme::Container::Box)
        .into())

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

fn calendar_show_button<'a>(dp: &'a IpgDatePicker, 
                            btn_style: Option<IpgButtonStyle>) 
                            -> Element<'a, Message, Theme, Renderer> {
    
    let label = if let Some(lb) = dp.label.clone() {
        lb
    } else {
        "Calendar".to_string()
    };

    let show_btn: Element<DPMessage, Theme, Renderer> = 
                    Button::new(text(label))
                                    .on_press(DPMessage::ShowModal)
                                    .height(Length::Shrink)
                                    .width(Length::Shrink)
                                    .style(move|theme, status|
                                        ipg_button::get_styling(theme, status,
                                            &btn_style, 
                                            &dp.button_style_standard
                                        ))
                                    .into();

    let s_btn: Element<'a, Message, Theme, Renderer> = 
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
    -> Element<'_, Message, Theme, Renderer> 
{

    let btn_arrow_width = 18.0 * size_factor;
    let btn_arrow_height = 15.0 * size_factor;
    let arrow_size = 11.0 * size_factor;

    // sets a width for all month names which prevents shifing when month names differ.
    let month_container_width = 45.0 * size_factor; 
    let text_size = 9.0 * size_factor;
    let padding = 0;

    let left_btn: Element<DPMessage, Theme, Renderer> = 
        Button::new(left_arrow_icon(arrow_size))
                .on_press(DPMessage::MonthLeftPressed(selected_month_index))
                .width(btn_arrow_width)
                .height(btn_arrow_height)
                .padding(padding)
                .style(move|theme, status| button::text(theme, status))
                .into();
    let month_left_btn: Element<'_, Message, Theme, Renderer> = 
        left_btn.map(move |message| Message::DatePicker(id, message));

    let right_btn: Element<DPMessage, Theme, Renderer> = 
                Button::new(right_arrow_icon(arrow_size))
                        .on_press(DPMessage::MonthRightPressed(selected_month_index))
                        .width(btn_arrow_width)
                        .height(btn_arrow_height)
                        .padding(padding)
                        .style(move|theme, status| button::text(theme, status))
                        .into();
    let month_right_btn: Element<'_, Message, Theme, Renderer> = 
                right_btn.map(move |message| Message::DatePicker(id, message));

    let left_btn: Element<DPMessage, Theme, Renderer> = 
                Button::new(left_arrow_icon(arrow_size))
                        .on_press(DPMessage::YearLeftPressed)
                        .width(btn_arrow_width)
                        .height(btn_arrow_height)
                        .padding(padding)
                        .style(move|theme, status| button::text(theme, status))
                        .into();
    let year_left_btn: Element<'_, Message, Theme, Renderer> = 
                left_btn.map(move |message| Message::DatePicker(id, message));

    let right_btn: Element<DPMessage, Theme, Renderer> = 
                Button::new(right_arrow_icon(arrow_size))
                        .on_press(DPMessage::YearRightPressed)
                        .width(btn_arrow_width)
                        .height(btn_arrow_height)
                        .padding(padding)
                        .style(move|theme, status| button::text(theme, status))
                        .into();
    let year_right_btn: Element<'_, Message, Theme, Renderer> = 
                right_btn.map(move |message| Message::DatePicker(id, message));

    let selected_month_cont: Element<Message, Theme, Renderer> = 
            Container::new(Text::new(selected_month.to_owned())
                        .size(text_size))
                        .align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Center)
                        .width(Length::Fixed(month_container_width))
                        .into();

    Row::with_children(vec![
        Row::with_children(vec![
                            month_left_btn,
                            selected_month_cont, 
                            month_right_btn,
                            // --------------------------------------
                            year_left_btn,
                            
                            Text::new(selected_year.to_string())
                                        .size(text_size)
                                        .into(),
                            year_right_btn,
                        ])
                        .spacing(2)
                        .align_y(Alignment::Center).into(),
    ])
    .align_y(Alignment::Center)
    .width(Length::Fill)
    .into()

}


fn get_calendar_days(id: usize, selected_year: i32, 
                        selected_month_index: usize, 
                        selected_day: usize,
                        size_factor: f32) 
                        -> Element<'static, Message, Theme, Renderer> 
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
    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};

    match message {
        DPMessage::ShowModal => {
            // Non callback just sending the values.
            wci.show = Some(true);
            wci.is_submitted = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::HideModal => {
            // Non callback just sending the values.
            wci.show = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::DayPressed(day) => {
            // Non callback just sending the values.
            wci.selected_day = Some(day);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::DatePickerFormat(date_format) => {
            // Non callback just sending the values.
            wci.date_format = Some(date_format);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::MonthRightPressed(index) => {
            // Non callback just sending the values.
            wci.index = Some(index);
            wci.increment_value = Some(1);
            wci.is_submitted = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::MonthLeftPressed(index) => {
            // Non callback just sending the values.
            wci.index = Some(index);
            wci.increment_value = Some(-1);
            wci.is_submitted = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::YearRightPressed => {
            // Non callback just sending the values.
            wci.selected_year = Some(1);
            wci.is_submitted = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::YearLeftPressed => {
            // Non callback just sending the values.
            wci.selected_year = Some(-1);
            wci.is_submitted = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::OnSubmit => {
            wci.is_submitted = Some(true);
            let wco = set_or_get_widget_callback_data(state, wci);

            process_callback(id, "on_submit".to_string(), wco.selected_date);
        }
    }
}


pub fn process_callback(id: usize, event_name: String, selected_date: Option<String>) {
    let ud1 = access_user_data1();
    let app_cbs = access_callbacks();

    // Retrieve the callback
    let callback = match app_cbs.callbacks.get(&(id, event_name)) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_cbs);

    // Check user data from ud1
    if let Some(user_data) = ud1.user_data.get(&id) {
        Python::attach(|py| {
            if let Some(date) = &selected_date {
                if let Err(err) = callback.call1(py, (id, date.clone(), user_data)) {
                    panic!("DatePicker callback error: {err}");
                }
            } else {
                if let Err(err) = callback.call1(py, (id, user_data)) {
                    panic!("DatePicker callback error: {err}");
                }
            }
        });
        drop(ud1); // Drop ud1 before processing ud2
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Check user data from ud2
    let ud2 = access_user_data2();
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::attach(|py| {
            if let Some(date) = &selected_date {
                if let Err(err) = callback.call1(py, (id, date.clone(), user_data)) {
                    panic!("DatePicker callback error: {err}");
                }
            } else {
                if let Err(err) = callback.call1(py, (id, user_data)) {
                    panic!("DatePicker callback error: {err}");
                }
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only id and selected_date
    Python::attach(|py| {
        if let Some(date) = selected_date {
            if let Err(err) = callback.call1(py, (id, date)) {
                panic!("DatePicker callback error: {err}");
            }
        } else {
            if let Err(err) = callback.call1(py, (id,)) {
                panic!("DatePicker callback error: {err}");
            }
        }
    });
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