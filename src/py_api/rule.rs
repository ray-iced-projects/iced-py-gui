
use pyo3::{PyResult, pyfunction};

use crate::{access_state, graphics::colors::IpgColor, 
    state::{IpgWidgets, get_id, set_state_of_widget}, 
    widgets::ipg_rule::{IpgRule, IpgRuleStyle}};


/// Add a rule widget.
///
/// A horizontal or vertical line used as a visual separator.
///
/// Parameters
/// ----------
/// parent_id : str
///     Sets the parent container ID that this rule belongs to.
/// is_vertical : bool, Optional
///     Whether the rule is oriented vertically.
/// thickness : int, Optional
///     Sets the thickness of the rule in logical pixels.
/// style_id : int, Optional
///     Sets the ID of a custom style created with ``add_rule_style``.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
/// show : bool, default True
///     Whether the rule is visible.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created rule.
#[pyfunction]
#[pyo3(signature = (
    parent_id,
    is_vertical=None, 
    thickness=None,
    style_id=None,
    gen_id=None,
    show=true,
    ))]
pub fn add_rule(
    parent_id: String,
    is_vertical: Option<bool>,
    thickness: Option<u32>,
    style_id: Option<usize>,
    gen_id: Option<usize>,
    show: bool
    ) -> PyResult<usize> 
{
    let id = get_id(gen_id);

    set_state_of_widget(id, parent_id.clone());

    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgRule(
        IpgRule {
            id,
            parent_id,
            is_vertical,
            thickness,
            style_id,
            show,
        }));

    drop(state);
    Ok(id)

}


/// Add styling to a rule.
///
/// Creates a custom style that can be applied to a rule
/// via its ``style_id`` parameter.
///
/// Parameters
/// ----------
/// color : IpgColor, Optional
///     Sets the rule color using a predefined color variant.
/// color_rgba : list of float, Optional
///     Sets the rule color in rgba format as [r, g, b, a].
/// border_radius : list of float, Optional
///     Sets the radius of the corners as [all] or
///     [top-left, top-right, bottom-right, bottom-left].
/// fillmode_percent : float, Optional
///     Sets the fill mode as a percentage of the available space.
/// fillmode_padded : int, Optional
///     Sets the fill mode with equal padding on both sides.
/// fillmode_asymmetric_padding : list of int, Optional
///     Sets the fill mode with asymmetric padding as [start, end].
/// snap : bool, Optional
///     Whether to snap the rule to the pixel grid.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric style ID to pass to a rule's ``style_id``.
#[pyfunction]
#[pyo3(signature = (
    color=None, 
    color_rgba=None,
    border_radius=None,
    fillmode_percent=None,
    fillmode_padded=None,
    fillmode_asymmetric_padding=None,
    snap=None,
    gen_id=None
    ))]
pub fn add_rule_style(
    color: Option<IpgColor>,
    color_rgba: Option<[f32; 4]>,
    border_radius: Option<Vec<f32>>,
    fillmode_percent: Option<f32>,
    fillmode_padded: Option<u16>,
    fillmode_asymmetric_padding: Option<[u16; 2]>,
    snap: Option<bool>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let color = IpgColor::rgba_ipg_color_to_iced(color_rgba, color, 1.0);
    
    let mut state = access_state();

    state.widgets.insert(id, IpgWidgets::IpgRuleStyle(
        IpgRuleStyle {
            id,
            color,
            border_radius,
            fillmode_percent,
            fillmode_padded,
            fillmode_asymmetric_padding,
            snap,
        }));

    drop(state);
    Ok(id)
}

