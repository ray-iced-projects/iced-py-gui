#!/usr/bin/env python3
"""
Button use demo
"""

from icedpygui import Window, Column, Container, \
    add_button, ButtonParam, Arrow, update_widget, start_session

counter = 0
def on_press(btn_id: int):
    match counter:
        case 0: update_widget(btn_id, ButtonParam.ArrowStyle, Arrow.ArrowRight)
        case 1: update_widget(btn_id, ButtonParam.Clip, True)
    
#  First add a window
with Window(title="Button Parameters",
            size=(800, 600), center=True):

    # Add container to hold the button
    with Container(align_center=True, fill=True):
        add_button(label="Press Me to change my parameters", on_press=on_press)

start_session()
