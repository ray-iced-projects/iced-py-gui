#!/usr/bin/env python3
"""
Separator demo

Add a separator widget.

A visual separator using lines, dots, or a labelled divider.

Parameters
----------
parent_id : str
    Sets the parent container ID that this separator belongs to.
label : str, Optional
    Sets the text label displayed in the separator.
separator_type : SeparatorType, Optional
    Sets the type of separator (line, dot, label, etc.).
label_left_width : float, Optional
    Sets the width of the line to the left of the label.
label_right_width : float, Optional
    Sets the width of the line to the right of the label.
dot_radius : float, Optional
    Sets the radius of each dot in logical pixels.
dot_count : int, Optional
    Sets the number of dots to display.
dot_fill : bool, default True
    Whether the dots are filled.
dot_border_width : float, Optional
    Sets the border width of each dot in logical pixels.
line_length : float, Optional
    Sets the length of the separator line in logical pixels.
line_thickness : float, Optional
    Sets the thickness of the separator line in logical pixels.
width : float, Optional
    Sets the Fixed width in logical pixels.
width_fill : bool, default False
    Whether the separator fills available width.
height : float, Optional
    Sets the Fixed height in logical pixels.
height_fill : bool, default False
    Whether the separator fills available height.
spacing : float, Optional
    Sets the spacing between separator elements in logical pixels.
style_id : int, Optional
    Sets the ID of a custom style created with ``add_separator_style``.
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.
show : bool, default True
    Whether the separator is visible.

Returns
-------
int
    The numeric widget ID of the newly created separator.
*********************************************************

Add styling to a separator.

Creates a custom style that can be applied to a separator
via its ``style_id`` parameter.

Parameters
----------
ipg_color : Color, Optional
    Sets the separator color using a predefined color variant.
ipg_color_alpha : float, Optional
    Sets the alpha of the Color.
rgba_color : list of float, Optional
    Sets the separator color in rgba format as [r, g, b, a].
border_ipg_color : Color, Optional
    Sets the border color using a predefined color variant.
border_ipg_color_alpha : float, Optional
    Sets the alpha of the Color.
border_rgba_color : list of float, Optional
    Sets the border color in rgba format as [r, g, b, a].
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.

Returns
-------
int
    The numeric style ID to pass to a separator's ``style_id``.
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
