#!/usr/bin/env python3
"""
Color Picker use demo

Add a color_picker widget.

A color picker that opens from a button, allowing the user
to select a color interactively.

Parameters
----------
parent_id : str
    Sets the parent container ID that this color picker belongs to.
label : str,  Optional
    Sets the Text label displayed on the button.
gen_id : int,  Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.
on_press : callable,  Optional
    Sets the Callback method to invoke when the button is pressed.
on_select : callable,  Optional
    Sets the Callback method to invoke when a color is selected.
on_cancel : callable,  Optional
    Sets the Callback method to invoke when the color selection is cancelled.
color : Color,  Optional
    Sets the initial color using a predefined color variant.
color_alpha : float,  Optional
    Sets the alpha of the Color.
color_rgba : list of float,  Optional
    Sets the initial color in rgba format as [r, g, b, a].
width : float,  Optional
    Sets the Fixed width in logical pixels.
width_fill : bool, default False
    Whether the button fills available width.
height : float,  Optional
    Sets the Fixed height in logical pixels.
height_fill : bool, default False
    Whether the button fills available height.
padding : list of float,  Optional
    Sets the Padding as [all], [vertical, horizontal], or
    [top, right, bottom, left].
clip : bool,  Optional
    Whether to clip content that overflows the button.
style_id : int,  Optional
    Sets the ID of a custom style created with ``add_button_style``.
style_std : ButtonStyleStd,  Optional
    Sets the predefined standard style variant.
style_arrow : Arrow,  Optional
    Sets the arrow icon style for the button.
user_data : Any,  Optional
    Sets the Arbitrary data forwarded to callbacks.
show : bool, default False
    Whether the color picker is visible.

Returns
-------
int
    The numeric widget ID of the newly created color picker.
"""

from icedpygui import Window, Column, Container,\
    add_color_picker, add_text, \
    start_session, update_widget, TextParam


def color_selected(_cp_id: int, color: list):
    """
    Color selected callback
    Need to change the list color to a str type
    Since the color is being displayed as text
    otherwise use as is or convert to what is needed
    """
    string = f"{color}"

    # update the text
    update_widget(text_id, TextParam.Content, string)

    # update the text rgba color
    update_widget(text_id, TextParam.ColorRgba, color)


def cp_opened(_cp_id: int):
    """Color Picker Callback"""
    print("color picker opened")


def cp_canceled(_cp_id: int):
    """Color Picker Callback"""
    print("color picker canceled")


# Add a window first
with Window(
    title="ColorPicker",
    size=(500.0, 500.0),
    center=True):

    # Add the container.
    with Container(
        fill=True,
        align_center=True):

        # Add a column to hold multiple widgets
        with Column(spacing=20.0):

            add_color_picker(
                on_press=cp_opened, # Button to open color picker
                on_select=color_selected, # the color selection selected
                on_cancel=cp_canceled) # the color selection cancel


            text_id = add_text(content="Color value here")

start_session()
