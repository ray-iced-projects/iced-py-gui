//! ipg_timer

use std::time::Instant;

use pyo3::{Py, PyAny, Python, pyclass, pyfunction};
type PyObject = Py<PyAny>;
use crate::{IpgState, access_state, py_api::helpers::{try_extract_boolean, try_extract_u64}, widgets::callbacks::invoke_callback_with_args};


#[derive(Clone, Debug, Hash)]
pub struct TimerState {
    pub id: usize,
    pub enable: bool,
    pub last_tick: Instant,
    pub start: Option<u64>,
    pub stop: Option<u64>,
    pub duration_ms: u64,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            id: 0,
            enable: false,
            last_tick: Instant::now(),
            start: None,
            stop: None,
            duration_ms: 0,
        }
    }
}

pub fn timer_callback(_state: &mut IpgState, id: usize, instant: Instant) {
    dbg!(&id, &instant.elapsed());

    invoke_callback_with_args(id, "on_tick", "Timer", instant.elapsed());
}



#[pyfunction]
#[pyo3(signature = (wid, param, value))]
pub fn update_timer(
    wid: usize, 
    param: PyObject, 
    value: PyObject) 
{
    dbg!("update timer");
    let mut state = access_state();

    if let Some(tmr) = 
        state.timer_state.get_mut(&wid) {
            let param = try_extract_param(&param);
            match param {
                IpgTimerParam::DurationMs => {
                    tmr.duration_ms = try_extract_u64(&value, "IpgTimerParam.DurationMs")
                },
                IpgTimerParam::Enable => {
                    dbg!("enabling");
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


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTimerParam {
    DurationMs,
    Enable,
    Start,
    Stop,
}


