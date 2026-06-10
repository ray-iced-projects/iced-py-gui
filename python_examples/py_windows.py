#!/usr/bin/env python3
"""
Window use demo

Adds a window to the application.

"""

from icedpygui import (
    Window,
    Column,
    Container,
    add_button,
    add_space,
    add_text,
    add_text_input,
    add_toggler,
    start_session,
    add_event_window,
    update_widget,
    generate_id,
    WindowParam,
    WindowLevel,
    TextParam,
    WindowTheme)


popup_id = generate_id()
wnd2 = generate_id()
wnd4 = generate_id()

wnd1_pos = [100, 25]
wnd2_pos = [550, 25]
popup_pos = [600.0, 300.0]
wnd4_pos = [1000.0, 25.0]


# ****************Functions for modifying the window 4 parameters*************

def toggle_debug(_tog_id: int, value: bool):
    """Debug requires a bool value therefore, for convenience, a toggler is used"""
    update_widget(wnd4, WindowParam.Debug, value)


def toggle_decorations(_btn_id: int):
    """The decorator is just toggled therefore a button works.  Supply the window id."""
    update_widget(wnd4, WindowParam.Decorations, wnd4)


def toggle_resize(_btn_id: int, value: bool):
    """The resize of the window requires a list [width, height]"""
    if value:
        update_widget(wnd4, WindowParam.Size, [300.0, 400.0])
    else:
        update_widget(wnd4, WindowParam.Size, [300.0, 600.0])


def toggle_level(_tog_id: int, value: bool):
    """# The level of the window, move it over another window to see the effect"""
    if value:
        update_widget(wnd4, WindowParam.Level, WindowLevel.AlwaysOnBottom)
    else:
        update_widget(wnd4, WindowParam.Level, WindowLevel.Normal)


def toggle_move_to(_tog_id: int, value: bool):
    """# Move the window to a new position, required a list[pos_x, pos_y]"""
    if value:
        update_widget(wnd4, WindowParam.Position, [900.0, 25.0])
    else:
        update_widget(wnd4, WindowParam.Position, [1000.0, 25.0])


def toggle_theme(_tog_id: int, value: bool):
    """Theme toggle"""
    if value:
        update_widget(wnd4, WindowParam.Theme, WindowTheme.Light)
    else:
        update_widget(wnd4, WindowParam.Theme, WindowTheme.Dark)

# ****************Functions for changes and events in window 1*****************

def change_scale(_input_id: int, value: str):
    """Since the input value is a string, need to convert to  a float"""
    update_widget(wnd1, WindowParam.ScaleFactor, float(value))


def show_window_2(_btn_id: int, window_id: int):
    """The user data is a window_id"""
    update_widget(window_id, WindowParam.Hidden, False)
    update_widget(s_h_text_id, TextParam.Content, f"Window with id {window_id} is shown")


def close_window(_btn_id: int, window_id: int):
    """The user data is a window_id"""
    update_widget(window_id, WindowParam.Hidden, True)
    update_widget(s_h_text_id, TextParam.Content, f"Window with id {window_id} is closed")

# Since the event name is return, normally you would probably just create one def
#  and match the event name
def event_on_closed(wnd_id: int, _event_name: str):
    """Window events return a window_id and event name at the minimum."""
    update_widget(event_text_id, TextParam.Content, f"You closed window width id {wnd_id}")


def event_on_move(wnd_id: int, _event_name: str, position: dict):
    """Window OnMove callback"""
    update_widget(event_text_id, TextParam.Content,\
        f"Window with id {wnd_id} \nhas position {position}")


def event_on_resize(wnd_id: int, _event_name: str, size: dict):
    """Window OnResize"""
    update_widget(event_resize_id, TextParam.Content,\
                  f"Window with id {wnd_id} \nhas size of {size}")


def event_focused(wnd_id, _event_name: str):
    """Window OnFocused callback"""
    update_widget(event_focused_id, TextParam.Content,\
                  f"Window with id {wnd_id} has the focus")


def event_unfocused(wnd_id, _event_name: str):
    """Window OnUnFocused callback"""
    update_widget(event_unfocused_id, TextParam.Content,\
                  f"Window with id {wnd_id} was unfocused")

# *******************add functions for close requested ******************************

def event_on_close_requested(wnd_id: int, _event_name: str):
    """This responds to the close request event"""

    if wnd_id == wnd2:
        # show window to acknowledge close or not
        update_widget(wnd2, WindowParam.Hidden, False)
        update_widget(popup_id, WindowParam.Hidden, False)

def close_window2_and_popup(_btn_id: int):
    """Close the requested popup window"""
    update_widget(popup_id, WindowParam.Hidden, True)
    update_widget(wnd2, WindowParam.Hidden, True)


def close_window_canceled(_btn_id: int):
    """Window CloseCanceled callback"""
    update_widget(popup_id, WindowParam.Hidden, True)

# ****************************Add the event widget for the windows*******************************
add_event_window(
        enabled=True,
        on_closed=event_on_closed,
        on_moved=event_on_move,
        on_resized=event_on_resize,
        on_close_requested=event_on_close_requested,
        on_focused=event_focused,
        on_unfocused=event_unfocused)

# ******************Add the 1st window***************************
with Window(
    title="Window 1",
    size=(400.0, 600.0),
    position=wnd1_pos) as wnd1:

    # add a container to center things
    with Container(fill=True):

        # Add a column for multiple widgets
        with Column(spacing=20.0):

            # Add some text
            add_text(content="Input scale factor")

            # add the input widget
            add_text_input(
                    width=200.0,
                    placeholder="scale factor (float)",
                    on_submit=change_scale)

            # Add show hide text
            s_h_text_id = add_text(content="Window 2 is closed")

            # Add event text
            event_text_id = add_text(content="This will change when an event occurs")

            event_resize_id = add_text(content="This will change when resized event occurs")

            event_close_request_id = add_text(content="You have no close requests")

            event_focused_id = add_text(content="No window has the focus")

            event_unfocused_id = add_text(content="No window has the focus")

            # add a button to show the 2nd window
            add_button(
                    label="Show Window 2",
                    on_press=show_window_2,
                    user_data=wnd2)

            add_button(
                    label="Show Close request window",
                    on_press=show_window_2,
                    user_data=popup_id)

# ************Add the 2nd window ****************************
# To get a close request from this window, exit_on_close must be set to False.
# Windows default to True.
with Window(
        title="Window 2",
        size=(400.0, 400.0),
        position=wnd2_pos,
        hidden=True,
        exit_on_close_request=False,
        gen_id=wnd2):

    with Container(align_center=True, width_fill=True, height_fill=True):
        add_text(content="When you try closing the window, a popup will will \
            appear and then you can close me")


# **************Add close request window or popup window***********************
# Note the window is closed, acting like a modal in this case.
with Window(
        title="Close Requested",
        size=(300.0, 300.0),
        position=popup_pos,
        hidden=True,
        level=WindowLevel.AlwaysOnTop,
        exit_on_close_request=False,
        gen_id=popup_id):

    with Container(width_fill=True, height_fill=True):

        with Column(spacing=20.0):

            add_text(content="With a little bit more programming,\
                you could place this popup window anyplace on the screen.")

            add_space(height=30.0)

            add_button(
                    label="Close Window 2",
                    on_press=close_window2_and_popup)

            add_button(
                    label="Cancel Window 2 Closing",
                    on_press=close_window_canceled)


# ************Add the 4th window ****************************
# This window is for changing the window parameters
with Window(title="Window 4", size=(300.0, 600.0),
            position=wnd4_pos, gen_id=wnd4):

    with Column(spacing=20.0):

        add_toggler(
                label="Toggle Debug",
                toggled=toggle_debug)

        add_button(
                label="Toggle Decorations",
                on_press=toggle_decorations)

        add_toggler(
                label="Toggle Window Resize",
                toggled=toggle_resize)

        add_toggler(
                label="Toggle Position",
                toggled=toggle_move_to)

        add_toggler(
            label="Toggle Level",
            toggled=toggle_level)

        add_toggler(
                label="Toggle Theme",
                toggled=toggle_theme)

start_session()
