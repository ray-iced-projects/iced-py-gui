//! ipg_enums
#![allow(clippy::enum_variant_names)]
use pyo3::pyclass;

use crate::ipg_widgets::ipg_button::{IpgButton, IpgButtonStyle};
use crate::ipg_widgets::ipg_canvas::IpgCanvas;
use crate::ipg_widgets::ipg_card::{IpgCard, IpgCardStyle};
use crate::ipg_widgets::ipg_checkbox::{IpgCheckBox, IpgCheckboxStyle};
use crate::ipg_widgets::ipg_color_picker::{IpgColorPicker, IpgColorPickerStyle};
// use super::ipg_color_picker::IpgColorPicker;
use crate::ipg_widgets::ipg_container::{IpgContainer, IpgContainerStyle};
use crate::ipg_widgets::ipg_column::IpgColumn;
use crate::ipg_widgets::ipg_date_picker::IpgDatePicker;
use crate::ipg_widgets::ipg_divider::{IpgDividerHorizontal, IpgDividerVertical, IpgDividerStyle};
use crate::ipg_widgets::ipg_image::IpgImage;
use crate::ipg_widgets::ipg_menu::{IpgMenu, IpgMenuBarStyle, IpgMenuStyle};
// use super::ipg_modal::IpgModal;
use crate::ipg_widgets::ipg_mousearea::IpgMouseArea;
use crate::ipg_widgets::ipg_opaque::{IpgOpaque, IpgOpaqueStyle};
// use super::ipg_pane_grid::{IpgPaneGrid, IpgPane};
use crate::ipg_widgets::ipg_pick_list::{IpgPickList, IpgPickListStyle};
use crate::ipg_widgets::ipg_progress_bar::{IpgProgressBar, IpgProgressBarStyle};
use crate::ipg_widgets::ipg_radio::{IpgRadio, IpgRadioStyle};
use crate::ipg_widgets::ipg_row::IpgRow;
use crate::ipg_widgets::ipg_rule::{IpgRule, IpgRuleStyle};
use crate::ipg_widgets::ipg_scrollable::{IpgScrollable, IpgScrollableStyle};
use crate::ipg_widgets::ipg_selectable_text::IpgSelectableText;
use crate::ipg_widgets::ipg_separator::{IpgSeparator, IpgSeparatorStyle};
use crate::ipg_widgets::ipg_slider::{IpgSlider, IpgSliderStyle};
use crate::ipg_widgets::ipg_space::IpgSpace;
use crate::ipg_widgets::ipg_stack::IpgStack;
use crate::ipg_widgets::ipg_svg::IpgSvg;
use crate::ipg_widgets::ipg_table::{IpgTable, IpgTableStyle};
use crate::ipg_widgets::ipg_text::IpgText;
// use super::ipg_text_editor::IpgTextEditor;
use crate::ipg_widgets::ipg_text_input::{IpgTextInput, IpgTextInputStyle};
// use super::ipg_text_rich::IpgRichText;
use crate::ipg_widgets::ipg_timer::{IpgTimer, IpgTimerStyle};
use crate::ipg_widgets::ipg_timer_canvas::{IpgCanvasTimer, IpgCanvasTimerStyle};
use crate::ipg_widgets::ipg_toggle::{IpgToggler, IpgTogglerStyle};
use crate::ipg_widgets::ipg_tool_tip::{IpgToolTip, IpgToolTipStyle};
use crate::ipg_widgets::ipg_window::IpgWindow;


#[derive(Debug, Clone)]
pub enum IpgContainers {
    IpgCanvas(IpgCanvas),
    IpgColumn(IpgColumn),
    IpgContainer(IpgContainer),
    IpgMenu(IpgMenu),
    // IpgModal(IpgModal),
    IpgMouseArea(IpgMouseArea),
    IpgOpaque(IpgOpaque),
    IpgStack(IpgStack),
    IpgTable(IpgTable),
    // IpgPaneGrid(IpgPaneGrid),
    // IpgPane(IpgPane),
    IpgRow(IpgRow),
    IpgScrollable(IpgScrollable),
    IpgToolTip(IpgToolTip),
    IpgWindow(IpgWindow),
}

#[derive(Debug, Clone)]
pub enum IpgWidgets {
    IpgButton(IpgButton),
    IpgButtonStyle(IpgButtonStyle),
    IpgCard(IpgCard),
    IpgCardStyle(IpgCardStyle),
    IpgCheckBox(IpgCheckBox),
    IpgCheckboxStyle(IpgCheckboxStyle),
    IpgColorPicker(IpgColorPicker),
    IpgColorPickerStyle(IpgColorPickerStyle),
    IpgContainerStyle(IpgContainerStyle),
    IpgDividerHorizontal(IpgDividerHorizontal),
    IpgDividerVertical(IpgDividerVertical),
    IpgDividerStyle(IpgDividerStyle),
    IpgDatePicker(IpgDatePicker),
    IpgImage(IpgImage),
    IpgMenuStyle(IpgMenuStyle),
    IpgMenuBarStyle(IpgMenuBarStyle),
    IpgOpaqueStyle(IpgOpaqueStyle),
    IpgPickList(IpgPickList),
    IpgPickListStyle(IpgPickListStyle),
    IpgProgressBar(IpgProgressBar),
    IpgProgressBarStyle(IpgProgressBarStyle),
    IpgRadio(IpgRadio),
    IpgRadioStyle(IpgRadioStyle),
    IpgRule(IpgRule),
    IpgRuleStyle(IpgRuleStyle),
    IpgScrollableStyle(IpgScrollableStyle),
    IpgSelectableText(IpgSelectableText),
    IpgSeparator(IpgSeparator),
    IpgSeparatorStyle(IpgSeparatorStyle),
    IpgSlider(IpgSlider),
    IpgSliderStyle(IpgSliderStyle),
    IpgSpace(IpgSpace),
    IpgSvg(IpgSvg),
    IpgTableStyle(IpgTableStyle),
    IpgText(IpgText),
    // IpgRichText(IpgRichText),
    // IpgTextEditor(IpgTextEditor),
    IpgTextInput(IpgTextInput),
    IpgTextInputStyle(IpgTextInputStyle),
    IpgTimer(IpgTimer),
    IpgTimerStyle(IpgTimerStyle),
    IpgCanvasTimer(IpgCanvasTimer),
    IpgCanvasTimerStyle(IpgCanvasTimerStyle),
    IpgToggler(IpgToggler),
    IpgTogglerStyle(IpgTogglerStyle),
    IpgToolTipStyle(IpgToolTipStyle),
}


#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgAlignment {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgHorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
#[pyclass(eq, eq_int)]
pub enum IpgVerticalAlignment {
    Top,
    Center,
    Bottom,
}

pub fn get_alignment(align: IpgAlignment) -> Alignment {

    match align {
        IpgAlignment::Start => Alignment::Start,
        IpgAlignment::Center => Alignment::Center,
        IpgAlignment::End => Alignment::End,
    }
}

pub fn get_horizontal_alignment(h_align: &IpgHorizontalAlignment) -> Horizontal {

    match h_align {
        IpgHorizontalAlignment::Left => Horizontal::Left,
        IpgHorizontalAlignment::Center => Horizontal::Center,
        IpgHorizontalAlignment::Right => Horizontal::Right,
    }
}

pub fn get_vertical_alignment(v_align: &IpgVerticalAlignment) -> Vertical {
    
    match v_align {
        IpgVerticalAlignment::Top => Vertical::Top,
        IpgVerticalAlignment::Center => Vertical::Center,
        IpgVerticalAlignment::Bottom => Vertical::Bottom,
    }
}

// These alignments return options so that only the canvas text alignment needs one py value type
pub fn try_extract_ipg_horizontal_alignment(value: &PyObject) 
        -> Option<IpgHorizontalAlignment> {
    Python::attach(|py| {

        let res = value.extract::<IpgHorizontalAlignment>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })
}

pub fn try_extract_ipg_vertical_alignment(value: &PyObject) -> Option<IpgVerticalAlignment> {
    Python::attach(|py| {

        let res = value.extract::<IpgVerticalAlignment>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })
}

pub fn try_extract_ipg_alignment(value: &PyObject) -> Option<IpgAlignment> {
    Python::attach(|py| {

        let res = value.extract::<IpgAlignment>(py);
        match res {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    })
}
