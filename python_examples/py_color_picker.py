#!/usr/bin/env python3
"""
Color Picker use demo

"""

from icedpygui import (
    Window,
    Column,
    Container,
    ColorPicker,
    add_text,
    TextParam,
    update_widget,
    add_button,
    add_button_style,
    start_session)


btn_style = add_button_style(border_radius=[8.0])

def color_selected(_cp_id: int, color: str):
    """
    Color selected callback
    """
    # update the text
    update_widget(text_id, TextParam.Content, color)



def cp_opened(_cp_id: int, opened: bool):
    """Color Picker Callback"""
    print(f"color picker opened {opened}")

def cp_canceled(_cp_id: int):
    """Color Picker Callback"""
    print("color picker canceled")


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

            with ColorPicker(
                on_open=cp_opened, # Callback when button pressed
                on_submit=color_selected, # Callback with the selected color
                on_cancel=cp_canceled, # Callback when canceled
                ):
                # Important: a button needs to be added to open the color picker
                # This allows one to style the button as one wishes.
                add_button(label="Color Picker",
                           padding=[3.0],
                           style_id=btn_style)

            text_id = add_text(content="Color value here")

start_session()
