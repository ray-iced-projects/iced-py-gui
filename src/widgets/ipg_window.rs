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
    WidgetParamUpdate, extract_param, set_opt_bool, set_opt_f32, set_opt_f32_array_2, set_opt_string, set_opt_u32_array_2, set_opt_vec_u8
};

#[derive(Debug, Clone)]
pub struct IpgWindow {
    pub id: usize,
    pub title: Option<String>,
    pub size: Option<[f32; 2]>,
    pub maximized: Option<bool>,
    pub fullscreen: Option<bool>,
    pub hidden: Option<bool>,
    pub center: Option<bool>,
    pub position: Option<[f32; 2]>,
    pub min_size: Option<[f32; 2]>,
    pub max_size: Option<[f32; 2]>,
    pub theme: Option<IpgWindowTheme>,
    pub resizable: Option<bool>,
    pub minimizable: Option<bool>,
    pub closeable: Option<bool>,
    pub decorations: Option<bool>,
    pub transparent: Option<bool>,
    pub blur: Option<bool>,
    pub level: Option<IpgWindowLevel>,
    pub icon_rgba: Option<Vec<u8>>,
    pub icon_width_height: Option<[u32; 2]>,
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

pub fn add_windows(state: &mut IpgState) -> Vec<Task<Message>> {

    let mut spawn_window: Vec<Task<Message>> = vec![];

    for i in 0..state.windows.len() {
        
        let win = &state.windows[i];
        let is_hidden = win.hidden.unwrap_or(false);
        let is_fullscreen = !is_hidden && win.fullscreen.unwrap_or(false);
        let visible = !is_hidden;
        let mode = if is_hidden {
            window::Mode::Hidden
        } else if is_fullscreen {
            window::Mode::Fullscreen
        } else {
            window::Mode::Windowed
        };
        
        let def_setting = window::Settings::default();
        
        let level = if let Some(lv) = &state.windows[i].level {
            IpgWindowLevel::to_iced(lv)
        } else {
            Level::default()
        };

        let position = if let Some(ct) = state.windows[i].center {
            Position::Centered
        } else {
            if let Some(pos) = state.windows[i].position {
                let point = iced::Point::new(pos[0], pos[1]);
                Position::Specific(point)
            } else {
                Position::default()
            }
        };
        
        let size = if let Some(s) = state.windows[i].size {
            Size::new(s[0], s[1])
        } else {
            def_setting.size
        };

        let min_size = if let Some(s) = state.windows[i].min_size {
            Some(Size::new(s[0], s[1]))
        } else {
            def_setting.min_size
        };

        let max_size = if let Some(s) = state.windows[i].max_size {
            Some(Size::new(s[0], s[1]))
        } else {
            def_setting.max_size
        };

        let icon = 
            if state.windows[i].icon_rgba.is_some()  && state.windows[i].icon_width_height.is_some() {
                    let [width, height] = state.windows[i].icon_width_height.unwrap();
                    let results = 
                    icon::from_rgba(state.windows[i].icon_rgba.clone().unwrap(), width, height);
                    match results {
                        Ok(icon) => Some(icon),
                        Err(err) => panic!("Unable to get windows Icon {}", err)
                    }
                } else { None };


        let (iced_id, open) = window::open(window::Settings {
            size,
            maximized: state.windows[i].maximized.unwrap_or(def_setting.maximized),
            fullscreen: is_fullscreen,
            position,
            min_size,
            max_size,
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
        spawn_window.push(open.map(move|_|Message::WindowOpened(iced_id, None, size)));
        
    }

    spawn_window

}

pub fn construct_window(content: Vec<Element<Message>>) -> Element<Message> {
    Column::with_children(content).into()
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
pub enum IpgWindowParam {
    Center,
    Closeable,
    Debug,
    Decorations,
    ExitOnCloseRequest,
    Fullscreen,
    Hidden,
    IconRgba,
    IconWidthHeight,
    Level,
    MaxSize,
    Maximized,
    MinSize,
    Minimizable,
    Position,
    Resizable,
    ScaleFactor,
    Size,
    Theme,
    Title,
    Transparent,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementation
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for IpgWindow {
    type Param = IpgWindowParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            IpgWindowParam::Center => set_opt_bool(&mut self.center, value, "Center"),
            IpgWindowParam::Closeable => set_opt_bool(&mut self.closeable, value, "Closeable"),
            IpgWindowParam::Debug => set_opt_bool(&mut self.debug, value, "Debug"),
            IpgWindowParam::Decorations => {
                let mut state = access_window_actions();
                state.decorations.push(try_extract_usize(value, "Decorations"));
                drop(state);
            },
            IpgWindowParam::ExitOnCloseRequest => set_opt_bool(&mut self.exit_on_close_request, value, "ExitOnCloseRequest"),
            IpgWindowParam::Fullscreen => {
                set_opt_bool(&mut self.fullscreen, value, "Fullscreen");
                let mode = if self.fullscreen == Some(true) {
                    window::Mode::Fullscreen
                } else {
                    window::Mode::Windowed
                };
                let mut state = access_window_actions();
                state.mode.push((self.id, mode));
                drop(state);
            },
            IpgWindowParam::Hidden => {
                set_opt_bool(&mut self.hidden, value, "Hidden");
                let mode = if self.hidden == Some(true) {
                    window::Mode::Hidden
                } else if self.fullscreen == Some(true) {
                    window::Mode::Fullscreen
                } else {
                    window::Mode::Windowed
                };
                let mut state = access_window_actions();
                state.mode.push((self.id, mode));
                drop(state);
            },
            IpgWindowParam::IconRgba => set_opt_vec_u8(&mut self.icon_rgba, value, "IconRgba"),
            IpgWindowParam::IconWidthHeight => set_opt_u32_array_2(&mut self.icon_width_height, value, "IconWidthHeight"),
            IpgWindowParam::Level => {
                let ipg_level = try_extract_level(value);
                let level = IpgWindowLevel::to_iced(&ipg_level);
                self.level = Some(ipg_level);
                let mut state = access_window_actions();
                state.level.push((self.id, level));
                drop(state);
            },
            IpgWindowParam::MaxSize => set_opt_f32_array_2(&mut self.max_size, value, "MaxSize"),
            IpgWindowParam::Maximized => set_opt_bool(&mut self.maximized, value, "Maximized"),
            IpgWindowParam::MinSize => set_opt_f32_array_2(&mut self.min_size, value, "MinSize"),
            IpgWindowParam::Minimizable => set_opt_bool(&mut self.minimizable, value, "Minimizable"),
            IpgWindowParam::Position => {
                let val = try_extract_vec_f32(value, "Position");
                let mut state = access_window_actions();
                state.position.push((self.id, val[0], val[1]));
                drop(state);
            },
            IpgWindowParam::Resizable => set_opt_bool(&mut self.resizable, value, "Resizable"),
            IpgWindowParam::ScaleFactor => set_opt_f32(&mut self.scale_factor, value, "ScaleFactor"),
            IpgWindowParam::Size => {
                let val = try_extract_vec_f32(value, "Size");
                let mut state = access_window_actions();
                state.resize.push((self.id, val[0], val[1]));
                drop(state);
            },
            IpgWindowParam::Theme => {
                self.theme = Some(extract_param::<IpgWindowTheme>(value));
            },
            IpgWindowParam::Title => set_opt_string(&mut self.title, value, "Title"),
            IpgWindowParam::Transparent => set_opt_bool(&mut self.transparent, value, "Transparent"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::{Python, IntoPyObjectExt};

    fn make_window() -> IpgWindow {
        IpgWindow {
            id: 0,
            title: None,
            size: None,
            maximized: None,
            fullscreen: None,
            hidden: None,
            center: None,
            position: None,
            min_size: None,
            max_size: None,
            theme: None,
            resizable: None,
            minimizable: None,
            closeable: None,
            decorations: None,
            transparent: None,
            blur: None,
            level: None,
            icon_rgba: None,
            icon_width_height: None,
            exit_on_close_request: None,
            scale_factor: None,
            debug: None,
        }
    }

    fn py_obj<T: for<'py> IntoPyObjectExt<'py>>(val: T) -> PyObject {
        Python::initialize();
        Python::attach(|py| val.into_py_any(py).unwrap())
    }

    fn py_none() -> PyObject {
        Python::initialize();
        Python::attach(|py| py.None().into_py_any(py).unwrap())
    }

    #[test]
    fn test_debug() {
        let mut w = make_window();
        w.param_update(IpgWindowParam::Debug, &py_obj(true));
        assert_eq!(w.debug, Some(true));
        w.param_update(IpgWindowParam::Debug, &py_none());
        assert_eq!(w.debug, None);
    }

    #[test]
    fn test_scale_factor() {
        let mut w = make_window();
        w.param_update(IpgWindowParam::ScaleFactor, &py_obj(1.5f32));
        assert_eq!(w.scale_factor, Some(1.5));
        w.param_update(IpgWindowParam::ScaleFactor, &py_none());
        assert_eq!(w.scale_factor, None);
    }
}

