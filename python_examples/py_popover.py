#!/usr/bin/env python3
"""
PopOver use demo

PopOver can be used in 2 ways.
1.  Use a widget to open it like a button, etc.
2.  Opened it using a callback to set the opened parameter to True
    or just set it to True to be opened when the program starts.
"""

from icedpygui import (
    Window,
    Column,
    Container,
    ContainerStyleStd,
    PopOver,
    PopOverParam,
    add_button,
    add_text,
    update_widget,
    start_session)


def open_popover(_btn_id: int):
    """Called to open PopUp"""
    update_widget(popover_id, PopOverParam.Opened, True)

def pop_opened(pop_id: int):
    """Called when the popup is opened"""
    print("Popup opened", pop_id)

def on_pop_closed(pop_id: int):
    """Called when popup closed"""
    print("PopUp closed", pop_id)

def clicked_outside(pop_id: int):
    """Called when mouse clicked outside popup"""
    print("Clicked outside and closing", pop_id)
    update_widget(pop_id, PopOverParam.Opened, False)


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
            with PopOver(
                position_top=True,
                on_open=pop_opened,
                       on_close=on_pop_closed,
                       on_click_outside=clicked_outside) as popover_id:
                add_button(label="Press Me", on_press=open_popover)
                with Container(style_std=ContainerStyleStd.BorderedBox):
                    add_text(content="I'm a PopOver Container, Press outside to close")

            # add the popup but set the opened to True to see it immediately
            with PopOver(opened=True):
                with Container(style_std=ContainerStyleStd.BorderedBox):
                    add_text(content="I'm an immediately opened PopOver")

start_session()
