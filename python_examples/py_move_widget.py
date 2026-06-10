#!/usr/bin/env python3
"""
Widget move use demo
"""
from icedpygui import (Window, Column, Row, start_session,
        add_button, add_text, add_space, move_widget)

ids = []


def move(_btn_id: int, item: any):
    """Move Widget Callback"""
    # equate the tuple items to help interpretation
    widget_id = item[0]
    move_after = item[2]
    move_before = item[3]

    # move the widget
    move_widget(
            wid=widget_id,
            move_after=move_after,
            move_before=move_before,
            target_parent_id=target_id,)


with Window(title="Move Widget",
            size=(400.0, 600.0),
            center=True) as wnd_id:

    with Row(spacing=50, padding=[20]):

        with Column(spacing=5) as target_id:
            for i in range(0, 10):
                ids.append(add_text(content=f"{i}"))

        with Column(spacing=20):
            add_space(height=20)
            add_button(
                    label="Move number 5 to end",
                    on_press=move,
                    user_data=(ids[5], "col_1", None, None))

            add_button(
                    label="Move number 5 after 0",
                    on_press=move,
                    user_data=(ids[5], "col_1", ids[0], None))

            add_button(
                    label="Move number 5 before 0",
                    on_press=move,
                    user_data=(ids[5], "col_1", None, ids[0]))

start_session()
