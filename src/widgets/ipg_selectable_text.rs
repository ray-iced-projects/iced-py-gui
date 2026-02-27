//!ipg_selectable_text
#![allow(clippy::enum_variant_names)]
use crate::access_user_data1;
use crate::app;
use crate::access_callbacks;
use crate::state::IpgWidgets;
use crate::widgets::enums::IpgAlignmentX;
use crate::widgets::enums::IpgShaping;
use crate::widgets::enums::IpgAlignmentY;
use crate::widgets::enums::h_v_centered;
use crate::widgets::ipg_text::IpgWrapping;
use crate::widgets::widget_param_update::WidgetParamUpdate;
use crate::widgets::widget_param_update::set_bool;
use crate::widgets::widget_param_update::set_halign;
use crate::widgets::widget_param_update::set_height;
use crate::widgets::widget_param_update::set_height_fill;
use crate::widgets::widget_param_update::set_iced_color_from_rgba;
use crate::widgets::widget_param_update::set_opt_f32;
use crate::widgets::widget_param_update::set_opt_iced_color;
use crate::widgets::widget_param_update::set_opt_text_shaping;
use crate::widgets::widget_param_update::set_string;
use crate::widgets::widget_param_update::set_valign;
use crate::widgets::widget_param_update::set_width;
use crate::widgets::widget_param_update::set_width_fill;

use iced::Color;
use iced::{Length, Element, Point};
use iced::widget::text::Style;
use iced::widget::{MouseArea, Text};
use iced::mouse::Interaction;

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct IpgSelectableText {
    pub id: usize,
    pub parent_id: String,
    pub content: String,
    pub width: Length,
    pub height: Length,
    pub center: Option<bool>,
    pub align_x: Option<IpgAlignmentX>,
    pub align_y: Option<IpgAlignmentY>,
    pub line_height: Option<f32>,
    pub size: Option<f32>,
    pub show: bool,
    pub font_id: Option<usize>,
    pub shaping: Option<IpgShaping>,
    pub wrapping: Option<IpgWrapping>,
    pub text_color: Option<Color>,
}

#[derive(Debug, Clone)]
pub enum SLTXTMessage {
    OnPress,
    OnRelease,
    OnRightPress,
    OnRightRelease,
    OnMiddlePress,
    OnMiddleRelease,
    OnMove(Point),
    OnEnter,
    OnExit,
}


pub fn construct_selectable_text<'a>(
    sl_text: &'a IpgSelectableText,
    font_opt: Option<&'a IpgWidgets>) 
    -> Option<Element<'a, app::Message>> {

    if !sl_text.show {
        return None
    }
    
    let txt: Text<> = Text::new(sl_text.content.clone()
                        )
                        .style(move|_|{
                            let mut style = Style::default();
                            style.color = sl_text.text_color;
                            style
                            }
                        );
    let txt = 
        if let Some(sz) = sl_text.size {
            txt.size(sz)
        } else { txt };

    let txt = 
        if let Some(lh) = sl_text.line_height {
            txt.line_height(lh)
        } else { txt };

    let txt = 
        if sl_text.center == Some(true) {
            let (h, v) = h_v_centered();
            txt.align_x(h).align_y(v)
        } else { txt };

    let txt = 
        if let Some(align) = &sl_text.align_x {
            txt.align_x(align.to_iced())
        } else { txt };

    let txt = 
        if let Some(align) = &sl_text.align_y {
            txt.align_y(align.to_iced())
        } else { txt };
    
    let txt = 
        if let Some(wd) = font_opt {
            match wd {
                IpgWidgets::IpgFont(font) => {
                    txt.font(font.to_iced())
                },
                _ => txt
            }
        } else { txt };

    let txt = 
        if let Some(sh) = &sl_text.shaping {
            txt.shaping(sh.to_iced())
        } else { txt };

    let txt = 
        if let Some(wr) = &sl_text.wrapping {
            txt.wrapping(wr.to_iced())
        } else { txt };

    
    let ma: Element<'_, SLTXTMessage> = 
                MouseArea::new(txt)
                    .on_press(SLTXTMessage::OnPress)
                    .on_release(SLTXTMessage::OnRelease)
                    .on_right_press(SLTXTMessage::OnRightPress)
                    .on_right_release(SLTXTMessage::OnRightRelease)
                    .on_middle_press(SLTXTMessage::OnMiddlePress)
                    .on_middle_release(SLTXTMessage::OnMiddleRelease)
                    .on_move(SLTXTMessage::OnMove)
                    .on_enter(SLTXTMessage::OnEnter)
                    .on_exit(SLTXTMessage::OnExit)
                    .interaction(Interaction::Pointer)
                    .into();

    Some(ma.map(move |message| app::Message::SelectableText(sl_text.id, message)))

}

pub fn selectable_text_callback(id: usize, message: SLTXTMessage) {

    match message {
        SLTXTMessage::OnPress => {
            process_callback(id, "on_press".to_string(), None);
        },
        SLTXTMessage::OnRelease => {
            process_callback(id, "on_release".to_string(), None);
        },
        SLTXTMessage::OnRightPress => {
            process_callback(id, "on_right_press".to_string(), None);
        },
        SLTXTMessage::OnRightRelease => {
            process_callback(id, "on_right_release".to_string(), None);
        },
        SLTXTMessage::OnMiddlePress => {
            process_callback(id, "on_middle_press".to_string(), None);
        },
        SLTXTMessage::OnMiddleRelease => {
            process_callback(id, "on_middle_release".to_string(), None);
        },
        SLTXTMessage::OnEnter => {
            process_callback(id, "on_enter".to_string(), None);
        },
        SLTXTMessage::OnMove(point) => {
            let points: Option<(String, f32, String, f32)> = Some(
                ("x".to_string(), point.x,
                "y".to_string(), point.y));
            
            process_callback(id, "on_move".to_string(), points);
        },
        SLTXTMessage::OnExit => {
            process_callback(id, "on_exit".to_string(), None);
        },
    }
}


fn process_callback(
    id: usize, 
    event_name: String, 
    points_opt: Option<(String, f32, String, f32)>) 
{
    let ud1 = access_user_data1();
    let ud_opt = ud1.user_data.get(&id);

    let app_cbs = access_callbacks();
    let callback = match app_cbs.callbacks.get(&(id, event_name)) {
        Some(cb) => cb,
        None => return,
    };

    let cb = Python::attach(|py| callback.clone_ref(py));
    drop(app_cbs);

    // Execute the callback with user data from ud1
    if let Some(user_data) = ud_opt {
        Python::attach(|py| {
            let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone(), user_data)),
                None => cb.call1(py, (id, user_data)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("SelectableText callback error with user_data from ud1: {err}")
            }
        });
        drop(ud1); // Drop ud1 after processing
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Execute the callback with user data from ud2
    // let ud2 = access_user_data2();
    
    // if let Some(user_data) = ud2.user_data.get(&id) {
    //     Python::attach(|py| {
    //         let res = match points_opt {
    //             Some(ref points) => cb.call1(py, (id, points.clone(), user_data)),
    //             None => cb.call1(py, (id, user_data)),
    //         };

    //         match res {
    //             Ok(_) => (),
    //             Err(err) => panic!("SelectableText callback error with user_data from ud2: {err}")
    //         }
    //     });
    //     drop(ud2); // Drop ud2 after processing
    //     return;
    // }
    // drop(ud2); // Drop ud2 if no user data is found

    // Execute the callback without user data
    Python::attach(|py| {
        let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone())),
                None => cb.call1(py, (id,)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("SelectableText callback error without user_data: {err}")
            }
    });

}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgSelectableTextParam {
    AlignX,
    AlignY,
    Content,
    Height,
    HeightFill,
    LineHeight,
    Shaping,
    Show,
    Size,
    TextColor, 
    TextRgba,
    Width,
    WidthFill,
    Wrapping,
}


// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgSelectableText {
    type Param = IpgSelectableTextParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgSelectableTextParam::AlignX => set_halign(&mut self.align_x, value, name),
            IpgSelectableTextParam::AlignY => set_valign(&mut self.align_y, value, name),
            IpgSelectableTextParam::Content => set_string(&mut self.content, value, name),
            IpgSelectableTextParam::Height => set_height(&mut self.height, value, name),
            IpgSelectableTextParam::HeightFill => set_height_fill(&mut self.height, value, name),
            IpgSelectableTextParam::LineHeight => set_opt_f32(&mut self.line_height, value, name),
            IpgSelectableTextParam::Shaping => set_opt_text_shaping(&mut self.shaping, value, name),
            IpgSelectableTextParam::Show => set_bool(&mut self.show, value, name),
            IpgSelectableTextParam::Size => set_opt_f32(&mut self.size, value, name),
            IpgSelectableTextParam::TextColor  => set_opt_iced_color(&mut self.text_color, value, name),
            IpgSelectableTextParam::TextRgba => set_iced_color_from_rgba(&mut self.text_color, value, name),
            IpgSelectableTextParam::Width => set_width(&mut self.width, value, name),
            IpgSelectableTextParam::WidthFill => set_width_fill(&mut self.width, value, name),
            IpgSelectableTextParam::Wrapping => self.wrapping = IpgWrapping::extract(value),
        }
    }
}
