//! ipg_divider

use iced::{Background, Color, Element, Length, Theme};
use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use crate::{IpgState, access_callbacks, access_user_data1, app, 
    py_api::helpers::get_radius, state::IpgWidgets, 
    widgets::{callbacks::{WidgetCallbackIn, set_or_get_widget_callback_data}, 
    divider::{self, Direction, Status, Style, divider_horizontal, divider_vertical}}};
use crate::widgets::widget_param_update::{
    WidgetParamUpdate,
    set_bool, set_f32, set_opt_bool, set_opt_f32, set_opt_usize, set_opt_vec_f32,
    set_vec_f32, set_opt_iced_color, set_rgba_color_via_ipg,
};




#[derive(Debug, Clone, PartialEq)]
pub struct IpgDividerHorizontal {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub widths: Vec<f32>,
    pub handle_width: f32,
    pub handle_height: f32,
    pub handle_offsets: Option<Vec<f32>>,
    pub include_last_handle: bool,
    pub width: Length,
    pub height: Length,
    pub index_in_use: usize,
    pub value_in_use: f32,
    pub style_id: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpgDividerVertical {
    pub id: usize,
    pub parent_id: String,
    pub show: bool,
    pub heights: Vec<f32>,
    pub handle_width: f32,
    pub handle_height: f32,
    pub handle_offsets: Option<Vec<f32>>,
    pub include_last_handle: bool,
    pub width: Length,
    pub height: Length,
    pub index_in_use: usize,
    pub value_in_use: f32,
    pub style_id: Option<usize>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct IpgDividerStyle {
    pub id: usize,
    pub background_color: Option<Color>,
    pub background_color_hovered: Option<Color>,
    pub background_transparent: Option<bool>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
    pub border_radius: Option<Vec<f32>>,
}

#[derive(Debug, Clone)]
pub enum DivMessage {
    OnChange((usize, usize, f32)),
    OnRelease,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgDividerDirection {
    /// Horizontal resizing
    Horizontal,
    /// Vertical resizing
    Vertical,
}

pub fn construct_divider_horizontal<'a>(
        divider: &'a IpgDividerHorizontal, 
        style_opt: Option<&IpgWidgets>) 
        -> Option<Element<'a, app::Message>> {

    if !divider.show {
        return None
    }

    let style = get_divider_style(style_opt);

    let offsets = match divider.handle_offsets.clone() {
        Some(offsets) => offsets,
        None => {
            let mut offsets = vec![-divider.handle_width/2.0; divider.widths.len()-1];
            offsets.extend([-divider.handle_width]);
            offsets
        }
    };

    let div: Element<DivMessage, Theme> = 
        divider_horizontal(
            divider.id,
            divider.widths.clone(),
            divider.handle_width,
            divider.handle_height, 
            DivMessage::OnChange
            )
            .on_release(DivMessage::OnRelease)
            .direction(Direction::Horizontal)
            .width(divider.width)
            .height(divider.height)
            .handle_offsets(offsets)
            .include_last_handle(divider.include_last_handle)
            .style(move|theme, status|
                get_styling(theme, status,
                style.clone())
                )
            .into();

    Some(div.map(move |message| app::Message::Divider(divider.id, message)))
}

pub fn construct_divider_vertical<'a>(
        divider: &'a IpgDividerVertical, 
        style_opt: Option<&IpgWidgets>) 
        -> Option<Element<'a, app::Message>> {

    if !divider.show {
        return None
    }

    let style = get_divider_style(style_opt);

    let offsets = match divider.handle_offsets.clone() {
        Some(offsets) => offsets,
        None => {
            let mut offsets = vec![-divider.handle_height/2.0; divider.heights.len()-1];
            offsets.extend([-divider.handle_height]);
            offsets
        }
    };

    let div: Element<DivMessage, Theme> = 
        divider_vertical(
            divider.id,
            divider.heights.clone(),
            divider.handle_width,
            divider.handle_height, 
            DivMessage::OnChange
            )
            .on_release(DivMessage::OnRelease)
            .direction(Direction::Vertical)
            .width(divider.width)
            .height(divider.height)
            .handle_offsets(offsets)
            .include_last_handle(divider.include_last_handle)
            .style(move|theme, status|
                get_styling(theme, status,
                style.clone())
                )
            .into();

    Some(div.map(move |message| app::Message::Divider(divider.id, message)))
}

pub fn divider_callback(state: &mut IpgState, id: usize, message: DivMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
           
    match message {
        DivMessage::OnChange((id, index, value)) => {
            wci.value_f32 = Some(value);
            wci.value_usize = Some(index);
            wci.value_str = Some("on_change".to_string());
            let _ = set_or_get_widget_callback_data(state, wci);
            process_callback(
                id, 
                "on_change".to_string(), 
                index, 
                value);
        },
        DivMessage::OnRelease => {
            // to be consistent, returning values for both
            wci.value_str = Some("on_release".to_string());
            let wco = set_or_get_widget_callback_data(state, wci);
            process_callback(
                id, 
                "on_release".to_string(), 
                wco.value_usize.unwrap(), 
                wco.value_f32.unwrap());
        },
    }
}

pub fn process_callback(id: usize, event_name: String, index: usize, value: f32) {
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
            let res = callback.call1(py, (id, index, value, user_data));
            if let Err(err) = res {
                panic!("Divider callback error: {err}");
            }
        });
        drop(ud1); // Drop ud1 before processing ud2
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // // Check user data from ud2
    // let ud2 = access_user_data2();
    // if let Some(user_data) = ud2.user_data.get(&id) {
    //     Python::attach(|py| {
    //         let res = callback.call1(py, (id, index, value, user_data));
    //         if let Err(err) = res {
    //             panic!("Divider callback error: {err}");
    //         }
    //     });
    //     drop(ud2); // Drop ud2 after processing
    //     return;
    // }
    // drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only id, index, and value
    Python::attach(|py| {
        let res = callback.call1(py, (id, index, value));
        if let Err(err) = res {
            panic!("Divider callback error: {err}");
        }
    });
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgDividerParam {
    HandleWidth,
    HandleHeight,
    Widths,
    Heights,
    StyleId,
    Show,
}

fn get_styling(theme: &Theme, 
                status: Status,
                style_opt: Option<IpgDividerStyle>) 
                -> Style {

    if style_opt.is_none() {
        return divider::primary(theme, status)
    }     
    
    let style = style_opt.unwrap();

    if style.background_transparent == Some(true) {
        return divider::transparent(theme, status);
    }

    let mut base_style = divider::primary(theme, status);

    if let Some(bc) = style.background_color {
        base_style.background = Background::Color(bc);
    };

    if let Some(br) = style.border_radius {
        base_style.border_radius = 
            get_radius(br,  "Divider".to_string());
        }

    if let Some(bc) = style.border_color {
        base_style.border_color = bc;
    }

    if let Some(bw) = style.border_width {
        base_style.border_width = bw;
    }
    let mut hovered_style = base_style;

    if let Some(bch) = style.background_color_hovered {
        hovered_style.background = bch.into();
    }

    match status 
    {
        Status::Active => base_style,
        Status::Hovered => hovered_style,
        Status::Dragged => base_style, // active and drag are same
    }


}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgDividerStyleParam {
    BackgroundIpgColor,
    BackgroundRgbaColor,
    BackgroundTransparent,
    BorderIpgColor,
    BorderRgbaColor,
    BorderWidth,
    BorderRadius,
}

fn get_divider_style(style: Option<&IpgWidgets>) -> Option<IpgDividerStyle>{
    match style {
        Some(IpgWidgets::IpgDividerStyle(style)) => {
            Some(style.clone())
        }
            _ => None,
        }
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgDividerHorizontal {
    type Param = IpgDividerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgDividerParam::HandleWidth  => set_f32(&mut self.handle_width, value, name),
            IpgDividerParam::HandleHeight => set_f32(&mut self.handle_height, value, name),
            IpgDividerParam::Widths       => set_vec_f32(&mut self.widths, value, name),
            IpgDividerParam::Heights      => panic!("Horizontal Divider must use the Widths not Heights"),
            IpgDividerParam::StyleId      => set_opt_usize(&mut self.style_id, value, name),
            IpgDividerParam::Show         => set_bool(&mut self.show, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgDividerVertical {
    type Param = IpgDividerParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgDividerParam::HandleWidth  => set_f32(&mut self.handle_width, value, name),
            IpgDividerParam::HandleHeight => set_f32(&mut self.handle_height, value, name),
            IpgDividerParam::Widths       => panic!("Vertical Divider must use the Heights not Widths"),
            IpgDividerParam::Heights      => set_vec_f32(&mut self.heights, value, name),
            IpgDividerParam::StyleId      => set_opt_usize(&mut self.style_id, value, name),
            IpgDividerParam::Show         => set_bool(&mut self.show, value, name),
        }
    }
}

impl WidgetParamUpdate for IpgDividerStyle {
    type Param = IpgDividerStyleParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgDividerStyleParam::BackgroundIpgColor   => set_opt_iced_color(&mut self.background_color, value, name),
            IpgDividerStyleParam::BackgroundRgbaColor  => set_rgba_color_via_ipg(&mut self.background_color, value, name),
            IpgDividerStyleParam::BackgroundTransparent => set_opt_bool(&mut self.background_transparent, value, name),
            IpgDividerStyleParam::BorderIpgColor       => set_opt_iced_color(&mut self.border_color, value, name),
            IpgDividerStyleParam::BorderRgbaColor      => set_rgba_color_via_ipg(&mut self.border_color, value, name),
            IpgDividerStyleParam::BorderWidth          => set_opt_f32(&mut self.border_width, value, name),
            IpgDividerStyleParam::BorderRadius         => set_opt_vec_f32(&mut self.border_radius, value, name),
        }
    }
}
