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
    Color,
    Scrollable,
    Row,
    start_session,
    add_checkbox,
    add_checkbox_style,
    add_text,
    custom_palette,
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
                            (StylePart.Background,  PaletteKey.Base,     1.0),
                            (StylePart.Text,        PaletteKey.BaseText, 0.8),
                            (StylePart.Border,      PaletteKey.Strong,   1.0),
                            ),
    ),
    ((WidgetStatus.Hovered, StateVariant.Unchecked), (
                            (StylePart.Background,  PaletteKey.Weak,     1.0),
                            (StylePart.Text,        PaletteKey.BaseText, 0.8),
                            (StylePart.Border,      PaletteKey.Strong,   1.0),
                            ),
    ),
    ((WidgetStatus.Disabled, StateVariant.Unchecked), (
                            (StylePart.Background,  PaletteKey.Weaker,   1.0),
                            (StylePart.Text,        PaletteKey.BaseText, 0.8),
                            (StylePart.Border,      PaletteKey.Strong,   1.0),
                            ),
    ),
    ((WidgetStatus.Active, StateVariant.Checked), (
                            (StylePart.Background,  PaletteKey.Base,     1.0),
                            (StylePart.Icon,        PaletteKey.BaseText, 1.0),
                            (StylePart.Text,        PaletteKey.BaseText, 0.8),
                            (StylePart.Border,      PaletteKey.Strong,   1.0),
                            ),
    ),
    ((WidgetStatus.Hovered, StateVariant.Checked), (
                            (StylePart.Background,  PaletteKey.Strong,   1.0),
                            (StylePart.Icon,        PaletteKey.BaseText, 1.0),
                            (StylePart.Text,        PaletteKey.BaseText, 0.8),
                            (StylePart.Border,      PaletteKey.Strong,   1.0),
                            ),
    ),
    ((WidgetStatus.Disabled, StateVariant.Checked), (
                            (StylePart.Background,  PaletteKey.Weak,     1.0),
                            (StylePart.Icon,        PaletteKey.BaseText, 1.0),
                            (StylePart.Text,        PaletteKey.BaseText, 0.8),
                            (StylePart.Border,      PaletteKey.Strong,   1.0),
                            ),
    ),
]

# Select the color you want, use ColorPicker
new_color = [0.32, 0.2, 0.13, 1.0]

# Get the custom palette
pal_id = custom_palette(rgba=new_color, statuses=pal)

# If you want a border, use the style to add the width
style_id = add_checkbox_style(border_width=2.0)

# Get the instructions
cwd = os.getcwd()
FILE_PATH = os.path.join(cwd, "python_examples", "py_checkbox", "py_checkbox_text.txt")
state = {"file": ""}

try:
    with open(FILE_PATH, "r", encoding='utf-8') as file:
        state["file"] = file.read()
except FileNotFoundError:
    print(f"*********The file does not exist using {FILE_PATH}.*******")

# ---------------------------------------------------------------------------
# GUI — Initial display with a TokyoNight
# ---------------------------------------------------------------------------
with Window(title="Checkbox Custom Palette",
            size=(1100, 850), center=True, theme=WindowTheme.TokyoNight) as wnd_id:
    with Scrollable(height=800):
        with Column(spacing=10, padding=[20], width_fill=True):
            with Scrollable(height=500):
                # Add the text editor with the instructions
                txt_ed_style_id = add_text_editor_style(border_width=0)
                add_text_editor(content=state["file"],
                                width_fill=True,
                                style_id=txt_ed_style_id)

            with Column(spacing=10, width_fill=True, padding=[20]):

                add_text(content="The Checkbox statuses: Active, Hovered, and Disabled")

                with Column(spacing=20, width_fill=True, height_fill=True):
                    add_text(content="******Custom Status Styling******")

                    # add the row of checkboxes with the fixed status
                    # then add a normal checkbox to see how it actually works
                    with Row(spacing=20):
                        add_checkbox(label="Status: Unchecked",
                                    palette_id=pal_id, active=True,
                                    lock_is_checked=False,
                                    style_id=style_id)
                        add_checkbox(label="Status: Checked",
                                    palette_id=pal_id, active=True,
                                    lock_is_checked=True,
                                    style_id=style_id)
                        add_checkbox(label="Status: Disabled", disabled=True,
                                     lock_is_checked=False, style_id=style_id)

                    with Row(spacing=20):
                        # Custom checkbox without the status locked
                        add_checkbox(label="Normal Custom Checkbox",
                                     palette_id=pal_id,
                                     style_id=style_id)

                        # Standard default checkbox
                        add_checkbox(label="Normal Default Checkbox")

                        # adding some border styling
                        bd_style_id = add_checkbox_style(border_width=2.0,
                                                         border_color=Color.WHITE,
                                                         border_rounded=5)
                        add_checkbox(label="Normal Default Checkbox with border fixed",
                                     style_id=bd_style_id)

start_session()
