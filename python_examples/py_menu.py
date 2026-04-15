#!/usr/bin/env python3
"""
Menu use demo

Select menu items to show actions.
The menu can be placed anywhere you like.
"""
from operator import is_none

from icedpygui import Window, Column, Container, start_session, \
    Menu, MenuBarItem, MenuParam, MenuBarItemParam,  add_button, add_text, \
    add_separator, SeparatorType, update_widget


state = {"bar_testing_id": 0,
         "item_testing_id": 0,
         "bar_width": None,
         "bar_height": None,
         "bar_spacing": 10.0,
         "bar_padding": None,
         "bar_scroll_speed_line": None,
         "bar_scroll_speed_pixel": None,
         "item_spacing": 5.0,
         "item_padding": None,
         "item_width": 200.0,
         "item_offset": None,
         }

def on_press(_id, name: str):
    """Button callback"""
    print(f"selected: {name}")


def toggle_spacing(_id):
    """Button callback"""
    if state["item_spacing"] == 5.0:
        state["item_spacing"] = 10.0
    else:
        state["item_spacing"] = 5.0

    update_widget(state["item_testing_id"], MenuBarItemParam.Spacing, state["item_spacing"])

def toggle_padding(_id):
    """Button callback"""
    if is_none(state["item_padding"]):
        state["item_padding"] = [20.0]
    else:
        state["item_padding"] = None

    update_widget(state["item_testing_id"], MenuBarItemParam.Padding, state["item_padding"])

def toggle_width(_id):
    """Button callback"""
    if state["item_width"] == 200.0:
        state["item_width"] = 250.0
    else:
        state["item_width"] = 200.0

    update_widget(state["item_testing_id"], MenuBarItemParam.Width, state["item_width"])

def toggle_offset(_id):
    """Button callback"""
    if is_none(state["item_offset"]):
        state["item_offset"] = 50.0
    else:
        state["item_offset"] = None

    update_widget(state["item_testing_id"], MenuBarItemParam.Offset, state["item_offset"])


def toggle_bar_spacing(_id):
    """Button callback"""
    if state["bar_spacing"] == 10.0:
        state["bar_spacing"] = 20.0
    else:
        state["bar_spacing"] = 10.0

    update_widget(state["bar_testing_id"], MenuParam.Spacing, state["bar_spacing"])

def toggle_bar_padding(_id):
    """Button callback"""
    if is_none(state["bar_padding"]):
        state["bar_padding"] = [20.0]
    else:
        state["bar_padding"] = None

    update_widget(state["bar_testing_id"], MenuParam.Padding, state["bar_padding"])

def toggle_bar_width(_id):
    """Button callback"""
    if is_none(state["bar_width"]):
        state["bar_width"] = 400.0
    else:
        state["bar_width"] = None

    update_widget(state["bar_testing_id"], MenuParam.Width, state["bar_width"])

def toggle_bar_height(_id):
    """Button callback"""
    if is_none(state["bar_height"]):
        state["bar_height"] = 50.0
    else:
        state["bar_height"] = None

    update_widget(state["bar_testing_id"], MenuParam.Height, state["bar_height"])

def toggle_scroll_speed_line(_id):
    """Button callback"""
    # The default value for bar_scroll_speed_line is 60.0
    # which is equivalent to one mouse wheel notch
    if is_none(state["bar_scroll_speed_line"]):
        state["bar_scroll_speed_line"] = 1.0
    else:
        state["bar_scroll_speed_line"] = None

    update_widget(state["bar_testing_id"], MenuParam.ScrollSpeedLine,
                  state["bar_scroll_speed_line"])

def toggle_scroll_speed_pixel(_id):
    """Button callback"""
    # The scroll_speed_pixels is only for Trackpads and high-precision scroll
    # wheels that report exact pixel deltas.
    # Laptops with trackpads typically produce this. The pixel multiplier
    # (default 1.0) is usually used but you can change if you want.
    if is_none(state["bar_scroll_speed_pixel"]):
        state["bar_scroll_speed_pixel"] = 2.0
    else:
        state["bar_scroll_speed_pixel"] = None
    print(state["bar_scroll_speed_pixel"])
    update_widget(state["bar_testing_id"], MenuParam.ScrollSpeedPixel,
                  state["bar_scroll_speed_pixel"])



# Add a window
with Window(title="Menu", center=True, size=[500, 400]):

    with Container(padding=[20.0], fill=True):
        with Column(spacing=20):
            add_text(content="Select menu items to see the callback printing")
            add_text(content="Select the Testing to see the effect of setting parameters")

            with Menu(spacing=10.0) as state["bar_testing_id"]:

                # Each MenuBarItem groups a bar widget (first child) with its dropdown items
                with MenuBarItem(width=75.0, spacing=5.0):

                    add_text(content="File") # bar item

                    # dropdown items
                    add_button(label="New",
                            if_menu_btn=True,
                            on_press=on_press,
                            user_data="New")

                    add_button(label="Open",
                            if_menu_btn=True,
                            on_press=on_press,
                            user_data="Open")

                    add_button(label="Save",
                            if_menu_btn=True,
                            on_press=on_press,
                            user_data="Save")

                with MenuBarItem(width=75.0, spacing=5.0, padding=[5.0]):

                    add_text(content="Edit") # bar item

                    # dropdown items
                    add_button(label="Cut",
                            if_menu_btn=True,
                            on_press=on_press,
                            user_data="Cut")

                    add_button(label="Copy",
                            if_menu_btn=True,
                            on_press=on_press,
                            user_data="Copy")

                    add_separator(separator_type=SeparatorType.Dot,
                                dot_radius=3.0,
                                dot_count=10,
                                spacing=3.0,)

                    add_button(label="Paste",
                            if_menu_btn=True,
                            on_press=on_press,
                            user_data="Paste")

                with MenuBarItem():

                    add_text(content="Help") # bar item

                    # dropdown items
                    add_button(label="About",
                            if_menu_btn=True,
                            on_press=on_press,
                            user_data="About")

                with MenuBarItem(width=200.0, spacing=state["item_spacing"],
                                 padding=state["item_padding"],
                                 offset=state["item_offset"]) as state["item_testing_id"]:

                    add_text(content="Item Testing") # bar item

                    # dropdown items
                    add_button(label="Press to Toggle Spacing",
                            if_menu_btn=True,
                            on_press=toggle_spacing)

                    add_button(label="Press to Toggle Padding",
                            if_menu_btn=True,
                            on_press=toggle_padding)

                    add_button(label="Press to Toggle Width",
                            if_menu_btn=True,
                            on_press=toggle_width)

                    add_button(label="Press to Toggle Offset",
                            if_menu_btn=True,
                            on_press=toggle_offset)

                with MenuBarItem(width=state["bar_width"],
                                 spacing=state["bar_spacing"]):

                    add_text(content="Bar Testing") # bar item

                    # dropdown items
                    add_button(label="Press to Toggle Spacing",
                            if_menu_btn=True,
                            on_press=toggle_bar_spacing)

                    add_button(label="Press to Toggle Padding",
                            if_menu_btn=True,
                            on_press=toggle_bar_padding)

                    add_button(label="Press to Toggle Width",
                            if_menu_btn=True,
                            on_press=toggle_bar_width)

                    add_button(label="Press to Toggle Height",
                            if_menu_btn=True,
                            on_press=toggle_bar_height)

                    add_button(label="Press to Toggle Scroll Speed Line",
                            if_menu_btn=True,
                            on_press=toggle_scroll_speed_line)

                    add_button(label="Press to Toggle Scroll Speed Pixel",
                            if_menu_btn=True,
                            on_press=toggle_scroll_speed_pixel)

                    for _ in range(10):
                        add_text(content="Just text to show scrolling")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
