
use std::sync::Mutex;
use std::sync::MutexGuard;

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug)]
pub struct WindowActions {
    pub mode: Vec<(usize, iced::window::Mode)>,
    pub decorations: Vec<usize>,
    pub resize: Vec<(usize, f32, f32)>,
    pub position: Vec<(usize, f32, f32)>,
    pub level: Vec<(usize, iced::window::Level)>,
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

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgWindowLevel {
    Normal,
    AlwaysOnBottom,
    AlwaysOnTop,
}

pub fn get_level(level: &IpgWindowLevel) -> iced::window::Level {
    match level {
        IpgWindowLevel::Normal => iced::window::Level::Normal,
        IpgWindowLevel::AlwaysOnBottom => iced::window::Level::AlwaysOnBottom,
        IpgWindowLevel::AlwaysOnTop => iced::window::Level::AlwaysOnTop,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[pyclass(eq, eq_int)]
pub enum IpgWindowMode {
    Windowed,
    FullScreen,
    Closed,
}

pub fn get_iced_mode(mode: &IpgWindowMode) -> iced::window::Mode {
    match mode {
        IpgWindowMode::Windowed => iced::window::Mode::Windowed,
        IpgWindowMode::FullScreen => iced::window::Mode::Fullscreen,
        IpgWindowMode::Closed => iced::window::Mode::Hidden,
    }
}

pub fn get_ipg_mode(mode: iced::window::Mode) -> IpgWindowMode {
    match mode {
        iced::window::Mode::Windowed => IpgWindowMode::Windowed,
        iced::window::Mode::Fullscreen => IpgWindowMode::FullScreen,
        iced::window::Mode::Hidden => IpgWindowMode::Closed,
    }
}
