#!/usr/bin/env python3
"""
Container use demo

Add a container widget.

A container wraps a single child widget and provides alignment,
sizing, padding, and optional styling.

Parameters
----------
window_id : str
    Sets the window this container belongs to.
container_id : str
    Sets the Unique string identifier for the container.
parent_id : str,  Optional
    Sets the parent container ID.  Defaults to the window itself.
width : float,  Optional
    Sets the Fixed width in logical pixels.
width_fill : bool, default False
    Whether the container fills available width.
fill : bool, Optional
    Whether to fill both the available width and height
height : float,  Optional
    Sets the Fixed height in logical pixels.
height_fill : bool, default False
    Whether the container fills available height.
clip : bool,  Optional
    Whether to clip content that overflows the container.
max_height : float,  Optional
    Sets the Maximum height in logical pixels.
max_width : float,  Optional
    Sets the Maximum width in logical pixels.
align_top_left : bool,  Optional
    Whether to Align the child to the top-left corner.
align_top_center : bool,  Optional
    Whether to Align the child to the top-centre.
align_top_right : bool,  Optional
    Whether to Align the child to the top-right corner.
align_center_left : bool,  Optional
    Whether to Align the child to the centre-left.
align_center : bool,  Optional
    Whether to Align the child to the centre.
align_center_right : bool,  Optional
    Whether to Align the child to the centre-right.
align_bottom_left : bool,  Optional
    Whether to Align the child to the bottom-left corner.
align_bottom_center : bool,  Optional
    Whether to Align the child to the bottom-centre.
align_bottom_right : bool,  Optional
    Whether to Align the child to the bottom-right corner.
padding : list of float,  Optional
    Sets the Padding as ``[all]``, ``[vertical, horizontal]``, or
    ``[top, right, bottom, left]``.
show : bool, default True
    Whether the container is visible.
style_id : int,  Optional
    Sets the ID of a custom style created with ``add_container_style``.
style_std : ContainerStyleStd,  Optional
    Sets the predefined standard style variant.

Returns
-------
int
    The numeric widget ID of the newly created container.
"""

from icedpygui import Window, Column, Container, Row, start_session, \
    add_text, ContainerStyleStd


# Add the window
with Window(
    title="Column",
    size=(600, 600),
    center=True) as wnd:

    # Need a column and row to hold the widget
    with Column(spacing=20.0, padding=[20.0], width_fill=True, height_fill=True):

        add_text(content="Alignments: booleans")

        with Row(width_fill=True, height=75.0, spacing=20.0):
            with Container(align_top_left=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="top_left")

            with Container(align_top_center=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="top_center")

            with Container(align_top_right=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="top_right")

        with Row(width_fill=True, height=75.0, spacing=20.0):

            with Container(align_center_left=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="center_left")

            with Container(align_center=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="center")

            with Container(align_center_right=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="center_right")

        with Row(width_fill=True, height=75.0, spacing=20.0):

            with Container(align_bottom_left=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="bottom_left")

            with Container(align_bottom_center=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="bottom_center")

            with Container(align_bottom_right=True,
                          width=175, height_fill=True,
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="bottom_right")

        add_text(content="Padding padding=[all] or padding=[top, right, bottom, left]")

        with Row(width_fill=True, height=75.0, spacing=20.0):
            with Container(padding=[20.0],
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="padding all")

            with Container(padding=[0.0, 20.0, 0.0, 20.0],
                          style_std=ContainerStyleStd.BorderedBox):
                add_text(content="padding left and right")


# last thing is to start the session
start_session()
