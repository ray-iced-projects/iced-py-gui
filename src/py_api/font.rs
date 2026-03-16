//! Font module - provides add_font pyfunction

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::state::{IpgWidgets, access_state, get_id};
use crate::widgets::ipg_font::{IpgFont, IpgFontStretch, IpgFontStyle, IpgFontFamily, IpgFontWeight};


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
    family: Option<IpgFontFamily>,
    family_name: Option<String>,
    weight: Option<IpgFontWeight>,
    stretch: Option<IpgFontStretch>,
    style: Option<IpgFontStyle>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgFont(
        IpgFont {
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
