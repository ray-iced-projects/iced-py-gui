//! ipg_timer

use std::time::Instant;

use pyo3::pyclass;

use crate::IpgState;


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


pub fn timer_callback(
    state: &mut IpgState, 
    ts: TimerState ) {
    
    dbg!(&state, &ts);
    
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTimerParam {
    DurationMs,
    Enable,
    Start,
    Stop,
}


