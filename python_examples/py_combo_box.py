#!/usr/bin/env python3
"""
Picklist use demo
"""

from icedpygui import (
    Window,
    Column,
    Container,
    start_session,
    add_space,
    add_combobox,
    add_text)

# The data returns the item selected and can be named anything.
# The update items uses the text widget id and the "content" parameter
# to update.  The value is what you want the content parameter to equal.
def picked_item(cb_id: int, data: str):
    """ComboBox callback"""
    print(f"cb_id = {cb_id} data = {data}")


def picked_item_with_user_data(cb_id: int, data: str, user_data: any):
    """Picklist callback"""
    print(f"pl_id = {cb_id} data = {data}, user_data = {user_data}")


# Add window
with Window(title="Pick List Demo", size=(400, 400), center=True):
    with Container(fill=True, align_center=True):

        # add column to hold multiple widgets
        with Column(spacing=20, height_fill=True):

            add_space(height=50.0)

            # A ComboBox requires that the options you want to select be in a list.
            options = ["Hello", "World"]

            add_text(content="The ComboBox is good for long " +
                     "lists when you begin typing, the " +
                     "box will filter the selection")

            txt_id = add_text(content="Selected Word")

            # A combobox is added.
            # Unless the container that you put the
            # combobox into has a defined width,
            # you must define a width
            add_combobox(
                options=options,
                placeholder="Choose a Word...",
                on_select=picked_item,
                width=150)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
