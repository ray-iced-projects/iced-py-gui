#!/usr/bin/env python3
"""
Window use demo
"""

from icedpygui import Window, Container, add_text, start_session,\
    add_window, add_container



with Window( title="Window 1", center=True, size=(200, 200)):
    with Container(align_center=True, width_fill=True, height_fill=True):

        add_text(content="This is Some Text")


with Window( title="Window 2", center=True, size=(200, 200), position=[]):
    with Container(align_center=True, width_fill=True, height_fill=True):

        add_text(content="This is Some Text")


add_window(window_id="main2", title="Window 3", center=True, size=(200, 200))
add_container(container_id="cont", window_id="main2", align_center=True)
add_text(parent_id="cont", content="This is Some Text")

start_session()
