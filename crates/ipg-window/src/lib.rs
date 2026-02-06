#![allow(dead_code)]
use iced::window;
use iced::{Element, Theme, Size};
use iced::widget::Column;

use pyo3::{pyclass, Py, PyAny, Python};
type PyObject = Py<PyAny>;

use ipg_helpers::{try_extract_boolean, try_extract_f64, 
    try_extract_u64, try_extract_vec_f32};
use ipg_types::Message;

mod actions;
use actions::access_window_actions;

use crate::actions::{IpgWindowLevel, IpgWindowMode, get_iced_mode, get_level};

#[derive(Debug, Clone)]
pub struct IpgWindow {
    pub id: usize,
    pub title: String,
    pub size: Size,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub theme: Theme,
    pub position: window::Position,
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
        position: window::Position,
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


pub fn construct_window(content: Vec<Element<Message>>) -> Element<Message> {
    Column::with_children(content).into()
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


fn extract_theme(theme_opt: Option<PyObject>) -> IpgWindowTheme {

    let theme = match theme_opt {
        Some(th) => th,
        None => return IpgWindowTheme::Dark,
    };

    Python::attach(|py| {
        let res = theme.extract::<IpgWindowTheme>(py);
            
        match res {
            Ok(theme) => theme,
            Err(_) => panic!("Window theme extraction failed."),
        }
    }) 
}

pub fn get_iced_window_theme(theme: IpgWindowTheme) -> Theme {

    match theme {
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
        IpgWindowTheme::SolarizedDark => Theme::SolarizedDark,
        IpgWindowTheme::SolarizedLight => Theme::SolarizedLight,
        IpgWindowTheme::TokyoNight => Theme::TokyoNight,
        IpgWindowTheme::TokyoNightLight => Theme::TokyoNightLight,
        IpgWindowTheme::TokyoNightStorm => Theme::TokyoNightStorm,
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
            wnd.theme = get_iced_window_theme(val);
        },
        IpgWindowParam::ScaleFactor => {
            wnd.scale_factor = try_extract_f64(value, name);
        },
        IpgWindowParam::Mode => {
            let ipg_mode = try_extract_mode(value);
            let mode = get_iced_mode(&ipg_mode);
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
            let level = get_level(&ipg_level);
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

