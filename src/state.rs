//! Global state management using static Mutexes
//! 
//! This module provides the shared state that Python interacts with before Iced starts,
//! and that Iced copies/clones when it begins running.
#![allow(unused)]
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use iced::window;
use iced::Theme;
use once_cell::sync::Lazy;
use pyo3::{Py, PyAny};

use crate::widgets::ipg_checkbox::{IpgCheckBox, IpgCheckboxStyle};
use crate::widgets::ipg_column::IpgColumn;
use crate::widgets::ipg_container::{IpgContainer, IpgContainerStyle};
use crate::widgets::ipg_events::IpgEvents;
use crate::widgets::ipg_font::IpgFont;
use crate::widgets::ipg_row::IpgRow;
use crate::widgets::ipg_window::IpgWindow;
use crate::widgets::ipg_button::{IpgButton, IpgButtonStyle};

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;

// ============================================================================
// Container and Widget enums - minimal versions for the prototype
// ============================================================================

#[derive(Debug, Clone)]
pub enum IpgContainers {
    IpgWindow(IpgWindow),
    IpgColumn(IpgColumn),
    IpgContainer(IpgContainer),
    IpgRow(IpgRow),
}

#[derive(Debug, Clone)]
pub enum IpgWidgets {
    IpgButton(IpgButton),
    IpgButtonStyle(IpgButtonStyle),
    IpgCheckBox(IpgCheckBox),
    IpgCheckboxStyle(IpgCheckboxStyle),
    IpgContainerStyle(IpgContainerStyle),
    IpgFont(IpgFont),
}

// ============================================================================
// IpgIds - tracks parent/child relationships
// ============================================================================

#[derive(Debug, Clone)]
pub struct IpgIds {
    pub id: usize,
    pub parent_uid: usize,
    pub container_id: Option<String>,
    pub parent_id: String,
    pub is_container: bool,
}

// ============================================================================
// Callbacks storage
// ============================================================================

#[derive(Debug)]
pub struct Callbacks {
    pub(crate) callbacks: Lazy<HashMap<(usize, String), PyObject>>,
}

pub static CALLBACKS: Mutex<Callbacks> = Mutex::new(Callbacks {
    callbacks: Lazy::new(|| HashMap::new()),
});

pub fn access_callbacks() -> MutexGuard<'static, Callbacks> {
    CALLBACKS.lock().unwrap()
}

impl Callbacks {
    pub fn insert(&mut self, id: usize, event_name: String, callback: PyObject) {
        self.callbacks.insert((id, event_name), callback);
    }
    
    pub fn get(&self, id: usize, event_name: &str) -> Option<&PyObject> {
        self.callbacks.get(&(id, event_name.to_string()))
    }
}

// ============================================================================
// Event storage
// ============================================================================

#[derive(Debug)]
pub struct Events {
    pub events: Lazy<HashMap<(usize, String), PyObject>>,
}

pub static EVENTS: Mutex<Events> = Mutex::new(Events {
    events:  Lazy::new(||HashMap::new()),
});

pub fn access_events() -> MutexGuard<'static, Events> {
    EVENTS.lock().unwrap()
}

// ============================================================================
// User data storage
// ============================================================================

#[derive(Debug)]
pub struct UserData1 {
    pub user_data: Lazy<HashMap<usize, PyObject>>,
}

pub static USERDATA1: Mutex<UserData1> = Mutex::new(UserData1 {
    user_data: Lazy::new(|| HashMap::new()),
});

pub fn access_user_data1() -> MutexGuard<'static, UserData1> {
    USERDATA1.lock().unwrap()
}

impl UserData1 {
    pub fn insert(&mut self, id: usize, data: PyObject) {
        self.user_data.insert(id, data);
    }
    
    pub fn get(&self, id: usize) -> Option<&PyObject> {
        self.user_data.get(&id)
    }
}

#[derive(Debug)]
pub struct UserData2 {
    pub user_data: Lazy<HashMap<usize, PyObject>>,
}

pub static USERDATA2: Mutex<UserData2> = Mutex::new(UserData2 {
    user_data: Lazy::new(|| HashMap::new()),
});

pub fn access_user_data2() -> MutexGuard<'static, UserData2> {
    USERDATA2.lock().unwrap()
}

impl UserData2 {
    pub fn insert(&mut self, id: usize, data: PyObject) {
        self.user_data.insert(id, data);
    }
    
    pub fn get(&self, id: usize) -> Option<&PyObject> {
        self.user_data.get(&id)
    }
}

#[derive(Debug)]
pub struct UpdateWidgets {
    // (wid, item, value)
    pub updates: Vec<(usize, PyObject, PyObject)>, 
    // window_id_widget_id, (window_id, wid, target_container_str_id, move_after(wid), move_before(wid))
    pub moves: Vec<(String, usize, String, Option<usize>, Option<usize>)>,
    // window_id, wid
    pub deletes: Vec<(String, usize)>,
    pub shows: Vec<(String, Vec<(usize, bool)>)>,
    pub dataframes: Vec<(usize, PyObject)>, // PyDataFrame for polar later
    pub new_widgets: Lazy<HashMap<usize, IpgWidgets>>,
}

pub static UPDATE_WIDGETS: Mutex<UpdateWidgets> = Mutex::new(UpdateWidgets {
    updates: vec![],
    moves: vec![],
    deletes: vec![],
    shows: vec![],
    dataframes: vec![],
    new_widgets: Lazy::new(||HashMap::new()),
});

pub fn access_update_widgets() -> MutexGuard<'static, UpdateWidgets> {
    UPDATE_WIDGETS.lock().unwrap()
}

#[derive(Debug)]
pub struct WindowActions {
    pub mode: Vec<(usize, window::Mode)>,
    pub decorations: Vec<usize>,
    pub resize: Vec<(usize, f32, f32)>,
    pub position: Vec<(usize, f32, f32)>,
    pub level: Vec<(usize, window::Level)>,
}

pub static WINDOW_ACTIONS: Mutex<WindowActions> = Mutex::new(WindowActions {
    mode: vec![],
    decorations: vec![],
    resize: vec![],
    position: vec![],
    level: vec![],
});

pub fn access_window_actions() -> MutexGuard<'static, WindowActions> {
    WINDOW_ACTIONS.lock().unwrap()
}


// ============================================================================
// Main State - stores all widget/container definitions before Iced starts
// ============================================================================

#[derive(Debug)]
pub struct State {
    pub ids: Lazy<HashMap<usize, Vec<IpgIds>>>,  // <window_id=usize, Vec<IpgIds=structure>>
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
        ids: Lazy::new(||HashMap::new()),
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

impl State {
    pub fn next_id(&mut self) -> usize {
        self.last_id += 1;
        self.last_id
    }
}

// ============================================================================
// IpgState - the runtime state that Iced uses (cloned from static State)
// ============================================================================

#[derive(Default, Debug, Clone)]
pub struct IpgState {
    pub ids: HashMap<usize, Vec<IpgIds>>,
    pub last_id: usize,

    pub containers: HashMap<usize, IpgContainers>,
    pub container_ids: HashMap<usize, Vec<usize>>,
    pub container_str_ids: HashMap<String, usize>,
    pub container_wnd_str_ids: HashMap<String, String>,
    pub container_window_usize_ids: HashMap<usize, usize>,

    pub widgets: HashMap<usize, IpgWidgets>,
    pub widget_container_ids: HashMap<usize, String>,

    pub windows: Vec<IpgWindow>,
    pub windows_str_ids: HashMap<String, usize>,
    pub windows_iced_ipg_ids: HashMap<window::Id, usize>,
    pub windows_opened: Vec<window::Id>,
    pub windows_hidden: Vec<window::Id>,
    pub window_debug: HashMap<window::Id, (usize, bool)>,
    pub window_theme: HashMap<window::Id, (usize, Theme)>,
    pub window_mode: HashMap<window::Id, (usize, window::Mode)>,

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

impl IpgState {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Clone data from the static mutex state into IpgState for Iced runtime
pub fn clone_state_to_runtime(runtime_state: &mut IpgState) {
    let mut state = access_state();
    
    runtime_state.ids = state.ids.clone();
    runtime_state.last_id = state.last_id;
    runtime_state.containers = state.containers.clone();
    runtime_state.container_ids = state.container_ids.clone();
    runtime_state.container_str_ids = state.container_str_ids.clone();
    runtime_state.container_wnd_str_ids = state.container_wnd_str_ids.clone();
    runtime_state.container_window_usize_ids = state.container_window_usize_ids.clone();
    runtime_state.widgets = state.widgets.clone();
    runtime_state.widget_container_ids = state.widget_container_ids.clone();
    runtime_state.windows = state.windows.clone();
    runtime_state.windows_str_ids = state.windows_str_ids.clone();

    runtime_state.events = state.events.clone();
    runtime_state.keyboard_event_id_enabled = state.keyboard_event_id_enabled;
    runtime_state.mouse_event_id_enabled = state.mouse_event_id_enabled; 
    runtime_state.timer_event_id_enabled = state.timer_event_id_enabled;
    runtime_state.canvas_timer_event_id_enabled = state.canvas_timer_event_id_enabled;
    runtime_state.window_event_id_enabled = state.window_event_id_enabled;
    runtime_state.touch_event_id_enabled = state.touch_event_id_enabled;
    runtime_state.timer_duration = state.timer_duration;
    runtime_state.canvas_timer_duration = state.canvas_timer_duration;

    // Zero out transferred data so process_updates won't re-process them
    state.widgets = Lazy::new(|| HashMap::new());
    state.widget_container_ids = Lazy::new(|| HashMap::new());
    state.windows = vec![];
    
    drop(state);
}

// ============================================================================
// Helper functions for adding callbacks/user data
// ============================================================================

/// Find the parent container's usize ID from its string ID
fn find_parent_uid(ipg_ids: &[IpgIds], parent_id: String) -> usize {
    for id_info in ipg_ids.iter() {
        if id_info.container_id == Some(parent_id.clone()) {
            return id_info.id;
        }
    }
    panic!("Parent id {:?} not found in find_parent_uid()", parent_id)
}

/// Set up widget state - registers the widget with its parent container
pub fn set_state_of_widget(id: usize, parent_id: String) {
    let state = access_state();

    // Find the window string ID from the container string ID
    let wnd_id_str = match state.container_wnd_str_ids.get(&parent_id) {
        Some(id) => id.clone(),
        None => panic!(
            "The main window id could not be found using parent_id {}, check that your parent_id matches a container",
            parent_id
        ),
    };

    // Find the window usize ID
    let wnd_id_usize = match state.windows_str_ids.get(&wnd_id_str) {
        Some(id) => *id,
        None => panic!("window id {} could not be found in set_state_of_widget", wnd_id_str),
    };

    drop(state);

    let mut state = access_state();

    // Find the parent's usize ID
    let parent_uid = find_parent_uid(state.ids.get(&wnd_id_usize).unwrap(), parent_id.clone());

    // Register this widget with the window's ID tracking
    state.ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds {
        id,
        parent_uid,
        container_id: None,
        parent_id,
        is_container: false,
    });

    drop(state);
}

pub fn set_state_of_widget_running_state(
    state: &mut IpgState,
    id: usize,  
    parent_id: String)
{
    let wnd_id_str = match state.container_wnd_str_ids.get(&parent_id) {
        Some(id) => id.clone(),
        None => panic!("The main window id could not be found using parent_id {}, check that your parent_id matches a container ", parent_id)
    };

    let wnd_id_usize = match state.windows_str_ids.get(&wnd_id_str) {
        Some(id) => *id,
        None => panic!("window id {} could not be found in set_state_of_widget", wnd_id_str),
    };

    let parent_uid = find_parent_uid(state.ids.get(&wnd_id_usize).unwrap(), parent_id.clone());
    
    state.ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds{id, parent_uid, container_id: None,
                                                        parent_id, is_container: false});

}

pub fn set_state_of_container(
    id: usize, 
    window_id: String, 
    container_id: Option<String>, 
    parent_id: String) 
{
    let state = access_state();

    let wnd_id_usize = match state.windows_str_ids.get(&window_id) {
        Some(id) => *id,
        None => panic!("The main window id could not be found using window_id {}", window_id)
    };
    drop(state);

    let mut state = access_state();

    match container_id.clone() {
        Some(container_id_str) => state.container_wnd_str_ids.insert(container_id_str, window_id),
        None => None,
    };
    
    let parent_uid = find_parent_uid(state.ids.get(&wnd_id_usize).unwrap(), parent_id.clone());
    
    state.ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds{id, parent_uid, container_id,
                                                        parent_id, is_container: true});

    state.container_ids.get_mut(&wnd_id_usize).unwrap().push(id);

    drop(state);

}

pub fn set_state_cont_wnd_ids(
    state: &mut State, 
    wnd_id: &String, 
    cnt_str_id: String, 
    cnt_id: usize, name: String) 
{
    state.container_str_ids.insert(cnt_str_id.clone(), cnt_id);

    let wnd_id_usize_opt = state.windows_str_ids.get(wnd_id);

    let wnd_id_usize = match wnd_id_usize_opt {
        Some(id) => *id,
        None => panic!("{}: could not get window usize id", name),
    };

    state.container_str_ids.insert(cnt_str_id, cnt_id);

    state.container_window_usize_ids.insert(cnt_id, wnd_id_usize);
}

pub fn get_id(gen_id: Option<usize>) -> usize {
    
    let mut state = access_state();

    // Get or generate ID
    let id = match gen_id {
        Some(gid) => gid,
        None => {
            state.last_id += 1;
            state.last_id
        }
    };

    drop(state);

    id
}

// pub fn set_state_of_widget_running_state(
//     state: &mut IpgState,
//     id: usize,  
//     parent_id: String)
// {
//     let wnd_id_str = match state.container_wnd_str_ids.get(&parent_id) {
//         Some(id) => id.clone(),
//         None => panic!("The main window id could not be found using parent_id {}, check that your parent_id matches a container ", parent_id)
//     };

//     let wnd_id_usize = match state.windows_str_ids.get(&wnd_id_str) {
//         Some(id) => *id,
//         None => panic!("window id {} could not be found in set_state_of_widget", wnd_id_str),
//     };

//     let parent_uid = find_parent_uid(state.ids.get(&wnd_id_usize).unwrap(), parent_id.clone());
    
//     state.ids.get_mut(&wnd_id_usize).unwrap().push(IpgIds{id, parent_uid, container_id: None,
//                                                         parent_id, is_container: false});

// }

pub fn add_callback_to_mutex(id: usize, event_name: String, callback: PyObject) {
    let mut callbacks = access_callbacks();
    callbacks.insert(id, event_name, callback);
    drop(callbacks);
}

pub fn add_user_data_to_mutex(
    id: usize, 
    user_data: PyObject) 
{
    let mut lock = USERDATA1.try_lock();
    if let Ok(ref mut ud) = lock {
        ud.user_data.insert(id, user_data);
        
    } else {
        let mut temp_ud = access_user_data2();
        temp_ud.user_data.insert(id, user_data);
        drop(temp_ud);
    }
    drop(lock);
}
