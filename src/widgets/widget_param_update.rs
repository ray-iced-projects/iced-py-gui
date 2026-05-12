//! Widget parameter update — trait-based dispatch with shared helpers.
use pyo3::{Py, PyAny, Python};

use crate::state::{Containers, Widgets};

type PyObject = Py<PyAny>;

// ---------------------------------------------------------------------------
// Trait — each widget implements this alongside its struct
// ---------------------------------------------------------------------------

pub trait WidgetParamUpdate {
    /// The `#[pyclass]` param enum for this widget (e.g. `ButtonParam`).
    type Param: for<'py> pyo3::FromPyObject<'py>;

    /// Apply a single parameter update.
    fn param_update(&mut self, param: Self::Param, value: &PyObject);
}

// ---------------------------------------------------------------------------
// Dispatch — one line per widget variant
// ---------------------------------------------------------------------------

// Helper for copy and paste to new widget
// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

// impl WidgetParamUpdate for <widget> {
//     type Param = <WidgetParam>;

//     fn param_update(&mut self, param: Self::Param, value: &PyObject) {
//         match param {
//         }
//     }
// }

// impl WidgetParamUpdate for <WidgetStyle> {
//     type Param = <WidgetStyleParam>;

//     fn param_update(&mut self, param: Self::Param, value: &PyObject) {
//         match param {
//         }
//     }
// }


pub fn param_update(
    widget: &mut Widgets,
    item: &PyObject,
    value: &PyObject,
) {
    match widget {
        Widgets::Button(w) => apply_update(w, item, value),
        Widgets::ButtonStyle(w) => apply_update(w, item, value),
        Widgets::CardStyle(w) => apply_update(w, item, value),
        Widgets::CheckBox(w) => apply_update(w, item, value),
        Widgets::CheckboxStyle(w) => apply_update(w, item, value),
        Widgets::ComboBox(w) => apply_update(w, item, value),
        Widgets::ContainerStyle(w) => apply_update(w, item, value),
        Widgets::DatePicker(w) => apply_update(w, item, value),
        Widgets::Divider(w) => apply_update(w, item, value),
        Widgets::DividerStyle(w) => apply_update(w, item, value),
        Widgets::Font(w) => apply_update(w, item, value),
        Widgets::Icon(_) => panic!("Icon does not support param_update"),
        Widgets::Image(w) => apply_update(w, item, value),
        // Widgets::MenuStyle(w) => apply_update(w, item, value),
        Widgets::PickList(w) => apply_update(w, item, value),
        Widgets::PickListStyle(w) => apply_update(w, item, value),
        Widgets::ProgressBar(w) => apply_update(w, item, value),
        Widgets::ProgressBarStyle(w) => apply_update(w, item, value),
        Widgets::Radio(w) => apply_update(w, item, value),
        Widgets::RadioStyle(w) => apply_update(w, item, value),
        Widgets::Rule(w) => apply_update(w, item, value),
        Widgets::RuleStyle(w) => apply_update(w, item, value),
        Widgets::Separator(w) => apply_update(w, item, value),
        Widgets::SeparatorStyle(w) => apply_update(w, item, value),
        Widgets::Space(w) => apply_update(w, item, value),
        Widgets::Slider(w) => apply_update(w, item, value),
        Widgets::SliderStyle(w) => apply_update(w, item, value),
        Widgets::AutoScrollStyle(w) => apply_update(w, item, value),
        Widgets::RailStyle(w) => apply_update(w, item, value),
        Widgets::ScrollableStyle(w) => apply_update(w, item, value),
        Widgets::Scroller(w) => apply_update(w, item, value),
        Widgets::Span(w) => apply_update(w, item, value),
        Widgets::Svg(w) => apply_update(w, item, value),
        Widgets::Text(w) => apply_update(w, item, value),
        Widgets::TextEditor(w) => apply_update(w, item, value),
        Widgets::TextEditorStyle(w) => apply_update(w, item, value),
        Widgets::TextInput(w) => apply_update(w, item, value),
        Widgets::TextInputStyle(w) => apply_update(w, item, value),
        Widgets::Toggler(w) => apply_update(w, item, value),
        Widgets::TogglerStyle(w) => apply_update(w, item, value),
    }
}

// ---------------------------------------------------------------------------
// Dispatch for containers — one line per container variant
// ---------------------------------------------------------------------------

pub fn container_param_update(
    container: &mut Containers,
    item: &PyObject,
    value: &PyObject,
) {
    match container {
        Containers::Card(w) => apply_update(w, item, value),
        Containers::Column(w) => apply_update(w, item, value),
        Containers::Container(w) => apply_update(w, item, value),
        Containers::Float(w)=> apply_update(w, item, value),
        Containers::Grid(w)=> apply_update(w, item, value),
        Containers::MenuItem(w) => apply_update(w, item, value),
        Containers::MouseArea(w) => apply_update(w, item, value),
        Containers::Opaque(_) => panic!("Opaque does not support param_update"),
        Containers::RichText(w) => apply_update(w, item, value),
        Containers::Row(w) => apply_update(w, item, value),
        Containers::Scrollable(w) => apply_update(w, item, value),
        Containers::Stack(w) => apply_update(w, item, value),
        Containers::Table(w) => apply_update(w, item, value),
        Containers::ToolTip(w) => apply_update(w, item, value),
        Containers::Window(w) => apply_update(w, item, value),
        _ => panic!("{:?} does not support param_update", container)
    }
}


fn apply_update<W: WidgetParamUpdate>(
    widget: &mut W,
    item: &PyObject,
    value: &PyObject,
) {
    let param = extract_param::<W::Param>(item);
    widget.param_update(param, value);
}


// ---------------------------------------------------------------------------
// Generic param extraction (works for any #[pyclass] enum)
// ---------------------------------------------------------------------------

pub fn extract_param<T>(update_obj: &PyObject) -> T
where
    T: for<'py> pyo3::FromPyObject<'py>,
{
    Python::attach(|py| {
        update_obj
            .extract::<T>(py)
            .unwrap_or_else(|err| panic!("param extraction failed: {}", err))
    })
}

// ---------------------------------------------------------------------------
// Shared value-update helpers — use these in WidgetParamUpdate impls
// ---------------------------------------------------------------------------

pub fn set_t_value<T>(field: &mut T, value: &PyObject, name: &str)
where
    T: for<'py> pyo3::FromPyObject<'py>,
{
    *field = Python::attach(|py| {
        value
            .extract::<T>(py)
            .unwrap_or_else(|err| panic!("{name} extraction failed: {err}"))
    });
}
