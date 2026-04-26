//! helpers

use std::collections::HashMap;

use crate::access_state;

use iced::border::Radius;
use iced::{window, Length, Padding};


pub fn find_key_for_value(ids: HashMap<window::Id, usize>, value: usize) -> window::Id {
    let state = access_state();
    let map = &ids;
    let id = map.iter()
        .find_map(|(key, &val)| if val == value { Some(key) } else { None });
    
    match id {
        Some(id) => {
            let iced_id = *id;
            drop(state);
            iced_id
        },
        None => panic!("Unable to find the iced id via the ipg id {}.", value)
    }
}

pub fn get_len(fill: Option<bool>, length_fill: Option<bool>, length: Option<f32>) -> Length {
    if fill == Some(true) || length_fill == Some(true) {
        return Length::Fill
    }

    if length.is_some() {
        return Length::Fixed(length.unwrap())
    }

    Length::Shrink
}

// Standard method for padding
pub fn get_padding(padding: &Option<Vec<f32>>)-> Padding {
    let pd = if let Some(pd) = padding {
        pd
    } else { return Padding::default() };

    let len = pd.len();
    match len {
    0 => Padding::default(),
    1 => Padding::from(pd[0]),
    2 => Padding::from(vec_to_array2_f32(&pd)),
    3 => panic!("Padding must have a List of 1, 2, or 4 items"),
    4 => {
        let mut pad = Padding::default();
        pad = pad.top(pd[0]);
        pad = pad.right(pd[1]);
        pad = pad.bottom(pd[2]);
        pad = pad.left(pd[3]);
        pad
    },
    _ => panic!("Padding must have a List of less than 4 items"),
    }
}

pub fn get_radius(border_radius: &Vec<f32>, widget_name: String) -> Radius {
    if border_radius.len() == 1 {
        Radius::from(border_radius[0])
    } else if border_radius.len() == 4 {
        let mut rad = Radius::default();
        rad = rad.top_left(border_radius[0]);
        rad = rad.top_right(border_radius[1]); 
        rad = rad.bottom_right(border_radius[2]); 
        rad = rad.bottom_left(border_radius[3]);
        rad
    } else {
        panic!("{} style: Border radius must be a list of 1 or 4 items", widget_name)
    }
}


fn vec_to_array2_f32(arr: &[f32]) -> [f32; 2] {
    [arr[0], arr[1]]
}


pub const MONTH_NAMES: [&str; 13] = ["", "January", "Feburary", "March", 
                                        "April", "May", "June", "July", 
                                        "August", "September", "October", 
                                        "November", "December"];
                                
pub const DATE_FORMATS: [&str; 3] = ["mm-dd-YYYY", "YYYY-mm-dd", "mm-dd-YY"];
pub const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
pub const DAYS: [&str; 7] = ["S", "M", "T", "W", "T", "F", "S"];

pub fn format_date(format: String, year: i32, month: usize, day: usize) -> String {

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
