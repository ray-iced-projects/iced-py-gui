#!/usr/bin/env python3
"""
Method the create the palette for a widgets.
"""

import os
from icedpygui import (
    Window,
    ColorPicker,
    Column,
    Container,
    ContainerStyleStd,
    PopUp,
    Row,
    start_session,
    add_button,
    add_text,
    TextParam,
    add_text_input,
    update_widget,
    )


state = {
    "path": "",
    "color": [],
    }

def path_submit_check(_it_id: int, path: str):
    """Path Check - handles both relative and absolute paths"""
    if not path:
        update_widget(path_text_id, TextParam.Content, "Please enter a path.")
        return

    # Check if relative path and convert to absolute if needed
    if os.path.isabs(path):
        check_path = path
    else:
        # Convert relative path to absolute (relative to current working directory)
        check_path = os.path.abspath(path)

    if os.path.exists(check_path):
        update_widget(path_text_id, TextParam.Content, f"Path is valid: {check_path}")
        state["path"] = check_path
    else:
        update_widget(path_text_id, TextParam.Content, f"Path not found: {check_path}")


def define_color_via_ti(_ti_id: int, __value: str):
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
                # add_text_input(placeholder="Enter the path to file",
                #                width=300,
                #                on_submit=path_submit_check)
                # with PopUp():
                #     with Container(style_std=ContainerStyleStd.BorderedBox):
                #         path_text_id = add_text(content="The path you enter is not found.")
                
                with PopUp() as popup_scan_id:
                    with Container(style_std=ContainerStyleStd.BorderedBox):
                        with Column() as scan_path_col_id:
                            add_text(content="")

            with Row(spacing=5):
                add_text(content="Step 2: Submit color using ColorPicker or input with TextInput ")
                with ColorPicker(on_submit=define_color_via_cp):
                    add_button(label="Color Picker")
                add_text_input(placeholder="Input color list format [#, #, #, #]",
                               on_submit=define_color_via_ti)

start_session()
