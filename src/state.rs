//! Global state management using static Mutexes
//! 
//! This module provides the shared state that Python interacts with before Iced starts,
//! and that Iced copies/clones when it begins running.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

use iced::window;
use iced::Theme;
use once_cell::sync::Lazy;
use pyo3::{Py, PyAny};

use crate::widgets::window::IpgWindow;
use crate::widgets::button::IpgButton;

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;

// ============================================================================
// Container and Widget enums - minimal versions for the prototype
// ============================================================================

#[derive(Debug, Clone)]
pub enum IpgContainers {
    IpgWindow(IpgWindow),
    // Add more containers as needed
}

#[derive(Debug, Clone)]
pub enum IpgWidgets {
    IpgButton(IpgButton),
    // Add more widgets as needed
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
    callbacks: Lazy<HashMap<(usize, String), PyObject>>,
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
// User data storage
// ============================================================================

#[derive(Debug)]
pub struct UserData {
    user_data: Lazy<HashMap<usize, PyObject>>,
}

pub static USERDATA: Mutex<UserData> = Mutex::new(UserData {
    user_data: Lazy::new(|| HashMap::new()),
});

pub fn access_user_data() -> MutexGuard<'static, UserData> {
    USERDATA.lock().unwrap()
}

impl UserData {
    pub fn insert(&mut self, id: usize, data: PyObject) {
        self.user_data.insert(id, data);
    }
    
    pub fn get(&self, id: usize) -> Option<&PyObject> {
        self.user_data.get(&id)
    }
}

// ============================================================================
// Main State - stores all widget/container definitions before Iced starts
// ============================================================================

#[derive(Debug)]
pub struct State {
    pub ids: Lazy<HashMap<usize, Vec<IpgIds>>>,  // <window_id, Vec<IpgIds>>
    pub last_id: usize,
    pub gen_ids: Vec<usize>,

    pub containers: Lazy<HashMap<usize, IpgContainers>>,
    pub container_ids: Lazy<HashMap<usize, Vec<usize>>>,  // <window_id, vec<container_id>>
    pub container_str_ids: Lazy<HashMap<String, usize>>,  // container string -> usize id
    pub container_wnd_str_ids: Lazy<HashMap<String, String>>,  // container string -> window string
    pub container_window_usize_ids: Lazy<HashMap<usize, usize>>,  // container usize -> window usize

    pub widgets: Lazy<HashMap<usize, IpgWidgets>>,
    pub widget_container_ids: Lazy<HashMap<usize, String>>,  // widget_id -> container_id string

    pub windows: Vec<IpgWindow>,
    pub windows_str_ids: Lazy<HashMap<String, usize>>,  // window string -> usize id
}

pub static STATE: Mutex<State> = Mutex::new(State {
    ids: Lazy::new(|| HashMap::new()),
    last_id: 0,
    gen_ids: vec![],
    containers: Lazy::new(|| HashMap::new()),
    container_ids: Lazy::new(|| HashMap::new()),
    container_str_ids: Lazy::new(|| HashMap::new()),
    container_wnd_str_ids: Lazy::new(|| HashMap::new()),
    container_window_usize_ids: Lazy::new(|| HashMap::new()),
    widgets: Lazy::new(|| HashMap::new()),
    widget_container_ids: Lazy::new(|| HashMap::new()),
    windows: vec![],
    windows_str_ids: Lazy::new(|| HashMap::new()),
});

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
}

impl IpgState {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Clone data from the static mutex state into IpgState for Iced runtime
pub fn clone_state_to_runtime(runtime_state: &mut IpgState) {
    let state = access_state();
    
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
    
    drop(state);
}

// ============================================================================
// Helper functions for adding callbacks/user data
// ============================================================================

pub fn add_callback(id: usize, event_name: String, callback: PyObject) {
    let mut callbacks = access_callbacks();
    callbacks.insert(id, event_name, callback);
    drop(callbacks);
}

pub fn add_user_data(id: usize, data: PyObject) {
    let mut user_data = access_user_data();
    user_data.insert(id, data);
    drop(user_data);
}
