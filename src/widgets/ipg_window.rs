//! Window widget definition
#![allow(unused)]
use iced::widget::Column;
use iced::window::settings::PlatformSpecific;
use iced::window::{self, Level, Position, icon};
use iced::{Element, Size, Task, Theme};
use pyo3::{Python, pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::IpgState;
use crate::app::Message;
use crate::py_api::helpers::{try_extract_boolean, 
    try_extract_f32, try_extract_usize, try_extract_vec_f32};
use crate::state::access_window_actions;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, extract_param,
    set_opt_bool, set_opt_f32,
};

#[derive(Debug, Clone)]
pub struct IpgWindow {
    pub id: usize,
    pub title: Option<String>,
    pub size: Option<Size>,
    pub maximized: Option<bool>,
    pub fullscreen: Option<bool>,
    pub centered: Option<bool>,
    pub position: Option<(f32, f32)>,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub theme: Option<IpgWindowTheme>,
    pub visible: Option<bool>,
    pub resizable: Option<bool>,
    pub minimizable: Option<bool>,
    pub closeable: Option<bool>,
    pub decorations: Option<bool>,
    pub transparent: Option<bool>,
    pub blur: Option<bool>,
    pub level: Option<IpgWindowLevel>,
    pub icon_rgba: Option<Vec<u8>>,
    pub icon_width_height: Option<(u32, u32)>,
    pub exit_on_close_request: Option<bool>,
    pub scale_factor: Option<f32>,
    pub debug: Option<bool>,
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
    Position,
    Size,
    Theme,
    ScaleFactor,
}

pub fn add_windows(state: &mut IpgState) -> Vec<Task<Message>> {

    let mut modes: Vec<(usize, window::Mode)> = vec![];

    let mut spawn_window: Vec<Task<Message>> = vec![];

    for i in 0..state.windows.len() {
        
        let (visible, mode) = if let Some(vis) = state.windows[i].visible {
            modes.push((state.windows[i].id, window::Mode::Hidden));
           ( false, window::Mode::Hidden)
        } else {
            modes.push((state.windows[i].id, window::Mode::Windowed));
            (true, window::Mode::Windowed)
        };
        
        let (fullscreen, mode) = if let Some(full) = state.windows[i].fullscreen {
            modes.push((state.windows[i].id, window::Mode::Fullscreen));
            (true, window::Mode::Fullscreen)
        } else { (false, mode) };

        let def_setting = window::Settings::default();
        
        let level = if let Some(lv) = &state.windows[i].level {
            IpgWindowLevel::to_iced(lv)
        } else {
            Level::default()
        };

        let position = if let Some(ct) = state.windows[i].centered {
            Position::Centered
        } else {
            if let Some(pos) = state.windows[i].position {
                let point = iced::Point::new(pos.0, pos.1);
                Position::Specific(point)
            } else {
                Position::default()
            }
        };
        

        let icon = 
            if state.windows[i].icon_rgba.is_some()  && state.windows[i].icon_width_height.is_some() {
                    let (width, height) = state.windows[i].icon_width_height.unwrap();
                    let results = 
                    icon::from_rgba(state.windows[i].icon_rgba.clone().unwrap(), width, height);
                    match results {
                        Ok(icon) => Some(icon),
                        Err(err) => panic!("Unable to get windows Icon {}", err)
                    }
                } else { None };


        let (iced_id, open) = window::open(window::Settings {
            size: state.windows[i].size.unwrap_or(def_setting.size),
            maximized: state.windows[i].maximized.unwrap_or(def_setting.maximized),
            fullscreen,
            position,
            min_size: state.windows[i].min_size,
            max_size: state.windows[i].max_size,
            visible,
            resizable: state.windows[i].resizable.unwrap_or(def_setting.resizable),
            minimizable: state.windows[i].minimizable.unwrap_or(def_setting.minimizable),
            closeable: state.windows[i].closeable.unwrap_or(def_setting.closeable),
            decorations: state.windows[i].decorations.unwrap_or(true),
            transparent: state.windows[i].transparent.unwrap_or(def_setting.blur),
            blur: state.windows[i].blur.unwrap_or(def_setting.blur),
            icon,
            level,
            exit_on_close_request: state.windows[i].exit_on_close_request.unwrap_or(def_setting.exit_on_close_request),
            platform_specific: PlatformSpecific::default(),
        });
        
        let id = state.windows[i].id;

        let debug = 
            if let Some(db) = state.windows[i].debug {
                db
            } else { false };

        let theme = if let Some(theme) = &state.windows[i].theme {
            theme.to_iced()
        } else {
            Theme::TokyoNight
        };
       

        state.window_debug.insert(iced_id, (id, debug));
        state.window_theme.insert(iced_id, (id, theme));
        state.window_mode.insert(iced_id, (id, mode));
        state.windows_opened.push(iced_id);
        if !visible {
            state.windows_hidden.push(iced_id);
        }

        let ipg_id = state.windows[i].id;
        state.windows_iced_ipg_ids.insert(iced_id, ipg_id);
        let size = if let Some(sz) = state.windows[i].size {
            sz
        } else {
            Size::new(1024.0, 768.0)
        };
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
            wnd.debug = Some(try_extract_boolean(value, name));
        },
        IpgWindowParam::Theme => {
            wnd.theme = Some(try_extract_ipg_theme(value));
        },
        IpgWindowParam::ScaleFactor => {
            wnd.scale_factor = Some(try_extract_f32(value, name));
        },
        IpgWindowParam::Decorations => {
            let val = try_extract_usize(value, name);
            let mut state = access_window_actions();
            state.decorations.push(val);
            drop(state)
        },
        IpgWindowParam::Level => {
            let ipg_level = try_extract_level(value);
            let level = IpgWindowLevel::to_iced(&ipg_level);
            wnd.level = Some(ipg_level);
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

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgWindow {
    type Param = IpgWindowParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
        match param {
            IpgWindowParam::Debug => set_opt_bool(&mut self.debug, value, name),
            IpgWindowParam::Theme => {
                self.theme = Some(extract_param::<IpgWindowTheme>(value, "WindowTheme"));
            }
            IpgWindowParam::ScaleFactor => set_opt_f32(&mut self.scale_factor, value, name),
            IpgWindowParam::Decorations => {
                let val = try_extract_usize(value, name);
                let mut state = access_window_actions();
                state.decorations.push(val);
                drop(state);
            }
            IpgWindowParam::Level => {
                let ipg_level = try_extract_level(value);
                let level = IpgWindowLevel::to_iced(&ipg_level);
                self.level = Some(ipg_level);
                let mut state = access_window_actions();
                state.level.push((self.id, level));
                drop(state);
            }
            IpgWindowParam::Position => {
                let val = try_extract_vec_f32(value, name);
                let mut state = access_window_actions();
                state.position.push((self.id, val[0], val[1]));
                drop(state);
            }
            IpgWindowParam::Size => {
                let val = try_extract_vec_f32(value, name);
                let mut state = access_window_actions();
                state.resize.push((self.id, val[0], val[1]));
                drop(state);
            }
        }
    }
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

