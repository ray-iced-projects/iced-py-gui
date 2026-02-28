//! ipg_checkbox
use std::collections::HashMap;

use crate::widgets::enums::IpgShaping;
use crate::state::{access_callbacks, access_user_data1, 
    access_user_data2, IpgState};
use crate::app::Message;
use crate::widgets::ipg_text::IpgWrapping;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,
    set_bool, set_opt_f32, set_opt_string, set_opt_usize, set_opt_vec_f32,
    set_width, set_width_fill,
    set_opt_iced_color, set_iced_color_from_rgba,
};
use crate::widgets::callbacks::{set_or_get_widget_callback_data, 
    WidgetCallbackIn};
use crate::state::IpgWidgets;

use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap_icon::{IpgIcon, icon_to_char};

use crate::widgets::styling::apply_border_overrides;

use iced::advanced::text;
use iced::{Background, Element, Color, Length, Theme};
use iced::widget::text::{LineHeight, Shaping};
use iced::widget::{Checkbox, checkbox};
use iced::theme::palette;

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgCheckBox {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub is_checked: bool,
    pub label: Option<String>,
    pub width: Length,
    pub size: Option<f32>,
    pub spacing: Option<f32>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub text_shaping: Option<IpgShaping>,
    pub text_wrapping: Option<IpgWrapping>,
    pub text_font_id: Option<usize>,
    pub icon_font_id: Option<usize>,
    pub icon: Option<IpgIcon>,
    pub icon_size: Option<f32>,
    pub icon_line_height: Option<f32>,
    pub icon_shaping: Option<IpgShaping>,
    pub style_id: Option<usize>,
    pub style_standard: Option<IpgCheckboxStyleStandard>,
}

#[derive(Debug, Clone)]
pub enum ChkMessage {
    OnToggle(bool),
}

impl IpgCheckBox {

    fn lookup<'a>(&self, widgets: &'a HashMap<usize, IpgWidgets>, id: Option<usize>) -> Option<&'a IpgWidgets> {
        id.and_then(|id| widgets.get(&id))
    }

    pub fn construct<'a>(
        &self,
        widgets: &HashMap<usize, IpgWidgets>,
    ) -> Option<Element<'a, Message>> {

        if !self.show {
            return None
        };

        let style_opt = 
            self.lookup(widgets, self.style_id)
                .and_then(IpgWidgets::as_button_style).cloned();

        // Icon related
        let code_point = 
            if let Some(ic) = self.icon {
                icon_to_char(ic)
            } else {
                icon_to_char(IpgIcon::Check)
            };

        let size = 
            if let Some(sz) = self.icon_size {
                Some(iced::Pixels(sz))
            } else { None };

        let shaping = 
            if let Some(sh) = &self.icon_shaping {
                IpgShaping::to_iced(sh)
            } else { Shaping::default() };

        let line_height = 
            if let Some(lh) = self.icon_line_height {
                LineHeight::Relative(lh)
            } else { LineHeight::default() };

        let icon = 
            checkbox::Icon {
                font: BOOTSTRAP_FONT,
                code_point,
                size,
                line_height,
                shaping,
            };
        
        // Text related
        let text_line_height = 
            if let Some(lh) = self.text_line_height {
                text::LineHeight::Relative(lh)
            } else { text::LineHeight::default() };

        let text_shaping = 
            if let Some(ts) = &self.text_shaping {
                IpgShaping::to_iced(ts)
            } else { Shaping::default() };

    let chk = 
            Checkbox::new(self.is_checked)
                .on_toggle(ChkMessage::OnToggle)
                .width(self.width)
                .text_line_height(text_line_height)
                .text_shaping(text_shaping)
                .icon(icon)
                .style(move|theme: &Theme, status| {   
                    if let Some(st) = &style_opt {
                        st.to_iced(theme, status, &self.style_standard)
                    } else {
                       match &self.style_standard {
                            Some(std) => std.to_iced(theme, status),
                            None => checkbox::primary(theme, status),
                        }
                    }
                    });
        
        let chk = 
            if let Some(lb) = &self.label {
                chk.label(lb.clone())
            } else { chk };

        let chk = 
            if let Some(sz) = self.size {
                chk.size(sz)
            } else { chk };

        let chk: Element<'_, ChkMessage> = 
            if let Some(sp) = self.spacing {
                chk.spacing(sp).into()
            } else { chk.into() };

        Some(chk.map(move |message| Message::CheckBox(self.id, message)))

    }
}


#[derive(Debug, Clone, Default)]
pub struct IpgCheckboxStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub accent_color: Option<Color>,
    pub border_color: Option<Color>,
    pub border_radius: Option<Vec<f32>>,
    pub border_width: Option<f32>,
    pub icon_color: Option<Color>,
    pub text_color: Option<Color>,
}

impl IpgCheckboxStyle {
    /// Apply user-defined style overrides to an existing iced checkbox::Style
    pub fn to_iced(
         &self, 
        theme: &Theme, 
        status: checkbox::Status,
        std_style_opt: &Option<IpgCheckboxStyleStandard>,
        ) -> checkbox::Style {

        // Default the style to primary unless user supplies another standard style.
        let style = if let Some(std) = std_style_opt {
            std.to_iced(theme, status)
        } else { checkbox::primary(theme, status) };

        // If user suppies a bkg color then pair with the text color, 
        // if user suppied a text color too.
        let mut style = if let Some(bkg) = self.background_color {
            let border_color = if let Some(bc) = self.border_color {
                bc
            } else { style.border.color };
        
            // let palette = palette::Background::new(bkg, border_color);
            
            // let base = styled(palette.base);
        
            let is_checked = matches!(status,
                checkbox::Status::Active { is_checked: true }
                | checkbox::Status::Hovered { is_checked: true }
                | checkbox::Status::Disabled { is_checked: true }
            );

        
            match status {
                checkbox::Status::Active { is_checked } => styled(
                    palette.background.strong.color,
                    palette.background.base,
                    palette.primary.base.text,
                    palette.primary.base,
                    is_checked,
                ),
                checkbox::Status::Hovered { is_checked } => styled(
                    palette.background.strong.color,
                    palette.background.weak,
                    palette.primary.base.text,
                    palette.primary.strong,
                    is_checked,
                ),
                checkbox::Status::Disabled { is_checked } => styled(
                    palette.background.weak.color,
                    palette.background.weaker,
                    palette.primary.base.text,
                    palette.background.strong,
                    is_checked,
                ),
            }
        };

        apply_border_overrides(
            &mut style.border, self.border_color,
            &self.border_radius, self.border_width, "Checkbox",
        );

        if let Some(color) = self.icon_color {
            style.icon_color = color;
        };

        style

    }

}

fn styled(
    border_color: Color,
    base: palette::Pair,
    icon_color: Color,
    accent: palette::Pair,
    is_checked: bool,
) -> checkbox::Style {

    let (background, border) = if is_checked {
        (accent, accent.color)
    } else {
        (base, border_color)
    };

    checkbox::Style {
        background: Background::Color(background.color),
        icon_color,
        border: iced::Border {
            radius: 2.0.into(),
            width: 1.0,
            color: border,
        },
        text_color: None,
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCheckboxStyleStandard {
    Danger,
    Primary,
    Secondary,
    Success,
}

impl IpgCheckboxStyleStandard {
    pub fn to_iced (
        &self,
        theme: &Theme, 
        status: checkbox::Status, 
        ) -> checkbox::Style {
        
        match self {
            IpgCheckboxStyleStandard::Danger => {
                checkbox::danger(theme, status)
            },
            IpgCheckboxStyleStandard::Primary => {
                checkbox::primary(theme, status)
            },
            IpgCheckboxStyleStandard::Secondary => {
                checkbox::secondary(theme, status)
            },
            IpgCheckboxStyleStandard::Success => {
                checkbox::success(theme, status)
            },
        }
    }
}

pub fn checkbox_callback(state: &mut IpgState, id: usize, message: ChkMessage) {

    match message {
        ChkMessage::OnToggle(on_toggle) => {
            let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
            wci.on_toggle = Some(on_toggle);
            let _ = set_or_get_widget_callback_data(state, wci);

            process_callback(id, on_toggle, "on_toggle".to_string());
        }
    }
}

pub fn process_callback(
        id: usize, 
        is_checked: bool, 
        event_name: String) 
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
            if let Err(err) = callback.call1(py, (id, is_checked, user_data)) {
                panic!("Checkbox callback error: {err}");
            }
        });
        drop(ud1); // Drop ud1 before processing ud2
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Check user data from ud2
    let ud2 = access_user_data2();
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::attach(|py| {
            if let Err(err) = callback.call1(py, (id, is_checked, user_data)) {
                panic!("Checkbox callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only the id and is_checked
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, is_checked)) {
            panic!("Checkbox callback error: {err}");
        }
    });
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCheckboxParam {
    Icon,
    IconFont,
    IconLineHeight,
    IconSize,
    IconShaping,
    IsChecked,
    Label,
    Spacing,
    Style,
    StyleStandard,
    TextFont,
    TextLineHeight,
    TextShaping,
    TextSize,
    Width,
    WidthFill,
    Show,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgCheckboxStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BackgroundIpgColorHovered,
    BackgroundRgbaColorHovered,
    AccentIpgColor,
    AccentRgbaColor,
    AccentIpgColorHovered,
    AccentRgbaColorHovered,
    BorderIpgColor,
    BorderRgbaColor,
    BorderRadius,
    BorderWidth,
    IconIpgColor,
    IconRgbaColor,
    TextIpgColor,
    TextRgbaColor,
}

fn extract_chk_style_standard(
    value: &PyObject, 
    ) -> IpgCheckboxStyleStandard {
    
    Python::attach(|py| {

        let res = 
            value.extract::<IpgCheckboxStyleStandard>(py);
        match res {
            Ok(val) => val,
            Err(_) => panic!("Unable to extract python object for CheckboxStyleStandard"),
        }
    })
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgCheckBox {
    type Param = IpgCheckboxParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgCheckboxParam::Icon => {
                self.icon = Some(IpgIcon::extract(value));
            }
            IpgCheckboxParam::IconFont     => { /* TODO */ }
            IpgCheckboxParam::IconLineHeight => set_opt_f32(&mut self.icon_line_height, value, name),
            IpgCheckboxParam::IconSize     => set_opt_f32(&mut self.icon_size, value, name),
            IpgCheckboxParam::IconShaping  => {
                self.icon_shaping = IpgShaping::extract(value);
            }
            IpgCheckboxParam::IsChecked    => set_bool(&mut self.is_checked, value, name),
            IpgCheckboxParam::Label        => set_opt_string(&mut self.label, value, name),
            IpgCheckboxParam::Show         => set_bool(&mut self.show, value, name),
            IpgCheckboxParam::Spacing      => set_opt_f32(&mut self.spacing, value, name),
            IpgCheckboxParam::TextLineHeight => set_opt_f32(&mut self.text_line_height, value, name),
            IpgCheckboxParam::TextShaping  => {
                self.text_shaping = IpgShaping::extract(value);
            }
            IpgCheckboxParam::TextSize     => set_opt_f32(&mut self.text_size, value, name),
            IpgCheckboxParam::Style        => set_opt_usize(&mut self.style_id, value, name),
            IpgCheckboxParam::StyleStandard => {
                self.style_standard = Some(extract_chk_style_standard(value));
            }
            IpgCheckboxParam::Width        => set_width(&mut self.width, value, name),
            IpgCheckboxParam::WidthFill    => set_width_fill(&mut self.width, value, name),
            IpgCheckboxParam::TextFont     => { /* TODO */ }
        }
    }
}

impl WidgetParamUpdate for IpgCheckboxStyle {
    type Param = IpgCheckboxStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgCheckboxStyleParam::BackgroundIpgColor       => set_opt_iced_color(&mut self.background_color, value, name),
            IpgCheckboxStyleParam::BackgroundRgbaColor      => set_iced_color_from_rgba(&mut self.background_color, value, name),
            IpgCheckboxStyleParam::BackgroundIpgColorHovered => set_opt_iced_color(&mut self.background_color_hovered, value, name),
            IpgCheckboxStyleParam::BackgroundRgbaColorHovered => set_iced_color_from_rgba(&mut self.background_color_hovered, value, name),
            IpgCheckboxStyleParam::AccentIpgColor           => set_opt_iced_color(&mut self.accent_color, value, name),
            IpgCheckboxStyleParam::AccentRgbaColor          => set_iced_color_from_rgba(&mut self.accent_color, value, name),
            IpgCheckboxStyleParam::AccentIpgColorHovered    => set_opt_iced_color(&mut self.accent_color_hovered, value, name),
            IpgCheckboxStyleParam::AccentRgbaColorHovered   => set_iced_color_from_rgba(&mut self.accent_color_hovered, value, name),
            IpgCheckboxStyleParam::BorderIpgColor           => set_opt_iced_color(&mut self.border_color, value, name),
            IpgCheckboxStyleParam::BorderRgbaColor          => set_iced_color_from_rgba(&mut self.border_color, value, name),
            IpgCheckboxStyleParam::BorderRadius             => set_opt_vec_f32(&mut self.border_radius, value, name),
            IpgCheckboxStyleParam::BorderWidth              => set_opt_f32(&mut self.border_width, value, name),
            IpgCheckboxStyleParam::IconIpgColor             => set_opt_iced_color(&mut self.icon_color, value, name),
            IpgCheckboxStyleParam::IconRgbaColor            => set_iced_color_from_rgba(&mut self.icon_color, value, name),
            IpgCheckboxStyleParam::TextIpgColor             => set_opt_iced_color(&mut self.text_color, value, name),
            IpgCheckboxStyleParam::TextRgbaColor            => set_iced_color_from_rgba(&mut self.text_color, value, name),
        }
    }
}