#!/usr/bin/env python3
"""
Mouse Area use demo
"""

from icedpygui import Window, Container, MouseArea,\
    add_text, add_mouse_area, start_session

def on_press(wid, _user_data):
    """OnLeft press callback"""
    print("on left press", wid)


def on_release(ma_id, _user_data):
    """OnLeft release callback"""
    print("on left release", ma_id)


def on_right_press(ma_id, _user_data):
    """OnRight press callback"""
    print("on right press", ma_id)


def on_right_release(ma_id, _user_data):
    """OnRight release callback"""
    print("on right release", ma_id)


def on_middle_press(ma_id, _user_data):
    """OnMiddle press callback"""
    print("on middle press", ma_id)


def on_middle_release(ma_id, _user_data):
    """OnMiddle release callback"""
    print("on mma_iddle release", ma_id)


def on_enter(ma_id, user_data):
    """OnEnter callback"""
    print("entered", ma_id, user_data)


def on_move(ma_id: int, point: dict, _user_data):
    """OnMove callback"""
    print("on move", ma_id, point)


def on_exit(ma_id, _user_data):
    """OnExit callback"""
    print("on exit", ma_id)


with Window(
    title="Mouse Area",
    size=(400, 400),
    center=True,
    debug=True):

    with Container(
        width_fill=True,
        height_fill=True,
        align_center=True,):

        with MouseArea(
            on_enter=on_enter,
            on_exit=on_exit,
            on_move=on_move,
            on_press=on_press,
            on_release=on_release,
            on_middle_press=on_middle_press,
            on_middle_release=on_middle_release,
            on_right_press=on_right_press,
            on_right_release=on_right_release,
            user_data="Some Data"
            ):

            # A text widget was added here but you can also add containers or other widgets too.
            add_text(content="my content 1")

            # you will probably rarely add more than one item to a mousearea
            # but the option for more is there.
            add_text(content="my content 2")

start_session()
