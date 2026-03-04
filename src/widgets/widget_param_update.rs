//! Widget parameter update — trait-based dispatch with shared helpers.
#![allow(unused)]

use iced::{Color, Length};
use pyo3::{Py, PyAny, Python};

use crate::graphics::bootstrap_arrow::IpgArrow;
use crate::graphics::colors::IpgColor;
use crate::py_api::helpers::{
    get_height, get_width, try_extract_boolean, try_extract_f32, try_extract_f32_array_2, try_extract_string, try_extract_style_standard, try_extract_u16, try_extract_u16_array_2, try_extract_u32, try_extract_usize, try_extract_vec_f32, try_extract_vec_str, try_extract_vec_vec_f32
};
use crate::state::{IpgContainers, IpgWidgets};
use crate::widgets::enums::{IpgAlignment, IpgAlignmentX, IpgShaping, IpgAlignmentY};
use crate::widgets::ipg_text::IpgWrapping;

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

// Helper copy and paste
// ---------------------------------------------------------------------------
// WidgetParamUpdate implementations
// ---------------------------------------------------------------------------

// impl WidgetParamUpdate for IpgRule {
//     type Param = IpgRuleParam;

//     fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
//         match param {
//         }
//     }
// }

// impl WidgetParamUpdate for IpgRuleStyle {
//     type Param = IpgRuleStyleParam;

//     fn param_update(&mut self, param: Self::Param, value: &PyObject, name: String) {
//         match param {
//         }
//     }
// }


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
        IpgWidgets::IpgColorPicker(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgContainerStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgDatePicker(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgDivider(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgDividerStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgFont(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgImage(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgOpaqueStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgPickList(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgPickListStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgProgressBar(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgProgressBarStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgRadio(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgRadioStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgRule(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgRuleStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgSelectableText(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgSeparator(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgSeparatorStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgSpace(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgSlider(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgSliderStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgText(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgToggler(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgTogglerStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgSvg(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgScrollbar(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgRailStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgAutoScrollStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgScrollableStyleConfig(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgTableStyle(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgTextInput(w) => apply_update(w, item, value, name),
        IpgWidgets::IpgTextInputStyle(w) => apply_update(w, item, value, name),
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
        IpgContainers::IpgMouseArea(w) => apply_update(w, item, value, name),
        IpgContainers::IpgOpaque(w) => apply_update(w, item, value, name),
        IpgContainers::IpgRow(w) => apply_update(w, item, value, name),
        IpgContainers::IpgScrollable(w) => apply_update(w, item, value, name),
        IpgContainers::IpgStack(w) => apply_update(w, item, value, name),
        IpgContainers::IpgTable(w) => apply_update(w, item, value, name),
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

pub fn set_t_value<T>(field: &mut T, value: &PyObject, name: String)
where
    T: for<'py> pyo3::FromPyObject<'py>,
{
    *field = Python::attach(|py| {
        value
            .extract::<T>(py)
            .unwrap_or_else(|err| panic!("{name} extraction failed: {err}"))
    });
}

pub fn set_bool(field: &mut bool, value: &PyObject, name: String) {
    *field = try_extract_boolean(value, name);
}

pub fn set_opt_bool(field: &mut Option<bool>, value: &PyObject, name: String) {
    *field = Some(try_extract_boolean(value, name));
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

pub fn set_iced_color(field: &mut Color, value: &PyObject, name: String) {
    let rgba = IpgColor::extract_rgba(value, name);
    *field = Color::from(rgba);
}

pub fn set_opt_iced_color(field: &mut Option<Color>, value: &PyObject, name: String) {
    let color = IpgColor::extract(value, name);
    *field = IpgColor::rgba_ipg_color_to_iced(None, Some(color), 1.0, false);
}

pub fn set_iced_color_from_rgba(field: &mut Option<Color>, value: &PyObject, name: String) {
    *field = Some(Color::from(IpgColor::extract_rgba(value, name)));
}

pub fn set_ipg_color(field: &mut IpgColor, value: &PyObject, name: String) {
    *field = IpgColor::extract(value, name);
}

pub fn set_f32(field: &mut f32, value: &PyObject, name: String) {
    *field = try_extract_f32(value, name);
}

pub fn set_opt_f32(field: &mut Option<f32>, value: &PyObject, name: String) {
    *field = Some(try_extract_f32(value, name));
}

pub fn set_opt_vec_f32(field: &mut Option<Vec<f32>>, value: &PyObject, name: String) {
    *field = Some(try_extract_vec_f32(value, name));
}

pub fn set_vec_f32(field: &mut Vec<f32>, value: &PyObject, name: String) {
    *field = try_extract_vec_f32(value, name);
}

pub fn set_vec_vec_f32(field: &mut Vec<Vec<f32>>, value: &PyObject, name: String) {
    *field = try_extract_vec_vec_f32(value, name);
}

pub fn set_opt_u32(field: &mut Option<u32>, value: &PyObject, name: String) {
    *field = Some(try_extract_u32(value, name));
}

pub fn set_opt_u16(field: &mut Option<u16>, value: &PyObject, name: String) {
    *field = Some(try_extract_u16(value, name));
}

pub fn set_opt_usize(field: &mut Option<usize>, value: &PyObject, name: String) {
    *field = Some(try_extract_usize(value, name));
}

pub fn set_opt_f32_array_2(field: &mut Option<[f32; 2]>, value: &PyObject, name: String) {
    *field = Some(try_extract_f32_array_2(value, name));
}

pub fn set_opt_u16_array_2(field: &mut Option<[u16; 2]>, value: &PyObject, name: String) {
    *field = Some(try_extract_u16_array_2(value, name));
}

pub fn set_string(field: &mut String, value: &PyObject, name: String) {
    *field = try_extract_string(value, name);
}

pub fn set_opt_string(field: &mut Option<String>, value: &PyObject, name: String) {
    *field = Some(try_extract_string(value, name));
}

pub fn set_vec_string(field: &mut Vec<String>, value: &PyObject, name: String) {
    *field = try_extract_vec_str(value, name);
}

pub fn set_rgba_color_via_ipg(field: &mut Option<Color>, value: &PyObject, name: String) {
    let rgba = IpgColor::extract_rgba(value, name);
    *field = IpgColor::rgba_ipg_color_to_iced(Some(rgba), None, 1.0, false);
}

pub fn set_halign(field: &mut Option<IpgAlignmentX>, value: &PyObject, name:String) {
    *field = IpgAlignmentX::extract(value);
}

pub fn set_valign(field: &mut Option<IpgAlignmentY>, value: &PyObject, name: String) {
    *field = IpgAlignmentY::extract(value);
}

pub fn set_align(field: &mut Option<IpgAlignment>, value: &PyObject, name: String) {
    *field = IpgAlignment::extract(value);
}

pub fn set_opt_text_shaping(field: &mut Option<IpgShaping>, value: &PyObject, name: String) {
    *field = IpgShaping::extract(value)
}

pub fn set_opt_text_wrapping(field: &mut Option<IpgWrapping>, value: &PyObject, name: String) {
    *field = IpgWrapping::extract(value)
}

pub fn set_opt_ipg_arrow(field: &mut Option<IpgArrow>, value: &PyObject, name: String) {
    *field = IpgArrow::extract(value)
}
