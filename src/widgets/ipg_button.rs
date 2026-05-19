//! Button widget definition
use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::bootstrap_arrow::Arrow;
use crate::graphics::colors::Color;
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback;
use crate::py_api::helpers::{get_len, get_padding, get_radius};
use crate::style::styling::{ColorStatus, palette_pick};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value
};

use iced::widget::text::Wrapping;
use iced::{Border, Shadow, Vector, alignment, gradient};
use iced::widget::{button, text};
use iced::{Element, Theme};
use iced::theme::palette;

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Button {
    pub id: usize,
    pub label: Option<String>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub padding: Option<Vec<f32>>,
    pub clip: Option<bool>,
    pub status_active: Option<bool>,
    pub status_hovered: Option<bool>,
    pub status_pressed: Option<bool>,
    pub status_disabled: Option<bool>,
    pub font_id: Option<usize>,
    pub style_id: Option<usize>,
    pub style_std: Option<ButtonStyleStd>,
    pub style_arrow: Option<Arrow>,
    pub show: bool,
}

impl Button {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }
    
    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>,
        ) -> Option<Element<'a, Message>> {
        
        if !self.show { return None }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_button_style).cloned();

        let font_opt = 
        self.lookup(widgets, self.font_id)
            .and_then(Widgets::as_font).cloned();

        let txt = if let Some(sa) = &self.style_arrow {
            let ar = Arrow::to_string(sa);
            text(ar).font(iced::Font::new("bootstrap-icons"))
        } else {
            if let Some(lbl) = &self.label {
                text(lbl)
            } else {
                text("")
            }
        };

        let txt = if let Some(style) = &style_opt {
            // Center is the default but is overridden by any other alignment
            // so center needs to be set first
            let txt = txt.align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center);

            let txt = 
                if style.text_top_left == Some(true) {
                    txt.align_x(alignment::Horizontal::Left)
                        .align_y(alignment::Vertical::Top)
                } else { txt };

            let txt = 
                if style.text_top_center == Some(true) {
                    txt.align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Top)
                } else { txt };

            let txt = 
                if style.text_top_right == Some(true) {
                    txt.align_x(alignment::Horizontal::Right)
                        .align_y(alignment::Vertical::Top)
                } else { txt };
            
            let txt = 
                if style.text_center_left == Some(true) {
                    txt.align_x(alignment::Horizontal::Left)
                        .align_y(alignment::Vertical::Center)
                } else { txt };

            let txt = 
                if style.text_center_right == Some(true) {
                    txt.align_x(alignment::Horizontal::Right)
                        .align_y(alignment::Vertical::Center)
                } else { txt };

            let txt = 
                if style.text_bottom_left == Some(true) {
                    txt.align_x(alignment::Horizontal::Left)
                        .align_y(alignment::Vertical::Bottom)
                } else { txt };

            let txt = 
                if style.text_bottom_center == Some(true) {
                    txt.align_x(alignment::Horizontal::Center)
                        .align_y(alignment::Vertical::Bottom)
                } else { txt };

            let txt = 
                if style.text_bottom_right == Some(true) {
                    txt.align_x(alignment::Horizontal::Right)
                        .align_y(alignment::Vertical::Bottom)
                } else { txt };
            
            let txt = 
                if let Some(size) = style.text_size {
                    txt.size(size)
                } else {txt};

            let text_color = Color::rgba_ipg_color_to_iced(style.text_rgba, &style.text_color, style.text_color_alpha);

            // If status colors are present, to_iced() handles coloring via
            // button::Style.text_color, and an explicit .color() here would override it.
            let has_status_text = style.text_color_active.is_some()   || style.text_rgba_active.is_some()
                || style.text_color_hovered.is_some()  || style.text_rgba_hovered.is_some()
                || style.text_color_pressed.is_some()  || style.text_rgba_pressed.is_some()
                || style.text_color_disabled.is_some() || style.text_rgba_disabled.is_some();

            let txt = if let Some(tc) = text_color {
                if !has_status_text { txt.color(tc) } else { txt }
            } else { txt };

            // default is word so not checked
            let txt = 
                if style.wrapping_none == Some(true) {
                    txt.wrapping(Wrapping::None)
                } else if style.wrapping_glyph == Some(true) {
                    txt.wrapping(Wrapping::Glyph)
                } else if style.wrapping_word_glyph == Some(true) {
                    txt.wrapping(Wrapping::WordOrGlyph)
                } else { txt };
            txt

        } else { txt };

        let txt = 
            if let Some(f) = font_opt {
                txt.font(f.to_iced())
            } else { txt };

        let txt = 
            if self.clip == Some(true) {
                txt.wrapping(Wrapping::None)
            } else { txt };
            
        let btn_statuses = 
            BtnStatus {
                active: self.status_active,
                hovered: self.status_hovered,
                pressed: self.status_pressed,
                disabled: self.status_disabled,
            };

        let override_st = btn_statuses.to_iced();

        let btn = 
            button(txt)
                .padding(get_padding(&self.padding))
                .on_press(Message::Button(self.id, BtnMessage::OnPress))
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height))
                .style(move |theme: &Theme, status| {
                    let status = override_st.unwrap_or(status);
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, status)
                    } else {
                        match &self.style_std {
                            Some(std) => std.to_iced(theme, status),
                            None => button::primary(theme, status),
                        }
                    }
                }).into();

        Some(btn)

    }
}

#[derive(Debug, Clone)]
pub enum BtnMessage {
    OnPress,
}

pub fn button_callback(id: usize, message: BtnMessage) {
    match message {
        BtnMessage::OnPress => {
            invoke_callback(id, "on_press", "Button");
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ButtonStyle {
    pub id: usize,
    pub bkg_color: Option<Color>,
    pub bkg_color_alpha: Option<f32>,
    pub bkg_rgba: Option<[f32; 4]>,
    // When text color used, all status calculated
    pub text_color: Option<Color>,
    pub text_color_alpha: Option<f32>,
    pub text_rgba: Option<[f32; 4]>,

    pub text_top_left: Option<bool>,
    pub text_top_center: Option<bool>,
    pub text_top_right: Option<bool>,
    pub text_center_left: Option<bool>,
    pub text_center: Option<bool>,
    pub text_center_right: Option<bool>,
    pub text_bottom_left: Option<bool>,
    pub text_bottom_center: Option<bool>,
    pub text_bottom_right: Option<bool>,
    pub text_size: Option<f32>,

    pub wrapping_none: Option<bool>,
    pub wrapping_glyph: Option<bool>,
    pub wrapping_word_glyph: Option<bool>,

    // When a status is used, these override text_color
    pub text_color_active: Option<Color>,
    pub text_color_alpha_active: Option<f32>,
    pub text_rgba_active: Option<[f32; 4]>,

    pub text_color_hovered: Option<Color>,
    pub text_color_alpha_hovered: Option<f32>,
    pub text_rgba_hovered: Option<[f32; 4]>,

    pub text_color_pressed: Option<Color>,
    pub text_color_alpha_pressed: Option<f32>,
    pub text_rgba_pressed: Option<[f32; 4]>,

    pub text_color_disabled: Option<Color>,
    pub text_color_alpha_disabled: Option<f32>,
    pub text_rgba_disabled: Option<[f32; 4]>,
    
    pub gradient_color_stops: Option<Vec<Option<Color>>>,
    pub gradient_color_alpha_stops: Option<Vec<Option<f32>>>,
    pub gradient_rgba_stops: Option<Vec<Option<[f32; 4]>>>,
    pub gradient_offset_stops: Option<Vec<Option<f32>>>,
    pub gradient_degrees: Option<f32>,
    pub gradient_radians: Option<f32>,

    pub border_color_active: Option<Color>,
    pub border_color_alpha_active: Option<f32>,
    pub border_rgba_active: Option<[f32; 4]>,

    pub border_color_hovered: Option<Color>,
    pub border_color_alpha_hovered: Option<f32>,
    pub border_rgba_hovered: Option<[f32; 4]>,

    pub border_color_pressed: Option<Color>,
    pub border_color_alpha_pressed: Option<f32>,
    pub border_rgba_pressed: Option<[f32; 4]>,

    pub border_color_disabled: Option<Color>,
    pub border_color_alpha_disabled: Option<f32>,
    pub border_rgba_disabled: Option<[f32; 4]>,

    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    
    pub shadow_color: Option<Color>,
    pub shadow_color_alpha: Option<f32>,
    pub shadow_rgba: Option<[f32; 4]>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,

    pub snap: Option<bool>,
}

#[derive(Debug, Default)]
pub struct BtnStatus {
    active: Option<bool>,
    hovered: Option<bool>,
    pressed: Option<bool>,
    disabled: Option<bool>,
}

impl BtnStatus {
    fn to_iced(&self) -> Option<button::Status> {
         // override the real status with user-specified one
        if self.disabled == Some(true) {
            Some(button::Status::Disabled)
        } else if self.pressed == Some(true) {
            Some(button::Status::Pressed)
        } else if self.hovered == Some(true) {
            Some(button::Status::Hovered)
        } else if self.active == Some(true) {
            Some(button::Status::Active)
        } else { None }
    }
}

// Standard styles are:
// Background,
// Danger,
// Primary,
// Secondary,
// Subtle (unique settings),
// Success,
// Warning,
// Text,
//
// Status    |  Standard Styles
// Active    |  base
// Hovered   |  strong
// Pressed   |  base
// Disabled  |  base => background scale_alpha(0.5)
//
// Status    |  Text button
// Active    |  base
// Hovered   |  base text scale alpha(0.8)
// Pressed   |  base
// Disabled  |  base => background scale_alpha(0.5)
//
// Status    |  Background Custom Colors
// Active    |  base
// Hovered   |  weak
// Pressed   |  strong
// Disabled  |  base => background scale_alpha(0.5)
//
// Status    |  Standard Style Subtle (unique)
// Active    |  base
// Hovered   |  strong
// Pressed   |  base
// Disabled  |  base => background scale_alpha(0.5)

impl ButtonStyle {
    /// Apply user-defined style overrides to an existing iced button::Style
    pub fn to_iced(
        &self,
        theme: &Theme,
        status: button::Status,
    ) -> button::Style {

        // Convert status to index once: 0=Active, 1=Hovered, 2=Pressed, 3=Disabled
        let idx = match status {
            button::Status::Active   => 0,
            button::Status::Hovered  => 1,
            button::Status::Pressed  => 2,
            button::Status::Disabled => 3,
        };

        let background_color =
            Color::rgba_ipg_color_to_iced(self.bkg_rgba, &self.bkg_color, self.bkg_color_alpha);

        let text_color =
            Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha);

        let bkg_grad_color_stops =
            Color::gradient_stops_to_iced(&self.gradient_rgba_stops, &self.gradient_color_stops, &self.gradient_color_alpha_stops, self.gradient_offset_stops.clone());

        let grad_radians = self.gradient_radians
            .or_else(|| self.gradient_degrees.map(f32::to_radians))
            .unwrap_or(0.0);

        let shd_color =
            Color::rgba_ipg_color_to_iced(self.shadow_rgba, &self.shadow_color, self.shadow_color_alpha);

        // One can use the theme text color but the background
        // is needed together to produce the correct colors.
        // When no background is supplied we default to primary, so pair
        // txt_color with the primary palette text rather than window background text.
        let primary_text = theme.palette().primary.base.text;
        let txt_color = text_color.unwrap_or(
            if background_color.is_none() { primary_text }
            else { theme.palette().background.base.text }
        );

        let bkg = background_color
            .map(|c| palette::Background::new(c, txt_color))
            .unwrap_or_else(|| palette::Background::new(theme.seed().primary, txt_color));

        let bkg_color = palette_pick(&bkg, idx);

        let linear =
            if let Some(stops) = bkg_grad_color_stops {
                let mut linear = gradient::Linear::new(grad_radians);
                for stop in stops.iter() {
                    let sg = palette::Background::new(stop.color, txt_color);
                    linear = linear.add_stop(stop.offset, palette_pick(&sg, idx));
                }
                Some(linear)
            } else { None };

        // Per-status text color logic:
        // - txt_color is the ultimate fallback (from global text_color or theme default).
        // - If text_color_active is set, it acts as the base for all unset statuses.
        // - Any individual status color overrides its own slot.
        let status_base = Color::rgba_ipg_color_to_iced(
            self.text_rgba_active, &self.text_color_active, self.text_color_alpha_active,
        ).unwrap_or(txt_color);
        let text_color = ColorStatus {
            active:   None,  // always resolves to status_base via pick fallback
            hovered:  Color::rgba_ipg_color_to_iced(self.text_rgba_hovered,  &self.text_color_hovered,  self.text_color_alpha_hovered),
            pressed:  Color::rgba_ipg_color_to_iced(self.text_rgba_pressed,  &self.text_color_pressed,  self.text_color_alpha_pressed),
            disabled: Color::rgba_ipg_color_to_iced(self.text_rgba_disabled, &self.text_color_disabled, self.text_color_alpha_disabled),
        }.pick(idx, status_base);

        // Border color: use per-status override, fall back to matching palette color
        let bc_color = ColorStatus {
            active:   Color::rgba_ipg_color_to_iced(self.border_rgba_active,   &self.border_color_active,   self.border_color_alpha_active),
            hovered:  Color::rgba_ipg_color_to_iced(self.border_rgba_hovered,  &self.border_color_hovered,  self.border_color_alpha_hovered),
            pressed:  Color::rgba_ipg_color_to_iced(self.border_rgba_pressed,  &self.border_color_pressed,  self.border_color_alpha_pressed),
            disabled: Color::rgba_ipg_color_to_iced(self.border_rgba_disabled, &self.border_color_disabled, self.border_color_alpha_disabled),
        }.pick(idx, bkg_color);

        let shadow =
            if shd_color.is_some() && self.shadow_blur_radius.is_some() {
                let sg = palette::Background::new(shd_color.unwrap(), txt_color);
                let offset = self.shadow_offset_xy
                    .map(|of| Vector { x: of[0], y: of[1] })
                    .unwrap_or_default();
                Shadow {
                    color: palette_pick(&sg, idx),
                    offset,
                    blur_radius: self.shadow_blur_radius.unwrap_or(0.0),
                }
            } else { Shadow::default() };

        let radius = self.border_radius.as_ref()
            .map(|rd| get_radius(rd, "button".to_string()))
            .unwrap_or_default();

        let width = self.border_width.unwrap_or(0.0);
        let snap  = self.snap.unwrap_or(false);

        let background = if let Some(lin) = linear {
            Some(iced::Background::Gradient(lin.into()))
        } else {
            Some(bkg_color.into())
        };

        button::Style {
            background,
            text_color,
            border: Border { color: bc_color, width, radius },
            shadow,
            snap,
        }
    }

}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ButtonStyleStd {
    Background,
    Danger,
    Primary,
    Secondary,
    Subtle,
    Success,
    Warning,
    Text,
}

impl ButtonStyleStd {
    pub fn to_iced(
        &self,
        theme: &Theme, 
        status: button::Status, 
        ) -> button::Style {
        
        match self {
            ButtonStyleStd::Background => {
                button::background(theme, status)
            },
            ButtonStyleStd::Danger => {
                button::danger(theme, status)
            },
            ButtonStyleStd::Primary => {
                button::primary(theme, status)
            },
            ButtonStyleStd::Secondary => {
                button::secondary(theme, status)
            },
            ButtonStyleStd::Subtle => {
                button::subtle(theme, status)
            },
            ButtonStyleStd::Success => {
                button::success(theme, status)
            },
            ButtonStyleStd::Warning => {
                button::warning(theme, status)
            },
            ButtonStyleStd::Text => {
                button::text(theme, status)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ButtonParam {
    Clip,
    Fill,
    FontId,
    Height,
    HeightFill,
    Label,
    Padding,
    Show,
    StatusActive,
    StatusDisabled,
    StatusHovered,
    StatusPressed,
    StyleArrow,
    StyleId,
    StyleStd,
    Width,
    WidthFill,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ButtonStyleParam {
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRgba,

    TextColor,
    TextColorAlpha,
    TextRgba,

    TextAlignBottomCenter,
    TextAlignBottomLeft,
    TextAlignBottomRight,
    TextAlignCenter,
    TextAlignCenterLeft,
    TextAlignCenterRight,
    TextAlignTopCenter,
    TextAlignTopLeft,
    TextAlignTopRight,
    TextSize,

    WrappingNone,
    WrappingGlyph,
    WrappingWordGlyph,

    TextColorActive,
    TextColorAlphaActive,
    TextRgbaActive,

    TextColorHovered,
    TextColorAlphaHovered,
    TextRgbaHovered,

    TextColorPressed,
    TextColorAlphaPressed,
    TextRgbaPressed,

    TextColorDisabled,
    TextColorAlphaDisabled,
    TextRgbaDisabled,

    GradientColorStops,
    GradientColorAlphaStops,
    GradientRgbaStops,
    GradientDegrees,
    GradientRadians,

    BorderColorActive,
    BorderColorAlphaActive,
    BorderRgbaActive,

    BorderColorHovered,
    BorderColorAlphaHovered,
    BorderRgbaHovered,

    BorderColorPressed,
    BorderColorAlphaPressed,
    BorderRgbaPressed,

    BorderColorDisabled,
    BorderColorAlphaDisabled,
    BorderRgbaDisabled,

    BorderRadius,
    BorderWidth,

    ShadowColor,
    ShadowColorAlpha,
    ShadowRgba,
    ShadowOffsetXy,
    ShadowBlurRadius,

    Snap,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Button {
    type Param = ButtonParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ButtonParam::Clip => set_t_value(&mut self.clip, value, "ButtonParam::Clip"),
            ButtonParam::Fill => set_t_value(&mut self.fill, value, "ButtonParam::Fill"),
            ButtonParam::FontId => set_t_value(&mut self.font_id, value, "ButtonParam::FontId"),
            ButtonParam::Height => set_t_value(&mut self.height, value, "ButtonParam::Height"),
            ButtonParam::HeightFill => set_t_value(&mut self.height, value, "ButtonParam::HeightFill"),
            ButtonParam::Label => set_t_value(&mut self.label, value, "ButtonParam::Label"),
            ButtonParam::Padding => set_t_value(&mut self.padding, value, "ButtonParam::Padding"),
            ButtonParam::Show => set_t_value(&mut self.show, value, "Show"),
            ButtonParam::StatusActive => set_t_value(&mut self.status_active, value, "ButtonParam::StatusActive"),
            ButtonParam::StatusDisabled => set_t_value(&mut self.status_disabled, value, "ButtonParam::StatusDisabled"),
            ButtonParam::StatusHovered => set_t_value(&mut self.status_hovered, value, "uttonParam::StatusHovered"),
            ButtonParam::StatusPressed => set_t_value(&mut self.status_pressed, value, "ButtonParam::StatusPressed"),
            ButtonParam::StyleArrow => set_t_value(&mut self.style_arrow, value, "ButtonParam::StyleArrow"),
            ButtonParam::StyleId => set_t_value(&mut self.style_id, value, "ButtonParam::StyleId"),
            ButtonParam::StyleStd => set_t_value(&mut self.style_std, value, "ButtonParam::StyleStd"),
            ButtonParam::Width => set_t_value(&mut self.width, value, "ButtonParam::Width"),
            ButtonParam::WidthFill => set_t_value(&mut self.width, value, "ButtonParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for ButtonStyle {
    type Param = ButtonStyleParam;
    
    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ButtonStyleParam::BackgroundColor => set_t_value(&mut self.bkg_color, value, "ButtonStyleParam::BackgroundColor"),
            ButtonStyleParam::BackgroundColorAlpha => set_t_value(&mut self.bkg_color_alpha, value, "ButtonStyleParam::BackgroundColorAlpha"),
            ButtonStyleParam::BackgroundRgba => set_t_value(&mut self.bkg_rgba, value, "ButtonStyleParam::BackgroundRgba"),
            ButtonStyleParam::TextColor => set_t_value(&mut self.text_color, value, "ButtonStyleParam::TextColor"),
            ButtonStyleParam::TextColorAlpha => set_t_value(&mut self.text_color_alpha, value, "ButtonStyleParam::TextColorAlpha"),
            ButtonStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "ButtonStyleParam::TextRgba"),
            ButtonStyleParam::TextAlignBottomCenter => set_t_value(&mut self.text_bottom_center, value, "ButtonStyleParam::TextAlignBottomCenter"),
            ButtonStyleParam::TextAlignBottomLeft => set_t_value(&mut self.text_bottom_left, value, "ButtonStyleParam::TextAlignBottomLeft"),
            ButtonStyleParam::TextAlignBottomRight => set_t_value(&mut self.text_bottom_right, value, "ButtonStyleParam::TextAlignBottomRight"),
            ButtonStyleParam::TextAlignCenter => set_t_value(&mut self.text_center, value, "ButtonStyleParam::TextAlignCenter"),
            ButtonStyleParam::TextAlignCenterLeft => set_t_value(&mut self.text_center_left, value, "ButtonStyleParam::TextAlignCenterLeft"),
            ButtonStyleParam::TextAlignCenterRight => set_t_value(&mut self.text_center_right, value, "ButtonStyleParam::TextAlignCenterRight"),
            ButtonStyleParam::TextAlignTopCenter => set_t_value(&mut self.text_top_center, value, "ButtonStyleParam::TextAlignTopCenter"),
            ButtonStyleParam::TextAlignTopLeft => set_t_value(&mut self.text_top_left, value, "ButtonStyleParam::TextAlignTopLeft"),
            ButtonStyleParam::TextAlignTopRight => set_t_value(&mut self.text_top_right, value, "ButtonStyleParam::TextAlignTopRight"),
            ButtonStyleParam::TextSize => set_t_value(&mut self.text_size, value, "ButtonStyleParam::TextSize"),
            ButtonStyleParam::TextColorActive => set_t_value(&mut self.text_color_active, value, "ButtonStyleParam::TextColorActive"),
            ButtonStyleParam::TextColorAlphaActive => set_t_value(&mut self.text_color_alpha_active, value, "ButtonStyleParam::TextColorAlphaActive"),
            ButtonStyleParam::TextRgbaActive => set_t_value(&mut self.text_rgba_active, value, "ButtonStyleParam::TextRgbaActive"),
            ButtonStyleParam::TextColorHovered => set_t_value(&mut self.text_color_hovered, value, "ButtonStyleParam::TextColorHovered"),
            ButtonStyleParam::TextColorAlphaHovered => set_t_value(&mut self.text_color_alpha_hovered, value, "ButtonStyleParam::TextColorAlphaHovered"),
            ButtonStyleParam::TextRgbaHovered => set_t_value(&mut self.text_rgba_hovered, value, "ButtonStyleParam::TextRgbaHovered"),
            ButtonStyleParam::TextColorPressed => set_t_value(&mut self.text_color_pressed, value, "ButtonStyleParam::TextColorPressed"),
            ButtonStyleParam::TextColorAlphaPressed => set_t_value(&mut self.text_color_alpha_pressed, value, "ButtonStyleParam::TextColorAlphaPressed"),
            ButtonStyleParam::TextRgbaPressed => set_t_value(&mut self.text_rgba_pressed, value, "ButtonStyleParam::TextRgbaPressed"),
            ButtonStyleParam::TextColorDisabled => set_t_value(&mut self.text_color_disabled, value, "ButtonStyleParam::TextColorDisabled"),
            ButtonStyleParam::TextColorAlphaDisabled => set_t_value(&mut self.text_color_alpha_disabled, value, "ButtonStyleParam::TextColorAlphaDisabled"),
            ButtonStyleParam::TextRgbaDisabled => set_t_value(&mut self.text_rgba_disabled, value, "ButtonStyleParam::TextRgbaDisabled"),
            ButtonStyleParam::GradientColorStops => set_t_value(&mut self.gradient_color_stops, value, "ButtonStyleParam::GradientColorStops"),
            ButtonStyleParam::GradientColorAlphaStops => set_t_value(&mut self.gradient_color_alpha_stops, value, "ButtonStyleParam::GradientColorAlphaStops"),
            ButtonStyleParam::GradientRgbaStops => set_t_value(&mut self.gradient_rgba_stops, value, "ButtonStyleParam::GradientRgbaStops"),
            ButtonStyleParam::GradientDegrees => set_t_value(&mut self.gradient_degrees, value, "ButtonStyleParam::GradientDegrees"),
            ButtonStyleParam::GradientRadians => set_t_value(&mut self.gradient_radians, value, "ButtonStyleParam::GradientRadians"),
            ButtonStyleParam::BorderColorActive => set_t_value(&mut self.border_color_active, value, "ButtonStyleParam::BorderColorActive"),
            ButtonStyleParam::BorderColorAlphaActive => set_t_value(&mut self.border_color_alpha_active, value, "ButtonStyleParam::BorderColorAlphaActive"),
            ButtonStyleParam::BorderRgbaActive => set_t_value(&mut self.border_rgba_active, value, "ButtonStyleParam::BorderRgbaActive"),
            ButtonStyleParam::BorderColorHovered => set_t_value(&mut self.border_color_hovered, value, "ButtonStyleParam::BorderColorHovered"),
            ButtonStyleParam::BorderColorAlphaHovered => set_t_value(&mut self.border_color_alpha_hovered, value, "ButtonStyleParam::BorderColorAlphaHovered"),
            ButtonStyleParam::BorderRgbaHovered => set_t_value(&mut self.border_rgba_hovered, value, "ButtonStyleParam::BorderRgbaHovered"),
            ButtonStyleParam::BorderColorPressed => set_t_value(&mut self.border_color_pressed, value, "ButtonStyleParam::BorderColorPressed"),
            ButtonStyleParam::BorderColorAlphaPressed => set_t_value(&mut self.border_color_alpha_pressed, value, "ButtonStyleParam::BorderColorAlphaPressed"),
            ButtonStyleParam::BorderRgbaPressed => set_t_value(&mut self.border_rgba_pressed, value, "ButtonStyleParam::BorderRgbaPressed"),
            ButtonStyleParam::BorderColorDisabled => set_t_value(&mut self.border_color_disabled, value, "ButtonStyleParam::BorderColorDisabled"),
            ButtonStyleParam::BorderColorAlphaDisabled => set_t_value(&mut self.border_color_alpha_disabled, value, "ButtonStyleParam::BorderColorAlphaDisabled"),
            ButtonStyleParam::BorderRgbaDisabled => set_t_value(&mut self.border_rgba_disabled, value, "ButtonStyleParam::BorderRgbaDisabled"),
            ButtonStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "ButtonStyleParam::BorderRadius"),
            ButtonStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "ButtonStyleParam::BorderWidth"),
            ButtonStyleParam::ShadowColor => set_t_value(&mut self.shadow_color, value, "ButtonStyleParam::ShadowColor"),
            ButtonStyleParam::ShadowColorAlpha => set_t_value(&mut self.shadow_color_alpha, value, "ButtonStyleParam::ShadowColorAlpha"),
            ButtonStyleParam::ShadowRgba => set_t_value(&mut self.shadow_rgba, value, "ButtonStyleParam::ShadowRgba"),
            ButtonStyleParam::ShadowOffsetXy => set_t_value(&mut self.shadow_offset_xy, value, "ButtonStyleParam::ShadowOffsetXy"),
            ButtonStyleParam::ShadowBlurRadius => set_t_value(&mut self.shadow_blur_radius, value, "ButtonStyleParam::ShadowBlurRadius"),
            ButtonStyleParam::Snap => set_t_value(&mut self.snap, value, "ButtonStyleParam::Snap"),
            ButtonStyleParam::WrappingNone => set_t_value(&mut self.wrapping_none, value, "ButtonStyleParam::WrappingNone"),
            ButtonStyleParam::WrappingGlyph => set_t_value(&mut self.wrapping_glyph, value, "ButtonStyleParam::WrappingGlyph"),
            ButtonStyleParam::WrappingWordGlyph => set_t_value(&mut self.wrapping_word_glyph, value, "ButtonStyleParam::WrappingWordGlyph"),
        }
    }
}
