//!Helpers
use iced::{Border, Element, Length, Padding, Pixels, Point, Rectangle, Theme};
use iced::theme::palette;
use iced::widget::{canvas, combo_box};
use iced::widget::{button, container, column, row, space, text, Checkbox, TextInput};

use crate::app::Message;


pub fn get_content(id: usize, size: Option<f32>, year: i32, month: &String, index: usize) -> Element<'static, Message> {
    let size = size.unwrap_or(1.0).max(1.0);
    let content =
        column(vec![
            create_first_row_arrows(id, 
                MONTH_NAMES[month.into()], 
                index, 
                year,
                size),
            
            // Column titles S M T W T F S
            row(
                vec![space().width(7.0*size).into(), 
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

    container(content)
            .width(width)
            .height(height)
            .style(|theme| {
                container::bordered_box(theme)
            })
        .into()

}

const MONTH_NAMES: [&str; 13] = ["", "January", "Feburary", "March", 
                                        "April", "May", "June", "July", 
                                        "August", "September", "October", 
                                        "November", "December"];
                                
const DATE_FORMATS: [&str; 3] = ["mm-dd-YYYY", "YYYY-mm-dd", "mm-dd-YY"];
const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const DAYS: [&str; 7] = ["S", "M", "T", "W", "T", "F", "S"];

fn format_date(format: String, year: i32, month: usize, day: usize) -> String {

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

fn convert_to_len_two(value: usize) -> String {

    if value < 10 {
        "0".to_string() + &value.to_string() 
    } else {
        value.to_string()
    }
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
            .padding(2.0)
            .style(move|theme, status| 
                button::primary(theme, status)
            )
            .into();
                                
    let close_button: Element<Message, Theme, Renderer> = 
        cl_button.map(move |message| Message::DatePicker(id, message));

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
            .padding(3.0)
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
