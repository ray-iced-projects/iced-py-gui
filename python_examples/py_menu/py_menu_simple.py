"""
Complet Menu
"""
from icedpygui import (
    Window,
    Container,
    start_session,
    Menu,
    MenuBar,
    SubMenu,
    MenuItem,
    MenuButton,
    add_text,
)

state = {"cut": 0,
         "copy": 0,
         "paste": 0,}


def on_item_press(btn_id):
    """Menu item press"""
    print("The button item was pressed", btn_id)


def on_item_press_ud(btn_id, label: str):
    """Menu item press with user data"""
    print("The button item was pressed", btn_id, label)


def on_item_press_edit(menu_btn_id: int):
    """Method uses the MenuButton id"""
    key = next((k for k, v in state.items() if v == menu_btn_id), None)
    if key is not None:
        print(f"Doing some action for menu item {key}")
    else:
        print("Menu Item could not be found")


def do_action(_btn_id: int, label: str):
    """Prints some action label"""
    print(f"Do some action: {label}")


# Add a window
with Window(title="Simple Menu", center=True, size=[600, 600]):

    with Container(fill=True):
        SIZE = 13.0
        LABEL_SIZE = 13.0

        with Menu():
            with MenuBar(
                label="File",
                label_size=LABEL_SIZE,
                width=50.0,
                dropdown_width=200,
                spacing=5,
                gap=10,
                ):
                with MenuItem():
                    # MenuButton is a special button that accepts a single widget
                    # same as a Container.  Use Column or Row if more than one
                    # widget is needed.  See the menu_complex example.
                    # This case used the user_data to pass on the label but see
                    # the edit below for another way.
                    with MenuButton(on_press=on_item_press_ud, user_data="New Text"):
                        add_text(content="New Text File", size=LABEL_SIZE)
                with MenuItem():
                    with MenuButton(on_press=on_item_press_ud, user_data="New File"):
                        add_text(content="New File...", size=LABEL_SIZE)
                # IMPORTANT: The MenuItem before the SubMenu content will be the label
                # that when pressed or hovered opens the submenu.  Therefore, the submenu must
                # be in the MenuItem and therefore must be within the 'with' statement.
                # if you see an error then it should indicate your issue.
                # To help witht eh error message, you can use the str_id="Some text"
                # in the MenuItem, SubMenu, MenuBar which will be printed when the error occurs.
                # IMPORTANT: The MenuItem() before the SubMenu DOES NOT use a MenuButton.
                with MenuItem():
                    add_text(content="New Window with Profile  >>", size=LABEL_SIZE)
                    with SubMenu(width=100):
                        with MenuItem():
                            with MenuButton(on_press=on_item_press_ud, user_data="Profile 1"):
                                add_text(content="Profile 1", size=LABEL_SIZE)
                        with MenuItem():
                            with MenuButton(on_press=on_item_press_ud, user_data="Profile 2"):
                                add_text(content="Profile 2", size=LABEL_SIZE)

                with MenuItem():
                    with MenuButton(on_press=on_item_press_ud, user_data="Open File"):
                        add_text(content="Open File...", size=LABEL_SIZE)
                with MenuItem():
                    with MenuButton(on_press=on_item_press_ud, user_data="Open Folder"):
                        add_text(content="Open Folder...", size=LABEL_SIZE)

            with MenuBar(
                label="Edit",
                label_size=LABEL_SIZE,
                width=50.0,
                dropdown_width=100,
                spacing=10,
                ):

                # Showing a different way to know which label was pressed
                # This method uses the state dict where the MenuButton id
                # is used to find the menu key that was pressed.
                with MenuItem():
                    with MenuButton(on_press=on_item_press_edit) as state["cut"]:
                        add_text(content="Cut", size=LABEL_SIZE)
                with MenuItem():
                    with MenuButton(on_press=on_item_press_edit) as state["copy"]:
                        add_text(content="Copy", size=LABEL_SIZE)
                with MenuItem():
                    with MenuButton(on_press=on_item_press_edit) as state["paste"]:
                        print(state["Paste"])
                        add_text(content="Paste", size=LABEL_SIZE)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
