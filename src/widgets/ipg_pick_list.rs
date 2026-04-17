//! ipg_pick_list
use std::collections::HashMap;

use crate::app;
use crate::IpgState;
use crate::graphics::bootstrap_arrow::Arrow;
use crate::graphics::colors::Color;
use crate::py_api::helpers::get_len;
use crate::py_api::helpers::{get_padding, get_radius};
use crate::state::Widgets;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};
use super::callbacks::invoke_callback_with_args;

use iced::widget::pick_list::{self, Status};
use iced::{Font, Pixels, Theme};
use iced::{Element};
use iced::widget;
use iced::widget::pick_list::{Handle, Icon};
use iced::overlay::menu;

use pyo3::pyclass;
use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct PickList {
    pub id: usize,
    pub options: Vec<String>,
    pub placeholder: Option<String>,
    pub selected: Option<String>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub menu_height: Option<f32>,
    pub menu_height_fill: Option<bool>,
    pub padding: Option<Vec<f32>>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub handle: Option<PickListHandle>,
    pub arrow_size: Option<f32>,
    pub dynamic_close: Option<Arrow>,
    pub dynamic_open: Option<Arrow>,
    pub custom_static: Option<Arrow>,
    pub style_id: Option<usize>,
    pub show: bool,
}


#[derive(Debug, Clone)]
pub enum PLMessage {
    OnSelect(String),
}

impl PickList {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, Widgets>, id: Option<usize>) -> Option<&'a Widgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &'a self, 
        widgets: &HashMap<usize, Widgets>,
    ) -> Option<Element<'a, app::Message>> {
        
        if!self.show {
            return None
        }

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_pick_list_style).cloned();

        let handle = if let Some(hd) = &self.handle {
            get_handle(
                &hd, 
                self.arrow_size, 
                &self.dynamic_close,
                &self.dynamic_open,
                &self.custom_static)
        } else { Handle::None };

        let pl = 
            widget::PickList::new(self.options.clone(), 
                self.selected.clone(), 
                PLMessage::OnSelect,
            )
            .placeholder(self.placeholder.clone().unwrap_or("".to_string()))
            .width(get_len(None, self.width_fill, self.width))
            .menu_height(get_len(None, self.menu_height_fill, self.menu_height))
            .padding(get_padding(&self.padding))
            .handle(handle)
            .style(move|theme: &Theme, status| {   
                if let Some(st) = &style_opt {
                        st.to_iced(theme, status)
                    } else {
                       pick_list::default(theme, status)
                    }
                });

        let style_opt2 = 
            self.lookup(widgets, self.style_id)
                .and_then(Widgets::as_pick_list_style).cloned();

        let pl = if let Some(st) = style_opt2.filter(|s| s.has_menu_style()) {
            pl.menu_style(move |theme: &Theme| st.to_menu_style(theme))
        } else { pl };

        let pl = if let Some(ts) = self.text_size {
            pl.text_size(ts)
        } else { pl };

        let pl = if let Some(lh) = self.text_line_height {
            pl.text_line_height(lh)
        } else { pl };

        let pl: Element<'_, PLMessage> = pl.into();
        Some(pl.map(move |message| app::Message::PickList(self.id, message)))

    }
 }


 pub fn pick_list_callback(state: &mut IpgState, id: usize, message: PLMessage) {
    match message {
        PLMessage::OnSelect(selected) => {
            // Update widget state directly
            if let Some(Widgets::PickList(pl)) = state.widgets.get_mut(&id) {
                pl.selected = Some(selected.clone());
            }
            invoke_callback_with_args(id, "on_select", "PickList", selected,
                "def cb(wid: int, selected: str)");
        },
    }
 }


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum PickListHandle {
    Default,
    Arrow,
    Dynamic,
    None,
    Static,
}


fn get_handle(ipg_handle: &PickListHandle, 
                arrow_size: Option<f32>,
                closed: &Option<Arrow>,
                opened: &Option<Arrow>,
                custom: &Option<Arrow>,
            ) -> Handle<Font> 
{
    match ipg_handle {
        PickListHandle::Default => Handle::default(),
        PickListHandle::Arrow => {
            match arrow_size {
                Some(ars) => Handle::Arrow { size: Some(Pixels(ars)) },
                None => Handle::Arrow { size: None },
            }
        },
        PickListHandle::Dynamic => {
            let arrow_closed = match closed {
                Some(cls) => Arrow::to_char(cls),
                None => Arrow::to_char(&Arrow::ArrowBarRight),
            };

            let arrow_opened = match opened {
                Some(op) => Arrow::to_char(op),
                None => Arrow::to_char(&Arrow::ArrowBarRight),
            };

            let size = arrow_size.map(Pixels);

            Handle::Dynamic { 
                closed: Icon { 
                    code_point: arrow_closed, 
                    font: iced::Font::with_name("bootstrap-icons"), 
                    size, 
                    line_height: Default::default(), 
                    shaping: Default::default()
                }, 
                open: Icon {
                    code_point: arrow_opened,
                    font: iced::Font::with_name("bootstrap-icons"), 
                    size, 
                    line_height: Default::default(), 
                    shaping: Default::default()} 
                }
        },
        PickListHandle::None => Handle::None,
        PickListHandle::Static => {
                let custom_type = match custom {
                    Some(cust) => Arrow::to_char(cust),
                    None => Arrow::to_char(&Arrow::ArrowBarRight),
                };

                let size = arrow_size.map(Pixels);

                Handle::Static(Icon { 
                    code_point: custom_type, 
                    font: iced::Font::with_name("bootstrap-icons"), 
                    size, 
                    line_height: Default::default(), 
                    shaping: Default::default()
                }
            )
        },
    }
}

#[derive(Debug, Clone, Default)]
pub struct PickListStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_alpha: Option<f32>,
    pub background_rgba: Option<[f32; 4]>,
    pub text_color: Option<Color>,
    pub text_color_alpha: Option<f32>,
    pub text_rgba: Option<[f32; 4]>,
    pub handle_color: Option<Color>,
    pub handle_color_alpha: Option<f32>,
    pub handle_rgba: Option<[f32; 4]>,
    pub placeholder_color: Option<Color>,
    pub placeholder_color_alpha: Option<f32>,
    pub placeholder_rgba: Option<[f32; 4]>,
    pub border_color: Option<Color>,
    pub border_color_alpha: Option<f32>,
    pub border_rgba: Option<[f32; 4]>,
    pub border_color_hovered: Option<Color>,
    pub border_color_hovered_alpha: Option<f32>,
    pub border_rgba_hovered: Option<[f32; 4]>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub menu_background_color: Option<Color>,
    pub menu_background_color_alpha: Option<f32>,
    pub menu_background_rgba: Option<[f32; 4]>,
    pub menu_text_color: Option<Color>,
    pub menu_text_color_alpha: Option<f32>,
    pub menu_text_rgba: Option<[f32; 4]>,
    pub menu_selected_text_color: Option<Color>,
    pub menu_selected_text_color_alpha: Option<f32>,
    pub menu_selected_text_rgba: Option<[f32; 4]>,
    pub menu_selected_background_color: Option<Color>,
    pub menu_selected_background_color_alpha: Option<f32>,
    pub menu_selected_background_rgba: Option<[f32; 4]>,
    pub menu_border_color: Option<Color>,
    pub menu_border_color_alpha: Option<f32>,
    pub menu_border_rgba: Option<[f32; 4]>,
    pub menu_border_radius: Option<Vec<f32>>,
    pub menu_border_width: Option<f32>,
}

impl PickListStyle {
    pub fn to_iced(
        &self, 
        theme: &Theme, 
        status: Status, 
        ) -> pick_list::Style {
        
        let mut active_style = pick_list::default(theme, Status::Active);

        let background_color = 
            Color::rgba_ipg_color_to_iced(self.background_rgba, &self.background_color, self.background_color_alpha);
        let border_color = 
            Color::rgba_ipg_color_to_iced(self.border_rgba, &self.border_color, self.border_color_alpha);
        let border_color_hovered = 
            Color::rgba_ipg_color_to_iced(self.border_rgba_hovered, &self.border_color_hovered, self.border_color_hovered_alpha);
        let handle_color = 
            Color::rgba_ipg_color_to_iced(self.handle_rgba, &self.handle_color, self.handle_color_alpha);
        let placeholder_color = 
            Color::rgba_ipg_color_to_iced(self.placeholder_rgba, &self.placeholder_color, self.placeholder_color_alpha);
        let text_color = 
            Color::rgba_ipg_color_to_iced(self.text_rgba, &self.text_color, self.text_color_alpha);

        if let Some(bc) = background_color {
            active_style.background = bc.into();
        }
        
        if let Some(hc) = handle_color {
            active_style.handle_color = hc;
        }

        if let Some(pc) = placeholder_color {
            active_style.placeholder_color = pc;
        }

        if let Some(tc) = text_color {
            active_style.text_color = tc;
        }

        if let Some(br) = &self.border_radius {
        active_style.border.radius = 
            get_radius(&br, "PickList".to_string());
        }

        if let Some(bw) = self.border_width {
            active_style.border.width = bw;
        };

        if let Some(bc) = border_color && status == Status::Active {
            active_style.border.color = bc;
        }

        let mut hover_opened_style = active_style;
        
        if let Some(bch) = border_color_hovered {
            hover_opened_style.border.color = bch;
        }
        
        match status {
            Status::Active => active_style,
            Status::Hovered | Status::Opened { .. } => hover_opened_style,
        }

    }

    pub fn to_menu_style(&self, theme: &Theme) -> menu::Style {
        let mut style = menu::default(theme);

        let bg = Color::rgba_ipg_color_to_iced(
            self.menu_background_rgba, &self.menu_background_color, self.menu_background_color_alpha);
        let tc = Color::rgba_ipg_color_to_iced(
            self.menu_text_rgba, &self.menu_text_color, self.menu_text_color_alpha);
        let stc = Color::rgba_ipg_color_to_iced(
            self.menu_selected_text_rgba, &self.menu_selected_text_color, self.menu_selected_text_color_alpha);
        let sbg = Color::rgba_ipg_color_to_iced(
            self.menu_selected_background_rgba, &self.menu_selected_background_color, self.menu_selected_background_color_alpha);
        let bc = Color::rgba_ipg_color_to_iced(
            self.menu_border_rgba, &self.menu_border_color, self.menu_border_color_alpha);

        if let Some(c) = bg {
            style.background = c.into();
        }
        if let Some(c) = tc {
            style.text_color = c;
        }
        if let Some(c) = stc {
            style.selected_text_color = c;
        }
        if let Some(c) = sbg {
            style.selected_background = c.into();
        }
        if let Some(c) = bc {
            style.border.color = c;
        }
        if let Some(br) = &self.menu_border_radius {
            style.border.radius = get_radius(br, "PickListMenu".to_string());
        }
        if let Some(bw) = self.menu_border_width {
            style.border.width = bw;
        }

        style
    }

    pub fn has_menu_style(&self) -> bool {
        self.menu_background_color.is_some()
            || self.menu_background_rgba.is_some()
            || self.menu_text_color.is_some()
            || self.menu_text_rgba.is_some()
            || self.menu_selected_text_color.is_some()
            || self.menu_selected_text_rgba.is_some()
            || self.menu_selected_background_color.is_some()
            || self.menu_selected_background_rgba.is_some()
            || self.menu_border_color.is_some()
            || self.menu_border_rgba.is_some()
            || self.menu_border_radius.is_some()
            || self.menu_border_width.is_some()
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum PickListParam {
    ArrowSize,
    CustomStatic,
    DynamicClose,
    DynamicOpen,
    Handle,
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
pub enum PickListStyleParam {
    BackgroundColor,
    BackgroundColorAlpha,
    BackgroundRgba,
    TextColor,
    TextColorAlpha,
    TextRgba,
    HandleColor,
    HandleColorAlpha,
    HandleRgba,
    PlaceholderColor,
    PlaceholderColorAlpha,
    PlaceholderRgba,
    BorderColor,
    BorderColorAlpha,
    BorderRgba,
    BorderColorHovered,
    BorderColorHoveredAlpha,
    BorderRgbaHovered,
    BorderRadius,
    BorderWidth,
    MenuBackgroundColor,
    MenuBackgroundColorAlpha,
    MenuBackgroundRgba,
    MenuTextColor,
    MenuTextColorAlpha,
    MenuTextRgba,
    MenuSelectedTextColor,
    MenuSelectedTextColorAlpha,
    MenuSelectedTextRgba,
    MenuSelectedBackgroundColor,
    MenuSelectedBackgroundColorAlpha,
    MenuSelectedBackgroundRgba,
    MenuBorderColor,
    MenuBorderColorAlpha,
    MenuBorderRgba,
    MenuBorderRadius,
    MenuBorderWidth,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for PickList {
    type Param = PickListParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            PickListParam::ArrowSize => set_t_value(&mut self.arrow_size, value, "PickListParam::ArrowSize"),
            PickListParam::CustomStatic => set_t_value(&mut self.custom_static, value, "PickListParam::CustomStatic"),
            PickListParam::DynamicClose => set_t_value(&mut self.dynamic_close, value, "PickListParam::DynamicClose"),
            PickListParam::DynamicOpen => set_t_value(&mut self.dynamic_open, value, "PickListParam::DynamicOpen"),
            PickListParam::Handle => set_t_value(&mut self.handle, value, "PickListParam::Handle"),
            PickListParam::MenuHeight => set_t_value(&mut self.menu_height, value, "PickListParam::MenuHeight"),
            PickListParam::Options => set_t_value(&mut self.options, value, "PickListParam::Options"),
            PickListParam::Padding => set_t_value(&mut self.padding, value, "PickListParam::Padding"),
            PickListParam::Placeholder => set_t_value(&mut self.placeholder, value, "PickListParam::Placeholder"),
            PickListParam::Selected => set_t_value(&mut self.selected, value, "PickListParam::Selected"),
            PickListParam::Show => set_t_value(&mut self.show, value, "PickListParam::Show"),
            PickListParam::StyleId => set_t_value(&mut self.style_id, value, "PickListParam::StyleId"),
            PickListParam::TextLineHeight => set_t_value(&mut self.text_line_height,value, "PickListParam::TextLineHeight"),
            PickListParam::TextSize => set_t_value(&mut self.text_size, value, "PickListParam::TextSize"),
            PickListParam::Width => set_t_value(&mut self.width, value, "PickListParam::Width"),
        }
    }
}

impl WidgetParamUpdate for PickListStyle {
    type Param = PickListStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            PickListStyleParam::BackgroundColor => set_t_value(&mut self.background_color, value, "PickListStyleParam::BackgroundColor"),
            PickListStyleParam::BackgroundColorAlpha => set_t_value(&mut self.background_color_alpha, value, "PickListStyleParam::BackgroundColorAlpha"),
            PickListStyleParam::BackgroundRgba => set_t_value(&mut self.background_rgba, value, "PickListStyleParam::BackgroundRgba"),
            PickListStyleParam::TextColor => set_t_value(&mut self.text_color, value, "PickListStyleParam::TextColor"),
            PickListStyleParam::TextColorAlpha => set_t_value(&mut self.text_color_alpha, value, "PickListStyleParam::TextColorAlpha"),
            PickListStyleParam::TextRgba => set_t_value(&mut self.text_rgba, value, "PickListStyleParam::TextRgba"),
            PickListStyleParam::HandleColor => set_t_value(&mut self.handle_color, value, "PickListStyleParam::HandleColor"),
            PickListStyleParam::HandleColorAlpha => set_t_value(&mut self.handle_color_alpha, value, "PickListStyleParam::HandleColorAlpha"),
            PickListStyleParam::HandleRgba => set_t_value(&mut self.handle_rgba, value, "PickListStyleParam::HandleRgba"),
            PickListStyleParam::PlaceholderColor => set_t_value(&mut self.placeholder_color, value, "PickListStyleParam::PlaceholderColor"),
            PickListStyleParam::PlaceholderColorAlpha => set_t_value(&mut self.placeholder_color_alpha, value, "PickListStyleParam::PlaceholderColorAlpha"),
            PickListStyleParam::PlaceholderRgba => set_t_value(&mut self.placeholder_rgba, value, "PickListStyleParam::PlaceholderRgba"),
            PickListStyleParam::BorderColor => set_t_value(&mut self.border_color, value, "PickListStyleParam::BorderColor"),
            PickListStyleParam::BorderColorAlpha => set_t_value(&mut self.border_color_alpha, value, "PickListStyleParam::BorderColorAlpha"),
            PickListStyleParam::BorderRgba => set_t_value(&mut self.border_rgba, value, "PickListStyleParam::BorderRgba"),
            PickListStyleParam::BorderColorHovered => set_t_value(&mut self.border_color_hovered, value, "PickListStyleParam::BorderColorHovered"),
            PickListStyleParam::BorderColorHoveredAlpha => set_t_value(&mut self.border_color_hovered_alpha, value, "PickListStyleParam::BorderColorHoveredAlpha"),
            PickListStyleParam::BorderRgbaHovered => set_t_value(&mut self.border_rgba_hovered, value, "PickListStyleParam::BorderRgbaHovered"),
            PickListStyleParam::BorderRadius => set_t_value(&mut self.border_radius, value, "PickListStyleParam::BorderRadius"),
            PickListStyleParam::BorderWidth => set_t_value(&mut self.border_width, value, "PickListStyleParam::BorderWidth"),
            PickListStyleParam::MenuBackgroundColor => set_t_value(&mut self.menu_background_color, value, "PickListStyleParam::MenuBackgroundColor"),
            PickListStyleParam::MenuBackgroundColorAlpha => set_t_value(&mut self.menu_background_color_alpha, value, "PickListStyleParam::MenuBackgroundColorAlpha"),
            PickListStyleParam::MenuBackgroundRgba => set_t_value(&mut self.menu_background_rgba, value, "PickListStyleParam::MenuBackgroundRgba"),
            PickListStyleParam::MenuTextColor => set_t_value(&mut self.menu_text_color, value, "PickListStyleParam::MenuTextColor"),
            PickListStyleParam::MenuTextColorAlpha => set_t_value(&mut self.menu_text_color_alpha, value, "PickListStyleParam::MenuTextColorAlpha"),
            PickListStyleParam::MenuTextRgba => set_t_value(&mut self.menu_text_rgba, value, "PickListStyleParam::MenuTextRgba"),
            PickListStyleParam::MenuSelectedTextColor => set_t_value(&mut self.menu_selected_text_color, value, "PickListStyleParam::MenuSelectedTextColor"),
            PickListStyleParam::MenuSelectedTextColorAlpha => set_t_value(&mut self.menu_selected_text_color_alpha, value, "PickListStyleParam::MenuSelectedTextColorAlpha"),
            PickListStyleParam::MenuSelectedTextRgba => set_t_value(&mut self.menu_selected_text_rgba, value, "PickListStyleParam::MenuSelectedTextRgba"),
            PickListStyleParam::MenuSelectedBackgroundColor => set_t_value(&mut self.menu_selected_background_color, value, "PickListStyleParam::MenuSelectedBackgroundColor"),
            PickListStyleParam::MenuSelectedBackgroundColorAlpha => set_t_value(&mut self.menu_selected_background_color_alpha, value, "PickListStyleParam::MenuSelectedBackgroundColorAlpha"),
            PickListStyleParam::MenuSelectedBackgroundRgba => set_t_value(&mut self.menu_selected_background_rgba, value, "PickListStyleParam::MenuSelectedBackgroundRgba"),
            PickListStyleParam::MenuBorderColor => set_t_value(&mut self.menu_border_color, value, "PickListStyleParam::MenuBorderColor"),
            PickListStyleParam::MenuBorderColorAlpha => set_t_value(&mut self.menu_border_color_alpha, value, "PickListStyleParam::MenuBorderColorAlpha"),
            PickListStyleParam::MenuBorderRgba => set_t_value(&mut self.menu_border_rgba, value, "PickListStyleParam::MenuBorderRgba"),
            PickListStyleParam::MenuBorderRadius => set_t_value(&mut self.menu_border_radius, value, "PickListStyleParam::MenuBorderRadius"),
            PickListStyleParam::MenuBorderWidth => set_t_value(&mut self.menu_border_width, value, "PickListStyleParam::MenuBorderWidth"),
        }
    }
}
