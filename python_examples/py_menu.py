#!/usr/bin/env python3
"""
BMenu use demo

Select menu items to show actions.
The menu can be placed anywhere you like.
"""
from icedpygui import Window, Container, start_session, \
    Menu, MenuBarItem,  add_button, add_text, \
    add_separator, SeparatorType


def on_press(_id, name: str):
    """Button callback"""
    print(f"selected: {name}")


def set_spacing(_id):
    """Button callback"""
    print("set spacing")


# Add a window
with Window(title="Menu", center=True, size=[400, 400]):

    with Container(padding=[20.0], fill=True):

        with Menu(bar_spacing=10.0, item_spacings=[5.0],
                  item_widths=[75.0], item_paddings=[5.0]):

            # Each MenuBarItem groups a bar widget (first child) with its dropdown items
            with MenuBarItem():
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

            with MenuBarItem():
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

            with MenuBarItem():
                add_text(content="Testing") # bar item

                # dropdown items
                add_button(label="Set Spacing",
                           if_menu_btn=True,
                           on_press=set_spacing)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
