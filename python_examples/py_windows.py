#!/usr/bin/env python3
"""
Window use demo
"""

from icedpygui import Window, Container, \
    add_text, add_event_window,\
    update_widget, generate_id, IpgWindowParam, IpgWindowLevel,\
    IpgWindowMode, IpgTextParam, IpgWindowTheme

popup_id = generate_id()
wnd2 = generate_id()
wnd4 = generate_id()


# ****************Functions for modifying the window 4 parameters*************

def toggle_debug(_tog_id: int, value: bool):
    """Debug requires a bool value therefore, for convenience, a toggler is used"""
    update_widget(wnd4, IpgWindowParam.Debug, value)


def toggle_decorations(_btn_id: int):
    """The decorator is just toggled therefore a button works.  Supply the window id."""
    update_widget(wnd4, IpgWindowParam.Decorations, wnd4)


def toggle_resize(_btn_id: int, value: bool):
    """The resize of the window requires a list [width, height]"""
    if value:
        update_widget(wnd4, IpgWindowParam.Size, [300.0, 400.0])
    else:
        update_widget(wnd4, IpgWindowParam.Size, [300.0, 600.0])


def toggle_level(_tog_id: int, value: bool):
    """# The level of the window, move it over another window to see the effect"""
    if value:
        update_widget(wnd4, IpgWindowParam.Level, IpgWindowLevel.AlwaysOnBottom)
    else:
        update_widget(wnd4, IpgWindowParam.Level, IpgWindowLevel.Normal)


def toggle_move_to(_tog_id: int, value: bool):
    """# Move the window to a new position, required a list[pos_x, pos_y]"""
    if value:
        update_widget(wnd4, IpgWindowParam.Position, [900.0, 25.0])
    else:
        update_widget(wnd4, IpgWindowParam.Position, [1000.0, 25.0])


def toggle_theme(_tog_id: int, value: bool):
    """Theme toggle"""
    if value:
        update_widget(wnd4, IpgWindowParam.Theme, IpgWindowTheme.Light)
    else:
        update_widget(wnd4, IpgWindowParam.Theme, IpgWindowTheme.Dark)

# ****************Functions for changes and events in window 1*****************

def change_scale(_input_id: int, value: str):
    """Since the input value is a string, need to convert to  a float"""
    update_widget(wnd1, IpgWindowParam.ScaleFactor, float(value))
    update_widget(wnd2, IpgWindowParam.ScaleFactor, float(value))


def show_window(_btn_id: int, window_id: int):
    """The user data is a window_id"""
    update_widget(window_id, IpgWindowParam.Mode, IpgWindowMode.Windowed)
    update_widget(s_h_text_id, IpgTextParam.Content, f"Window with id {window_id} is shown")


def close_window(_btn_id: int, window_id: int):
    """The user data is a window_id"""
    update_widget(wnd2, IpgWindowParam.Mode, IpgWindowMode.Closed)
    update_widget(s_h_text_id, IpgTextParam.Content, f"Window with id {window_id} is closed")

# Since the event name is return, normally you would probably just create one def
#  and match the event name
def event_on_closed(wnd_id: int, _event_name: str):
    """Window events return a window_id and event name at the minimum."""
    update_widget(event_text_id, IpgTextParam.Content, f"You closed window width id {wnd_id}")


def event_on_move(wnd_id: int, _event_name: str, position: dict):
    """Window OnMove callback"""
    update_widget(event_text_id, IpgTextParam.Content,\
        f"Window with id {wnd_id} \nhas position {position}")


def event_on_resize(wnd_id: int, _event_name: str, size: dict):
    """Window OnResize"""
    update_widget(event_resize_id, IpgTextParam.Content,\
                  f"Window with id {wnd_id} \nhas size of {size}")


def event_focused(wnd_id, _event_name: str):
    """Window OnFocused callback"""
    update_widget(event_focused_id, IpgTextParam.Content,\
                  f"Window with id {wnd_id} has the focus")


def event_unfocused(wnd_id, _event_name: str):
    """Window OnUnFocused callback"""
    update_widget(event_unfocused_id, IpgTextParam.Content,\
                  f"Window with id {wnd_id} was unfocused")


def event_on_file_hovered(wnd_id: int, _event_name: str, file_name: str):
    """Window OnFileHovered callback"""
    update_widget(event_file_hovered_id, IpgTextParam.Content,\
                  f"File, {file_name}, was hovered over window with id {wnd_id}")


def event_on_file_dropped(wnd_id: int, _event_name: str, file_name: str):
    """Window OnFileDropped callback"""
    update_widget(event_file_dropped_id, IpgTextParam.Content,\
       f"File, {file_name}, was dropped on window with id {wnd_id}")


def event_on_files_hover_left(wnd_id: int, _event_name: str):
    """Window OnFilesHoverLeft"""
    update_widget(event_file_hover_left_id, IpgTextParam.Content, f"File hover left window with id {wnd_id}.")


# *******************add functions for close requested ******************************

def event_on_close_requested(wnd_id: int, _event_name: str):
    """This responds to the close request event
    IMPORTANT: Once this event is used, you must also update any
    other window with a close statement since all window are now
    calling this event function.
    """
    if wnd_id == wnd2:
        # show window to acknowledge close or not
        update_widget(popup_id, IpgWindowParam.Mode, IpgWindowMode.Windowed)
    else:
        # If not the window of interest, close it.
        update_widget(wnd_id, IpgWindowParam.Mode, IpgWindowMode.Closed)


def close_window_using_popup(_btn_id: int, window_ids: list):
    """Close the requested and popup window"""
    update_widget(window_ids[0], IpgWindowParam.Mode, IpgWindowMode.Closed)
    update_widget(window_ids[1], IpgWindowParam.Mode, IpgWindowMode.Closed)


def close_window_canceled(btn_id, window_id: int):
    """Window CloseCanceled callback"""
    update_widget(window_id, IpgWindowParam.Mode, IpgWindowMode.Closed)

# ****************************Add the event widget for the windows*******************************
add_event_window(
        enabled=True,
        on_closed=event_on_closed,
        on_moved=event_on_move,
        on_resized=event_on_resize,
        on_close_requested=event_on_close_requested,
        on_focused=event_focused,
        on_unfocused=event_unfocused,
        on_file_hovered=event_on_file_hovered,
        on_file_dropped=event_on_file_dropped,
        on_files_hovered_left=event_on_files_hover_left)

# ******************Add the 1st window***************************
with Window(
    id="main1",
    title="Window 1",
    size=(400.0, 600.0), 
    position=(100, 25)) as wnd1:

    # add a container to center things
    with Container(
            window_id="main1", 
            id="cont",
            width_fill=True,
            height_fill=True):

        # Add a column for multiple widgets
        add_column(
                window_id="main1", 
                id="col", 
                parent_id="cont")

        # Add some text
        add_text(
                parent_id="col", 
                content="Input scale factor")

        # add the input widget
        add_text_input(
                parent_id="col",
                width=200.0,
                placeholder="scale factor (float)", 
                on_submit=change_scale)

        # Add show hide text
        s_h_text_id = add_text(
                            parent_id="col", 
                            content="Window 2 is closed")

        # Add event text
        event_text_id = add_text(
                                parent_id="col", 
                                content="This will change when an event occurs")

        event_resize_id = add_text(
                                parent_id="col", 
                                content="This will change when resized event occurs")

        event_close_request_id = add_text(
                                        parent_id="col", 
                                        content="You have no close requests")

        event_focused_id = add_text(
                                    parent_id="col", 
                                    content="No window has the focus")

        event_unfocused_id = add_text(
                                    parent_id="col", 
                                    content="No window has the focus")

        event_file_hovered_id = add_text(
                                        parent_id="col", 
                                        content="No file has been hovered")

        event_file_dropped_id = add_text(
                                        parent_id="col", 
                                        content="No file has been dropped")

        event_file_hover_left_id = add_text(
                                            parent_id="col", 
                                            content="No file hover has left")

        # add a button to show the 2nd window
        add_button(
                parent_id="col",
                label="Show Window",
                on_press=show_window,
                user_data=wnd2)

        add_button(
                parent_id="col",
                label="Show Close request window",
                on_press=show_window,
                user_data=popup_id)

# ************Add the 2nd window ****************************
# To get a close request from this window, exit_on_close must be set to False.
# Windows default to True.
add_window(
        id="main2", 
        title="Window 2",
    size=(400.0, 400.0),  
        position=(600, 25),
        mode=IpgWindowMode.Closed,
        exit_on_close=False,
        gen_id=wnd2)

add_container(
        window_id="main2", 
        id="cont")

add_button(
        parent_id="cont", 
        label="Hide Window",
        on_press=close_window,
        user_data=wnd2)


# **************Add close request window ***********************
# Note the window is closed, acting like a modal in this case.
add_window(
        id="close_request",
        title="Close Requested",
    size=(300.0, 300.0),
        pos_centered=True,
        mode=IpgWindowMode.Closed,
        gen_id=popup_id)

add_container(
        window_id="close_request",
        id="cont",
        width_fill=True, 
        height_fill=True)

add_column(
        window_id="close_request",
        id="col",
        parent_id="cont")

add_text(
        parent_id="col", 
        content="With a little bit more programming, you could place this popup window anyplace on the screen.")

add_space(
        parent_id="col", 
        height=30.0)

add_button(
        parent_id="col", 
        label="Close Window 2", 
        on_press=close_window_using_popup,
        user_data=[wnd2, popup_id])

add_button(
        parent_id="col", 
        label="Cancel Window 2 Closing", 
        on_press=close_window_canceled,
        user_data=popup_id)


# ************Add the 4th window ****************************
# This window is for changing the window parameters
add_window(
        id="main4", 
        title="Window 4",
    size=(300.0, 600.0),  
        position=(1000.0, 25.0),
        gen_id=wnd4)

add_column(
        window_id="main4", 
        id="col")

add_toggler(
        parent_id="col", 
        label="Toggle Debug",
        toggled=toggle_debug)

add_button(
        parent_id="col", 
        label="Toggle Decorations",
        on_press=toggle_decorations)

add_toggler(
        parent_id="col", 
        label="Toggle Window Resize",
        toggled=toggle_resize)

add_toggler(
        parent_id="col", 
        label="Toggle Position",
        toggled=toggle_move_to)

add_toggler(
    parent_id="col", 
    label="Toggle Level",
    toggled=toggle_level)

add_toggler(
        parent_id="col", 
        label="Toggle Theme",
        toggled=toggle_theme)

start_session()
