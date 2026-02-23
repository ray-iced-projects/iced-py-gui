
use pyo3::{PyResult, pyfunction};

use crate::{access_state, graphics::colors::IpgColor, 
    state::{IpgWidgets, get_id, set_state_of_widget}, 
    widgets::ipg_rule::{IpgRule, IpgRuleStyle}};



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

    let color = IpgColor::rgba_ipg_color_to_iced(color_rgba, color, 1.0, false);
    
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

