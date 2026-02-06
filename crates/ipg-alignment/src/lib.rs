
use iced::Alignment;
use iced::alignment::{Horizontal, Vertical};

use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;



#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgAlignment {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgHorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgVerticalAlignment {
    Top,
    Center,
    Bottom,
}

pub fn get_alignment(align: IpgAlignment) -> Alignment {

    match align {
        IpgAlignment::Start => Alignment::Start,
        IpgAlignment::Center => Alignment::Center,
        IpgAlignment::End => Alignment::End,
    }
}

pub fn get_horizontal_alignment(h_align: &IpgHorizontalAlignment) -> Horizontal {

    match h_align {
        IpgHorizontalAlignment::Left => Horizontal::Left,
        IpgHorizontalAlignment::Center => Horizontal::Center,
        IpgHorizontalAlignment::Right => Horizontal::Right,
    }
}

pub fn get_vertical_alignment(v_align: &IpgVerticalAlignment) -> Vertical {
    
    match v_align {
        IpgVerticalAlignment::Top => Vertical::Top,
        IpgVerticalAlignment::Center => Vertical::Center,
        IpgVerticalAlignment::Bottom => Vertical::Bottom,
    }
}

// These alignments return options so that only the canvas text alignment needs one py value type
pub fn try_extract_ipg_horizontal_alignment(value: &PyObject) 
        -> Option<IpgHorizontalAlignment> {
    Python::attach(|py| {

        let res = value.extract::<IpgHorizontalAlignment>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })
}

pub fn try_extract_ipg_vertical_alignment(value: &PyObject) -> Option<IpgVerticalAlignment> {
    Python::attach(|py| {

        let res = value.extract::<IpgVerticalAlignment>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })
}

pub fn try_extract_ipg_alignment(value: &PyObject) -> Option<IpgAlignment> {
    Python::attach(|py| {

        let res = value.extract::<IpgAlignment>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })
}
