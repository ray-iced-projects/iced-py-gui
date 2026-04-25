#!/usr/bin/env python3
"""
Container styling use demo

Add styling to a container.

Creates a custom style that can be applied to a container
via its style_id parameter.

Parameters
----------
background_color : Color, Optional
    Sets the background color using a predefined color variant.
background_rgba : list of float, Optional
    Sets the background color in rgba format as [r, g, b, a].
background_alpha : float, Optional
    Sets the alpha transparency for the background color.
background_gradient_color_stop : Color, Optional
    Sets the stop color of the background gradient.
background_gradient_rgba_stop : list of float, Optional
    Sets the stop color of the background gradient in rgba format as [r, g, b, a].
background_gradient_degrees : float, Optional
    Sets the background gradient angle in degrees.
background_gradient_radians : float, Optional
    Sets the background gradient angle in radians.
background_gradient_alpha : float, Optional
    Sets the alpha transparency for the gradient stop color.
border_color : Color, Optional
    Sets the border color using a predefined color variant.
border_rgba : list of float, Optional
    Sets the border color in rgba format as [r, g, b, a].
border_alpha : float, Optional
    Sets the alpha transparency for the border color.
border_radius : list of float, Optional
    Sets the radius of the corners as [all] or
    [top-left, top-right, bottom-right, bottom-left].
border_width : float, Optional
    Sets the border width in logical pixels.
shadow_color : Color, Optional
    Sets the shadow color using a predefined color variant.
shadow_rgba : list of float, Optional
    Sets the shadow color in rgba format as [r, g, b, a].
shadow_alpha : float, Optional
    Sets the alpha transparency for the shadow color.
shadow_offset_xy : list of float, Optional
    Sets the shadow offset as [x, y] in logical pixels.
shadow_blur_radius : float, Optional
    Sets the shadow blur radius in logical pixels.
text_color : Color, Optional
    Sets the text color using a predefined color variant.
text_rgba : list of float, Optional
    Sets the text color in rgba format as [r, g, b, a].
text_alpha : float, Optional
    Sets the alpha transparency for the text color.
snap : bool, Optional
    Whether to snap the container to the pixel grid.
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.

Returns
-------
int
    The numeric style ID to pass to a container's style_id.
"""

from icedpygui import Window, Column, Container, start_session, \
    add_container_style, ContainerStyleStd, Color, add_text

# Add the styling container widget
cont_style = add_container_style(
                background_color=Color.AQUA,
                border_color=Color.BLUE,
                border_radius=[10.0],
                border_width=10.0,
                shadow_color=Color.YELLOW,
                shadow_blur_radius=20.0,
                shadow_offset_xy=[8.0, 8.0],
                text_color=Color.BLACK)

# Add the windows
with Window(title="Container Styling",
            size=(500, 600), center=True):

    # add a ccolumn to hold the containers
    with Column(spacing=50.0, align_center=True, padding=[20.0], width_fill=True):

        with Container(
            width=200.0,
            height=200.0,
            align_center=True,
            style_id=cont_style):

            add_text(content="Custom Style")

        # add the container to work on
        with Container(
            width=200.0,
            height=200.0,
            align_center=True,
            style_std=ContainerStyleStd.RoundedBox):

            add_text(content="Std Style: RoundedBox")


# last thing is to start the session
start_session()
