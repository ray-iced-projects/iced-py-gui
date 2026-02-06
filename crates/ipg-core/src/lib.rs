



#[derive(Debug, Clone)]
pub enum Message {
    Button(usize, BTNMessage),
    Canvas(CanvasMessage),
    Card(usize, CardMessage),
    CheckBox(usize, CHKMessage),
    ColorPicker(usize, ColPikMessage),
    DatePicker(usize, DPMessage),
    Divider(usize, DivMessage),
    EventKeyboard(Event),
    EventMouse(Event),
    EventWindow((window::Id, Event)),
    EventTouch(Event),
    Image(usize, ImageMessage),
    // Modal(usize, ModalMessage),
    PickList(usize, PLMessage),
    Radio(usize, RDMessage),
    Scrolled(scrollable::Viewport, usize),
    SelectableText(usize, SLTXTMessage),
    Slider(usize, SLMessage),
    Svg(usize, SvgMessage),

    TableSync(scrollable::AbsoluteOffset, usize),
    TableDividerChanged((usize, usize, f32)),
    TableDividerReleased(usize),

    TextInput(usize, TIMessage),
    Toggler(usize, TOGMessage),
    CanvasTextBlink,
    Tick,
    CanvasTick,
    Timer(usize, TIMMessage),
    CanvasTimer(usize, CanvasTimerMessage),
    FontLoaded(Result<(), font::Error>),
    WindowOpened(window::Id, Option<Point>, Size),

    MouseAreaOnPress(usize),
    MouseAreaOnRelease(usize),
    MouseAreaOnRightPress(usize),
    MouseAreaOnRightRelease(usize),
    MouseAreaOnMiddlePress(usize),
    MouseAreaOnMiddleRelease(usize),
    MouseAreaOnEnter(usize),
    MouseAreaOnMove(Point, usize),
    MouseAreaOnExit(usize),

    OpaqueOnPress(usize),
}

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
