"""
texting
"""
from icedpygui import (
    Window,
    Container,
    start_session,
    add_button,
    )

def on_press(wid: int):
    """Button"""
    print(wid)


with Window(title="Testing", center=True):
    with Container(align_center=True, fill=True):
        add_button(label="Button 1", padding=[10], on_press=on_press)


start_session()
