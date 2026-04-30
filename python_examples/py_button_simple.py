#!/usr/bin/env python3
"""
Button use demo
"""

from icedpygui import Window, Container, Column, start_session, \
    add_button, add_button_style, Color



def on_press(btn_id: int):
    """Callback to change the state of the button"""
    print(btn_id)

bkg_color = add_button_style(bkg_color=Color.RED)

#  First add a window
with Window(title="Button Parameters",
            size=(400, 400), center=True):

    # Add container to hold the button
    with Container(fill=True, align_center=True):
        with Column(spacing=20):
            add_button(label="status=Active", status_active=True, style_id=bkg_color)
            add_button(label="status=Hovered", status_hovered=True, style_id=bkg_color)
            add_button(label="status=Pressed", status_pressed=True, style_id=bkg_color)
            add_button(label="status=Disabled", status_disabled=True, style_id=bkg_color)

start_session()
