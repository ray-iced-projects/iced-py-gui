//! Common enums used across widgets

use pyo3::pyclass;

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

impl IpgHorizontalAlignment {
    pub fn to_iced(&self) -> iced::alignment::Horizontal {
        match self {
            IpgHorizontalAlignment::Left => iced::alignment::Horizontal::Left,
            IpgHorizontalAlignment::Center => iced::alignment::Horizontal::Center,
            IpgHorizontalAlignment::Right => iced::alignment::Horizontal::Right,
        }
    }
}

impl IpgVerticalAlignment {
    pub fn to_iced(&self) -> iced::alignment::Vertical {
        match self {
            IpgVerticalAlignment::Top => iced::alignment::Vertical::Top,
            IpgVerticalAlignment::Center => iced::alignment::Vertical::Center,
            IpgVerticalAlignment::Bottom => iced::alignment::Vertical::Bottom,
        }
    }
}
