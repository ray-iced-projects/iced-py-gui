//! Common enums used across widgets

use pyo3::{Python, pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use iced::{self, Radians, 
    widget::{image::FilterMethod}};


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum Rotation {
    Floating,
    Solid,
}

impl Rotation {
    pub fn default() -> Self {
        Self::Floating
    }

    pub fn to_iced(&self, rad: Option<f32>) -> iced::Rotation {
        let rads = if let Some(rad) = rad {
            rad
        } else { 0.0 };
        match self {
            Rotation::Floating => iced::Rotation::Floating(Radians(rads)),
            Rotation::Solid => iced::Rotation::Solid(Radians(rads)),
        }
    }

    pub fn extract(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<Rotation>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })  
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum ContentFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}

impl ContentFit {
    pub fn default() -> Self {
        ContentFit::Contain
    }

    pub fn to_iced(&self) -> iced::ContentFit {
        match self {
            ContentFit::Contain => iced::ContentFit::Contain,
            ContentFit::Cover => iced::ContentFit::Cover,
            ContentFit::Fill => iced::ContentFit::Fill,
            ContentFit::None => iced::ContentFit::None,
            ContentFit::ScaleDown => iced::ContentFit::ScaleDown,
        }
    }

    pub fn extract(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<ContentFit>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })  
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum ColorFilter {
    Linear,
    Nearest,
}

impl ColorFilter {
    pub fn default() -> Self {
        ColorFilter::Linear
    }

    pub fn to_iced(&self) -> FilterMethod {
        match self {
            ColorFilter::Linear => FilterMethod::Linear,
            ColorFilter::Nearest => FilterMethod::Nearest,
        }
    }

    pub fn extract(value: &PyObject) -> Option<Self> {
        Python::attach(|py| {
            let res = value.extract::<ColorFilter>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => None,
            }
        })  
    }
}
