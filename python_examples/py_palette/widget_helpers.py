#!/usr/bin/env python3
"""
helper file for creating a palette
"""

from dataclasses import dataclass, field
from typing import TYPE_CHECKING
from enum import Enum
from pathlib import Path
import os
import yaml

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

DEMO_FILE_DIR = Path.home() / ".python_examples/py_palette"
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


def match_widget_str(w_str: str) -> Widget:
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

def populate_widget_config(pc: PaletteCreator):
    """Saves the widget config to a .yml file"""
    match pc.widget_name:
        case "button":
            pass
        case "checkbox":
            pass

@dataclass
class WidgetConfig:
    """Editable widget palette config.

    `mappings` is keyed by (status, variant); each value maps a part name
    ("Background", "Text", "Border", "Icon") to its PartRule.
    """
    widget: str = ""
    selected_color: list[float] = field(default_factory=lambda: [0.0, 0.0, 0.0, 1.0])
    statuses: list[str] = field(default_factory=list)
    state_variants: list[str] = field(default_factory=list)
    parts: list[str] = field(default_factory=list)
    mappings: dict[tuple[str, str], dict[str, PartRule]] = field(default_factory=dict)

    @classmethod
    def from_config_dict(cls, data: dict) -> "WidgetConfig":
        """Build a WidgetConfig from a parsed YAML config dict."""
        cfg = cls(
            widget=data.get("widget", ""),
            selected_color=list(data.get("selected_color", [0.0, 0.0, 0.0, 1.0])),
            statuses=list(data.get("statuses", [])),
            state_variants=list(data.get("state_variants", [])),
            parts=list(data.get("parts", [])),
        )
        for mapping in data.get("palette_mappings", []):
            key = (mapping.get("status"), mapping.get("variant"))
            rules: dict[str, PartRule] = {}
            for part in mapping.get("parts", []):
                rule = PartRule(
                    part=part.get("part"),
                    key=part.get("key"),
                    alpha=part.get("alpha", 1.0),
                    description=part.get("description", ""),
                )
                rules[rule.part] = rule
            cfg.mappings[key] = rules
        return cfg

    def get_rule(self, status: str, variant: str, part: str) -> PartRule | None:
        """Return the PartRule for a status/variant/part, or None."""
        return self.mappings.get((status, variant), {}).get(part)

    def set_rule(self, status: str, variant: str, part: str,
                 key: str, alpha: float = 1.0, description: str = ""):
        """Create or overwrite a rule for a status/variant/part."""
        self.mappings.setdefault((status, variant), {})[part] = PartRule(
            part=part, key=key, alpha=alpha, description=description)

    def remove_rule(self, status: str, variant: str, part: str):
        """Remove a rule; drops the mapping entry when it becomes empty."""
        rules = self.mappings.get((status, variant))
        if rules and part in rules:
            del rules[part]
            if not rules:
                del self.mappings[(status, variant)]

    def to_config_dict(self) -> dict:
        """Serialize back to the YAML config dict shape."""
        palette_mappings = [
            {
                "status": status,
                "variant": variant,
                "parts": [
                    {
                        "part": r.part,
                        "key": r.key,
                        "alpha": r.alpha,
                        "description": r.description,
                    }
                    for r in rules.values()
                ],
            }
            for (status, variant), rules in self.mappings.items()
        ]
        return {
            "widget": self.widget,
            "selected_color": self.selected_color,
            "statuses": self.statuses,
            "state_variants": self.state_variants,
            "parts": self.parts,
            "palette_mappings": palette_mappings,
        }

@dataclass
class PartRule:
    """A single style part rule within a status/variant mapping."""
    part: str
    key: str
    alpha: float = 1.0
    description: str = ""


def load_demo_config(pc: PaletteCreator) -> PaletteCreator:
    """Load the demo widget configs file"""
    # Local import breaks the circular dependency with py_palette_create.
    # from python_examples.py_palette.py_palette_create import WidgetConfig

    cwd = os.getcwd()

    file_path = os.path.join(cwd, "python_examples",
                            "py_palette", f"{pc.widget_name}_palette_config.yml")
    try:
        with open(file_path, "r", encoding='utf-8') as file:
            config_file = file.read()
    except FileNotFoundError:
        print(f"*********The file does not exist using {file_path}.*******")
        return pc

    match pc.widget_name:
        case "button" | "checkbox":
            pc.widget_config = WidgetConfig.from_config_dict(yaml.safe_load(config_file))

    return pc
