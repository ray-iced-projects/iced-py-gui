//!ipg_radio
use crate::py_api::helpers::get_padding;
use crate::widgets::enums::IpgShaping;
use crate::widgets::ipg_text::IpgWrapping;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_bool, set_height, 
    set_iced_color_from_rgba, set_opt_f32, 
    set_opt_iced_color, set_opt_text_shaping, 
    set_opt_text_wrapping, set_opt_usize, 
    set_opt_vec_f32, set_vec_string, set_width};
use crate::{access_callbacks, access_user_data1, IpgState};
use crate::app;
use crate::state::IpgWidgets;

use iced::widget::radio::{self, Status};
use iced::{Color, Element, Length, Theme};
use iced::widget::{Column, Radio, Row};

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgRadio {
    pub id: usize,
    pub parent_id: String,
    pub labels: Vec<String>,
    pub direction: IpgRadioDirection,
    pub spacing: Option<f32>,
    pub radio_spacing: Option<f32>,
    pub padding: Option<Vec<f32>>,
    pub show: bool,
    pub is_selected: Option<usize>,
    pub width: Length,
    pub height: Length,
    pub size: Option<f32>,
    pub text_spacing: Option<f32>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub text_shaping: Option<IpgShaping>,
    pub text_wrapping: Option<IpgWrapping>,
    pub font_id: Option<usize>,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub struct IpgRadioStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    pub dot_color: Option<Color>,
    pub dot_color_hovered: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub text_color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRadioDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub enum RDMessage {
    RadioSelected(usize),
}


pub fn construct_radio<'a>(rad: &'a IpgRadio, 
                        style_opt: Option<&'a IpgWidgets>,
                        font_opt:  Option<&'a IpgWidgets>) 
                        -> Option<Element<'a, app::Message>> {
    
    if !rad.show {
        return None
    }

    let style_opt = get_radio_style(style_opt);

    let selected = rad.is_selected;

    let mut radio_elements = vec![];

    for (i, label) in  rad.labels.iter().enumerate() {
        let style: Option<IpgRadioStyle> = 
            style_opt.map(|st| IpgRadioStyle{
                id: st.id, 
                background_color: st.background_color, 
                background_color_hovered: st.background_color_hovered, 
                dot_color: st.dot_color, 
                dot_color_hovered: st.dot_color_hovered, 
                border_color: st.border_color, 
                border_width: st.border_width, 
                text_color: st.text_color });

        let mut rd: Radio<'_, RDMessage> = Radio::new(
            label.clone(), 
            i,
            selected,
            RDMessage::RadioSelected
            )
            .width(rad.width)
            .style(move|theme: &Theme, status| {
                get_styling(theme, status, 
                style,
                )});

        if let Some(sz) = rad.size {
            rd = rd.size(sz);
        }

        if let Some(sp) = rad.spacing {
            rd = rd.spacing(sp);
        }

        if let Some(ts) = rad.text_size {
            rd = rd.text_size(ts);
        }

        if let Some(lh) = rad.text_line_height {
            rd = rd.text_line_height(lh);
        }

        if let Some(sh) = &rad.text_shaping {
            rd = rd.text_shaping(sh.to_iced());
        }

        if let Some(wp) = &rad.text_wrapping {
            rd = rd.text_wrapping(wp.to_iced());
        }

        
        let rd = 
        if let Some(wd) = font_opt {
            match wd {
                IpgWidgets::IpgFont(font) => {
                    rd.font(font.to_iced())
                },
                _ => rd
            }
        } else { rd };
        

        radio_elements.push(rd);
        
    }

    let elements: Vec<Element<'_, RDMessage>> = 
        radio_elements.into_iter().map(|r| r.into()).collect();

    let rd: Element<RDMessage> = match rad.direction {
            IpgRadioDirection::Horizontal =>{
                let mut rw: Row<'_, RDMessage> = 
                    Row::with_children(elements)
                        .width(rad.width)
                        .height(rad.height)
                        .padding(get_padding(&rad.padding));
                
                if let Some(rd_sp) = rad.radio_spacing {
                    rw = rw.spacing(rd_sp);
                }
                rw.into()
            },
            IpgRadioDirection::Vertical => {
                let mut col: Column<'_, RDMessage> = 
                    Column::with_children(elements)
                        .padding(get_padding(&rad.padding))
                        .width(rad.width)
                        .height(rad.height);

                if let Some(rd_sp) = rad.radio_spacing {
                    col = col.spacing(rd_sp);
                }                                    
                col.into()                                                               
            },
    };

    Some(rd.map(move |message| app::Message::Radio(rad.id, message)))

}


pub fn radio_callback(state: &mut IpgState, id: usize, message: RDMessage) {

    let widget_opt = state.widgets.get_mut(&id);

    let widgets = match widget_opt {
        Some(rd) => rd,
        None => panic!("Radio callback with id {} could not be found", id),
    };

    let radio: &mut IpgRadio = widgets.as_radio_mut()
        .expect("Radio expected IpgRadio in IpgWidgets");

    let ch_usize = match message {
        RDMessage::RadioSelected(index) => index,
    };

    radio.is_selected = Some(ch_usize);

    process_callback(id, "on_select".to_string(), ch_usize, radio.labels[ch_usize].clone());
    
}


fn process_callback(
    id: usize, 
    event_name: String, 
    index: usize, 
    label: String) 
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
            if let Err(err) = callback.call1(py, (id, (index, label), user_data)) {
                panic!("Radio callback error: {err}");
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
    //         if let Err(err) = callback.call1(py, (id, (index, label), user_data)) {
    //             panic!("Radio callback error: {err}");
    //         }
    //     });
    //     drop(ud2); // Drop ud2 after processing
    //     return;
    // }
    // drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with the id, index, and label
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, (index, label))) {
            panic!("Radio callback error: {err}");
        }
    });
}




#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRadioParam {
    Direction,
    FontId,
    Height,
    HeightFill,
    IsIndex,
    Labels,
    Padding,
    RadioSpacing,
    Show,
    Size,
    Spacing,
    StyleId,
    TextLineHeight,
    TextShaping,
    TextSize,
    TextSpacing,
    TextWrapping,
    Width,
    WidthFill,
}


pub fn extract_radio_direction(direct_obj: &PyObject) -> IpgRadioDirection {
    Python::attach(|py| {
        let res = direct_obj.extract::<IpgRadioDirection>(py);
            
        match res {
            Ok(direction) => direction,
            Err(_) => panic!("RadioDirection failed to extract."),
        }
    })  
}

pub fn get_styling(theme: &Theme, status: Status, 
                    style_opt: Option<IpgRadioStyle>,
                    ) -> radio::Style {
    
    if style_opt.is_none() {
        return radio::default(theme, status)
    }
    
    let mut base_style = radio::default(theme, status);

    let style = style_opt.unwrap();

    base_style.text_color = style.text_color;
    
    if style.background_color.is_some() {
        base_style.background = style.background_color.unwrap().into();
    }

    if style.dot_color.is_some() {
        base_style.dot_color = style.dot_color.unwrap();
    }
    
    // border color changes to inner color during hover
    if style.border_color.is_some() {
        base_style.border_color = style.border_color.unwrap();
    }
    
    if style.border_width.is_some() {
        base_style.border_width = style.border_width.unwrap();
    }
        

    match status {
        Status::Active{..} => base_style,
        Status::Hovered{..} => {
            if style.background_color_hovered.is_some() {
                base_style.background = style.background_color_hovered.unwrap().into();
            }
            if style.dot_color_hovered.is_some() {
                base_style.dot_color = style.dot_color_hovered.unwrap();
            }
            base_style
        },
    }

}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgRadioStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BorderIpgColor,
    BorderRgbaColor,
    BorderWidth,
    DotIpgColor,
    DotRgbaColor,
    DotIpgColorHovered,
    DotRgbaColorHovered,
    TextIpgColor,
    TextRgbaColor,
}

fn get_radio_style(style: Option<&IpgWidgets>) -> Option<IpgRadioStyle>{
    match style {
        Some(IpgWidgets::IpgRadioStyle(style)) => {
            Some(*style)
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgRadio {
    type Param = IpgRadioParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgRadioParam::Direction => self.direction = extract_radio_direction(value),
            IpgRadioParam::FontId => set_opt_usize(&mut self.font_id, value, name),
            IpgRadioParam::Height => set_height(&mut self.height, value, name),
            IpgRadioParam::HeightFill => set_height(&mut self.height, value, name),
            IpgRadioParam::IsIndex => set_opt_usize(&mut self.is_selected, value, name),
            IpgRadioParam::Labels => set_vec_string(&mut self.labels, value, name),
            IpgRadioParam::Padding => set_opt_vec_f32(&mut self.padding, value, name),
            IpgRadioParam::RadioSpacing => set_opt_f32(&mut self.radio_spacing, value, name),
            IpgRadioParam::Show => set_bool(&mut self.show, value, name),
            IpgRadioParam::TextShaping => set_opt_text_shaping(&mut self.text_shaping, value, name),
            IpgRadioParam::Size => set_opt_f32(&mut self.size, value, name),
            IpgRadioParam::Spacing => set_opt_f32(&mut self.spacing, value, name),
            IpgRadioParam::StyleId => set_opt_usize(&mut self.style_id, value, name),
            IpgRadioParam::TextLineHeight => set_opt_f32(&mut self.text_line_height, value, name),
            IpgRadioParam::TextSize => set_opt_f32(&mut self.text_size, value, name),
            IpgRadioParam::TextSpacing => set_opt_f32(&mut self.text_spacing, value, name),
            IpgRadioParam::Width => set_width(&mut self.width, value, name),
            IpgRadioParam::WidthFill => set_width(&mut self.width, value, name),
            IpgRadioParam::TextWrapping => set_opt_text_wrapping(&mut self.text_wrapping, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgRadioStyle {
    type Param = IpgRadioStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgRadioStyleParam::BackgroundIpgColor => 
                set_opt_iced_color(&mut self.background_color, value, name),
            IpgRadioStyleParam::BackgroundRgbaColor => 
                set_iced_color_from_rgba(&mut self.background_color, value, name),
            IpgRadioStyleParam::BorderIpgColor => 
                set_opt_iced_color(&mut self.border_color, value, name),
            IpgRadioStyleParam::BorderRgbaColor => 
                set_iced_color_from_rgba(&mut self.border_color, value, name),
            IpgRadioStyleParam::BorderWidth => 
                set_opt_f32(&mut self.border_width, value, name),
            IpgRadioStyleParam::DotIpgColor => 
                set_opt_iced_color(&mut self.dot_color, value, name),
            IpgRadioStyleParam::DotRgbaColor => 
                set_iced_color_from_rgba(&mut self.dot_color, value, name),
            IpgRadioStyleParam::DotIpgColorHovered => 
                set_opt_iced_color(&mut self.dot_color_hovered, value, name),
            IpgRadioStyleParam::DotRgbaColorHovered => 
                set_iced_color_from_rgba(&mut self.dot_color_hovered, value, name),
            IpgRadioStyleParam::TextIpgColor => 
                set_opt_iced_color(&mut self.text_color, value, name),
            IpgRadioStyleParam::TextRgbaColor => 
                set_iced_color_from_rgba(&mut self.text_color, value, name),
        }
    }
}

