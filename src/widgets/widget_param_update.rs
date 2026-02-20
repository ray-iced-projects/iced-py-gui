//! Widget parameter update — trait-based dispatch with shared helpers.
#![allow(unused)]

use iced::{Color, Length};
use pyo3::{Py, PyAny, Python};

use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::{
    get_height, get_width, try_extract_array_2, try_extract_boolean,
    try_extract_f32, try_extract_string, try_extract_usize,
    try_extract_vec_f32, try_extract_style_standard,
};
use crate::state::{IpgContainers, IpgWidgets};
use crate::widgets::enums::{IpgAlignment, IpgHorizontalAlignment, IpgVerticalAlignment};

type PyObject = Py<PyAny>;

// ---------------------------------------------------------------------------
// Trait — each widget implements this alongside its struct
// ---------------------------------------------------------------------------

pub trait WidgetParamUpdate {
    /// The `#[pyclass]` param enum for this widget (e.g. `IpgButtonParam`).
    type Param: for<'py> pyo3::FromPyObject<'py>;

    /// Apply a single parameter update.
    fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String);
}

// ---------------------------------------------------------------------------
// Dispatch — one line per widget variant
// ---------------------------------------------------------------------------

pub fn param_update(
    widget: &mut IpgWidgets,
    item: &PyObject,
    value: &PyObject,
    name: String,
) {
    match widget {
        IpgWidgets::IpgButton(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgButtonStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgCheckBox(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgCheckboxStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgContainerStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgDatePicker(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgDividerHorizontal(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgDividerVertical(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgDividerStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgFont(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgText(w) => apply_update(w, item, value, name),
        _ => (),
    }
}

// ---------------------------------------------------------------------------
// Dispatch for containers — one line per container variant
// ---------------------------------------------------------------------------

pub fn container_param_update(
    container: &mut IpgContainers,
    item: &PyObject,
    value: &PyObject,
    name: String,
) {
    match container {
        IpgContainers::IpgColumn(w) => apply_update(w, item, value, name),
        IpgContainers::IpgContainer(w) => apply_update(w, item, value, name),
        IpgContainers::IpgRow(w) => apply_update(w, item, value, name),
        IpgContainers::IpgWindow(w) => apply_update(w, item, value, name),
    }
}

fn apply_update<W: WidgetParamUpdate>(
    widget: &mut W,
    item: &PyObject,
    value: &PyObject,
    name: String,
) {
    let param = extract_param::<W::Param>(item, &name);
    widget.param_update(param, value, name);
}

// ---------------------------------------------------------------------------
// Generic param extraction (works for any #[pyclass] enum)
// ---------------------------------------------------------------------------

pub fn extract_param<T>(update_obj: &PyObject, widget_name: &str) -> T
where
    T: for<'py> pyo3::FromPyObject<'py>,
{
    Python::attach(|py| {
        update_obj
            .extract::<T>(py)
            .unwrap_or_else(|err| panic!("{} param extraction failed: {}", widget_name, err))
    })
}

// ---------------------------------------------------------------------------
// Shared value-update helpers — use these in WidgetParamUpdate impls
// ---------------------------------------------------------------------------

pub fn set_bool(field: &mut bool, value: &PyObject, name: String) {
    *field = try_extract_boolean(value, name);
}

pub fn set_opt_bool(field: &mut Option<bool>, value: &PyObject, name: String) {
    *field = Some(try_extract_boolean(value, name));
}

pub fn set_opt_f32(field: &mut Option<f32>, value: &PyObject, name: String) {
    *field = Some(try_extract_f32(value, name));
}

pub fn set_opt_string(field: &mut Option<String>, value: &PyObject, name: String) {
    *field = Some(try_extract_string(value, name));
}

pub fn set_opt_vec_f32(field: &mut Option<Vec<f32>>, value: &PyObject, name: String) {
    *field = Some(try_extract_vec_f32(value, name));
}

pub fn set_opt_usize(field: &mut Option<usize>, value: &PyObject, name: String) {
    *field = Some(try_extract_usize(value, name));
}

pub fn set_width(field: &mut Length, value: &PyObject, name: String) {
    let val = try_extract_f32(value, name);
    *field = get_width(Some(val), false);
}

pub fn set_width_fill(field: &mut Length, value: &PyObject, name: String) {
    let val = try_extract_boolean(value, name);
    *field = get_width(None, val);
}

pub fn set_height(field: &mut Length, value: &PyObject, name: String) {
    let val = try_extract_f32(value, name);
    *field = get_height(Some(val), false);
}

pub fn set_height_fill(field: &mut Length, value: &PyObject, name: String) {
    let val = try_extract_boolean(value, name);
    *field = get_height(None, val);
}

pub fn set_ipg_color(field: &mut Option<Color>, value: &PyObject, name: String) {
    let color = IpgColor::extract(value, name);
    *field = IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
}

pub fn set_rgba_color(field: &mut Option<Color>, value: &PyObject, name: String) {
    *field = Some(Color::from(IpgColor::extract_rgba(value, name)));
}

pub fn set_f32(field: &mut f32, value: &PyObject, name: String) {
    *field = try_extract_f32(value, name);
}

pub fn set_string(field: &mut String, value: &PyObject, name: String) {
    *field = try_extract_string(value, name);
}

pub fn set_vec_f32(field: &mut Vec<f32>, value: &PyObject, name: String) {
    *field = try_extract_vec_f32(value, name);
}

pub fn set_opt_array_2(field: &mut Option<[f32; 2]>, value: &PyObject, name: String) {
    *field = Some(try_extract_array_2(value, name));
}

pub fn set_rgba_color_via_ipg(field: &mut Option<Color>, value: &PyObject, name: String) {
    let rgba = IpgColor::extract_rgba(value, name);
    *field = IpgColor::rgba_ipg_color_to_iced(Some(rgba), None, 1.0, false);
}

pub fn set_halign(field: &mut Option<IpgHorizontalAlignment>, value: &PyObject) {
    *field = IpgHorizontalAlignment::extract(value);
}

pub fn set_valign(field: &mut Option<IpgVerticalAlignment>, value: &PyObject) {
    *field = IpgVerticalAlignment::extract(value);
}

pub fn set_align(field: &mut Option<IpgAlignment>, value: &PyObject) {
    *field = IpgAlignment::extract(value);
}
