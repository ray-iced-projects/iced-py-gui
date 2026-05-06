//!Helpers
use super::color_math::{color_at, hue_to_rgb, mix, rgb_to_sv};
use iced::{Border, Element, Length, Padding, Pixels, Point, Rectangle, Theme};
use iced::theme::palette;
use iced::widget::canvas;
use iced::widget::{button, container, column, radio, row, slider, text, Checkbox, TextInput};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorOutFormat {
    Float,
    Hex,
    Integer,
    Percent,
}

#[derive(Debug, Clone, Copy)]
pub enum RGBA { R, G, B, A, H }

pub fn selected_color_format_to_text(
    format: Option<ColorOutFormat>,
    selected_color: [f32; 4],
) -> String {
    let [r, g, b, a] = selected_color;
    match format {
        Some(ColorOutFormat::Integer) => format!(
            "[{}, {}, {}, {}]",
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            (a * 255.0).round() as u8,
        ),
        Some(ColorOutFormat::Float) => format!(
            "[{:.2}, {:.2}, {:.2}, {:.2}]", r, g, b, a
        ),
        Some(ColorOutFormat::Hex) => format!(
            "[#{:02X}{:02X}{:02X}{:02X}]",
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            (a * 255.0).round() as u8,
        ),
        Some(ColorOutFormat::Percent) => format!(
            "[{:.0}%, {:.0}%, {:.0}%, {:.0}%]",
            r * 100.0, g * 100.0, b * 100.0, a * 100.0
        ),
        None => String::new(),
    }
}

pub fn btn_style(theme: &Theme, status: button::Status) -> button::Style {
    let mut style = button::primary(theme, status);
    style.border = Border {
        radius: 5.0.into(),
        ..Default::default()
    };
    style
}

pub fn hue_rail_gradient() -> iced::Background {
    use std::f32::consts::FRAC_PI_2;
    let gradient = iced::gradient::Linear::new(FRAC_PI_2)
        .add_stop(0.0,        iced::Color::from_rgb(1.0, 0.0, 0.0)) // red
        .add_stop(1.0 / 6.0, iced::Color::from_rgb(1.0, 1.0, 0.0)) // yellow
        .add_stop(2.0 / 6.0, iced::Color::from_rgb(0.0, 1.0, 0.0)) // green
        .add_stop(3.0 / 6.0, iced::Color::from_rgb(0.0, 1.0, 1.0)) // cyan
        .add_stop(4.0 / 6.0, iced::Color::from_rgb(0.0, 0.0, 1.0)) // blue
        .add_stop(5.0 / 6.0, iced::Color::from_rgb(1.0, 0.0, 1.0)) // magenta
        .add_stop(1.0,        iced::Color::from_rgb(1.0, 0.0, 0.0)); // red again
    iced::Background::Gradient(gradient.into())
}

pub fn slider_style(
    theme: &Theme,
    status: slider::Status,
    rgba: RGBA,
    value: u8,
) -> slider::Style {
    let mut style = iced::widget::slider::default(theme, status);

    let base = match rgba {
        RGBA::R => iced::Color::from_rgb(1.0, 0.0, 0.0),   // RED
        RGBA::G => iced::Color::from_rgb(0.0, 0.502, 0.0), // GREEN
        RGBA::B => iced::Color::from_rgb(0.0, 0.0, 1.0),   // BLUE
        RGBA::A => iced::Color::BLACK,
        RGBA::H => iced::Color::BLACK, // overridden below
    };

    let pal = palette::Background::new(base, iced::Color::WHITE);

    let color = match status {
        slider::Status::Active  => pal.base.color,
        slider::Status::Hovered => pal.strong.color,
        slider::Status::Dragged => pal.weak.color,
    };

    let rail_backgrounds = if matches!(rgba, RGBA::H) {
        let transparent = iced::Background::Color(iced::Color::TRANSPARENT);
        (transparent, transparent)
    } else if matches!(rgba, RGBA::A) {
        let alpha = value as f32 / 255.0;
        let bg = iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, alpha));
        (bg, bg)
    } else {
        (color.into(), pal.strong.color.into())
    };

    let rail = slider::Rail {
        backgrounds: rail_backgrounds,
        width: 15.0,
        border: iced::Border {
            radius: 10.0.into(),
            width: 2.0,
            ..Default::default()
        },
    };
    style.rail = rail;
    style.handle.shape = iced::widget::slider::HandleShape::Rectangle {
        width: 6,
        border_radius: 4.0.into(),
    };
    style
}

pub fn palette_swatch<M: 'static>(
    label: &'static str,
    color: iced::Color,
) -> Element<'static, M> {
    let [r, g, b, _] = [color.r, color.g, color.b, color.a];
    let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let fg = if luminance > 0.45 { iced::Color::BLACK } else { iced::Color::WHITE };
    container(text(label).size(10.0))
        .style(move |_theme: &Theme| container::Style {
            background: Some(iced::Background::Color(color)),
            text_color: Some(fg),
            border: iced::Border { radius: 4.0.into(), ..Default::default() },
            ..Default::default()
        })
        .padding(Padding::new(3.0))
        .center_x(Length::Fill)
        .height(18.0)
        .into()
}

pub fn palette_panel<M: 'static>(selected: [f32; 4]) -> Element<'static, M> {
    let c = [selected[0], selected[1], selected[2]];
    let white = [1.0f32; 3];
    let black = [0.0f32; 3];

    column(vec![
        palette_swatch("weakest",  mix(c, white, 0.80)),
        palette_swatch("weaker",   mix(c, white, 0.55)),
        palette_swatch("weak",     mix(c, white, 0.30)),
        palette_swatch("base",     iced::Color::from_rgb(c[0], c[1], c[2])),
        palette_swatch("strong",   mix(c, black, 0.25)),
        palette_swatch("stronger", mix(c, black, 0.50)),
        palette_swatch("strongest",mix(c, black, 0.70)),
    ])
    .spacing(3.0)
    .width(80.0)
    .into()
}

pub fn rgba_slider<'a, Message>(
    label: &'a str,
    value: u8,
    rgba: RGBA,
    on_change: impl Fn(u8) -> Message + 'a,
    on_input: impl Fn(RGBA, String) -> Message + 'a,
) -> iced::widget::Row<'a, Message>
where
    Message: Clone + 'a,
{
    let sld = slider(0..=255, value, on_change)
        .step(1)
        .width(200.0)
        .style(move |theme, status| slider_style(theme, status, rgba, value));

    let input_text = TextInput::new(
            &"".to_string(),
            &value.to_string(),
        )
        .on_input(move |s| on_input(rgba, s))
        .size(Pixels(12.0))
        .padding(Padding::default().left(5));

    row(vec![
        text(label.to_owned()).into(),
        sld.into(),
        input_text.into(),
    ])
    .spacing(3.0)
}

/// Canvas program that draws the HSV color square.
pub struct HsvSquare<Message> {
    pub hue: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub on_pick: Box<dyn Fn(u8, u8, u8) -> Message>,
}

/// Internal drag state for the canvas.
#[derive(Default)]
pub struct HsvSquareState {
    is_dragging: bool,
}

impl<Message: 'static> canvas::Program<Message> for HsvSquare<Message> {
    type State = HsvSquareState;

    fn update(
        &self,
        state: &mut HsvSquareState,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        match event {
            canvas::Event::Mouse(iced::mouse::Event::ButtonPressed(
                iced::mouse::Button::Left,
            )) => {
                if let Some(pos) = cursor.position_in(bounds) {
                    state.is_dragging = true;
                    let (r, g, b) = color_at(pos, bounds.size(), self.hue);
                    return Some(canvas::Action::publish((self.on_pick)(r, g, b)));
                }
                None
            }
            canvas::Event::Mouse(iced::mouse::Event::CursorMoved { .. }) => {
                if state.is_dragging {
                    if let Some(pos) = cursor.position_in(bounds) {
                        let (r, g, b) = color_at(pos, bounds.size(), self.hue);
                        return Some(canvas::Action::publish((self.on_pick)(r, g, b)));
                    }
                }
                None
            }
            canvas::Event::Mouse(iced::mouse::Event::ButtonReleased(
                iced::mouse::Button::Left,
            )) => {
                state.is_dragging = false;
                None
            }
            _ => None,
        }
    }

    fn draw(
        &self,
        _state: &HsvSquareState,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let size = bounds.size();

        // Layer 1: white (left) → pure hue (right) — saturation axis
        frame.fill_rectangle(
            Point::ORIGIN,
            size,
            canvas::Fill {
                style: canvas::Style::Gradient(
                    canvas::Gradient::Linear(
                        canvas::gradient::Linear::new(
                            Point::new(0.0, 0.0),
                            Point::new(size.width, 0.0),
                        )
                        .add_stop(0.0, iced::Color::WHITE)
                        .add_stop(1.0, hue_to_rgb(self.hue)),
                    ),
                ),
                ..Default::default()
            },
        );

        // Layer 2: transparent (bottom) → black (top) — value/brightness axis
        frame.fill_rectangle(
            Point::ORIGIN,
            size,
            canvas::Fill {
                style: canvas::Style::Gradient(
                    canvas::Gradient::Linear(
                        canvas::gradient::Linear::new(
                            Point::new(0.0, size.height), // transparent at bottom
                            Point::new(0.0, 0.0),          // black at top
                        )
                        .add_stop(0.0, iced::Color::TRANSPARENT)
                        .add_stop(1.0, iced::Color::BLACK),
                    ),
                ),
                ..Default::default()
            },
        );

        // Selector circle at the current color position (unfilled, white stroke)
        let (s, v) = rgb_to_sv(self.r, self.g, self.b);
        let cx = s * size.width;
        let cy = v * size.height;
        let radius = 5.0_f32;
        frame.stroke(
            &canvas::Path::circle(Point::new(cx, cy), radius),
            canvas::Stroke::default()
                .with_color(iced::Color::WHITE)
                .with_width(2.0),
        );

        vec![frame.into_geometry()]
    }
}

pub fn hue_slider_row<M, F>(
    hue_value: u8,
    format: Option<ColorOutFormat>,
    selected_color: [f32; 4],
    on_hue_change: impl Fn(u8) -> M + 'static,
    on_format_selected: F,
) -> Element<'static, M>
where
    M: Clone + 'static,
    F: Fn(ColorOutFormat) -> M + Clone + 'static,
{
    let hue_sld = container(
        slider(0..=255, hue_value, on_hue_change)
            .step(1)
            .width(200.0)
            .style(move |theme, status| slider_style(theme, status, RGBA::H, 0)),
    )
    .style(|_theme| container::Style {
        background: Some(hue_rail_gradient()),
        border: iced::Border {
            radius: 10.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let size = 12.0;
    let text_size = 14.0;
    let rad_int = radio("Int", ColorOutFormat::Integer, format, on_format_selected.clone())
        .size(size)
        .text_size(text_size);
    let rad_float = radio("Float", ColorOutFormat::Float, format, on_format_selected.clone())
        .size(size)
        .text_size(text_size);
    let rad_hex = radio("Hex", ColorOutFormat::Hex, format, on_format_selected.clone())
        .size(size)
        .text_size(text_size);
    let rad_percent = radio("Percent", ColorOutFormat::Percent, format, on_format_selected)
        .size(size)
        .text_size(text_size);

    let rad_row = row([
        rad_int.into(),
        rad_float.into(),
        rad_hex.into(),
        rad_percent.into(),
    ])
    .spacing(5.0);

    let col = column([hue_sld.into(), rad_row.into()])
        .spacing(5.0)
        .into();

    let bkg = iced::Color::from(selected_color);
    let [r, g, b, _] = selected_color;
    let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let text_color = if luminance > 0.5 { iced::Color::BLACK } else { iced::Color::WHITE };

    let color_label: Element<M> = text(selected_color_format_to_text(format, selected_color))
        .size(Pixels(10.0))
        .color(text_color)
        .into();

    let value_cont = container(color_label)
        .style(move |_| container::background(bkg))
        .center_x(150)
        .center_y(Length::Fill)
        .width(150.0)
        .height(Length::Fill)
        .into();

    row([col, value_cont]).spacing(10.0).into()
}

pub fn submit_row<M: Clone + 'static>(
    show_palette: bool,
    on_submit: M,
    on_cancel: M,
    on_copy: M,
    on_show_palette: impl Fn(bool) -> M + 'static,
) -> iced::widget::Row<'static, M> {
    let size = Pixels(12.0);

    let submit_btn: Element<M> = button(text("Submit").size(size))
        .on_press(on_submit)
        .padding(5.0)
        .style(|theme, status| btn_style(theme, status))
        .into();

    let cancel_btn: Element<M> = button(text("Cancel").size(size))
        .on_press(on_cancel)
        .padding(5.0)
        .style(|theme, status| btn_style(theme, status))
        .into();

    let clipbrd_btn: Element<M> = button(text("ClipBoard").size(size))
        .on_press(on_copy)
        .padding(5.0)
        .style(|theme, status| btn_style(theme, status))
        .into();

    let palette_chk: Element<M> = Checkbox::new(show_palette)
        .label("Show Palette")
        .on_toggle(on_show_palette)
        .size(14.0)
        .text_size(14.0)
        .into();

    row([submit_btn, cancel_btn, clipbrd_btn, palette_chk])
        .spacing(15.0)
        .align_y(iced::Alignment::Center)
}
