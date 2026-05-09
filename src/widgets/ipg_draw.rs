use std::collections::HashMap;

use iced::{Element, widget::{container, Id}};

use crate::{IpgState, app::Message, ipg_widgets::ipg_canvas_draw::{canvas_draw::{CanvasState, CanvasWidget}, import_export::import_widgets}, widgets::widget_param_update::WidgetParamUpdate};

use pyo3::{PyResult, Python, types::PyAnyMethods, pyclass, Py, PyAny};
type PyObject = Py<PyAny>;
use crate::ipg_widgets::ipg_canvas_draw::import_export::ExportWidget;


#[derive(Debug, Clone)]
pub struct Draw {
    pub id: usize,
    pub curves: HashMap<Id, CanvasWidget>,
    pub text_curves: HashMap<Id, CanvasWidget>,
}

impl Draw {
    pub fn construct<'a>(
        &'a self,
        canvas_state: &'a CanvasState,
    ) -> Option<Element<'a, Message>> {
        let id = self.id;

        let canvas = canvas_state
            .view(&canvas_state.curves, &canvas_state.text_curves)
            .map(move |cw| Message::CanvasDraw(id, cw));

        Some(container(canvas).into())
    }
}


pub fn draw_callback(_state: &mut IpgState, _id: usize, _message: CanvasWidget) {
    // TODO: handle canvas events
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum DrawParam {
    None,
    Curves,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Draw {
    type Param = DrawParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            DrawParam::None => todo!(),
            DrawParam::Curves => {
                let c = extract_curves(value);
                match c {
                    Ok(crv) => {
                        self.curves = crv.0;
                        self.text_curves = crv.1;
                    },
                    Err(err) => panic!("Curve extraction failed {err}")
                }
            },
        }
    }
}


pub fn extract_curves(curves: &PyObject) -> PyResult<(HashMap<Id, CanvasWidget>, HashMap<Id, CanvasWidget>)> {
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
