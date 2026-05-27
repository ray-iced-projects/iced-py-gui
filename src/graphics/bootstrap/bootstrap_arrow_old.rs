//! Arrow

use pyo3::{pyclass, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::graphics::bootstrap::bootstrap;

use super::bootstrap::{icon_to_string, icon_to_char};


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum Arrow {
    ArrowBarLeft,             // ←|
    ArrowBarRight,            // |→
    ArrowBarUp,               // ↑—
    ArrowClockwise,           // ↻
    ArrowCounterclockwise,    // ↺
    ArrowDown,                // ↓
    ArrowDownCircle,          // ○↓ can't see
    ArrowDownCircleFill,      // ●↓
    ArrowDownLeft,            // ↙
    ArrowDownLeftCircle,      // ○↙
    ArrowDownLeftCircleFill,  // ●↙
    ArrowDownLeftSquare,      // □↙
    ArrowDownLeftSquareFill,  // ■↙
    ArrowDownRight,           // ↘
    ArrowDownRightCircle,     // ○↘
    ArrowDownRightCircleFill, // ●↘
    ArrowDownRightSquare,     // □↘
    ArrowDownRightSquareFill, // ■↘
    ArrowDownShort,           // ↓ (short)
    ArrowDownSquare,          // □↓
    ArrowDownSquareFill,      // ■↓
    ArrowDownUp,              // ↕
    ArrowLeft,                // ←
    ArrowLeftCircle,          // ○←
    ArrowLeftCircleFill,      // ●←
    ArrowLeftRight,           // ↔
    ArrowLeftShort,           // ← (short)
    ArrowLeftSquare,          // □←
    ArrowLeftSquareFill,      // ■←
    ArrowNinezerodegDown,     // ↱↓  90° turn → down
    ArrowNinezerodegLeft,     // ↓↰  90° turn → left
    ArrowNinezerodegRight,    // ↑↱  90° turn → right
    ArrowNinezerodegUp,       // ↲↑  90° turn → up
    ArrowRepeat,              // ↺↻  repeat
    ArrowReturnLeft,          // ↵←  return left
    ArrowReturnRight,         // ↵→  return right
    ArrowRight,               // →
    ArrowRightCircle,         // ○→
    ArrowRightCircleFill,     // ●→
    ArrowRightShort,          // → (short)
    ArrowRightSquare,         // □→
    ArrowRightSquareFill,     // ■→
    ArrowThroughHeart,        // →♡
    ArrowThroughHeartFill,    // →♥
    ArrowUp,                  // ↑
    ArrowUpCircle,            // ○↑
    ArrowUpCircleFill,        // ●↑
    ArrowUpLeft,              // ↖
    ArrowUpLeftCircle,        // ○↖
    ArrowUpLeftCircleFill,    // ●↖
    ArrowUpLeftSquare,        // □↖
    ArrowUpLeftSquareFill,    // ■↖
    ArrowUpRight,             // ↗
    ArrowUpRightCircle,       // ○↗
    ArrowUpRightCircleFill,   // ●↗
    ArrowUpRightSquare,       // □↗
    ArrowUpRightSquareFill,   // ■↗
    ArrowUpShort,             // ↑ (short)
    ArrowUpSquare,            // □↑
    ArrowUpSquareFill,        // ■↑
    Arrows,                   // ⇄  bidirectional
    ArrowsAngleContract,      // ↘↗  angle contract
    ArrowsAngleExpand,        // ↖↙  angle expand
    ArrowsCollapse,           // →←  collapse horizontal
    ArrowsCollapseVertical,   // ↓↑  collapse vertical
    ArrowsExpand,             // ←→  expand horizontal
    ArrowsExpandVertical,     // ↑↓  expand vertical
    ArrowsFullscreen,         // ↖↗↙↘  fullscreen
    ArrowsMove,               // ✥   move (all directions)
    ArrowsVertical,           // ↕   vertical
}

impl Arrow {
    pub fn to_string(arrow: &Arrow) -> String {
        match arrow {
            Arrow::ArrowBarLeft => icon_to_string(bootstrap::Bootstrap::ArrowBarLeft),
            Arrow::ArrowBarRight => icon_to_string(bootstrap::Bootstrap::ArrowBarRight),
            Arrow::ArrowBarUp => icon_to_string(bootstrap::Bootstrap::ArrowBarUp),
            Arrow::ArrowClockwise => icon_to_string(bootstrap::Bootstrap::ArrowClockwise),
            Arrow::ArrowCounterclockwise => icon_to_string(bootstrap::Bootstrap::ArrowCounterclockwise),
            Arrow::ArrowDown => icon_to_string(bootstrap::Bootstrap::ArrowDown),
            Arrow::ArrowDownCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownCircle),
            Arrow::ArrowDownCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownCircleFill),
            Arrow::ArrowDownLeft => icon_to_string(bootstrap::Bootstrap::ArrowDownLeft),
            Arrow::ArrowDownLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircle),
            Arrow::ArrowDownLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftCircleFill),
            Arrow::ArrowDownLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquare),
            Arrow::ArrowDownLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownLeftSquareFill),
            Arrow::ArrowDownRight => icon_to_string(bootstrap::Bootstrap::ArrowDownRight),
            Arrow::ArrowDownRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircle),
            Arrow::ArrowDownRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowDownRightCircleFill),
            Arrow::ArrowDownRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquare),
            Arrow::ArrowDownRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownRightSquareFill),
            Arrow::ArrowDownShort => icon_to_string(bootstrap::Bootstrap::ArrowDownShort),
            Arrow::ArrowDownSquare => icon_to_string(bootstrap::Bootstrap::ArrowDownSquare),
            Arrow::ArrowDownSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowDownSquareFill),
            Arrow::ArrowDownUp => icon_to_string(bootstrap::Bootstrap::ArrowDownUp),
            Arrow::ArrowLeft => icon_to_string(bootstrap::Bootstrap::ArrowLeft),
            Arrow::ArrowLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircle),
            Arrow::ArrowLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowLeftCircleFill),
            Arrow::ArrowLeftRight => icon_to_string(bootstrap::Bootstrap::ArrowLeftRight),
            Arrow::ArrowLeftShort => icon_to_string(bootstrap::Bootstrap::ArrowLeftShort),
            Arrow::ArrowLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquare),
            Arrow::ArrowLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowLeftSquareFill),
            Arrow::ArrowNinezerodegDown => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegDown),
            Arrow::ArrowNinezerodegLeft => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegLeft),
            Arrow::ArrowNinezerodegRight => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegRight),
            Arrow::ArrowNinezerodegUp => icon_to_string(bootstrap::Bootstrap::ArrowNinezerodegUp),
            Arrow::ArrowRepeat => icon_to_string(bootstrap::Bootstrap::ArrowRepeat),
            Arrow::ArrowReturnLeft => icon_to_string(bootstrap::Bootstrap::ArrowReturnLeft),
            Arrow::ArrowReturnRight => icon_to_string(bootstrap::Bootstrap::ArrowReturnRight),
            Arrow::ArrowRight => icon_to_string(bootstrap::Bootstrap::ArrowRight),
            Arrow::ArrowRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowRightCircle),
            Arrow::ArrowRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowRightCircleFill),
            Arrow::ArrowRightShort => icon_to_string(bootstrap::Bootstrap::ArrowRightShort),
            Arrow::ArrowRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowRightSquare),
            Arrow::ArrowRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowRightSquareFill),
            Arrow::ArrowThroughHeart => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeart),
            Arrow::ArrowThroughHeartFill => icon_to_string(bootstrap::Bootstrap::ArrowThroughHeartFill),
            Arrow::ArrowUp => icon_to_string(bootstrap::Bootstrap::ArrowUp),
            Arrow::ArrowUpCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpCircle),
            Arrow::ArrowUpCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpCircleFill),
            Arrow::ArrowUpLeft => icon_to_string(bootstrap::Bootstrap::ArrowUpLeft),
            Arrow::ArrowUpLeftCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircle),
            Arrow::ArrowUpLeftCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftCircleFill),
            Arrow::ArrowUpLeftSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquare),
            Arrow::ArrowUpLeftSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpLeftSquareFill),
            Arrow::ArrowUpRight => icon_to_string(bootstrap::Bootstrap::ArrowUpRight),
            Arrow::ArrowUpRightCircle => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircle),
            Arrow::ArrowUpRightCircleFill => icon_to_string(bootstrap::Bootstrap::ArrowUpRightCircleFill),
            Arrow::ArrowUpRightSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquare),
            Arrow::ArrowUpRightSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpRightSquareFill),
            Arrow::ArrowUpShort => icon_to_string(bootstrap::Bootstrap::ArrowUpShort),
            Arrow::ArrowUpSquare => icon_to_string(bootstrap::Bootstrap::ArrowUpSquare),
            Arrow::ArrowUpSquareFill => icon_to_string(bootstrap::Bootstrap::ArrowUpSquareFill),
            Arrow::Arrows => icon_to_string(bootstrap::Bootstrap::Arrows),
            Arrow::ArrowsAngleContract => icon_to_string(bootstrap::Bootstrap::ArrowsAngleContract),
            Arrow::ArrowsAngleExpand => icon_to_string(bootstrap::Bootstrap::ArrowsAngleExpand),
            Arrow::ArrowsCollapse => icon_to_string(bootstrap::Bootstrap::ArrowsCollapse),
            Arrow::ArrowsCollapseVertical => icon_to_string(bootstrap::Bootstrap::ArrowsCollapseVertical),
            Arrow::ArrowsExpand => icon_to_string(bootstrap::Bootstrap::ArrowsExpand),
            Arrow::ArrowsExpandVertical => icon_to_string(bootstrap::Bootstrap::ArrowsExpandVertical),
            Arrow::ArrowsFullscreen => icon_to_string(bootstrap::Bootstrap::ArrowsFullscreen),
            Arrow::ArrowsMove => icon_to_string(bootstrap::Bootstrap::ArrowsMove),
            Arrow::ArrowsVertical => icon_to_string(bootstrap::Bootstrap::ArrowsVertical),
        }
    }


    pub fn to_char(arrow: &Arrow) -> char {
        match arrow {
            Arrow::ArrowBarLeft => icon_to_char(bootstrap::Bootstrap::ArrowBarLeft),
            Arrow::ArrowBarRight => icon_to_char(bootstrap::Bootstrap::ArrowBarRight),
            Arrow::ArrowBarUp => icon_to_char(bootstrap::Bootstrap::ArrowBarUp),
            Arrow::ArrowClockwise => icon_to_char(bootstrap::Bootstrap::ArrowClockwise),
            Arrow::ArrowCounterclockwise => icon_to_char(bootstrap::Bootstrap::ArrowCounterclockwise),
            Arrow::ArrowDown => icon_to_char(bootstrap::Bootstrap::ArrowDown),
            Arrow::ArrowDownCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownCircle),
            Arrow::ArrowDownCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownCircleFill),
            Arrow::ArrowDownLeft => icon_to_char(bootstrap::Bootstrap::ArrowDownLeft),
            Arrow::ArrowDownLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftCircle),
            Arrow::ArrowDownLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftCircleFill),
            Arrow::ArrowDownLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftSquare),
            Arrow::ArrowDownLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownLeftSquareFill),
            Arrow::ArrowDownRight => icon_to_char(bootstrap::Bootstrap::ArrowDownRight),
            Arrow::ArrowDownRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowDownRightCircle),
            Arrow::ArrowDownRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowDownRightCircleFill),
            Arrow::ArrowDownRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownRightSquare),
            Arrow::ArrowDownRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownRightSquareFill),
            Arrow::ArrowDownShort => icon_to_char(bootstrap::Bootstrap::ArrowDownShort),
            Arrow::ArrowDownSquare => icon_to_char(bootstrap::Bootstrap::ArrowDownSquare),
            Arrow::ArrowDownSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowDownSquareFill),
            Arrow::ArrowDownUp => icon_to_char(bootstrap::Bootstrap::ArrowDownUp),
            Arrow::ArrowLeft => icon_to_char(bootstrap::Bootstrap::ArrowLeft),
            Arrow::ArrowLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowLeftCircle),
            Arrow::ArrowLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowLeftCircleFill),
            Arrow::ArrowLeftRight => icon_to_char(bootstrap::Bootstrap::ArrowLeftRight),
            Arrow::ArrowLeftShort => icon_to_char(bootstrap::Bootstrap::ArrowLeftShort),
            Arrow::ArrowLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowLeftSquare),
            Arrow::ArrowLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowLeftSquareFill),
            Arrow::ArrowNinezerodegDown => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegDown),
            Arrow::ArrowNinezerodegLeft => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegLeft),
            Arrow::ArrowNinezerodegRight => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegRight),
            Arrow::ArrowNinezerodegUp => icon_to_char(bootstrap::Bootstrap::ArrowNinezerodegUp),
            Arrow::ArrowRepeat => icon_to_char(bootstrap::Bootstrap::ArrowRepeat),
            Arrow::ArrowReturnLeft => icon_to_char(bootstrap::Bootstrap::ArrowReturnLeft),
            Arrow::ArrowReturnRight => icon_to_char(bootstrap::Bootstrap::ArrowReturnRight),
            Arrow::ArrowRight => icon_to_char(bootstrap::Bootstrap::ArrowRight),
            Arrow::ArrowRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowRightCircle),
            Arrow::ArrowRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowRightCircleFill),
            Arrow::ArrowRightShort => icon_to_char(bootstrap::Bootstrap::ArrowRightShort),
            Arrow::ArrowRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowRightSquare),
            Arrow::ArrowRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowRightSquareFill),
            Arrow::ArrowThroughHeart => icon_to_char(bootstrap::Bootstrap::ArrowThroughHeart),
            Arrow::ArrowThroughHeartFill => icon_to_char(bootstrap::Bootstrap::ArrowThroughHeartFill),
            Arrow::ArrowUp => icon_to_char(bootstrap::Bootstrap::ArrowUp),
            Arrow::ArrowUpCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpCircle),
            Arrow::ArrowUpCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpCircleFill),
            Arrow::ArrowUpLeft => icon_to_char(bootstrap::Bootstrap::ArrowUpLeft),
            Arrow::ArrowUpLeftCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftCircle),
            Arrow::ArrowUpLeftCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftCircleFill),
            Arrow::ArrowUpLeftSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftSquare),
            Arrow::ArrowUpLeftSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpLeftSquareFill),
            Arrow::ArrowUpRight => icon_to_char(bootstrap::Bootstrap::ArrowUpRight),
            Arrow::ArrowUpRightCircle => icon_to_char(bootstrap::Bootstrap::ArrowUpRightCircle),
            Arrow::ArrowUpRightCircleFill => icon_to_char(bootstrap::Bootstrap::ArrowUpRightCircleFill),
            Arrow::ArrowUpRightSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpRightSquare),
            Arrow::ArrowUpRightSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpRightSquareFill),
            Arrow::ArrowUpShort => icon_to_char(bootstrap::Bootstrap::ArrowUpShort),
            Arrow::ArrowUpSquare => icon_to_char(bootstrap::Bootstrap::ArrowUpSquare),
            Arrow::ArrowUpSquareFill => icon_to_char(bootstrap::Bootstrap::ArrowUpSquareFill),
            Arrow::Arrows => icon_to_char(bootstrap::Bootstrap::Arrows),
            Arrow::ArrowsAngleContract => icon_to_char(bootstrap::Bootstrap::ArrowsAngleContract),
            Arrow::ArrowsAngleExpand => icon_to_char(bootstrap::Bootstrap::ArrowsAngleExpand),
            Arrow::ArrowsCollapse => icon_to_char(bootstrap::Bootstrap::ArrowsCollapse),
            Arrow::ArrowsCollapseVertical => icon_to_char(bootstrap::Bootstrap::ArrowsCollapseVertical),
            Arrow::ArrowsExpand => icon_to_char(bootstrap::Bootstrap::ArrowsExpand),
            Arrow::ArrowsExpandVertical => icon_to_char(bootstrap::Bootstrap::ArrowsExpandVertical),
            Arrow::ArrowsFullscreen => icon_to_char(bootstrap::Bootstrap::ArrowsFullscreen),
            Arrow::ArrowsMove => icon_to_char(bootstrap::Bootstrap::ArrowsMove),
            Arrow::ArrowsVertical => icon_to_char(bootstrap::Bootstrap::ArrowsVertical),
        }
    }

}
