//! Window widget definition

use iced::window::{self, Position};
use iced::{Size, Theme};
use pyo3::{Python, pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

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
