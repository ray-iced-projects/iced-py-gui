"""
Test menu click to trigger button error
"""
from icedpygui import (
    Window,
    Column,
    Container,
    start_session,
    Menu,
    MenuBarItem,
    add_button,
    add_checkbox,
    add_text,
)


# Add a window
with Window(title="Menu Test - Click File", center=True, size=[600, 600]):

    with Container(padding=[20.0], fill=True):
        with Column(spacing=20):
            with Menu(spacing=20.0):

                # First item of the MenuBarItem is the bar item followed by the dropdown items
                with MenuBarItem(width=125, spacing=5.0, offset=3.0):

                    add_text(content="File") # bar item - CLICK THIS
                    # dropdown items
                    add_text(content="Test1")
                    add_text(content="Test2")
                    add_button(label="New")
                    add_checkbox(label="Test3")

start_session()
