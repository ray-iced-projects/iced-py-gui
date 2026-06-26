//! Button widget definition
use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::bootstrap::bootstrap_arrow::Arrow;
use crate::graphics::colors::Color;
use crate::py_api::colors::{CustomPalette, PaletteKey, StateVariant, StylePart, WidgetStatus};
use crate::state::Widgets;
use crate::widgets::callbacks::invoke_callback;
use crate::py_api::helpers::{get_len, get_padding, get_radius};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value
};

use iced::widget::text::Wrapping;
use iced::{Shadow, Vector, alignment, gradient};
use iced::widget::{button, text};
use iced::{Element, Theme};

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
    pub disabled: Option<bool>,
    pub font_id: Option<usize>,
    pub style_id: Option<usize>,
    pub style_std: Option<ButtonStyleStd>,
    pub style_arrow: Option<Arrow>,
    pub palette_id: Option<usize>,
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

        let c_pal_opt = 
        self.lookup(widgets, self.palette_id)
            .and_then(Widgets::as_palette).cloned();

        let txt = if let Some(sa) = &self.style_arrow {
            let ar = sa.to_char().to_string();
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
            
        let btn = 
            button(txt)
                .padding(get_padding(&self.padding))
                .on_press(Message::Button(self.id, BtnMessage::OnPress))
                .width(get_len(self.fill, self.width_fill, self.width))
                .height(get_len(self.fill, self.height_fill, self.height))
                .style(move |theme: &Theme, status| {
                    let status = if self.disabled == Some(true) {
                        button::Status::Disabled
                    } else { status };
                    if style_opt.is_some() || c_pal_opt.is_some() {
                        let btn_st = ButtonStyle::default();
                        let st = style_opt.as_ref().unwrap_or(&btn_st);
                        st.to_iced(theme, status, &c_pal_opt, &self.style_std)
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

    pub gradient_color_stops: Option<Vec<Option<Color>>>,
    pub gradient_color_alpha_stops: Option<Vec<Option<f32>>>,
    pub gradient_rgba_stops: Option<Vec<Option<[f32; 4]>>>,
    pub gradient_offset_stops: Option<Vec<Option<f32>>>,
    pub gradient_degrees: Option<f32>,
    pub gradient_radians: Option<f32>,

    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    
    pub shadow_color: Option<Color>,
    pub shadow_color_alpha: Option<f32>,
    pub shadow_rgba: Option<[f32; 4]>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,

    pub snap: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ButtonStatus {
    Active,
    Hovered,
    Pressed,
    Disabled,
}

impl ButtonStyle {
    /// Apply user-defined style overrides to an existing iced button::Style
    pub fn to_iced(
        &self,
        theme: &Theme,
        status: button::Status,
        c_pal_opt: &Option<CustomPalette>,
        style_std: &Option<ButtonStyleStd>,
    ) -> button::Style {

        let shd_color =
            Color::rgba_ipg_color_to_iced(self.shadow_rgba, &self.shadow_color, self.shadow_color_alpha);

        let bd_color =
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);

        let shadow =
            if shd_color.is_some() && self.shadow_blur_radius.is_some() {
                let offset = self.shadow_offset_xy
                    .map(|of| Vector { x: of[0], y: of[1] })
                    .unwrap_or_default();
                Shadow {
                    color: shd_color.unwrap(),
                    offset,
                    blur_radius: self.shadow_blur_radius.unwrap(),
                }
            } else { Shadow::default() };

        let radius = self.border_radius.as_ref()
            .map(|rd| get_radius(rd, "button".to_string()))
            .unwrap_or(2.0.into());

        let snap  = self.snap.unwrap_or(false);

        // style_std overides all except for border, shadow, and snap
        if style_std.is_some() || c_pal_opt.is_none() {
            let mut style =  if let Some(std) = style_std {
                std.to_iced(theme, status)
            } else { button::primary(theme, status) };
            
            style.shadow = shadow;

            let mut border = iced::Border::default();
            border.color = bd_color.unwrap_or_default();
            border.radius = radius;
            border.width = self.border_width.unwrap_or_default();
            style.border = border;
            style.snap = self.snap.unwrap_or_default();

            return style
        }

        let bkg_grad_color_stops =
            Color::gradient_stops_to_iced(&self.gradient_rgba_stops, &self.gradient_color_stops, &self.gradient_color_alpha_stops, self.gradient_offset_stops.clone());

        let grad_radians = self.gradient_radians
            .or_else(|| self.gradient_degrees.map(f32::to_radians))
            .unwrap_or(0.0);

        let custom_pal = if let Some(cp) = c_pal_opt {
            cp
        } else {
            let palette = theme.palette();
            let background = palette.background;
            &CustomPalette { 
                id: 0, 
                palette: background, 
                statuses: None, 
            }
         };

        let mut default_statuses: HashMap<WidgetStatus, HashMap<StylePart, (PaletteKey, f32)>> = HashMap::new();
        
        let mut inner = HashMap::new();
        inner.insert(StylePart::Background, (PaletteKey::Base,  1.0));
        inner.insert(StylePart::Text, (PaletteKey::Base, 1.0));
        inner.insert(StylePart::Border, (PaletteKey::Base, 1.0));
        default_statuses.insert(WidgetStatus::Active, inner);
                                                    
        let mut inner = HashMap::new();
        inner.insert(StylePart::Background, (PaletteKey::Base,  1.0));
        inner.insert(StylePart::Text, (PaletteKey::Base, 1.0));
        inner.insert(StylePart::Border, (PaletteKey::Base, 1.0));
        default_statuses.insert(WidgetStatus::Pressed, inner);
                
        let mut inner = HashMap::new();
        inner.insert(StylePart::Background, (PaletteKey::Strong,  1.0));
        inner.insert(StylePart::Text, (PaletteKey::Strong, 1.0));
        inner.insert(StylePart::Border, (PaletteKey::Strong, 1.0));
        default_statuses.insert(WidgetStatus::Hovered, inner);
         
        let mut inner = HashMap::new();
        inner.insert(StylePart::Background, (PaletteKey::Base,  0.5));
        inner.insert(StylePart::Text, (PaletteKey::Base, 0.8));
        inner.insert(StylePart::Background, (PaletteKey::Base,  0.5));
        inner.insert(StylePart::Border, (PaletteKey::Strong, 0.5));
        default_statuses.insert(WidgetStatus::Disabled, inner);
        
        let statuses = if let Some(status) = custom_pal.statuses.as_ref() {
            let mut collapsed: HashMap<WidgetStatus, HashMap<StylePart, (PaletteKey, f32)>> = HashMap::new();

            for ((widget_status, variant), parts) in status {
                let part_map: HashMap<StylePart, (PaletteKey, f32)> = parts
                    .iter()
                    .map(|(style_part, palette_key, alpha)| {
                        (style_part.clone(), (palette_key.clone(), *alpha))
                    })
                    .collect();

                // Button has no variant dimension; prefer Unchecked if provided.
                if *variant == StateVariant::Unchecked || !collapsed.contains_key(widget_status) {
                    collapsed.insert(widget_status.clone(), part_map);
                }
            }

            let mut merged = collapsed;

            for (default_widget_status, default_parts) in &default_statuses {
                let merged_parts = merged
                    .entry(default_widget_status.clone())
                    .or_insert_with(|| default_parts.clone());

                for (default_style_part, default_pair) in default_parts {
                    merged_parts
                        .entry(default_style_part.clone())
                        .or_insert_with(|| default_pair.clone());
                }
            }

            merged
        } else { default_statuses };

        // Build gradient background once; used in every match arm when stops are supplied.
        let gradient_background: Option<iced::Background> =
            bkg_grad_color_stops.map(|stops| {
                let mut linear = gradient::Linear::new(grad_radians);
                for stop in stops {
                    linear = linear.add_stop(stop.offset, stop.color);
                }
                iced::Background::Gradient(linear.into())
            });

        let cust_color = custom_pal.palette;
        let theme_color = theme.palette().background;

        let resolve_parts = |widget_status: WidgetStatus| {
            let parts = statuses.get(&widget_status).unwrap();
            let (bkg_key, alpha) = parts.get(&StylePart::Background).unwrap();
            let (text_key, text_alpha) = parts.get(&StylePart::Text).unwrap();
            let (bd_key, bd_alpha) = parts.get(&StylePart::Border).unwrap();

            let background_color = bkg_key.pal_key_to_color(&theme_color, &cust_color).scale_alpha(*alpha);
            let text_color = text_key.pal_key_to_color(&theme_color, &cust_color).scale_alpha(*text_alpha);
            let border_color = bd_key.pal_key_to_color(&theme_color, &cust_color).scale_alpha(*bd_alpha);

            // The style border color overrides the palette border color
            let bd_color = 
                if let Some(bc) = bd_color {
                    bc
                } else { border_color };
            
            (background_color, text_color, bd_color, *alpha)
        };

        match status {
            button::Status::Active => {
                let (bkg_color, text_color, b_color, _) = resolve_parts(WidgetStatus::Active);
                button::Style {
                    background: gradient_background.clone()
                        .or(Some(iced::Background::Color(bkg_color))),
                    text_color,
                    border: iced::Border{
                        color: b_color,
                        width: self.border_width.unwrap_or_default(),
                        radius,
                    },
                    shadow,
                    snap,
                }
            },
            button::Status::Pressed => {
                let (bkg_color, text_color, b_color, _) = resolve_parts(WidgetStatus::Pressed);
                button::Style {
                    background: gradient_background.clone()
                        .or(Some(iced::Background::Color(bkg_color))),
                    text_color,
                    border: iced::Border{
                        color: b_color,
                        width: self.border_width.unwrap_or_default(),
                        radius,
                    },
                    shadow,
                    snap,
                }
            },
            button::Status::Hovered => {
                let (bkg_color, text_color, b_color, _) = resolve_parts(WidgetStatus::Hovered);
                button::Style {
                    background: gradient_background.clone()
                        .or(Some(iced::Background::Color(bkg_color))),
                    text_color,
                    border: iced::Border{
                        color: b_color,
                        width: self.border_width.unwrap_or_default(),
                        radius,
                    },
                    shadow,
                    snap,
                }
            },
            button::Status::Disabled => {
                let (bkg_color, text_color, b_color, alpha) = resolve_parts(WidgetStatus::Disabled);
                let background = gradient_background.clone()
                    .map(|g| g.scale_alpha(alpha))
                    .or(Some(iced::Background::Color(bkg_color)));
                button::Style {
                    background,
                    text_color,
                    border: iced::Border{
                        color: b_color,
                        width: self.border_width.unwrap_or_default(),
                        radius,
                    },
                    shadow,
                    snap,
                }
            },
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
    Disabled,
    Fill,
    FontId,
    Height,
    HeightFill,
    Label,
    Padding,
    Show,
    StyleArrow,
    StyleId,
    StyleStd,
    Width,
    WidthFill,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ButtonStyleParam {
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

    GradientColorStops,
    GradientColorAlphaStops,
    GradientRgbaStops,
    GradientDegrees,
    GradientRadians,

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
            ButtonParam::Disabled => set_t_value(&mut self.disabled, value, "ButtonParam::Disabled"),
            ButtonParam::Fill => set_t_value(&mut self.fill, value, "ButtonParam::Fill"),
            ButtonParam::FontId => set_t_value(&mut self.font_id, value, "ButtonParam::FontId"),
            ButtonParam::Height => set_t_value(&mut self.height, value, "ButtonParam::Height"),
            ButtonParam::HeightFill => set_t_value(&mut self.height, value, "ButtonParam::HeightFill"),
            ButtonParam::Label => set_t_value(&mut self.label, value, "ButtonParam::Label"),
            ButtonParam::Padding => set_t_value(&mut self.padding, value, "ButtonParam::Padding"),
            ButtonParam::Show => set_t_value(&mut self.show, value, "Show"),
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
            ButtonStyleParam::GradientColorStops => set_t_value(&mut self.gradient_color_stops, value, "ButtonStyleParam::GradientColorStops"),
            ButtonStyleParam::GradientColorAlphaStops => set_t_value(&mut self.gradient_color_alpha_stops, value, "ButtonStyleParam::GradientColorAlphaStops"),
            ButtonStyleParam::GradientRgbaStops => set_t_value(&mut self.gradient_rgba_stops, value, "ButtonStyleParam::GradientRgbaStops"),
            ButtonStyleParam::GradientDegrees => set_t_value(&mut self.gradient_degrees, value, "ButtonStyleParam::GradientDegrees"),
            ButtonStyleParam::GradientRadians => set_t_value(&mut self.gradient_radians, value, "ButtonStyleParam::GradientRadians"),
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
