"""
texting
"""
from icedpygui import (
    Window,
    Container,
    start_session,
    add_button,
    )



with Window(title="Testing"):
    with Container():
        add_button(
            label="New")

start_session()
