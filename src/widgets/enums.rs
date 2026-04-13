//! Common enums used across widgets

use pyo3::pyclass;

use iced::{self, Radians, 
    widget::image};


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
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
}



#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
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
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum FilterMethod {
    Linear,
    Nearest,
}

impl FilterMethod {
    pub fn to_iced(&self) -> image::FilterMethod {
        match self {
            FilterMethod::Linear => image::FilterMethod::Linear,
            FilterMethod::Nearest => image::FilterMethod::Nearest,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
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
}
