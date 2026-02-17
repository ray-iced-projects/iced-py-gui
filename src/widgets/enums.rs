//! Common enums used across widgets

use pyo3::pyclass;
use iced::{self, widget::text::Shaping, alignment, Alignment};

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
}
