//! ipg_combo_box
use crate::app;
use crate::IpgState;
use crate::state::Widgets;
use crate::widgets::widget_param_update::extract_param;
use crate::widgets::widget_param_update::{WidgetParamUpdate, set_t_value};
use super::callbacks::invoke_callback_with_args;
use crate::py_api::helpers::{get_len, get_padding};

use iced::widget::combo_box;
use iced::widget::text::Ellipsis;
use iced::{Element};
use iced::widget;

use pyo3::pyclass;
use pyo3::{Py, PyAny};
type PyObject = Py<PyAny>;


#[derive(Debug, Clone)]
pub struct ComboBox {
    pub id: usize,
    pub cb_state: combo_box::State<String>,
    pub placeholder: Option<String>,
    pub selected: Option<String>,
    pub width: Option<f32>,
    pub width_fill: Option<bool>,
    pub menu_height: Option<f32>,
    pub menu_height_fill: Option<bool>,
    pub padding: Option<Vec<f32>>,
    pub text_size: Option<f32>,
    pub text_line_height: Option<f32>,
    pub text_ellipsis: Ellipsis,
    pub show: bool,
}

#[derive(Debug, Clone)]
pub enum CBMessage {
    OnSelect(String),
    OnOpen,
    OnClose,
    OnInput,
}

impl ComboBox {

    pub fn construct<'a>(
        &'a self, 
    ) -> Option<Element<'a, app::Message>> {
        
        if !self.show {
            return None
        }

        let placeholder = self.placeholder.as_deref().unwrap_or_default();

        let cb = widget::combo_box(
            &self.cb_state,
            placeholder,
            self.selected.as_ref(),
            CBMessage::OnSelect,
        );

        let cb = cb
            .width(get_len(None, self.width_fill, self.width))
            .menu_height(get_len(None, self.menu_height_fill, self.menu_height))
            .padding(get_padding(&self.padding))
            .ellipsis(self.text_ellipsis)
            .on_open(CBMessage::OnOpen)
            .on_close(CBMessage::OnClose);

        let cb = if let Some(ts) = self.text_size {
            cb.size(ts)
        } else { cb };

        let cb = if let Some(lh) = self.text_line_height {
            cb.line_height(lh)
        } else { cb };

        let cb: Element<'_, CBMessage> = cb.into();
        Some(cb.map(move |message| app::Message::ComboBox(self.id, message)))

    }
 }


 pub fn combo_box_callback(state: &mut IpgState, id: usize, message: CBMessage) {
    match message {
        CBMessage::OnSelect(selected) => {
            // Update widget state directly
            if let Some(Widgets::ComboBox(cb)) = state.widgets.get_mut(&id) {
                cb.selected = Some(selected.clone());
            }
            invoke_callback_with_args(id, "on_select", "ComboBox", selected,
                "def cb(wid: int, selected: str)");
        },
        CBMessage::OnOpen => {
            invoke_callback_with_args(id, "on_open", "ComboBox", (),
                "def cb(wid: int)");
        },
        CBMessage::OnClose => {
            invoke_callback_with_args(id, "on_close", "ComboBox", (),
                "def cb(wid: int)");
        },
        CBMessage::OnInput => {
            invoke_callback_with_args(id, "on_input", "ComboBox", (),
                "def cb(wid: int)");
        },
    }
 }


#[derive(Debug, Clone, PartialEq, Hash)]
#[pyclass(eq, eq_int, hash, frozen)]
pub enum ComboBoxParam {
    MenuHeight,
    Options,
    Padding,
    Placeholder,
    Selected,
    Show,
    TextLineHeight,
    TextSize,
    Width,
}

// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

impl WidgetParamUpdate for ComboBox {
    type Param = ComboBoxParam;

    fn param_update(&mut self, param: Self::Param, value: &PyObject) {
        match param {
            ComboBoxParam::MenuHeight => set_t_value(&mut self.menu_height, value, "ComboBoxParam::MenuHeight"),
            ComboBoxParam::Options => {
                let options: Vec<String> = extract_param(value);
                self.cb_state = combo_box::State::new(options.clone());
            },
            ComboBoxParam::Padding => set_t_value(&mut self.padding, value, "ComboBoxParam::Padding"),
            ComboBoxParam::Placeholder => set_t_value(&mut self.placeholder, value, "ComboBoxParam::Placeholder"),
            ComboBoxParam::Selected => set_t_value(&mut self.selected, value, "ComboBoxParam::Selected"),
            ComboBoxParam::Show => set_t_value(&mut self.show, value, "ComboBoxParam::Show"),
            ComboBoxParam::TextLineHeight => set_t_value(&mut self.text_line_height,value, "ComboBoxParam::TextLineHeight"),
            ComboBoxParam::TextSize => set_t_value(&mut self.text_size, value, "ComboBoxParam::TextSize"),
            ComboBoxParam::Width => set_t_value(&mut self.width, value, "ComboBoxParam::Width"),
        }
    }
}
