//! Font module - provides add_font_style and load_font pyfunctions

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::state::{Widgets, access_state, get_id};
use crate::widgets::ipg_font::{Font, FontStretch, FontStyle, FontFamily, FontWeight, IpgIcon};
use crate::graphics::bootstrap::bootstrap_arrow::Arrow;
use crate::graphics::bootstrap::bootstrap_icon::Icon;
use crate::graphics::BOOTSTRAP_FONT;
use crate::graphics::bootstrap::bootstrap_icon::icon_to_char;
use crate::graphics::bootstrap::bootstrap_arrow;


/// Load a font from a .ttf or .otf file path.
///
/// The font will be available for use after the application starts.
/// Must be called before start_session().
///
/// Parameters
/// ----------
/// path : str
///     The path to the font file (.ttf or .otf).
#[pyfunction]
pub fn load_font(path: String) -> PyResult<()> {
    let bytes = std::fs::read(&path)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(
            format!("Failed to read font file '{}': {}", path, e)
        ))?;
    let mut state = access_state();
    state.user_fonts.push(bytes);
    drop(state);
    Ok(())
}


/// Add a font widget.
///
/// Returns the widget ID.
/// 
/// Parameters
/// ----------
/// family: FontFamily, Optional
///     Stes the font family, see FontFamily.
/// family_name: str, Optional
/// Sets the name of the font to use.
/// weight: FontWeight, Optional
///     Sets the font weight, see FontWeight.
/// stretch: FontStretch Optional
///     Sets the font stretch, see FontStretch.
/// style: FontStyle Optional
///     Sets the font style, see FontStyle.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created widget.

#[pyfunction]
#[pyo3(signature = (
    family=None,
    family_name=None,
    weight=None,
    stretch=None,
    style=None,
    gen_id=None
    ))]
pub fn add_font_style(
    family: Option<FontFamily>,
    family_name: Option<String>,
    weight: Option<FontWeight>,
    stretch: Option<FontStretch>,
    style: Option<FontStyle>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, Widgets::Font(
        Font {
            id,
            family,
            family_name,
            weight,
            stretch,
            style,
        }));

    drop(state);
    Ok(id)
}


/// Add an icon descriptor for use as a pick_list handle icon.
///
/// Supply **one** of the icon source options:
///
/// * ``arrow`` — a bootstrap ``Arrow`` enum variant (e.g. ``Arrow.ArrowDown``)
/// * ``icon``  — a bootstrap ``Icon`` enum variant (e.g. ``Icon.Folder``)
/// * ``font_id`` + ``code_point`` — a custom font registered with
///   ``add_font_style``, with the glyph's Unicode code point as an integer
///   (e.g. ``0xe040``).
///
/// Parameters
/// ----------
/// arrow : Arrow, optional
///     A bootstrap arrow glyph.
/// icon : Icon, optional
///     A bootstrap icon glyph.
/// font_id : int, optional
///     ID of a font style (``add_font_style``) for a custom icon font.
/// code_point : int, optional
///     Unicode code point when using a custom ``font_id``.
/// size : float, optional
///     Glyph size in pixels.
/// line_height : float, optional
///     Relative line height multiplier. Defaults to 1.0.
/// gen_id : int, optional
///     Pre-reserved widget ID (from ``generate_id``).
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created icon descriptor.
#[pyfunction]
#[pyo3(signature = (
    arrow=None,
    icon=None,
    code_point=None,
    font_id=None,
    size=None,
    line_height=None,
    gen_id=None,
    ))]
pub fn add_icon(
    arrow: Option<Arrow>,
    icon: Option<Icon>,
    code_point: Option<u32>,
    font_id: Option<usize>,
    size: Option<f32>,
    line_height: Option<f32>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let (resolved_font, resolved_char) = if let Some(arr) = arrow {
        (BOOTSTRAP_FONT, bootstrap_arrow::Arrow::to_char(&arr))
    } else if let Some(ic) = icon {
        (BOOTSTRAP_FONT, icon_to_char(ic))
    } else if let (Some(fid), Some(cp)) = (font_id, code_point) {
        let state = access_state();
        let font = state.widgets.get(&fid)
            .and_then(Widgets::as_font)
            .map(|f| f.to_iced())
            .unwrap_or_default();
        drop(state);
        let ch = char::from_u32(cp)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err(
                format!("Invalid Unicode code point: 0x{:x}", cp)
            ))?;
        (font, ch)
    } else {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "add_icon: supply arrow=, icon=, or both font_id= and code_point="
        ));
    };

    let mut state = access_state();

    state.widgets.insert(id, Widgets::Icon(
        IpgIcon {
            id,
            font: resolved_font,
            code_point: resolved_char,
            size,
            line_height,
        }));

    drop(state);
    Ok(id)
}


#[pyfunction]
#[pyo3(signature = (
    arrow,
    ))]
pub fn arrow_to_str(
    arrow: Arrow,
    ) -> PyResult<String>
{
    let ar = bootstrap_arrow::Arrow::to_string(&arrow);
    Ok(ar)
}

/// Return a list of every Arrow enum variant.
///
/// Useful for iterating over all available bootstrap arrow icons.
///
/// Returns
/// -------
/// list[Arrow]
///     All Arrow variants in declaration order.
#[pyfunction]
pub fn arrow_variants() -> Vec<Arrow> {
    vec![
        Arrow::ArrowBarLeft,
        Arrow::ArrowBarRight,
        Arrow::ArrowBarUp,
        Arrow::ArrowClockwise,
        Arrow::ArrowCounterclockwise,
        Arrow::ArrowDown,
        Arrow::ArrowDownCircle,
        Arrow::ArrowDownCircleFill,
        Arrow::ArrowDownLeft,
        Arrow::ArrowDownLeftCircle,
        Arrow::ArrowDownLeftCircleFill,
        Arrow::ArrowDownLeftSquare,
        Arrow::ArrowDownLeftSquareFill,
        Arrow::ArrowDownRight,
        Arrow::ArrowDownRightCircle,
        Arrow::ArrowDownRightCircleFill,
        Arrow::ArrowDownRightSquare,
        Arrow::ArrowDownRightSquareFill,
        Arrow::ArrowDownShort,
        Arrow::ArrowDownSquare,
        Arrow::ArrowDownSquareFill,
        Arrow::ArrowDownUp,
        Arrow::ArrowLeft,
        Arrow::ArrowLeftCircle,
        Arrow::ArrowLeftCircleFill,
        Arrow::ArrowLeftRight,
        Arrow::ArrowLeftShort,
        Arrow::ArrowLeftSquare,
        Arrow::ArrowLeftSquareFill,
        Arrow::ArrowNinezerodegDown,
        Arrow::ArrowNinezerodegLeft,
        Arrow::ArrowNinezerodegRight,
        Arrow::ArrowNinezerodegUp,
        Arrow::ArrowRepeat,
        Arrow::ArrowReturnLeft,
        Arrow::ArrowReturnRight,
        Arrow::ArrowRight,
        Arrow::ArrowRightCircle,
        Arrow::ArrowRightCircleFill,
        Arrow::ArrowRightShort,
        Arrow::ArrowRightSquare,
        Arrow::ArrowRightSquareFill,
        Arrow::ArrowThroughHeart,
        Arrow::ArrowThroughHeartFill,
        Arrow::ArrowUp,
        Arrow::ArrowUpCircle,
        Arrow::ArrowUpCircleFill,
        Arrow::ArrowUpLeft,
        Arrow::ArrowUpLeftCircle,
        Arrow::ArrowUpLeftCircleFill,
        Arrow::ArrowUpLeftSquare,
        Arrow::ArrowUpLeftSquareFill,
        Arrow::ArrowUpRight,
        Arrow::ArrowUpRightCircle,
        Arrow::ArrowUpRightCircleFill,
        Arrow::ArrowUpRightSquare,
        Arrow::ArrowUpRightSquareFill,
        Arrow::ArrowUpShort,
        Arrow::ArrowUpSquare,
        Arrow::ArrowUpSquareFill,
        Arrow::Arrows,
        Arrow::ArrowsAngleContract,
        Arrow::ArrowsAngleExpand,
        Arrow::ArrowsCollapse,
        Arrow::ArrowsCollapseVertical,
        Arrow::ArrowsExpand,
        Arrow::ArrowsExpandVertical,
        Arrow::ArrowsFullscreen,
        Arrow::ArrowsMove,
        Arrow::ArrowsVertical,
    ]
}
