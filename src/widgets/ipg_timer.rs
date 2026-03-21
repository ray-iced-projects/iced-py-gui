//! ipg_timer

use std::time::Instant;

use pyo3::{pyclass, Py, PyAny};

use crate::IpgState;

// Type alias to replace deprecated PyObject
type PyObject = Py<PyAny>;


#[derive(Default, Clone, Debug)]
pub enum TimerState {
    #[default]
    Idle,
    Ticking {
        last_tick: Instant,
        start: Option<u64>,
        stop: Option<u64>,
        duration_ms: u64,
    },
}


pub fn timer_callback(
    state: &mut IpgState, 
    id: usize, 
    instant: Instant,
    start: Option<u64>,
    stop: Option<u64>,
    duration_ms: u64 ) {
    
    
    
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgTimerParam {
    DurationMs,
}

