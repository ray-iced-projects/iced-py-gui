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
    MenuSubItem,
    add_button,
    add_button_style,
    ButtonStyleStd,
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

def on_item_press(btn_id):
    """Menu item press"""
    print("The button item was pressed", btn_id)

btn_style = add_button_style(text_center_left=True)

# Add a window
with Window(title="Menu", center=True, size=[600, 600]):

    with Container(padding=[20.0], fill=True):
        with Column(spacing=20):
            with Menu(
                bar_items=bar_items,
                bar_widths=[100.0],
                on_bar_item_press=on_bar_item_press,
                on_bar_item_enter=on_bar_item_enter,
                on_bar_item_exit=on_bar_item_exit,
                ):

                # The MenuBarItem's order must match the order of the Menu bar_items position.
                with MenuBarItem():
                    # dropdown items for "File"
                    add_button(
                        label="New",
                        width=100,
                        on_press=on_item_press)

                    with MenuSubItem(label="Open Recent 1"):
                        add_button(
                            label="project1.py",
                            width=100,
                            style_std=ButtonStyleStd.Text)
                        add_button(
                            label="project2.py",
                            width=100,
                            style_std=ButtonStyleStd.Text)

                        with MenuSubItem(label="Open Recent Sub"):
                            add_button(
                                label="project1.py",
                                width=100,
                                style_std=ButtonStyleStd.Text)
                            add_button(
                                label="project2.py",
                                width=100,
                                style_std=ButtonStyleStd.Text)

                            with MenuSubItem(label="Open Recent Sub's Sub"):
                                add_button(
                                    label="project1.py",
                                    width=100,
                                    style_std=ButtonStyleStd.Text)
                                add_button(
                                    label="project2.py",
                                    width=100,
                                    style_std=ButtonStyleStd.Text)

                    add_button(
                        label="Search",
                        width=100,
                        style_std=ButtonStyleStd.Text,
                        on_press=on_item_press,
                        user_data="Search")

                    add_button(
                        label="Open",
                        width=100,
                        style_std=ButtonStyleStd.Text,
                        on_press=on_item_press,
                        user_data="Open")

                    add_button(
                        label="Save",
                        width=100,
                        style_std=ButtonStyleStd.Text,
                        on_press=on_item_press,
                        user_data="Save")

                with MenuBarItem():
                    # dropdown items for "Edit"
                    add_button(
                        label="Cut",
                        width=100,
                        style_std=ButtonStyleStd.Text,
                        on_press=on_item_press,
                        user_data="Cut")

                    add_button(
                        label="Copy",
                        width=100,
                        style_std=ButtonStyleStd.Text,
                        on_press=on_item_press,
                        user_data="Copy")

                    add_button(
                        label="Paste",
                        width=100,
                        style_std=ButtonStyleStd.Text,
                        on_press=on_item_press,
                        user_data="Paste")

                with MenuBarItem():
                    # dropdown items "Help"
                    add_button(
                        label="About",
                        width=100,
                        style_std=ButtonStyleStd.Text,
                        on_press=on_item_press,
                        user_data="About")



# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
