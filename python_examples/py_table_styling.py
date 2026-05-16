#!/usr/bin/env python3
"""
Table styling demo
"""

from icedpygui import (
    Window,
    Column,
    Container,
    start_session,
    ColorPicker,
    add_button,
    ButtonParam,
    add_table,
    TableStyleParam,
    add_table_style,
    add_radio,
    add_text,
    update_widget,
)

state = {"radio_selected": None,
         "name": None}

def set_cp_label_table(_rd_id: int, index: int):
    """Set color picker button label"""

    state["radio_selected"] = index
    state["name"] = "table"
    match index:
        case 0:
            update_widget(
                wid=cp_btn_id,
                param=ButtonParam.Label,
                value="Set Table Bkg")
        case 1:
            update_widget(
                wid=cp_btn_id,
                param=ButtonParam.Label,
                value="Set Table Border Color")

        case 2:
            update_widget(
                wid=cp_btn_id,
                param=ButtonParam.Label,
                value="Set Text Color")


def set_cp_label_sash(_rd_id: int, index: int):
    """Set color picker button label"""
    state["radio_selected"] = index
    state["name"] = "sash"
    match index:
        case 0:
            update_widget(
                wid=cp_btn_id,
                param=ButtonParam.Label,
                value="Set Sash Bkg")

        case 1:
            update_widget(
                wid=cp_btn_id,
                param=ButtonParam.Label,
                value="Set Sash Hover Color")

def set_color(_cp_id: int, color: list):
    """Set Color"""
    match state["name"]:
        case "table":
            set_table_color(color)
        case "sash":
            set_sash_color(color)


def set_table_color(color: list):
    """Set Color"""
    print(color, state["radio_selected"])
    match state["radio_selected"]:
        case 0:
            update_widget(
                wid=tbl_style_id,
                param=TableStyleParam.BkgRgba,
                value=color)

        case 1:
            update_widget(
                wid=tbl_style_id,
                param=TableStyleParam.BorderRgba,
                value=color)

            update_widget(
                wid=tbl_style_id,
                param=TableStyleParam.BorderWidth,
                value=2.0)

        case 2:
            update_widget(
                wid=tbl_style_id,
                param=TableStyleParam.TextRgba,
                value=2.0)


def set_sash_color(color: list):
    """Set sash color"""
    match state["radio_selected"]:
        case 0:
            update_widget(
                wid=tbl_style_id,
                param=TableStyleParam.SashBkgRgba,
                value=color)

        case 1:
            update_widget(
                wid=tbl_style_id,
                param=TableStyleParam.SashHoverRgba,
                value=color)


column_widths = [100.0] * 4
width = sum(column_widths)

headers = ["str", "one", "two", "three"]
body = [
    [0.0, 1.0, 2.0, 3.0],
    [0.0, 2.0, 4.0, 6.0],
    [0.0, 3.0, 6.0, 9.0],
    [0.0, 4.0, 8.0, 12.0],
    [0.0, 5.0, 10.0, 15.0],
    [0.0, 6.0, 12.0, 18.0],
    [0.0, 7.0, 14.0, 21.0],
    [0.0, 8.0, 16.0, 24.0],
    [0.0, 9.0, 18.0, 27.0],
    [0.0, 10.0, 20.0, 30.0],
    [0.0, 11.0, 22.0, 33.0],
]
footers = ["", "", "", ""]


# adding the default style by suppling no arguments
tbl_style_id = add_table_style()


# Add the window
with Window(
        title="Table Styling Demo",
        size=(700, 600),
        center=True):

    # Add the container for centering the table
    with Container(fill=True, align_center=True,):

        with Column(spacing=10):

            # Needed in callback
            radio_ids = []
            radio_ids.append(add_radio(
                labels=["Set Table Bkg", "Set Table border Color", "Set Table Text Color"],
                width=250,
                on_selected=set_cp_label_table))

            radio_ids.append(add_radio(
                labels=["Set Sash Color", "Set Sash Hover Color"],
                on_selected=set_cp_label_sash))


            with ColorPicker(on_submit=set_color):

                cp_btn_id = add_button(label="ColorPicker: No Selection Made")

            # The table is added.
            table_id = add_table(
                    headers=headers,
                    body=body,
                    footers=footers,
                    column_widths=column_widths,
                    height=150.0,
                    custom_footer_rows=1,
                    style_id=tbl_style_id
                    )

            footer = ["This", "is", "a", "footer"]
            footers = []
            for (i, _) in enumerate(range(0, len(footer))):
                footers.append(add_text(
                        parent_id=table_id,
                        content=footer[i],
                        size=14.0))

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
