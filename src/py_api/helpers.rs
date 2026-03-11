//! helpers
#![allow(unused)]
use std::collections::HashMap;

use crate::graphics::colors::IpgColor;
use crate::widgets::ipg_button::IpgButtonStyleStd;
use crate::widgets::styling::IpgStyleStandard;
use crate::access_state;

use iced::border::Radius;
use iced::{window, Alignment, Pixels};
use iced::{alignment::{Horizontal, Vertical}, Length, Padding};
use iced::widget::text::{Shaping, LineHeight};

use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;

use crate::widgets::enums::{Align, AlignX, AlignY};


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

// Standard method for Length using Width
pub fn get_length(value: Option<f32>, fill: bool)-> Length {
    // width overrides width_fill
    match value {
        Some(wd) => Length::Fixed(wd),
        None => {
                match fill {
                    true => Length::Fill,
                    false => Length::Shrink,
                }
        },
    }
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

fn vec_to_array2_f64(arr: &[f64]) -> [f32; 2] {
    [arr[0] as f32, arr[1] as f32]
}

fn vec_to_array2_f32(arr: &[f32]) -> [f32; 2] {
    [arr[0], arr[1]]
}


pub fn get_line_height(pixels: Option<u16>, relative: Option<f32>) -> LineHeight {
    if let Some(pixs) =  pixels {
        LineHeight::Absolute(Pixels(pixs.into()))
    } else if let Some(rel) = relative {
        LineHeight::Relative(rel)
    } else {
        LineHeight::default()
    }
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

pub fn try_extract_f32(value: &PyObject, name: String) -> f32 {
    Python::attach(|py| {
        let res = value.extract::<f32>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python float", name),
        }
    })  
}

pub fn try_extract_f32_opt(value: &PyObject, name: String) -> Option<f32> {
    Python::attach(|py| {
        let res = value.extract::<Option<f32>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python optional float", name),
        }
    })  
}

pub fn try_extract_i32_opt(value: &PyObject) -> Option<i32> {
    Python::attach(|py| {
        let res = value.extract::<i32>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })  
}

pub fn try_extract_u16(value: &PyObject, name: String) -> u16 {
    Python::attach(|py| {
        let res = value.extract::<u16>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract u16", name),
        }
    })  
}

pub fn try_extract_u32(value: &PyObject, name: String) -> u32 {
    Python::attach(|py| {
        let res = value.extract::<u32>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract u32", name),
        }
    })  
}

pub fn try_extract_usize(value: &PyObject, name: String) -> usize {
    Python::attach(|py| {
        let res = value.extract::<usize>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract usize", name),
        }
    })  
}

pub fn try_extract_vec_f32(value: &PyObject, name: String) -> Vec<f32> {
    Python::attach(|py| {
        let res = value.extract::<Vec<f32>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python list[float]", name),
        }
    })  
}

pub fn try_extract_vec_vec_f32(value: &PyObject, name: String) -> Vec<Vec<f32>> {
    Python::attach(|py| {
        let res = value.extract::<Vec<Vec<f32>>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python list[list[float]]", name),
        }
    })
}

pub fn try_extract_vec_f32_opt(value: &PyObject, name: String) -> Option<Vec<f32>> {
    Python::attach(|py| {
        let res = value.extract::<Vec<f32>>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })  
}

pub fn try_extract_vec_u16(value: &PyObject, name: String) -> Vec<u16> {
    Python::attach(|py| {
        let res = value.extract::<Vec<u16>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python list[uint]", name),
        }
    })  
}

pub fn try_extract_vec_usize(value: &PyObject, name: String) -> Vec<usize> {
    Python::attach(|py| {
        let res = value.extract::<Vec<usize>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python list[uint]", name),
        }
    })  
}

pub fn try_extract_f32_array_2(value: &PyObject, name: String) -> [f32; 2] {
    Python::attach(|py| {

        let res = value.extract::<[f32; 2]>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for 2 item list", name),
        }
    })
}

pub fn try_extract_u16_array_2(value: &PyObject, name: String) -> [u16; 2] {
    Python::attach(|py| {

        let res = value.extract::<[u16; 2]>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for 2 item list", name),
        }
    })
}

pub fn try_extract_string(value: &PyObject, name: &str) -> String {
    Python::attach(|py| {
        let res = value.extract::<String>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python str", name),
        }
    })  
}

pub fn try_extract_vec_str(value: &PyObject, name: String) -> Vec<String> {
    Python::attach(|py| {
        let res = value.extract::<Vec<String>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python list[str]", name),
        }
    })  
}

pub fn try_extract_boolean(value: &PyObject, name: String) -> bool {
    Python::attach(|py| {
        let res = value.extract::<bool>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python bool", name),
        }
    })  
}

pub fn try_extract_opt_boolean(value: &PyObject, name: &str) -> Option<bool> {
    Python::attach(|py| {
        let res = value.extract::<Option<bool>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python bool", name),
        }
    })  
}

pub fn try_extract_style_standard(value: &PyObject, name: String) -> IpgStyleStandard {
    Python::attach(|py| {

        let res = value.extract::<IpgStyleStandard>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for StyleStandard", name),
        }
    })
}


pub fn try_extract_point(value: &PyObject, name: String) -> [f32; 2] {
    Python::attach(|py| {

        let res = value.extract::<[f32; 2]>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for Point", name),
        }
    })
}

