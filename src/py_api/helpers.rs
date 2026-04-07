//! helpers

use std::collections::HashMap;

use crate::widgets::styling::StyleStandard;
use crate::access_state;

use iced::border::Radius;
use iced::{window, Length, Padding};

use pyo3::{Py, PyAny, Python};
type PyObject = Py<PyAny>;


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

// Standard method for Length using Width
pub fn get_length_fill(fill: Option<bool>)-> [Length; 2] {
    // fill overrides width and height
    match fill {
        Some(f) => 
            match f {
                true => [Length::Fill; 2],
                false => [Length::Shrink; 2],
            },
        None => [Length::Shrink; 2],
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

pub fn try_extract_f32(value: &PyObject, name: &str) -> f32 {
    Python::attach(|py| {
        let res = value.extract::<f32>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python float", name),
        }
    })  
}

pub fn try_extract_f32_opt(value: &PyObject, name: &str) -> Option<f32> {
    Python::attach(|py| {
        let res = value.extract::<Option<f32>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python optional float", name),
        }
    })  
}

pub fn try_extract_vec_u8_opt(value: &PyObject, name: &str) -> Option<Vec<u8>> {
    Python::attach(|py| {
        let res = value.extract::<Option<Vec<u8>>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract u8", name),
        }
    })  
}

pub fn try_extract_u16(value: &PyObject, name: &str) -> u16 {
    Python::attach(|py| {
        let res = value.extract::<u16>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract u16", name),
        }
    })  
}

pub fn try_extract_u32(value: &PyObject, name: &str) -> u32 {
    Python::attach(|py| {
        let res = value.extract::<u32>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract u32", name),
        }
    })  
}

pub fn try_extract_u64(value: &PyObject, name: &str) -> u64 {
    Python::attach(|py| {
        let res = value.extract::<u64>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract u64", name),
        }
    })  
}

pub fn try_extract_usize(value: &PyObject, name: &str) -> usize {
    Python::attach(|py| {
        let res = value.extract::<usize>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract usize", name),
        }
    })  
}

pub fn try_extract_opt_usize(value: &PyObject, name: &str) -> Option<usize> {
    Python::attach(|py| {
        let res = value.extract::<Option<usize>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract usize", name),
        }
    })  
}

pub fn try_extract_vec_f32(value: &PyObject, name: &str) -> Vec<f32> {
    Python::attach(|py| {
        let res = value.extract::<Vec<f32>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python list[float]", name),
        }
    })  
}

pub fn try_extract_opt_vec_f32(value: &PyObject, name: &str) -> Option<Vec<f32>> {
    Python::attach(|py| {
        let res = value.extract::<Option<Vec<f32>>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python list[float]", name),
        }
    })  
}

pub fn try_extract_vec_vec_f32(value: &PyObject, name: &str) -> Vec<Vec<f32>> {
    Python::attach(|py| {
        let res = value.extract::<Vec<Vec<f32>>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python list[list[float]]", name),
        }
    })
}

pub fn try_extract_vec_usize(value: &PyObject, name: &str) -> Vec<usize> {
    Python::attach(|py| {
        let res = value.extract::<Vec<usize>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python list[uint]", name),
        }
    })  
}

pub fn try_extract_f32_array_2(value: &PyObject, name: &str) -> [f32; 2] {
    Python::attach(|py| {

        let res = value.extract::<[f32; 2]>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for 2 item list", name),
        }
    })
}

pub fn try_extract_f32_opt_array_1_or_upto_4(value: &PyObject, name: &str) -> Option<Vec<f32>> {
    Python::attach(|py| {

        let res = value.extract::<Vec<f32>>(py);
        match res {
            Ok(val) => {
                if val.len() > 4 {panic!("{}-The radius must be a list of length 1 or < 4", name)}
                Some(val)
            },
            Err(_) => panic!("{}-Unable to extract python object for 2 item list", name),
        }
    })
}

pub fn try_extract_u16_array_2(value: &PyObject, name: &str) -> [u16; 2] {
    Python::attach(|py| {

        let res = value.extract::<[u16; 2]>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for 2 item list", name),
        }
    })
}

pub fn try_extract_opt_u32_array_2(value: &PyObject, name: &str) -> Option<[u32; 2]> {
    Python::attach(|py| {

        let res = value.extract::<Option<[u32; 2]>>(py);
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

pub fn try_extract_opt_string(value: &PyObject, name: &str) -> Option<String> {
    Python::attach(|py| {
        let res = value.extract::<Option<String>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python str", name),
        }
    })  
}

pub fn try_extract_vec_str(value: &PyObject, name: &str) -> Vec<String> {
    Python::attach(|py| {
        let res = value.extract::<Vec<String>>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python list[str]", name),
        }
    })  
}

pub fn try_extract_boolean(value: &PyObject, name: &str) -> bool {
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

pub fn try_extract_style_standard(value: &PyObject, name: &str) -> StyleStandard {
    Python::attach(|py| {

        let res = value.extract::<StyleStandard>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("{}-Unable to extract python object for StyleStandard", name),
        }
    })
}
