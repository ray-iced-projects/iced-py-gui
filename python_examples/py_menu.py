"""
Simple Menu
"""
from icedpygui import (
    Window,
    Column,
    Container,
    start_session,
    Menu,
    add_button_style,
)

bar_items = ["File", "Edit", "Help"]

def on_bar_item_press(bar_id, idx: int):
    """Menu bar callback"""
    print(f"press = id: {bar_id} name: {bar_items[idx]}")

def on_bar_item_enter(bar_id, idx: int):
    """Menu bar callback"""
    print(f"enter = id: {bar_id} name: {bar_items[idx]}")

def on_bar_item_exit(bar_id, idx: int):
    """Menu bar callback"""
    print(f"exit = id: {bar_id} name: {bar_items[idx]}")


btn_style = add_button_style(text_center_left=True)

# Add a window
with Window(title="Menu", center=True, size=[600, 600]):

    with Container(padding=[20.0], fill=True):
        with Column(spacing=20):
            with Menu(
                bar_items =bar_items,
                on_bar_item_press=on_bar_item_press,
                on_bar_item_enter=on_bar_item_enter,
                on_bar_item_exit=on_bar_item_exit,
                ):
                pass
                # The MenuBarItem's order must match the order of the Menu bar_items position.
                # with MenuBarItem(width=125, spacing=5.0, offset=3.0):
                #     # dropdown for "File"
                #     add_button(
                #         label="New",
                #         style_std=ButtonStyleStd.Text,
                #         on_press=on_press,
                #         user_data="New")
                #     add_separator(
                #         dot=True,
                #         dot_radius=3.0,
                #         dot_count=8,
                #         spacing=10.0)

                #     with MenuSubItem(label="Open Recent"):
                #         add_button(
                #             label="project1.py",
                #             style_std=ButtonStyleStd.Text)
                #         add_button(
                #             label="project2.py",
                #             style_std=ButtonStyleStd.Text)

                #     add_button(
                #         label="Search",
                #         style_std=ButtonStyleStd.Text,
                #         on_press=on_press,
                #         user_data="Search")

                #     add_separator(
                #         dot=True,
                #         dot_radius=3.0,
                #         dot_count=8,
                #         spacing=10.0,)

                #     add_button(
                #         label="Open",
                #         style_std=ButtonStyleStd.Text,
                #         on_press=on_press,
                #         user_data="Open")

                #     add_button(
                #         label="Save",
                #         style_std=ButtonStyleStd.Text,
                #         on_press=on_press,
                #         user_data="Save")

                # with MenuBarItem(width=50.0, spacing=5.0, offset=3.0):
                #     # dropdown items for "Edit"
                #     add_button(
                #         label="Cut",
                #         style_std=ButtonStyleStd.Text,
                #         on_press=on_press,
                #         user_data="Cut")

                #     add_button(
                #         label="Copy",
                #         style_std=ButtonStyleStd.Text,
                #         on_press=on_press,
                #         user_data="Copy")

                #     add_button(
                #         label="Paste",
                #         style_std=ButtonStyleStd.Text,
                #         on_press=on_press,
                #         user_data="Paste")

                # with MenuBarItem(width=75.0, offset=3.0):
                #     # dropdown items "Help"
                #     add_button(
                #         label="About",
                #         style_std=ButtonStyleStd.Text,
                #         on_press=on_press,
                #         user_data="About")



# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
