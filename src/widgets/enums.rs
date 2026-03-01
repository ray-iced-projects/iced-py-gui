//! Common enums used across widgets

use pyo3::{Python, pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use iced::{self, Alignment, Radians, Rotation, alignment, widget::{image::FilterMethod, text::Shaping}};


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
pub enum IpgAlignmentX {
    Left,
    Center,
    Right,
}

impl IpgAlignmentX {
    pub fn to_iced(&self) -> alignment::Horizontal {
        match self {
            IpgAlignmentX::Left => alignment::Horizontal::Left,
            IpgAlignmentX::Center => alignment::Horizontal::Center,
            IpgAlignmentX::Right => alignment::Horizontal::Right,
        }
    }

    pub fn extract(value: &PyObject) -> Option<IpgAlignmentX> {
        Python::attach(|py| {
            let res = value.extract::<IpgAlignmentX>(py);
            match res {
                Ok(val) => Some(val),
                Err(_) => panic!("Unable to extract python IpgHorizontalAlignment"),
            }
        })  
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgAlignmentY {
    Top,
    Center,
    Bottom,
}

impl IpgAlignmentY {
    pub fn to_iced(&self) -> iced::alignment::Vertical {
        match self {
            IpgAlignmentY::Top => alignment::Vertical::Top,
            IpgAlignmentY::Center => alignment::Vertical::Center,
            IpgAlignmentY::Bottom => alignment::Vertical::Bottom,
        }
    }

    pub fn extract(value: &PyObject) -> Option<IpgAlignmentY> {
        Python::attach(|py| {
            let res = value.extract::<IpgAlignmentY>(py);
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
