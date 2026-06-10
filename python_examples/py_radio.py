#!/usr/bin/env python3
"""
Radio button demo
"""

from icedpygui import (Window, Container, Column, Row, start_session,
    add_radio, add_radio_style, add_text, Color)



def rd1_selected(_rd_int: int, index: int):
    """Radio callback"""
    print(index, rd1_labels[index])

def rd2_selected(_rd_int: int, index: int):
    """Radio callback"""
    print(index, rd2_labels[index])


style_id = add_radio_style(border_color=Color.YELLOW, dot_color=Color.YELLOW_GREEN)

CONTENT_1 = "The spacing between the buttons\n\
Left is default, right is radio_spacing=10"

CONTENT_2 = "****Styling and Horizontal****\n\
For horzontal, radio spacing is needed because for vertical,\
the line_height of the text dictates the default radio spacing."

with Window(
    title="Radio Demo",
    size=(500, 600),
    center=True):

    with Container(fill=True, align_center=True):

        with Column(spacing=10.0):

            rd1_labels = ["one", "two", "three", "four"]
            rd2_labels = ["six", "seven", "eight", "nine"]

            add_text(content=CONTENT_1)

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
                labels=rd2_labels,
                spacing=20,
                on_selected=rd2_selected)

            add_text(content=CONTENT_2)

            add_radio(
                labels=rd2_labels,
                selected_index=1,
                horizontal=True,
                radio_spacing=8,
                style_id=style_id,
                on_selected=rd2_selected)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
