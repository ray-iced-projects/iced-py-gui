#!/usr/bin/env python3
"""
helper file for automating the demo widgets
"""

import os
from typing import TYPE_CHECKING
from icedpygui import (
    ButtonParam,
    ButtonStyleParam,
    MenuBarItemParam,
    TextParam,
    get_color_palette,
    update_widget,
    update_widget_params,
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


def demo_populate_palette_area(pc: PaletteCreator):
    """Updating the widget and checkboxes for palette matrix selection"""
    color = pc.widget_config.selected_color
    pc.palette = get_color_palette(rgba=color)
    statuses = pc.widget_config.statuses
    parts = pc.widget_config.parts
    pals_by_name = {str(k).rsplit('.', maxsplit=1)[-1]: v for k, v in pc.palette.items()}

    # Populate the text widget with parts_list coupled with status
    for (parts_index, part) in enumerate(parts):
        for (status_index, status) in enumerate(statuses):
            update_widget_params(pc.parts_list_ids[parts_index][status_index],
                          {TextParam.Content: f"{part}-{status}",
                           TextParam.Show: True})
            menu_bar_index = parts_index * len(statuses) + status_index
            update_widget(pc.menu_bar_ids[menu_bar_index], MenuBarItemParam.Show, True)

    # show the menu bars and update the palette bar items with the correct button color
    for (idx, _bar_id) in enumerate(pc.menu_bar_ids):
        # Calculate which part and status this menu bar corresponds to
        part_idx = idx // len(statuses)
        status_idx = idx % len(statuses)
        
        # Check if we're within the bounds of actual parts and statuses
        if part_idx >= len(parts) or status_idx >= len(statuses):
            continue

        part = parts[part_idx]
        pals_list = list(pals_by_name.items())

        # Filter palette based on whether part contains "Text"
        if "Text" in part:
            # Use only Text-related palettes for Text parts
            filtered_pals = [(name, rgba) for name, rgba in pals_list if "Text" in name]
        else:
            # Use non-Text palettes for non-Text parts
            filtered_pals = [(name, rgba) for name, rgba in pals_list if "Text" not in name]
        print(part)
        print(statuses[status_idx])
        # update the menu buttons with the palette name and bkg color
        for index in range(8):
            if index < len(filtered_pals):
                (name, rgba) = filtered_pals[index]
                update_widget_params(pc.palette_menu_item_btn_ids[idx][index], {
                                    ButtonParam.Label, name,
                                    ButtonParam.})
                
                update_widget(pc.palette_menu_item_btn_style_ids[idx][index],
                            ButtonStyleParam.BkgRgba, rgba)
