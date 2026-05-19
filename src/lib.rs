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
mod style;
mod iced_aw_widgets;
mod ipg_widgets;

// Minimal widget definitions (self-contained)
mod widgets;

// Re-export for internal use
pub use state::{
    access_state, access_callbacks, access_user_data1,
    add_callback_to_mutex, add_user_data_to_mutex, clone_state_to_runtime,
    WidgetNode, IpgState
};

use crate::ipg_widgets::ipg_canvas_draw::canvas_draw::{DrawMode, DrawWidget};
// Import pyfunctions from py_api modules
use crate::py_api::window::add_window;
use crate::py_api::button::{add_button, add_button_style};
use crate::py_api::clipboard::{clipboard_read, clipboard_write};
use crate::py_api::card::{add_card, add_card_style};
use crate::py_api::checkbox::{add_checkbox, add_checkbox_style};
use crate::py_api::colors::{get_color_palette, get_rgba_color};
use crate::py_api::color_picker::{add_color_picker};
use crate::py_api::column::add_column;
use crate::py_api::combo_box::add_combobox;
use crate::py_api::container::{add_container, add_container_style};
use crate::py_api::date_picker::add_date_picker;
use crate::py_api::draw::add_draw;
use crate::py_api::draw_update::{update_draw_params, delete_draw_widget};
use crate::py_api::events::{add_event_keyboard, add_event_mouse};
use crate::py_api::float::add_float;
use crate::py_api::grid::add_grid;
use crate::py_api::font::{add_font_style, add_icon, load_font};
use crate::py_api::image::add_image;
use crate::py_api::menu::{add_menu, add_menu_bar_item, add_menu_sub_item, add_menu_style};
use crate::py_api::mouse_area::add_mouse_area;
use crate::py_api::opaque::add_opaque;
use crate::py_api::progress_bar::{add_progress_bar, add_progress_bar_style};
use crate::py_api::radio::{add_radio, add_radio_style};
use crate::py_api::row::add_row;
use crate::py_api::rule::{add_rule, add_rule_style};
use crate::py_api::picklist::{add_pick_list, add_pick_list_style};
use crate::py_api::scrollable::{add_scrollable, add_scrollable_style, 
    add_scroller, add_autoscroll_style, add_rail_style};
use crate::py_api::separator::{add_separator, add_separator_style};
use crate::py_api::session::{start_session, generate_id};
use crate::py_api::slider::{add_slider, add_slider_style};
use crate::py_api::space::add_space;
use crate::py_api::stack::add_stack;
use crate::py_api::splitter::{add_splitter_h, add_splitter_v, add_splitter_style};
use crate::py_api::svg::add_svg;
use crate::py_api::table::{add_table, add_table_style};
use crate::py_api::text_input::{add_text_input, add_text_input_style};
use crate::py_api::text::add_text;
use crate::py_api::text_editor::{add_text_editor, add_text_editor_style};
use crate::py_api::text_rich::{add_rich_text, add_span};
use crate::py_api::toggle::{add_toggler, add_toggler_style};
use crate::py_api::tool_tip::add_tool_tip;
use crate::py_api::update::{update_widget, update_widget_params, delete_widget, hide_widget, move_widget, show_widget};

// Import enums from widgets module
use crate::widgets::enums::ContentFit;
use crate::widgets::ipg_draw::DrawParam;
use crate::widgets::ipg_font::{FontFamily, FontStretch, FontStyle, FontWeight};
use crate::widgets::ipg_image::ImageParam;
use crate::widgets::ipg_mouse_area::MousePointer;
use crate::widgets::ipg_progress_bar::{ProgressBarParam, ProgressBarStyleParam, ProgressBarStyleStd};
use crate::widgets::styling::StyleStandard;
use crate::graphics::{bootstrap_icon::Icon, bootstrap_arrow::Arrow};
use crate::graphics::colors::Color;

use crate::widgets::ipg_button::{ButtonParam, ButtonStyleParam, ButtonStyleStd};
use crate::widgets::ipg_card::{CardParam, CardStyleParam, CardStyleStd};
use crate::widgets::ipg_checkbox::{CheckboxParam, CheckboxStyleParam, CheckboxStyleStd};
use crate::widgets::ipg_column::ColumnParam;
use crate::widgets::ipg_container::{ContainerParam, ContainerStyleParam, ContainerStyleStd};
use crate::widgets::ipg_date_picker::DatePickerParam;
use crate::widgets::ipg_float::FloatParam;
use crate::widgets::ipg_grid::GridParam;
use crate::widgets::ipg_menu::{MenuBarItemParam, MenuParam, MenuStyleParam, MenuSubItemParam};
use crate::widgets::ipg_radio::{RadioParam, RadioStyleParam};
use crate::widgets::ipg_row::RowParam;
use crate::widgets::ipg_rule::{RuleParam, RuleStyleParam};
use crate::widgets::ipg_scrollable::{AutoScrollStyleParam, RailStyleParam, ScrollableParam, ScrollableStyleParam, ScrollerParam};
use crate::widgets::ipg_separator::{SeparatorParam, SeparatorStyleParam};
use crate::widgets::ipg_slider::{SliderParam, SliderStyleParam};
use crate::widgets::ipg_stack::StackParam;
use crate::widgets::ipg_splitter::{SplitterHParam, SplitterVParam, SplitterStyleParam};
use crate::widgets::ipg_svg::SvgParam;
use crate::widgets::ipg_table::{TableParam, TableStyleParam};
use crate::widgets::ipg_text_input::{TextInputParam, TextInputStyleParam};
use crate::widgets::ipg_text_rich::{RichTextParam, SpanParam};
use crate::widgets::ipg_text::{TextColorStd, TextParam};
use crate::widgets::ipg_timer::{TimerParam, update_timer};
use crate::widgets::ipg_toggle::{TogglerParam, TogglerStyleParam};
use crate::widgets::ipg_tool_tip::ToolTipParam;
use crate::widgets::ipg_window::{WindowLevel, WindowMode, WindowTheme, WindowParam};

// events
use crate::py_api::events::add_event_window;
use crate::py_api::timer::add_event_timer;


/// Python module definition
#[pymodule]
fn icedpygui(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Session functions
    m.add_function(wrap_pyfunction!(start_session, m)?)?;
    m.add_function(wrap_pyfunction!(generate_id, m)?)?;

    // Event functions
    m.add_function(wrap_pyfunction!(add_event_window, m)?)?;
    m.add_function(wrap_pyfunction!(add_event_timer, m)?)?;
    m.add_function(wrap_pyfunction!(add_event_keyboard, m)?)?;
    m.add_function(wrap_pyfunction!(add_event_mouse, m)?)?;
    
    // widgets
    m.add_function(wrap_pyfunction!(add_autoscroll_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_button_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_button, m)?)?;
    m.add_function(wrap_pyfunction!(clipboard_read, m)?)?;
    m.add_function(wrap_pyfunction!(clipboard_write, m)?)?;
    m.add_function(wrap_pyfunction!(add_card_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_card, m)?)?;
    m.add_function(wrap_pyfunction!(add_checkbox_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_checkbox, m)?)?;
    m.add_function(wrap_pyfunction!(add_color_picker, m)?)?;
    m.add_function(wrap_pyfunction!(add_column, m)?)?;
    m.add_function(wrap_pyfunction!(add_combobox, m)?)?;
    m.add_function(wrap_pyfunction!(add_container_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_container, m)?)?;
    m.add_function(wrap_pyfunction!(add_date_picker, m)?)?;
    m.add_function(wrap_pyfunction!(add_draw, m)?)?;
    m.add_function(wrap_pyfunction!(add_float, m)?)?;
    m.add_function(wrap_pyfunction!(add_font_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_icon, m)?)?;
    m.add_function(wrap_pyfunction!(add_grid, m)?)?;
    m.add_function(wrap_pyfunction!(add_image, m)?)?;
    m.add_function(wrap_pyfunction!(add_menu, m)?)?;
    m.add_function(wrap_pyfunction!(add_menu_bar_item, m)?)?;
    m.add_function(wrap_pyfunction!(add_menu_sub_item, m)?)?;
    m.add_function(wrap_pyfunction!(add_menu_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_mouse_area, m)?)?;
    m.add_function(wrap_pyfunction!(add_opaque, m)?)?;
    m.add_function(wrap_pyfunction!(add_pick_list_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_pick_list, m)?)?;
    m.add_function(wrap_pyfunction!(add_progress_bar_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_progress_bar, m)?)?;
    m.add_function(wrap_pyfunction!(add_radio_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_radio, m)?)?;
    m.add_function(wrap_pyfunction!(add_rail_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_rich_text, m)?)?;
    m.add_function(wrap_pyfunction!(add_row, m)?)?;
    m.add_function(wrap_pyfunction!(add_rule_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_rule, m)?)?;
    m.add_function(wrap_pyfunction!(add_scrollable_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_scrollable, m)?)?;
    m.add_function(wrap_pyfunction!(add_scroller, m)?)?;
    m.add_function(wrap_pyfunction!(add_separator_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_separator, m)?)?;
    m.add_function(wrap_pyfunction!(add_slider_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_slider, m)?)?;
    m.add_function(wrap_pyfunction!(add_space, m)?)?;
    m.add_function(wrap_pyfunction!(add_span, m)?)?;
    m.add_function(wrap_pyfunction!(add_stack, m)?)?;
    m.add_function(wrap_pyfunction!(add_splitter_h, m)?)?;
    m.add_function(wrap_pyfunction!(add_splitter_v, m)?)?;
    m.add_function(wrap_pyfunction!(add_splitter_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_svg, m)?)?;
    m.add_function(wrap_pyfunction!(add_svg, m)?)?;
    m.add_function(wrap_pyfunction!(add_table, m)?)?;
    m.add_function(wrap_pyfunction!(add_table_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_text_editor, m)?)?;
    m.add_function(wrap_pyfunction!(add_text_editor_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_text_input_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_text_input, m)?)?;
    m.add_function(wrap_pyfunction!(add_text, m)?)?;
    m.add_function(wrap_pyfunction!(add_toggler_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_toggler_style, m)?)?;
    m.add_function(wrap_pyfunction!(add_toggler, m)?)?;
    m.add_function(wrap_pyfunction!(add_tool_tip, m)?)?;
    
    m.add_function(wrap_pyfunction!(add_window, m)?)?;
    m.add_function(wrap_pyfunction!(delete_widget, m)?)?;
    m.add_function(wrap_pyfunction!(hide_widget, m)?)?;
    m.add_function(wrap_pyfunction!(load_font, m)?)?;
    m.add_function(wrap_pyfunction!(load_font, m)?)?;
    m.add_function(wrap_pyfunction!(move_widget, m)?)?;
    m.add_function(wrap_pyfunction!(show_widget, m)?)?;
    m.add_function(wrap_pyfunction!(update_timer, m)?)?;
    m.add_function(wrap_pyfunction!(update_widget_params, m)?)?;
    m.add_function(wrap_pyfunction!(update_widget, m)?)?;
    m.add_function(wrap_pyfunction!(update_draw_params, m)?)?;
    m.add_function(wrap_pyfunction!(delete_draw_widget, m)?)?;

    // Widget parameters
    m.add_class::<Arrow>()?;
    m.add_class::<AutoScrollStyleParam>()?;
    m.add_class::<ButtonParam>()?;
    m.add_class::<ButtonStyleParam>()?;
    m.add_class::<ButtonStyleStd>()?;
    m.add_class::<CardParam>()?;
    m.add_class::<CardStyleParam>()?;
    m.add_class::<CardStyleStd>()?;
    m.add_class::<CheckboxParam>()?;
    m.add_class::<CheckboxStyleParam>()?;
    m.add_class::<CheckboxStyleParam>()?;
    m.add_class::<CheckboxStyleStd>()?;
    m.add_class::<ColumnParam>()?;
    m.add_class::<ContainerParam>()?;
    m.add_class::<ContainerParam>()?;
    m.add_class::<ContainerStyleParam>()?;
    m.add_class::<ContainerStyleStd>()?;
    m.add_class::<DatePickerParam>()?;
    m.add_class::<DrawMode>()?;
    m.add_class::<DrawParam>()?;
    m.add_class::<DrawWidget>()?;
    m.add_class::<FloatParam>()?;
    m.add_class::<GridParam>()?;
    m.add_class::<ImageParam>()?;
    m.add_class::<MenuBarItemParam>()?;
    m.add_class::<MenuParam>()?;
    m.add_class::<MenuStyleParam>()?;
    m.add_class::<MenuSubItemParam>()?;
    m.add_class::<ProgressBarParam>()?;
    m.add_class::<ProgressBarStyleParam>()?;
    m.add_class::<ProgressBarStyleStd>()?;
    m.add_class::<RadioParam>()?;
    m.add_class::<RadioStyleParam>()?;
    m.add_class::<RailStyleParam>()?;
    m.add_class::<RichTextParam>()?;
    m.add_class::<RowParam>()?;
    m.add_class::<RuleParam>()?;
    m.add_class::<RuleStyleParam>()?;
    m.add_class::<ScrollableParam>()?;
    m.add_class::<ScrollableStyleParam>()?;
    m.add_class::<ScrollerParam>()?;
    m.add_class::<SeparatorParam>()?;
    m.add_class::<SeparatorStyleParam>()?;
    m.add_class::<SliderParam>()?;
    m.add_class::<SliderStyleParam>()?;
    m.add_class::<SpanParam>()?;
    m.add_class::<StackParam>()?;
    m.add_class::<SplitterHParam>()?;
    m.add_class::<SplitterVParam>()?;
    m.add_class::<SplitterStyleParam>()?;
    m.add_class::<StyleStandard>()?;
    m.add_class::<SvgParam>()?;
    m.add_class::<TableParam>()?;
    m.add_class::<TableStyleParam>()?;
    m.add_class::<TextColorStd>()?;
    m.add_class::<TextInputParam>()?;
    m.add_class::<TextInputStyleParam>()?;
    m.add_class::<TextParam>()?;
    m.add_class::<TimerParam>()?;
    m.add_class::<TogglerParam>()?;
    m.add_class::<TogglerStyleParam>()?;
    m.add_class::<ToolTipParam>()?;
    m.add_class::<WindowLevel>()?;
    m.add_class::<WindowMode>()?;
    m.add_class::<WindowParam>()?;
    m.add_class::<WindowTheme>()?;

    //Color functions
    m.add_function(wrap_pyfunction!(get_rgba_color, m)?)?;
    m.add_function(wrap_pyfunction!(get_color_palette, m)?)?;
    
    // Enums
    m.add_class::<Arrow>()?;
    m.add_class::<Color>()?;
    m.add_class::<Icon>()?;
    m.add_class::<ContentFit>()?;
    m.add_class::<FontFamily>()?;
    m.add_class::<FontWeight>()?;
    m.add_class::<FontStretch>()?;
    m.add_class::<FontStyle>()?;
    m.add_class::<MousePointer>()?;
    Ok(())
}
