#!/usr/bin/env python3
"""
Popup use demo

Popup can be used in 2 ways.
1.  Use a widget to open it like a button, etc.
2.  Opened it using a callback to set the opened parameter to True
    or just set it to True to be opened when the program starts.
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
    """Called to open PopUp"""
    update_widget(popup_id, PopUpParam.Opened, True)

def pop_opened(pop_id: int):
    """Called when the popup is opened"""
    print("Popup opened", pop_id)

def on_pop_closed(pop_id: int):
    """Called when popup closed"""
    print("PopUp closed", pop_id)

def clicked_outside(pop_id: int):
    """Called when mouse clicked outside popup"""
    print("Clicked outside and closing", pop_id)
    update_widget(pop_id, PopUpParam.Opened, False)


# Add a window first
with Window(
    title="ColorPicker",
    size=(600.0, 600.0),
    center=True):

    # Add the container to center the widget.
    with Container(
        fill=True,
        align_center=True):

        # Add a column to hold multiple widgets
        with Column(spacing=20.0, width=200):
            # Add the popup
            with PopUp(on_open=pop_opened,
                       on_close=on_pop_closed,
                       on_click_outside=clicked_outside) as popup_id:
                add_button(label="Press Me", on_press=open_popup)
                with Container(style_std=ContainerStyleStd.BorderedBox):
                    add_text(content="I'm a PopUp Container, Press outside to close")

            # add the popup but set the opened to True to see it immediately
            with PopUp(opened=True):
                with Container(style_std=ContainerStyleStd.BorderedBox):
                    add_text(content="I'm an immediately opened PopUp")

start_session()
