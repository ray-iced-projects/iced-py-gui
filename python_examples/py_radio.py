#!/usr/bin/env python3
"""
Radio button demo

Radio buttons allows one to select an item from a list of buttons
"""

from icedpygui import Window, Container, Column, Row, start_session, \
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

        with Column(spacing=10.0):

            rd1_labels = ["one", "two", "three", "four"]

            add_text(content="The spacing between the buttons")
            add_text(content="Left is default, right is radio_spacing=10")
            with Row(spacing=30):
                add_radio(
                    labels=rd1_labels,
                    on_selected=rd1_selected)

                add_radio(
                    labels=rd1_labels,
                    radio_spacing=10,
                    on_selected=rd1_selected)

            add_text(content="The spacing=20 between the button and the text")
            add_radio(
                labels=["one", "two", "three", "four"],
                spacing=20,
                on_selected=rd2_selected)

            add_text(content="styling and horizontal")
            add_text(content="For horzontal, radio spacing is needed because for vertical,")
            add_text(content="the line_height of the text dictates the default radio spacing.")
            add_radio(
                labels=["one", "two", "three", "four"],
                selected_index=1,
                horizontal=True,
                radio_spacing=8,
                style_id=style_id,
                on_selected=rd2_selected)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
