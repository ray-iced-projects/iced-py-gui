#!/usr/bin/env python3
"""
Color Picker use demo
"""

from icedpygui import Window, Column, Container,\
    add_button_style, add_color_picker, add_text, \
    start_session, update_widget, IpgTextParam


def color_selected(_cp_id: int, color: list, _user_data: any):
    """
    Color selected callback
    Need to change the list color to a str type
    Since the color is being displayed as text
    otherwise use as is or convert to what is needed
    """

    string = "["
    for c in color:
        string += str(c) + ", "
    string += "]"

    # update the text
    update_widget(
        wid=text_id,
        param=IpgTextParam.Content,
        value=string)

    # update the text color
    update_widget(
        wid=text_id,
        # Note the type is Rgba, not IpgColor
        # You could use a value of type IpgColor
        # then use the TextColor without using the colorpicker
        param=IpgTextParam.TextRgba,
        value=color)


def cp_opened(_cp_id: int, _user_data: any):
    """ColorPicker is opened callback"""
    print("color picker opened")


def cp_canceled(_cp_id: int, _user_data: any):
    """ColorPicker selection canceled"""
    print("color picker canceled")

# Since the color_picker is using a button
# to open the selection widget, then
# to style the button you use the button styling
cp_style = add_button_style(border_radius=[5.0])


# Add a window first
with Window(
    title="ColorPicker",
    size=(500.0, 500.0),
    center=True):

    # Add the container to center both x and y (default).  Holds only one widget.
    with Container(
        width_fill=True,
        height_fill=True,
        align_center=True):

        # Add a column to hold multiple widgets
        with Column(spacing=20.0):

            add_color_picker(
                on_press=cp_opened, # Button to open color picker
                on_select=color_selected, # the color selection selected
                on_cancel=cp_canceled, # the color selection cancel
                padding=[5.0],
                style_id=cp_style,
                user_data="Something") #user data not used but supplied for testing
                # If you use user_data, all callback will require the user_data parameter
                # or whatever name you want for it.

            text_id = add_text(
                        content="Color value here")

start_session()
