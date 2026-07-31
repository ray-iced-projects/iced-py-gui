#!/usr/bin/env python3
"""
Popup use demo

Popup can be used in 2 ways.
1.  Use a widget to open it like a button, etc.
2.  Opened it using a callback to set the opened parameter to True
"""

from icedpygui import (
    Window,
    Column,
    Container,
    ContainerStyleStd,
    PopUp,
    PopUpParam,
    add_button,
    add_text,
    update_widget,
    start_session)


def open_popup(_btn_id: int):
    """Open PopUp method"""
    print(pop_id)
    update_widget(pop_id, PopUpParam.Opened, True)



# Add a window first
with Window(
    title="ColorPicker",
    size=(600.0, 600.0),
    center=True):

    # Add the container.
    with Container(
        fill=True,
        align_center=True):

        # Add a column to hold multiple widgets
        with Column(spacing=20.0, width=200):
            with PopUp() as pop_id:
                add_button(label="Press Me", on_press=open_popup)
                with Container(style_std=ContainerStyleStd.BorderedBox):
                    add_text(content="I'm a PopUp")

            with PopUp(opened=True):
                with Container(style_std=ContainerStyleStd.BorderedBox):
                    add_text(content="I'm a PopUp")

start_session()
