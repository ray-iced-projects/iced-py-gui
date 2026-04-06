//! Common enums used across widgets

use pyo3::{Python, pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use iced::{self, Alignment, Radians, Rotation, 
    widget::{image::FilterMethod}};


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum Align {
    Start,
    Center,
    End,
}

impl Align {
    pub fn to_iced(&self) -> Alignment {
        match self {
            Align::Start => Alignment::Start,
            Align::Center => Alignment::Center,
            Align::End => Alignment::End,
        }
    }

    pub fn extract(value: &PyObject) -> Option<Align> {
        Python::attach(|py| {
            let res = value.extract::<Align>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => panic!("Unable to extract python IpgAlignment"),
            }
        })  
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRotation {
    Floating,
    Solid,
}

impl IpgRotation {
    pub fn default() -> Rotation {
        Rotation::Floating(Radians(0.0))
    }

    pub fn to_iced(&self, rad: Option<f32>) -> Rotation {
        let rads = if let Some(rad) = rad {
            rad
        } else { 0.0 };
        match self {
            IpgRotation::Floating => Rotation::Floating(Radians(rads)),
            IpgRotation::Solid => Rotation::Solid(Radians(rads)),
        }
    }

    pub fn extract(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<IpgRotation>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })  
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgContentFit {
    Contain,
    Cover,
    Fill,
    IpgNone,
    ScaleDown,
}

impl IpgContentFit {
    pub fn default() -> Self {
        IpgContentFit::Contain
    }

    pub fn to_iced(&self) -> iced::ContentFit {
        match self {
            IpgContentFit::Contain => iced::ContentFit::Contain,
            IpgContentFit::Cover => iced::ContentFit::Cover,
            IpgContentFit::Fill => iced::ContentFit::Fill,
            IpgContentFit::IpgNone => iced::ContentFit::None,
            IpgContentFit::ScaleDown => iced::ContentFit::ScaleDown,
        }
    }

    pub fn extract(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<IpgContentFit>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })  
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgColorFilter {
    Linear,
    Nearest,
}

impl IpgColorFilter {
    pub fn default() -> Self {
        IpgColorFilter::Linear
    }

    pub fn to_iced(&self) -> FilterMethod {
        match self {
            IpgColorFilter::Linear => FilterMethod::Linear,
            IpgColorFilter::Nearest => FilterMethod::Nearest,
        }
    }

    pub fn extract(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<IpgColorFilter>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })  
    }
}
