#!/usr/bin/env python3
"""
helper file for automating the demo widgets
"""

from typing import TYPE_CHECKING
from icedpygui import (
    add_button,
    add_button_style,
    CheckboxParam,
    PickListParam,
    add_pick_list,
    get_color_palette,
    move_widget,
    update_widget,
    update_widget_params,
)
from python_examples.py_palette.widget_helpers import set_widget_status

if TYPE_CHECKING:
    from python_examples.py_palette.py_palette_create import PaletteCreator


def demo_populate_widget_checkboxes(pc: PaletteCreator):
    """Updating the widget and checkboxes for palette matrix selection"""
    color = pc.widget_config.selected_color
    pc.palette = get_color_palette(rgba=color)
    statuses = pc.widget_config.statuses
    parts = pc.widget_config.parts
    pal_by_name = {str(k).rsplit('.', maxsplit=1)[-1]: v for k, v in pc.palette.items()}

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
            pc.palette_widget_ids.append(widget_id)
            part_id = add_pick_list(parent_id=pc.row_ids[idx],
                                    options=parts,
                                    placeholder="Select a part",
                                    width=100)
            pc.palette_parts_ids.append(part_id)
            idx += 1

    # Update checkboxes in a 8x#statuses matrix (8 rows x statuses/columns)
    # Also set picklist initial selection to first part (Background)
    default_part = parts[0] if parts else None

    row_idx = 0
    for pal_name in pal_by_name.items():
        if "Text" not in pal_name[0]:  # Check the key part of the tuple
            palette_tier_name = pal_name[0]

            # Check if default_part has rules for this palette tier; if so, select it in picklist
            if default_part and row_idx < len(pc.palette_parts_ids):
                # Check if any rule exists for this part+palette_tier combination
                has_rules = any(
                    should_checkbox_be_checked(pc, status, palette_tier_name, default_part)
                    for status in statuses
                )
                if has_rules:
                    update_widget(
                        pc.palette_parts_ids[row_idx],
                        PickListParam.Selected,
                        default_part
                    )

            for col_idx, status in enumerate(statuses):
                # Check if this palette_tier+status+part combination has a rule
                is_checked = should_checkbox_be_checked(pc, status, palette_tier_name, default_part)

                update_widget_params(pc.checkbox_grid[row_idx][col_idx][0], {
                    CheckboxParam.Show: True,
                    CheckboxParam.Label: status,
                    CheckboxParam.IsChecked: is_checked,
                })
                # Update the grid with the correct checked state
                pc.checkbox_grid[row_idx][col_idx] = (pc.checkbox_grid[row_idx][col_idx][0], is_checked)

            move_widget(wid=pc.palette_widget_ids[row_idx], move_before=pc.checkbox_grid[row_idx][0][0])
            move_widget(wid=pc.palette_parts_ids[row_idx], move_before=pc.checkbox_grid[row_idx][0][0])
            row_idx += 1

def should_checkbox_be_checked(pc, status: str, palette_tier_name: str, part_name: str = None) -> bool:
    """Check if checkbox should be checked based on part rule matching the palette tier.

    A checkbox is checked if:
    - There exists a rule for (status, variant, part_name)
    - AND that rule's key (palette tier) matches the palette_tier_name
    """
    if not part_name:
        part_name = pc.widget_config.parts[0] if pc.widget_config.parts else None
    if not part_name:
        return False

    # Check all variants to find a matching rule
    for variant in pc.widget_config.state_variants:
        rule = pc.widget_config.get_rule(status, variant, part_name)
        if rule and rule.key == palette_tier_name:
            return True
    return False


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
