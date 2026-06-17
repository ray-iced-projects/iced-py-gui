//! ipg_combo_box
use std::collections::HashMap;

use crate::app;
use crate::IpgState;
use crate::graphics::colors::Color;
use crate::graphics::colors::background;
use crate::py_api::helpers::get_radius;
use crate::state::Widgets;
use crate::widgets::widget_param_update::extract_param;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};
use super::callbacks::invoke_callback_with_args;
use crate::py_api::helpers::{get_len, get_padding};

use iced::Border;
use iced::Shadow;
use iced::Vector;
use iced::overlay::menu;
use iced::widget::combo_box;
use iced::widget::text::Ellipsis;
use iced::Element;
use iced::widget;

use iced::widget::text_input;
use pyo3::pyclass;
use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct ComboBox {
    pub id: usize,
    pub cb_state: combo_box::State<String>,
    pub placeholder: Option<String>,
    pub selected: Option<String>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub menu_height: Option<f32>,
    pub menu_height_fill: Option<bool>,
    pub padding: Option<Vec<f32>>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub text_ellipsis: Ellipsis,
    pub font_id: Option<usize>,
    pub menu_style_id: Option<usize>,
    pub input_style_id: Option<usize>,
    pub show: bool,
}

#[derive(Debug, Clone)]
pub enum CBMessage {
    OnSelect(String),
    OnOpen,
    OnClose,
    OnInput,
}

impl ComboBox {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self,
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, app::Message>> {
        
        if !self.show {
            return None
        }

        let font_opt = 
            self.lookup(widgets, self.font_id)
                .and_then(Widgets::as_font).cloned();

        let style_opt = 
            self.lookup(widgets, self.menu_style_id)
            .and_then(Widgets::as_combobox_menu_style).cloned();

        let placeholder = self.placeholder.as_deref().unwrap_or_default();

        let cb = widget::combo_box(
            &self.cb_state,
            placeholder,
            self.selected.as_ref(),
            CBMessage::OnSelect,
        );

        let cb = cb
            .width(get_len(None, self.width_fill, self.width))
            .menu_height(get_len(None, self.menu_height_fill, self.menu_height))
            .padding(get_padding(&self.padding))
            .ellipsis(self.text_ellipsis)
            .on_open(CBMessage::OnOpen)
            .on_close(CBMessage::OnClose)
            .menu_style(move |theme| {
                if let Some(st) = &style_opt {
                    st.to_iced(theme)
                } else { menu::default(theme) }
            });

        let cb = if let Some(ts) = self.text_size {
            cb.size(ts)
        } else { cb };

        let cb = if let Some(lh) = self.text_line_height {
            cb.line_height(lh)
        } else { cb };

        let cb = if let Some(f) = font_opt {
            cb.font(f.to_iced())
        } else { cb };

        let cb: Element<'_, CBMessage> = cb.into();
        Some(cb.map(move |message| app::Message::ComboBox(self.id, message)))

    }
 }


 pub fn combo_box_callback(state: &mut IpgState, id: usize, message: CBMessage) {
    match message {
        CBMessage::OnSelect(selected) => {
            // Update widget state directly
            if let Some(Widgets::ComboBox(cb)) = state.widgets.get_mut(&id) {
                cb.selected = Some(selected.clone());
            }
            invoke_callback_with_args(id, "on_select", "ComboBox", selected,
                "def cb(wid: int, selected: str)");
        },
        CBMessage::OnOpen => {
            invoke_callback_with_args(id, "on_open", "ComboBox", (),
                "def cb(wid: int)");
        },
        CBMessage::OnClose => {
            invoke_callback_with_args(id, "on_close", "ComboBox", (),
                "def cb(wid: int)");
        },
        CBMessage::OnInput => {
            invoke_callback_with_args(id, "on_input", "ComboBox", (),
                "def cb(wid: int)");
        },
    }
 }

#[derive(Debug, Clone)]
 pub struct ComboBoxMenuStyle {
    pub id: usize,

    pub palette_base_color: Option<Color>,
    pub palette_base_alpha: Option<f32>,
    pub palette_base_rgba: Option<[f32; 4]>,

    pub selected_text_color: Option<Color>,
    pub selected_text_alpha: Option<f32>,
    pub selected_text_rgba: Option<[f32; 4]>,

    pub selected_bkg_color: Option<Color>,
    pub selected_bkg_alpha: Option<f32>,
    pub selected_bkg_rgba: Option<[f32; 4]>,
    
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,

    pub shadow_color: Option<Color>,
    pub shadow_color_alpha: Option<f32>,
    pub shadow_rgba: Option<[f32; 4]>,
    pub shadow_offset_xy: Option<[f32; 2]>,
    pub shadow_blur_radius: Option<f32>,
}

impl ComboBoxMenuStyle {
    pub fn to_iced(
        &self,
        theme: &iced::Theme,
    ) -> menu::Style {

        let shd_color =
            Color::rgba_ipg_color_to_iced(self.shadow_rgba, &self.shadow_color, self.shadow_color_alpha);

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
            .map(|rd| get_radius(rd, "combo_box".to_string()))
            .unwrap_or(0.0.into());

        let border_width = if let Some(w) = self.border_width {
            w
        } else { 1.0 };

        let palette = theme.palette();

        let palette_base_opt = 
            Color::rgba_ipg_color_to_iced(self.palette_base_rgba, &self.palette_base_color, self.palette_base_alpha);
        
        let bkg = if let Some(color) = palette_base_opt {
            background(color)
        } else { palette.background };
        
        let selected_txt_color_opt = 
            Color::rgba_ipg_color_to_iced(self.selected_text_rgba, &self.selected_text_color, self.selected_text_alpha);

        let selected_text_color = if let Some(t_color) = selected_txt_color_opt {
            t_color
        } else { palette.primary.strong.text };

        let selected_bkg_color_opt = 
            Color::rgba_ipg_color_to_iced(self.selected_bkg_rgba, &self.selected_bkg_color, self.selected_bkg_alpha);

        let selected_background = if let Some(bkg) = selected_bkg_color_opt {
            bkg.into()
        } else { palette.primary.strong.color.into() };

        menu::Style {
            background: bkg.weak.color.into(),
            border: Border {
                width: border_width,
                radius,
                color: bkg.strong.color,
            },
            text_color: bkg.weak.text,
            selected_text_color,
            selected_background,
            shadow,
        }

    }
            
}

#[derive(Debug, Clone)]
pub struct ComboBoxInputStyle {
    pub id: usize,

    pub palette_base_color: Option<Color>,
    pub palette_base_alpha: Option<f32>,
    pub palette_base_rgba: Option<[f32; 4]>,

    pub selected_text_color: Option<Color>,
    pub selected_text_alpha: Option<f32>,
    pub selected_text_rgba: Option<[f32; 4]>,

    pub border_color: Option<Color>,
    pub border_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,

    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
}

impl ComboBoxInputStyle {
    pub fn to_iced(
        &self,
        theme: &iced::Theme,
        status: text_input::Status,
    ) -> text_input::Style {

        let radius = self.border_radius.as_ref()
            .map(|rd| get_radius(rd, "combo_box".to_string()))
            .unwrap_or(2.0.into());

        let border_width = if let Some(w) = self.border_width {
            w
        } else { 1.0 };

        let palette = theme.palette();

        let palette_base_opt = 
            Color::rgba_ipg_color_to_iced(self.palette_base_rgba, &self.palette_base_color, self.palette_base_alpha);
        
        let bkg = if let Some(color) = palette_base_opt {
            background(color)
        } else { palette.background };
        
        let selected_txt_color_opt = 
            Color::rgba_ipg_color_to_iced(self.selected_text_rgba, &self.selected_text_color, self.selected_text_alpha);

        let selected_text_color = if let Some(t_color) = selected_txt_color_opt {
            t_color
        } else { palette.primary.weak.color };

        let border_color = 
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_alpha);

        let border = if let Some(bc) = border_color {
            bc
        } else { palette.primary.strong.color };

        let active = text_input::Style {
            background: iced::Background::Color(bkg.base.color),
            border: Border {
                radius,
                width: border_width,
                color: bkg.strong.color,
            },
            icon: bkg.weak.text,
            placeholder: bkg.weak.text,
            value: bkg.base.text,
            selection: selected_text_color,
        };

        let style = match status {
            text_input::Status::Active => active,
            text_input::Status::Hovered => text_input::Style {
                border: Border {
                    color: bkg.base.text,
                    ..active.border
                },
                ..active
            },
            text_input::Status::Focused { .. } => text_input::Style {
                border: Border {
                    color: border,
                    ..active.border
                },
                ..active
            },
            text_input::Status::Disabled => text_input::Style {
                background: iced::Background::Color(bkg.weak.color),
                value: active.placeholder,
                placeholder: bkg.strongest.color,
                ..active
            },
        };

        style

    }

}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ComboBoxParam {
    FontId,
    MenuHeight,
    Options,
    Padding,
    Placeholder,
    Selected,
    Show,
    StyleId,
    TextLineHeight,
    TextSize,
    Width,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ComboBoxMenuStyleParam {
    PaletteBaseColor,
    PaletteBaseAlpha,
    PaletteBaseRgba,

    SelectedTextColor,
    SelectedTextAlpha,
    SelectedTextRgba,

    SelectedBkgColor,
    SelectedBkgAlpha,
    SelectedBkgRgba,
    
    BorderRadius,
    BorderWidth,

    ShadowColor,
    ShadowColorAlpha,
    ShadowRgba,
    ShadowOffsetXY,
    ShadowBlurRadius,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ComboBoxInputStyleParam {
    PaletteBaseColor,
    PaletteBaseAlpha,
    PaletteBaseRgba,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for ComboBox {
    type Param = ComboBoxParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ComboBoxParam::FontId => set_t_value(&mut self.font_id, value, "ComboBoxParam::FontId"),
            ComboBoxParam::MenuHeight => set_t_value(&mut self.menu_height, value, "ComboBoxParam::MenuHeight"),
            ComboBoxParam::Options => {
                let options: Vec<String> = extract_param(value);
                self.cb_state = combo_box::State::new(options.clone());
            },
            ComboBoxParam::Padding => set_t_value(&mut self.padding, value, "ComboBoxParam::Padding"),
            ComboBoxParam::Placeholder => set_t_value(&mut self.placeholder, value, "ComboBoxParam::Placeholder"),
            ComboBoxParam::Selected => set_t_value(&mut self.selected, value, "ComboBoxParam::Selected"),
            ComboBoxParam::Show => set_t_value(&mut self.show, value, "ComboBoxParam::Show"),
            ComboBoxParam::StyleId => set_t_value(&mut self.menu_style_id, value, "ComboBoxParam::StyleId"),
            ComboBoxParam::TextLineHeight => set_t_value(&mut self.text_line_height,value, "ComboBoxParam::TextLineHeight"),
            ComboBoxParam::TextSize => set_t_value(&mut self.text_size, value, "ComboBoxParam::TextSize"),
            ComboBoxParam::Width => set_t_value(&mut self.width, value, "ComboBoxParam::Width"),
        }
    }
}


impl WidgetParamUpdate for ComboBoxMenuStyle {
    type Param = ComboBoxMenuStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ComboBoxMenuStyleParam::PaletteBaseColor => set_t_value(&mut self.palette_base_color, value, "ComboBoxStyleParam::PaletteBaseColor"),
            ComboBoxMenuStyleParam::PaletteBaseAlpha => set_t_value(&mut self.palette_base_alpha, value, "ComboBoxStyleParam::PaletteBaseAlpha"),
            ComboBoxMenuStyleParam::PaletteBaseRgba => set_t_value(&mut self.palette_base_rgba, value, "ComboBoxStyleParam::PaletteBaseRgba"),
            ComboBoxMenuStyleParam::SelectedTextColor => set_t_value(&mut self.selected_text_color, value, "ComboBoxStyleParam::SelectedTextColor"),
            ComboBoxMenuStyleParam::SelectedTextAlpha => set_t_value(&mut self.selected_text_alpha, value, "ComboBoxStyleParam::SelectedTextAlpha"),
            ComboBoxMenuStyleParam::SelectedTextRgba => set_t_value(&mut self.selected_text_rgba, value, "ComboBoxStyleParam::SelectedTextRgba"),
            ComboBoxMenuStyleParam::SelectedBkgColor => set_t_value(&mut self.selected_bkg_color, value, "ComboBoxStyleParam::SelectedBkgColor"),
            ComboBoxMenuStyleParam::SelectedBkgAlpha => set_t_value(&mut self.selected_bkg_alpha, value, "ComboBoxStyleParam::SelectedBkgAlpha"),
            ComboBoxMenuStyleParam::SelectedBkgRgba => set_t_value(&mut self.selected_bkg_rgba, value, "ComboBoxStyleParam::SelectedBkgRgba"),
            ComboBoxMenuStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "ComboBoxStyleParam::BorderRadius"),
            ComboBoxMenuStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "ComboBoxStyleParam::BorderWidth"),
            ComboBoxMenuStyleParam::ShadowColor => set_t_value(&mut self.shadow_color, value, "ComboBoxStyleParam::ShadowColor"),
            ComboBoxMenuStyleParam::ShadowColorAlpha => set_t_value(&mut self.shadow_color_alpha, value, "ComboBoxStyleParam::ShadowColorAlpha"),
            ComboBoxMenuStyleParam::ShadowRgba => set_t_value(&mut self.shadow_rgba, value, "ComboBoxStyleParam::ShadowRgba"),
            ComboBoxMenuStyleParam::ShadowOffsetXY => set_t_value(&mut self.shadow_offset_xy, value, "ComboBoxStyleParam::ShadowOffsetXY"),
            ComboBoxMenuStyleParam::ShadowBlurRadius => set_t_value(&mut self.shadow_blur_radius, value, "ComboBoxStyleParam::ShadowBlurRadius"),
        }
    }
}

impl WidgetParamUpdate for ComboBoxInputStyle {
    type Param = ComboBoxInputStyleParam;

    fn param_update(&mut self, param: Self::Param, _value: &PyObject) {
        match param {
            ComboBoxInputStyleParam::PaletteBaseColor => todo!(),
            ComboBoxInputStyleParam::PaletteBaseAlpha => todo!(),
            ComboBoxInputStyleParam::PaletteBaseRgba => todo!(),
        }
    }
}
