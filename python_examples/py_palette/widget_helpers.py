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


def place_widget(pc: PaletteCreator, widget: Widget):
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
        """Build a WidgetConfig from a parsed YAML config dict (part-oriented).

        Converts part-oriented YAML structure to internal (status, variant) -> parts structure.
        YAML structure:
          - part: Background
            statuses:
              Active:               # Single-variant: {key, alpha, description}
                key: Base
                alpha: 1.0
              OR
              Active:               # Multi-variant: {Unchecked: {...}, Checked: {...}}
                Unchecked:
                  key: Base
                  alpha: 1.0
                Checked:
                  key: Strong
                  alpha: 1.0
        """
        cfg = cls(
            widget=data.get("widget", ""),
            selected_color=list(data.get("selected_color", [0.0, 0.0, 0.0, 1.0])),
            statuses=list(data.get("statuses", [])),
            state_variants=list(data.get("state_variants", [])),
            parts=list(data.get("parts", [])),
        )
        # Parse part-oriented YAML and convert to (status, variant) -> parts structure
        for part_mapping in data.get("palette_mappings", []):
            part_name = part_mapping.get("part")
            statuses_dict = part_mapping.get("statuses", {})

            for status, status_value in statuses_dict.items():
                if not isinstance(status_value, dict):
                    continue

                # Detect structure: "key" in dict means single-variant, else multi-variant
                if "key" in status_value:
                    # Single-variant case: {key, alpha, description}
                    variant = cfg.state_variants[0] if cfg.state_variants else "NoVariant"
                    rule = PartRule(
                        part=part_name,
                        key=status_value.get("key"),
                        alpha=status_value.get("alpha", 1.0),
                        description=status_value.get("description", ""),
                    )
                    cfg.mappings.setdefault((status, variant), {})[part_name] = rule
                else:
                    # Multi-variant case: {variant1: {...}, variant2: {...}}
                    for variant, rule_data in status_value.items():
                        if isinstance(rule_data, dict) and "key" in rule_data:
                            rule = PartRule(
                                part=part_name,
                                key=rule_data.get("key"),
                                alpha=rule_data.get("alpha", 1.0),
                                description=rule_data.get("description", ""),
                            )
                            cfg.mappings.setdefault((status, variant), {})[part_name] = rule
        return cfg

    @classmethod
    def get_unique_parts_status_from_file(cls, parts_file: str) -> list[str]:
        """Extract all unique parts and statuses across all widgets from the parts YAML file.

        Args:
            parts_filea: YAML file content as a string (widget_palette_parts.yml)

        Returns:
            Sorted list of unique part names (e.g., ['Background', ...])
        """
        try:
            with open(parts_file, encoding='utf-8') as stream:
                data = yaml.safe_load(stream)

            if not data:
                print("yml file not found")
                return []

            unique_parts = set()
            unique_statuses = set()

            # Iterate through all widget definitions in the YAML
            for _widget_name, widget_config in data.items():
                # Skip non-dict entries (like file_type, version)
                if isinstance(widget_config, dict) and 'parts' in widget_config:
                    parts_list = widget_config['parts']
                    if isinstance(parts_list, list):
                        unique_parts.update(parts_list)
                if isinstance(widget_config, dict) and 'statuses' in widget_config:
                    status_list = widget_config['statuses']
                    if isinstance(status_list, list):
                        unique_statuses.update(status_list)

            # Return sorted list for consistent ordering
            return (sorted(list(unique_parts)), sorted(list(unique_statuses)))

        except Exception as e:
            print(f"Error parsing parts file: {e}")
            return []

    @classmethod
    def get_widget_names_from_file(cls, parts_file: str) -> list[str]:
        """Extract all widgets names across all widgets from the parts YAML file.

        Args:
            parts_file: YAML file content as a string (widget_palette_parts.yml)

        Returns:
            Sorted list of names (e.g., ['button', ...])
        """
        try:
            with open(parts_file, encoding='utf-8') as stream:
                data = yaml.safe_load(stream)

            if not data:
                print("yml file not found")
                return []

            names = []

            # Iterate through all widget definitions in the YAML
            for name, _widget_config in data.items():
                names.append(name)

            # Return sorted list for consistent ordering
            return sorted(list(names))

        except Exception as e:
            print(f"Error parsing parts file: {e}")
            return []

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
        """Serialize back to the part-oriented YAML config dict shape."""
        # Reorganize from (status, variant) -> parts to parts -> statuses
        parts_index: dict[str, dict] = {}

        for (status, variant), rules in self.mappings.items():
            for part_name, rule in rules.items():
                if part_name not in parts_index:
                    parts_index[part_name] = {"part": part_name, "statuses": {}}

                if status not in parts_index[part_name]["statuses"]:
                    parts_index[part_name]["statuses"][status] = {}

                # Store rule data indexed by variant if multi-variant, else directly
                rule_data = {
                    "key": rule.key,
                    "alpha": rule.alpha,
                    "description": rule.description,
                }

                if len(self.state_variants) > 1:
                    # Multi-variant: nest under variant key
                    parts_index[part_name]["statuses"][status][variant] = rule_data
                else:
                    # Single variant: store directly
                    parts_index[part_name]["statuses"][status] = rule_data

        palette_mappings = list(parts_index.values())
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
