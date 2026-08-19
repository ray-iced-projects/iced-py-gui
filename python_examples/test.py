"""
Testing doc examples
"""
from icedpygui import (
    Window,
    Column,
    Container,
    add_button,
    start_session)

def btn_cb(btn_id: int):
    print(btn_id)

def cb_with_data(btn_id: int, data: any):
    print(btn_id, data)

with Window(title="My App", center=True):
    with Container(align_center=True, fill=True):
        with Column(spacing=20):
                add_button(label="Press Me", on_press=btn_cb)
                add_button(
                    label="Press Me",
                    on_press=cb_with_data,
                    user_data="Some Data"
                )

start_session()
