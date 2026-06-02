//! Displays a [`Card`](crate::widget::Card).
//!
//! *This API requires the following crate features to be activated: card*


use iced::{Background, Theme};

use crate::iced_aw_widgets::card::aw_status::{Status, StyleFn};

use crate::graphics::colors;

/// The appearance of a [`Card`](crate::widget::card::Card).
#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// The background of the [`Card`](crate::widget::card::Card).
    pub background: Background,

    /// The border radius of the [`Card`](crate::widget::card::Card).
    pub border_radius: f32,

    /// The border width of the [`Card`](crate::widget::card::Card).
    pub border_width: f32,

    /// The border color of the [`Card`](crate::widget::card::Card).
    pub border_color: iced::Color,

    /// The background of the head of the [`Card`](crate::widget::card::Card).
    pub head_background: Background,

    /// The text color of the head of the [`Card`](crate::widget::card::Card).
    pub head_text_color: iced::Color,

    /// The background of the body of the [`Card`](crate::widget::card::Card).
    pub body_background: Background,

    /// The text color of the body of the [`Card`](crate::widget::card::Card).
    pub body_text_color: iced::Color,

    /// The background of the foot of the [`Card`](crate::widget::card::Card).
    pub foot_background: Background,

    /// The text color of the foot of the [`Card`](crate::widget::card::Card).
    pub foot_text_color: iced::Color,

    /// The color of the close icon of the [`Card`](crate::widget::card::Card).
    pub close_color: iced::Color,
}

/// The appearance of a [`Card`](crate::widget::card::Card).
pub trait Catalog {
    ///Style for the trait to use.
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> Self::Class<'a>;

    /// The [`Style`] of a class with the given status.
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background: iced::Color::WHITE.into(),
            border_radius: 10.0,
            border_width: 1.0,
            border_color: [0.87, 0.87, 0.87].into(),
            head_background: Background::Color([0.87, 0.87, 0.87].into()),
            head_text_color: iced::Color::BLACK,
            body_background: iced::Color::TRANSPARENT.into(),
            body_text_color: iced::Color::BLACK,
            foot_background: iced::Color::TRANSPARENT.into(),
            foot_text_color: iced::Color::BLACK,
            close_color: iced::Color::BLACK,
        }
    }
}

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self, Style>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The primary theme of a [`Card`](crate::widget::card::Card).
#[must_use]
pub fn primary(theme: &Theme, _status: Status) -> Style {
    let palette = theme.palette();
    backing_with_text(theme, palette.primary.base.color, palette.primary.base.text)
}

/// The secondary theme of a [`Card`](crate::widget::card::Card).
#[must_use]
pub fn secondary(theme: &Theme, _status: Status) -> Style {
    let palette = theme.palette();
    backing_with_text(theme, palette.secondary.base.color, palette.secondary.base.text)
}

/// The success theme of a [`Card`](crate::widget::card::Card).
#[must_use]
pub fn success(theme: &Theme, _status: Status) -> Style {
    let palette = theme.palette();
    backing_with_text(theme, palette.success.base.color, palette.success.base.text)
}

/// The danger theme of a [`Card`](crate::widget::card::Card).
#[must_use]
pub fn danger(theme: &Theme, _status: Status) -> Style {
    let palette = theme.palette();
    backing_with_text(theme, palette.danger.base.color, palette.danger.base.text)
}

/// The warning theme of a [`Card`](crate::widget::card::Card).
#[must_use]
pub fn warning(theme: &Theme, _status: Status) -> Style {
    let palette = theme.palette();
    backing_with_text(theme, palette.warning.base.color, palette.warning.base.text)
}

/// The light theme of a [`Card`](crate::widget::card::Card).
#[must_use]
pub fn light(theme: &Theme, _status: Status) -> Style {
    backing_only(theme, colors::Color::LIGHT.to_iced())
}

/// The dark theme of a [`Card`](crate::widget::card::Card).
#[must_use]
pub fn dark(theme: &Theme, _status: Status) -> Style {
    backing_with_text(theme, colors::Color::DARK.to_iced(), iced::Color::WHITE)
}

/// The white theme of a [`Card`](crate::widget::card::Card).
#[must_use]
pub fn white(theme: &Theme, _status: Status) -> Style {
    backing_only(theme, iced::Color::WHITE)
}

fn backing_with_text(theme: &Theme, color: iced::Color, text_color: iced::Color) -> Style {
    let palette = theme.palette();

    Style {
        border_color: color,
        head_background: color.into(),
        head_text_color: text_color,
        close_color: text_color,
        background: palette.background.base.color.into(),
        body_text_color: palette.background.base.text,
        foot_text_color: palette.background.base.text,
        ..Style::default()
    }
}

fn backing_only(theme: &Theme, color: iced::Color) -> Style {
    let palette = theme.palette();

    Style {
        border_color: color,
        head_background: color.into(),
        background: palette.background.base.color.into(),
        body_text_color: palette.background.base.text,
        foot_text_color: palette.background.base.text,
        ..Style::default()
    }
}
