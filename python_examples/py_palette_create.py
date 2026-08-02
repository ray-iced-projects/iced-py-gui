#!/usr/bin/env python3
"""
Method the create the palette for a widgets.
"""

import os
from pathlib import Path
from icedpygui import (
    Window,
    ColorPicker,
    Column,
    Container,
    ContainerStyleStd,
    PopUp,
    Row,
    Scrollable,
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
    "current_dir": str(Path.home()),  # Start at home directory
    "tree_items": [],  # Track tree item IDs for updates
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


def on_folder_click(_btn_id: int):
    """Placeholder for folder navigation"""


def on_file_click(_file_btn_id: int, file_path: str):
    """Select file and update path"""
    state["path"] = file_path
    update_widget(path_text_id, TextParam.Content, f"✓ Selected: {file_path}")


def on_parent_dir_click(_btn_id: int):
    """Go to parent directory"""
    parent = Path(state["current_dir"]).parent
    if parent != Path(state["current_dir"]):  # Not at root
        state["current_dir"] = str(parent)
        build_tree_view()


def build_tree_view():
    """Build directory listing as text display"""
    try:
        current_path = Path(state["current_dir"])

        # Build directory listing text
        lines = [f"📁 Current: {current_path}\n"]

        # Add parent directory option
        if current_path.parent != current_path:
            lines.append("  📁 .. (parent)\n")

        # Get sorted directory contents
        try:
            items = sorted(current_path.iterdir(), key=lambda x: (not x.is_dir(), x.name))
        except PermissionError:
            lines.append("  ❌ Cannot access this directory\n")
            update_widget(path_text_id, TextParam.Content, "".join(lines))
            return

        # Add folders first, then files
        for item in items:
            try:
                if item.is_dir():
                    lines.append(f"  📁 {item.name}/\n")
                elif item.is_file():
                    lines.append(f"  📄 {item.name}\n")
            except (PermissionError, OSError):
                # Skip inaccessible items
                continue

        # Update display
        update_widget(path_text_id, TextParam.Content, "".join(lines))

    except Exception as e:
        update_widget(path_text_id, TextParam.Content, f"Error: {str(e)}")



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

                with PopUp(opened=True, position_right=True, snap_within_viewport=True) as popup_scan_id:
                    with Container(style_std=ContainerStyleStd.BorderedBox, width_fill=True):
                        with Scrollable():
                            with Column(spacing=5, padding=[20]) as scan_path_col_id:
                                add_button(label="⬆️ Parent Directory", on_press=on_parent_dir_click)
                                path_text_id = add_text(content="Loading directory...")

            with Row(spacing=5):
                add_text(content="Step 2: Submit color using ColorPicker or input with TextInput ")
                with ColorPicker(on_submit=define_color_via_cp):
                    add_button(label="Color Picker")
                add_text_input(placeholder="Input color list format [#, #, #, #]",
                               on_submit=define_color_via_ti)

# Initialize the tree view with home directory
build_tree_view()

start_session()
