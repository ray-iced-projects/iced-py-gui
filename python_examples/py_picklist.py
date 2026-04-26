#!/usr/bin/env python3
"""
Picklist use demo


Add a pick list widget.

A dropdown pick list that lets the user select one option
from a list of choices.

Parameters
----------
parent_id : str
    Sets the parent container ID that this pick list belongs to.
options : list of str
    Sets the list of selectable options.
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.
on_select : callable, Optional
    Sets the Callback method to invoke when an option is selected.
width : float, Optional
    Sets the Fixed width in logical pixels.
width_fill : bool, default False
    Whether the pick list fills available width.
menu_height : float, Optional
    Sets the Fixed height of the dropdown menu in logical pixels.
menu_height_fill : bool, default False
    Whether the dropdown menu fills available height.
padding : list of float, Optional
    Sets the Padding as [all], [vertical, horizontal], or
    [top, right, bottom, left].
placeholder : str, Optional
    Sets the placeholder text shown when no option is selected.
selected : str, Optional
    Sets the currently selected option.
text_size : float, Optional
    Sets the Font size for the text.
text_line_height : float, Optional
    Sets the Line height for the text.
handle : PickListHandle, Optional
    Sets the handle type for the pick list.
arrow_size : float, Optional
    Sets the size of the arrow icon.
dynamic_closed : Arrow, Optional
    Sets the arrow icon when the pick list is closed.
dynamic_open : Arrow, Optional
    Sets the arrow icon when the pick list is open.
custom_static : Arrow, Optional
    Sets the static custom arrow icon.
style_id : int, Optional
    Sets the ID of a custom style created with ``add_pick_list_style``.
user_data : Any, Optional
    Sets the Arbitrary data forwarded to callbacks.
show : bool, default True
    Whether the pick list is visible.

Returns
-------
int
    The numeric widget ID of the newly created pick list.

*********************************************************
Add styling to a pick list.

Creates a custom style that can be applied to a pick list
via its ``style_id`` parameter.

Parameters
----------
background_color : Color, Optional
    Sets the background color using a predefined color variant.
background_color_alpha : float, Optional
    Sets the alpha of the Color.
background_rgba : list of float, Optional
    Sets the background color in rgba format as [r, g, b, a].
text_color : Color, Optional
    Sets the text color using a predefined color variant.
text_color_alpha : float, Optional
    Sets the alpha of the Color.
text_rgba : list of float, Optional
    Sets the text color in rgba format as [r, g, b, a].
handle_color : Color, Optional
    Sets the handle color using a predefined color variant.
handle_color_alpha : float, Optional
    Sets the alpha of the Color.
handle_rgba : list of float, Optional
    Sets the handle color in rgba format as [r, g, b, a].
placeholder_color : Color, Optional
    Sets the placeholder text color using a predefined color variant.
placeholder_color_alpha : float, Optional
    Sets the alpha of the Color.
placeholder_rgba : list of float, Optional
    Sets the placeholder text color in rgba format as [r, g, b, a].
border_color : Color, Optional
    Sets the border color using a predefined color variant.
border_color_alpha : float, Optional
    Sets the alpha of the Color.
border_rgba : list of float, Optional
    Sets the border color in rgba format as [r, g, b, a].
border_color_hovered : Color, Optional
    Sets the border color when hovered using a predefined color variant.
border_color_hovered_alpha : float, Optional
    Sets the alpha of the Color.
border_rgba_hovered : list of float, Optional
    Sets the border color when hovered in rgba format as [r, g, b, a].
border_radius : list of float, Optional
    Sets the radius of the corners as [all] or
    [top-left, top-right, bottom-right, bottom-left].
border_width : float, Optional
    Sets the border width in logical pixels.
menu_background_color : Color, Optional
    Sets the dropdown menu background color using a predefined color variant.
menu_background_color_alpha : float, Optional
    Sets the alpha of the dropdown menu background Color.
menu_background_rgba : list of float, Optional
    Sets the dropdown menu background color in rgba format as [r, g, b, a].
menu_text_color : Color, Optional
    Sets the dropdown menu text color using a predefined color variant.
menu_text_color_alpha : float, Optional
    Sets the alpha of the dropdown menu text Color.
menu_text_rgba : list of float, Optional
    Sets the dropdown menu text color in rgba format as [r, g, b, a].
menu_selected_text_color : Color, Optional
    Sets the dropdown menu selected option text color using a predefined color variant.
menu_selected_text_color_alpha : float, Optional
    Sets the alpha of the dropdown menu selected option text Color.
menu_selected_text_rgba : list of float, Optional
    Sets the dropdown menu selected option text color in rgba format as [r, g, b, a].
menu_selected_background_color : Color, Optional
    Sets the dropdown menu selected option background color using a predefined color variant.
menu_selected_background_color_alpha : float, Optional
    Sets the alpha of the dropdown menu selected option background Color.
menu_selected_background_rgba : list of float, Optional
    Sets the dropdown menu selected option background color in rgba format as [r, g, b, a].
menu_border_color : Color, Optional
    Sets the dropdown menu border color using a predefined color variant.
menu_border_color_alpha : float, Optional
    Sets the alpha of the dropdown menu border Color.
menu_border_rgba : list of float, Optional
    Sets the dropdown menu border color in rgba format as [r, g, b, a].
menu_border_radius : list of float, Optional
    Sets the dropdown menu border radius as [all] or
    [top-left, top-right, bottom-right, bottom-left].
menu_border_width : float, Optional
    Sets the dropdown menu border width in logical pixels.
gen_id : int, Optional
    Obtains an ID of a widget that have not been created, used for the gen_id parameter.

Returns
-------
int
    The numeric style ID to pass to a pick list's ``style_id``.
"""

from icedpygui import Window, Column, Container, Row, start_session, \
    add_space, add_pick_list, add_pick_list_style, add_text, PickListHandle, \
    Arrow, Color

# The data returns the item selected and can be named anything.
# The update items uses the text widget id and the "content" parameter
# to update.  The value is what you want the content parameter to equal.
def picked_item(pl_id: int, data: str):
    """Picklist callback"""
    print(f"pl_id = {pl_id} data = {data}")


def picked_item_with_user_data(pl_id: int, data: str, user_data: any):
    """Picklist callback"""
    print(f"pl_id = {pl_id} data = {data}, user_data = {user_data}")


# Add some styling
colors = add_pick_list_style(
                background_color=Color.DARK_OLIVE_GREEN,
                border_color=Color.ANTIQUE_WHITE,
                border_color_hovered=Color.BLUE,
                handle_color=Color.DARK_SEA_GREEN,
                placeholder_color=Color.WHITE,
                menu_text_color=Color.YELLOW, # The menu list text color
                text_color=Color.BLACK, # The selected item color
                border_width=1.0,)


# Add window
with Window(title="Pick List Demo", center=True):
    with Container(fill=True, align_center=True):

        # add column to hold multiple widgets
        with Column(spacing=20):

            add_space(height=50.0)

            # add a row for picklist and a button to change option
            with Row(spacing=10):

                # A PickList requires that the options you want to select be in a list.
                options_str = ["Hello", "World"]
                options_int = [1, 2, 3]
                options_float = [1.1, 2.2, 3.3, 4.4, 5.5]
                options_bool = [False, True]
                options_mixed = ["one", 1, 1.1, True]

                # A PickList is added and the handle is not defined which
                # will result in a down arrow used in the PickList box.
                add_pick_list(
                        options=options_str,
                        placeholder="Choose a Word...",
                        on_select=picked_item)

                add_text(content="Default No Arrow used")

            # add a row for picklist and a button to change option
            with Row(spacing=10):

                # Another PickList is added and the handle is defined as an arrow which
                # will result in a down arrow used again but the size can be changed.
                add_pick_list(
                        options=options_int,
                        placeholder="Choose a Integer...",
                        handle=PickListHandle.Arrow,
                        arrow_size=30.0,
                        on_select=picked_item)

                add_text(content="Arrow Style with down arrow sized to 30")

            # add another row for picklist
            with Row(spacing=10):

                # Another PickList is added and the handle is defined as Dynamic which
                # will result in an arrow used for the open and closed position.
                add_pick_list(
                        options=options_float,
                        placeholder="Choose a Float...",
                        handle=PickListHandle.Dynamic,
                        arrow_size=20.0, # the dynamic arrows can be sized too, if needed
                        dynamic_close=Arrow.ArrowLeft,
                        dynamic_open=Arrow.ArrowDown,
                        on_select=picked_item)

                add_text(content="Dynamic Arrow Style with left to down arrow of size to 20")

            # add another row for picklist
            with Row(spacing=10):

                # Another PickList is added and the handle is defined as an Custom which
                # will result in a down arrow used again but the size can be changed.
                add_pick_list(
                        options=options_bool,
                        placeholder="Choose a Bool...",
                        handle=PickListHandle.Static,
                        arrow_size=20.0, # the custom arrows can be sized too, if needed
                        custom_static=Arrow.ArrowNinezerodegDown,
                        on_select=picked_item)

                add_text(content="Arrow Custom Style sized to 20.")

            # add another row for picklist
            with Row(spacing=10):

                # Another PickList is added and the handle is defined as an Custom which
                # will result in a down arrow used again but the size can be changed.
                add_pick_list(
                        options=options_mixed,
                        arrow_size=25.0,
                        placeholder="Choose a Any...",
                        style_id=colors,
                        on_select=picked_item_with_user_data,
                        user_data="Some data")

                add_text(content="Background, Border, Handle, selected text and menu text styling")

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
