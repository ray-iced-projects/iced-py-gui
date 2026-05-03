//! ipg_color_picker
use std::collections::HashMap;

use crate::IpgState;
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback;
use crate::widgets::ipg_button::ButtonStyleStd;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};
use crate::app::Message;
use iced::theme::palette;
use super::callbacks::{invoke_callback_with_args};

use crate::ipg_widgets::ipg_color_picker::color_picker::{Position, ColorPicker as CP, ColorValue};

use iced::Length::Fill;
use iced::widget::{Canvas, Checkbox, TextInput, button, canvas, center, column, container, radio, row, slider, text};
use iced::{Element, Length, Padding, Pixels, Point, Rectangle, Size, Task, Theme};


use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;

#[derive(Debug, Clone)]
pub struct ColorPicker {
    pub id: usize,
    pub opened: bool,
    pub initial_color: Option<ColorValue>,
    pub color_output_format: Option<ColorOutFormat>,
    pub gap: Option<u32>,
    pub snap_within_viewport: Option<bool>,
    pub positions: [Option<bool>; 6],
    
    pub btn_label: Option<String>,
    pub btn_style_id: Option<usize>,
    pub btn_style_std: Option<ButtonStyleStd>,

    pub r_value: u8,
    pub g_value: u8,
    pub b_value: u8,
    pub a_value: u8,
    pub hue_value: u8,
    pub show_palette: bool,
}

impl ColorPicker {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>,
        ) -> Option<Element<'a, Message>> {

        //  For the btn styling
        let style_opt = 
            self.lookup(widgets, self.btn_style_id)
                .and_then(Widgets::as_button_style).cloned();

        let label = if let Some(lbl) = &self.btn_label {
            lbl
        } else {
            "Color Picker"
        };

        let btn = 
            button(label)
                .on_press(ColPikMessage::Noop)
                .style(move |theme: &Theme, status| {
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, status)
                    } else {
                        match &self.btn_style_std {
                            Some(std) => std.to_iced(theme, status),
                            None => button::primary(theme, status),
                        }
                    }
                });
        
        let submit_row = submit_row(self.show_palette).into();

        let r_sld_row = 
            rgba_slider("r", self.r_value, RGBA::R, ColPikMessage::RSldOnChange).into();
        let g_sld_row = 
            rgba_slider("g", self.g_value, RGBA::G, ColPikMessage::GSldOnChange).into();
        let b_sld_row = 
            rgba_slider("b", self.b_value, RGBA::B, ColPikMessage::BSldOnChange).into();
        let a_sld_row = 
            rgba_slider("a", self.a_value, RGBA::A, ColPikMessage::ASldOnChange).into();

        let rgba_col: Element<'_, ColPikMessage> = 
            column(vec![r_sld_row, g_sld_row, b_sld_row, a_sld_row])
            .spacing(5.0)
            .into();

        let grad_cont: Element<'_, ColPikMessage> = Canvas::new(HsvSquare {
                hue: self.hue_value,
                r: self.r_value,
                g: self.g_value,
                b: self.b_value,
            })
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(100.0))
            .into();
        
        // Top row with gradient window and rgba sliders
        let grad_rgba_row = 
            row(vec![grad_cont, rgba_col])
            .width(Length::Fill)
            .spacing(5.0)
            .into();

        // contains a col of hue slider and radios
        // in a row with the selcted color container
        let hue_row = 
            hue_slider_row(
                self.hue_value, 
                self.color_output_format,
                self.current_color()
            ).into();

        let col: Element<ColPikMessage> = {
            let sliders_col: Element<ColPikMessage> =
                column(vec![grad_rgba_row, hue_row])
                    .spacing(10.0)
                    .into();

            if self.show_palette {
                let pal = palette_panel(self.current_color());
                row(vec![pal, sliders_col])
                    .spacing(8.0)
                    .into()
            } else {
                sliders_col
            }
        };

        let content: Element<'_, ColPikMessage> = 
            container(column(vec![col, submit_row]).spacing(10.0))
                .width(if self.show_palette { 460.0 } else { 370.0 })
                .height(190.0)
                .padding(5.0).into();

        let position = get_open_position(self.positions);

        let cp: Element<'_, ColPikMessage> = 
            CP::new(
                btn,
                content,
                self.current_color(),
                position,
            )
            .opened(self.opened)
            .on_open(ColPikMessage::SetOpened)
            .gap(10)
            
            .style(container::rounded_box)
            .into();
        
        Some(center(cp.map(move |message| 
            Message::ColorPicker(self.id, message))).into())

    }

    fn current_color(&self) -> [f32; 4] {
        [
            self.r_value as f32 / 255.0,
            self.g_value as f32 / 255.0,
            self.b_value as f32 / 255.0,
            self.a_value as f32 / 255.0,
        ]
    }

}


fn get_open_position(positions: [Option<bool>; 6]) -> Position {

    let index = positions.iter()
        .position(|p| *p == Some(true)).unwrap_or(5);

    match index {
        0 => Position::FollowCursor,
        1 => Position::Bottom,
        2 => Position::Left,
        3 => Position::Right,
        4 => Position::Top,
        5 | _ => Position::Center,
    }
}

struct HsvSquare {
    hue: u8,
    r: u8,
    g: u8,
    b: u8,
}

/// Internal drag state for the canvas.
#[derive(Default)]
struct HsvSquareState {
    is_dragging: bool,
}

impl canvas::Program<ColPikMessage> for HsvSquare {
    type State = HsvSquareState;

    fn update(
        &self,
        state: &mut HsvSquareState,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> Option<canvas::Action<ColPikMessage>> {
        match event {
            canvas::Event::Mouse(iced::mouse::Event::ButtonPressed(
                iced::mouse::Button::Left,
            )) => {
                if let Some(pos) = cursor.position_in(bounds) {
                    state.is_dragging = true;
                    let (r, g, b) = color_at(pos, bounds.size(), self.hue);
                    return Some(canvas::Action::publish(ColPikMessage::CanvasColorPicked(r, g, b)));
                }
                None
            }
            canvas::Event::Mouse(iced::mouse::Event::CursorMoved { .. }) => {
                if state.is_dragging {
                    if let Some(pos) = cursor.position_in(bounds) {
                        let (r, g, b) = color_at(pos, bounds.size(), self.hue);
                        return Some(canvas::Action::publish(ColPikMessage::CanvasColorPicked(r, g, b)));
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
                        .add_stop(1.0, hue_to_rgb(self.hue))
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
                            Point::new(0.0, 0.0),         // black at top
                        )
                        .add_stop(0.0, iced::Color::TRANSPARENT)
                        .add_stop(1.0, iced::Color::BLACK)
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


#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum ColPikMessage {
    Noop,
    Submit,
    Cancel,
    CopyClipBoard,
    SetOpened(bool),
    ShowPalette(bool),
    RSldOnChange(u8),
    GSldOnChange(u8),
    BSldOnChange(u8),
    ASldOnChange(u8),
    OnHueChange(u8),
    RgbaOnInput(RGBA, String),
    CanvasColorPicked(u8, u8, u8),
    ColorFormatSelected(ColorOutFormat),
}

pub fn color_picker_callback(
    state: &mut IpgState, 
    id: usize, 
    message: ColPikMessage,
) -> Option<Task<Message>> {
        match message {
            ColPikMessage::Submit => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    cp.opened = false;
                let text = selected_color_format_to_text(
                        cp.color_output_format,
                        cp.current_color(),
                    );
                invoke_callback_with_args(id, "on_submit", "ColorPicker", text,
                    "returns color values");
                }
                None
            }
            ColPikMessage::Cancel => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    cp.opened = false;
                    invoke_callback(id, "on_cancel", "ColorPicker");
                }
                None
            },
            ColPikMessage::CopyClipBoard => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    let text = selected_color_format_to_text(
                        cp.color_output_format,
                        cp.current_color(),
                    );
                    return Some(iced::clipboard::write(text).discard::<Message>());
                }
                None
            },
            ColPikMessage::SetOpened(opened) => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    cp.opened = opened;
                    invoke_callback(id, "on_open", "ColorPicker");
                }
                None
            },
            ColPikMessage::RSldOnChange(value) => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    cp.r_value = value;
                    cp.hue_value = rgb_to_hue(cp.r_value, cp.g_value, cp.b_value);
                }
                None
            },
            ColPikMessage::GSldOnChange(value) => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    cp.g_value = value;
                    cp.hue_value = rgb_to_hue(cp.r_value, cp.g_value, cp.b_value);
                }
                None
            },
            ColPikMessage::BSldOnChange(value) => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    cp.b_value = value;
                    cp.hue_value = rgb_to_hue(cp.r_value, cp.g_value, cp.b_value);
                }
                None
            },
            ColPikMessage::ASldOnChange(value) => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    cp.a_value = value;
                }
                None
            },
            ColPikMessage::OnHueChange(value) => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    cp.hue_value = value;
                    let rgb = hue_to_rgb(value);
                    cp.r_value = (rgb.r * 255.0).round() as u8;
                    cp.g_value = (rgb.g * 255.0).round() as u8;
                    cp.b_value = (rgb.b * 255.0).round() as u8;
                }
                None
            },
            ColPikMessage::Noop => None,
            ColPikMessage::CanvasColorPicked(r, g, b) => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    cp.r_value = r;
                    cp.g_value = g;
                    cp.b_value = b;
                    cp.hue_value = rgb_to_hue(r, g, b);
                }
                None
            },
            ColPikMessage::RgbaOnInput(rgba, input) => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    if let Ok(v) = input.parse::<u8>() {
                        match rgba {
                            RGBA::R => { cp.r_value = v; cp.hue_value = rgb_to_hue(cp.r_value, cp.g_value, cp.b_value); }
                            RGBA::G => { cp.g_value = v; cp.hue_value = rgb_to_hue(cp.r_value, cp.g_value, cp.b_value); }
                            RGBA::B => { cp.b_value = v; cp.hue_value = rgb_to_hue(cp.r_value, cp.g_value, cp.b_value); }
                            RGBA::A => { cp.a_value = v; }
                            RGBA::H => {
                                cp.hue_value = v;
                                let rgb = hue_to_rgb(v);
                                cp.r_value = (rgb.r * 255.0).round() as u8;
                                cp.g_value = (rgb.g * 255.0).round() as u8;
                                cp.b_value = (rgb.b * 255.0).round() as u8;
                            }
                        }
                    }
                }
                None
            },
            ColPikMessage::ColorFormatSelected(format) => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    cp.color_output_format = Some(format);
                }
                None
            }
            ColPikMessage::ShowPalette(checked) => {
                if let Some(Widgets::ColorPicker(cp)) = state.widgets.get_mut(&id) {
                    cp.show_palette = checked;
                }
                None
            }
        }
    }


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ColorOutFormat {
    Float,
    Hex,
    Integer,
    Percent,
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ColorPickerParam {
    ColorAlpha,
    ColorRgba,
    Color,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ColorPickerStyleParam {
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRbga,
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
    BorderRadius,
    BorderWidth,
    ShadowColor,
    ShadowColorAlpha,
    ShadowRgba,
    ShadowOffsetX,
    ShadowOffsetY,
    ShadowBlurRadius,
    TextColor,
    TextColorAlpha,
    TextRgba
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for ColorPicker {
    type Param = ColorPickerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ColorPickerParam::ColorAlpha => set_t_value(&mut self.initial_color, value, "ColorPickerParam::ColorAlpha"),
            ColorPickerParam::ColorRgba => set_t_value(&mut self.initial_color, value, "ColorPickerParam::ColorRgba"),
            ColorPickerParam::Color => set_t_value(&mut self.initial_color, value, "ColorPickerParam::Color"),
        }
    }
}


fn hue_slider_row(
    hue_value: u8,
    format: Option<ColorOutFormat>,
    selected_color: [f32; 4],
) -> iced::Element<'static, ColPikMessage> {
    let hue_sld = container(
        slider(0..=255, hue_value, ColPikMessage::OnHueChange)
            .step(1)
            .width(200.0)
            .style(move |theme, status| slider_style(theme, status, RGBA::H, 0))
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
    let rad_int = 
        radio("Int", ColorOutFormat::Integer,format, 
            ColPikMessage::ColorFormatSelected)
            .size(size)
            .text_size(text_size);
    let rad_float = 
        radio("Float", ColorOutFormat::Float, format,
            ColPikMessage::ColorFormatSelected)
            .size(size)
            .text_size(text_size);
    let rad_hex = 
        radio("Hex", ColorOutFormat::Hex, format,
            ColPikMessage::ColorFormatSelected)
            .size(size)
            .text_size(text_size);
    let rad_percent = 
        radio("Percent", ColorOutFormat::Percent, format,
            ColPikMessage::ColorFormatSelected)
            .size(size)
            .text_size(text_size);

    let rad_row = 
        row([rad_int.into(),
        rad_float.into(),
        rad_hex.into(),
        rad_percent.into(), 
        ]).spacing(5.0);

    let col = column([
        hue_sld.into(),
        rad_row.into(),
        ])
        .spacing(5.0).into();

    let bkg = iced::Color::from(selected_color);
    let [r, g, b, _] = selected_color;
    let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let text_color = if luminance > 0.5 { iced::Color::BLACK } else { iced::Color::WHITE };

    let color_label: Element<ColPikMessage> = 
        text(selected_color_format_to_text(format, selected_color))
            .size(Pixels(10.0))
            .color(text_color)
            .into();

    let value_cont = 
        container(color_label)
        .style(move|_| {
            let style = container::background(bkg);
            style
        })
        .center_x(150)
        .center_y(Fill)
        .width(150.0)
        .height(Fill)
        .into();

    row([col, value_cont]).spacing(10.0).into()

}

fn selected_color_format_to_text(
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

fn submit_row(show_palette: bool) -> iced::widget::Row<'static, ColPikMessage> {
    let size = Pixels(12.0);
    let txt: Element<ColPikMessage> = text("Submit").size(size).into();
    
    let submit_btn: Element<ColPikMessage> = 
        button(txt)
            .on_press(ColPikMessage::Submit)
            .padding(5.0)
            .into();

    let txt: Element<ColPikMessage> = text("Cancel").size(size).into();
    let cancel_btn: Element<ColPikMessage> = 
        button(txt)
            .on_press(ColPikMessage::Cancel)
            .padding(5.0)
            .into();

    let txt: Element<ColPikMessage> = text("ClipBoard").size(size).into();
    let clipbrd_btn: Element<ColPikMessage> = 
        button(txt)
            .on_press(ColPikMessage::CopyClipBoard)
            .padding(5.0)
            .into();

    let palette_chk: Element<ColPikMessage> =
        Checkbox::new(show_palette)
            .label("Show Palette")
            .on_toggle(ColPikMessage::ShowPalette)
            .size(12.0)
            .text_size(12.0)
            .into();

    row([submit_btn, cancel_btn, clipbrd_btn, palette_chk])
        .spacing(15.0)
        .align_y(iced::Alignment::Center)
}


fn palette_swatch(label: &'static str, color: iced::Color) -> Element<'static, ColPikMessage> {
    let [r, g, b, _] = [color.r, color.g, color.b, color.a];
    let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let fg = if luminance > 0.45 { iced::Color::BLACK } else { iced::Color::WHITE };
    container(
        text(label).size(10.0)
    )
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

/// Mix `color` with white (t=0) or black (t=1) by factor `amount`.
fn mix(color: [f32; 3], mix_color: [f32; 3], amount: f32) -> iced::Color {
    iced::Color::from_rgb(
        color[0] + (mix_color[0] - color[0]) * amount,
        color[1] + (mix_color[1] - color[1]) * amount,
        color[2] + (mix_color[2] - color[2]) * amount,
    )
}

fn palette_panel(selected: [f32; 4]) -> Element<'static, ColPikMessage> {
    let c = [selected[0], selected[1], selected[2]];
    let white = [1.0f32; 3];
    let black = [0.0f32; 3];

    column(vec![
        palette_swatch("weakest", mix(c, white, 0.80)),
        palette_swatch("weaker",  mix(c, white, 0.55)),
        palette_swatch("weak",    mix(c, white, 0.30)),
        palette_swatch("base",    iced::Color::from_rgb(c[0], c[1], c[2])),
        palette_swatch("strong",  mix(c, black, 0.25)),
        palette_swatch("stronger",mix(c, black, 0.50)),
        palette_swatch("strongest",mix(c, black, 0.70)),
    ])
    .spacing(3.0)
    .width(80.0)
    .into()
}

#[derive(Debug, Clone, Copy)]
pub enum RGBA { R, G, B, A, H}

fn rgba_slider<'a>(
    label: &'a str, 
    value: u8,
    rgba: RGBA,
    on_change: impl Fn(u8) -> ColPikMessage + 'a
) -> iced::widget::Row<'a, ColPikMessage> {

    let sld = 
        slider(0..=255, value, on_change)
            .step(1)
            .width(200.0)
            .style(move|theme, status| {
                slider_style(theme, status, rgba, value)
            });

    let input_text =  
        TextInput::new(
            &"".to_string(), 
            &value.to_string()
        )
        .on_input(move |s| ColPikMessage::RgbaOnInput(rgba, s))
        .size(Pixels(12.0))
        .padding(Padding::default().left(5));

    row(vec![
        text(label.to_owned()).into(), 
        sld.into(), 
        input_text.into()
    ]).spacing(3.0)
    
}

/// Compute the HSV-space color at canvas position `pos` given `hue`.
fn color_at(pos: Point, size: Size, hue: u8) -> (u8, u8, u8) {
    let s = (pos.x / size.width).clamp(0.0, 1.0);
    let v = (pos.y / size.height).clamp(0.0, 1.0);
    let c = hsv_to_rgb(hue, s, v);
    (
        (c.r * 255.0).round() as u8,
        (c.g * 255.0).round() as u8,
        (c.b * 255.0).round() as u8,
    )
}

/// Convert HSV (hue 0-255, sat 0-1, val 0-1) to RGB.
fn hsv_to_rgb(hue: u8, s: f32, v: f32) -> iced::Color {
    let h = (hue as f32 / 255.0) * 360.0;
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match h as u32 {
        0..=59    => (c, x, 0.0),
        60..=119  => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _         => (c, 0.0, x),
    };
    iced::Color::from_rgb(r + m, g + m, b + m)
}

/// Returns (saturation, value) in 0..=1 for the given RGB.
fn rgb_to_sv(r: u8, g: u8, b: u8) -> (f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let s = if max > 0.0 { (max - min) / max } else { 0.0 };
    (s, max)
}

pub fn rgb_to_hue(r: u8, g: u8, b: u8) -> u8 {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    if delta < 1e-6 {
        return 0; // achromatic
    }
    let hue_deg = if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    let hue_deg = if hue_deg < 0.0 { hue_deg + 360.0 } else { hue_deg };
    ((hue_deg / 360.0) * 255.0).round() as u8
}

fn hue_to_rgb(hue: u8) -> iced::Color {
    let h = (hue as f32 / 255.0) * 360.0;
    let c = 1.0_f32;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let (r, g, b) = match h as u32 {
        0..=59    => (c, x, 0.0),
        60..=119  => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _         => (c, 0.0, x),
    };
    iced::Color::from_rgb(r, g, b)
}

fn hue_rail_gradient() -> iced::Background {
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

fn slider_style(
    theme: &Theme, 
    status: slider::Status,
    rgba: RGBA,
    value: u8,
) -> slider::Style {
    let mut style = iced::widget::slider::default(theme, status);

    let base = 
        match rgba {
            RGBA::R => iced::Color::from_rgb(1.0, 0.0, 0.0), // RED
            RGBA::G => iced::Color::from_rgb(0.0, 0.502, 0.0), //GREEN
            RGBA::B => iced::Color::from_rgb(0.0, 0.0, 1.0), //BLUE
            RGBA::A => iced::Color::BLACK,
            RGBA::H => iced::Color::BLACK, // overridden below
        };

    let palette = palette::Background::new(base, iced::Color::WHITE);

    let color = match status {
        slider::Status::Active => palette.base.color,
        slider::Status::Hovered => palette.strong.color,
        slider::Status::Dragged => palette.weak.color,
    };

    let rail_backgrounds = if matches!(rgba, RGBA::H) {
        let transparent = iced::Background::Color(iced::Color::TRANSPARENT);
        (transparent, transparent)
    } else if matches!(rgba, RGBA::A) {
        let alpha = value as f32 / 255.0;
        let bg = iced::Background::Color(iced::Color::from_rgba(0.0, 0.0, 0.0, alpha));
        (bg, bg)
    } else {
        (color.into(), palette.strong.color.into())
    };

    let rail = slider::Rail {
        backgrounds: rail_backgrounds,
        width: 15.0, 
        border: iced::Border {
            radius: 10.0.into(),
            width: 2.0,
            ..Default::default() 
        }
        };
    style.rail = rail;
    style.handle.shape = iced::widget::slider::HandleShape::Rectangle {
        width: 6,
        border_radius: 4.0.into(),
    };
    style
}
