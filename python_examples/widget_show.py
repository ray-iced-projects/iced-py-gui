#!/usr/bin/env python3
"""
Widget show demo
"""
from icedpygui import Window, Column, Container, start_session, \
    add_button, add_text, show_widget, hide_widget


def show(_id: int):
    """Show"""
    show_widget(txt_id1)
    show_widget(txt_id2)

def hide(_id: int):
    """Hide"""
    hide_widget(txt_id1)
    hide_widget(txt_id2)

with Window(title="Test Widget Delete", center=True):
    with Container(fill=True, align_center=True):
        with Column(spacing=20.0):
            add_button(
                label="Pressing me to SHOW the text below.",
                on_press=show)
            add_button(
                label="Pressing me to HIDE the text below.",
                on_press=hide)
            txt_id1 = add_text(content="Hi there", show=False)
            txt_id2 = add_text(content="Hi there", show=False)

start_session()
