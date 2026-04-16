#!/usr/bin/env python3
"""
Picklist use demo

PickList allows one to select an item from a dropdown menu
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
