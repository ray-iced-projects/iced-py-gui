#!/usr/bin/env python3
"""
Widget move use demo
"""
from icedpygui import Window, Column, Container, add_button, \
    add_text, move_widget, start_session


def move_1_below_2(_id: int):
    """Frist widget moved"""
    move_widget(
        wid=txt_id1,
        move_after=txt_id2
    )

def move_2_below_3(_id: int):
    """Second widget moved"""
    move_widget(
        wid=txt_id2,
        move_after=txt_id3
    )

with Window(title="Test Widget Move", center=True):
    with Container(fill=True, align_center=True):
        with Column(spacing=20.0):
            add_button(
                label="Pressing me to move text 1 below text 2.",
                on_press=move_1_below_2)
            txt_id1 = add_text(content="Hi there, 1")
            txt_id2 = add_text(content="Hi there, 2")
            add_button(
                label="Pressing me to move text 2 below text 3.",
                on_press=move_2_below_3)
            with Column(spacing=20.0):
                txt_id3 = add_text(content="Hi there, 3")

start_session()
