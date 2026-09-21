#!/usr/bin/env python3
"""
helper file for automating the demo widgets
"""

import os
from typing import TYPE_CHECKING
from icedpygui import (
    ContainerStyleParam,
    TextParam,
    get_color_palette,
    update_widget,
    update_widget_params,
    update_user_data,
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
    # get/generating the data
    color = pc.selected_color
    pc.palette = get_color_palette(rgba=color)
    statuses = [str(status).rsplit('.', maxsplit=1)[-1] for (status, variant), parts in pc.statuses]
    pals_by_name = {str(k).rsplit('.', maxsplit=1)[-1]: v for k, v in pc.palette.items()}
    pals_list = list(pals_by_name.items())
    num_statuses = len(statuses)

    # Populate the columns left to right
    # i.e. Background-Active, popup button, Opacity, Border width
    for (parts_idx, part) in enumerate(pc.widget_parts):
        for (status_idx, status) in enumerate(statuses):
            part_id = pc.parts_list_ids[parts_idx * num_statuses + status_idx]
            update_widget_params(part_id,
                          {TextParam.Content: f"{part}-{status}",
                           TextParam.Show: True})
            update_user_data(part_id, (part, status))

            # Filter palette based on whether part contains "Text"
            if "Text" in part:
                # Use only Text-related palettes for Text parts
                filtered_pals = [(name, rgba) for name, rgba in pals_list if "Text" in name]
            else:
                # Use non-Text palettes for non-Text parts
                filtered_pals = [(name, rgba) for name, rgba in pals_list if "Text" not in name]

            # update the popup containers and mouse areas with the palette name and bkg color
            # The add the new user_data so that when the button is pressed,
            # the user data can be used to update the new widget

            # Get the ids
            (popup_id, open_id) = pc.popup_open_btn_ids[parts_idx * num_statuses + status_idx]
            

            # iterate through the 8 palette containers updating
            # with the palette name
            row = parts_idx * num_statuses + status_idx
            for pal_index in range(8):
                (name, rgba) = filtered_pals[pal_index]
                update_widget(pc.palette_popup_cnt_text_ids[row][pal_index],
                                TextParam.Content, name)
                update_widget(pc.palette_popup_cnt_style_ids[row][pal_index],
                                            ContainerStyleParam.BkgRgba, rgba)
                update_user_data(pc.palette_popup_ma_ids[row][pal_index],
                                    (row, pal_index))
