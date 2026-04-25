#!/usr/bin/env python3
"""
Menu use demo

Menu uses add_menu, add_menu_bar_item, and add_menu_style

Add a menu widget.

A horizontal menu bar with dropdown menus.  Each top-level bar
widget and its dropdown items are grouped inside a ``MenuBarItem``
context manager.  The first child of each ``MenuBarItem`` is
rendered on the bar; the remaining children become dropdown items.

Parameters
----------
window_id : str
    Sets the window this menu belongs to.
container_id : str
    Sets the unique string identifier for the menu.
parent_id : str, Optional
    Sets the parent container ID.  Defaults to the window itself.
items_close_on_click_global : bool, Optional
    Global default for closing dropdowns on item click.  Used
    when neither the per-Item nor per-dropdown value is set.
    Defaults to ``False``.
items_close_on_background_click_global : bool, Optional
    Global default for closing dropdowns on background click.
    Used when neither the per-Item nor per-dropdown value is
    set.  Defaults to ``False``.
height : float, Optional
    Sets the fixed height of the menu bar in logical pixels.
padding : list of float, Optional
    Sets the padding inside the menu bar as ``[all]``,
    ``[vertical, horizontal]``, or
    ``[top, right, bottom, left]``.
spacing : float, Optional
    Sets the horizontal spacing between bar items.
width : float, Optional
    Sets the fixed width of the menu bar in logical pixels.
width_fill : bool, Optional
    Whether to fill the width of a container holding the menu bar.
close_on_bar_item_click : bool, Optional
    Whether the dropdown closes when a menu bar item is clicked.
close_on_bar_background_click : bool, Optional
    Whether the dropdown closes when clicking outside the menu bar.
cursor_bounds_margin: float, Optional
    Sets the margin where, if the cursor moves outside this area,
    the menu will be closed.
scroll_speed_line: float, Optional
    The speed of the scrolling when items are out of the screen or container.
    The default is 60 lines which is 1 notch of the mouse wheel.
scroll_speed_pixel: float, Optional
    The scroll_speed_pixels is only for Trackpads and high-precision scroll
    wheels that report exact pixel deltas.
    Laptops with trackpads typically produce this. The pixel multiplier
    (default 1.0) is usually used but you can change if you want.
on_select : callable, Optional
    Sets the callback method to invoke when a menu item is
    selected.
style_id : int, Optional
    Sets the ID of a custom style created with
    ``add_menu_style``.
style_std_primary : bool, Optional
    Whether to use the primary standard style.
show : bool, default True
    Whether the menu is visible.
user_data : Any, Optional
    Sets arbitrary data forwarded to callbacks.
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used
    for the gen_id parameter.

Returns
-------
int
    The numeric widget ID of the newly created menu.

Add a menu bar item container.

Groups a bar-level widget with its dropdown items.  The first
child added inside the ``MenuBarItem`` is rendered on the menu
bar; all subsequent children become dropdown items.

Parameters
----------
window_id : str
    Sets the window this menu bar item belongs to.
container_id : str
    Sets the unique string identifier for the menu bar item.
parent_id : str, Optional
    Sets the parent container ID.  Defaults to the window itself.
width : float, Optional
    Sets the width of this dropdown menu in logical pixels.
spacing : float, Optional
    Sets the vertical spacing between dropdown items.
offset : float, Optional
    Sets the horizontal offset of the dropdown relative to its
    bar item.
paddings : list of float, Optional
    Sets the padding inside this dropdown as ``[all]``,
    ``[vertical, horizontal]``, or
    ``[top, right, bottom, left]``.
close_on_item_click : bool, Optional
    Per-dropdown override for closing when an item is clicked.
    Overrides the global default set on the Menu.
close_on_background_click : bool, Optional
    Per-dropdown override for closing when the background is
    clicked.  Overrides the global default set on the Menu.
show : bool, default True
    Whether the menu bar item is visible.
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used
    for the gen_id parameter.

Returns
-------
int
    The numeric widget ID of the newly created menu bar item.

Add styling to a menu.

Creates a custom style that can be applied to a menu via its
``style_id`` parameter.  The style has three sections: **bar**
(the horizontal menu bar), **menu** (the dropdown panels), and
**path** (the highlighted trail from bar item to open menu).

Parameters
----------
bar_background_color : Color, Optional
    Sets the bar background color.
bar_background_rgba : list[float, 4], Optional
    Sets the bar background color in rgba format.
bar_background_alpha : float, Optional
    Sets the alpha transparency for the bar background color.
bar_border_color : Color, Optional
    Sets the bar border color.
bar_border_rgba : list[float, 4], Optional
    Sets the bar border color in rgba format.
bar_border_alpha : float, Optional
    Sets the alpha transparency for the bar border color.
bar_border_radius : list of float, Optional
    Sets the bar border radius, ``[float]`` = all corners,
    ``[float, 4]`` = [top-left, top-right, bottom-right,
    bottom-left].
bar_border_width : float, Optional
    Sets the bar border width.
bar_shadow_color : Color, Optional
    Sets the bar shadow color.
bar_shadow_rgba : list[float, 4], Optional
    Sets the bar shadow color in rgba format.
bar_shadow_alpha : float, Optional
    Sets the alpha transparency for the bar shadow color.
bar_shadow_offset_xy : list[float, 2], Optional
    Sets the bar shadow offset as [x, y].
bar_shadow_blur_radius : float, Optional
    Sets the bar shadow blur radius.
menu_background_color : Color, Optional
    Sets the dropdown menu background color.
menu_background_rgba : list[float, 4], Optional
    Sets the dropdown menu background color in rgba format.
menu_background_alpha : float, Optional
    Sets the alpha transparency for the dropdown menu
    background color.
menu_border_color : Color, Optional
    Sets the dropdown menu border color.
menu_border_rgba : list[float, 4], Optional
    Sets the dropdown menu border color in rgba format.
menu_border_alpha : float, Optional
    Sets the alpha transparency for the dropdown menu border
    color.
menu_border_radius : list of float, Optional
    Sets the dropdown menu border radius, ``[float]`` = all
    corners, ``[float, 4]`` = [top-left, top-right,
    bottom-right, bottom-left].
menu_border_width : float, Optional
    Sets the dropdown menu border width.
menu_shadow_color : Color, Optional
    Sets the dropdown menu shadow color.
menu_shadow_rgba : list[float, 4], Optional
    Sets the dropdown menu shadow color in rgba format.
menu_shadow_alpha : float, Optional
    Sets the alpha transparency for the dropdown menu shadow
    color.
menu_shadow_offset_xy : list[float, 2], Optional
    Sets the dropdown menu shadow offset as [x, y].
menu_shadow_blur_radius : float, Optional
    Sets the dropdown menu shadow blur radius.
path_background_color : Color, Optional
    Sets the path highlight background color.
path_background_rgba : list[float, 4], Optional
    Sets the path highlight background color in rgba format.
path_background_alpha : float, Optional
    Sets the alpha transparency for the path background color.
path_border_color : Color, Optional
    Sets the path highlight border color.
path_border_rgba : list[float, 4], Optional
    Sets the path highlight border color in rgba format.
path_border_alpha : float, Optional
    Sets the alpha transparency for the path border color.
path_border_radius : list of float, Optional
    Sets the path highlight border radius, ``[float]`` = all
    corners, ``[float, 4]`` = [top-left, top-right,
    bottom-right, bottom-left].
path_border_width : float, Optional
    Sets the path highlight border width.
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used
    for the gen_id parameter.

Returns
-------
int
    The numeric style ID to pass as ``style_id`` to
    ``add_menu``.
"""
from operator import is_none

from icedpygui import Window, Column, Container, start_session, \
    Menu, MenuBarItem, MenuParam, MenuBarItemParam,  add_button, add_text, \
    add_separator, update_widget


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

INSTR = "Select menu items to see the callback printing\n\n\
Select the Testing to see the effect of setting parameters, \
it's best if you toggle back to the first setting(i.e. toggle twice) on each.\n\n\
For the scrolling test, click, then scroll see to it slow down, then click again to cancel.\n\n\
For the pixel scroll, you'll not see an effect unless you are using a laptop mouse pad or other device \
with high resolution mouse scrolling."

# Add a window
with Window(title="Menu", center=True, size=[600, 600]):

    with Container(padding=[20.0], fill=True):
        with Column(spacing=20):
            add_text(content=INSTR)

            with Menu(spacing=10.0) as state["bar_testing_id"]:

                # First item of the MenuBarItem is the bar item followed by the dropdown items
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

                    add_separator(dot=True,
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
