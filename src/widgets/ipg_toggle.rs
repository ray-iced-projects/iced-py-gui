//! ipg_toggler
use iced::advanced::text;
use iced::widget::toggler::{self, Status};
use iced::widget::Toggler;
use iced::{Color, Element, Length, Theme};
use iced::theme::palette::{deviate, mix};

use pyo3::{pyclass, Py, PyAny};

use crate::app::Message;
use crate::py_api::helpers::get_radius;
use crate::state::IpgWidgets;
use crate::widgets::ipg_text::TextWrapping;
use crate::widgets::widget_param_update::{WidgetParamUpdate, 
    set_bool, set_iced_color_from_rgba, set_opt_bool, set_opt_f32, 
    set_opt_iced_color, set_opt_string, set_opt_text_shaping, 
    set_opt_text_wrapping, set_opt_usize, set_opt_vec_f32, 
    set_width, set_width_fill};
use crate::IpgState;
use crate::widgets::callbacks::invoke_callback_with_args;
use crate::widgets::ipg_text::TextShaping;
type PyObject = Py<PyAny>;




#[derive(Debug, Clone)]
pub struct IpgToggler {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub is_toggled: bool,
    pub label: Option<String>,
    pub width: Length,
    pub size: Option<f32>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub text_center: Option<bool>,
    pub text_left: Option<bool>,
    pub text_right: Option<bool>,
    pub text_shaping: Option<TextShaping>,
    pub text_wrapping: Option<TextWrapping>,
    pub spacing: Option<f32>,
    pub font_id: Option<usize>,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum TOGMessage {
    Toggled(bool),
}

pub fn construct_toggler<'a>(
    ipg_tog: &'a IpgToggler, 
    widget: Option<&IpgWidgets>,
    font_opt:  Option<&'a IpgWidgets>) 
    -> Option<Element<'a, Message>> {
    
    if !ipg_tog.show {
        return None
    }

    let style_opt = 
        widget.and_then(IpgWidgets::as_toggler_style).cloned();

    let mut tog =  
        Toggler::new(ipg_tog.is_toggled)
            .on_toggle(TOGMessage::Toggled)
            .width(ipg_tog.width)
            .style(move|theme: &Theme, status| {
                if let Some(st) = &style_opt {
                    st.set_style(theme, status)
                } else {
                    toggler::default(theme, status)
                } 
            });

    if let Some(lb)  = &ipg_tog.label {
        tog = tog.label(lb);
    }

    if let Some(sz) = ipg_tog.size {
        tog = tog.size(sz);
    }

    if ipg_tog.text_center == Some(true){
        tog = tog.text_alignment(text::Alignment::Center);
    }

    if ipg_tog.text_left == Some(true){
        tog = tog.text_alignment(text::Alignment::Left);
    }

    if ipg_tog.text_right == Some(true){
        tog = tog.text_alignment(text::Alignment::Right);
    }

    if let Some(lh) = ipg_tog.text_line_height {
        tog = tog.text_line_height(lh);
    }

    if let Some(ts) = ipg_tog.text_size {
        tog = tog.text_size(ts);
    }

    if let Some(tw) = ipg_tog.text_wrapping {
        tog = tog.text_wrapping(tw.to_iced());
    }

    if let Some(sp) = ipg_tog.spacing {
        tog = tog.spacing(sp);
    }

    if let Some(wd) = font_opt {
        match wd {
            IpgWidgets::IpgFont(font) => {
                tog = tog.font(font.to_iced());
            },
            _ => ()
        }
    }

    let tog: Element<'_, TOGMessage> = tog.into();
    Some(tog.map(move |message| Message::Toggler(ipg_tog.id, message)))

}


pub fn toggle_callback(state: &mut IpgState, id: usize, message: TOGMessage) {
    match message {
        TOGMessage::Toggled(is_toggled) => {
            // Update widget state directly
            if let Some(IpgWidgets::IpgToggler(tog)) = state.widgets.get_mut(&id) {
                tog.is_toggled = is_toggled;
            }
            invoke_callback_with_args(id, "toggled", "Toggler", is_toggled);
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTogglerParam {
    FontId,
    Label,
    Show,
    Size,
    Spacing,
    StyleId,
    TextCenter,
    TextLeft,
    TextRight,
    TextLineHeight,
    TextShaping,
    TextSize,
    TextWrapping,
    Width,
    WidthFill,
}

#[derive(Debug, Clone)]
pub struct IpgTogglerStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_border_color: Option<Color>,
    pub background_border_width: Option<f32>,

    pub foreground_color: Option<Color>,
    pub foreground_border_color: Option<Color>,
    pub foreground_border_width: Option<f32>,
    
    pub text_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub padding_ratio: Option<f32>,
}

//      Status	            Background	       Foreground
// Active + toggled	        bg_base	            fg_base
// Active + untoggled	    deviate(bg, 0.15)	deviate(fg, 0.15)
// Hovered + toggled	    bg_base	            fg_base at 50% alpha
// Hovered + untoggled	    deviate(bg, 0.15)	deviate(fg, 0.1)
// Disabled + toggled	    deviate(bg, 0.1)	mix(fg, bg, 0.4)
// Disabled + untoggled	    deviate(bg, 0.2)	mix(fg, bg, 0.4)
impl IpgTogglerStyle {
    fn set_style(
        &self, 
        theme: &Theme, 
        status: Status, 
        ) -> toggler::Style {
        
        // Use user-supplied base colors or fall back to theme palette colors
        let palette = theme.extended_palette();

        let bg_base = self.background_color
            .unwrap_or(palette.primary.base.color);
        let fg_base = self.foreground_color
            .unwrap_or(palette.primary.base.text);

        // Derive variants: deviate auto-lightens dark colors and darkens light ones
        let bg_untoggled = deviate(bg_base, 0.15);      // like "strong"
        let bg_disabled_on = deviate(bg_base, 0.1);
        let bg_disabled_off = deviate(bg_base, 0.2);

        let background = match status {
            Status::Active { is_toggled } | Status::Hovered { is_toggled } => {
                if is_toggled { bg_base } else { bg_untoggled }
            }
            Status::Disabled { is_toggled } => {
                if is_toggled { bg_disabled_on } else { bg_disabled_off }
            }
        };

        let fg_untoggled = deviate(fg_base, 0.15);
        let fg_hovered_on = Color { a: 0.5, ..fg_base };
        let fg_hovered_off = deviate(fg_base, 0.1);
        let fg_disabled = mix(fg_base, background, 0.4);

        let foreground = match status {
            Status::Active { is_toggled } => {
                if is_toggled { fg_base } else { fg_untoggled }
            }
            Status::Hovered { is_toggled } => {
                if is_toggled { fg_hovered_on } else { fg_hovered_off }
            }
            Status::Disabled { .. } => fg_disabled,
        };

        let border_radius = if let Some(br) = &self.border_radius {
            Some(get_radius(&br, "toggle".to_string()))
        } else { None };

        toggler::Style {
            background: background.into(),
            foreground: foreground.into(),
            foreground_border_width: self.foreground_border_width.unwrap_or(0.0),
            foreground_border_color: self.foreground_border_color.unwrap_or(Color::TRANSPARENT),
            background_border_width: self.background_border_width.unwrap_or(0.0),
            background_border_color: Color::TRANSPARENT,
            text_color: self.text_color,
            border_radius,
            padding_ratio: self.padding_ratio.unwrap_or(0.1),
        }

    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTogglerStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BackgroundBorderIpgColor,
    BackgroundBorderRgbaColor,
    BackgroundBorderWidth,
    ForegroundIpgColor,
    ForegroundRgbaColor,
    ForegroundBorderIpgColor,
    ForegroundBorderRgbaColor,
    ForegroundBorderWidth,
    TextIpgColor,
    TextRgbaColor,
    BorderRadius,
    PaddingRatio,
}



// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgToggler {
    type Param = IpgTogglerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        let name = String::new();
        match param {
            IpgTogglerParam::FontId => set_opt_usize(&mut self.font_id, value, name),
            IpgTogglerParam::Label => set_opt_string(&mut self.label, value, "Label"),
            IpgTogglerParam::Show => set_bool(&mut self.show, value, name),
            IpgTogglerParam::Size => set_opt_f32(&mut self.size, value, name),
            IpgTogglerParam::Spacing => set_opt_f32(&mut self.spacing, value, name),
            IpgTogglerParam::StyleId => set_opt_usize(&mut self.style_id, value, name),
            IpgTogglerParam::TextCenter => set_opt_bool(&mut self.text_center, value, name),
            IpgTogglerParam::TextLeft => set_opt_bool(&mut self.text_left, value, name),
            IpgTogglerParam::TextRight => set_opt_bool(&mut self.text_right, value, name),
            IpgTogglerParam::TextLineHeight => set_opt_f32(&mut self.text_line_height, value, name),
            IpgTogglerParam::TextShaping => set_opt_text_shaping(&mut self.text_shaping, value, name),
            IpgTogglerParam::TextSize => set_opt_f32(&mut self.text_size, value, name),
            IpgTogglerParam::TextWrapping => set_opt_text_wrapping(&mut self.text_wrapping, value, name),
            IpgTogglerParam::Width => set_width(&mut self.width, value, name),
            IpgTogglerParam::WidthFill => set_width_fill(&mut self.width, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgTogglerStyle {
    type Param = IpgTogglerStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        let name = String::new();
        match param {
            IpgTogglerStyleParam::BackgroundIpgColor =>
                set_opt_iced_color(&mut self.background_color, value, name),
            IpgTogglerStyleParam::BackgroundRgbaColor =>
                set_iced_color_from_rgba(&mut self.background_color, value, name),
            IpgTogglerStyleParam::BackgroundBorderIpgColor =>
                set_opt_iced_color(&mut self.background_border_color, value, name),
            IpgTogglerStyleParam::BackgroundBorderRgbaColor =>
                set_iced_color_from_rgba(&mut self.background_border_color, value, name),
            IpgTogglerStyleParam::BackgroundBorderWidth =>
                set_opt_f32(&mut self.background_border_width, value, name),
            IpgTogglerStyleParam::ForegroundIpgColor =>
                set_opt_iced_color(&mut self.foreground_color, value, name),
            IpgTogglerStyleParam::ForegroundRgbaColor =>
                set_iced_color_from_rgba(&mut self.foreground_color, value, name),
            IpgTogglerStyleParam::ForegroundBorderIpgColor =>
                set_opt_iced_color(&mut self.foreground_border_color, value, name),
            IpgTogglerStyleParam::ForegroundBorderRgbaColor =>
                set_iced_color_from_rgba(&mut self.foreground_border_color, value, name),
            IpgTogglerStyleParam::ForegroundBorderWidth =>
                set_opt_f32(&mut self.foreground_border_width, value, name),
            IpgTogglerStyleParam::TextIpgColor =>
                set_opt_iced_color(&mut self.text_color, value, name),
            IpgTogglerStyleParam::TextRgbaColor =>
                set_iced_color_from_rgba(&mut self.text_color, value, name),
            IpgTogglerStyleParam::BorderRadius =>
                set_opt_vec_f32(&mut self.border_radius, value, name),
            IpgTogglerStyleParam::PaddingRatio =>
                set_opt_f32(&mut self.padding_ratio, value, name),
        }
    }
}
