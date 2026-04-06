//! Widget parameter update — trait-based dispatch with shared helpers.
#![allow(unused)]
use iced::Length;
use pyo3::{Py, PyAny, Python};

use crate::graphics::bootstrap_arrow::Arrow;
use crate::graphics::colors::Color;
use crate::py_api::helpers::{
    get_length, get_length_fill, try_extract_boolean, try_extract_f32, try_extract_f32_array_2, 
    try_extract_f32_opt, try_extract_f32_opt_array_1_or_upto_4, try_extract_opt_boolean, 
    try_extract_opt_string, try_extract_opt_u32_array_2, try_extract_opt_usize, try_extract_opt_vec_f32, 
    try_extract_string, try_extract_u16, try_extract_u16_array_2, try_extract_u32, 
    try_extract_u64,  try_extract_vec_f32, try_extract_vec_str, try_extract_vec_u8_opt, try_extract_vec_vec_f32
};
use crate::state::{IpgContainers, IpgWidgets};
use crate::widgets::ipg_text::{TextShaping, TextWrapping};

type PyObject = Py<PyAny>;

// ---------------------------------------------------------------------------
// Trait — each widget implements this alongside its struct
// ---------------------------------------------------------------------------

pub trait WidgetParamUpdate {
    /// The `#[pyclass]` param enum for this widget (e.g. `IpgButtonParam`).
    type Param: for<'py> pyo3::FromPyObject<'py>;

    /// Apply a single parameter update.
    fn param_update(&mut self, param: Self::Param, value: &PyObject);
}

// ---------------------------------------------------------------------------
// Dispatch — one line per widget variant
// ---------------------------------------------------------------------------

// Helper copy and paste
// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

// impl WidgetParamUpdate for IpgRule {
//     type Param = IpgRuleParam;

//     fn param_update(&mut self, param: Self::Param, value: &PyObject) {
//         match param {
//         }
//     }
// }

// impl WidgetParamUpdate for IpgRuleStyle {
//     type Param = IpgRuleStyleParam;

//     fn param_update(&mut self, param: Self::Param, value: &PyObject) {
//         match param {
//         }
//     }
// }


pub fn param_update(
    widget: &mut IpgWidgets,
    item: &PyObject,
    value: &PyObject,
) {
    match widget {
        IpgWidgets::IpgButton(w) => apply_update(w, item, value),
        IpgWidgets::IpgButtonStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgCard(w) => apply_update(w, item, value),
        IpgWidgets::IpgCardStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgCheckBox(w) => apply_update(w, item, value),
        IpgWidgets::IpgCheckboxStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgColorPicker(w) => apply_update(w, item, value),
        IpgWidgets::IpgContainerStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgDatePicker(w) => apply_update(w, item, value),
        IpgWidgets::IpgDivider(w) => apply_update(w, item, value),
        IpgWidgets::IpgDividerStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgFont(w) => apply_update(w, item, value),
        IpgWidgets::IpgImage(w) => apply_update(w, item, value),
        IpgWidgets::IpgMenuStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgPickList(w) => apply_update(w, item, value),
        IpgWidgets::IpgPickListStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgProgressBar(w) => apply_update(w, item, value),
        IpgWidgets::IpgProgressBarStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgRadio(w) => apply_update(w, item, value),
        IpgWidgets::IpgRadioStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgRule(w) => apply_update(w, item, value),
        IpgWidgets::IpgRuleStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgSeparator(w) => apply_update(w, item, value),
        IpgWidgets::IpgSeparatorStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgSpace(w) => apply_update(w, item, value),
        IpgWidgets::IpgSlider(w) => apply_update(w, item, value),
        IpgWidgets::IpgSliderStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgText(w) => apply_update(w, item, value),
        IpgWidgets::IpgToggler(w) => apply_update(w, item, value),
        IpgWidgets::IpgTogglerStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgSvg(w) => apply_update(w, item, value),
        IpgWidgets::IpgScrollableStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgScroller(w) => apply_update(w, item, value),
        IpgWidgets::IpgRailStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgAutoScrollStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgTextEditor(w) => apply_update(w, item, value),
        IpgWidgets::IpgTextInput(w) => apply_update(w, item, value),
        IpgWidgets::IpgTextInputStyle(w) => apply_update(w, item, value),
        IpgWidgets::IpgRichText(w) => apply_update(w, item, value),
        
            }
}

// ---------------------------------------------------------------------------
// Dispatch for containers — one line per container variant
// ---------------------------------------------------------------------------

pub fn container_param_update(
    container: &mut IpgContainers,
    item: &PyObject,
    value: &PyObject,
) {
    match container {
        IpgContainers::IpgColumn(w) => apply_update(w, item, value),
        IpgContainers::IpgContainer(w) => apply_update(w, item, value),
        IpgContainers::IpgFloat(w)=> apply_update(w, item, value),
        IpgContainers::IpgGrid(w)=> apply_update(w, item, value),
        IpgContainers::IpgMenu(w) => apply_update(w, item, value),
        IpgContainers::IpgMouseArea(w) => apply_update(w, item, value),
        IpgContainers::IpgOpaque(_) => panic!("IpgOpaque does not support param_update"),
        IpgContainers::IpgRow(w) => apply_update(w, item, value),
        IpgContainers::IpgScrollable(w) => apply_update(w, item, value),
        IpgContainers::IpgStack(w) => apply_update(w, item, value),
        IpgContainers::IpgTable(w) => apply_update(w, item, value),
        IpgContainers::IpgToolTip(w) => apply_update(w, item, value),
        IpgContainers::IpgWindow(w) => apply_update(w, item, value),
        IpgContainers::IpgMenuBarItem(_) => panic!("IpgMenuBarItem does not support param_update"),
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

pub fn set_bool(field: &mut bool, value: &PyObject, name: &str) {
    *field = try_extract_boolean(value, name);
}

pub fn set_toggle_bool(field: &mut bool, value: &PyObject, name: &str) {
    let b = try_extract_boolean(value, name);
    *field = !b;
}

pub fn set_opt_bool(field: &mut Option<bool>, value: &PyObject, name: &str) {
    *field = try_extract_opt_boolean(value, name);
}

pub fn set_opt_bool_from_opt(field: &mut Option<bool>, value: &PyObject, name: &str) {
    *field = try_extract_opt_boolean(value, name);
}

pub fn set_width(field: &mut Length, value: &PyObject, name: &str) {
    let val = try_extract_f32_opt(value, name);
    *field = get_length(val, false);
}

pub fn set_width_fill(field: &mut Length, value: &PyObject, name: &str) {
    let val = try_extract_boolean(value, name);
    *field = get_length(None, val);
}

pub fn set_height(field: &mut Length, value: &PyObject, name: &str) {
    let val = try_extract_f32_opt(value, name);
    *field = get_length(val, false);
}

pub fn set_height_fill(field: &mut Length, value: &PyObject, name: &str) {
    let val = try_extract_boolean(value, name);
    *field = get_length(None, val);
}

pub fn set_lengths_fill(field1: &mut Length, field2: &mut Length, value: &PyObject, name: &str) {
    let val = try_extract_opt_boolean(value, name);
    [*field1, *field2] = get_length_fill(val);
}

pub fn set_iced_color(field: &mut iced::Color, value: &PyObject, name: &str) {
    let rgba = Color::extract_rgba(value, name);
    *field = iced::Color::from(rgba);
}

pub fn set_opt_iced_color(field: &mut Option<iced::Color>, value: &PyObject, name: &str) {
    let color = Color::extract_opt(value, name);
    *field = Color::rgba_ipg_color_to_iced(None, color, None);
}

pub fn set_opt_iced_color_from_rgba(field: &mut Option<iced::Color>, value: &PyObject, name: &str) {
    let rgba_opt = Color::extract_rgba_opt(value, name);
    *field = Color::rgba_ipg_color_to_iced(rgba_opt, None, None);
}

pub fn set_ipg_color(field: &mut Color, value: &PyObject, name: &str) {
    *field = Color::extract(value, name);
}

pub fn set_f32(field: &mut f32, value: &PyObject, name: &str) {
    *field = try_extract_f32(value, name);
}

pub fn set_opt_f32(field: &mut Option<f32>, value: &PyObject, name: &str) {
    *field = try_extract_f32_opt(value, name);
}

pub fn set_opt_vec_f32(field: &mut Option<Vec<f32>>, value: &PyObject, name: &str) {
    *field = try_extract_opt_vec_f32(value, name);
}

pub fn set_opt_vec_f32_1_or_upto_4(field: &mut Option<Vec<f32>>, value: &PyObject, name: &str) {
    *field = try_extract_f32_opt_array_1_or_upto_4(value, name);
}

pub fn set_vec_f32(field: &mut Vec<f32>, value: &PyObject, name: &str) {
    *field = try_extract_vec_f32(value, name);
}

pub fn set_vec_vec_f32(field: &mut Vec<Vec<f32>>, value: &PyObject, name: &str) {
    *field = try_extract_vec_vec_f32(value, name);
}

pub fn set_opt_u32(field: &mut Option<u32>, value: &PyObject, name: &str) {
    *field = Some(try_extract_u32(value, name));
}

pub fn set_opt_u64(field: &mut Option<u64>, value: &PyObject, name: &str) {
    *field = Some(try_extract_u64(value, name));
}

pub fn set_opt_u16(field: &mut Option<u16>, value: &PyObject, name: &str) {
    *field = Some(try_extract_u16(value, name));
}

pub fn set_opt_vec_u8(field: &mut Option<Vec<u8>>, value: &PyObject, name: &str) {
    *field = try_extract_vec_u8_opt(value, name);
}

pub fn set_opt_usize(field: &mut Option<usize>, value: &PyObject, name: &str) {
    *field = try_extract_opt_usize(value, name);
}

pub fn set_opt_f32_array_2(field: &mut Option<[f32; 2]>, value: &PyObject, name: &str) {
    *field = Some(try_extract_f32_array_2(value, name));
}

pub fn set_opt_f32_array_1_or_to_4(field: &mut Option<Vec<f32>>, value: &PyObject, name: &str) {
    *field = try_extract_f32_opt_array_1_or_upto_4(value, name);
}

pub fn set_opt_u16_array_2(field: &mut Option<[u16; 2]>, value: &PyObject, name: &str) {
    *field = Some(try_extract_u16_array_2(value, name));
}

pub fn set_opt_u32_array_2(field: &mut Option<[u32; 2]>, value: &PyObject, name: &str) {
    *field = try_extract_opt_u32_array_2(value, name);
}

pub fn set_string(field: &mut String, value: &PyObject, name: &str) {
    *field = try_extract_string(value, name);
}

pub fn set_opt_string(field: &mut Option<String>, value: &PyObject, name: &str) {
    *field = try_extract_opt_string(value, name);
}

pub fn set_vec_string(field: &mut Vec<String>, value: &PyObject, name: &str) {
    *field = try_extract_vec_str(value, name);
}

pub fn set_rgba_color_via_ipg(field: &mut Option<iced::Color>, value: &PyObject, name: &str) {
    let rgba = Color::extract_rgba(value, name);
    *field = Color::rgba_ipg_color_to_iced(Some(rgba), None, None);
}

pub fn set_opt_text_shaping(field: &mut Option<TextShaping>, value: &PyObject, name: &str) {
    *field = TextShaping::extract(value)
}

pub fn set_opt_text_wrapping(field: &mut Option<TextWrapping>, value: &PyObject, name: &str) {
    *field = TextWrapping::extract(value)
}

pub fn set_opt_ipg_arrow(field: &mut Option<Arrow>, value: &PyObject, name: &str) {
    *field = Arrow::extract(value)
}
