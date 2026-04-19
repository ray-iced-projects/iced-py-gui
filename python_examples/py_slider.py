#!/usr/bin/env python3
"""
Slider demo

By dragging a slider, one can change values of something
"""

from icedpygui import Window, Column, Row, start_session, \
    add_slider, add_slider_style, SliderParam, add_text, TextParam, \
    add_progress_bar, update_widget, ProgressBarParam, add_space, Color, \
    add_button

state = {"show":  True}

# Couple of callbacks for change and release
# The slider_id is not used since we are updating the bar and the text
def slider_on_change(_slider_id: int, data: float):
    """Slide changed"""
    update_widget(on_change_id, TextParam.Content, f"On Change value is {data}")
    update_widget(bar_id, ProgressBarParam.Value, data)

def slider_on_release(_slider_id: int, data: float):
    """Slider release"""
    update_widget(on_release_id, TextParam.Content, f"On Release value is {data}")


# The callbacks below allow you to change all of the parameters for a widget.
# They may or may not have frequent usage but it makes the gui very flexible
# when the data that may be loaded effects the placement, sizes, etc. used.
# These callbacks also demonstrate the usage of the widget parameters and
# are used in the testing of the code to make sure it behaves as expected.
def change_width(_btn_id):
    """Button callback"""
    update_widget(sl_id, SliderParam.Width, 200.0)
    # change bar too
    update_widget(bar_id, ProgressBarParam.Width, 200.0)


def change_height(_btn_id):
    """Button callback"""
    update_widget(sl_id, SliderParam.Height, 30.0)


def change_min(_btn_id):
    """Button callback"""
    update_widget(sl_id, SliderParam.Min, 50.0)
    update_widget(min_txt_id, TextParam.Content, "50")


def change_max(_btn_id):
    """Button callback"""
    update_widget(sl_id, SliderParam.Max, 150.0)
    update_widget(max_txt_id, TextParam.Content, "150")


def change_step(_btn_id):
    """Button callback"""
    update_widget(sl_id, SliderParam.Step, 5.0)


def change_value(_btn_id):
    """Button callback"""
    update_widget(sl_id, SliderParam.Value, 100.0)


def toggle_show(_btn_id):
    """Button callback"""
    state["show"] = not state["show"]
    update_widget(sl_id, SliderParam.Show, state["show"])



# Add a slider style for the colors
sl_style = add_slider_style(
                    rail_color=Color.GREEN,
                    rail_color_hovered=Color.GREEN_YELLOW,
                    handle_color=Color.LIGHT_GREEN,
                    rail_width=10.0,
                    rail_border_radius=[8.0],
                    handle_rectangle_width=20,
                    handle_rectangle_border_radius=[5.0],
                    handle_border_width=2.0,
                    handle_border_color=Color.DARK_GREEN,)


# Add the window
with Window(
        title="Slider Demo",
        size=(600, 600),
        position=(100, 25)):

    # Add the column and center the widgets in it.
    with Column(
            align_center=True,
            fill=True,
            spacing=10):

        # Add some instructions.
        add_text(content="The below progress bar is showing the change")

        # Equate the bar to get an id for the callback use.
        bar_id = add_progress_bar(
                        min=0.0,
                        max=100.0,
                        value=50.0,
                        width=300.0)

        # Add a slide to change the value with two callbacks
        add_text(content="Use the Slider below to Change the Values")
        sl_id = add_slider(
                        min=0.0, max=100.0,
                        step=0.5, value=50.0,
                        width=300.0,
                        on_change=slider_on_change,
                        on_release=slider_on_release,
                        style_id=sl_style)

        # Add some value at beginning and end
        with Row(
                width=320.0,
                padding=[5.0]):

            min_txt_id = add_text(content="0")

            add_space(width=220.0)

            max_txt_id = add_text(content="100")

        # Add a couple of text widget to display some data
        on_change_id = add_text(content="On Change value is 0")

        on_release_id = add_text(content="On Release value is 0")

        # Add a little extra spacing
        add_space(height=10)

        # add_rows for buttons
        with Row(spacing=10):

            add_button(
                label="Press Me to Change Width",
                on_press=change_width)

            add_button(
                label="Press Me to Change Height",
                on_press=change_height)

        with Row(spacing=10):

            add_button(
                label="Press Me to Change Min",
                on_press=change_min)

            add_button(
                label="Press Me to Change Max",
                on_press=change_max)

        with Row(spacing=10):

            add_button(
                label="Press Me to Change Step",
                on_press=change_step)

            add_button(
                label="Press Me to Change Value",
                on_press=change_value)


        add_button(
            label="Press Me to Toggle Show",
            on_press=toggle_show)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
