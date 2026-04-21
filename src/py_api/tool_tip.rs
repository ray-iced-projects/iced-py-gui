

use pyo3::{PyResult, pyfunction};

use crate::{access_state, state::{Containers, get_id, 
    set_state_cont_wnd_ids, set_state_of_container}, 
    widgets::{ipg_container::ContainerStyleStd, ipg_tool_tip::ToolTip}};



#[pyfunction]
#[pyo3(signature = (
    window_id, 
    container_id,
    text=None,
    parent_id=None,
    position_follow_cursor=None,
    position_bottom=None,
    position_left=None,
    position_top=None,
    position_right=None, 
    gap=None, 
    padding=None, 
    snap_within_viewport=None,
    delay_sec=None, 
    style_id=None,
    style_std=None,
    gen_id=None,
    ))]
pub fn add_tool_tip(
    window_id: String,
    container_id: String,
    text: Option<String>,
    parent_id: Option<String>,
    position_follow_cursor: Option<bool>,
    position_bottom: Option<bool>,
    position_left: Option<bool>,
    position_top: Option<bool>,
    position_right: Option<bool>,
    gap: Option<u32>,
    padding: Option<f32>,
    snap_within_viewport: Option<bool>,
    delay_sec: Option<u64>,
    style_id: Option<usize>,
    style_std: Option<ContainerStyleStd>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{

    let id = get_id(gen_id);

    let prt_id = match parent_id {
        Some(id) => id,
        None => window_id.clone(),
    };

    set_state_of_container(id, window_id.clone(), Some(container_id.clone()), prt_id);

    let mut state = access_state();

    set_state_cont_wnd_ids(&mut state, &window_id, container_id, id, "add_tool_tip".to_string());

    state.containers.insert(id, Containers::ToolTip(
        ToolTip { 
            id,
            position_follow_cursor,
            position_bottom,
            position_left,
            position_top,
            position_right,
            text,
            gap,
            padding,
            snap_within_viewport,
            delay_sec,
            style_id,
            style_std,
        }));

    drop(state);
    Ok(id)

}

