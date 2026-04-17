#!/usr/bin/env python3
"""
Picklist use demo

PickList allows one to select an item from a dropdown menu
"""
from icedpygui import Window, Column, Container, Row, start_session, ContainerStyleStd, \
    add_progress_bar, add_progress_bar_style, ProgressBarParam, ProgressBarStyleStd, \
    add_button, add_space, add_text_input, add_text, TextParam, Color, update_widget, \
    update_widget_params

# global var for callback
state = {"value": 25.0,
         "step": 1.0}

# increment the bar, the pg_id comes in as the user_data for the button
# This could be a global var or usually placed in a class
def change_value_plus(_btn_id: int, pb_id: int):
    """Button Callback"""
    state["value"] += state["step"]
    update_widget(pb_id, ProgressBarParam.Value, state["value"])
    update_widget(current_value_id, TextParam.Content, f"Current Value = {state["value"]}")


# decrement the bar value
def change_value_minus(_btn_id: int, pb_id: int):
    """Button callback"""
    state["value"] -= state["step"]
    update_widget(pb_id, ProgressBarParam.Value, state["value"])
    update_widget(current_value_id, TextParam.Content, f"Current Value = {state["value"]}")


def change_min(_text_input_id: int, min_val: str, pb_id: int):
    """Text Input callback"""
    # text_input values are str therefore they are changed to  a float
    min_float = float(min_val)

    update_widget_params(pb_id, {ProgressBarParam.Min: min_float,
                                 ProgressBarParam.Value: min_float})
    # Since the text content requires a str, the min_val can be used as is
    update_widget(min_text, TextParam.Content, min_val)
    # update the displayed value
    state["value"] = min_float
    update_widget(current_value_id, TextParam.Content, f"Current Value = {min_val}")


def change_max(_text_input_id: int, max_val: str, pb_id: int):
    """Text Input callback"""
    max_float = float(max_val)
    update_widget(pb_id, ProgressBarParam.Max, max_float)
    update_widget(max_text, TextParam.Content, max_val)

def change_step(_text_input_id: int, step: str):
    """Text Input callback"""
    state["step"] = float(step)
    update_widget(step_value_id, TextParam.Content, f"Step Value = {step}")


# Add the window
with Window(title="Progress bar Demo", center=True):

    # Add the container to help with alignment
    with Container(fill=True, align_center=True):

        # Add a column for the widgets
        with Column(align_center=True, spacing=20):

            add_space(height=50.0)

            # add a row for the  for the pg bar
            with Container(align_center=True, width=400, height=100,
                           style_std=ContainerStyleStd.BorderedBox):
                with Column(spacing=10, align_center=True):
                    with Row(spacing=10.0):
                        # add text on left side
                        min_text = add_text(content="50")
                        # Add the pg bar
                        pg_id = add_progress_bar(
                                        min=0.0,
                                        max=100.0,
                                        width=300,
                                        value=state["value"])
                        # add text on right side
                        max_text = add_text(content="100")
                    # Add a text widget for current value
                    current_value_id = add_text(content=f"Current Value = {state["value"]}")
                    # Add a text widget for step value
                    step_value_id = add_text(content=f"Step Value = {state["step"]}")

            # Adding new column because current column has too small of a spacing value
            # This column can go into the column above because container only holds 1 widget
            # If this was more complex, you could add another container to the window then proceed.
            with Column(align_center=True, spacing=20):

                add_text(content="These buttons are for demo, normally you would link \n \
                    them to some changing values")

                # Add row for increment and decrement buttons
                with Row(spacing=20):

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


                add_text(content="Inputting new values for min and max")

                # add row for min and max
                with Row():

                    # text input widgets are used for the inputs
                    # which you convert to floats in the callback
                    # Numeric input widgets to come.  No error checking done.
                    add_text_input(
                            placeholder="Enter New Min",
                            on_submit=change_min,
                            width=150.0,
                            user_data=pg_id)

                    add_text_input(
                            placeholder="Enter New Max",
                            on_submit=change_max,
                            width=150.0,
                            user_data=pg_id)

                    add_text_input(
                            placeholder="Enter New Step Value",
                            on_submit=change_step,
                            width=200.0)



                # add some styling to a new bar
                border = add_progress_bar_style(
                            border_radius=[8.0],
                            border_color=Color.LIGHT_BLUE,
                            border_width=3.0,
                            background_color=Color.DARK_BLUE,
                            bar_color=Color.ALICE_BLUE)

                # adding some descritive text
                add_text(content="Styling with a new bar color, background color, and border")

                # Adding another bar and styling with a new background, bar color, and border.
                add_progress_bar(
                    min=0.0,
                    max=100.0,
                    width=200,
                    value=50.0,
                    style_id=border)

                # adding some descritive text
                add_text(content="Styling with Danger standard style only")

                # Adding another bar with just a standard styling.
                add_progress_bar(
                        min=0.0,
                        max=100.0,
                        width=200,
                        value=50.0,
                        style_std=ProgressBarStyleStd.Danger)




# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
