#!/usr/bin/env python3
"""
Separator demo

Decorator widget for separating widgets
"""

from icedpygui import Window, Container, Column, start_session, \
    add_separator

# Add a window first
# if you make debug=True, you can see
# the outline of the box containing
# the separators and the effect of the
# width and height settings.
with Window(
    title="Separator Demo",
    size=(600, 600),
    center=True,
    debug=False):

    with Container(fill=True, align_center=True):

        with Column(spacing=20.0, align_center=True):

            add_separator(
                dot=True,
                dot_radius=3.0,
                dot_count=10,
                spacing=10.0,)

            add_separator(
                line=True,
                line_length=150.0,
                line_thickness=2.0)

            add_separator(
                label="label",
            )

            # adding small lines before and after label
            add_separator(
                label=" label ",
                label_left_width=2.0,
                label_right_width=2.0,
            )

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
