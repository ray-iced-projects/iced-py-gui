//! Font module - provides add_font pyfunction

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::state::{Widgets, access_state, get_id};
use crate::widgets::ipg_font::{Font, FontStretch, FontStyle, FontFamily, FontWeight};


/// Add a font widget.
///
/// Returns the widget ID.
#[pyfunction]
#[pyo3(signature = (
    family=None,
    family_name=None,
    weight=None,
    stretch=None,
    style=None,
    gen_id=None
    ))]
pub fn add_font(
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
