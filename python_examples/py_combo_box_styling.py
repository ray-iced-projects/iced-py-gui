#!/usr/bin/env python3
"""
Picklist use demo
"""

from icedpygui import (
    Window,
    Column,
    Color,
    Container,
    start_session,
    add_combobox,
    add_combobox_style,
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


palette_style_id = add_combobox_style(palette_base_color=Color.DARK_GREEN)
selected_style_id = add_combobox_style(selected_bkg_color=Color.LIGHT_YELLOW,
                                       selected_text_color=Color.LIGHT_GREEN)
shadow_style_id = add_combobox_style(shadow_color=Color.ALICE_BLUE,
                                     shadow_blur_radius=5.0,
                                     shadow_offset_xy=[10, 10])

# Add window
with Window(title="Pick List Demo", size=(500, 600), center=True):
    with Container(fill=True, align_center=True):

        # add column to hold multiple widgets
        with Column(spacing=20, height_fill=True):

            # A ComboBox requires that the options you want to select be in a list.
            options = ["Hello", "World"]

            add_text(content="The styling using the palette_base.. will changes:\n" +
                     "background: weak variation from base\n" +
                     "border radius: strong variation from base\n" +
                     "text color: weak variation from base")

            txt_id = add_text(content="Selected Word")

            # Unless the container that you put the combobox into has a defined width,
            # you must define a width
            add_text(content="Default styling")
            add_combobox(
                options=options,
                placeholder="Choose a Word...",
                on_select=picked_item,
                width=150,
                )

            add_text(content="palette_styling")
            add_combobox(
                options=options,
                placeholder="Choose a Word...",
                on_select=picked_item,
                width=150,
                style_id=palette_style_id,
                )

            add_text(content="Selected item styling")
            add_combobox(
                options=options,
                placeholder="Choose a Word...",
                on_select=picked_item,
                width=150,
                style_id=selected_style_id,
                )

            add_text(content="Shadowm styling")
            add_combobox(
                options=options,
                placeholder="Choose a Word...",
                on_select=picked_item,
                width=150,
                style_id=shadow_style_id,
                )


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
