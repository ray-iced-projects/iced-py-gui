#!/usr/bin/env python3
"""
Update widget user data demo
"""

from icedpygui import (Window, Column, Container, 
    add_button, 
    update_user_data,
    get_user_data,
    start_session)


def update(btn_id: int):
    """Updating user datae"""
    update_user_data(btn_id, (1, 2))
    print(get_user_data(btn_id))
    update_user_data(btn_id, ("Some Data", "More Data"))
    print(get_user_data(btn_id))



with Window(title="Test Widget Delete", center=True):
    with Container(fill=True, align_center=True):
        with Column(spacing=20.0):
            add_button(
                label="Pressing me to update user data",
                on_press=update)
            

start_session()
