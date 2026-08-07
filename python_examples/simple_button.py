"""
texting
"""
from icedpygui import (
    Window,
    Container,
    Column,
    start_session,
    add_button,
    )

def on_press(wid: int):
    """Button"""
    print(wid)

with Window(title="Testing", center=True):
    with Container(align_center=True, fill=True):
        with Column(spacing=20):
            add_button(label="Button 1", on_press=on_press)
            add_button(label="Button 2", on_press=on_press)


start_session()
