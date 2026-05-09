//! ColorPickerState
use super::color_math::{hue_to_rgb, rgb_to_hue};
use super::helpers::{
    hue_slider_row, palette_panel, rgba_slider, selected_color_format_to_text,
    submit_row, ColorOutFormat, HsvSquare, RGBA,
};
use iced::widget::{canvas::Canvas, container, column, row};
use iced::{Element, Length};

/// A message produced internally by [`ColorPickerState::view`].
/// Wrap this in one variant of your own `Message` enum and pass it back
/// to [`ColorPickerState::update`].
#[derive(Debug, Clone)]
pub enum ContentMsg {
    RChanged(u8),
    GChanged(u8),
    BChanged(u8),
    AChanged(u8),
    HueChanged(u8),
    RgbaInput(RGBA, String),
    CanvasPicked(u8, u8, u8),
    FormatSelected(ColorOutFormat),
    ShowPalette(bool),
    Submit,
    Cancel,
    Copy,
}

/// Events produced by [`ColorPickerState::update`] that require
/// the host application to act (side effects or state changes outside
/// of the color picker itself).
#[derive(Debug, Clone)]
pub enum ColorPickerEvent {
    /// The user confirmed the selection. Carries the current RGBA color.
    Submitted([f32; 4]),
    /// The user cancelled without confirming.
    Cancelled,
    /// The user requested a clipboard copy. Carries the formatted text.
    Copy(String),
}

/// Self-contained state for the color picker panel.
#[derive(Debug, Clone)]
pub struct ColorPickerState {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
    pub hue: u8,
    pub format: Option<ColorOutFormat>,
    pub show_palette: bool,
}

impl ColorPickerState {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: 255,
            hue: rgb_to_hue(r, g, b),
            format: Some(ColorOutFormat::Integer),
            show_palette: false,
        }
    }

    pub fn current_color(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }

    pub fn current_color_text(&self) -> String {
        selected_color_format_to_text(self.format, self.current_color())
    }

    /// Apply a [`ContentMsg`] to the state, returning a [`ColorPickerEvent`]
    /// if the host application needs to act.
    pub fn update(&mut self, msg: ContentMsg) -> Option<ColorPickerEvent> {
        match msg {
            ContentMsg::Submit => {
                return Some(ColorPickerEvent::Submitted(self.current_color()));
            }
            ContentMsg::Cancel => {
                return Some(ColorPickerEvent::Cancelled);
            }
            ContentMsg::Copy => {
                return Some(ColorPickerEvent::Copy(self.current_color_text()));
            }
            ContentMsg::RChanged(v) => {
                self.r = v;
                self.hue = rgb_to_hue(self.r, self.g, self.b);
            }
            ContentMsg::GChanged(v) => {
                self.g = v;
                self.hue = rgb_to_hue(self.r, self.g, self.b);
            }
            ContentMsg::BChanged(v) => {
                self.b = v;
                self.hue = rgb_to_hue(self.r, self.g, self.b);
            }
            ContentMsg::AChanged(v) => {
                self.a = v;
            }
            ContentMsg::HueChanged(v) => {
                self.hue = v;
                let rgb = hue_to_rgb(v);
                self.r = (rgb.r * 255.0).round() as u8;
                self.g = (rgb.g * 255.0).round() as u8;
                self.b = (rgb.b * 255.0).round() as u8;
            }
            ContentMsg::CanvasPicked(r, g, b) => {
                self.r = r;
                self.g = g;
                self.b = b;
                self.hue = rgb_to_hue(r, g, b);
            }
            ContentMsg::RgbaInput(channel, s) => {
                if let Ok(v) = s.parse::<u8>() {
                    match channel {
                        RGBA::R => {
                            self.r = v;
                            self.hue = rgb_to_hue(self.r, self.g, self.b);
                        }
                        RGBA::G => {
                            self.g = v;
                            self.hue = rgb_to_hue(self.r, self.g, self.b);
                        }
                        RGBA::B => {
                            self.b = v;
                            self.hue = rgb_to_hue(self.r, self.g, self.b);
                        }
                        RGBA::A => {
                            self.a = v;
                        }
                        RGBA::H => {
                            self.hue = v;
                            let rgb = hue_to_rgb(v);
                            self.r = (rgb.r * 255.0).round() as u8;
                            self.g = (rgb.g * 255.0).round() as u8;
                            self.b = (rgb.b * 255.0).round() as u8;
                        }
                    }
                }
            }
            ContentMsg::FormatSelected(f) => {
                self.format = Some(f);
            }
            ContentMsg::ShowPalette(b) => {
                self.show_palette = b;
            }
        }
        None
    }

    /// Build the content element for this color picker panel.
    ///
    /// Pass the returned element directly as the `content` argument to
    /// [`ColorPicker::new`]. The `on_msg` closure maps internal [`ContentMsg`]
    /// values to your application's message type.
    pub fn view<M>(&self, on_msg: impl Fn(ContentMsg) -> M + Clone + 'static) -> Element<'static, M>
    where
        M: Clone + 'static,
    {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        let a = self.a;
        let hue = self.hue;
        let format = self.format;
        let show_palette = self.show_palette;
        let color = self.current_color();

        let wrap = |msg: ContentMsg| on_msg.clone()(msg);

        let r_row = rgba_slider(
            "r", r, RGBA::R,
            {let f = on_msg.clone(); move |v| f(ContentMsg::RChanged(v))},
            {let f = on_msg.clone(); move |ch, s| f(ContentMsg::RgbaInput(ch, s))},
        ).into();
        let g_row = rgba_slider(
            "g", g, RGBA::G,
            {let f = on_msg.clone(); move |v| f(ContentMsg::GChanged(v))},
            {let f = on_msg.clone(); move |ch, s| f(ContentMsg::RgbaInput(ch, s))},
        ).into();
        let b_row = rgba_slider(
            "b", b, RGBA::B,
            {let f = on_msg.clone(); move |v| f(ContentMsg::BChanged(v))},
            {let f = on_msg.clone(); move |ch, s| f(ContentMsg::RgbaInput(ch, s))},
        ).into();
        let a_row = rgba_slider(
            "a", a, RGBA::A,
            {let f = on_msg.clone(); move |v| f(ContentMsg::AChanged(v))},
            {let f = on_msg.clone(); move |ch, s| f(ContentMsg::RgbaInput(ch, s))},
        ).into();

        let rgba_col = column(vec![r_row, g_row, b_row, a_row])
            .spacing(5.0)
            .into();

        let grad_cont: Element<M> = Canvas::new(HsvSquare {
            hue,
            r,
            g,
            b,
            on_pick: Box::new({
                let f = on_msg.clone();
                move |r, g, b| f(ContentMsg::CanvasPicked(r, g, b))
            }),
        })
        .width(Length::Fixed(100.0))
        .height(Length::Fixed(100.0))
        .into();

        let grad_rgba_row = row(vec![grad_cont, rgba_col])
            .width(Length::Fill)
            .spacing(5.0)
            .into();

        let hue_row = hue_slider_row(
            hue,
            format,
            color,
            {let f = on_msg.clone(); move |v| f(ContentMsg::HueChanged(v))},
            {let f = on_msg.clone(); move |fmt| f(ContentMsg::FormatSelected(fmt))},
        ).into();

        let grad_hue_col: Element<M> = column(vec![grad_rgba_row, hue_row])
            .spacing(10.0)
            .into();

        let srow = submit_row(
            show_palette,
            wrap(ContentMsg::Submit),
            wrap(ContentMsg::Cancel),
            wrap(ContentMsg::Copy),
            {let f = on_msg.clone(); move |b| f(ContentMsg::ShowPalette(b))},
        )
        .into();

        let final_col: Element<M> = if show_palette {
            let pal = palette_panel(color);
            let rw = row(vec![pal, grad_hue_col]).spacing(8.0).into();
            column(vec![rw, srow]).spacing(3.0).into()
        } else {
            column(vec![grad_hue_col, srow]).spacing(3.0).into()
        };

        container(final_col)
            .width(if show_palette { 460.0 } else { 370.0 })
            .height(190.0)
            .padding(5.0)
            .into()
    }
}

impl Default for ColorPickerState {
    fn default() -> Self {
        Self::new(70, 30, 200)
    }
}
