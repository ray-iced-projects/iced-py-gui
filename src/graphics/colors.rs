//! Predefined color palette based on the CSS color palette
//!
//!
//! Thanks to:
//! * [W3 Schools](https://www.w3schools.com/cssref/css_colors.asp)
//! * [Corecoding](https://corecoding.com/utilities/rgb-or-hex-to-float.php)
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
use pyo3::{Py, PyAny, Python, pyclass};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone, Default, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum Color {
    #[default]
    PRIMARY,
    SECONDARY,
    SUCCESS,
    DANGER,
    WARNING,
    INFO,
    LIGHT,
    DARK,
    BACKGROUND_THEME,
    ALICE_BLUE,
    ANTIQUE_WHITE,
    AQUA,
    AQUAMARINE,
    AZURE,
    BEIGE,
    BISQUE,
    BLACK,
    BLANCHED_ALMOND,
    BLUE,
    BLUE_VIOLET,
    BROWN,
    BURLY_WOOD,
    CADET_BLUE,
    CHARTREUSE,
    CHOCOLATE,
    CORAL,
    CORNFLOWER_BLUE,
    CORNSILK,
    CRIMSON,
    CYAN,
    DARK_BLUE,
    DARK_CYAN,
    DARK_GOLDEN_ROD,
    DARK_GRAY,
    DARK_GREY,
    DARK_GREEN,
    DARK_KHAKI,
    DARK_MAGENTA,
    DARK_OLIVE_GREEN,
    DARK_ORANGE,
    DARK_ORCHID,
    DARK_RED,
    DARK_SALMON,
    DARK_SEA_GREEN,
    DARK_SLATE_BLUE,
    DARK_SLATE_GRAY,
    DARK_SLATE_GREY,
    DARK_TURQUOISE,
    DARK_VIOLET,
    DEEP_PINK,
    DEEP_SKY_BLUE,
    DIM_GRAY,
    DIM_GREY,
    DODGER_BLUE,
    FIRE_BRICK,
    FLORAL_WHITE,
    FOREST_GREEN,
    FUCHSIA,
    GAINSBORO,
    GHOST_WHITE,
    GOLD,
    GOLDEN_ROD,
    GRAY,
    GREY,
    GREEN,
    GREEN_YELLOW,
    HONEY_DEW,
    HOT_PINK,
    INDIAN_RED,
    INDIGO,
    IVORY,
    KHAKI,
    LAVENDER,
    LAVENDER_BLUSH,
    LAWN_GREEN,
    LEMON_CHIFFON,
    LIGHT_BLUE,
    LIGHT_CORAL,
    LIGHT_CYAN,
    LIGHT_GOLDEN_ROD_YELLOW,
    LIGHT_GRAY,
    LIGHT_GREY,
    LIGHT_GREEN,
    LIGHT_PINK,
    LIGHT_SALMON,
    LIGHT_SEA_GREEN,
    LIGHT_SKY_BLUE,
    LIGHT_SLATE_GRAY,
    LIGHT_SLATE_GREY,
    LIGHT_STEEL_BLUE,
    LIGHT_YELLOW,
    LIME,
    LIME_GREEN,
    LINEN,
    MAGENTA,
    MAROON,
    MEDIUM_AQUA_MARINE,
    MEDIUM_BLUE,
    MEDIUM_ORCHID,
    MEDIUM_PURPLE,
    MEDIUM_SEA_GREEN,
    MEDIUM_SLATE_BLUE,
    MEDIUM_SPRING_GREEN,
    MEDIUM_TURQUOISE,
    MEDIUM_VIOLET_RED,
    MIDNIGHT_BLUE,
    MINT_CREAM,
    MISTY_ROSE,
    MOCCASIN,
    NAVAJO_WHITE,
    NAVY,
    OLD_LACE,
    OLIVE,
    OLIVE_DRAB,
    ORANGE,
    ORANGE_RED,
    ORCHID,
    PALE_GOLDEN_ROD,
    PALE_GREEN,
    PALE_TURQUOISE,
    PALE_VIOLET_RED,
    PAPAYA_WHIP,
    PEACH_PUFF,
    PERU,
    PINK,
    PLUM,
    POWDER_BLUE,
    PURPLE,
    REBECCA_PURPLE,
    RED,
    ROSY_BROWN,
    ROYAL_BLUE,
    SADDLE_BROWN,
    SALMON,
    SANDY_BROWN,
    SEA_GREEN,
    SEA_SHELL,
    SIENNA,
    SILVER,
    SKY_BLUE,
    SLATE_BLUE,
    SLATE_GRAY,
    SLATE_GREY,
    SNOW,
    SPRING_GREEN,
    STEEL_BLUE,
    TAN,
    TEAL,
    THISTLE,
    TOMATO,
    TRANSPARENT,
    TURQUOISE,
    VIOLET,
    WHEAT,
    WHITE,
    WHITE_SMOKE,
    YELLOW,
    YELLOW_GREEN,
}

impl Color {
    pub fn rgba_ipg_color_to_iced(rgba: Option<[f32; 4]>, color: &Option<Color>, alpha: Option<f32>) -> Option<iced::Color> {
        if let Some(rgba) = rgba {
            Some(iced::Color::from_rgba(rgba[0], rgba[1], rgba[2], rgba[3]))
        } else if let Some(c) = color {
            let color = c.to_iced();
            Some(color.scale_alpha(alpha.unwrap_or(1.0)))
        } else {
            None
        }
    }

    pub fn extract_rgba(value: &PyObject, name: &str) -> [f32; 4] {
        Python::attach(|py| {

            let res = value.extract::<[f32; 4]>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("{}-Unable to extract python object for RGBA color", name),
            }
        })
    }

    pub fn extract_rgba_opt(value: &PyObject, name: &str) -> Option<[f32; 4]> {
        Python::attach(|py| {

            let res = value.extract::<Option<Vec<f32>>>(py);
            match res {
                Ok(val) => {
                    if val.is_none() { return None };
                    match val.unwrap().try_into() {
                        Ok(arr) => Some(arr),
                        Err(_) => panic!("{} The RGBA value need to have an array of 4 floats ", name)
                    }
                },
                Err(_) => panic!("{}-Unable to extract python object for optional RGBA color", name),
            }
        })
    }

    pub fn extract(value: &PyObject, name: &str) -> Color {
        Python::attach(|py| {

            let res = value.extract::<Color>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("{}-Unable to extract python object for IpgColor", name),
            }
        })
    }

    pub fn extract_opt(value: &PyObject, name: &str) -> Option<Color> {
        Python::attach(|py| {

            let res = value.extract::<Option<Color>>(py);
            match res {
                Ok(val) => val,
                Err(_) => panic!("{}-Unable to extract python object for optional IpgColor", name),
            }
        })
    }

    fn to_iced(&self) -> iced::Color {
        match self {
            Color::PRIMARY => PRIMARY,
            Color::SECONDARY => SECONDARY,
            Color::SUCCESS => SUCCESS,
            Color::DANGER => DANGER,
            Color::WARNING => WARNING,
            Color::INFO => INFO,
            Color::LIGHT => LIGHT,
            Color::DARK => DARK,
            Color::BACKGROUND_THEME => BACKGROUND_THEME,
            Color::ALICE_BLUE => ALICE_BLUE,
            Color::ANTIQUE_WHITE => ANTIQUE_WHITE,
            Color::AQUA => AQUA,
            Color::AQUAMARINE => AQUAMARINE,
            Color::AZURE => AZURE,
            Color::BEIGE => BEIGE,
            Color::BISQUE => BISQUE,
            Color::BLACK => BLACK,
            Color::BLANCHED_ALMOND => BLANCHED_ALMOND,
            Color::BLUE => BLUE,
            Color::BLUE_VIOLET => BLUE_VIOLET,
            Color::BROWN => BROWN,
            Color::BURLY_WOOD => BURLY_WOOD,
            Color::CADET_BLUE => CADET_BLUE,
            Color::CHARTREUSE => CHARTREUSE,
            Color::CHOCOLATE => CHOCOLATE,
            Color::CORAL => CORAL,
            Color::CORNFLOWER_BLUE => CORNFLOWER_BLUE,
            Color::CORNSILK => CORNSILK,
            Color::CRIMSON => CRIMSON,
            Color::CYAN => CYAN,
            Color::DARK_BLUE => DARK_BLUE,
            Color::DARK_CYAN => DARK_CYAN,
            Color::DARK_GOLDEN_ROD => DARK_GOLDEN_ROD,
            Color::DARK_GRAY => DARK_GRAY,
            Color::DARK_GREY => DARK_GREY,
            Color::DARK_GREEN => DARK_GREEN,
            Color::DARK_KHAKI => DARK_KHAKI,
            Color::DARK_MAGENTA => DARK_MAGENTA,
            Color::DARK_OLIVE_GREEN => DARK_OLIVE_GREEN,
            Color::DARK_ORANGE => DARK_ORANGE,
            Color::DARK_ORCHID => DARK_ORCHID,
            Color::DARK_RED => DARK_RED,
            Color::DARK_SALMON => DARK_SALMON,
            Color::DARK_SEA_GREEN => DARK_SEA_GREEN,
            Color::DARK_SLATE_BLUE => DARK_SLATE_BLUE,
            Color::DARK_SLATE_GRAY => DARK_SLATE_GRAY,
            Color::DARK_SLATE_GREY => DARK_SLATE_GREY,
            Color::DARK_TURQUOISE => DARK_TURQUOISE,
            Color::DARK_VIOLET => DARK_VIOLET,
            Color::DEEP_PINK => DEEP_PINK,
            Color::DEEP_SKY_BLUE => DEEP_SKY_BLUE,
            Color::DIM_GRAY => DIM_GRAY,
            Color::DIM_GREY => DIM_GREY,
            Color::DODGER_BLUE => DODGER_BLUE,
            Color::FIRE_BRICK => FIRE_BRICK,
            Color::FLORAL_WHITE => FLORAL_WHITE,
            Color::FOREST_GREEN => FOREST_GREEN,
            Color::FUCHSIA => FUCHSIA,
            Color::GAINSBORO => GAINSBORO,
            Color::GHOST_WHITE => GHOST_WHITE,
            Color::GOLD => GOLD,
            Color::GOLDEN_ROD => GOLDEN_ROD,
            Color::GRAY => GRAY,
            Color::GREY => GREY,
            Color::GREEN => GREEN,
            Color::GREEN_YELLOW => GREEN_YELLOW,
            Color::HONEY_DEW => HONEY_DEW,
            Color::HOT_PINK => HOT_PINK,
            Color::INDIAN_RED => INDIAN_RED,
            Color::INDIGO => INDIGO,
            Color::IVORY => IVORY,
            Color::KHAKI => KHAKI,
            Color::LAVENDER => LAVENDER,
            Color::LAVENDER_BLUSH => LAVENDER_BLUSH,
            Color::LAWN_GREEN => LAWN_GREEN,
            Color::LEMON_CHIFFON => LEMON_CHIFFON,
            Color::LIGHT_BLUE => LIGHT_BLUE,
            Color::LIGHT_CORAL => LIGHT_CORAL,
            Color::LIGHT_CYAN => LIGHT_CYAN,
            Color::LIGHT_GOLDEN_ROD_YELLOW => LIGHT_GOLDEN_ROD_YELLOW,
            Color::LIGHT_GRAY => LIGHT_GRAY,
            Color::LIGHT_GREY => LIGHT_GREY,
            Color::LIGHT_GREEN => LIGHT_GREEN,
            Color::LIGHT_PINK => LIGHT_PINK,
            Color::LIGHT_SALMON => LIGHT_SALMON,
            Color::LIGHT_SEA_GREEN => LIGHT_SEA_GREEN,
            Color::LIGHT_SKY_BLUE => LIGHT_SKY_BLUE,
            Color::LIGHT_SLATE_GRAY => LIGHT_SLATE_GRAY,
            Color::LIGHT_SLATE_GREY => LIGHT_SLATE_GREY,
            Color::LIGHT_STEEL_BLUE => LIGHT_STEEL_BLUE,
            Color::LIGHT_YELLOW => LIGHT_YELLOW,
            Color::LIME => LIME,
            Color::LIME_GREEN => LIME_GREEN,
            Color::LINEN => LINEN,
            Color::MAGENTA => MAGENTA,
            Color::MAROON => MAROON,
            Color::MEDIUM_AQUA_MARINE => MEDIUM_AQUA_MARINE,
            Color::MEDIUM_BLUE => MEDIUM_BLUE,
            Color::MEDIUM_ORCHID => MEDIUM_ORCHID,
            Color::MEDIUM_PURPLE => MEDIUM_PURPLE,
            Color::MEDIUM_SEA_GREEN => MEDIUM_SEA_GREEN,
            Color::MEDIUM_SLATE_BLUE => MEDIUM_SLATE_BLUE,
            Color::MEDIUM_SPRING_GREEN => MEDIUM_SPRING_GREEN,
            Color::MEDIUM_TURQUOISE => MEDIUM_TURQUOISE,
            Color::MEDIUM_VIOLET_RED => MEDIUM_VIOLET_RED,
            Color::MIDNIGHT_BLUE => MIDNIGHT_BLUE,
            Color::MINT_CREAM => MINT_CREAM,
            Color::MISTY_ROSE => MISTY_ROSE,
            Color::MOCCASIN => MOCCASIN,
            Color::NAVAJO_WHITE => NAVAJO_WHITE,
            Color::NAVY => NAVY,
            Color::OLD_LACE => OLD_LACE,
            Color::OLIVE => OLIVE,
            Color::OLIVE_DRAB => OLIVE_DRAB,
            Color::ORANGE => ORANGE,
            Color::ORANGE_RED => ORANGE_RED,
            Color::ORCHID => ORCHID,
            Color::PALE_GOLDEN_ROD => PALE_GOLDEN_ROD,
            Color::PALE_GREEN => PALE_GREEN,
            Color::PALE_TURQUOISE => PALE_TURQUOISE,
            Color::PALE_VIOLET_RED => PALE_VIOLET_RED,
            Color::PAPAYA_WHIP => PAPAYA_WHIP,
            Color::PEACH_PUFF => PEACH_PUFF,
            Color::PERU => PERU,
            Color::PINK => PINK,
            Color::PLUM => PLUM,
            Color::POWDER_BLUE => POWDER_BLUE,
            Color::PURPLE => PURPLE,
            Color::REBECCA_PURPLE => REBECCA_PURPLE,
            Color::RED => RED,
            Color::ROSY_BROWN => ROSY_BROWN,
            Color::ROYAL_BLUE => ROYAL_BLUE,
            Color::SADDLE_BROWN => SADDLE_BROWN,
            Color::SALMON => SALMON,
            Color::SANDY_BROWN => SANDY_BROWN,
            Color::SEA_GREEN => SEA_GREEN,
            Color::SEA_SHELL => SEA_SHELL,
            Color::SIENNA => SIENNA,
            Color::SILVER => SILVER,
            Color::SKY_BLUE => SKY_BLUE,
            Color::SLATE_BLUE => SLATE_BLUE,
            Color::SLATE_GRAY => SLATE_GRAY,
            Color::SLATE_GREY => SLATE_GREY,
            Color::SNOW => SNOW,
            Color::SPRING_GREEN => SPRING_GREEN,
            Color::STEEL_BLUE => STEEL_BLUE,
            Color::TAN => TAN,
            Color::TEAL => TEAL,
            Color::THISTLE => THISTLE,
            Color::TOMATO => TOMATO,
            Color::TRANSPARENT => iced::Color::TRANSPARENT,
            Color::TURQUOISE => TURQUOISE,
            Color::VIOLET => VIOLET,
            Color::WHEAT => WHEAT,
            Color::WHITE => WHITE,
            Color::WHITE_SMOKE => WHITE_SMOKE,
            Color::YELLOW => YELLOW,
            Color::YELLOW_GREEN => YELLOW_GREEN,
        }
    }
}

/// Primary <span style="color:dodgerblue">iced::Color</span>.
pub const PRIMARY: iced::Color = DODGER_BLUE;

/// Secondary <span style="color:dimgray">iced::Color</span>.
pub const SECONDARY: iced::Color = DIM_GRAY;

/// Success <span style="color:limeGreen">iced::Color</span>.
pub const SUCCESS: iced::Color = LIME_GREEN;

/// Danger <span style="color:red">iced::Color</span>.
pub const DANGER: iced::Color = RED;

/// Warning <span style="color:gold">iced::Color</span>.
pub const WARNING: iced::Color = GOLD;

/// Info <span style="color:skyBlue">iced::Color</span>.
pub const INFO: iced::Color = SKY_BLUE;

/// Light <span style="color:ghostWhite">iced::Color</span>.
pub const LIGHT: iced::Color = GHOST_WHITE;

/// Dark <span style="color:rgb(0.204, 0.227, 0.251)">iced::Color</span>.
pub const DARK: iced::Color = iced::Color::from_rgb(0.204, 0.227, 0.251);

/// Background Theme, color not used but Background Theme tested for.
pub const BACKGROUND_THEME: iced::Color = iced::Color::from_rgb(0.204, 0.227, 0.251);

/// Alice Blue <span style="color:aliceBlue">iced::Color</span>.
pub const ALICE_BLUE: iced::Color = iced::Color::from_rgb(0.941, 0.973, 1.0);

/// Antique White <span style="color:antiqueWhite">iced::Color</span>.
pub const ANTIQUE_WHITE: iced::Color = iced::Color::from_rgb(0.98, 0.922, 0.843);

/// Aqua <span style="color:aqua">iced::Color</span>.
pub const AQUA: iced::Color = iced::Color::from_rgb(0.0, 1.0, 1.0);

/// Aquamarine <span style="color:aquamarine">iced::Color</span>.
pub const AQUAMARINE: iced::Color = iced::Color::from_rgb(0.498, 1.0, 0.831);

/// Azure <span style="color:azure">iced::Color</span>.
pub const AZURE: iced::Color = iced::Color::from_rgb(0.941, 1.0, 1.0);

/// Beige <span style="color:beige">iced::Color</span>.
pub const BEIGE: iced::Color = iced::Color::from_rgb(0.961, 0.961, 0.863);

/// Bisque <span style="color:bisque">iced::Color</span>.
pub const BISQUE: iced::Color = iced::Color::from_rgb(1.0, 0.894, 0.769);

/// Black <span style="color:black">iced::Color</span>.
pub const BLACK: iced::Color = iced::Color::BLACK;

/// Blanched Almond <span style="color:blanchedAlmond">iced::Color</span>.
pub const BLANCHED_ALMOND: iced::Color = iced::Color::from_rgb(1.0, 0.922, 0.804);

/// Blue <span style="color:blue">iced::Color</span>.
pub const BLUE: iced::Color = iced::Color::from_rgb(0.0, 0.0, 1.0);

/// Blue Violet <span style="color:blueViolet">iced::Color</span>.
pub const BLUE_VIOLET: iced::Color = iced::Color::from_rgb(0.541, 0.169, 0.886);

/// Brown <span style="color:brown">iced::Color</span>.
pub const BROWN: iced::Color = iced::Color::from_rgb(0.647, 0.165, 0.165);

/// Burly Wood <span style="color:burlyWood">iced::Color</span>.
pub const BURLY_WOOD: iced::Color = iced::Color::from_rgb(0.871, 0.722, 0.529);

/// Cadet Blue <span style="color:cadetBlue">iced::Color</span>.
pub const CADET_BLUE: iced::Color = iced::Color::from_rgb(0.373, 0.62, 0.627);

/// Chartreuse <span style="color:chartreuse">iced::Color</span>.
pub const CHARTREUSE: iced::Color = iced::Color::from_rgb(0.498, 1.0, 0.0);

/// Chocolate <span style="color:chocolate">iced::Color</span>.
pub const CHOCOLATE: iced::Color = iced::Color::from_rgb(0.824, 0.412, 0.118);

/// Coral <span style="color:coral">iced::Color</span>.
pub const CORAL: iced::Color = iced::Color::from_rgb(1.0, 0.498, 0.314);

/// Cornflower Blue <span style="color:cornflowerBlue">iced::Color</span>.
pub const CORNFLOWER_BLUE: iced::Color = iced::Color::from_rgb(0.392, 0.584, 0.929);

/// Cornsilk <span style="color:cornsilk">iced::Color</span>.
pub const CORNSILK: iced::Color = iced::Color::from_rgb(1.0, 0.973, 0.863);

/// Crimson <span style="color:crimson">iced::Color</span>.
pub const CRIMSON: iced::Color = iced::Color::from_rgb(0.863, 0.078, 0.235);

/// Cyan <span style="color:cyan">iced::Color</span>.
pub const CYAN: iced::Color = iced::Color::from_rgb(0.0, 1.0, 1.0);

/// Dark Blue <span style="color:darkBlue">iced::Color</span>.
pub const DARK_BLUE: iced::Color = iced::Color::from_rgb(0.0, 0.0, 0.545);

/// Dark Cyan <span style="color:darkCyan">iced::Color</span>.
pub const DARK_CYAN: iced::Color = iced::Color::from_rgb(0.0, 0.545, 0.545);

/// Dark Golden Rod <span style="color:darkGoldenRod">iced::Color</span>.
pub const DARK_GOLDEN_ROD: iced::Color = iced::Color::from_rgb(0.722, 0.525, 0.043);

/// Dark Gray <span style="color:darkGray">iced::Color</span>.
pub const DARK_GRAY: iced::Color = iced::Color::from_rgb(0.663, 0.663, 0.663);

/// Dark Grey <span style="color:darkGrey">iced::Color</span>.
pub const DARK_GREY: iced::Color = DARK_GRAY;

/// Dark Green <span style="color:darkGreen">iced::Color</span>.
pub const DARK_GREEN: iced::Color = iced::Color::from_rgb(0.0, 0.392, 0.0);

/// Dark Khaki <span style="color:darkKhaki">iced::Color</span>.
pub const DARK_KHAKI: iced::Color = iced::Color::from_rgb(0.741, 0.718, 0.42);

/// Dark Magenta <span style="color:darkMagenta">iced::Color</span>.
pub const DARK_MAGENTA: iced::Color = iced::Color::from_rgb(0.545, 0.0, 0.545);

/// Dark Olive Green <span style="color:darkOliveGreen">iced::Color</span>.
pub const DARK_OLIVE_GREEN: iced::Color = iced::Color::from_rgb(0.333, 0.42, 0.184);

/// Dark Orange <span style="color:darkOrange">iced::Color</span>.
pub const DARK_ORANGE: iced::Color = iced::Color::from_rgb(1.0, 0.549, 0.0);

/// Dark Orchid <span style="color:darkOrchid">iced::Color</span>.
pub const DARK_ORCHID: iced::Color = iced::Color::from_rgb(0.6, 0.196, 0.8);

/// Dark Red <span style="color:darkRed">iced::Color</span>.
pub const DARK_RED: iced::Color = iced::Color::from_rgb(0.545, 0.0, 0.0);

/// Dark Salmon <span style="color:darkSalmon">iced::Color</span>.
pub const DARK_SALMON: iced::Color = iced::Color::from_rgb(0.914, 0.588, 0.478);

/// Dark Sea Green <span style="color:darkSeaGreen">iced::Color</span>.
pub const DARK_SEA_GREEN: iced::Color = iced::Color::from_rgb(0.561, 0.737, 0.561);

/// Dark Slate Blue <span style="color:darkSlateBlue">iced::Color</span>.
pub const DARK_SLATE_BLUE: iced::Color = iced::Color::from_rgb(0.282, 0.239, 0.545);

/// Dark Slate Gray <span style="color:darkSlateGray">iced::Color</span>.
pub const DARK_SLATE_GRAY: iced::Color = iced::Color::from_rgb(0.184, 0.31, 0.31);

/// Dark Slate Grey <span style="color:darkSlateGrey">iced::Color</span>.
pub const DARK_SLATE_GREY: iced::Color = DARK_SLATE_GRAY;

/// Dark Turquoise <span style="color:darkTurquoise">iced::Color</span>.
pub const DARK_TURQUOISE: iced::Color = iced::Color::from_rgb(0.0, 0.808, 0.82);

/// Dark Violet <span style="color:darkViolet">iced::Color</span>.
pub const DARK_VIOLET: iced::Color = iced::Color::from_rgb(0.58, 0.0, 0.827);

/// Deep Pink <span style="color:deepPink">iced::Color</span>.
pub const DEEP_PINK: iced::Color = iced::Color::from_rgb(1.0, 0.078, 0.576);

/// Deep Sky Blue <span style="color:deepSkyBlue">iced::Color</span>.
pub const DEEP_SKY_BLUE: iced::Color = iced::Color::from_rgb(0.0, 0.749, 1.0);

/// Dim Gray <span style="color:dimgray">iced::Color</span>.
pub const DIM_GRAY: iced::Color = iced::Color::from_rgb(0.412, 0.412, 0.412);

/// Dim Grey <span style="color:dimgrey">iced::Color</span>.
pub const DIM_GREY: iced::Color = DIM_GRAY;

/// Dodger Blue <span style="color:dodgerBlue">iced::Color</span>.
pub const DODGER_BLUE: iced::Color = iced::Color::from_rgb(0.118, 0.565, 1.0);

/// Fire Brick <span style="color:fireBrick">iced::Color</span>.
pub const FIRE_BRICK: iced::Color = iced::Color::from_rgb(0.698, 0.133, 0.133);

/// Floral White <span style="color:floralWhite">iced::Color</span>.
pub const FLORAL_WHITE: iced::Color = iced::Color::from_rgb(1.0, 0.98, 0.941);

/// Forest Green <span style="color:forestGreen">iced::Color</span>.
pub const FOREST_GREEN: iced::Color = iced::Color::from_rgb(0.133, 0.545, 0.133);

/// Fuchsia <span style="color:fuchsia">iced::Color</span>.
pub const FUCHSIA: iced::Color = iced::Color::from_rgb(1.0, 0.0, 1.0);

/// Gainsboro <span style="color:gainsboro">iced::Color</span>.
pub const GAINSBORO: iced::Color = iced::Color::from_rgb(0.863, 0.863, 0.863);

/// Ghost White <span style="color:ghostWhite">iced::Color</span>.
pub const GHOST_WHITE: iced::Color = iced::Color::from_rgb(0.973, 0.973, 1.0);

/// Gold <span style="color:gold">iced::Color</span>.
pub const GOLD: iced::Color = iced::Color::from_rgb(1.0, 0.843, 0.0);

/// Golden Rod <span style="color:goldenRod">iced::Color</span>.
pub const GOLDEN_ROD: iced::Color = iced::Color::from_rgb(0.855, 0.647, 0.125);

/// Gray <span style="color:gray">iced::Color</span>.
pub const GRAY: iced::Color = iced::Color::from_rgb(0.502, 0.502, 0.502);

/// Grey <span style="color:grey">iced::Color</span>.
pub const GREY: iced::Color = GRAY;

/// Green <span style="color:green">iced::Color</span>.
pub const GREEN: iced::Color = iced::Color::from_rgb(0.0, 0.502, 0.0);

/// Green Yellow <span style="color:greenYellow">iced::Color</span>.
pub const GREEN_YELLOW: iced::Color = iced::Color::from_rgb(0.678, 1.0, 0.184);

/// Honey Dew <span style="color:honeyDew">iced::Color</span>.
pub const HONEY_DEW: iced::Color = iced::Color::from_rgb(0.941, 1.0, 0.941);

/// Hot Pink <span style="color:hotPink">iced::Color</span>.
pub const HOT_PINK: iced::Color = iced::Color::from_rgb(1.0, 0.412, 0.706);

/// Indian Red <span style="color:indianRed">iced::Color</span>.
pub const INDIAN_RED: iced::Color = iced::Color::from_rgb(0.804, 0.361, 0.361);

/// Indigo <span style="color:indigo">iced::Color</span>.
pub const INDIGO: iced::Color = iced::Color::from_rgb(0.294, 0.0, 0.51);

/// Ivory <span style="color:ivory">iced::Color</span>.
pub const IVORY: iced::Color = iced::Color::from_rgb(1.0, 1.0, 0.941);

/// Khaki <span style="color:khaki">iced::Color</span>.
pub const KHAKI: iced::Color = iced::Color::from_rgb(0.941, 0.902, 0.549);

/// Lavender <span style="color:lavender">iced::Color</span>.
pub const LAVENDER: iced::Color = iced::Color::from_rgb(0.902, 0.902, 0.98);

/// Lavender Blush <span style="color:lavenderBlush">iced::Color</span>.
pub const LAVENDER_BLUSH: iced::Color = iced::Color::from_rgb(1.0, 0.941, 0.961);

/// Lawn Green <span style="color:lawnGreen">iced::Color</span>.
pub const LAWN_GREEN: iced::Color = iced::Color::from_rgb(0.486, 0.988, 0.0);

/// Lemon Chiffon <span style="color:lemonChiffon">iced::Color</span>.
pub const LEMON_CHIFFON: iced::Color = iced::Color::from_rgb(1.0, 0.98, 0.804);

/// Light Blue <span style="color:lightBlue">iced::Color</span>.
pub const LIGHT_BLUE: iced::Color = iced::Color::from_rgb(0.678, 0.847, 0.902);

/// Light Coral <span style="color:lightCoral">iced::Color</span>.
pub const LIGHT_CORAL: iced::Color = iced::Color::from_rgb(0.941, 0.502, 0.502);

/// Light Cyan <span style="color:lightCyan">iced::Color</span>.
pub const LIGHT_CYAN: iced::Color = iced::Color::from_rgb(0.878, 1.0, 1.0);

/// Light Golden Rod Yellow <span style="color:lightGoldenRodYellow">iced::Color</span>.
pub const LIGHT_GOLDEN_ROD_YELLOW: iced::Color = iced::Color::from_rgb(0.98, 0.98, 0.824);

/// Light Gray <span style="color:lightGray">iced::Color</span>.
pub const LIGHT_GRAY: iced::Color = iced::Color::from_rgb(0.827, 0.827, 0.827);

/// Light Grey <span style="color:lightGrey">iced::Color</span>.
pub const LIGHT_GREY: iced::Color = LIGHT_GRAY;

/// Light Green <span style="color:lightGreen">iced::Color</span>.
pub const LIGHT_GREEN: iced::Color = iced::Color::from_rgb(0.565, 0.933, 0.565);

/// Light Pink <span style="color:lightPink">iced::Color</span>.
pub const LIGHT_PINK: iced::Color = iced::Color::from_rgb(1.0, 0.714, 0.757);

/// Light Salmon <span style="color:lightSalmon">iced::Color</span>.
pub const LIGHT_SALMON: iced::Color = iced::Color::from_rgb(1.0, 0.627, 0.478);

/// Light Sea Green <span style="color:lightSeaGreen">iced::Color</span>.
pub const LIGHT_SEA_GREEN: iced::Color = iced::Color::from_rgb(0.125, 0.698, 0.667);

/// Light Sky Blue <span style="color:lightSkyBlue">iced::Color</span>.
pub const LIGHT_SKY_BLUE: iced::Color = iced::Color::from_rgb(0.529, 0.808, 0.98);

/// Light Slate Gray <span style="color:lightSlateGray">iced::Color</span>.
pub const LIGHT_SLATE_GRAY: iced::Color = iced::Color::from_rgb(0.467, 0.533, 0.6);

/// Light Slate Grey <span style="color:lightSlateGrey">iced::Color</span>.
pub const LIGHT_SLATE_GREY: iced::Color = LIGHT_SLATE_GRAY;

/// Light Steel Blue <span style="color:lightSteelBlue">iced::Color</span>.
pub const LIGHT_STEEL_BLUE: iced::Color = iced::Color::from_rgb(0.69, 0.769, 0.871);

/// Light Yellow <span style="color:lightYellow">iced::Color</span>.
pub const LIGHT_YELLOW: iced::Color = iced::Color::from_rgb(1.0, 1.0, 0.878);

/// Lime <span style="color:lime">iced::Color</span>.
pub const LIME: iced::Color = iced::Color::from_rgb(0.0, 1.0, 0.0);

/// Lime Green <span style="color:limeGreen">iced::Color</span>.
pub const LIME_GREEN: iced::Color = iced::Color::from_rgb(0.196, 0.804, 0.196);

/// Linen <span style="color:linen">iced::Color</span>.
pub const LINEN: iced::Color = iced::Color::from_rgb(0.98, 0.941, 0.902);

/// Magenta <span style="color:magenta">iced::Color</span>.
pub const MAGENTA: iced::Color = iced::Color::from_rgb(1.0, 0.0, 1.0);

/// Maroon <span style="color:maroon">iced::Color</span>.
pub const MAROON: iced::Color = iced::Color::from_rgb(0.502, 0.0, 0.0);

/// Medium Aqua Marine <span style="color:mediumAquaMarine">iced::Color</span>.
pub const MEDIUM_AQUA_MARINE: iced::Color = iced::Color::from_rgb(0.4, 0.804, 0.667);

/// Medium Blue <span style="color:mediumBlue">iced::Color</span>.
pub const MEDIUM_BLUE: iced::Color = iced::Color::from_rgb(0.0, 0.0, 0.804);

/// Medium Orchid <span style="color:mediumOrchid">iced::Color</span>.
pub const MEDIUM_ORCHID: iced::Color = iced::Color::from_rgb(0.729, 0.333, 0.827);

/// Medium Purple <span style="color:mediumPurple">iced::Color</span>.
pub const MEDIUM_PURPLE: iced::Color = iced::Color::from_rgb(0.576, 0.439, 0.859);

/// Medium Sea Green <span style="color:mediumSeaGreen">iced::Color</span>.
pub const MEDIUM_SEA_GREEN: iced::Color = iced::Color::from_rgb(0.235, 0.702, 0.443);

/// Medium Slate Blue <span style="color:mediumSlateBlue">iced::Color</span>.
pub const MEDIUM_SLATE_BLUE: iced::Color = iced::Color::from_rgb(0.482, 0.408, 0.933);

/// Medium Spring Green <span style="color:mediumSpringGreen">iced::Color</span>.
pub const MEDIUM_SPRING_GREEN: iced::Color = iced::Color::from_rgb(0.0, 0.98, 0.604);

/// Medium Turquoise <span style="color:mediumTurquoise">iced::Color</span>.
pub const MEDIUM_TURQUOISE: iced::Color = iced::Color::from_rgb(0.282, 0.82, 0.8);

/// Medium Violet Red <span style="color:mediumVioletRed">iced::Color</span>.
pub const MEDIUM_VIOLET_RED: iced::Color = iced::Color::from_rgb(0.78, 0.082, 0.522);

/// Midnight Blue <span style="color:midnightBlue">iced::Color</span>.
pub const MIDNIGHT_BLUE: iced::Color = iced::Color::from_rgb(0.098, 0.098, 0.439);

/// Mint Cream <span style="color:mintCream">iced::Color</span>.
pub const MINT_CREAM: iced::Color = iced::Color::from_rgb(0.961, 1.0, 0.98);

/// Misty Rose <span style="color:mistyRose">iced::Color</span>.
pub const MISTY_ROSE: iced::Color = iced::Color::from_rgb(1.0, 0.894, 0.882);

/// Moccasin <span style="color:moccasin">iced::Color</span>.
pub const MOCCASIN: iced::Color = iced::Color::from_rgb(1.0, 0.894, 0.71);

/// Navajo White <span style="color:navajo_white">iced::Color</span>.
pub const NAVAJO_WHITE: iced::Color = iced::Color::from_rgb(1.0, 0.871, 0.678);

/// Navy <span style="color:navy">iced::Color</span>.
pub const NAVY: iced::Color = iced::Color::from_rgb(0.0, 0.0, 0.502);

/// Old Lace <span style="color:oldLace">iced::Color</span>.
pub const OLD_LACE: iced::Color = iced::Color::from_rgb(0.992, 0.961, 0.902);

/// Olive <span style="color:olive">iced::Color</span>.
pub const OLIVE: iced::Color = iced::Color::from_rgb(0.502, 0.502, 0.0);

/// Olive Drab <span style="color:oliveDrab">iced::Color</span>.
pub const OLIVE_DRAB: iced::Color = iced::Color::from_rgb(0.42, 0.557, 0.137);

/// Orange <span style="color:orange">iced::Color</span>.
pub const ORANGE: iced::Color = iced::Color::from_rgb(1.0, 0.647, 0.0);

/// Orange Red <span style="color:orangeRed">iced::Color</span>.
pub const ORANGE_RED: iced::Color = iced::Color::from_rgb(1.0, 0.271, 0.0);

/// Orchid <span style="color:orchid">iced::Color</span>.
pub const ORCHID: iced::Color = iced::Color::from_rgb(0.855, 0.439, 0.839);

/// Pale Golden Rod <span style="color:paleGoldenRod">iced::Color</span>.
pub const PALE_GOLDEN_ROD: iced::Color = iced::Color::from_rgb(0.933, 0.91, 0.667);

/// Pale Green <span style="color:paleGreen">iced::Color</span>.
pub const PALE_GREEN: iced::Color = iced::Color::from_rgb(0.596, 0.984, 0.596);

/// Pale Turquoise <span style="color:paleTurquoise">iced::Color</span>.
pub const PALE_TURQUOISE: iced::Color = iced::Color::from_rgb(0.686, 0.933, 0.933);

/// Pale Violet Red <span style="color:paleVioletRed">iced::Color</span>.
pub const PALE_VIOLET_RED: iced::Color = iced::Color::from_rgb(0.859, 0.439, 0.576);

/// Papaya Whip <span style="color:papayaWhip">iced::Color</span>.
pub const PAPAYA_WHIP: iced::Color = iced::Color::from_rgb(1.0, 0.937, 0.835);

/// Peach Puff <span style="color:peachPuff">iced::Color</span>.
pub const PEACH_PUFF: iced::Color = iced::Color::from_rgb(1.0, 0.855, 0.725);

/// Peru <span style="color:peru">iced::Color</span>.
pub const PERU: iced::Color = iced::Color::from_rgb(0.804, 0.522, 0.247);

/// Pink <span style="color:pink">iced::Color</span>.
pub const PINK: iced::Color = iced::Color::from_rgb(1.0, 0.753, 0.796);

/// Plum <span style="color:plum">iced::Color</span>.
pub const PLUM: iced::Color = iced::Color::from_rgb(0.867, 0.627, 0.867);

/// Powder Blue <span style="color:powderBlue">iced::Color</span>.
pub const POWDER_BLUE: iced::Color = iced::Color::from_rgb(0.69, 0.878, 0.902);

/// Purple <span style="color:purple">iced::Color</span>.
pub const PURPLE: iced::Color = iced::Color::from_rgb(0.502, 0.0, 0.502);

/// Rebecca Purple <span style="color:rebeccaPurple">iced::Color</span>.
pub const REBECCA_PURPLE: iced::Color = iced::Color::from_rgb(0.4, 0.2, 0.6);

/// Red <span style="color:red">iced::Color</span>.
pub const RED: iced::Color = iced::Color::from_rgb(1.0, 0.0, 0.0);

/// Rosy Brown <span style="color:rosyBrown">iced::Color</span>.
pub const ROSY_BROWN: iced::Color = iced::Color::from_rgb(0.737, 0.561, 0.561);

/// Royal Blue <span style="color:royalBlue">iced::Color</span>.
pub const ROYAL_BLUE: iced::Color = iced::Color::from_rgb(0.255, 0.412, 0.882);

/// Saddle Brown <span style="color:saddleBrown">iced::Color</span>.
pub const SADDLE_BROWN: iced::Color = iced::Color::from_rgb(0.545, 0.271, 0.075);

/// Salmon <span style="color:salmon">iced::Color</span>.
pub const SALMON: iced::Color = iced::Color::from_rgb(0.98, 0.502, 0.447);

/// Sandy Brown <span style="color:sandyBrown">iced::Color</span>.
pub const SANDY_BROWN: iced::Color = iced::Color::from_rgb(0.957, 0.643, 0.376);

/// Sea Green <span style="color:seaGreen">iced::Color</span>.
pub const SEA_GREEN: iced::Color = iced::Color::from_rgb(0.18, 0.545, 0.341);

/// Sea Shell <span style="color:seaShell">iced::Color</span>.
pub const SEA_SHELL: iced::Color = iced::Color::from_rgb(1.0, 0.961, 0.933);

/// Sienna <span style="color:sienna">iced::Color</span>.
pub const SIENNA: iced::Color = iced::Color::from_rgb(0.627, 0.322, 0.176);

/// Silver <span style="color:silver">iced::Color</span>.
pub const SILVER: iced::Color = iced::Color::from_rgb(0.753, 0.753, 0.753);

/// Sky Blue <span style="color:skyBlue">iced::Color</span>.
pub const SKY_BLUE: iced::Color = iced::Color::from_rgb(0.529, 0.808, 0.922);

/// Slate Blue <span style="color:slateBlue">iced::Color</span>.
pub const SLATE_BLUE: iced::Color = iced::Color::from_rgb(0.416, 0.353, 0.804);

/// Slate Gray <span style="color:slateGray">iced::Color</span>.
pub const SLATE_GRAY: iced::Color = iced::Color::from_rgb(0.439, 0.502, 0.565);

/// Slate Grey <span style="color:slateGrey">iced::Color</span>.
pub const SLATE_GREY: iced::Color = SLATE_GRAY;

/// Snow <span style="color:snow">iced::Color</span>.
pub const SNOW: iced::Color = iced::Color::from_rgb(1.0, 0.98, 0.98);

/// Spring Green <span style="color:springGreen">iced::Color</span>.
pub const SPRING_GREEN: iced::Color = iced::Color::from_rgb(0.0, 1.0, 0.498);

/// Steel Blue <span style="color:steelBlue">iced::Color</span>.
pub const STEEL_BLUE: iced::Color = iced::Color::from_rgb(0.275, 0.51, 0.706);

/// Tan <span style="color:tan">iced::Color</span>.
pub const TAN: iced::Color = iced::Color::from_rgb(0.824, 0.706, 0.549);

/// Teal <span style="color:teal">iced::Color</span>.
pub const TEAL: iced::Color = iced::Color::from_rgb(0.0, 0.502, 0.502);

/// Thistle <span style="color:thistle">iced::Color</span>.
pub const THISTLE: iced::Color = iced::Color::from_rgb(0.847, 0.749, 0.847);

/// Tomato <span style="color:tomato">iced::Color</span>.
pub const TOMATO: iced::Color = iced::Color::from_rgb(1.0, 0.388, 0.278);

/// Turquoise <span style="color:turquoise">iced::Color</span>.
pub const TURQUOISE: iced::Color = iced::Color::from_rgb(0.251, 0.878, 0.816);

/// Violet <span style="color:violet">iced::Color</span>.
pub const VIOLET: iced::Color = iced::Color::from_rgb(0.933, 0.51, 0.933);

/// Wheat <span style="color:wheat">iced::Color</span>.
pub const WHEAT: iced::Color = iced::Color::from_rgb(0.961, 0.871, 0.702);

/// White <span style="color:white">iced::Color</span>.
pub const WHITE: iced::Color = iced::Color::WHITE;

/// White Smoke <span style="color:whiteSmoke">iced::Color</span>.
pub const WHITE_SMOKE: iced::Color = iced::Color::WHITE;

/// Yellow <span style="color:yellow">iced::Color</span>.
pub const YELLOW: iced::Color = iced::Color::from_rgb(1.0, 1.0, 0.0);

/// Yellow Green <span style="color:yellowGreen">iced::Color</span>.
pub const YELLOW_GREEN: iced::Color = iced::Color::from_rgb(0.604, 0.804, 0.196);
