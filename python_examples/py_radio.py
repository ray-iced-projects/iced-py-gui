#!/usr/bin/env python3
"""
Radio button use demo

Radio buttons allows one to select an item from a list of buttons
"""

from icedpygui import Window, Container, Column, start_session, \
    add_radio, add_radio_style, add_text, Color



def rd1_selected(_rd_int: int, index: int):
    """Radio callback"""
    print(index, rd1_labels[index])

def rd2_selected(_rd_int: int, index: int):
    """Radio callback"""
    print(index)


style_id = add_radio_style(border_color=Color.YELLOW, dot_color=Color.YELLOW_GREEN)

with Window(
    title="Radio Demo",
    size=(500, 600),
    center=True):

    with Container(fill=True, align_center=True):

        with Column(spacing=20.0):

            rd1_labels = ["one", "two", "three", "four"]

            add_text(content="Note the spacing between the buttons, default=8")
            add_radio(
                labels=rd1_labels,
                radio_spacing=15,
                on_selected=rd1_selected)

            add_text(content="Note the spacing between the button and the text")
            add_radio(
                labels=["one", "two", "three", "four"],
                spacing=20,
                on_selected=rd2_selected)

            add_text(content="styling")
            add_radio(
                labels=["one", "two", "three", "four"],
                spacing=20,
                selected_index=1,
                style_id=style_id,
                on_selected=rd2_selected)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
