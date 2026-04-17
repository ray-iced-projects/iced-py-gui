#!/usr/bin/env python3
"""
Picklist use demo

PickList allows one to select an item from a dropdown menu
"""
from icedpygui import Window, Column, Container, Row, start_session, \
    add_progress_bar, add_progress_bar_style, ProgressBarParam, ProgressBarStylStd, \
    add_button, add_space, add_text_input, add_text, TextParam, Color, update_widget

# global var for callback
state = {"value": 75.0,
        "hide": True}

# The callbacks below allow you to change all of the parameters for a widget.
# They may or may not have frequent usage but it makes the gui very flexible
# when the data that may be loaded effects the placement, sizes, etc. used.
# These callbacks also demonstrate the usage of the widget parameters and
# are used in the testing of the code to make sure it behaves as expected.

# increment the bar, the pg_id comes in as the user_data for the button
# This could be a global var or usually placed in a class
def change_value_plus(_btn_id: int, pb_id: int):
    """Button Callback"""
    state["value"] += 1
    update_widget(pb_id, ProgressBarParam.Value, state["value"])
    update_widget(current_value_id, TextParam.Content, f"Current Value = {state["value"]}")


# decrement the bar value
def change_value_minus(_btn_id: int, pb_id: int):
    """Button callback"""
    state["value"] -= 1
    update_widget(pb_id, ProgressBarParam.Value, state["value"])
    update_widget(current_value_id, TextParam.Content, f"Current Value = {state["value"]}")


def change_min(_text_input_id: int, min_val: str, pb_id: int):
    """Text Input callback"""
    # text_input values are str therefore they are changed to  a float
    # Int and float input soon to follow.
    min_float = float(min_val)
    update_widget(pb_id, ProgressBarParam.Min, min_float)
    # Since the text content requires a str, the min_val can be used
    update_widget(min_text, TextParam.Content, min_val)


def change_max(_text_input_id: int, max_val: str, pb_id: int):
    """Text Input callback"""
    max_float = float(max_val)
    update_widget(pb_id, ProgressBarParam.Max, max_float)
    update_widget(max_text, TextParam.Content, max_val)


def change_height(_btn_id: int, pb_id: int):
    """Button callback"""
    update_widget(pb_id, ProgressBarParam.Height, 30.0)


def change_width(_btn_id: int, pb_id: int):
    """Button callback"""
    update_widget(pb_id, ProgressBarParam.Width, 300.0)


def change_width_to_fill(_btn_id: int, pb_id: int):
    """Button callback"""
    update_widget(pb_id, ProgressBarParam.WidthFill, True)


def hide_bar(_btn_id: int, pb_id: int):
    """Button callback"""
    state["hide"] = not state["hide"]
    update_widget(pb_id, ProgressBarParam.Show, state["hide"])


# Add the window
with Window(title="Progress bar Demo", center=True):

    # Add the container to help with alignment
    with Container(fill=True):

        # Add a column for the widgets
        with Column(align_center=True, spacing=2):

            add_space(height=50.0)

            # add a row for the  for the pg bar
            with Row(width=400.0):

                # Add the pg bar
                pg_id = add_progress_bar(
                                min=50.0,
                                max=100.0,
                                value=state["value"])

            # add a row to display text value at the start and end of the pg bar
            with Row(width=400.0):

                # The text, space and more text just below the pg bar.
                min_text = add_text(content="50")
                sp_id = add_space(width=320.0)
                max_text = add_text(content="100")

        # Adding new column because current column has too small of a spacing value
        # This column can go into the column above because container only holds 1 widget
        # If this was more complex, you could add another container to the window then proceed.
        with Column(align_center=True):

            # Add a text widget for current value
            current_value_id = add_text(content=f"Current Value = {state["value"]}")

            # Add row for increment and decrement buttons
            with Row():

                # Increment button
                add_button(
                        label="Press Me to + ",
                        on_press=change_value_plus,
                        user_data=pg_id)

                # Decrement button
                add_button(
                        label="Press Me to - ",
                        on_press=change_value_minus,
                        user_data=pg_id)

            # add row for min and max
            with Row():

                # text input widgets are used for the inputs
                # which you convert to floats in the callback
                # Numeric input widgets to come.  No error checking done.
                add_text_input(
                        placeholder="Enter Min",
                        on_submit=change_min,
                        width=150.0,
                        user_data=pg_id)

                add_text_input(
                        placeholder="Enter Max",
                        on_submit=change_max,
                        width=150.0,
                        user_data=pg_id)

            # Add a button the short the bar
            add_button(
                    label="Press Me to shorten the bar",
                    on_press=change_width,
                    user_data=pg_id)

            # Add a button the lengthen the bar
            add_button(
                    label="Press Me to to fill the bar width, do the above first",
                    on_press=change_width_to_fill,
                    user_data=pg_id)

            # Add a button to hide the bar
            add_button(
                    label="Press me to hide/show the bar.",
                    on_press=hide_bar,
                    user_data=pg_id)

            # add some styling to a new bar
            border = add_progress_bar_style(
                        border_radius=[8.0],
                        border_color=Color.BLUE,
                        border_width=3.0,
                        background_color=Color.LIGHT_BLUE,
                        bar_color=Color.ALICE_BLUE)


            # Adding another bar and styling with a new background, bar color, and border.
            add_progress_bar(
                min=0.0,
                max=100.0,
                value=50.0,
                style_id=border)

            add_text(
                content="Styling with a new bar color, background color, and border")

            # Adding another bar with just a standard styling.
            add_progress_bar(
                    min=0.0,
                    max=100.0,
                    value=50.0,
                    style_std=ProgressBarStylStd.Danger)

            add_text(
                content="Styling with Danger standard style only")


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
