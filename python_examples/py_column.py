#!/usr/bin/env python3
"""
Column use demo

Add a column widget.

A column lays out its children vertically from top to bottom.

Parameters
----------
window_id : str
    Sets the window this column belongs to.
container_id : str
    Sets the Unique string identifier for the column.
parent_id : str,  Optional
    Sets the parent container ID.  Defaults to the window itself.
width : float,  Optional
    Sets the Fixed width in logical pixels.
width_fill : bool, default False
    Whether the column fills available width.
height : float,  Optional
    Sets the Fixed height in logical pixels.
height_fill : bool, default False
    Whether the column fills available height.
fill : bool, Optional
    Whether to fill both the available width and height
max_width : float,  Optional
    Sets the Maximum width in logical pixels.
padding : list of float,  Optional
    Sets the Padding as ``[all]``, ``[vertical, horizontal]``, or
    ``[top, right, bottom, left]``.
spacing : float,  Optional
    Sets the Vertical spacing between children in logical pixels.
align_left : bool,  Optional
    Whether to Align children to the left.
align_center : bool,  Optional
    Whether to Align children to the horizontal centre.
align_right : bool,  Optional
    Whether to Align children to the right.
clip : bool,  Optional
    Whether to clip content that overflows the column.
show : bool, default True
    Whether the column is visible.

Returns
-------
int
    The numeric widget ID of the newly created column.
"""
from icedpygui import Window, Column, Container, Row, start_session, \
    add_text, ContainerStyleStd


# Add the window
with Window(
    title="Column",
    size=(600, 600),
    center=True) as wnd:

    # Need a column and row to hold the widgets
    with Column(spacing=20.0, padding=[20.0], fill=True):

        add_text(content="Spacing of 10.0 and 20.0")

        with Row(width_fill=True, height=100.0, spacing=20.0):
            # We add two text widgets to each column below to demonstrate spacing
            # The container is just for showing a little background for highlighting
            with Container(width=200.0, height_fill=True,
                    style_std=ContainerStyleStd.BorderedBox):
                with Column(spacing=10.0):
                    add_text(content="Text in Column 1")
                    add_text(content="Text in Column 1")

            with Container(width=200.0, height_fill=True,
                    style_std=ContainerStyleStd.BorderedBox):
                with Column(spacing=20.0):
                    add_text(content="Text in Column 2")
                    add_text(content="Text in Column 2")

        add_text(content="Padding of [all] and [top, right, botton, left]\n" +
                "Note the space around the Column outline")


        with Row(width_fill=True, height=100.0, spacing=20.0):
            with Container(style_std=ContainerStyleStd.BorderedBox):
                with Column(padding=[20.0]):
                    add_text(content="Padding all")
                    add_text(content="Padding all")

            with Container(style_std=ContainerStyleStd.BorderedBox):
                with Column(padding=[20.0, 0.0, 20.0, 0.0]):
                    add_text(content="Padding Top/Bottom")
                    add_text(content="Padding Top/Bottom")

        add_text(content="Alignment parameter = align\n" +
                 "values = align_left(default), align_center, align_right\n" +
                 "NOTE the empty space(s) beside each text group")


        with Row(width_fill=True, height=75.0, spacing=20.0):
            with Container(style_std=ContainerStyleStd.BorderedBox):
                with Column(width=175, height=100, align_left=True):
                    add_text(content="Left")
                    add_text(content="Left")

            with Container(style_std=ContainerStyleStd.BorderedBox):
                with Column(width=175, height=100, align_center=True):
                    add_text(content="Center")
                    add_text(content="Center")

            with Container(style_std=ContainerStyleStd.BorderedBox):
                with Column(width=175, height=100, align_right=True):
                    add_text(content="Right")
                    add_text(content="Right")

# last thing is to start the session
start_session()
