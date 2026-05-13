"""
Simple Menu
"""
from icedpygui import (
    Window,
    Column,
    Container,
    start_session,
    Menu,
    MenuBarItem,
    add_button,
    add_text,
    ButtonStyleStd)

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



# Add a window
with Window(title="Menu", center=True, size=[600, 600]):

    with Container(padding=[20.0], fill=True):
        with Column(spacing=20):
            with Menu(spacing=10.0) as state["bar_testing_id"]:

                # First item of the MenuBarItem is the bar item followed by the dropdown items
                with MenuBarItem(spacing=5.0):

                    add_text(content="File") # bar item

                    # dropdown items
                    add_button(
                        label="New",
                        style_std=ButtonStyleStd.Text,
                        on_press=on_press,
                        user_data="New")

                    add_button(
                        label="Open",
                        style_std=ButtonStyleStd.Text,
                        on_press=on_press,
                        user_data="Open")

                    add_button(
                        label="Save",
                        style_std=ButtonStyleStd.Text,
                        on_press=on_press,
                        user_data="Save")

                with MenuBarItem(width=75.0, spacing=5.0, padding=[5.0]):

                    add_text(content="Edit") # bar item

                    # dropdown items
                    add_button(
                        label="Cut",
                        style_std=ButtonStyleStd.Text,
                        on_press=on_press,
                        user_data="Cut")

                    add_button(
                        label="Copy",
                        style_std=ButtonStyleStd.Text,
                        on_press=on_press,
                        user_data="Copy")

                    add_button(
                        label="Paste",
                        style_std=ButtonStyleStd.Text,
                        on_press=on_press,
                        user_data="Paste")

                with MenuBarItem():

                    add_text(content="Help") # bar item

                    # dropdown items
                    add_button(
                        label="About",
                        style_std=ButtonStyleStd.Text,
                        on_press=on_press,
                        user_data="About")


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
