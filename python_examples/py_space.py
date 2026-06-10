#!/usr/bin/env python3
"""
Space demo
"""

from icedpygui import (Window, Column, start_session,
    add_space, add_text)

# Add the window, debug is set to true to be able to see the space widget.
with Window(
        title="Space Demo 1",
        size=(400, 600),
        position=(100, 25),
        debug=True):

    # Add the column for the widgets, centered
    with Column(align_center=True, fill=True):

        # Add the space
        add_space(width=50.0, height=200.0)

        # Add some info text
        add_text(
            content="\nThere is a space outlined above,"
                    " \nwidth=100.0, height=50.0\n "
                    "If you resize the window, the spacing does not change because it is set, "
                    "unlike on window 2")

# Add another window
with Window(
    title="Space Demo 2",
    size=(400, 600),
    position=(600, 25),
    debug=True):

    # Add the column for the widgets, centered
    with Column(align_center=True, fill=True):

        # Add the space, since the window debug=True you will be able to see it.
        add_space(width_fill=True, height=100.0)

        # Add some text for info
        add_text(
            content="\nThere is a space outlined above, \nwidth_fill=True, height=100.0\n ")

        add_text(
            content="\nif you drag the window width, the spacing grows because width_fill=True\n ")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
