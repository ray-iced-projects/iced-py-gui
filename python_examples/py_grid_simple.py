#!/usr/bin/env python3
"""
Grid use demo
"""

from icedpygui import (
    Window, Column, Container, ContainerStyleStd, Grid, GridParam, start_session,
    add_button, add_text, update_widget, delete_widget)

state = {
    "cont_ids": [],
    "txt_ids": [],
}


def change_grid_size(_btn_id: int):
    """Method to change grid size"""
    update_widget(grid_id, GridParam.ColumnsAmount, 3)
    indexes_to_remove = {3, 7}
    delete_widget(state["cont_ids"][3])
    delete_widget(state["cont_ids"][7])
    # just house keeping to make sure the ids remain accurate.
    state["cont_ids"] = [item for i,
                         item in enumerate(state["cont_ids"]) if i not in indexes_to_remove]
    state["txt_ids"] = [item for i,
                        item in enumerate(state["txt_ids"]) if i not in indexes_to_remove]


with Window(title="Demo"):
    with Container(fill=True, align_center=True):
        with Column(spacing=20):
            with Grid(columns_amount=4, width=400, spacing=3.0) as grid_id:
                # typical row/column iteration
                for row in range(2):
                    for col in range(4):
                        with Container(height=40, align_center=True,
                                    style_std=ContainerStyleStd.BorderedBox) as cnt_id:
                            state["cont_ids"].append(cnt_id)
                            txt_id = add_text(content=f"Grid {row} {col}")
                            state["txt_ids"].append(txt_id)

            add_button(label="Change Grid size", on_press=change_grid_size)
            add_text(content=("When the columns are changed, in this case to 3, \n"
                              "The user will need to add_or remove the content as needed."))

start_session()
