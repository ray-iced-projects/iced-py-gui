#!/usr/bin/env python3
"""
helper file for creating a palette
"""

from typing import TYPE_CHECKING
from enum import Enum
from icedpygui import (
    add_button,
    add_button_style,
    ButtonStyleParam,
    add_checkbox,
    add_checkbox_style,
    CheckboxStyleParam,
    update_widget_params,
)

if TYPE_CHECKING:
    from python_examples.py_palette.py_palette_create import PaletteCreator





BUTTON_WIDTH = 150

class Widget(Enum):
    """List of widgets"""
    BUTTON = "button"
    CHECKBOX = "checkbox"
    CARD = "card"
    MENU = "menu"
    PICK_LIST = "pick_list"
    RADIO = "radio"
    PROGRESS_BAR = "progress_bar"
    SASH = "sash"
    SCROLLABLE = "scrollable"
    SLIDER = "slider"
    TEXT_EDITOR = "text_editor"
    TEXT_INPUT = "text_input"
    TOGGLE = "toggle"
    CONTAINER = "container"
    COLOR_PICKER = "color_picker"
    DATE_PICKER = "date_picker"
    RULE = "rule"
    SVG = "svg"
    TEXT = "text"
    TOOL_TIP = "tool_tip"


def place_widgets(pc: PaletteCreator, widget: Widget):
    """Add the selected widget with status"""
    match widget:
        case Widget.BUTTON:
            pc.widget_active_style_id = add_button_style()
            pc.widget_active_id = add_button(
                label="Status=Active",
                parent_id=pc.new_widget_row_id,
                active=True,
                padding=[10],
                width=BUTTON_WIDTH,
                style_id=pc.widget_active_style_id
                )

            pc.widget_hovered_style_id = add_button_style()
            pc.widget_hovered_id = add_button(
                label="Status=Hovered",
                parent_id=pc.new_widget_row_id,
                hovered=True,
                padding=[10],
                width=BUTTON_WIDTH,
                style_id=pc.widget_hovered_style_id
                )

            pc.widget_pressed_style_id = add_button_style()
            pc.widget_pressed_id = add_button(
                label="Status=Pressed",
                parent_id=pc.new_widget_row_id,
                pressed=True,
                padding=[10],
                width=BUTTON_WIDTH,
                style_id=pc.widget_pressed_style_id
                )

            pc.widget_disabled_style_id = add_button_style()
            pc.widget_disabled_id = add_button(
                label="Status=Disabled",
                parent_id=pc.new_widget_row_id,
                disabled=True,
                padding=[10],
                width=BUTTON_WIDTH,
                style_id=pc.widget_disabled_style_id
                )

        case Widget.CHECKBOX:
            pc.widget_active_style_id = add_checkbox_style()
            pc.widget_active_id = add_checkbox(
                label="Status=Active",
                parent_id=pc.new_widget_row_id,
                active=True,
                width=BUTTON_WIDTH,
                style_id=pc.widget_active_style_id
                )

            pc.widget_hovered_style_id = add_checkbox_style()
            pc.widget_hovered_id = add_checkbox(
                label="Status=Hovered",
                parent_id=pc.new_widget_row_id,
                hovered=True,
                width=BUTTON_WIDTH,
                style_id=pc.widget_hovered_style_id
                )

            pc.widget_disabled_style_id = add_checkbox_style()
            pc.widget_disabled_id = add_checkbox(
                label="Status=Disabled",
                parent_id=pc.new_widget_row_id,
                disabled=True,
                width=BUTTON_WIDTH,
                style_id=pc.widget_disabled_style_id
                )

# the widget and palette is found by row=status_index, col=pal_idx of the matrix
def set_widget_status(pc: PaletteCreator, pal_idx: int, status_index: int):
    """Radio select for status"""
    statuses = pc.widget_parts.get("statuses")
    _pal, bkg_rgba = list(pc.palette.items())[pal_idx*2]
    _text_pal, bkg_text_color = list(pc.palette.items())[pal_idx*2 + 1]
    status = statuses[status_index]

    # update the widget
    match pc.widget_name:
        case "button":
            match status:
                case "Active":
                    update_widget_params(
                        pc.widget_active_style_id, {
                        ButtonStyleParam.BkgRgba: bkg_rgba,
                        ButtonStyleParam.TextRgba: bkg_text_color
                        })
                case "Hovered":
                    update_widget_params(
                        pc.widget_hovered_style_id, {
                        ButtonStyleParam.BkgRgba: bkg_rgba,
                        ButtonStyleParam.TextRgba: bkg_text_color
                        })
                case "Pressed":
                    update_widget_params(
                        pc.widget_pressed_style_id, {
                        ButtonStyleParam.BkgRgba: bkg_rgba,
                        ButtonStyleParam.TextRgba: bkg_text_color
                        })
                case "Disabled":
                    update_widget_params(
                        pc.widget_disabled_style_id, {
                        ButtonStyleParam.BkgRgba: bkg_rgba,
                        ButtonStyleParam.TextRgba: bkg_text_color
                        })
        case "checkbox":
            match status:
                case "Active":
                    update_widget_params(
                        pc.widget_active_style_id, {
                        CheckboxStyleParam.BkgRgba: bkg_rgba,
                        CheckboxStyleParam.TextRgba: bkg_text_color
                        })
                case "Hovered":
                    update_widget_params(
                        pc.widget_hovered_style_id, {
                        CheckboxStyleParam.BkgRgba: bkg_rgba,
                        CheckboxStyleParam.TextRgba: bkg_text_color
                        })
                case "Disabled":
                    update_widget_params(
                        pc.widget_disabled_style_id, {
                        CheckboxStyleParam.BkgRgba: bkg_rgba,
                        CheckboxStyleParam.TextRgba: bkg_text_color
                        })


def match_widget_str(w_str: str):
    """Match the str to return the Widget"""
    match w_str.lower():
        case "button":
            return Widget.BUTTON
        case "checkbox":
            return Widget.CHECKBOX
        case "card":
            return Widget.CARD
        case "menu":
            return Widget.MENU
        case "pick_list":
            return Widget.PICK_LIST
        case "radio":
            return Widget.RADIO
        case "progress_bar":
            return Widget.PROGRESS_BAR
        case "sash":
            return Widget.SASH
        case "scrollable":
            return Widget.SCROLLABLE
        case "slider":
            return Widget.SLIDER
        case "text_editor":
            return Widget.TEXT_EDITOR
        case "text_input":
            return Widget.TEXT_INPUT
        case "toggle":
            return Widget.TOGGLE
        case "container":
            return Widget.CONTAINER
        case "color_picker":
            return Widget.COLOR_PICKER
        case "date_picker":
            return Widget.DATE_PICKER
        case "rule":
            return Widget.RULE
        case "svg":
            return Widget.SVG
        case "text":
            return Widget.TEXT
        case "tool_tip":
            return Widget.TOOL_TIP
        case _:
            return None
