//! Core state management for IcedPyGui
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use iced::{window, Color, Length, Point, Theme};
use iced::widget::scrollable;
use once_cell::sync::Lazy;

// Re-export all message types from ipg-types
pub use ipg_types::*;

#[derive(Debug)]
pub struct State {
    pub ids_ipd_ids: Lazy<HashMap<usize, Vec<IpgIds>>>,  // <window_id=usize, Vec<IpgIds=structure>>
    pub last_id: usize,
    pub gen_ids: Vec<usize>,

    pub containers: Lazy<HashMap<usize, IpgContainers>>,
    pub container_ids: Lazy<HashMap<usize, Vec<usize>>>,  // <window_id=usize, vec<container_id=usize>>
    pub container_str_ids: Lazy<HashMap<String, usize>>, // get container usize id based on container string
    pub container_wnd_str_ids: Lazy<HashMap<String, String>>, // get window string id based on container string id
    pub container_window_usize_ids: Lazy<HashMap<usize, usize>>, //get window usize id based on container usize id
    
    pub widgets: Lazy<HashMap<usize, IpgWidgets>>,
    pub widget_container_ids: Lazy<HashMap<usize, String>>, //widget_id=usize, container_id=String
    
    pub windows: Vec<IpgWindow>,
    pub windows_iced_ipg_ids: Lazy<HashMap<window::Id, usize>>, // <iced id, ipg id>
    pub windows_str_ids: Lazy<HashMap<String, usize>>,  // <ipg_id=str, ipg id>
    pub window_debug: Lazy<HashMap<window::Id, (usize, bool)>>, // (wid, debug)
    pub window_theme: Lazy<HashMap<window::Id, (usize, Theme)>>, // (wid, window Theme)
    pub window_mode: Lazy<HashMap<window::Id, (usize, window::Mode)>>,

    pub events: Vec<IpgEvents>,
    pub keyboard_event_id_enabled: (usize, bool),
    pub mouse_event_id_enabled: (usize, bool),
    pub timer_event_id_enabled: (usize, bool),
    pub canvas_timer_event_id_enabled: (usize, bool),
    pub window_event_id_enabled: (usize, bool),
    pub touch_event_id_enabled: (usize, bool),
    pub timer_duration: u64,
    pub canvas_timer_duration: u64,

}

pub static STATE: Mutex<State> = Mutex::new(
    State {
        ids_ipd_ids: Lazy::new(||HashMap::new()),
        last_id: 0,
        gen_ids: vec![],

        containers: Lazy::new(||HashMap::new()),
        container_ids: Lazy::new(||HashMap::new()),
        container_str_ids: Lazy::new(||HashMap::new()),
        container_wnd_str_ids: Lazy::new(||HashMap::new()),
        container_window_usize_ids: Lazy::new(||HashMap::new()),

        widgets: Lazy::new(||HashMap::new()),
        widget_container_ids: Lazy::new(||HashMap::new()),

        windows: vec![],
        windows_iced_ipg_ids: Lazy::new(||HashMap::new()),
        windows_str_ids: Lazy::new(||HashMap::new()),
        window_debug: Lazy::new(||HashMap::new()),
        window_theme: Lazy::new(||HashMap::new()),
        window_mode: Lazy::new(||HashMap::new()),
        
        events: vec![],
        keyboard_event_id_enabled: (0, false),
        mouse_event_id_enabled: (0, false), 
        timer_event_id_enabled: (0, false),
        canvas_timer_event_id_enabled: (0, false),
        window_event_id_enabled: (0, false),
        touch_event_id_enabled: (0, false),
        timer_duration: 0,
        canvas_timer_duration: 0,

    }
);

pub fn access_state() -> MutexGuard<'static, State> {
    STATE.lock().unwrap()
}

#[derive(Debug)]
pub struct CanvasState {
    pub canvas_ids_str: Lazy<HashMap<String, usize>>,
    pub curves: Lazy<HashMap<usize, IpgWidget>>,
    pub text_curves: Lazy<HashMap<usize, IpgWidget>>,
    pub image_curves: Lazy<HashMap<usize, IpgWidget>>,
    pub width: Length,
    pub height: Length,
    pub background: Option<Color>,
    pub border_color: Option<Color>,
    pub border_width: Option<f32>,
}

pub static CANVAS_STATE: Mutex<CanvasState> = Mutex::new(
    CanvasState {
        canvas_ids_str: Lazy::new(||HashMap::new()),
        curves: Lazy::new(||HashMap::new()),
        text_curves: Lazy::new(||HashMap::new()),
        image_curves: Lazy::new(||HashMap::new()),
        width: Length::Fill,
        height: Length::Fill,
        background: None,
        border_color: None,
        border_width: None,
        },
);

pub fn access_canvas_state() -> MutexGuard<'static, CanvasState> {
    CANVAS_STATE.lock().unwrap()
}

#[derive(Default, Debug, Clone)]
pub struct IpgState {
    pub ids: HashMap<usize, Vec<IpgIds>>,  // <window_id=usize, Vec<IpgIds=structure>>
    pub last_id: usize,

    pub containers: HashMap<usize, IpgContainers>,
    pub container_ids: HashMap<usize, Vec<usize>>,  // <window_id=usize, vec<container_id=usize>>
    pub container_wnd_str_ids: HashMap<String, String>, // get window string id based on container string id
    pub container_str_ids: HashMap<String, usize>, // get container usize id based on container string
    pub container_window_usize_ids: HashMap<usize, usize>, //get window usize id based on container usize id
    
    pub widgets: HashMap<usize, IpgWidgets>,
    pub widget_container_ids: HashMap<usize, String>, //widget_id=usize, container_id=String
    
    pub windows_iced_ipg_ids: HashMap<window::Id, usize>, // <iced id, ipg id>
    pub windows_str_ids: HashMap<String, usize>,  // <ipg_id=str, ipg id>
    pub windows: Vec<IpgWindow>,
    pub window_debug: HashMap<window::Id, (usize, bool)>, // (wid, debug)
    pub window_theme: HashMap<window::Id, (usize, Theme)>, // (wid, window Theme)
    pub window_mode: HashMap<window::Id, (usize, window::Mode)>,
    pub windows_opened: Vec<window::Id>,
    pub windows_hidden: Vec<window::Id>,

    pub container_style: HashMap<String, IpgContainerStyle>,
    pub button_style: HashMap<String, IpgButtonStyle>,
    pub checkbox_style: HashMap<String, IpgCheckboxStyle>,
    pub color_picker_style: HashMap<String, IpgColorPickerStyle>,
    // pub menu_bar_style: HashMap<String, IpgMenuBarStyle>,
    // pub menu_style: HashMap<String, IpgMenuStyle>,
    // pub menu_separator_style: HashMap<String, IpgMenuSeparatorStyle>,
    pub opaque_style: HashMap<String, IpgOpaqueStyle>,
    pub pick_list_style: HashMap<String, IpgPickListStyle>,
    pub progress_bar_style: HashMap<String, IpgProgressBarStyle>,
    pub radio_style:  HashMap<String, IpgRadioStyle>,
    pub rule_style:  HashMap<String, IpgRuleStyle>,
    pub slider_style:  HashMap<String, IpgSliderStyle>,
    pub text_input_style: HashMap<String, IpgTextInputStyle>,
    pub toggler_style: HashMap<String, IpgTogglerStyle>,
    pub scrollable_style: HashMap<String, IpgScrollableStyle>,

    pub keyboard_event_id_enabled: (usize, bool),
    pub mouse_event_id_enabled: (usize, bool),
    pub timer_event_id_enabled: (usize, bool),
    pub canvas_timer_event_id_enabled: (usize, bool),
    pub window_event_id_enabled: (usize, bool),
    pub touch_event_id_enabled: (usize, bool),
    pub timer_duration: u64,
    pub canvas_timer_duration: u64,

    pub mode: Vec<(usize, window::Mode)>,
    pub decorations: Vec<usize>,
    pub resize: Vec<(usize, f32, f32)>,
    pub position: Vec<(usize, f32, f32)>,
    pub level: Vec<(usize, window::Level)>,
}

impl IpgState {
    pub fn new() -> Self {
        IpgState {
            ids: HashMap::new(),
            last_id: 0,

            containers: HashMap::new(),
            container_ids: HashMap::new(),
            container_wnd_str_ids: HashMap::new(),
            container_str_ids: HashMap::new(),
            container_window_usize_ids: HashMap::new(),

            widgets: HashMap::new(),
            widget_container_ids: HashMap::new(),

            windows_iced_ipg_ids: HashMap::new(),
            windows_str_ids: HashMap::new(),
            windows: vec![],
            window_debug: HashMap::new(),
            window_theme: HashMap::new(),
            window_mode: HashMap::new(),
            windows_opened: vec![],
            windows_hidden: vec![],

            container_style: HashMap::new(),
            button_style: HashMap::new(),
            checkbox_style: HashMap::new(),
            color_picker_style: HashMap::new(),
            // menu_bar_style: HashMap::new(),
            // menu_style: HashMap::new(),
            // menu_separator_style: HashMap::new(),
            opaque_style: HashMap::new(),
            pick_list_style: HashMap::new(),
            progress_bar_style: HashMap::new(),
            radio_style: HashMap::new(),
            rule_style: HashMap::new(),
            slider_style: HashMap::new(),
            text_input_style: HashMap::new(),
            toggler_style: HashMap::new(),
            scrollable_style: HashMap::new(),

            keyboard_event_id_enabled: (0, false),
            mouse_event_id_enabled: (0, false), 
            timer_event_id_enabled: (0, false),
            canvas_timer_event_id_enabled: (0, false),
            window_event_id_enabled: (0, false),
            touch_event_id_enabled: (0, false),
            timer_duration: 0,
            canvas_timer_duration: 0,

            mode: vec![],
            decorations: vec![],
            resize: vec![],
            position: vec![],
            level: vec![],
        }
    }
}

pub fn add_windows(state: &mut IpgState) -> Vec<Task<Message>> {

    let mut modes: Vec<(usize, window::Mode)> = vec![];

    let mut spawn_window: Vec<Task<Message>> = vec![];

    for i in 0..state.windows.len() {
        let visible = match state.windows[i].mode {
            IpgWindowMode::Windowed => {
                modes.push((state.windows[i].id, window::Mode::Windowed));
                true
            },
            IpgWindowMode::FullScreen => {
                modes.push((state.windows[i].id, window::Mode::Fullscreen));
                true
            },
            IpgWindowMode::Closed => {
                modes.push((state.windows[i].id, window::Mode::Hidden));
                false
            },
        };
        let (iced_id, open) = window::open(window::Settings {
            size: state.windows[i].size,
            min_size: state.windows[i].min_size,
            max_size: state.windows[i].max_size,
            position: state.windows[i].position,
            visible,
            resizable: state.windows[i].resizable,
            decorations: state.windows[i].decorations,
            transparent: state.windows[i].transparent,
            level: get_level(&state.windows[i].level),
            exit_on_close_request: state.windows[i].exit_on_close_request,
            ..Default::default()
        });
        let id = state.windows[i].id;
        let debug = state.windows[i].debug;
        let theme = state.windows[i].theme.clone();
        let mode = state.windows[i].mode.clone();

        state.window_debug.insert(iced_id, (id, debug));
        state.window_theme.insert(iced_id, (id, theme));
        state.window_mode.insert(iced_id, (id, get_iced_mode(&mode)));
        state.windows_opened.push(iced_id);
        if !visible {
            state.windows_hidden.push(iced_id);
        }

        let ipg_id = state.windows[i].id;
        state.windows_iced_ipg_ids.insert(iced_id, ipg_id);
        let size = state.windows[i].size;
        spawn_window.push(open.map(move|_|Message::WindowOpened(iced_id, None, size)));
        
    }

    spawn_window

}

pub fn check_for_dup_container_ids(id: usize, container_id: Option<String>) {

    let state = access_state();
    
    let parents = match state.ids_ipd_ids.get(&id) {
        Some(ids) => ids,
        None => panic!("Ids in check_for_dup_container_ids not found")
    };

    for parent in parents {
        if container_id == parent.container_id {
            panic!("Container Id {:?} is not unique", container_id);
        }
    }
    
    drop(state);
}

pub fn find_key_for_value(ids: HashMap<window::Id, usize>, value: usize) -> window::Id {
    let state = access_state();
    let map = &ids;
    let id = map.iter()
        .find_map(|(key, &val)| if val == value { Some(key) } else { None });
    
    match id {
        Some(id) => {
            let iced_id = *id;
            drop(state);
            iced_id
        },
        None => panic!("Unable to find the iced id via the ipg id {}.", value)
    }
}



#[derive(Default, Debug)]
pub struct WidgetCallbackIn {
    pub id: usize,
    pub choice: Option<Choice>,
    pub choice_index: Option<usize>,
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
    pub scroller_ids: Option<(Option<scrollable::Id>, Option<scrollable::Id>, Option<scrollable::Id>)>,
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
            IpgWidgets::IpgButton(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgButtonStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgCard(crd) => {
                let is_open = match wci.value_bool {
                    Some(open) => open,
                    None => panic!("Card is_open value not found"),
                };
                crd.is_open = is_open;
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgCardStyle(_) => {
                return WidgetCallbackOut::default();
            },
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
                    cp.color = Color::from_rgba(color[0] as f32, color[1] as f32, 
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
            IpgWidgets::IpgDividerStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgColorPickerStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgContainerStyle(_) => {
                return WidgetCallbackOut::default();
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
                    dp.show_calendar = wci.show.unwrap();
                };
                return WidgetCallbackOut{
                    selected_date: Some(dp.selected_date.clone()),
                    ..Default::default()
                }
            },
            IpgWidgets::IpgImage(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgOpaqueStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgPickList(pl) => {
                pl.selected = wci.value_str;
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgPickListStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgProgressBar(_) => {
                return WidgetCallbackOut::default()
            },
            IpgWidgets::IpgProgressBarStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgRadio(_) => {
                return WidgetCallbackOut::default()
            },
            IpgWidgets::IpgRadioStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgRule(_) => {
                return WidgetCallbackOut::default()
            },
            IpgWidgets::IpgRuleStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgScrollableStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgSelectableText(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgSeparator(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgSeparatorStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgSlider(slider) => {
                if wci.value_f64.is_some() {
                    slider.value = match wci.value_f64 {
                        Some(v) => v as f32,
                        None => panic!("Slider submit value could not be found"),
                    };
                }
                return WidgetCallbackOut{
                    value_f32: Some(slider.value),
                    ..Default::default()
                }
            },
            IpgWidgets::IpgSliderStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgSpace(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgSvg(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgTableStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgText(_) => {
                return WidgetCallbackOut::default()
            },
            // IpgWidgets::IpgRichText(_) => {
            //     return WidgetCallbackOut::default()
            // },
            IpgWidgets::IpgTextInput(ti) => {
                ti.value = wci.value_str.unwrap();
                return WidgetCallbackOut{
                    value_str: Some(ti.value.clone()),
                    ..Default::default()
                }
            },
            IpgWidgets::IpgTextInputStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgTimer(tim) => {
                tim.counter += 1;
                // value_str is set when a tick occurs
                // so no value_bool present
                if wci.value_str.is_none() {
                    tim.started = wci.value_bool.unwrap();
                }
                return WidgetCallbackOut{
                    counter: Some(tim.counter),
                    duration: Some(tim.duration_ms),
                    value_bool: Some(tim.started),
                    ..Default::default()
                }
            },
            IpgWidgets::IpgTimerStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgCanvasTimer(ctim) => {
                ctim.counter += 1;
                // value_str is set when a tick occurs
                // so no value_bool present
                if wci.value_str.is_none() {
                    ctim.started = wci.value_bool.unwrap();
                }
                return WidgetCallbackOut{
                    counter: Some(ctim.counter),
                    duration: Some(ctim.duration_ms),
                    value_bool: Some(ctim.started),
                    ..Default::default()
                }
            },
            IpgWidgets::IpgCanvasTimerStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgToggler(tog) => {
                if let Some(tg) = wci.on_toggle { tog.is_toggled = tg }
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgTogglerStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgToolTipStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgMenuStyle(_) => {
                return WidgetCallbackOut::default();
            },
            IpgWidgets::IpgMenuBarStyle(_) => {
                return WidgetCallbackOut::default();
            },
            
            }
    } else {

        let container_opt = state.containers.get_mut(&wci.id);
        if container_opt.is_some() {
            match container_opt.unwrap() {
                IpgContainers::IpgTable(tbl) => {
                    let mut wco = WidgetCallbackOut::default();
                    if wci.value_str == Some("sync".to_string()) {
                        let mut ids = (Some(tbl.body_scroller_id.clone()), None, None);
                        if tbl.header_enabled {
                            ids.1 = Some(tbl.header_scroller_id.clone());
                        } 
                        if tbl.custom_footer_rows > 0 {
                            ids.2 = Some(tbl.footer_scroller_id.clone());
                        }
                        wco.scroller_ids = Some(ids);
                        return wco;
                    }
                    // resizing
                    let index = wci.value_usize.unwrap();
                   
                    let value = if wci.value_f32 < tbl.min_column_width {
                        tbl.min_column_width.unwrap()
                    } else {
                        wci.value_f32.unwrap()
                    };

                    if tbl.table_width_fixed && index == tbl.column_widths.len()-1 {
                        // don't change width just return vec
                        wco.vec_f32 = tbl.column_widths.clone();
                        return wco;
                    }

                    // get diff
                    let diff = tbl.column_widths[index] - value;

                    // change the widths porportionally if enabled
                    if !tbl.table_width_fixed && index == tbl.column_widths.len()-1 {
                        if tbl.column_proportional_resize {
                            let table_width: f32 = tbl.column_widths.iter().sum();
                            let percent = 1.0 - diff/table_width;
                            
                            let mut new_widths = vec![];
                            for width in tbl.column_widths.iter() {
                                new_widths.push(width * percent)
                            }
                            
                            tbl.column_widths = new_widths.clone();
                            wco.vec_f32 = new_widths;
                            
                            return wco;
                        }
                    }
                    
                    // # Adjust the left side
                    tbl.column_widths[index] = value;
                    
                    // # Adjust the right side unless at end
                    if index < tbl.column_widths.len()-1 {
                            tbl.column_widths[index+1] += diff
                    }
                    wco.vec_f32 = tbl.column_widths.clone();
                    return wco;
                
                },
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
        IpgContainers::IpgCanvas(_) => {
            WidgetCallbackOut::default()
        },
        IpgContainers::IpgMouseArea(_) => {
            WidgetCallbackOut::default()
        },
        IpgContainers::IpgTable(_) => {
            WidgetCallbackOut::default()
        }
        IpgContainers::IpgScrollable(_) => {
            WidgetCallbackOut::default()
        }
        _ => {
            WidgetCallbackOut::default()
        }
    }
        
}

pub fn button_callback(id: usize, message: BTNMessage) {

    match message {
        BTNMessage::OnPress => {
            process_button_callback(id, "on_press".to_string());
        }
    }
}

pub fn process_button_callback(
        id: usize, 
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
            if let Err(err) = callback.call1(py, (id, user_data)) {
                panic!("Button callback error: {err}");
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
            if let Err(err) = callback.call1(py, (id, user_data)) {
                panic!("Button callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only the id
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id,)) {
            panic!("Button callback error: {err}");
        }
    });
}

pub fn card_callback(_state: &mut IpgState, id: usize, message: CardMessage) {
    match message {
        CardMessage::OnClose => {
            let _ = 
                WidgetCallbackIn{
                    id,
                    value_bool: Some(false),
                    ..Default::default()
                };
            process_card_callback(id, "on_close".to_string());
        }
    }
}

pub fn process_card_callback(id: usize, event_name: String) {
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
            if let Err(err) = callback.call1(py, (id, user_data)) {
                panic!("Card callback error: {err}");
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
            if let Err(err) = callback.call1(py, (id, user_data)) {
                panic!("Card callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only the id
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id,)) {
            panic!("Card callback error: {err}");
        }
    });
}

pub fn checkbox_callback(state: &mut IpgState, id: usize, message: CHKMessage) {

    match message {
        CHKMessage::OnToggle(on_toggle) => {
            let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
            wci.on_toggle = Some(on_toggle);
            let _ = set_or_get_widget_callback_data(state, wci);

            process_chkbox_callback(id, on_toggle, "on_toggle".to_string());
        }
    }
}

pub fn process_chkbox_callback(
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

pub fn color_picker_callback(state: &mut IpgState, id: usize, message: ColPikMessage) {
    let mut wci: WidgetCallbackIn = WidgetCallbackIn::default();
    match message {
        ColPikMessage::OnCancel => {
            wci.id = id;
            wci.value_bool = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_color_picker_callback(id, "on_cancel".to_string(), None);
        },
        ColPikMessage::OnSubmit(color) => {
            wci.id = id;
            wci.value_bool = Some(false);
            wci.color = Some(convert_color_to_list(color));
            let _ = set_or_get_widget_callback_data(state, wci);
            process_color_picker_callback(id, "on_submit".to_string(), Some(convert_color_to_list(color)));
        },
        ColPikMessage::OnPress => {
            wci.id = id;
            wci.value_bool = Some(true);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_color_picker_callback(id, "on_press".to_string(), None);
        },
    }
}


pub fn process_color_picker_callback(id: usize, event_name: String, color: Option<Vec<f64>>) 
{
    let ud1 = access_user_data1();
    let app_cbs = access_callbacks();

    // Retrieve the callback
    let callback = match app_cbs.callbacks.get(&(id, event_name.clone())) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_cbs);

    // Check user data from ud1
    if let Some(user_data) = ud1.user_data.get(&id) {
        Python::attach(|py| {
            if event_name == "on_submit".to_string() {
                if let Err(err) = callback.call1(py, (id, color, user_data)) {
                    panic!("ColorPicker callback error: {err}");
                }
            } else {
                if let Err(err) = callback.call1(py, (id, user_data)) {
                    panic!("ColorPicker callback error: {err}");
                }
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
            if event_name == "on_submit".to_string() {
                if let Err(err) = callback.call1(py, (id, color, user_data)) {
                    panic!("ColorPicker callback error: {err}");
                }
            } else {
                if let Err(err) = callback.call1(py, (id, user_data)) {
                    panic!("ColorPicker callback error: {err}");
                }
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the 
    // callback with only the id and color except for on_pressed
    // which has only an id.
    Python::attach(|py| {
        if event_name == "on_submit".to_string() {
            if let Err(err) = callback.call1(py, (id, color)) {
                panic!("ColorPicker callback error: {err}");
            }
        } else {
            if let Err(err) = callback.call1(py, (id,)) {
                panic!("ColorPicker callback error: {err}");
            }
        }
    });
}

pub fn date_picker_callback(state: &mut IpgState, id: usize, message: DPMessage) {
    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};

    match message {
        DPMessage::ShowModal => {
            // Non callback just sending the values.
            wci.show = Some(true);
            wci.is_submitted = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::HideModal => {
            // Non callback just sending the values.
            wci.show = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::DayPressed(day) => {
            // Non callback just sending the values.
            wci.selected_day = Some(day);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::DatePickerFormat(date_format) => {
            // Non callback just sending the values.
            wci.date_format = Some(date_format);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::MonthRightPressed(index) => {
            // Non callback just sending the values.
            wci.index = Some(index);
            wci.increment_value = Some(1);
            wci.is_submitted = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::MonthLeftPressed(index) => {
            // Non callback just sending the values.
            wci.index = Some(index);
            wci.increment_value = Some(-1);
            wci.is_submitted = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::YearRightPressed => {
            // Non callback just sending the values.
            wci.selected_year = Some(1);
            wci.is_submitted = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::YearLeftPressed => {
            // Non callback just sending the values.
            wci.selected_year = Some(-1);
            wci.is_submitted = Some(false);
            let _ = set_or_get_widget_callback_data(state, wci);
        }
        DPMessage::OnSubmit => {
            wci.is_submitted = Some(true);
            let wco = set_or_get_widget_callback_data(state, wci);

            process_datepicker_callback(id, "on_submit".to_string(), wco.selected_date);
        }
    }
}


pub fn process_datepicker_callback(id: usize, event_name: String, selected_date: Option<String>) {
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
            if let Some(date) = &selected_date {
                if let Err(err) = callback.call1(py, (id, date.clone(), user_data)) {
                    panic!("DatePicker callback error: {err}");
                }
            } else {
                if let Err(err) = callback.call1(py, (id, user_data)) {
                    panic!("DatePicker callback error: {err}");
                }
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
            if let Some(date) = &selected_date {
                if let Err(err) = callback.call1(py, (id, date.clone(), user_data)) {
                    panic!("DatePicker callback error: {err}");
                }
            } else {
                if let Err(err) = callback.call1(py, (id, user_data)) {
                    panic!("DatePicker callback error: {err}");
                }
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only id and selected_date
    Python::attach(|py| {
        if let Some(date) = selected_date {
            if let Err(err) = callback.call1(py, (id, date)) {
                panic!("DatePicker callback error: {err}");
            }
        } else {
            if let Err(err) = callback.call1(py, (id,)) {
                panic!("DatePicker callback error: {err}");
            }
        }
    });
}

pub fn divider_callback(state: &mut IpgState, id: usize, message: DivMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
           
    match message {
        DivMessage::OnChange((id, index, value)) => {
            wci.value_f32 = Some(value);
            wci.value_usize = Some(index);
            wci.value_str = Some("on_change".to_string());
            let _ = set_or_get_widget_callback_data(state, wci);
            process_divider_callback(
                id, 
                "on_change".to_string(), 
                index, 
                value);
        },
        DivMessage::OnRelease => {
            // to be consistent, returning values for both
            wci.value_str = Some("on_release".to_string());
            let wco = set_or_get_widget_callback_data(state, wci);
            process_divider_callback(
                id, 
                "on_release".to_string(), 
                wco.value_usize.unwrap(), 
                wco.value_f32.unwrap());
        },
    }
}

pub fn process_divider_callback(id: usize, event_name: String, index: usize, value: f32) {
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

    // Check user data from ud2
    let ud2 = access_user_data2();
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::attach(|py| {
            let res = callback.call1(py, (id, index, value, user_data));
            if let Err(err) = res {
                panic!("Divider callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only id, index, and value
    Python::attach(|py| {
        let res = callback.call1(py, (id, index, value));
        if let Err(err) = res {
            panic!("Divider callback error: {err}");
        }
    });
}

pub fn image_callback(id: usize, message: ImageMessage) {

    match message {
        ImageMessage::OnPress => {
            process_callback(id, "on_press".to_string(), None);
        },
        ImageMessage::OnRelease => {
            process_callback(id, "on_release".to_string(), None);
        },
        ImageMessage::OnRightPress => {
            process_callback(id, "on_right_press".to_string(), None);
        },
        ImageMessage::OnRightRelease => {
            process_callback(id, "on_right_release".to_string(), None);
        },
        ImageMessage::OnMiddlePress => {
            process_callback(id, "on_middle_press".to_string(), None);
        },
        ImageMessage::OnMiddleRelease => {
            process_callback(id, "on_middle_release".to_string(), None);
        },
        ImageMessage::OnEnter => {
            process_callback(id, "on_enter".to_string(), None);
        },
        ImageMessage::OnMove(point) => {
            let points: Option<HashMap<String, f32>> = Some(HashMap::from([
                ("x".to_string(), point.x),
                ("y".to_string(), point.y)
            ]));
            
            process_image_callback(id, "on_move".to_string(), points);
        },
        ImageMessage::OnExit => {
            process_image_callback(id, "on_exit".to_string(), None);
        },
    }
}

fn process_image_callback(
    id: usize,
    event_name: String,
    points_opt: Option<HashMap<String, f32>>,
) {
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
                Err(err) => panic!("Image callback error with user_data from ud1: {err}")
            }
        });
        drop(ud1); // Drop ud1 after processing
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Execute the callback with user data from ud2
    let ud2 = access_user_data2();
    
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::attach(|py| {
            let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone(), user_data)),
                None => cb.call1(py, (id, user_data)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("Image callback error with user_data from ud2: {err}")
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // Execute the callback without user data
    Python::attach(|py| {
        let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone())),
                None => cb.call1(py, (id,)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("Image callback error without user_data: {err}")
            }
    });
}

pub fn modal_callback(state: &mut IpgState, 
                        id: usize, 
                        message: ModalMessage) {

    let wci = WidgetCallbackIn{id, ..Default::default()};

    match message {
        ModalMessage::OnOpen => {
            let mut wco = set_or_get_widget_callback_data(state, wci);
            wco.id = id;
            wco.event_name = "on_open".to_string();
            process_modal_callback(wco);
        }
    }
}

pub fn process_modal_callback(wco: WidgetCallbackOut) 
{
    let app_cbs = access_callbacks();

    let callback_present = 
        app_cbs.callbacks.get(&(wco.id, wco.event_name.clone()));

    let callback_opt = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let callback = match callback_opt {
        Some(cb) => cb,
        None => panic!("Modal callback could not be found with id {}", wco.id),
    };

    Python::attach(|py| {
            if wco.user_data.is_some() {
                let user_data = match wco.user_data {
                    Some(ud) => ud,
                    None => panic!("User Data could not be found in Modal callback"),
                };
                let res = callback.call1(py, (
                                                                    wco.id,  
                                                                    user_data
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Modal: 2 parameters (id, user_data) are required or a python error in this function. {er}"),
                }
            } else {
                let res = callback.call1(py, (
                                                                    wco.id,  
                                                                    ));
                match res {
                    Ok(_) => (),
                    Err(er) => panic!("Modal: 1 parameter (id) is required or possibly a python error in this function. {er}"),
                }
            } 
    });
    
    drop(app_cbs);    
}

pub fn mousearea_callback(_state: &mut IpgState, id: usize, event_name: String) {
    
    process_mousearea_callback(id, event_name, None);

}

pub fn mousearea_callback_point(_state: &mut IpgState, 
                                id: usize, 
                                point: Point, 
                                event_name: String,
                                ) {

    let points: Option<(String, f32, String, f32)> = Some(
                ("x".to_string(), point.x,
                "y".to_string(), point.y));

    process_mousearea_callback(id, event_name, points);
}


fn process_mousearea_callback(
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
                Err(err) => panic!("MouseArea callback error with user_data from ud1: {err}")
            }
        });
        drop(ud1); // Drop ud1 after processing
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Execute the callback with user data from ud2
    let ud2 = access_user_data2();
    
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::attach(|py| {
            let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone(), user_data)),
                None => cb.call1(py, (id, user_data)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("MouseArea callback error with user_data from ud2: {err}")
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // Execute the callback without user data
    Python::attach(|py| {
        let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone())),
                None => cb.call1(py, (id,)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("MouseArea callback error without user_data: {err}")
            }
    });

}

pub fn opaque_callback(_state: &mut IpgState, id: usize, event_name: String) {
    
    process_opaque_callback(id, event_name);
}


fn process_opaque_callback(id: usize, event_name: String) 
{
    let ud = access_user_data1();
    let user_data_opt = ud.user_data.get(&id);

    let app_cbs = access_callbacks();

    let callback_present = 
        app_cbs.callbacks.get(&(id, event_name));
    
    let callback = match callback_present {
        Some(cb) => cb,
        None => return,
    };

    let cb = 
        Python::attach(|py| {
            callback.clone_ref(py)
        });

    drop(app_cbs);
              
    Python::attach(|py| {
        if user_data_opt.is_some() {
            let res = cb.call1(py, (
                                                        id,
                                                        user_data_opt.unwrap()  
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Opaque: Only 2 parameter (id, user_data) 
                                    is required or a python error in this function. {er}"),
            }
        } else {
            let res = cb.call1(py, (
                                                        id,  
                                                        ));
            match res {
                Ok(_) => (),
                Err(er) => panic!("Opaque: Only 1 parameter (id) 
                                    is required or a python error in this function. {er}"),
            }
        }
        
        
    });
    
    drop(ud);   

}

pub fn pick_list_callback(state: &mut IpgState, id: usize, message: PLMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};

    match message {
        PLMessage::OnSelect(selected) => {
            wci.value_str = Some(selected.clone());
            let _ = set_or_get_widget_callback_data(state, wci);
            
            process_pick_list_callback(id, "on_select".to_string(), selected);
        },
    }
 }


 fn process_pick_list_callback(
        id: usize, 
        event_name: String, 
        selected: String) 
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
            if let Err(err) = callback.call1(py, (id, selected, user_data)) {
                panic!("PickList callback error: {err}");
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
            if let Err(err) = callback.call1(py, (id, selected, user_data)) {
                panic!("PickList callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only the id and selected
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, selected)) {
            panic!("PickList callback error: {err}");
        }
    });

 }

 use crate::choice_enums::Choice;
 pub fn radio_callback(state: &mut IpgState, id: usize, message: RDMessage) {

    let widget_opt = state.widgets.get_mut(&id);

    let widgets = match widget_opt {
        Some(rd) => rd,
        None => panic!("Radio callback with id {} could not be found", id),
    };

    let radio: &mut IpgRadio = match_widgets(widgets);

    let ch_usize = match message {
        RDMessage::RadioSelected(choice) => {
            match choice {
                Choice::Choice0(ch) => {
                    ch as usize
                },
                Choice::Choice1(ch) => {
                    ch as usize
                },
                Choice::Choice2(ch) => {
                    ch as usize
                },
                Choice::Choice3(ch) => {
                    ch as usize
                },
                Choice::Choice4(ch) => {
                    ch as usize
                },
                Choice::Choice5(ch) => {
                    ch as usize
                },
                Choice::Choice6(ch) => {
                    ch as usize
                },
                Choice::Choice7(ch) => {
                    ch as usize
                },
                Choice::Choice8(ch) => {
                    ch as usize
                },
                Choice::Choice9(ch) => {
                    ch as usize
                },
                Choice::Choice10(ch) => {
                    ch as usize
                },
                Choice::Choice11(ch) => {
                    ch as usize
                },
                Choice::Choice12(ch) => {
                    ch as usize
                },
                Choice::Choice13(ch) => {
                    ch as usize
                },
                Choice::Choice14(ch) => {
                    ch as usize
                },
                Choice::Choice15(ch) => {
                    ch as usize
                },
                Choice::Choice16(ch) => {
                    ch as usize
                },
                Choice::Choice17(ch) => {
                    ch as usize
                },
                Choice::Choice18(ch) => {
                    ch as usize
                },
                Choice::Choice19(ch) => {
                    ch as usize
                },
                Choice::Choice20(ch) => {
                    ch as usize
                },
                Choice::Choice21(ch) => {
                    ch as usize
                },
                Choice::Choice22(ch) => {
                    ch as usize
                },
                Choice::Choice23(ch) => {
                    ch as usize
                },
                Choice::Choice24(ch) => {
                    ch as usize
                },
                Choice::Choice25(ch) => {
                    ch as usize
                },
            }
        }, 
        
    };

    radio.is_selected = Some(ch_usize);

    process_radio_callback(id, "on_select".to_string(), ch_usize, radio.labels[ch_usize].clone());
    
}

fn process_radio_callback(
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
    let ud2 = access_user_data2();
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::attach(|py| {
            if let Err(err) = callback.call1(py, (id, (index, label), user_data)) {
                panic!("Radio callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with the id, index, and label
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, (index, label))) {
            panic!("Radio callback error: {err}");
        }
    });
}

pub fn scrollable_callback(_state: &mut IpgState, id: usize, vp: Viewport) {
    let mut hmap = HashMap::new();
    hmap.insert("abs_x".to_string(), vp.absolute_offset().x);
    hmap.insert("abs_y".to_string(), vp.absolute_offset().y);
    hmap.insert("rel_x".to_string(), vp.relative_offset().x);
    hmap.insert("rel_y".to_string(), vp.relative_offset().y);
    hmap.insert("rev_x".to_string(), vp.absolute_offset_reversed().x);
    hmap.insert("rev_y".to_string(), vp.absolute_offset_reversed().y);
    
    process_scrollable_callback(id, "on_scroll".to_string(), hmap);
}


pub fn process_scrollable_callback(id: usize, 
                        event_name: String, 
                        hmap: HashMap<String, f32>) 
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
            if let Err(err) = callback.call1(py, (id, hmap, user_data)) {
                panic!("Scollable callback error: {err}");
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
            if let Err(err) = callback.call1(py, (id, hmap, user_data)) {
                panic!("Scrollable callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with the id and hmap
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, hmap)) {
            panic!("Scollable callback error: {err}");
        }
    });

}

pub fn selectable_text_callback(id: usize, message: SLTXTMessage) {

    match message {
        SLTXTMessage::OnPress => {
            process_selectable_texr_callback(id, "on_press".to_string(), None);
        },
        SLTXTMessage::OnRelease => {
            process_selectable_texr_callback(id, "on_release".to_string(), None);
        },
        SLTXTMessage::OnRightPress => {
            process_selectable_texr_callback(id, "on_right_press".to_string(), None);
        },
        SLTXTMessage::OnRightRelease => {
            process_selectable_texr_callback(id, "on_right_release".to_string(), None);
        },
        SLTXTMessage::OnMiddlePress => {
            process_selectable_texr_callback(id, "on_middle_press".to_string(), None);
        },
        SLTXTMessage::OnMiddleRelease => {
            process_selectable_texr_callback(id, "on_middle_release".to_string(), None);
        },
        SLTXTMessage::OnEnter => {
            process_selectable_texr_callback(id, "on_enter".to_string(), None);
        },
        SLTXTMessage::OnMove(point) => {
            let points: Option<(String, f32, String, f32)> = Some(
                ("x".to_string(), point.x,
                "y".to_string(), point.y));
            
            process_selectable_texr_callback(id, "on_move".to_string(), points);
        },
        SLTXTMessage::OnExit => {
            process_selectable_texr_callback(id, "on_exit".to_string(), None);
        },
    }
}


fn process_selectable_texr_callback(
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
    let ud2 = access_user_data2();
    
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::attach(|py| {
            let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone(), user_data)),
                None => cb.call1(py, (id, user_data)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("SelectableText callback error with user_data from ud2: {err}")
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

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

pub fn slider_callback(state: &mut IpgState, id: usize, message: SLMessage) {

    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
           
    match message {
        SLMessage::OnChange(value) => {
            wci.value_f64 = Some(value as f64);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_slider_callback(id, "on_change".to_string(), value);
        },
        SLMessage::OnRelease => {
            // to be consistent, returning value for both
            let wco = set_or_get_widget_callback_data(state, wci);
            process_slider_callback(id, "on_release".to_string(), wco.value_f32.unwrap());
        },
    }
}

pub fn process_slider_callback(
        id: usize, 
        event_name: String, 
        value: f32) 
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
            if let Err(err) = callback.call1(py, (id, value, user_data)) {
                panic!("Slider callback error: {err}");
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
            if let Err(err) = callback.call1(py, (id, value, user_data)) {
                panic!("Slider callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with only the id and value
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, value)) {
            panic!("Slider callback error: {err}");
        }
    });

}

pub fn svg_callback(_state: &mut IpgState, id: usize, message: SvgMessage) {

    match message {
        SvgMessage::OnPress => {
            process_svg_callback(id, "on_press".to_string(), None);
        },
        SvgMessage::OnRelease => {
            process_svg_callback(id, "on_release".to_string(), None);
        },
        SvgMessage::OnRightPress => {
            process_svg_callback(id, "on_right_press".to_string(), None);
        },
        SvgMessage::OnRightRelease => {
            process_svg_callback(id, "on_right_release".to_string(), None);
        },
        SvgMessage::OnMiddlePress => {
            process_svg_callback(id, "on_middle_press".to_string(), None);
        },
        SvgMessage::OnMiddleRelease => {
            process_svg_callback(id, "on_middle_release".to_string(), None);
        },
        SvgMessage::OnEnter => {
            process_svg_callback(id, "on_enter".to_string(), None);
        },
        SvgMessage::OnMove(point) => {
            let points: Option<HashMap<String, f32>> = Some(HashMap::from([
                ("x".to_string(), point.x),
                ("y".to_string(), point.y)
            ]));
            
            process_svg_callback(id, "on_move".to_string(), points);
        },
        SvgMessage::OnExit => {
            process_svg_callback(id, "on_exit".to_string(), None);
        },
    }
}


fn process_svg_callback(
    id: usize,
    event_name: String,
    points_opt: Option<HashMap<String, f32>>,
) {
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
                Err(err) => panic!("SVG callback error with user_data from ud1: {err}")
            }
        });
        drop(ud1); // Drop ud1 after processing
        return;
    }
    drop(ud1); // Drop ud1 if no user data is found

    // Execute the callback with user data from ud2
    let ud2 = access_user_data2();
    
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::attach(|py| {
            let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone(), user_data)),
                None => cb.call1(py, (id, user_data)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("SVG callback error with user_data from ud2: {err}")
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // Execute the callback without user data
    Python::attach(|py| {
        let res = match points_opt {
                Some(ref points) => cb.call1(py, (id, points.clone())),
                None => cb.call1(py, (id,)),
            };

            match res {
                Ok(_) => (),
                Err(err) => panic!("SVG callback error without user_data: {err}")
            }
    });

}

pub fn text_input_callback(state: &mut IpgState, id: usize, message: TIMessage) {
    // During the input, the widget is assigned the value so that it shows
    // during typing.  On submit, the text box is cleared, so no value.
    // However, in both cases the value is passed to the callback.
    let mut wci: WidgetCallbackIn = WidgetCallbackIn{id, ..Default::default()};
           
    match message {
        TIMessage::OnInput(value) => {
            wci.value_str = Some(value.clone());
            let wco = set_or_get_widget_callback_data(state, wci);
            process_text_input_callback(id, "on_input".to_string(), wco.value_str.unwrap());
        },
        TIMessage::OnSubmit(value) => {
            wci.value_str = Some(value.clone());
            let wco = set_or_get_widget_callback_data(state, wci);
            process_text_input_callback(id, "on_submit".to_string(), wco.value_str.unwrap());
        }
        TIMessage::OnPaste(value) => {
            wci.value_str = Some(value.clone());
            let _ = set_or_get_widget_callback_data(state, wci);

            process_text_input_callback(id, "on_paste".to_string(), value);
        }
            
    }
}

pub fn process_text_input_callback(
        id: usize, 
        event_name: String, 
        value: String) 
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
            if let Err(err) = callback.call1(py, (id, value, user_data)) {
                panic!("TextInput callback error: {err}");
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
            if let Err(err) = callback.call1(py, (id, value, user_data)) {
                panic!("TextInput callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with the id and value
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, value)) {
            panic!("TextInput callback error: {err}");
        }
    });

}

pub fn timer_callback(state: &mut IpgState, id: usize, started: bool) -> u64 {
    let mut wci = WidgetCallbackIn{id, ..Default::default()};
    wci.value_bool = Some(started);
    let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
    wco.id = id;
    // duration is the event time not total time
    let duration = wco.duration.unwrap_or(1);
    let (event_name, counter) = if started {
        ("on_start".to_string(), None)
    } else {
        ("on_stop".to_string(), wco.counter) // the counter is set by the tick cb
    };
    
    process_timer_callback(id, event_name, counter);
    // After the start, the duratiob is sent back 
    // to set the timer event duration
    duration
}

pub fn tick_callback(state: &mut IpgState)
{
    let id= state.timer_event_id_enabled.0;
    let mut wci = WidgetCallbackIn{id, ..Default::default()};
    wci.value_str = Some("on_tick".to_string());
    let wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
    process_timer_callback(id, "on_tick".to_string(), wco.counter);
}

fn process_timer_callback(
        id: usize, 
        event_name: String, 
        counter: Option<u64>)
{
    let ud1 = access_user_data1();
    let app_cbs = access_callbacks();

    // Retrieve the callback
    let callback = match app_cbs.callbacks.get(&(id, event_name.clone())) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_cbs);

    Python::attach(|py| {
        
        let name_bool = if event_name == "on_start".to_string() {
            true
        } else {
            false
        };

        if let Some(user_data) = ud1.user_data.get(&id) {
            let res = match name_bool {
                    true => callback.call1(py, (id, user_data)),
                    false => callback.call1(py, (id, counter, user_data)),
            };
            if let Err(err) = res {
                panic!("CanvasTimer callback error: {err}");
            } else {
                drop(ud1);
                return
            }
        }
        drop(ud1); // Drop ud1 if no user data is found

        // Check user data from ud2
        let ud2 = access_user_data2();

        if let Some(user_data) = ud2.user_data.get(&id) {
            let res = match name_bool {
                    true => callback.call1(py, (id, user_data)),
                    false => callback.call1(py, (id, counter, user_data)),
            };
            if let Err(err) = res {
                panic!("CanvasTimer callback error: {err}");
            } else {
                drop(ud2);
                return
            }
        }
        drop(ud2); // Drop ud1 if no user data is found

        let res = match name_bool {
            true => callback.call1(py, (id, )),
            false => callback.call1(py, (id, counter)),
        };
        if let Err(err) = res {
            panic!("CanvasTimer callback error: {err}");
        }
    });
}

pub fn canvas_timer_callback(state: &mut IpgState, id: usize, started: bool) -> u64 {

    let mut wci = WidgetCallbackIn{id, ..Default::default()};
    wci.value_bool = Some(started);
    let mut wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
    wco.id = id;
    // duration is the event time not total time
    let duration = wco.duration.unwrap_or(1);
    let (event_name, counter) = if started {
        ("on_start".to_string(), None)
    } else {
        ("on_stop".to_string(), wco.counter)
    };
    
    process_canvas_timer_callback(id, event_name, counter);
    duration 
}

pub fn canvas_tick_callback(state: &mut IpgState) 
{
    let id= state.canvas_timer_event_id_enabled.0;
    let mut wci = WidgetCallbackIn{id, ..Default::default()};
    wci.value_str = Some("on_tick".to_string());
    let wco: WidgetCallbackOut = set_or_get_widget_callback_data(state, wci);
    process_canvas_timer_callback(id, "on_tick".to_string(), wco.counter);
}

fn process_canvas_timer_callback(
        id: usize, 
        event_name: String, 
        counter: Option<u64>)
{
    let ud1 = access_user_data1();
    let app_cbs = access_callbacks();

    // Retrieve the callback
    let callback = match app_cbs.callbacks.get(&(id, event_name.clone())) {
        Some(cb) => Python::attach(|py| cb.clone_ref(py)),
        None => return,
    };

    drop(app_cbs);

    Python::attach(|py| {
        
        let name_bool = if event_name == "on_start".to_string() {
            true
        } else {
            false
        };

        if let Some(user_data) = ud1.user_data.get(&id) {
            let res = match name_bool {
                    true => callback.call1(py, (id, user_data)),
                    false => callback.call1(py, (id, counter, user_data)),
            };
            if let Err(err) = res {
                panic!("CanvasTimer callback error: {err}");
            } else {
                drop(ud1);
                return
            }
        }
        drop(ud1); // Drop ud1 if no user data is found

        // Check user data from ud2
        let ud2 = access_user_data2();

        if let Some(user_data) = ud2.user_data.get(&id) {
            let res = match name_bool {
                    true => callback.call1(py, (id, user_data)),
                    false => callback.call1(py, (id, counter, user_data)),
            };
            if let Err(err) = res {
                panic!("CanvasTimer callback error: {err}");
            } else {
                drop(ud2);
                return
            }
        }
        drop(ud2); // Drop ud1 if no user data is found

        let res = match name_bool {
            true => callback.call1(py, (id, )),
            false => callback.call1(py, (id, counter)),
        };
        if let Err(err) = res {
            panic!("CanvasTimer callback error: {err}");
        }
    });
}

pub fn toggle_callback(state: &mut IpgState, id: usize, message: TOGMessage) {

    let mut wci = WidgetCallbackIn{id, ..Default::default()};

    match message {
        TOGMessage::Toggled(on_toggle) => {
            wci.on_toggle = Some(on_toggle);
            let _ = set_or_get_widget_callback_data(state, wci);
            process_toggle_callback(id, "toggled".to_string(), on_toggle);
        }
    }
}

pub fn process_toggle_callback(
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
    let ud2 = access_user_data2();
    if let Some(user_data) = ud2.user_data.get(&id) {
        Python::attach(|py| {
            if let Err(err) = callback.call1(py, (id, toggled, user_data)) {
                panic!("Toggler callback error: {err}");
            }
        });
        drop(ud2); // Drop ud2 after processing
        return;
    }
    drop(ud2); // Drop ud2 if no user data is found

    // If no user data is found in both ud1 and ud2, call the callback with the id and toggled
    Python::attach(|py| {
        if let Err(err) = callback.call1(py, (id, toggled)) {
            panic!("Toggler callback error: {err}");
        }
    });
         
}
