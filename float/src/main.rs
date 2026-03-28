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
    Scale,
    ScaleClamped,
    Translate,
    TranslateScaled,
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
        
        let scale = if self.mode == Mode::ScaleClamped {
            1.0
        } else if self.mode == Mode::Scale {
            1.4
        } else { 1.0 };
        

        let mode_buttons = column![
            text("Float modes:").size(16),
            button(text("Scale only (scale 1.4)"))
                .on_press(Message::SetMode(Mode::Scale))
                .width(Fill),
            button(text("Translate only (+80, +200))"))
                .on_press(Message::SetMode(Mode::Translate))
                .width(Fill),
            button(text("Translate_scaled (+80, +100 * 1.5)"))
                .on_press(Message::SetMode(Mode::TranslateScaled))
                .width(Fill),
            button(text("Clamped to viewport"))
                .on_press(Message::SetMode(Mode::ScaleClamped))
                .width(Fill),
        ]
        .spacing(8)
        .width(200);

        // The card wrapped in float — this is the interesting part
        let card = float(
            container(
                column![
                    text("I'm a Float!").size(20),
                    text(match self.mode {
                        Mode::Scale => "Scaled 1.4x",
                        Mode::ScaleClamped => "Scale Clamped",
                        Mode::Translate => "Translate",
                        Mode::TranslateScaled => "Translate and Scaled",
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
        
        .translate(move |bounds, viewport| {
            match self.mode {
                Mode::Scale => {
                    Vector::ZERO
                }
                Mode::ScaleClamped => {
                    bounds.zoom(10.0).offset(&viewport.shrink(10))
                }
                Mode::Translate => {
                    Vector::new(80.0, 200.0)
                }
                Mode::TranslateScaled => {
                    Vector::new(40.0, 100.0)
                }
            }
        })
        .scale(scale)
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
