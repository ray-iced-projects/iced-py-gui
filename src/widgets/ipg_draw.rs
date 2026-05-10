use std::collections::HashMap;

use iced::{Element, widget::{container, Id}};

use crate::{IpgState, app::Message, ipg_widgets::ipg_canvas_draw::{canvas_draw::{DrawWidget, DrawMode, DrawState, DrawStatus, get_draw_mode_and_status, get_widget_id, set_widget_mode_or_status}, import_export::import_widgets}, state::{Containers, access_update_canvas_draw}, widgets::widget_param_update::extract_param};

use pyo3::{PyResult, Python, types::PyAnyMethods, pyclass, Py, PyAny};
type PyObject = Py<PyAny>;
use crate::ipg_widgets::ipg_canvas_draw::import_export::ExportWidget;


#[derive(Debug, Clone)]
pub struct Draw {
    pub id: usize,
    pub curves: HashMap<Id, DrawWidget>,
    pub text_curves: HashMap<Id, DrawWidget>,
}

impl Draw {
    pub fn construct<'a>(
        &'a self,
        draw_state: &'a DrawState,
    ) -> Option<Element<'a, Message>> {
        let id = self.id;

        let canvas = draw_state
            .view(&draw_state.curves, &draw_state.text_curves)
            .map(move |cw| Message::CanvasDraw(id, cw));

        Some(container(canvas).into())
    }
}


pub fn draw_callback(state: &mut IpgState, id: usize, mut widget: DrawWidget) {
    // Get info from the widget before any moves.
    let (draw_mode, draw_status) = get_draw_mode_and_status(&widget);
    let widget_id = get_widget_id(&widget);

    let Some(cs) = state.canvas_states.get_mut(&id) else { return };

    if matches!(widget, DrawWidget::Text(_)) {
        // ---- Text path ----
        match draw_status {
            DrawStatus::Completed => {
                // Insert or update — the entry may not exist yet if keypresses
                // were kept in Pending without publishing.
                widget = set_widget_mode_or_status(widget, Some(DrawMode::Display), None);
                cs.text_curves.insert(widget_id.clone(), widget.clone());
                cs.request_text_redraw_for(&widget_id);
                cs.timer_event_enabled = false;
                cs.draw_mode = DrawMode::Display;
            },
            DrawStatus::Delete => {
                cs.text_curves.remove(&widget_id);
                cs.remove_text_cache(&widget_id);
                cs.timer_event_enabled = false;
                cs.request_text_redraw();
                return;
            },
            DrawStatus::Inprogress => {
                if cs.text_curves.contains_key(&widget_id) {
                    cs.text_curves.entry(widget_id.clone()).and_modify(|k| *k = widget.clone());
                } else {
                    cs.text_curves.insert(widget_id.clone(), widget.clone());
                    // Seed a cache entry for this new text widget.
                    cs.request_text_redraw_for(&widget_id);
                }
            },
        }
        // For Edit/Rotate, also update edit_widget_id and ensure the entry is current.
        match draw_mode {
            DrawMode::Edit | DrawMode::Rotate => {
                cs.edit_widget_id = Some(widget_id.clone());
                cs.text_curves.entry(widget_id).and_modify(|k| *k = widget);
            },
            _ => (),
        }
        cs.request_text_redraw();
    } else {
        // ---- All other widgets ----
        match draw_status {
            DrawStatus::Completed => {
                widget = set_widget_mode_or_status(widget, Some(DrawMode::Display), None);
            },
            DrawStatus::Delete => {
                cs.curves.remove(&widget_id);
                cs.request_redraw();
                return;
            },
            DrawStatus::Inprogress => (),
        }
        if draw_mode == DrawMode::New {
            let widget = set_widget_mode_or_status(widget, Some(DrawMode::Display), Some(DrawStatus::Completed));
            cs.curves.insert(widget_id, widget);
        } else {
            // Edit or Rotate: update the existing entry in place.
            cs.edit_widget_id = Some(widget_id.clone());
            cs.curves.entry(widget_id).and_modify(|k| *k = widget);
        }
        cs.request_redraw();
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum DrawParam {
    None,
    Clear,
    Curves,
    SelectedWidget,
}


pub fn extract_curves(curves: &PyObject) -> PyResult<(HashMap<Id, DrawWidget>, HashMap<Id, DrawWidget>)> {
    Python::attach(|py| -> PyResult<_> {
        let obj = curves.bind(py);
        // Accept either a callable (returns a list) or a list directly.
        let result = if obj.is_callable() {
            obj.call0()?
        } else {
            obj.clone()
        };
        let json_mod = py.import("json")?;
        let json_str: String = json_mod
            .call_method1("dumps", (&result,))?
            .extract()?;
        let export_widgets: Vec<ExportWidget> = serde_json::from_str(&json_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(import_widgets(export_widgets))
    })    
}


pub fn process_draw_updates(
    state: &mut IpgState,
    draw: &mut Draw,
    item: &PyObject,
    value: &PyObject,
) {
    dbg!("process_draw_updates");
    let updates = access_update_canvas_draw();
    
        let param: DrawParam = extract_param(item);
        match param {
            DrawParam::None => (),
            DrawParam::Clear => state.canvas_states.clear(),
            DrawParam::Curves => {
                dbg!("curves");
                if let Some(Containers::CanvasDraw(draw)) = state.containers.get(&draw_id) {
                    let curves = extract_curves(value);
                    dbg!(&curves);
                    // if let Some(cs) = state.canvas_states.get_mut(&draw_id) {
                    //     cs.curves = curves;
                    //     cs.text_curves = text_curves;
                    //     cs.request_redraw();
                    //     cs.request_text_redraw();
                    // }
                
                }
            },
            DrawParam::SelectedWidget => todo!(),
        }
        
}