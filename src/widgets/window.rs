//! Window widget definition

use iced::window::{self, Position};
use iced::{Size, Theme};
use pyo3::pyclass;

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
