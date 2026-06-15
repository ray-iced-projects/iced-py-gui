//! styling
#![allow(dead_code)]
use pyo3::pyclass;


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum IpgStyleStandard {
    Primary,
    Success,
    Danger,
    Text,
}

/// Per-status optional color overrides.
/// idx: 0 = Active, 1 = Hovered, 2 = Pressed/Focused, 3 = Disabled
pub struct ColorStatus {
    pub active: Option<iced::Color>,
    pub hovered: Option<iced::Color>,
    pub pressed: Option<iced::Color>,
    pub disabled: Option<iced::Color>,
}

impl ColorStatus {
    /// Return the override for `idx`, or `default` if none is set.
    pub fn pick(&self, idx: usize, default: iced::Color) -> iced::Color {
        match idx {
            0 => self.active.unwrap_or(default),
            1 => self.hovered.unwrap_or(default),
            2 => self.pressed.unwrap_or(default),
            _ => self.disabled.unwrap_or(default),
        }
    }
}

/// Pick the appropriate palette color for a given status index.
/// 0=Active → base, 1=Hovered → weaker, 2=Pressed → strong, 3=Disabled → base×0.5α
pub fn palette_pick(bkg: &iced::theme::palette::Background, idx: usize) -> iced::Color {
    match idx {
        0 => bkg.base.color,
        1 => bkg.weaker.color,
        2 => bkg.strong.color,
        _ => bkg.base.color.scale_alpha(0.5),
    }
}
