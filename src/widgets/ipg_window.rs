//! Window widget definition
#![allow(unused)]
use iced::widget::Column;
use iced::window::{self, Position};
use iced::{Element, Size, Task, Theme};
use pyo3::{Python, pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::IpgState;
use crate::app::Message;
use crate::py_api::helpers::{try_extract_boolean, 
    try_extract_f64, try_extract_u64, try_extract_vec_f32};
use crate::state::access_window_actions;

#[derive(Debug, Clone)]
pub struct IpgWindow {
    pub id: usize,
    pub title: String,
    pub size: Size,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub theme: Theme,
    pub position: Position,
    pub exit_on_close_request: bool,
    pub resizable: bool,
    pub mode: IpgWindowMode,
    pub decorations: bool,
    pub transparent: bool,
    pub level: IpgWindowLevel,
    pub scale_factor: f64,
    pub debug: bool,
}

impl IpgWindow {
    pub fn new(
        id: usize,
        title: String,
        size: Size,
        min_size: Option<Size>,
        max_size: Option<Size>,
        position: Position,
        exit_on_close_request: bool,
        theme: Theme,
        resizable: bool,
        mode: IpgWindowMode,
        decorations: bool,
        transparent: bool,
        level: IpgWindowLevel,
        scale_factor: f64,
        debug: bool,
    ) -> Self {
        Self {
            id,
            title,
            size,
            min_size,
            max_size,
            position,
            exit_on_close_request,
            theme,
            resizable,
            mode,
            decorations,
            transparent,
            level,
            scale_factor,
            debug,
        }
    }
}

#[derive(Debug, Clone)]
pub enum WndMessage {
    TitleChanged(window::Id, String),
    NewWindow,
    ScaleInputChanged(window::Id, String),
    ScaleChanged(window::Id, String), 
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgWindowParam {
    Decorations,
    Debug,
    Level,
    Mode,
    Position,
    Size,
    Theme,
    ScaleFactor,
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
            level: IpgWindowLevel::to_iced(&state.windows[i].level),
            exit_on_close_request: state.windows[i].exit_on_close_request,
            ..Default::default()
        });
        let id = state.windows[i].id;
        let debug = state.windows[i].debug;
        let theme = state.windows[i].theme.clone();
        let mode = state.windows[i].mode.clone();

        state.window_debug.insert(iced_id, (id, debug));
        state.window_theme.insert(iced_id, (id, theme));
        state.window_mode.insert(iced_id, (id, IpgWindowMode::to_iced(&mode)));
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

pub fn construct_window(content: Vec<Element<Message>>) -> Element<Message> {
    Column::with_children(content).into()
}

pub fn window_item_update(wnd: &mut IpgWindow,
                            item: &PyObject,
                            value: &PyObject
                            )
{
    let update = try_extract_window_update(item);
    let name = "Window".to_string();
    match update {
        IpgWindowParam::Debug => {
            wnd.debug = try_extract_boolean(value, name);
        },
        IpgWindowParam::Theme => {
            let val = try_extract_ipg_theme(value);
            wnd.theme = IpgWindowTheme::to_iced(&val);
        },
        IpgWindowParam::ScaleFactor => {
            wnd.scale_factor = try_extract_f64(value, name);
        },
        IpgWindowParam::Mode => {
            let ipg_mode = try_extract_mode(value);
            let mode = IpgWindowMode::to_iced(&ipg_mode);
            wnd.mode = ipg_mode;
            let mut state = access_window_actions();
            state.mode.push((wnd.id, mode));
            drop(state)
        },
        IpgWindowParam::Decorations => {
            let val = try_extract_u64(value, name) as usize;
            let mut state = access_window_actions();
            state.decorations.push(val);
            drop(state)
        },
        IpgWindowParam::Level => {
            let ipg_level = try_extract_level(value);
            let level = IpgWindowLevel::to_iced(&ipg_level);
            wnd.level = ipg_level;
            let mut state = access_window_actions();
            state.level.push((wnd.id, level));
            drop(state)
        },
        IpgWindowParam::Position => {
            let val = try_extract_vec_f32(value, name);
            let mut state = access_window_actions();
            state.position.push((wnd.id, val[0], val[1]));
            drop(state)
        },
        IpgWindowParam::Size => {
            let val = try_extract_vec_f32(value, name);
            let mut state = access_window_actions();
            state.resize.push((wnd.id, val[0], val[1]));
            drop(state)
        },
    }

}


fn try_extract_window_update(update_obj: &PyObject) -> IpgWindowParam {

    Python::attach(|py| {
        let res = update_obj.extract::<IpgWindowParam>(py);
        match res {
            Ok(update) => update,
            Err(_) => panic!("Window update extraction failed"),
        }
    })
}

fn try_extract_ipg_theme(theme: &PyObject) -> IpgWindowTheme {

    Python::attach(|py| {
        let res = theme.extract::<IpgWindowTheme>(py);
        match res {
            Ok(theme) => theme,
            Err(_) => panic!("Window theme extraction failed"),
        }
    })
}

fn try_extract_mode(mode: &PyObject) -> IpgWindowMode {
    Python::attach(|py| {
        let res = mode.extract::<IpgWindowMode>(py);
        match res {
            Ok(mode) => mode,
            Err(e) => panic!("Window mode extraction failed with error {}", e),
        }
    })
}

fn try_extract_level(level: &PyObject) -> IpgWindowLevel {
    Python::attach(|py| {
        let res = level.extract::<IpgWindowLevel>(py);
        match res {
            Ok(level) => level,
            Err(e) => panic!("Window level extraction failed with error {}", e),
        }
    })
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgWindowTheme {
    Dark,
    Light,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    Dracula,
    Ferra,
    GruvboxLight,
    GruvboxDark,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Nord,
    Oxocarbon,
    SolarizedLight,
    SolarizedDark,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
}

impl IpgWindowTheme {
    pub fn to_iced(&self) -> Theme {
        match self {
            IpgWindowTheme::Dark => Theme::Dark,
            IpgWindowTheme::Light => Theme::Light,
            IpgWindowTheme::CatppuccinLatte => Theme::CatppuccinLatte,
            IpgWindowTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            IpgWindowTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            IpgWindowTheme::CatppuccinMocha => Theme::CatppuccinMocha,
            IpgWindowTheme::Dracula => Theme::Dracula,
            IpgWindowTheme::Ferra => Theme::Ferra,
            IpgWindowTheme::GruvboxLight => Theme::GruvboxLight,
            IpgWindowTheme::GruvboxDark => Theme::GruvboxDark,
            IpgWindowTheme::KanagawaWave => Theme::KanagawaWave,
            IpgWindowTheme::KanagawaDragon => Theme::KanagawaDragon,
            IpgWindowTheme::KanagawaLotus => Theme::KanagawaLotus,
            IpgWindowTheme::Moonfly => Theme::Moonfly,
            IpgWindowTheme::Nightfly => Theme::Nightfly,
            IpgWindowTheme::Nord => Theme::Nord,
            IpgWindowTheme::Oxocarbon => Theme::Oxocarbon,
            IpgWindowTheme::SolarizedLight => Theme::SolarizedLight,
            IpgWindowTheme::SolarizedDark => Theme::SolarizedDark,
            IpgWindowTheme::TokyoNight => Theme::TokyoNight,
            IpgWindowTheme::TokyoNightStorm => Theme::TokyoNightStorm,
            IpgWindowTheme::TokyoNightLight => Theme::TokyoNightLight,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgWindowLevel {
    Normal,
    AlwaysOnBottom,
    AlwaysOnTop,
}

impl IpgWindowLevel {
    pub fn to_iced(&self) -> window::Level {
        match self {
            IpgWindowLevel::Normal => window::Level::Normal,
            IpgWindowLevel::AlwaysOnBottom => window::Level::AlwaysOnBottom,
            IpgWindowLevel::AlwaysOnTop => window::Level::AlwaysOnTop,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[pyclass(eq, eq_int)]
pub enum IpgWindowMode {
    Windowed,
    FullScreen,
    Closed,
}

impl IpgWindowMode {
    pub fn to_iced(&self) -> window::Mode {
        match self {
            IpgWindowMode::Windowed => window::Mode::Windowed,
            IpgWindowMode::FullScreen => window::Mode::Fullscreen,
            IpgWindowMode::Closed => window::Mode::Hidden,
        }
    }
}
