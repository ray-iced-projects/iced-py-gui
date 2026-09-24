#!/usr/bin/env python3
"""
helper file for automating the demo widgets
"""

import os
from typing import TYPE_CHECKING
from icedpygui import (
    ButtonParam,
    ContainerStyleParam,
    InputFloatParam,
    TextParam,
    get_color_palette,
    get_widget_default_statuses,
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
    """Updating the widgets and for palette table selection"""
    # get/generating the data
    color = pc.selected_color
    pc.palette = get_color_palette(rgba=color)
    statuses = [str(status).rsplit('.', maxsplit=1)[-1] for (status, variant), parts in pc.statuses]
    pals_by_name = {str(k).rsplit('.', maxsplit=1)[-1]: v for k, v in pc.palette.items()}
    pals_list = list(pals_by_name.items())
    num_statuses = len(statuses)

    # get a default statuses for a button (any will do)
    default_statuses = get_widget_default_statuses(pc.widget_normal_style_id)

    # Build a lookup dict
    status_parts = {}
    for (status, variant), parts in default_statuses:
        status_key = str(status).rsplit('.', maxsplit=1)[-1].lower() # "active"
        status_parts[status_key] = {}
        for part, key, alpha in parts:
            part_key = str(part).rsplit('.', maxsplit=1)[-1].lower()  # "background"
            status_parts[status_key][part_key] = (key, alpha)

    # Populate the table by row
    # i.e. Background-Active, popup button, Opacity, Border width
    for (parts_idx, part) in enumerate(pc.widget_parts):
        for (status_idx, status) in enumerate(statuses):
            row_index = parts_idx * num_statuses + status_idx
            # get the id of the part
            part_id = pc.parts_list_ids[row_index]
            # update the text widget
            update_widget_params(part_id,
                          {TextParam.Content: f"{part}-{status}",
                           TextParam.Show: True})
            # add the user data for later use
            update_user_data(part_id, (part, status))

            # Filter palette based on whether part contains "Text"
            if "Text" in part:
                # Use only Text-related palettes for Text parts
                filtered_pals = [(name, rgba) for name, rgba in pals_list if "Text" in name]
            else:
                # Use non-Text palettes for non-Text parts
                filtered_pals = [(name, rgba) for name, rgba in pals_list if "Text" not in name]

            # iterate through the 8 palette containers updating
            # with the palette name and setting the bkg_rgba value
            for pal_index in range(8):
                (name, rgba) = filtered_pals[pal_index]
                # update the text widget inside the container
                update_widget(pc.palette_popup_cnt_text_ids[row_index][pal_index],
                                TextParam.Content, name)
                # update the container background
                update_widget(pc.palette_popup_cnt_style_ids[row_index][pal_index],
                                            ContainerStyleParam.BkgRgba, rgba)
                # update the user data for the mouse area
                update_user_data(pc.palette_popup_ma_ids[row_index][pal_index],
                                    (row_index, pal_index))

            # Get the ids of the popup and the button that opens the popup
            # The button needs to reflect the palette name using the demo color
            (_, btn_open_id) = pc.popup_open_btn_ids[row_index]
            key, alpha = status_parts[status.lower()][part.lower()]
            key_str = str(key).rsplit('.', maxsplit=1)[-1]
            update_widget_params(btn_open_id, {
                ButtonParam.Label: key_str,
                ButtonParam.Show: True})

            # Show the opacity and border widgets
            update_widget(pc.opacity_ids[row_index], InputFloatParam.Show, True)
            update_widget(pc.border_ids[row_index], InputFloatParam.Show, True)
