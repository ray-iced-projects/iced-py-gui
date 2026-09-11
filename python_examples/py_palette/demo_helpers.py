#!/usr/bin/env python3
"""
helper file for automating the demo widgets
"""

import os
from typing import TYPE_CHECKING
from icedpygui import (
    TextParam,
    get_color_palette,
    update_widget,
)


if TYPE_CHECKING:
    from python_examples.py_palette.py_palette_create import PaletteCreator


def load_demo_parts_file(pc: PaletteCreator):
    """Method to load the demo"""
    cwd = os.getcwd()
    file_path = os.path.join(cwd, "python_examples", "py_palette", "widget_palette_parts.yml")
    try:
        with open(file_path, "r", encoding='utf-8') as file:
            pc.parts_file = file.read()
    except FileNotFoundError:
        print(f"*********The file does not exist using {file_path}.*******")

    # pc.widget_list = get_widget_palette_list(pc.parts_file)


def demo_populate_palette_area(pc: PaletteCreator):
    """Updating the widget and checkboxes for palette matrix selection"""
    color = pc.widget_config.selected_color
    pc.palette = get_color_palette(rgba=color)
    statuses = pc.widget_config.statuses
    _parts = pc.widget_config.parts
    _pals_by_name = {str(k).rsplit('.', maxsplit=1)[-1]: v for k, v in pc.palette.items()}

    # show the menu items
    print(pc.palette_menu_item_btn_ids)
    # Populate the palette area
    for _ in range(len(statuses)):
        for pt_ids in pc.parts_list_ids:
            for (s_idx, _status) in enumerate(statuses):
                update_widget(pt_ids[s_idx], TextParam.Show, True)
                for _btn_id in pc.palette_menu_item_btn_ids:
                    for _pal in pc.palette:
                        pass
