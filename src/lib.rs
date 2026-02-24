//! Minimal lib.rs - prototype for modular widget architecture
//!
//! This demonstrates using module-level pyfunctions instead of a monolithic IPG pyclass.
//! This is a self-contained prototype that doesn't depend on the complex existing modules.

use pyo3::prelude::*;

// Core modules for the minimal prototype
mod state;
mod app;
mod py_api;
mod graphics;

// Minimal widget definitions (self-contained)
mod widgets;

// Re-export for internal use
pub use state::{
    access_state, access_callbacks, access_user_data1,
    add_callback_to_mutex, add_user_data_to_mutex, clone_state_to_runtime,
    IpgIds, IpgState
};

// Import pyfunctions from py_api modules
use crate::py_api::window::add_window;
use crate::py_api::button::{add_button, add_button_style};
use crate::py_api::checkbox::{add_checkbox, add_checkbox_style};
use crate::py_api::color_picker::{add_color_picker, add_color_picker_style};
use crate::py_api::column::add_column;
use crate::py_api::container::{add_container, add_container_style};
use crate::py_api::date_picker::add_date_picker;
use crate::py_api::divider::{add_divider_horizontal, 
    add_divider_vertical, add_divider_style};
use crate::py_api::font::add_font;
use crate::py_api::image::add_image;
use crate::py_api::mousearea::add_mouse_area;
use crate::py_api::opaque::{add_opaque_container, add_opaque_style};
use crate::py_api::progress_bar::{add_progress_bar, add_progress_bar_style};
use crate::py_api::radio::{add_radio, add_radio_style};
use crate::py_api::row::add_row;
use crate::py_api::rule::{add_rule, add_rule_style};
use crate::py_api::picklist::{add_pick_list, add_pick_list_style};
use crate::py_api::text::add_text;
use crate::py_api::toggle::{add_toggler, add_toggler_style};
use crate::py_api::selectable_text::add_selectable_text;
use crate::py_api::separator::{add_separator, add_separator_style};
use crate::py_api::space::add_space;
use crate::py_api::slider::{add_slider, add_slider_style};
use crate::py_api::session::{start_session, generate_id};
use crate::py_api::update::update_widget;
use crate::widgets::ipg_radio::{IpgRadioDirection, IpgRadioParam, IpgRadioStyleParam};
use crate::widgets::ipg_rule::{IpgRuleParam, IpgRuleStyleParam};
use crate::widgets::ipg_selectable_text::IpgSelectableTextParam;
use crate::widgets::ipg_separator::{IpgSeparatorParam, IpgSeparatorStyleParam, IpgSeparatorType};
use crate::widgets::ipg_slider::{IpgSliderParam, IpgSliderStyleParam};
use crate::widgets::ipg_toggle::{IpgTogglerParam, IpgTogglerStyleParam};

// Import enums from widgets module
use widgets::enums::{IpgAlignment, IpgHorizontalAlignment, IpgVerticalAlignment};
use widgets::ipg_window::{IpgWindowLevel, IpgWindowMode, IpgWindowTheme};
use widgets::styling::IpgStyleStandard;

use crate::graphics::{bootstrap_icon::IpgIcon, bootstrap_arrow::IpgArrow};
use crate::graphics::colors::IpgColor;
use crate::widgets::ipg_button::{IpgButtonParam, IpgButtonStyleParam, IpgButtonStyleStandard};
use crate::widgets::ipg_checkbox::{IpgCheckboxParam, IpgCheckboxStyleParam};
use crate::widgets::ipg_column::IpgColumnParam;
use crate::widgets::ipg_container::IpgContainerStyleParam;
use crate::widgets::ipg_date_picker::IpgDatePickerParam;
use crate::widgets::ipg_divider::{IpgDividerParam, IpgDividerStyleParam};
use crate::widgets::ipg_pick_list::IpgPickListHandle;
use crate::widgets::ipg_text::IpgTextParam;

/// Python module definition
#[pymodule]
fn icedpygui(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Session functions
    m.add_function(wrap_pyfunction!(start_session, m)?)?;
    m.add_function(wrap_pyfunction!(generate_id, m)?)?;
    
    // Widget functions
    m.add_function(wrap_pyfunction!(add_window, m)?)?;
    m.add_function(wrap_pyfunction!(add_button, m)?)?;
    m.add_function(wrap_pyfunction!(add_checkbox, m)?)?;
    m.add_function(wrap_pyfunction!(add_column, m)?)?;
    m.add_function(wrap_pyfunction!(add_color_picker, m)?)?;
    m.add_function(wrap_pyfunction!(add_container, m)?)?;
    m.add_function(wrap_pyfunction!(add_date_picker, m)?)?;
    m.add_function(wrap_pyfunction!(add_divider_horizontal, m)?)?;
    m.add_function(wrap_pyfunction!(add_divider_vertical, m)?)?;
    m.add_function(wrap_pyfunction!(add_image, m)?)?;
    m.add_function(wrap_pyfunction!(add_font, m)?)?;
    m.add_function(wrap_pyfunction!(add_mouse_area, m)?)?;
    m.add_function(wrap_pyfunction!(add_opaque_container, m)?)?;
    m.add_function(wrap_pyfunction!(add_pick_list, m)?)?;
    m.add_function(wrap_pyfunction!(add_progress_bar, m)?)?;
    m.add_function(wrap_pyfunction!(add_radio, m)?)?;
    m.add_function(wrap_pyfunction!(add_row, m)?)?;
    m.add_function(wrap_pyfunction!(add_rule, m)?)?;
    m.add_function(wrap_pyfunction!(add_slider, m)?)?;
    m.add_function(wrap_pyfunction!(add_selectable_text, m)?)?;
    m.add_function(wrap_pyfunction!(add_separator, m)?)?;
    m.add_function(wrap_pyfunction!(add_space, m)?)?;
    m.add_function(wrap_pyfunction!(add_toggler, m)?)?;
    m.add_function(wrap_pyfunction!(add_text, m)?)?;
    m.add_function(wrap_pyfunction!(update_widget, m)?)?;
    
    // styles
    m.add_function(wrap_pyfunction!(add_button_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_checkbox_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_color_picker_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_container_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_divider_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_opaque_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_pick_list_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_progress_bar_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_separator_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_slider_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_toggler_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_radio_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_rule_style, m)?)?;

    // style parameters
    m.add_class::<IpgButtonStyleParam>()?;
    m.add_class::<IpgButtonStyleStandard>()?;
    m.add_class::<IpgCheckboxStyleParam>()?;
    m.add_class::<IpgContainerStyleParam>()?;
    m.add_class::<IpgDividerStyleParam>()?;
    m.add_class::<IpgRadioStyleParam>()?;
    m.add_class::<IpgRuleStyleParam>()?;
    m.add_class::<IpgSeparatorStyleParam>()?;
    m.add_class::<IpgSliderStyleParam>()?;
    m.add_class::<IpgStyleStandard>()?;
    m.add_class::<IpgTogglerStyleParam>()?;

    // widget params
    m.add_class::<IpgArrow>()?;
    m.add_class::<IpgButtonParam>()?;
    m.add_class::<IpgCheckboxParam>()?;
    m.add_class::<IpgColumnParam>()?;
    m.add_class::<IpgDatePickerParam>()?;
    m.add_class::<IpgDividerParam>()?;
    m.add_class::<IpgRadioParam>()?;
    m.add_class::<IpgRuleParam>()?;
    m.add_class::<IpgSelectableTextParam>()?;
    m.add_class::<IpgSeparatorParam>()?;
    m.add_class::<IpgSliderParam>()?;
    m.add_class::<IpgTextParam>()?;
    m.add_class::<IpgTogglerParam>()?;
    m.add_class::<IpgWindowLevel>()?;
    m.add_class::<IpgWindowMode>()?;
    m.add_class::<IpgWindowTheme>()?;

    // Enums
    m.add_class::<IpgAlignment>()?;
    m.add_class::<IpgColor>()?;
    m.add_class::<IpgIcon>()?;
    m.add_class::<IpgPickListHandle>()?;
    m.add_class::<IpgHorizontalAlignment>()?;
    m.add_class::<IpgVerticalAlignment>()?;
    m.add_class::<IpgRadioDirection>()?;
    m.add_class::<IpgSeparatorType>()?;
    Ok(())
}
