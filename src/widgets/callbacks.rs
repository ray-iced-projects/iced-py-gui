//! callbacks
#![allow(dead_code)]
use crate::py_api::helpers::format_date;
use crate::{IpgState, py_api::helpers::MONTH_NAMES};
use crate::state::{access_callbacks, access_user_data2, IpgWidgets, USERDATA1};

use iced::Point;

use pyo3::{Py, PyAny, Python};
use pyo3::conversion::IntoPyObject;

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;


/// Invoke a widget callback with no additional arguments (like button's on_press).
/// 
/// The Python callback receives: `(id,)` or `(id, user_data)` if user_data is set.
pub fn invoke_callback(id: usize, event_name: &str, widget_name: &str) {
    let app_cbs = access_callbacks();
    
    let callback = match app_cbs.get(id, event_name) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };
    
    drop(app_cbs);
    
    let user_data_opt = get_user_data(id);

    Python::attach(|py| {
        let result = if let Some(user_data) = user_data_opt {
            callback.call1(py, (id, user_data))
        } else {
            callback.call1(py, (id,))
        };
        
        if let Err(err) = result {
            panic!("{widget_name} callback error: {err}");
        }
    });
}

/// Invoke a widget callback with additional widget-specific arguments.
/// 
/// The Python callback receives: `(id, args...)` or `(id, args..., user_data)` if user_data is set.
/// 
/// # Examples
/// - Checkbox: `invoke_callback_with_args(id, "on_toggle", "Checkbox", is_checked)`
/// - Slider: `invoke_callback_with_args(id, "on_change", "Slider", value)`
/// - Radio: `invoke_callback_with_args(id, "on_select", "Radio", (index, label))`
pub fn invoke_callback_with_args<A>(id: usize, event_name: &str, widget_name: &str, args: A)
where
    A: for<'py> IntoPyObject<'py> + Clone + Send,
{
    let app_cbs = access_callbacks();
    
    let callback = match app_cbs.get(id, event_name) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };
    
    drop(app_cbs);
    
    let user_data_opt = get_user_data(id);

    Python::attach(|py| {
        let result = if let Some(user_data) = user_data_opt {
            callback.call1(py, (id, args.clone(), user_data))
        } else {
            callback.call1(py, (id, args))
        };
        
        if let Err(err) = result {
            panic!("{widget_name} callback error: {err}");
        }
    });
}

/// Get user data for a widget, trying USERDATA1 first with fallback to USERDATA2.
fn get_user_data(id: usize) -> Option<PyObject> {
    let lock1 = USERDATA1.try_lock();
    if let Ok(ref ud1) = lock1 {
        let opt = ud1.get(id).map(|ud| Python::attach(|py| ud.clone_ref(py)));
        drop(lock1);
        opt
    } else {
        let ud2 = access_user_data2();
        let opt = ud2.get(id).map(|ud| Python::attach(|py| ud.clone_ref(py)));
        drop(ud2);
        opt
    }
}


#[derive(Default, Debug)]
pub struct WidgetCallbackIn {
    pub id: usize,
    // pub choice: Option<Choice>,
    // pub choice_index: Option<usize>,
    pub color: Option<Vec<f64>>,
    pub counter: Option<u64>,
    pub index: Option<usize>,
    pub index_table: Option<(usize, usize)>,
    pub increment_value: Option<i8>,
    pub is_submitted: Option<bool>,
    pub on_toggle: Option<bool>,
    pub is_checked: Option<bool>,
    pub point: Option<Point>,
    pub selected: Option<String>,
    pub selected_index: Option<usize>,
    pub selected_day: Option<usize>,
    pub selected_date: Option<String>,
    pub selected_month: Option<String>,
    pub selected_year: Option<i32>,
    pub started: Option<bool>,
    pub ticking: Option<bool>,
    pub date_format: Option<String>,
    pub show: Option<bool>,
    pub submit_str: Option<String>,
    pub value_f64: Option<f64>,
    pub value_f32: Option<f32>,
    pub value_str: Option<String>,
    pub value_bool: Option<bool>,
    pub value_usize: Option<usize>,
    pub on_tick_count: f32,
}

impl WidgetCallbackIn{}

#[derive(Default, Debug)]
pub struct WidgetCallbackOut {
    pub id: usize,
    pub color: Option<Vec<f64>>,
    pub duration: Option<u64>,
    pub counter: Option<u64>,
    pub event_name: String,
    pub is_checked: Option<bool>,
    pub index: Option<usize>,
    pub index_table: Option<(usize, usize)>,
    pub bar_index: Option<usize>,
    pub menu_index: Option<usize>,
    pub on_toggle: Option<bool>,
    pub on_modal_open: Option<bool>,
    pub points: Option<Vec<(String, f32)>>,
    pub scroll_pos: Vec<(String, f32)>,
    pub selected_index: Option<usize>,
    pub selected_label: Option<String>,
    pub selected_date: Option<String>,
    pub user_data: Option<PyObject>,
    pub button_user_data: Option<PyObject>,
    pub checkbox_user_data: Option<PyObject>,
    pub toggler_user_data: Option<PyObject>,
    pub scroller_user_data: Option<PyObject>,
    pub scroller_ids: Option<(Option<iced::widget::Id>, Option<iced::widget::Id>, Option<iced::widget::Id>)>,
    pub value_usize: Option<usize>,
    pub value_bool: Option<bool>,
    pub value_f64: Option<f64>,
    pub value_f32: Option<f32>,
    pub value_str: Option<String>,
    pub vec_f32: Vec<f32>,
}

impl WidgetCallbackOut{}

pub fn set_or_get_widget_callback_data(state: &mut IpgState, wci: WidgetCallbackIn) -> WidgetCallbackOut                                     
{
    let widget_opt = state.widgets.get_mut(&wci.id);

    if widget_opt.is_some() {
        match widget_opt.unwrap() {
            // IpgWidgets::IpgCard(crd) => {
            //     let is_open = match wci.value_bool {
            //         Some(open) => open,
            //         None => panic!("Card is_open value not found"),
            //     };
            //     crd.is_open = is_open;
            //     return WidgetCallbackOut::default();
            // },
            // IpgWidgets::IpgCardStyle(_) => {
            //     return WidgetCallbackOut::default();
            // },
            IpgWidgets::IpgCheckBox(cbox) => {
                cbox.is_checked = match wci.on_toggle {
                    Some(data) => data,
                    None => panic!("Checkbox is_checked not found")
                };
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgCheckboxStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgColorPicker(cp) => {
                cp.show = match wci.value_bool {
                    Some(s) => s,
                    None => panic!("The open value for color_picker could not be found"),
                };

                if wci.color.is_some() {
                    let color = match wci.color {
                        Some(c) => c,
                        None => panic!("The color value for color_picker could not be found"),
                    };
                    cp.color = iced::Color::from_rgba(color[0] as f32, color[1] as f32, 
                                            color[2] as f32, color[3] as f32);
                }
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgDividerHorizontal(div) => {
                let mut wco = WidgetCallbackOut::default();
                if wci.value_str == Some("on_change".to_string()) {
                    div.index_in_use = wci.value_usize.unwrap();
                    div.value_in_use = wci.value_f32.unwrap();
                    return wco;
                }
                if wci.value_str == Some("on_release".to_string()) {
                    wco.value_usize = Some(div.index_in_use);
                    wco.value_f32 = Some(div.value_in_use);
                    return wco
                }
            },
            IpgWidgets::IpgDividerVertical(div) => {
                let mut wco = WidgetCallbackOut::default();
                if wci.value_str == Some("on_change".to_string()) {
                    div.index_in_use = wci.value_usize.unwrap();
                    div.value_in_use = wci.value_f32.unwrap();
                    return wco;
                }
                if wci.value_str == Some("on_release".to_string()) {  
                    wco.value_usize = Some(div.index_in_use);
                    wco.value_f32 = Some(div.value_in_use);
                    return wco
                }
            },
            IpgWidgets::IpgDatePicker(dp) => {
                if wci.selected_day.is_some() {
                    dp.selected_day = wci.selected_day.unwrap();
                }
                // month index
                if wci.index.is_some() {
                    let increment = wci.increment_value.unwrap();
                    let index = wci.index.unwrap();
                    if index == 12 && increment == 1 {
                        dp.selected_month_index = 1
                    } else if index == 1 && increment == -1 {
                        dp.selected_month_index = 12;
                    } else if increment == -1 {
                        dp.selected_month_index -= 1;
                    } else {
                        dp.selected_month_index += 1;
                    }
            
                    dp.selected_month = MONTH_NAMES[dp.selected_month_index].to_string();
                }
        
                if wci.selected_year.is_some() {
                    let yr = wci.selected_year.unwrap();
                    dp.selected_year += yr;             
                }

                if wci.date_format.is_some() {
                    dp.selected_format = wci.date_format.unwrap();
                }
                dp.selected_date = 
                    format_date(
                        dp.selected_format.clone(), 
                        dp.selected_year, 
                        dp.selected_month_index, 
                        dp.selected_day
                        );
        
                if wci.is_submitted.is_some() {
                    dp.is_submitted = wci.is_submitted.unwrap();
                }
                if wci.show.is_some() {
                    dp.show_calendar = wci.show;
                };
                return WidgetCallbackOut{
                    selected_date: Some(dp.selected_date.clone()),
                    ..Default::default()
                }
            },
            IpgWidgets::IpgFont(_) => {
                return WidgetCallbackOut::default();
            }
            // IpgWidgets::IpgSlider(slider) => {
            //     if wci.value_f64.is_some() {
            //         slider.value = match wci.value_f64 {
            //             Some(v) => v as f32,
            //             None => panic!("Slider submit value could not be found"),
            //         };
            //     }
            //     return WidgetCallbackOut{
            //         value_f32: Some(slider.value),
            //         ..Default::default()
            //     }
            // },
            // // IpgWidgets::IpgRichText(_) => {
            // //     return WidgetCallbackOut::default()
            // // },
            // IpgWidgets::IpgTextInput(ti) => {
            //     ti.value = wci.value_str.unwrap();
            //     return WidgetCallbackOut{
            //         value_str: Some(ti.value.clone()),
            //         ..Default::default()
            //     }
            // },
            // IpgWidgets::IpgTimer(tim) => {
            //     tim.counter += 1;
            //     // value_str is set when a tick occurs
            //     // so no value_bool present
            //     if wci.value_str.is_none() {
            //         tim.started = wci.value_bool.unwrap();
            //     }
            //     return WidgetCallbackOut{
            //         counter: Some(tim.counter),
            //         duration: Some(tim.duration_ms),
            //         value_bool: Some(tim.started),
            //         ..Default::default()
            //     }
            // },
            // IpgWidgets::IpgCanvasTimer(ctim) => {
            //     ctim.counter += 1;
            //     // value_str is set when a tick occurs
            //     // so no value_bool present
            //     if wci.value_str.is_none() {
            //         ctim.started = wci.value_bool.unwrap();
            //     }
            //     return WidgetCallbackOut{
            //         counter: Some(ctim.counter),
            //         duration: Some(ctim.duration_ms),
            //         value_bool: Some(ctim.started),
            //         ..Default::default()
            //     }
            // },
            // IpgWidgets::IpgToggler(tog) => {
            //     if let Some(tg) = wci.on_toggle { tog.is_toggled = tg }
            //     return WidgetCallbackOut::default();
            // },
            _ => {
                return WidgetCallbackOut::default();
            },
            
            }
    } else {

        let container_opt = state.containers.get_mut(&wci.id);
        if container_opt.is_some() {
            match container_opt.unwrap() {
                // IpgContainers::IpgTable(tbl) => {
                //     let mut wco = WidgetCallbackOut::default();
                //     if wci.value_str == Some("sync".to_string()) {
                //         let mut ids = (Some(tbl.body_scroller_id.clone()), None, None);
                //         if tbl.header_enabled {
                //             ids.1 = Some(tbl.header_scroller_id.clone());
                //         } 
                //         if tbl.custom_footer_rows > 0 {
                //             ids.2 = Some(tbl.footer_scroller_id.clone());
                //         }
                //         wco.scroller_ids = Some(ids);
                //         return wco;
                //     }
                //     // resizing
                //     let index = wci.value_usize.unwrap();
                   
                //     let value = if wci.value_f32 < tbl.min_column_width {
                //         tbl.min_column_width.unwrap()
                //     } else {
                //         wci.value_f32.unwrap()
                //     };

                //     if tbl.table_width_fixed && index == tbl.column_widths.len()-1 {
                //         // don't change width just return vec
                //         wco.vec_f32 = tbl.column_widths.clone();
                //         return wco;
                //     }

                //     // get diff
                //     let diff = tbl.column_widths[index] - value;

                //     // change the widths porportionally if enabled
                //     if !tbl.table_width_fixed && index == tbl.column_widths.len()-1 {
                //         if tbl.column_proportional_resize {
                //             let table_width: f32 = tbl.column_widths.iter().sum();
                //             let percent = 1.0 - diff/table_width;
                            
                //             let mut new_widths = vec![];
                //             for width in tbl.column_widths.iter() {
                //                 new_widths.push(width * percent)
                //             }
                            
                //             tbl.column_widths = new_widths.clone();
                //             wco.vec_f32 = new_widths;
                            
                //             return wco;
                //         }
                //     }
                    
                //     // # Adjust the left side
                //     tbl.column_widths[index] = value;
                    
                //     // # Adjust the right side unless at end
                //     if index < tbl.column_widths.len()-1 {
                //             tbl.column_widths[index+1] += diff
                //     }
                //     wco.vec_f32 = tbl.column_widths.clone();
                //     return wco;
                
                // },
                _ => panic!("Callback: container not found")
            }
        }
    }
    panic!("get_set_wci: id {} not found", wci.id)
    
}


pub fn container_callback_data(state: &mut IpgState, wci: WidgetCallbackIn) -> WidgetCallbackOut {

    let container_type_opt = state.containers.get_mut(&wci.id);

    let container_type = match container_type_opt {
        Some(cont) => cont,
        None => panic!("Container with id {} could not be found", wci.id),
    };
    
    match container_type {
        // IpgContainers::IpgCanvas(_) => {
        //     WidgetCallbackOut::default()
        // },
        // IpgContainers::IpgMouseArea(_) => {
        //     WidgetCallbackOut::default()
        // },
        // IpgContainers::IpgTable(_) => {
        //     WidgetCallbackOut::default()
        // }
        // IpgContainers::IpgScrollable(_) => {
        //     WidgetCallbackOut::default()
        // }
        _ => {
            WidgetCallbackOut::default()
        }
    }
        
}
