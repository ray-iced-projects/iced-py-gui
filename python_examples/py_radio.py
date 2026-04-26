#!/usr/bin/env python3
"""
Radio button demo

Add a radio button group widget.

A group of radio buttons where the user can select one option
from a list of labels.

Parameters
----------
parent_id : str
    Sets the parent container ID that this radio group belongs to.
labels : list of str
    Sets the list of labels for each radio button.
horizontal : bool, default false
    Whether the layout direction is horizontal (default vertical).
spacing : float, Optional
    Sets the spacing between the radio circle and its label.
radio_spacing: float, Optional
    Sets the spacing between radio buttons.
padding : list of float, Optional
    Sets the Padding as [all], [vertical, horizontal], or
    [top, right, bottom, left].
width : float, Optional
    Sets the Fixed width in logical pixels.
width_fill : bool, default False
    Whether the radio group fills available width.
height : float, Optional
    Sets the Fixed height in logical pixels.
height_fill : bool, default False
    Whether the radio group fills available height.
on_selected : callable, Optional
    Sets the Callback method to invoke when a radio button is selected.
selected_index : int, Optional
    Sets the index of the initially selected radio button.
size : float, Optional
    Sets the size of the radio circle in logical pixels.
style_id : int, Optional
    Sets the ID of a custom style created with ``add_radio_style``.
font_id : int, Optional
    Sets the Font ID for the label text.
text_spacing : float, Optional
    Sets the spacing between the radio circle and text.
text_size : float, Optional
    Sets the Font size for the label text.
text_line_height : float, Optional
    Sets the Line height for the label text.
text_shaping : TextShaping, Optional
    Sets the Text shaping strategy for the labels.
text_wrapping : TextWrapping, Optional
    Sets the Text wrapping strategy for the labels.
user_data : Any, Optional
    Sets the Arbitrary data forwarded to callbacks.
show : bool, default True
    Whether the radio group is visible.
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.
Returns
-------
int
    The numeric widget ID of the newly created radio group.

*******************************************************************
Add styling to a radio button group.

Creates a custom style that can be applied to a radio group
via its ``style_id`` parameter.

Parameters
----------
background_color : Color, Optional
    Sets the background color using a predefined color variant.
background_color_alpha : float, Optional
    Sets the alpha of the Color.
background_rgba : list of float, Optional
    Sets the background color in rgba format as [r, g, b, a].
background_color_hovered : Color, Optional
    Sets the background color when hovered using a predefined color variant.
background_color_hovered_alpha : float, Optional
    Sets the alpha of the Color.
background_rgba_hovered : list of float, Optional
    Sets the background color when hovered in rgba format as [r, g, b, a].
border_color : Color, Optional
    Sets the border color using a predefined color variant.
border_color_alpha : float, Optional
    Sets the alpha of the Color.
border_rgba : list of float, Optional
    Sets the border color in rgba format as [r, g, b, a].
border_width : float, Optional
    Sets the border width in logical pixels.
dot_color : Color, Optional
    Sets the dot color using a predefined color variant.
dot_color_alpha : float, Optional
    Sets the alpha of the Color.
dot_rgba : list of float, Optional
    Sets the dot color in rgba format as [r, g, b, a].
dot_color_hovered : Color, Optional
    Sets the dot color when hovered using a predefined color variant.
dot_color_hovered_alpha : float, Optional
    Sets the alpha of the Color.
dot_rgba_hovered : list of float, Optional
    Sets the dot color when hovered in rgba format as [r, g, b, a].
text_color : Color, Optional
    Sets the text color using a predefined color variant.
text_color_alpha : float, Optional
    Sets the alpha of the Color.
text_rgba : list of float, Optional
    Sets the text color in rgba format as [r, g, b, a].
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.

Returns
-------
int
    The numeric style ID to pass to a radio group's ``style_id``.
"""

from icedpygui import Window, Container, Column, Row, start_session, \
    add_radio, add_radio_style, add_text, Color



def rd1_selected(_rd_int: int, index: int):
    """Radio callback"""
    print(index, rd1_labels[index])

def rd2_selected(_rd_int: int, index: int):
    """Radio callback"""
    print(index)


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
                labels=["one", "two", "three", "four"],
                spacing=20,
                on_selected=rd2_selected)

            add_text(content=CONTENT_2)

            add_radio(
                labels=["one", "two", "three", "four"],
                selected_index=1,
                horizontal=True,
                radio_spacing=8,
                style_id=style_id,
                on_selected=rd2_selected)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
