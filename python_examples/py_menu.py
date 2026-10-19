"""
Simple Menu
"""
from icedpygui import (
    Window,
    Container,
    Row,
    start_session,
    Menu,
    MenuBar,
    MenuButton,
    SubMenu,
    MenuItem,
    add_checkbox,
    add_separator,
    add_space,
    add_text,
)

def on_item_press(btn_id):
    """Menu item press"""
    print("The button item was pressed", btn_id)

def on_item_press_ud(btn_id, label: str):
    """Menu item press with user data"""
    print("The button item was pressed", btn_id, label)

def set_auto_save(_chk_id: int, checked: bool):
    """Prints autosave value"""
    print(f"auto save set to: {checked}")

def do_action(_btn_id: int, label: str):
    """Prints some action label"""
    print(f"Do some action: {label}")

state = {"is_checked": False}

def menu_row(
    checked: bool,
    label: str,
    label_width: float,
    keys: str,
    arrow: bool,
) -> int:
    """Generates the menu row"""
    size_ = 13.0

    with Row(spacing=5, align_center=True, width_fill=True):
        if checked is True or checked is False:
            add_checkbox(on_toggle=set_auto_save, width=20, is_checked=checked)
        else:
            add_space(width=20)

        with MenuButton(window_id=wnd_id, on_press=do_action, width_fill=True,
                        padding=[0], user_data=label):
            with Row(align_center=True, width_fill=True):
                add_text(content=label, size=size_, width=label_width)

                if keys is not None:
                    add_text(content=keys, size=size_,
                             align_center_right=True,
                             width_fill=True,
                             wrapping_none=True)
                else:
                    add_space(width_fill=True)

                if arrow is not None:
                    add_text(content=">", size=size_,
                             align_center_right=True,
                             width=25)
                else:
                    add_space(width=25)


def sub_menu_row(label: str):
    """Generates a sub menu row"""
    with Row(align_center=True, width=300):
        add_space(width=20)
        add_text(content=label, size=13, width_fill=True)
        add_text(content=">", width=25, size=14, align_center_right=True)


# Add a window
with Window(title="Menu", center=True, size=[600, 600]) as wnd_id:

    with Container(fill=True):
        SIZE = 13.0
        LABEL_SIZE = 13.0
        MAIN_WIDTH_1 = 275

        with Menu() as menu_id:
            with MenuBar(
                label="File",
                label_size=LABEL_SIZE,
                width=50.0,
                dropdown_width=MAIN_WIDTH_1,
                spacing=5,
                gap=10,
                ):
                with MenuItem():
                    menu_row(None, "New Text File", 200, "Ctrl+N", None)
                with MenuItem():
                    menu_row(None, "New File...", 200, "Ctrl+Alt+Super+N", None)
                with MenuItem():
                    menu_row(None, "New Window", 200, "Ctrl+Shift+N", None)
                with MenuItem():
                    menu_row(None, "New Window with Profile", 200, None, True)
                    with SubMenu(width=175):
                        with MenuItem():
                            menu_row(None, "Profile 1", 125, None, None)
                        with MenuItem():
                            menu_row(None, "Profile 2", 125, None, None)
                        with MenuItem():
                            menu_row(None, "Additional Profiles", 125, None, True)
                            with SubMenu(width=75):
                                with MenuItem():
                                    add_text(content="Profile 3", size=SIZE)
                                with MenuItem():
                                    add_text(content="Profile 4", size=SIZE)
                                with MenuItem():
                                    add_text(content="Profile 5", size=SIZE)
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_1)
                with MenuItem():
                    menu_row(None, "Open File...", 200, "Ctrl+O", None)
                with MenuItem():
                    menu_row(None, "Open Folder...", 200, "Ctrl+K Ctrl+O", None)
                with MenuItem():
                    menu_row(None, "Open Workspace from File...", 200, None, None)
                with MenuItem():
                    menu_row(None, "Open Recent", 200, None, True)
                    with SubMenu(width=200):
                        with MenuItem():
                            add_text(content="~/document/iced_menu", size=SIZE)
                with MenuItem():
                    add_separator(width=275)
                with MenuItem():
                    menu_row(None, "Save", 200, "Ctrl+S", None)
                with MenuItem():
                    menu_row(None, "Save as...", 200, "Ctrl+Shift+S", None)
                with MenuItem():
                    menu_row(None, "Save All", 200, None, None)
                with MenuItem():
                    add_separator(width=275)
                with MenuItem():
                    menu_row(None, "Share", 250, None, True)
                    with SubMenu(width=240):
                        with MenuItem():
                            add_text(content="Copy vscode.dec Link", size=SIZE)
                        with MenuItem():
                            add_separator(width=240)
                        with MenuItem():
                            add_text(content="Export Profile (default)", size=SIZE)
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_1)
                with MenuItem():
                    menu_row(state["is_checked"], "Auto Save", 200, None, None)
                with MenuItem():
                    menu_row(None, "Preferences", 200, None, True)
                    with SubMenu(width=125):
                        with MenuItem():
                            menu_row(None, "Profiles", 200, None, None )
                        with MenuItem():
                            menu_row(None, "Settings", 200, "Ctrl+,", None )
                        with MenuItem():
                            menu_row(None, "Extensions", 200, "Ctrl+Shift+X,", None )
                        with MenuItem():
                            menu_row(None, "Keyboard Shorcuts", 200, "Ctrl+K Ctrl+S,", None )
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_1)
                with MenuItem():
                    menu_row(None, "Revert File", 175, None, None )
                with MenuItem():
                    menu_row(None, "Close Editor", 175, "Ctrl+W", None )
                with MenuItem():
                    menu_row(None, "Close Folder", 175, "Ctrl+k F", None )
                with MenuItem():
                    menu_row(None, "Close Window", 175, "Ctrl+F4", None )
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_1)
                with MenuItem():
                    menu_row(None, "Exit", 125, "Ctrl+Q", None )

            MAIN_WIDTH_2 = 300
            with MenuBar(
                label="Edit",
                label_size=LABEL_SIZE,
                width=50.0,
                dropdown_width=MAIN_WIDTH_2,
                spacing=10,
                ):
                with MenuItem():
                    menu_row(None, "Undo", 125, "Ctrl+Z", None)
                with MenuItem():
                    menu_row(None, "Redo", 125, "Ctrl+Y", None)
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_2)
                with MenuItem():
                    menu_row(None, "Cut", 125, "Ctrl+X", None)
                with MenuItem():
                    menu_row(None, "Copy", 125, "Ctrl+C", None)
                with MenuItem():
                    menu_row(None, "Paste", 125, "Ctrl+V", None)
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_2)
                with MenuItem():
                    menu_row(None, "Find", 125, "Ctrl+F", None)
                with MenuItem():
                    menu_row(None, "Replace", 125, "Ctrl+H", None)
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_2)
                with MenuItem():
                    menu_row(None, "Find in Files", 150, "Ctrl+Shift+F", None)
                with MenuItem():
                    menu_row(None, "Replace in Files", 150, "Ctrl+Shift+H", None)
                with MenuItem():
                    menu_row(None, "Toggle Line Comment", 150, "Ctrl+/", None)
                with MenuItem():
                    menu_row(None, "Toggle Block Comment", 175, "Ctrl+Shift+A", None)
                with MenuItem():
                    menu_row(None, "Emmet: Expand Abreviation", 200, "Tab", None)

            with MenuBar(
                label="Misc",
                label_size=LABEL_SIZE,
                width=50.0,
                dropdown_width=200,
                spacing=20,
                ):

                with MenuItem():
                    add_separator(width=200)
                with MenuItem():
                    add_separator(width=200, dot=True, dot_radius=2, dot_count=10)
                with MenuItem():
                    add_separator(width=200, label="Label", label_left_width=50,
                                  label_right_width=50)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
