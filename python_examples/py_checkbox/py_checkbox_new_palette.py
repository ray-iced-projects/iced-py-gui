#!/usr/bin/env python3
"""
Checkbox palette demo — shows the colour palette for checkbox themes.

Checkbox statuses: Active {is_checked}, Hovered {is_checked}, Disabled {is_checked}
Default palette mapping:
  Base      → Active   (unchecked background)
  Weak      → Hovered  (unchecked hovered background)
  Weaker    → Disabled (unchecked disabled background)
  BaseAlpha → IsChecked (checked accent colour, alpha-dimmed for disabled)
"""

import os
from icedpygui import (
    Window,
    WindowTheme,
    Column,
    Scrollable,
    Row,
    start_session,
    add_checkbox,
    add_text,
    add_font_style,
    FontWeight,
    custom_palette,
    get_color_palette,
    PaletteKey,
    WidgetStatus,
    StylePart,
    StateVariant,
    add_text_editor,
    add_text_editor_style,
)


# ---------------------------------------------------------------------------
# Create all of the ids and additional needed items
# ---------------------------------------------------------------------------

# See exaplanation in the py_checkbox_text.txt or launch and read.
# Note that the Icon is not used in the unchecked.
pal = [
    ((WidgetStatus.Active, StateVariant.Unchecked), (
                            (StylePart.Border,      PaletteKey.Strong,   1.0),
                            (StylePart.Background,  PaletteKey.Base,     1.0),
                            (StylePart.Text,        PaletteKey.BaseText, 0.8),
                            ),
    ),
    ((WidgetStatus.Hovered, StateVariant.Unchecked), (
                            (StylePart.Border,      PaletteKey.Strong,   1.0),
                            (StylePart.Background,  PaletteKey.Weak,     1.0),
                            (StylePart.Text,        PaletteKey.BaseText, 0.8),
                            ),
    ),
    ((WidgetStatus.Disabled, StateVariant.Unchecked), (
                            (StylePart.Border,      PaletteKey.Weak,     1.0),
                            (StylePart.Background,  PaletteKey.Weaker,   1.0),
                            (StylePart.Text,        PaletteKey.BaseText, 0.8),
                            ),
    ),
    ((WidgetStatus.Active, StateVariant.Checked), (
                            (StylePart.Border,      PaletteKey.Base,     1.0),
                            (StylePart.Background,  PaletteKey.Base,     1.0),
                            (StylePart.Icon,        PaletteKey.BaseText, 0.8),
                            (StylePart.Text,        PaletteKey.BaseText, 1.0),
                            ),
    ),
    ((WidgetStatus.Hovered, StateVariant.Checked), (
                            (StylePart.Border,      PaletteKey.Strong,   1.0),
                            (StylePart.Background,  PaletteKey.Strong,   1.0),
                            (StylePart.Icon,        PaletteKey.BaseText, 0.8),
                            (StylePart.Text,        PaletteKey.BaseText, 1.0),
                            ),
    ),
    ((WidgetStatus.Disabled, StateVariant.Checked), (
                            (StylePart.Border,      PaletteKey.Strong,   1.0),
                            (StylePart.Background,  PaletteKey.Weak,     1.0),
                            (StylePart.Icon,        PaletteKey.BaseText, 0.8),
                            (StylePart.Text,        PaletteKey.BaseText, 1.0),
                            ),
    ),
]

# new_color = [0.1647, 0.7647, 0.8706, 1.0] # Primary for checking
new_color = [0.32, 0.2, 0.13, 1.0]
pal_id = custom_palette(rgba=new_color, statuses=pal)

# This demo will need the colors for the containers, normally
# only the above custom_palette() would be used.
color_pal = get_color_palette(rgba=new_color)

font_id = add_font_style(family_name="Roboto", weight=FontWeight.Bold)


cwd = os.getcwd()
FILE_PATH = os.path.join(cwd, "python_examples", "py_checkbox", "py_checkbox_text.txt")
state = {"file": ""}

try:
    with open(FILE_PATH, "r", encoding='utf-8') as file:
        state["file"] = file.read()
except FileNotFoundError:
    print(f"*********The file does not exist using {FILE_PATH}.*******")

# ---------------------------------------------------------------------------
# GUI — Initial display with a TokyoNight background (Selected by PickList)
# ---------------------------------------------------------------------------
with Window(title="Button Custom Palette",
            size=(1100, 850), center=True, theme=WindowTheme.TokyoNight) as wnd_id:
    with Scrollable(height=800):
        with Column(spacing=10, padding=[20], width_fill=True):
            with Scrollable(height=300):
                txt_ed_style_id = add_text_editor_style(border_width=0)
                add_text_editor(content=state["file"],
                                width_fill=True,
                                style_id=txt_ed_style_id)

            with Column(spacing=10, width_fill=True):
                add_text(content=(
                    ""))

                add_text(content="The Checkbox statuses: Active, Hovered, and Disabled")

                with Column(spacing=20, width_fill=True, height_fill=True):
                    add_text(content="******Custom Status Styling******")

                    with Row(spacing=20):
                        add_checkbox(label="Status: Unchecked",
                                    palette_id=pal_id, disabled=True)
                        add_checkbox(label="Status: Checked", is_checked=True,
                                        palette_id=pal_id, disabled=True)
                        add_checkbox(label="Status: Disabled",
                                    disabled=True)
start_session()
