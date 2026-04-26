#!/usr/bin/env python3
"""
Svg demo

Add an SVG widget.

Displays a scalable vector graphic from a file path with
optional mouse interaction.

Parameters
----------
parent_id : str
    Sets the parent container ID that this SVG belongs to.
path : str
    Sets the file path to the SVG image.
width : float, Optional
    Sets the Fixed width in logical pixels.
width_fill : bool, default False
    Whether the SVG fills available width.
height : float, Optional
    Sets the Fixed height in logical pixels.
height_fill : bool, default False
    Whether the SVG fills available height.
color_filter : Color, Optional
    Sets the color filter using a predefined color variant.
color_filter_alpha : float, Optional
    Sets the alpha of the Color.
rgba_filter : list of float, Optional
    Sets the color filter in rgba format as [r, g, b, a].
content_fit : ContentFit, Optional
    Sets the content fit strategy for the SVG.
rotation_solid : Rotation, Optional
    Sets the non-default rotation method to Solid for the SVG.
    Floating (default): The element will float while rotating. The layout will be kept exactly as it was
    before the rotation.
    Solid: The element will be solid while rotating. The layout will be adjusted to fit
    the rotated content.
rotation_radians : float, Optional
    Sets the rotation angle in radians.
opacity : float, Optional
    Sets the opacity of the SVG (0.0 to 1.0).
show : bool, default True
    Whether the SVG is visible.
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.

Returns
-------
int
    The numeric widget ID of the newly created SVG.
"""
import os
from icedpygui import Window, Column, Container, start_session, \
    add_svg, SvgParam, update_widget_params, add_button

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
