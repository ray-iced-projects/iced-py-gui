#!/usr/bin/env python3
"""
Method the create the palette for a widgets.
"""

from icedpygui import (
    Window,
    ColorPicker,
    Column,
    Container,
    Row,
    start_session,
    add_button,
    add_text,
    add_text_input,
    )


state = {
    "path": "",
    "color": [],
    }

def define_color_via_ti(_ti_id: int, value: str):
    """Color input in format [#, #, #, #]"""


def define_color_via_cp(_cp_id: int, color: list):
    "Color input from ColorPicker"
    state["color"] = color



with Window(title="Palette Creator", center=True):
    with Container(width_fill=True, padding=[20]):
        with Column(spacing=10):
            add_text(content=(
                "To define a palette use the following steps.\n"
                "1. Enter the path to the file"
                "2. Create a new color list using the ColorPicker or text_input\n"
                "3. Create the pallettes\n"
                "4. Create a pallette id\n"
                "5. Add the palette id created to the widget\n"))

            with Row(spacing=5):
                add_text(content="Step 1: ")
                add_text_input(placeholder="Enter the path to file", width=300)

            with Row(spacing=5):
                add_text(content="Step 2: Submit color using ColorPicker or input with TextInput ")
                with ColorPicker(on_submit=define_color_via_cp):
                    add_button(label="Color Picker")
                add_text_input(placeholder="Input color list format [#, #, #, #]",
                               on_submit=define_color_via_ti)

start_session()
