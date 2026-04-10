//! Font module - provides add_font_style and load_font pyfunctions

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::state::{Widgets, access_state, get_id};
use crate::widgets::ipg_font::{Font, FontStretch, FontStyle, FontFamily, FontWeight};


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
