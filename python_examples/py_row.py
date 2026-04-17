#!/usr/bin/env python3
"""
Row demo use demo
"""
from icedpygui import Window, Column, Container, Row, start_session, \
    add_text, ContainerStyleStd


# Add the window
with Window(
    title="Row Demo",
    size=(600, 600),
    center=True) as wnd:

    # Need a Column and row to hold the widgets
    with Column(spacing=20.0, padding=[20.0], width_fill=True, height_fill=True):

        add_text(content="Spacing of 10.0 and 20.0")

        with Row(width_fill=True, height=50.0, spacing=20.0):
            # We use add_row here to show the alternate version
            # We add two text widgets to each Row below to demonstrate spacing
            # The container is just for showing a little background for highlighting
            with Container(width=200.0, height_fill=True,
                    style_std=ContainerStyleStd.BorderedBox):
                with Row(spacing=10.0):
                    add_text(content="spacing 10")
                    add_text(content="spacing 10")

            with Container(width=200.0, height_fill=True,
                    style_std=ContainerStyleStd.BorderedBox):
                with Row(spacing=20.0):
                    add_text(content="spacing 20")
                    add_text(content="spacing 20")

        add_text(content="Padding of [all] and [top, right, botton, left]\n" +
                "Note the space around the Row outline")


        with Row(width_fill=True, height=100.0, spacing=20.0):
            with Container(style_std=ContainerStyleStd.BorderedBox):
                with Row(spacing=20.0, padding=[20.0]):
                    add_text(content="Padding all")
                    add_text(content="Padding all")

            with Container(style_std=ContainerStyleStd.BorderedBox):
                with Row(spacing=20.0, padding=[20.0, 0.0, 20.0, 0.0]):
                    add_text(content="Padding\nTop/Bottom")
                    add_text(content="Padding\nTop/Bottom")

        add_text(content="Alignment: bool\n" +
                 "NOTE the empty space(s) beside each text group")


        with Row(width_fill=True, height=75.0, spacing=20.0):
            with Container(style_std=ContainerStyleStd.BorderedBox):
                with Row(width=175, height=100, spacing=10.0, align_top=True):
                    add_text(content="top")
                    add_text(content="top")

            with Container(style_std=ContainerStyleStd.BorderedBox):
                with Row(width=175, height=100, spacing=10.0, align_center=True):
                    add_text(content="center")
                    add_text(content="center")

            with Container(style_std=ContainerStyleStd.BorderedBox):
                with Row(width=175, height=100, spacing=10.0, align_bottom=True):
                    add_text(content="bottom")
                    add_text(content="bottom")

# last thing is to start the session
start_session()
