//! Color definitions

use iced::Color;
use pyo3::pyclass;

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgColor {
    PRIMARY,
    SECONDARY,
    SUCCESS,
    DANGER,
    WARNING,
    INFO,
    LIGHT,
    DARK,
    WHITE,
    BLACK,
    RED,
    GREEN,
    BLUE,
    YELLOW,
    CYAN,
    MAGENTA,
    GRAY,
    DODGER_BLUE,
}

impl IpgColor {
    pub fn to_iced(&self) -> Color {
        match self {
            IpgColor::PRIMARY => Color::from_rgb(0.0, 0.478, 1.0),
            IpgColor::SECONDARY => Color::from_rgb(0.416, 0.455, 0.49),
            IpgColor::SUCCESS => Color::from_rgb(0.157, 0.655, 0.271),
            IpgColor::DANGER => Color::from_rgb(0.863, 0.208, 0.271),
            IpgColor::WARNING => Color::from_rgb(1.0, 0.761, 0.035),
            IpgColor::INFO => Color::from_rgb(0.082, 0.788, 0.89),
            IpgColor::LIGHT => Color::from_rgb(0.969, 0.973, 0.976),
            IpgColor::DARK => Color::from_rgb(0.129, 0.145, 0.161),
            IpgColor::WHITE => Color::WHITE,
            IpgColor::BLACK => Color::BLACK,
            IpgColor::RED => Color::from_rgb(1.0, 0.0, 0.0),
            IpgColor::GREEN => Color::from_rgb(0.0, 1.0, 0.0),
            IpgColor::BLUE => Color::from_rgb(0.0, 0.0, 1.0),
            IpgColor::YELLOW => Color::from_rgb(1.0, 1.0, 0.0),
            IpgColor::CYAN => Color::from_rgb(0.0, 1.0, 1.0),
            IpgColor::MAGENTA => Color::from_rgb(1.0, 0.0, 1.0),
            IpgColor::GRAY => Color::from_rgb(0.5, 0.5, 0.5),
            IpgColor::DODGER_BLUE => Color::from_rgb(0.118, 0.565, 1.0),
        }
    }
}
