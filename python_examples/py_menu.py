"""
Simple Menu
"""
from icedpygui import (
    Window,
    Container,
    start_session,
    Menu,
    MenuBar,
    SubMenu,
    add_menu_item,
    add_sub_menu_item,
    add_menu_separator,
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
        with Menu():
            with MenuBar(
                label="File",
                width=250,
                gap=5.0,
            ):
                add_menu_item(label="New File")
                add_menu_item(label="New Window")
                with SubMenu(label="New Window with Profile >"):
                    add_sub_menu_item(label="Profile 1")
                    add_sub_menu_item(label="Profile 2")
                with SubMenu(label="Additional Profiles >"):
                    add_sub_menu_item(label="Profile 3")
                    add_sub_menu_item(label="Profile 4")
                    add_sub_menu_item(label="Profile 5")
                add_menu_separator()
                add_menu_item(label="Open File")
                add_menu_item(label="Open Folder")

            with MenuBar(
                label="Edit",
                width=200.0,
            ):
                add_menu_item(label="Undo")
                add_menu_item(label="Redo")
                add_menu_item(label="Cut")
                add_menu_item(label="Copy")
                add_menu_item(label="Paste")


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
