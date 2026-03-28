use iced::border;
use iced::widget::{button, column, container, float, row, text};
use iced::{Center, Color, Element, Fill, Shadow, Theme, Vector};

pub fn main() -> iced::Result {
    iced::run(FloatExample::update, FloatExample::view)
}

#[derive(Default)]
struct FloatExample {
    mode: Mode,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum Mode {
    #[default]
    Normal,
    /// Scale only — content enlarges in place
    ScaleOnly,
    /// Scale + translate with a fixed offset
    TranslateFixed,
    /// Scale + translate clamped to viewport (like the gallery does)
    TranslateClamped,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SetMode(Mode),
}

impl FloatExample {
    fn update(&mut self, message: Message) {
        match message {
            Message::SetMode(mode) => {
                self.mode = mode;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mode = self.mode;

        // Scale is >1.0 for all non-Normal modes to activate floating
        let scale = if mode == Mode::Normal { 1.0 } else { 1.4 };

        let mode_buttons = column![
            text("Float modes:").size(16),
            button(text("Normal (scale 1.0)"))
                .on_press(Message::SetMode(Mode::Normal))
                .width(Fill),
            button(text("Scale only (1.4x, no translate)"))
                .on_press(Message::SetMode(Mode::ScaleOnly))
                .width(Fill),
            button(text("Translate fixed (+80, +200)"))
                .on_press(Message::SetMode(Mode::TranslateFixed))
                .width(Fill),
            button(text("Translate clamped to viewport"))
                .on_press(Message::SetMode(Mode::TranslateClamped))
                .width(Fill),
        ]
        .spacing(8)
        .width(200);

        // The card wrapped in float — this is the interesting part
        let card = float(
            container(
                column![
                    text("I'm a Float!").size(20),
                    text(match mode {
                        Mode::Normal => "Not floating (scale 1.0)",
                        Mode::ScaleOnly => "Scaled 1.4x in place",
                        Mode::TranslateFixed => "Scaled + shifted right & down",
                        Mode::TranslateClamped => "Scaled + kept inside viewport",
                    })
                    .size(13),
                ]
                .spacing(8)
                .align_x(Center),
            )
            .padding(30)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(palette.primary.weak.color.into()),
                    border: border::rounded(12)
                        .width(1)
                        .color(palette.primary.strong.color),
                    ..container::Style::default()
                }
            }),
        )
        .scale(scale)
        .translate(move |bounds, viewport| {
            // `bounds` = the original (un-scaled) rectangle of the content
            // `viewport` = the visible window area
            // Return a Vector to shift the floating content
            match mode {
                Mode::Normal | Mode::ScaleOnly => {
                    // No translation — content stays centered on its layout position
                    Vector::ZERO
                }
                Mode::TranslateFixed => {
                    // Shift the floating content by a fixed amount
                    Vector::new(80.0, 200.0)
                }
                Mode::TranslateClamped => {
                    // Compute where the scaled bounds would be, then nudge
                    // it back so it stays inside the viewport (with 10px margin).
                    // This is what the gallery example does.
                    bounds.zoom(scale).offset(&viewport.shrink(10))
                }
            }
        })
        .style(move |_theme| {
            let active = scale > 1.0;
            float::Style {
                shadow: Shadow {
                    color: Color::BLACK
                        .scale_alpha(if active { 0.5 } else { 0.0 }),
                    blur_radius: if active { 20.0 } else { 0.0 },
                    ..Shadow::default()
                },
                shadow_border_radius: border::radius(12),
            }
        });

        // Some background boxes so you can see the float overlapping them
        let background: Vec<Element<'_, Message>> = (0..4)
            .map(|i| {
                container(
                    text(format!("Background item {i}"))
                        .size(14)
                        .color(Color::WHITE),
                )
                .padding(20)
                .width(Fill)
                .style(move |theme: &Theme| {
                    let palette = theme.extended_palette();
                    container::Style {
                        background: Some(palette.secondary.base.color.into()),
                        border: border::rounded(8)
                            .width(1)
                            .color(palette.secondary.strong.color),
                        ..container::Style::default()
                    }
                })
                .into()
            })
            .collect();

        row![
            mode_buttons,
            column![card, column(background).spacing(8)]
                .spacing(12)
                .width(Fill)
                .align_x(Center),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}
