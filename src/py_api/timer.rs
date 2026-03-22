//! Timer module - provides add_timer pyfunction

use pyo3::prelude::*;
use pyo3::pyfunction;

use crate::access_state;
use crate::state::get_id;
use crate::widgets::ipg_timer::TimerState;


/// Add a timer widget.
///
/// A timer event which is attached to another widget.
///
/// Parameters
/// ----------
/// enable: bool,
/// start: uint, Optional
///     Whether to have a start time in ms, otherwise waits for a widget event
/// stop: uint, Optional
///     Whether to have a stop time in ms, otherwise waits for a widget event
/// duration_ms: uint, Optional
///     The time between ticks in milliseconds.
/// gen_id : int, Optional
///     Obtains an ID of a widget that have not been created, used for the gen_id parameter.
///
/// Returns
/// -------
/// int
///     The numeric widget ID of the newly created toggler.
#[pyfunction]
#[pyo3(signature = (
    enable=false,
    start=None,
    stop=None,
    duration_ms=None,
    gen_id=None,
))]
pub fn add_event_timer (
    enable: bool,
    start: Option<u64>,
    stop: Option<u64>,
    duration_ms: Option<u64>,
    gen_id: Option<usize>,
    ) -> PyResult<usize>
{
    let id = get_id(gen_id);

    let mut state = access_state();

    let duration_ms = if let Some(d) = duration_ms {
        d
    } else { 10 };

    state.timer_state.insert(id, 
        TimerState {
            id,
            enable,
            start,
            stop,
            duration_ms,
            ..Default::default()
        });
    
    drop(state);
    Ok(id)
}
