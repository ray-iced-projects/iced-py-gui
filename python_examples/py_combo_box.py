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
    print(f"from py: cb_id = {cb_id} data = {data}")


def picked_item_with_user_data(cb_id: int, data: str, user_data: any):
    """Picklist callback"""
    print(f"pl_id = {cb_id} data = {data}, user_data = {user_data}")


# Add window
with Window(title="Pick List Demo", size=(400, 500), center=True):
    with Container(fill=True, align_center=True):

        # add column to hold multiple widgets
        with Column(spacing=20, height_fill=True):

            add_space(height=50.0)

            # A ComboBox requires that the options you want to select be in a list.
            # The items will be converted to  a str when processed.
            # When given a long list, as one types in the wanted values, the combobox
            # will select those items to which contains typed in text to shorten the list.
            options = [
                "Apple", "Apricot", "Avocado",
                "Banana", "Blueberry", "Blackberry",
                "Cherry", "Cranberry", "Cantaloupe",
                "Date", "Dragonfruit",
                "Elderberry",
                "Fig",
                "Grape", "Grapefruit", "Guava",
                "Honeydew",
                "Kiwi",
                "Lemon", "Lime",
                "Mango", "Mulberry",
                "Nectarine",
                "Orange",
                "Papaya", "Peach", "Pear", "Pineapple", "Plum",
                "Raspberry",
                "Strawberry",
                "Tangerine", "Tangor",
                "Ugli",
                "Watermelon"
            ]

            add_text(content="The ComboBox is good for long " +
                     "lists when you begin typing, the " +
                     "box will filter the selection")

            # A combobox is added.
            # Unless the container that you put the
            # combobox into has a defined width,
            # you must define a width
            add_combobox(
                options=options,
                placeholder="Choose a Fruit...",
                on_select=picked_item,
                text_ellipsis_middle=True,
                width=150)

            # The below long text options show the ellipsis effect
            add_text(content="Ellipsis effect for the dropdown menu")
            options = [
                "Extraordinarily Long Apple Description",
                "Incredibly Long Blueberry Description",
                "Phenomenally Long Dragonfruit Description",
                "Unbelievably Long Watermelon Description"
            ]

            add_combobox(
                options=options,
                placeholder="Choose...",
                on_select=picked_item,
                text_ellipsis_middle=True,
                width=150)  # Narrow width to force truncation
            

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
