//! Window widget definition
use iced::widget::Column;
use iced::window::settings::PlatformSpecific;
use iced::window::{self, Level, Position, icon};
use iced::{Element, Size, Task, Theme};
use strum::EnumIter;
use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::IpgState;
use crate::app::Message;
use crate::state::access_window_actions;
use crate::widgets::widget_param_update::{
    WidgetParamUpdate, set_t_value
};

#[derive(Debug, Clone)]
pub struct Window {
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
    pub theme: Option<WindowTheme>,
    pub resizable: Option<bool>,
    pub minimizable: Option<bool>,
    pub closeable: Option<bool>,
    pub decorations: Option<bool>,
    pub transparent: Option<bool>,
    pub blur: Option<bool>,
    pub level: Option<WindowLevel>,
    pub icon_rgba: Option<Vec<u8>>,
    pub icon_width_height: Option<[u32; 2]>,
    pub exit_on_close_request: Option<bool>,
    pub scale_factor: Option<f32>,
    pub debug: Option<bool>,
}

// #[derive(Debug, Clone)]
// pub enum WndMessage {
//     TitleChanged(window::Id, String),
//     NewWindow,
//     ScaleInputChanged(window::Id, String),
//     ScaleChanged(window::Id, String), 
// }

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
            WindowLevel::to_iced(lv)
        } else {
            Level::default()
        };

        let position = if let Some(_) = state.windows[i].center {
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

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum WindowParam {
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

impl WidgetParamUpdate for Window {
    type Param = WindowParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            WindowParam::Center => set_t_value(&mut self.center, value, "WindowParam::Center"),
            WindowParam::Closeable => set_t_value(&mut self.closeable, value, "WindowParam::Closeable"),
            WindowParam::Debug => set_t_value(&mut self.debug, value, "WindowParam::Debug"),
            WindowParam::Decorations => {
                set_t_value(&mut self.decorations, value, "WindowParam::Decorations");
                if self.decorations == Some(true) {
                    let mut state = access_window_actions();
                    state.decorations.push(self.id);
                    drop(state);
                }
            },
            WindowParam::ExitOnCloseRequest => set_t_value(&mut self.exit_on_close_request, value, "WindowParam::ExitOnCloseRequest"),
            WindowParam::Fullscreen => {
                set_t_value(&mut self.fullscreen, value, "Fullscreen");
                let mode = if self.fullscreen == Some(true) {
                    window::Mode::Fullscreen
                } else {
                    window::Mode::Windowed
                };
                let mut state = access_window_actions();
                state.mode.push((self.id, mode));
                drop(state);
            },
            WindowParam::Hidden => {
                set_t_value(&mut self.hidden, value, "WindowParam::Hidden");
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
            WindowParam::IconRgba => set_t_value(&mut self.icon_rgba, value, "WindowParam::IconRgba"),
            WindowParam::IconWidthHeight => set_t_value(&mut self.icon_width_height, value, "WindowParam::IconWidthHeight"),
            WindowParam::Level => {
                set_t_value(&mut self.level, value, "WindowParam::Level");
                if let Some(lvl) = &self.level {
                    let level = lvl.to_iced();
                    let mut state = access_window_actions();
                    state.level.push((self.id, level));
                    drop(state);
                }
            },
            WindowParam::MaxSize => set_t_value(&mut self.max_size, value, "WindowParam::MaxSize"),
            WindowParam::Maximized => set_t_value(&mut self.maximized, value, "WindowParam::Maximized"),
            WindowParam::MinSize => set_t_value(&mut self.min_size, value, "WindowParam::MinSize"),
            WindowParam::Minimizable => set_t_value(&mut self.minimizable, value, "WindowParam::Minimizable"),
            WindowParam::Position => {
                set_t_value(&mut self.position, value, "WindowParam::Position");
                if let Some(pos) = self.position {
                    let mut state = access_window_actions();
                    state.position.push((self.id, pos[0], pos[1]));
                    drop(state);
                }
            },
            WindowParam::Resizable => set_t_value(&mut self.resizable, value, "WindowParam::Resizable"),
            WindowParam::ScaleFactor => set_t_value(&mut self.scale_factor, value, "WindowParam::ScaleFactor"),
            WindowParam::Size => {
                set_t_value(&mut self.size, value, "WindowParam::Size");
                if let Some(sz) = self.size {
                    let mut state = access_window_actions();
                    state.resize.push((self.id, sz[0], sz[1]));
                    drop(state);
                }
            },
            WindowParam::Theme => {
                let mut str: String = String::new();
                set_t_value(&mut str, value, "WindowParam::Theme");
                self.theme = Some(string_to_theme(str));
            },
            WindowParam::Title => set_t_value(&mut self.title, value, "WindowParam::Title"),
            WindowParam::Transparent => set_t_value(&mut self.transparent, value, "WindowParam::Transparent"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[derive(EnumIter)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum WindowTheme {
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

impl WindowTheme {
    pub fn to_iced(&self) -> Theme {
        match self {
            WindowTheme::Dark => Theme::Dark,
            WindowTheme::Light => Theme::Light,
            WindowTheme::CatppuccinLatte => Theme::CatppuccinLatte,
            WindowTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            WindowTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            WindowTheme::CatppuccinMocha => Theme::CatppuccinMocha,
            WindowTheme::Dracula => Theme::Dracula,
            WindowTheme::Ferra => Theme::Ferra,
            WindowTheme::GruvboxLight => Theme::GruvboxLight,
            WindowTheme::GruvboxDark => Theme::GruvboxDark,
            WindowTheme::KanagawaWave => Theme::KanagawaWave,
            WindowTheme::KanagawaDragon => Theme::KanagawaDragon,
            WindowTheme::KanagawaLotus => Theme::KanagawaLotus,
            WindowTheme::Moonfly => Theme::Moonfly,
            WindowTheme::Nightfly => Theme::Nightfly,
            WindowTheme::Nord => Theme::Nord,
            WindowTheme::Oxocarbon => Theme::Oxocarbon,
            WindowTheme::SolarizedLight => Theme::SolarizedLight,
            WindowTheme::SolarizedDark => Theme::SolarizedDark,
            WindowTheme::TokyoNight => Theme::TokyoNight,
            WindowTheme::TokyoNightStorm => Theme::TokyoNightStorm,
            WindowTheme::TokyoNightLight => Theme::TokyoNightLight,
        }
    }

}

fn string_to_theme(str_theme: String) -> WindowTheme {
    match str_theme.as_str() {
        "WindowTheme.Dark" => WindowTheme::Dark,
        "WindowTheme.Light" => WindowTheme::Light,
        "WindowTheme.CatppuccinLatte" => WindowTheme::CatppuccinLatte,
        "WindowTheme.CatppuccinFrappe" => WindowTheme::CatppuccinFrappe,
        "WindowTheme.CatppuccinMacchiato" => WindowTheme::CatppuccinMacchiato,
        "WindowTheme.CatppuccinMocha" => WindowTheme::CatppuccinMocha,
        "WindowTheme.Dracula" => WindowTheme::Dracula,
        "WindowTheme.Ferra" => WindowTheme::Ferra,
        "WindowTheme.GruvboxLight" => WindowTheme::GruvboxLight,
        "WindowTheme.GruvboxDark" => WindowTheme::GruvboxDark,
        "WindowTheme.KanagawaWave" => WindowTheme::KanagawaWave,
        "WindowTheme.KanagawaDragon" => WindowTheme::KanagawaDragon,
        "WindowTheme.KanagawaLotus" => WindowTheme::KanagawaLotus,
        "WindowTheme.Moonfly" => WindowTheme::Moonfly,
        "WindowTheme.Nightfly" => WindowTheme::Nightfly,
        "WindowTheme.Nord" => WindowTheme::Nord,
        "WindowTheme.Oxocarbon" => WindowTheme::Oxocarbon,
        "WindowTheme.SolarizedLight" => WindowTheme::SolarizedLight,
        "WindowTheme.SolarizedDark" => WindowTheme::SolarizedDark,
        "WindowTheme.TokyoNight" => WindowTheme::TokyoNight,
        "WindowTheme.TokyoNightStorm" => WindowTheme::TokyoNightStorm,
        "WindowTheme.TokyoNightLight" => WindowTheme::TokyoNightLight,
        _ => {
            eprintln!("Window Theme: {} not found, return default", str_theme);
            WindowTheme::TokyoNight
        }
    }
}


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum WindowLevel {
    Normal,
    AlwaysOnBottom,
    AlwaysOnTop,
}

impl WindowLevel {
    pub fn to_iced(&self) -> window::Level {
        match self {
            WindowLevel::Normal => window::Level::Normal,
            WindowLevel::AlwaysOnBottom => window::Level::AlwaysOnBottom,
            WindowLevel::AlwaysOnTop => window::Level::AlwaysOnTop,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum WindowMode {
    Windowed,
    FullScreen,
    Closed,
}

impl WindowMode {
    pub fn to_iced(&self) -> window::Mode {
        match self {
            WindowMode::Windowed => window::Mode::Windowed,
            WindowMode::FullScreen => window::Mode::Fullscreen,
            WindowMode::Closed => window::Mode::Hidden,
        }
    }
}
