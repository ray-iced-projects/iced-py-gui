//! Button widget definition
use std::collections::HashMap;

use crate::app::Message;
use crate::graphics::colors::Color;
use crate::py_api::colors::{CustomPalette, PaletteKey, StateVariant, StylePart, WidgetStatus};
use crate::py_api::helpers::{get_len, get_padding, get_radius};
use crate::state::Widgets;
use crate::widgets::ipg_menu::MenuMessage;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};

use iced::Widget;
use iced::widget::{button, column};
use iced::{Element, Theme};
use iced::{Shadow, Vector, gradient};

use pyo3::types::{PyDict, PyDictMethods, PyList, PyListMethods};
use pyo3::{Bound, Py, PyAny, PyResult, Python, pyclass};
type PyObject = Py<PyAny>;

/// Represents the user-forced button status, prioritizing explicit states over iced's computed status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UserButtonStatus {
    Active,
    Hovered,
    Pressed,
    Disabled,
    None,
}

#[derive(Debug, Clone)]
pub struct MenuButton {
    pub id: usize,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub height: Option<f32>,
    pub height_fill: Option<bool>,
    pub fill: Option<bool>,
    pub padding: Option<Vec<f32>>,
    pub clip: Option<bool>,
    pub style_id: Option<usize>,
    pub style_std: Option<MenuButtonStyleStd>,
    pub palette_id: Option<usize>,
    pub show: bool,
    pub active: Option<bool>,
    pub hovered: Option<bool>,
    pub pressed: Option<bool>,
    pub disabled: Option<bool>,
}

impl MenuButton {
    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        content: Vec<Element<'a, Message>>,
        widgets: &HashMap<usize, Widgets>
    ) -> Option<Element<'a, Message>> {
        if !self.show {
            return None;
        }

        let style_opt = self
            .lookup(widgets, self.style_id)
            .and_then(Widgets::as_menu_button_style)
            .cloned();

        let c_pal_opt = self
            .lookup(widgets, self.palette_id)
            .and_then(Widgets::as_palette)
            .cloned();

        let user_status = match (self.active, self.hovered, self.pressed, self.disabled) {
            (Some(true), None, None, None) => UserButtonStatus::Active,
            (None, Some(true), None, None) => UserButtonStatus::Hovered,
            (None, None, Some(true), None) => UserButtonStatus::Pressed,
            (None, None, None, Some(true)) => UserButtonStatus::Disabled,
            _ => UserButtonStatus::None,
        };

        let btn = button(column(content))
            .padding(get_padding(&self.padding))
            .on_press(Message::Menu(self.id, MenuMessage::OnPress))
            .width(get_len(self.fill, self.width_fill, self.width))
            .height(get_len(self.fill, self.height_fill, self.height))
            .style(move |theme: &Theme, status| {
                let status = match user_status {
                    UserButtonStatus::Active => button::Status::Active,
                    UserButtonStatus::Hovered => button::Status::Hovered,
                    UserButtonStatus::Pressed => button::Status::Pressed,
                    UserButtonStatus::Disabled => button::Status::Disabled,
                    UserButtonStatus::None => status,
                };
                if style_opt.is_some() || c_pal_opt.is_some() {
                    let btn_st = MenuButtonStyle::default();
                    let st = style_opt.as_ref().unwrap_or(&btn_st);
                    st.to_iced(theme, status, &c_pal_opt, &self.style_std)
                } else {
                    match &self.style_std {
                        Some(std) => std.to_iced(theme, status),
                        None => button::text(theme, status),
                    }
                }
            })
            .boxed();

        Some(btn)
        
    }
}

impl MenuButton {
    /// Serialize all button parameters into a Python dict keyed by field name.
    pub fn to_py_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("id", self.id)?;
        dict.set_item("width", self.width)?;
        dict.set_item("width_fill", self.width_fill)?;
        dict.set_item("height", self.height)?;
        dict.set_item("height_fill", self.height_fill)?;
        dict.set_item("fill", self.fill)?;
        dict.set_item("padding", self.padding.clone())?;
        dict.set_item("clip", self.clip)?;
        dict.set_item("style_id", self.style_id)?;
        dict.set_item("style_std", self.style_std.as_ref().map(|s| format!("{s:?}")))?;
        dict.set_item("palette_id", self.palette_id)?;
        dict.set_item("show", self.show)?;
        dict.set_item("active", self.active)?;
        dict.set_item("hovered", self.hovered)?;
        dict.set_item("pressed", self.pressed)?;
        dict.set_item("disabled", self.disabled)?;
        Ok(dict)
    }
}

#[derive(Debug, Clone, Default)]
pub struct MenuButtonStyle {
    pub id: usize,

    pub gradient_color_stops: Option<Vec<Color>>,
    pub gradient_color_alpha_stops: Option<Vec<f32>>,
    pub gradient_rgba_stops: Option<Vec<[f32; 4]>>,
    pub gradient_offset_stops: Option<Vec<f32>>,
    pub gradient_degrees: Option<f32>,
    pub gradient_radians: Option<f32>,

    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub border_rounded: Option<f32>,

    pub shadow_color: Option<Color>,
    pub shadow_color_alpha: Option<f32>,
    pub shadow_rgba: Option<[f32; 4]>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,

    pub snap: Option<bool>,
}

impl MenuButtonStyle {
    // Apply user-defined style overrides to an existing iced button::Style
    pub fn to_iced(
        &self,
        theme: &Theme,
        status: button::Status,
        c_pal_opt: &Option<CustomPalette>, // the palette using the palette_id
        style_std: &Option<MenuButtonStyleStd>,
    ) -> button::Style {
        let shd_color = Color::rgba_ipg_color_to_iced(self.shadow_rgba, &self.shadow_color, self.shadow_color_alpha);

        let shadow = if let (Some(color), Some(blur_radius)) = (shd_color, self.shadow_blur_radius) {
            let offset = self
                .shadow_offset_xy
                .map(|of| Vector { x: of[0], y: of[1] })
                .unwrap_or_default();
            Shadow {
                color,
                offset,
                blur_radius,
            }
        } else {
            Shadow::default()
        };

        let brd_color = Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);

        let radius = self
            .border_radius
            .as_ref()
            .map(|rd| get_radius(rd, "button".to_string()))
            .unwrap_or(2.0.into());

        let snap = self.snap.unwrap_or(false);

        // style_std overides all except for border, shadow, and snap
        if style_std.is_some() || c_pal_opt.is_none() {
            let mut style = if let Some(std) = style_std {
                std.to_iced(theme, status)
            } else {
                button::primary(theme, status)
            };

            style.shadow = shadow;

            let border = if let Some(rounded) = self.border_rounded {
                iced::border::rounded(rounded)
            } else {
                if let Some(width) = self.border_width {
                    iced::Border {
                        color: brd_color.unwrap_or_default(),
                        radius: radius,
                        width: width,
                    }
                } else {
                    iced::border::rounded(2)
                }
            };

            style.border = border;
            style.snap = self.snap.unwrap_or_default();

            return style;
        }

        let bkg_grad_color_stops = Color::gradient_stops_to_iced(
            &self.gradient_rgba_stops,
            &self.gradient_color_stops,
            &self.gradient_color_alpha_stops,
            self.gradient_offset_stops.clone(),
        );

        let grad_radians = self
            .gradient_radians
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

        let default_statuses = self.default_statuses();

        let statuses = if let Some(status) = custom_pal.statuses.as_ref() {
            let mut collapsed: HashMap<WidgetStatus, HashMap<StylePart, (PaletteKey, f32, Option<f32>)>> =
                HashMap::new();

            for ((widget_status, variant), parts) in status {
                let part_map: HashMap<StylePart, (PaletteKey, f32, Option<f32>)> = parts
                    .iter()
                    .map(|(style_part, palette_key, alpha, bd_width)| {
                        (style_part.clone(), (palette_key.clone(), *alpha, *bd_width))
                    })
                    .collect();

                // Button has no variant; NoVariant takes precedence over any selected StateVariant.
                if *variant == StateVariant::NoVariant || !collapsed.contains_key(widget_status) {
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
        } else {
            default_statuses
        };

        // Build gradient background once; used in every match arm when stops are supplied.
        let gradient_background: Option<iced::Background> = bkg_grad_color_stops.map(|stops| {
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
            let (bkg_key, alpha, _) = parts.get(&StylePart::Background).unwrap();
            let (text_key, text_alpha, _) = parts.get(&StylePart::Text).unwrap();
            let (bd_key, bd_alpha, bd_width) = parts.get(&StylePart::Border).unwrap();

            let background_color = bkg_key.pal_key_to_color(&theme_color, &cust_color).scale_alpha(*alpha);
            let text_color = text_key
                .pal_key_to_color(&theme_color, &cust_color)
                .scale_alpha(*text_alpha);
            let border_color = bd_key
                .pal_key_to_color(&theme_color, &cust_color)
                .scale_alpha(*bd_alpha);

            // The style border color overrides the palette border color
            let bd_color = if let Some(bc) = brd_color { bc } else { border_color };
            let bd_w = bd_width.unwrap_or(self.border_width.unwrap_or_default());

            (background_color, text_color, bd_color, *alpha, bd_w)
        };

        match status {
            button::Status::Active => {
                let (bkg_color, text_color, b_color, _, bd_width) = resolve_parts(WidgetStatus::Active);
                button::Style {
                    background: gradient_background.or(Some(iced::Background::Color(bkg_color))),
                    text_color,
                    border: iced::Border {
                        color: b_color,
                        width: bd_width,
                        radius,
                    },
                    shadow,
                    snap,
                }
            }
            button::Status::Pressed => {
                let (bkg_color, text_color, b_color, _, bd_width) = resolve_parts(WidgetStatus::Pressed);
                button::Style {
                    background: gradient_background.or(Some(iced::Background::Color(bkg_color))),
                    text_color,
                    border: iced::Border {
                        color: b_color,
                        width: bd_width,
                        radius,
                    },
                    shadow,
                    snap,
                }
            }
            button::Status::Hovered => {
                let (bkg_color, text_color, b_color, _, bd_width) = resolve_parts(WidgetStatus::Hovered);
                button::Style {
                    background: gradient_background.or(Some(iced::Background::Color(bkg_color))),
                    text_color,
                    border: iced::Border {
                        color: b_color,
                        width: bd_width,
                        radius,
                    },
                    shadow,
                    snap,
                }
            }
            button::Status::Disabled => {
                let (bkg_color, text_color, b_color, alpha, bd_width) = resolve_parts(WidgetStatus::Disabled);
                let background = gradient_background
                    .map(|g| g.scale_alpha(alpha))
                    .or(Some(iced::Background::Color(bkg_color)));
                button::Style {
                    background,
                    text_color,
                    border: iced::Border {
                        color: b_color,
                        width: bd_width,
                        radius,
                    },
                    shadow,
                    snap,
                }
            }
        }
    }

    pub fn default_statuses(&self) -> HashMap<WidgetStatus, HashMap<StylePart, (PaletteKey, f32, Option<f32>)>> {
        // HashMap definitions
        // StylePart: Bacground, Text, Border, ...
        // PaletteKey = Base, Basetext, Strong, StrongText, ...
        // f32: alpha, default = 1.0
        // Option<f32>: border width (Some overrides style width, None = 0.0)
        let mut default_statuses: HashMap<WidgetStatus, HashMap<StylePart, (PaletteKey, f32, Option<f32>)>> =
            HashMap::new();

        let mut inner = HashMap::new();
        inner.insert(StylePart::Background, (PaletteKey::Base, 1.0, None));
        inner.insert(StylePart::Text, (PaletteKey::BaseText, 1.0, None));
        inner.insert(StylePart::Border, (PaletteKey::Base, 1.0, None));
        default_statuses.insert(WidgetStatus::Active, inner);

        let mut inner = HashMap::new();
        inner.insert(StylePart::Background, (PaletteKey::Base, 1.0, None));
        inner.insert(StylePart::Text, (PaletteKey::BaseText, 1.0, None));
        inner.insert(StylePart::Border, (PaletteKey::Base, 1.0, None));
        default_statuses.insert(WidgetStatus::Pressed, inner);

        let mut inner = HashMap::new();
        inner.insert(StylePart::Background, (PaletteKey::Strong, 1.0, None));
        inner.insert(StylePart::Text, (PaletteKey::StrongText, 0.8, None));
        inner.insert(StylePart::Border, (PaletteKey::Strong, 1.0, None));
        default_statuses.insert(WidgetStatus::Hovered, inner);

        let mut inner = HashMap::new();
        inner.insert(StylePart::Background, (PaletteKey::Base, 0.5, None));
        inner.insert(StylePart::Text, (PaletteKey::BaseText, 0.8, None));
        inner.insert(StylePart::Border, (PaletteKey::Strong, 0.5, None));
        default_statuses.insert(WidgetStatus::Disabled, inner);

        default_statuses
    }

    // Returns statuses in the format custom_palette() expects:
    // [((WidgetStatus, StateVariant), [(StylePart, PaletteKey, alpha), ...]), ...]
    pub fn default_statuses_to_py_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let list = PyList::empty(py);
        // Sort so HashMap iteration order doesn't vary between runs.
        let mut statuses: Vec<_> = self.default_statuses().into_iter().collect();
        statuses.sort_by_key(|(status, _)| status.sort_index());
        for (widget_status, parts) in statuses {
            let mut parts_vec: Vec<_> = parts.into_iter().collect();
            parts_vec.sort_by_key(|(part, _)| part.sort_index());
            let parts_list = PyList::empty(py);
            for (style_part, (palette_key, alpha, border_width)) in parts_vec {
                let alpha_rounded = (alpha as f64 * 10000.0_f64).round() / 10000.0_f64;
                let bw = border_width.map(|w| w as f64);
                parts_list.append((style_part, palette_key, alpha_rounded, bw))?;
            }
            // Button has no variant.
            list.append(((widget_status, StateVariant::NoVariant), parts_list))?;
        }
        Ok(list)
    }

    // Serialize all button-style parameters into a Python dict keyed by field name.
    pub fn to_py_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let color_name = |c: &Option<Color>| c.as_ref().map(|c| format!("{c:?}"));
        let colors_names = |v: &Option<Vec<Color>>| {
            v.as_ref()
                .map(|v| v.iter().map(|c| format!("{c:?}")).collect::<Vec<_>>())
        };

        let dict = PyDict::new(py);
        dict.set_item("id", self.id)?;

        dict.set_item("gradient_color_stops", colors_names(&self.gradient_color_stops))?;
        dict.set_item("gradient_color_alpha_stops", self.gradient_color_alpha_stops.clone())?;
        dict.set_item(
            "gradient_rgba_stops",
            self.gradient_rgba_stops
                .as_ref()
                .map(|v| v.iter().map(|c| c.to_vec()).collect::<Vec<_>>()),
        )?;
        dict.set_item("gradient_offset_stops", self.gradient_offset_stops.clone())?;
        dict.set_item("gradient_degrees", self.gradient_degrees)?;
        dict.set_item("gradient_radians", self.gradient_radians)?;

        dict.set_item("border_radius", self.border_radius.clone())?;
        dict.set_item("border_rounded", self.border_rounded)?;

        dict.set_item("shadow_color", color_name(&self.shadow_color))?;
        dict.set_item("shadow_color_alpha", self.shadow_color_alpha)?;
        dict.set_item("shadow_rgba", self.shadow_rgba.map(|c| c.to_vec()))?;
        dict.set_item("shadow_offset_xy", self.shadow_offset_xy.map(|c| c.to_vec()))?;
        dict.set_item("shadow_blur_radius", self.shadow_blur_radius)?;

        dict.set_item("snap", self.snap)?;
        Ok(dict)
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuButtonStatus {
    Active,
    Hovered,
    Pressed,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuButtonStyleStd {
    Background,
    Danger,
    Primary,
    Secondary,
    Subtle,
    Success,
    Warning,
    Text,
}

impl MenuButtonStyleStd {
    pub fn to_iced(&self, theme: &Theme, status: button::Status) -> button::Style {
        match self {
            MenuButtonStyleStd::Background => button::background(theme, status),
            MenuButtonStyleStd::Danger => button::danger(theme, status),
            MenuButtonStyleStd::Primary => button::primary(theme, status),
            MenuButtonStyleStd::Secondary => button::secondary(theme, status),
            MenuButtonStyleStd::Subtle => button::subtle(theme, status),
            MenuButtonStyleStd::Success => button::success(theme, status),
            MenuButtonStyleStd::Warning => button::warning(theme, status),
            MenuButtonStyleStd::Text => button::text(theme, status),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuButtonParam {
    Clip,
    Disabled,
    Fill,
    Height,
    HeightFill,
    Padding,
    PaletteId,
    Show,
    StyleId,
    StyleStd,
    Width,
    WidthFill,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum MenuButtonStyleParam {
    GradientColorStops,
    GradientColorAlphaStops,
    GradientRgbaStops,
    GradientDegrees,
    GradientRadians,

    BorderRadius,
    BorderRounded,

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

impl WidgetParamUpdate for MenuButton {
    type Param = MenuButtonParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            MenuButtonParam::Clip => set_t_value(&mut self.clip, value, "MenuButtonParam::Clip"),
            MenuButtonParam::Disabled => set_t_value(&mut self.disabled, value, "MenuButtonParam::Disabled"),
            MenuButtonParam::Fill => set_t_value(&mut self.fill, value, "MenuButtonParam::Fill"),
            MenuButtonParam::Height => set_t_value(&mut self.height, value, "MenuButtonParam::Height"),
            MenuButtonParam::HeightFill => set_t_value(&mut self.height, value, "MenuButtonParam::HeightFill"),
            MenuButtonParam::Padding => set_t_value(&mut self.padding, value, "MenuButtonParam::Padding"),
            MenuButtonParam::PaletteId => set_t_value(&mut self.palette_id, value, "MenuButtonParam::PaletteId"),
            MenuButtonParam::Show => set_t_value(&mut self.show, value, "Show"),
            MenuButtonParam::StyleId => set_t_value(&mut self.style_id, value, "MenuButtonParam::StyleId"),
            MenuButtonParam::StyleStd => set_t_value(&mut self.style_std, value, "MenuButtonParam::StyleStd"),
            MenuButtonParam::Width => set_t_value(&mut self.width, value, "MenuButtonParam::Width"),
            MenuButtonParam::WidthFill => set_t_value(&mut self.width, value, "MenuButtonParam::WidthFill"),
        }
    }
}

impl WidgetParamUpdate for MenuButtonStyle {
    type Param = MenuButtonStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            
            MenuButtonStyleParam::GradientColorStops => set_t_value(
                &mut self.gradient_color_stops,
                value,
                "MenuButtonStyleParam::GradientColorStops",
            ),
            MenuButtonStyleParam::GradientColorAlphaStops => set_t_value(
                &mut self.gradient_color_alpha_stops,
                value,
                "MenuButtonStyleParam::GradientColorAlphaStops",
            ),
            MenuButtonStyleParam::GradientRgbaStops => set_t_value(
                &mut self.gradient_rgba_stops,
                value,
                "MenuButtonStyleParam::GradientRgbaStops",
            ),
            MenuButtonStyleParam::GradientDegrees => {
                set_t_value(&mut self.gradient_degrees, value, "MenuButtonStyleParam::GradientDegrees")
            },
            MenuButtonStyleParam::GradientRadians => {
                set_t_value(&mut self.gradient_radians, value, "MenuButtonStyleParam::GradientRadians")
            },
            MenuButtonStyleParam::BorderRadius => {
                set_t_value(&mut self.border_radius, value, "MenuButtonStyleParam::BorderRadius")
            },
            MenuButtonStyleParam::BorderRounded => {
                set_t_value(&mut self.border_rounded, value, "MenuButtonStyleParam::BorderRounded")
            },
            MenuButtonStyleParam::ShadowColor => {
                set_t_value(&mut self.shadow_color, value, "MenuButtonStyleParam::ShadowColor")
            },
            MenuButtonStyleParam::ShadowColorAlpha => set_t_value(
                &mut self.shadow_color_alpha,
                value,
                "MenuButtonStyleParam::ShadowColorAlpha",
            ),
            MenuButtonStyleParam::ShadowRgba => set_t_value(&mut self.shadow_rgba, value, "MenuButtonStyleParam::ShadowRgba"),
            MenuButtonStyleParam::ShadowOffsetXy => {
                set_t_value(&mut self.shadow_offset_xy, value, "MenuButtonStyleParam::ShadowOffsetXy")
            },
            MenuButtonStyleParam::ShadowBlurRadius => set_t_value(
                &mut self.shadow_blur_radius,
                value,
                "MenuButtonStyleParam::ShadowBlurRadius",
            ),
            MenuButtonStyleParam::Snap => set_t_value(&mut self.snap, value, "MenuButtonStyleParam::Snap"),
        }
    }
}
