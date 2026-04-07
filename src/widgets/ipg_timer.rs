//! ipg_timer

use std::time::Instant;

use pyo3::{Py, PyAny, Python, pyclass, pyfunction};
type PyObject = Py<PyAny>;
use crate::{IpgState, access_state, py_api::helpers::{try_extract_boolean, 
    try_extract_u64}, widgets::callbacks::invoke_callback_with_two_args};


#[derive(Clone, Debug, Hash)]
pub struct TimerState {
    pub id: usize,
    pub enable: bool,
    pub last_tick: Instant,
    pub duration_ms: u64,
    pub tick_count: u64,
    pub elapsed_ms: u64,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            id: 0,
            enable: false,
            last_tick: Instant::now(),
            duration_ms: 0,
            tick_count: 0,
            elapsed_ms: 0,
        }
    }
}

pub fn timer_callback(state: &mut IpgState, id: usize, _instant: Instant) {
    let ts = state.timer_state.get_mut(&id)
        .expect("timer_callback: timer not found");

    let was_disabled = ts.tick_count == 0;
    ts.tick_count += 1;
    ts.elapsed_ms += ts.duration_ms;

    let tick_count = ts.tick_count;
    let elapsed_ms = ts.elapsed_ms;

    // Sync updated counters back to the static mutex
    {
        let mut mutex_state = access_state();
        if let Some(mts) = mutex_state.timer_state.get_mut(&id) {
            mts.tick_count = tick_count;
            mts.elapsed_ms = elapsed_ms;
        }
    }

    if was_disabled {
        invoke_callback_with_two_args(id, "on_start", "Timer", tick_count, elapsed_ms);
    }

    invoke_callback_with_two_args(id, "on_tick", "Timer", tick_count, elapsed_ms);
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
                TimerParam::DurationMs => {
                    tmr.duration_ms = try_extract_u64(&value, "TimerParam.DurationMs");
                },
                TimerParam::Enable => {
                    let enable = try_extract_boolean(&value, "TimerParam.Enable");
                    if !enable && tmr.enable {
                        // Stopping: fire on_stop and reset counters
                        let tick_count = tmr.tick_count;
                        let elapsed_ms = tmr.elapsed_ms;
                        tmr.enable = false;
                        tmr.tick_count = 0;
                        tmr.elapsed_ms = 0;
                        drop(state);
                        invoke_callback_with_two_args(wid, "on_stop", "Timer", tick_count, elapsed_ms);
                        return;
                    }
                    tmr.enable = enable;
                },
            }
        } else {
            panic!("Update timer: Unable to find timer id {}", wid);
        };
    

    drop(state);
}

fn try_extract_param(value: &PyObject) -> TimerParam {
    Python::attach(|py| {
        let res = value.extract::<TimerParam>(py);
        match res {
            Ok(val) => val,
            Err(err) => panic!("Unable to extract TimerParam : {}", err),
        }
    })
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum TimerParam {
    DurationMs,
    Enable,
}


