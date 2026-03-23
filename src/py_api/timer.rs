//! Timer module - provides add_timer pyfunction

use pyo3::prelude::*;
use pyo3::{pyfunction, Py, PyAny};
type PyObject = Py<PyAny>;

use crate::{access_state, add_callback_to_mutex, add_user_data_to_mutex};
use crate::state::get_id;
use crate::widgets::ipg_timer::TimerState;


/// Add a timer event.
///
/// A timer event which can be controlled via update_timer.
///
/// Parameters
/// ----------
/// enable : bool
///     Whether the timer starts immediately.
/// duration_ms : int, Optional
///     The time between ticks in milliseconds (default 10).
/// on_start : Callable, Optional
///     Callback fired on the first tick after enabling.
/// on_tick : Callable, Optional
///     Callback fired on each tick. Receives (timer_id, tick_count, elapsed_ms).
/// on_stop : Callable, Optional
///     Callback fired when the timer is disabled. Receives (timer_id, tick_count, elapsed_ms).
/// user_data : Any, Optional
///     Any user data passed to the callback functions.
/// gen_id : int, Optional
///     Obtains an ID of a widget that has not been created.
///
/// Returns
/// -------
/// int
///     The numeric ID of the newly created timer.
#[pyfunction]
#[pyo3(signature = (
    enable=false,
    duration_ms=None,
    on_start=None,
    on_tick=None,
    on_stop=None,
    user_data=None,
    gen_id=None,
))]
pub fn add_event_timer (
    enable: bool,
    duration_ms: Option<u64>,
    on_start: Option<PyObject>,
    on_tick: Option<PyObject>,
    on_stop: Option<PyObject>,
    user_data: Option<PyObject>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    if let Some(py) = on_start {
        add_callback_to_mutex(id, "on_start".to_string(), py);
    }

    if let Some(py) = on_tick {
        add_callback_to_mutex(id, "on_tick".to_string(), py);
    }

    if let Some(py) = on_stop {
        add_callback_to_mutex(id, "on_stop".to_string(), py);
    }

    if let Some(py) = user_data {
        add_user_data_to_mutex(id, py);
    }

    let mut state = access_state();

    let duration_ms = if let Some(d) = duration_ms {
        d
    } else { 10 };

    state.timer_state.insert(id, 
        TimerState {
            id,
            enable,
            duration_ms,
            ..Default::default()
        });
    
    drop(state);
    Ok(id)
}
