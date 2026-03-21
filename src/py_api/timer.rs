//! Timer module - provides add_timer pyfunction


use std::time::Instant;

use pyo3::prelude::*;
use pyo3::{Py, PyAny, pyfunction};

use crate::state::{access_events, get_id};
use crate::widgets::ipg_timer::TimerState;
use crate::{access_state, add_user_data_to_mutex};
type PyObject = Py<PyAny>;



#[pyfunction]
#[pyo3(signature = (
    enabled=false,
    start=None,
    stop=None,
    duration_ms=None,
    on_tick=None,
    user_data=None,
    gen_id=None,
))]
pub fn add_event_timer (
    enabled: bool,
    start: Option<u64>,
    stop: Option<u64>,
    duration_ms: Option<u64>,
    on_tick: Option<PyObject>,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut events = access_events();

    if let Some(py) = on_tick {
        events.events.insert((id, "on_tick".to_string()), py);
    }

    drop(events);

    // Store user data if provided
    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let mut state = access_state();

    if enabled {
        state.timer_state.insert(id, 
            TimerState::Ticking {
                last_tick: Instant::now(),
                start,
                stop,
                duration_ms: duration_ms.unwrap_or(10),
            });
    } else {
        state.timer_state.insert(id, TimerState::Idle);
    }

    drop(state);
    Ok(id)
}
