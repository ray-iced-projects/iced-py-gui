use std::collections::HashMap;

use iced::{Element, widget::container};

use crate::{IpgState, app::Message, ipg_widgets::ipg_canvas_draw::canvas_draw::{CanvasState, CanvasWidget}, widgets::widget_param_update::WidgetParamUpdate};

use pyo3::{Py, PyAny, pyclass};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct Draw {
    pub id: usize,
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
    None
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for Draw {
    type Param = DrawParam;

    fn param_update(&mut self, param: Self::Param, _value: &PyObject) {
        match param {
            DrawParam::None => todo!(),
        }
    }
}
