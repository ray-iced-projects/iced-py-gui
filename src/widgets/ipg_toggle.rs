//! ipg_toggler
use iced::widget::toggler::{self, Status};
use iced::widget::Toggler;
use iced::{Color, Element, Length, Theme};
use iced::theme::palette::{deviate, mix};

use pyo3::{pyclass, Py, PyAny, Python};

use crate::app::Message;
use crate::py_api::helpers::get_radius;
use crate::state::IpgWidgets;
use crate::widgets::ipg_text::IpgWrapping;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_bool, set_halign, set_iced_color_from_rgba, set_opt_f32, set_opt_iced_color, set_opt_string, set_opt_text_shaping, set_opt_text_wrapping, set_opt_usize, set_opt_vec_f32, set_width, set_width_fill};
use crate::{IpgState, access_callbacks, access_user_data1};
use crate::widgets::callbacks::{WidgetCallbackIn, set_or_get_widget_callback_data};
use crate::widgets::enums::{IpgHorizontalAlignment, IpgShaping};
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
    pub text_alignment: Option<IpgHorizontalAlignment>,
    pub text_shaping: Option<IpgShaping>,
    pub text_wrapping: Option<IpgWrapping>,
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
    style_opt: Option<&IpgWidgets>,
    font_opt:  Option<&'a IpgWidgets>) 
    -> Option<Element<'a, Message>> {
    
    if !ipg_tog.show {
        return None
    }

    let style = style_opt.and_then(IpgWidgets::as_toggler_style).cloned();

    let mut tog =  
        Toggler::new(ipg_tog.is_toggled)
            .on_toggle(TOGMessage::Toggled)
            .width(ipg_tog.width)
            .style(move|theme: &Theme, status| {     
                get_styling(theme, status, 
                            style.clone()) 
            });

    if let Some(lb)  = &ipg_tog.label {
        tog = tog.label(lb);
    }

    if let Some(sz) = ipg_tog.size {
        tog = tog.size(sz);
    }

    if let Some(align) = &ipg_tog.text_alignment {
        tog = tog.text_alignment(align.to_iced());
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

    let mut wci = WidgetCallbackIn{id, ..Default::default()};

    match message {
        TOGMessage::Toggled(on_toggle) => {
            wci.on_toggle = Some(on_toggle);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(id, "toggled".to_string(), on_toggle);
        }
    }
}

pub fn process_callback(
    id: usize, 
    event_name: String, 
    toggled: bool) 
{
    let ud1 = access_user_data1();
    let app_cbs = access_callbacks();

    // Retrieve the callback
    let callback = match app_cbs.callbacks.get(&(id, event_name)) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_cbs);

    // Check user data from ud1
    if let Some(user_data) = ud1.user_data.get(&id) {
        Python::attach(|py| {
            if let Err(err) = callback.call1(py, (id, toggled, user_data)) {
                panic!("Toggler callback error: {err}");
            }
        });
        drop(ud1); // Drop ud1 before processing ud2
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Check user data from ud2
    // let ud2 = access_user_data2();
    // if let Some(user_data) = ud2.user_data.get(&id) {
    //     Python::attach(|py| {
    //         if let Err(err) = callback.call1(py, (id, toggled, user_data)) {
    //             panic!("Toggler callback error: {err}");
    //         }
    //     });
    //     drop(ud2); // Drop ud2 after processing
    //     return;
    // }
    // drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with the id and toggled
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, toggled)) {
            panic!("Toggler callback error: {err}");
        }
    });
         
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
    TextAlignment,
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

pub fn get_styling(theme: &Theme, status: Status, 
                    style_opt: Option<IpgTogglerStyle>,
                    ) -> toggler::Style {
    
    let tog_style = toggler::default(theme, status);

    let ipg_style = if let Some(so) = style_opt {
        so
    } else { return tog_style };

    // Use user-supplied base colors or fall back to theme palette colors
    let palette = theme.extended_palette();

    let bg_base = ipg_style.background_color
        .unwrap_or(palette.primary.base.color);
    let fg_base = ipg_style.foreground_color
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

    let border_radius = if let Some(br) = ipg_style.border_radius {
        Some(get_radius(&br, "toggle".to_string()))
    } else { None };

    toggler::Style {
        background: background.into(),
        foreground: foreground.into(),
        foreground_border_width: ipg_style.foreground_border_width.unwrap_or(0.0),
        foreground_border_color: ipg_style.foreground_border_color.unwrap_or(Color::TRANSPARENT),
        background_border_width: ipg_style.background_border_width.unwrap_or(0.0),
        background_border_color: Color::TRANSPARENT,
        text_color: ipg_style.text_color,
        border_radius,
        padding_ratio: ipg_style.padding_ratio.unwrap_or(0.1),
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

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgTogglerParam::FontId        => set_opt_usize(&mut self.font_id, value, name),
            IpgTogglerParam::Label         => set_opt_string(&mut self.label, value, name),
            IpgTogglerParam::Show          => set_bool(&mut self.show, value, name),
            IpgTogglerParam::Size          => set_opt_f32(&mut self.size, value, name),
            IpgTogglerParam::Spacing       => set_opt_f32(&mut self.spacing, value, name),
            IpgTogglerParam::StyleId       => set_opt_usize(&mut self.style_id, value, name),
            IpgTogglerParam::TextAlignment => set_halign(&mut self.text_alignment, value, name),
            IpgTogglerParam::TextLineHeight => set_opt_f32(&mut self.text_line_height, value, name),
            IpgTogglerParam::TextShaping   => set_opt_text_shaping(&mut self.text_shaping, value, name),
            IpgTogglerParam::TextSize      => set_opt_f32(&mut self.text_size, value, name),
            IpgTogglerParam::TextWrapping  => set_opt_text_wrapping(&mut self.text_wrapping, value, name),
            IpgTogglerParam::Width         => set_width(&mut self.width, value, name),
            IpgTogglerParam::WidthFill     => set_width_fill(&mut self.width, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgTogglerStyle {
    type Param = IpgTogglerStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
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
