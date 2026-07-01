//!Helpers
use chrono::{NaiveDate, Datelike};
use iced::advanced::widget::Text;
use iced::{Element, Length, Renderer, Theme, alignment};

use iced::widget::{Button, Container, PickList, Row, Space};
use iced::widget::{button, container, column, row, space, text};

use crate::widgets::ipg_date_picker::{DPMessage, DpContent};


pub fn get_content(size: Option<f32>, dpc: &DpContent) -> Element<'static, DPMessage, Theme, Renderer> {
    let size = size.unwrap_or(1.0).max(1.0);
    
    let content =
        column(vec![
            create_first_row_arrows(size, &dpc),
            
            // Column titles S M T W T F S
            row(
                vec![space().width(7.0*size).into(), 
                create_day_row(size)]
            ).width(Length::Fill).into(),
            
            // days of the month
            row(
                vec![Space::new().width(5.0*size).into(), 
                get_calendar_days(size, &dpc),
                ]).width(Length::Fill).into(),

            // close btn and format picklist
            row(
                vec![Space::new().width(5.0*size).into(), 
                create_select_row(
                    dpc.selected_format.clone(), 
                    size),
                ]).width(Length::Fill).into(),
            
            // bottom submit btn and selected date, if any
            row(
                vec![Space::new().width(5.0*size).into(),
                create_submit_row(
                        size, 
                        dpc.selected_date.clone())
                ]).width(Length::Fill).into(),
            
        ])
        .spacing(3.0*size)
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(iced::Alignment::Center);

    let width = Length::Fixed(dpc.show_width.clone() * size);
    let height = Length::Fixed(dpc.show_height.clone() * size);

    let cont  = container(content)
            .width(width)
            .height(height)
            .style(|theme| {
                container::bordered_box(theme)
            });

    cont.into()

}

const MONTH_NAMES: [&str; 13] = ["", "January", "Feburary", "March", 
                                        "April", "May", "June", "July", 
                                        "August", "September", "October", 
                                        "November", "December"];
                                
const DATE_FORMATS: [&str; 3] = ["mm-dd-YYYY", "YYYY-mm-dd", "mm-dd-YY"];
const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const DAYS: [&str; 7] = ["S", "M", "T", "W", "T", "F", "S"];

pub fn format_date(format: &String, year: i32, month: usize, day: usize) -> String {

    match format.as_str() {
        "YYYY-mm-dd" => {
            let mon_str = convert_to_len_two(month);
            let day_str = convert_to_len_two(day);
            format!("{}-{}-{}", year, mon_str, day_str)
        },
        "mm-dd-YYYY" => {
            let mon_str = convert_to_len_two(month);
            let day_str = convert_to_len_two(day);
            format!("{}-{}-{}", mon_str, day_str, year)
        },
        "mm-dd-YY" => {
            let mon_str = convert_to_len_two(month);
            let day_str = convert_to_len_two(day);
            let s = year.to_string();
            format!("{}-{}-{}", mon_str, day_str, &s[2..4])
        },
        _ => panic!("Calendar Date format {} not found", format)
    }
}

pub fn convert_to_len_two(value: usize) -> String {

    if value < 10 {
        "0".to_string() + &value.to_string() 
    } else {
        value.to_string()
    }
}

fn create_first_row_arrows(
    size_factor: f32,
    dpc: &DpContent) 
    -> Element<'static, DPMessage, Theme, Renderer> 
{
    let w = 18.0 * size_factor;
    let h = 15.0 * size_factor;
    let arrow_size = 11.0 * size_factor;
    let month_container_width = 45.0 * size_factor;
    let text_size = 9.0 * size_factor;
    let selected_month = MONTH_NAMES[dpc.selected_month_index];

    let selected_month_cont = 
        Container::new(text(selected_month.to_owned()).size(text_size))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .width(Length::Fixed(month_container_width))
            .into();

    let f_row: Element<DPMessage, Theme, Renderer> = row(vec![
        arrow_button(left_arrow_icon(arrow_size), DPMessage::MonthLeftPressed(dpc.selected_month_index), w, h),
        selected_month_cont,
        arrow_button(right_arrow_icon(arrow_size), DPMessage::MonthRightPressed(dpc.selected_month_index), w, h),
        arrow_button(left_arrow_icon(arrow_size), DPMessage::YearLeftPressed, w, h),
        text(dpc.selected_year).size(text_size).into(),
        arrow_button(right_arrow_icon(arrow_size), DPMessage::YearRightPressed, w, h),
    ])
    .spacing(2)
    .align_y(iced::Alignment::Center)
    .width(Length::Fill)
    .into();

    f_row

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


fn get_calendar_days(
        size_factor: f32,
        dpc: &DpContent,
    ) -> Element<'static, DPMessage, Theme, Renderer> 
{
    
    let selected_year = dpc.selected_year;
    let selected_month_index = dpc.selected_month_index;
    let selected_day = dpc.selected_day;

    let days = get_days_of_month(selected_year, selected_month_index as u32) as f32;

    let first_day_index = NaiveDate::from_ymd_opt(selected_year, selected_month_index as u32, 1).unwrap().num_days_from_ce();
    let first_day = NaiveDate::from_ymd_opt(selected_year, selected_month_index as u32, 1).unwrap().weekday();
    
    let mut weeks: usize = (days/7.0).ceil() as usize;
    if weeks as f32 * 7.0 < days + first_day_index as f32 {
        weeks += 1;
    } 

    let mut calendar_days: Vec<Element<'static, DPMessage, Theme, Renderer>> = vec![];

    let mut start_weekday = false;
    let mut start_correction = 0;

    for week in 0..weeks {

        let mut cal_row: Vec<Element<'static, DPMessage, Theme, Renderer>> = vec![];

        for d in 1..=7 {
            let mut day = week * 7 + d - start_correction;
            if !start_weekday {
            
                if *WEEKDAYS[d-1] == first_day.to_string() {
                    start_weekday = true;
                    start_correction = d-1;
                    day -= start_correction;
                    
                } else {
                    cal_row.push(
                        Space::new()
                            .width(15.0*size_factor)
                            .height(15.0*size_factor)
                            .into());
                }
            }
            if day <= days as usize && start_weekday {

                let content = 
                    text(day.to_string())
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

                cal_row.push(btn);

            }
        }
        
        calendar_days.push(Row::with_children(cal_row).spacing(5.0*size_factor).into());
    
    }

    let col = column(calendar_days)
        .align_x(iced::Alignment::Start)
        .width(Length::Fill)
        .padding(0);

    let col: Element<'static, DPMessage, Theme, Renderer> = col.into();
    col
}


fn create_day_row(size_factor: f32) -> Element<'static, DPMessage, Theme, Renderer> {
    
    let days = 
        DAYS.into_iter().map(|x| 
            text(x.to_string())
            .size(8.0*size_factor)
            .into())
            .collect::<Vec<Element<'static, DPMessage, Theme, Renderer>>>();

    Row::with_children(days).spacing(15.0*size_factor).width(Length::Fill).into()
}

fn create_select_row(
    selected_format: String,
    size_factor: f32,
    ) -> Element<'static, DPMessage, Theme, Renderer> 
{

    let date_formats = 
        DATE_FORMATS
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

    let close_text = 
        text("Close").size(10.0*size_factor);
    
    let close_button: Element<DPMessage, Theme, Renderer> = 
        Button::new(close_text)
            .on_press(DPMessage::HideModal)
            .padding(2.0)
            .style(move|theme, status| 
                button::primary(theme, status)
            )
            .into();
                                
    let picklist: Element<DPMessage, Theme, Renderer> = 
        PickList::new(
            Some(selected_format),
            date_formats,
            |s: &String| s.clone(),
        )
        .on_select(DPMessage::DatePickerFormat)
        .text_size(8.0*size_factor)
        .placeholder("Choose format...")
        .into();
    
    Row::with_children(vec![
        Row::with_children(vec![
            close_button,
            picklist,    
        ]).width(Length::Fill)
        .spacing(10.0*size_factor)
        .into(),

    ]).into()  
}


fn create_submit_row(size_factor: f32, selected_date: String) -> Element<'static, DPMessage, Theme, Renderer> 
{
    let submit_text: Element<DPMessage, Theme, Renderer> = text("Submit").size(10.0*size_factor).into();
    let clip_text: Element<DPMessage, Theme, Renderer> = text("ClipBoard").size(10.0*size_factor).into();


    let submit_btn: Element<DPMessage, Theme, Renderer> = 
        Button::new(submit_text)
            .padding(3.0)
            .on_press(DPMessage::OnSubmit)
            .style(move|theme, status| button::primary(theme, status))
            .into();

    let clip_btn: Element<DPMessage, Theme, Renderer> = 
        Button::new(clip_text)
            .padding(3.0)
            .on_press(DPMessage::CopyToClipBoard)
            .style(move|theme, status| button::primary(theme, status))
            .into();
    
    Row::new()
        .push(submit_btn)
        .push(text(selected_date).size(10.0*size_factor))
        .push(clip_btn)
        .width(Length::Fill)
        .spacing(10.0*size_factor)
        .wrap()
        .into()
}

use crate::graphics::BOOTSTRAP_FONT;
fn icon(unicode: char, size: f32) -> Text<'static, Theme, Renderer> {
    text(unicode.to_string())
        .font(BOOTSTRAP_FONT)
        .size(size)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
}

fn left_arrow_icon(size: f32) -> Text<'static, Theme, Renderer> {
    icon('\u{f12c}', size)
}

fn right_arrow_icon(size: f32) -> Text<'static, Theme, Renderer> {
    icon('\u{f135}', size)
}

fn arrow_button(icon: Text<'static, Theme, Renderer>, message: DPMessage, width: f32, height: f32) 
    -> Element<'static, DPMessage, Theme, Renderer> {
    
        Button::new(icon)
            .on_press(message)
            .width(width)
            .height(height)
            .padding(0)
            .style(move |theme, status| button::text(theme, status))
            .into()
}
