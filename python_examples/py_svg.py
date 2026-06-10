#!/usr/bin/env python3
"""
Svg demo
"""
import os
from icedpygui import (Window, Column, Container, start_session,
    add_svg, SvgParam, update_widget_params, add_button)

state = {
    "width": 400.0,
    "height": 400.0
}


# The size of the svg will only get as big as the size of
# the container it's in.
def increase_size(_btn_id: int):
    """Button callback"""
    state["width"] += 10
    state["height"] += 10
    update_widget_params(svg_id, {
        SvgParam.Width: state["width"],
        SvgParam.Height: state["height"]})


def decrease_size(_btn_id: int):
    """Button callback"""
    state["width"] -= 10
    state["height"] -= 10
    update_widget_params(svg_id, {
        SvgParam.Width: state["width"],
        SvgParam.Height: state["height"]})

# Floating will not resize the container
def rotate_with_floating(_btn_id: int):
    """Rotating with Floating"""
    update_widget_params(svg_id, {
        SvgParam.RotationDegrees: 45,
        SvgParam.RotationSolid: False}) # ensure it's false = Floating

# Floating will not resize the container
def rotate_with_solid(_btn_id: int):
    """Rotating with Floating"""
    update_widget_params(svg_id, {
        SvgParam.RotationDegrees: 45,
        SvgParam.RotationSolid: True})

# Setting up the image path
cwd = os.getcwd()
tiger_path = cwd + "/python_examples/resources/png_svg/tiger.svg"

with Window(
        title="Svg Demo",
        size=(600, 600),
        center=True,
        debug=True):

    with Container(width_fill=True, align_center=True):

        with Column(align_center=True, spacing=10):

            svg_id = add_svg(
                        path=tiger_path,
                        width=state["width"],
                        height=state["height"])

            add_button(
                    label="Increase Size",
                    on_press=increase_size)

            add_button(
                    label="Decrease Size",
                    on_press=decrease_size)

            add_button(
                    label="Rotate Float (default)",
                    on_press=rotate_with_floating)

            add_button(
                    label="Rotate Solid",
                    on_press=rotate_with_solid)
start_session()
