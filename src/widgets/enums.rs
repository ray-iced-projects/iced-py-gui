//! Common enums used across widgets

use pyo3::{Python, pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use iced::{self, Alignment, alignment, widget::text::Shaping};

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgShaping {
    Auto,
    Basic,
    Advanced,
}

impl IpgShaping {
    pub fn to_iced(&self) -> Shaping {
        match self {
            IpgShaping::Auto => Shaping::Auto,
            IpgShaping::Basic => Shaping::Basic,
            IpgShaping::Advanced => Shaping::Advanced,
        }
    }

    pub fn extract(value: &PyObject) -> Option<IpgShaping> {
        Python::attach(|py| {
            let res = value.extract::<IpgShaping>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => panic!("Unable to extract python IpgShaping"),
            }
        })  
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgAlignment {
    Start,
    Center,
    End,
}

impl IpgAlignment {
    pub fn to_iced(&self) -> Alignment {
        match self {
            IpgAlignment::Start => Alignment::Start,
            IpgAlignment::Center => Alignment::Center,
            IpgAlignment::End => Alignment::End,
        }
    }

    pub fn extract(value: &PyObject) -> Option<IpgAlignment> {
        Python::attach(|py| {
            let res = value.extract::<IpgAlignment>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => panic!("Unable to extract python IpgAlignment"),
            }
        })  
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgHorizontalAlignment {
    Left,
    Center,
    Right,
}

impl IpgHorizontalAlignment {
    pub fn to_iced(&self) -> alignment::Horizontal {
        match self {
            IpgHorizontalAlignment::Left => alignment::Horizontal::Left,
            IpgHorizontalAlignment::Center => alignment::Horizontal::Center,
            IpgHorizontalAlignment::Right => alignment::Horizontal::Right,
        }
    }

    pub fn extract(value: &PyObject) -> Option<IpgHorizontalAlignment> {
        Python::attach(|py| {
            let res = value.extract::<IpgHorizontalAlignment>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => panic!("Unable to extract python IpgHorizontalAlignment"),
            }
        })  
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgVerticalAlignment {
    Top,
    Center,
    Bottom,
}

impl IpgVerticalAlignment {
    pub fn to_iced(&self) -> iced::alignment::Vertical {
        match self {
            IpgVerticalAlignment::Top => alignment::Vertical::Top,
            IpgVerticalAlignment::Center => alignment::Vertical::Center,
            IpgVerticalAlignment::Bottom => alignment::Vertical::Bottom,
        }
    }

    pub fn extract(value: &PyObject) -> Option<IpgVerticalAlignment> {
        Python::attach(|py| {
            let res = value.extract::<IpgVerticalAlignment>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => panic!("Unable to extract python IpgVerticalAlignment"),
            }
        })  
    }
}

pub fn h_v_centered() -> (alignment::Horizontal, alignment::Vertical) {
    (alignment::Horizontal::Center, alignment::Vertical::Center)
}
