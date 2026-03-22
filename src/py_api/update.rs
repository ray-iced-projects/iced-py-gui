//! Update module - provides update_widget, delete_widget, add_widget, and show_widget pyfunctions

use pyo3::{Py, PyAny, Python, pyfunction};

use crate::{access_state, py_api::helpers::{try_extract_boolean, try_extract_u64}, state::access_update_widgets, widgets::ipg_timer::IpgTimerParam};
type PyObject = Py<PyAny>;

#[pyfunction]
#[pyo3(signature = (wid, param, value))]
pub fn update_widget(
    wid: usize, 
    param: PyObject, 
    value: PyObject) 
{

    let mut all_updates = access_update_widgets();

    all_updates.updates.push((wid, param, value));

    drop(all_updates);
}

#[pyfunction]
#[pyo3(signature = (wid))]
pub fn delete_widget(wid: usize) 
{
    let mut all_updates = access_update_widgets();

    all_updates.deletes.push(wid);

    drop(all_updates);
}

#[pyfunction]
#[pyo3(signature = (wid))]
pub fn show_widget(wid: usize)
{
    let mut all_updates = access_update_widgets();

    all_updates.shows.push((wid, true));

    drop(all_updates);
}

#[pyfunction]
#[pyo3(signature = (wid))]
pub fn hide_widget(wid: usize)
{
    let mut all_updates = access_update_widgets();

    all_updates.shows.push((wid, false));

    drop(all_updates);
}

#[pyfunction]
#[pyo3(signature = (
    wid, 
    move_after=None,
    move_before=None,
    target_parent_id=None
    ))]
pub fn move_widget(
    wid: usize,
    move_after: Option<usize>,
    move_before: Option<usize>,
    target_parent_id: Option<usize>)
{
    let mut all_updates = access_update_widgets();
    
    all_updates.moves.push((wid, move_after, move_before, target_parent_id));
    
    drop(all_updates);
}

#[pyfunction]
#[pyo3(signature = (wid, param, value))]
pub fn update_timer(
    wid: usize, 
    param: PyObject, 
    value: PyObject) 
{

    let mut state = access_state();

    if let Some(tmr) = 
        state.timer_state.get_mut(&wid) {
            let param = try_extract_param(&param);
            match param {
                IpgTimerParam::DurationMs => {
                    tmr.duration_ms = try_extract_u64(&value, "IpgTimerParam.DurationMs")
                },
                IpgTimerParam::Enable => {
                    tmr.enable = try_extract_boolean(&value, "IpgTimerParam.Enable")
                },
                IpgTimerParam::Start => {
                    tmr.start = Some(try_extract_u64(&value, "IpgTimerParam.Start"))
                },
                IpgTimerParam::Stop => {
                    tmr.stop = Some(try_extract_u64(&value, "IpgTimerParam.Stop"))
                },
            }
        } else {
            panic!("Update timer: Unable to find timer id {}", wid)
        };
    

    drop(state);
}

fn try_extract_param(value: &PyObject) -> IpgTimerParam {
    Python::attach(|py| {
        let res = value.extract::<IpgTimerParam>(py);
        match res {
            Ok(val) => val,
            Err(err) => panic!("Unable to extract IpgTimerParam : {}", err),
        }
    })
}
