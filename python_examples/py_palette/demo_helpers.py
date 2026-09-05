#!/usr/bin/env python3
"""
helper file for automating the demo widgets
"""

from typing import TYPE_CHECKING
from icedpygui import (
    add_button,
    add_button_style,
    add_checkbox,
    CheckboxParam,
    get_color_palette,
    update_widget
)
from python_examples.py_palette.widget_helpers import set_widget_status

if TYPE_CHECKING:
    from python_examples.py_palette.py_palette_create import PaletteCreator


def demo_populate_widget_checkboxes(pc: PaletteCreator):
    """Updating the widget and checkboxes for palette matrix selection"""
    color = pc.widget_config.selected_color
    pc.palette = get_color_palette(rgba=color)
    statuses = pc.widget_config.statuses
    print(statuses)
    pal_by_name = {str(k).rsplit('.', maxsplit=1)[-1]: v for k, v in pc.palette.items()}

    # Initialize the checkbox grid (8 rows x 4 columns)
    pc.checkbox_grid = [[] for _ in range(8)]

    # add the palette color buttons (one per status/column)
    idx = 0
    for (k, pal_color) in pal_by_name.items():
        if "Text" not in k:
            text_color = pal_by_name.get(k + "Text")
            btn_style_id = add_button_style(bkg_rgba=pal_color, text_rgba=text_color)
            widget_id = add_button(
                            parent_id=pc.row_ids[idx],
                            label=k,
                            width=100,
                            padding=[10],
                            style_id=btn_style_id)
            pc.widget_ids.append(widget_id)
            idx += 1

    # Create checkboxes in a 8x4 matrix (8 rows x 4 statuses/columns)
    for _c, (idx, status) in enumerate(enumerate(statuses)):
        for r in range(8):
            cb_id = add_checkbox(
                        parent_id=pc.row_ids[r],
                        label=status,
                        on_toggle=demo_on_status_checked,
                        user_data=pc,
                        )
            pc.checkbox_grid[r].append((cb_id, False))

    demo_set_widget_statuses():



def demo_set_widget_statuses():
    """Set the status based on the loaded config file"""
    

def demo_on_status_checked(cb_id: int, is_checked: bool, pc: PaletteCreator):
    """
    Enforce matrix selection: only one checked per row AND per column.
    On check, clear the rest of the target's row and column, then set it True.
    """
    grid = pc.checkbox_grid  # list[list[tuple[int, bool]]]

    # Locate the target id in the grid
    target_row = target_col = None
    for r, row_ in enumerate(grid):
        for c, (wid, _state) in enumerate(row_):
            if wid == cb_id:
                target_row, target_col = r, c
                break
        if target_row is not None:
            break

    if target_row is None:
        return  # id not found

    # Clear every True in the target's row (except the target)
    for c, (wid, checked) in enumerate(grid[target_row]):
        if c != target_col and checked:
            update_widget(wid, CheckboxParam.IsChecked, False)
            grid[target_row][c] = (wid, False)

    # Clear every True in the target's column (except the target)
    for r, row_ in enumerate(grid):
        if r != target_row:
            wid, checked = row_[target_col]
            if checked:
                update_widget(wid, CheckboxParam.IsChecked, False)
                row_[target_col] = (wid, False)

    # Set the target to its incoming state
    update_widget(cb_id, CheckboxParam.IsChecked, is_checked)
    grid[target_row][target_col] = (cb_id, is_checked)
    set_widget_status(pc, target_row, target_col)
