"""
Testing doc examples
"""
from icedpygui import (
    Window,
    Arrow,
    Container,
    add_button,
    start_session)

def on_press(btn_id: int):
    """test"""
    print(btn_id)

with Window(title="My App", center=True):
    with Container(align_center=True, fill=True):
        add_button(
            label="Press Me",
            on_press=on_press,
            style_arrow=Arrow.ArrowUp,
        )

start_session()
