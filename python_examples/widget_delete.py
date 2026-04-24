#!/usr/bin/env python3
"""
Delete widget demo
"""

from icedpygui import Window, Column, Container, add_button, delete_widget, start_session


def delete(wid: int):
    """Delete"""
    delete_widget(wid)
    delete_widget(btn_id)


with Window(title="Test Widget Delete", center=True):
    with Container(fill=True, align_center=True):
        with Column(spacing=20.0):
            add_button(
                label="Pressing me to deletes all",
                on_press=delete)
            btn_id = add_button(
                label="Press me to delete me",
                on_press=delete)


start_session()
