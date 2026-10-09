"""
Simple Menu
"""
from icedpygui import (
    Window,
    Container,
    start_session,
    Menu,
    MenuBarItem,
    MenuSubItem,
    add_button,
    ButtonStyleStd,
)

bar_labels = ["File", "Edit", "Help"]

def on_bar_item_press(bar_id, idx: int):
    """Menu bar callback"""
    print(f"press = id: {bar_id} name: {bar_labels[idx]}")

def on_bar_item_enter(bar_id, idx: int):
    """Menu bar callback"""
    print(f"enter = id: {bar_id} name: {bar_labels[idx]}")

def on_bar_item_exit(bar_id, idx: int):
    """Menu bar callback"""
    print(f"exit = id: {bar_id} name: {bar_labels[idx]}")

def on_item_press(btn_id):
    """Menu item press"""
    print("The button item was pressed", btn_id)

def on_item_press_ud(btn_id, label: str):
    """Menu item press with user data"""
    print("The button item was pressed", btn_id, label)


# Add a window
with Window(title="Menu", center=True, size=[600, 600]):

    with Container(fill=True):
        with Menu(
            bar_labels=bar_labels,
            bar_widths=[75.0],
            on_bar_item_press=on_bar_item_press,
            on_bar_item_enter=on_bar_item_enter,
            on_bar_item_exit=on_bar_item_exit,
            ):

            # The MenuBarItem's order must match the order of the Menu bar_items position.
            with MenuBarItem(width=175):
                # dropdown items for "File"
                add_button(
                    label="New",
                    width_fill=True,
                    on_press=on_item_press,
                    style_std=ButtonStyleStd.Text)

                with MenuSubItem(label="Top dropdown 1.0"):
                    add_button(
                        label="project1.py",
                        width=100,
                        style_std=ButtonStyleStd.Text)
                    add_button(
                        label="project2.py",
                        width=100,
                        style_std=ButtonStyleStd.Text)

                    with MenuSubItem(label="Inner dropdown 1.1"):
                        add_button(
                            label="project1.py",
                            width=100,
                            style_std=ButtonStyleStd.Text)
                        add_button(
                            label="project2.py",
                            width=100,
                            style_std=ButtonStyleStd.Text)

                        with MenuSubItem(label="Inner dropdown 1.2"):
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
                    width_fill=True,
                    on_press=on_item_press_ud,
                    style_std=ButtonStyleStd.Text,
                    user_data="Search")

                with MenuSubItem(label="Top dropdown 2.0"):
                    add_button(
                        label="project1.py",
                        width=100,
                        style_std=ButtonStyleStd.Text)
                    add_button(
                        label="project2.py",
                        width=100,
                        style_std=ButtonStyleStd.Text)


                add_button(
                    label="Open",
                    width_fill=True,
                    on_press=on_item_press_ud,
                    style_std=ButtonStyleStd.Text,
                    user_data="Open")

                add_button(
                    label="Save",
                    width_fill=True,
                    on_press=on_item_press_ud,
                    style_std=ButtonStyleStd.Text,
                    user_data="Save")

            with MenuBarItem():
                # dropdown items for "Edit"
                add_button(
                    label="Cut",
                    width_fill=True,
                    on_press=on_item_press_ud,
                    style_std=ButtonStyleStd.Text,
                    user_data="Cut")

                add_button(
                    label="Copy",
                    width_fill=True,
                    on_press=on_item_press_ud,
                    style_std=ButtonStyleStd.Text,
                    user_data="Copy")

                add_button(
                    label="Paste",
                    width_fill=True,
                    on_press=on_item_press_ud,
                    style_std=ButtonStyleStd.Text,
                    user_data="Paste")

            with MenuBarItem(width=175):
                # dropdown items "Help"
                add_button(
                    label="About",
                    width_fill=True,
                    on_press=on_item_press_ud,
                    style_std=ButtonStyleStd.Text,
                    user_data="About")
                with MenuSubItem(label="Top dropdown 3.0"):
                    add_button(
                        label="About 1",
                        width=100,
                        style_std=ButtonStyleStd.Text)
                    add_button(
                        label="About 2",
                        width=100,
                        style_std=ButtonStyleStd.Text)



# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
