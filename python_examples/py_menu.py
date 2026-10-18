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
    label:str,
    keys: str,
    arrow: str,
) -> int:
    """Generates the menu row"""
    size_ = 13.0

    with Row(spacing=10, align_center=True, width_fill=True):
        # add_checkbox and button container
        if checked:
            chk_id = add_checkbox(on_toggle=set_auto_save, width=20)
            print("menu_row()", "chk_id", chk_id)
        else:
            add_space(width=20)

        with MenuButton(window_id=wnd_id, on_press=do_action, width_fill=True,
                        padding=[0], user_data=label):
            with Row(align_center=True, width_fill=True):
                add_text(content=label, size=size_, width_fill=True)

                if keys is not None:
                    add_text(content=keys, size=size_, align_center_right=True, width_fill=True)
                else:
                    add_space()

                if arrow is not None:
                    add_text(content=">", size=size_, align_center_right=True, width=25)
                else:
                    add_space(width=25)


def sub_menu_row(label: str):
    """Generates a sub menu row"""
    with Row(align_center=True, width_fill=True):
        add_space(width=20)
        add_text(content=label, size=14, width_fill=True)
        add_text(content=">", width=25, size=14, align_center_right=True)


# Add a window
with Window(title="Menu", center=True, size=[600, 600]) as wnd_id:

    with Container(fill=True):
        SIZE = 13.0
        LABEL_SIZE = 13.0

        with Menu() as menu_id:
            with MenuBar(
                label="File",
                label_size=LABEL_SIZE,
                width=300.0,
                ) as menu_bar_id:

                with MenuItem():
                    menu_row(None, "New Text File", "Ctrl+N", None)
                    with SubMenu(width=200.0) as submenu_id:
                        txt_id = add_text(content="New Window with Profile")
                        with MenuItem():
                            add_text(content="Profile 1", size=SIZE)
                            with SubMenu(width=100.0):
                                add_text(content="Additional Profiles >", size=SIZE)
                                with MenuItem():
                                    add_text(content="Profile 2", size=SIZE)

            with MenuBar(
                label="Edit",
                label_size=LABEL_SIZE,
                width=300.0,
                ) as menu_bar_id:
                with MenuItem():
                    menu_row(None, "Copy", "Ctrl+C", None)
                with MenuItem() as mi_id:
                    menu_row(None, "Paste", "Ctrl+V", None)
                with MenuItem() as mi_id:
                    menu_row(None, "New File...", "Ctrl+Alt+Super+N", None)
                with MenuItem() as mi_id:
                    menu_row(None, "New Window", "Ctrl+Shift+N", None)
                    with SubMenu(width=200.0):
                        add_text(content="New Window with Profile")
                        with MenuItem():
                            add_text(content="Profile 1", size=SIZE)
                        with MenuItem():
                            add_text(content="Profile 2", size=SIZE)
                            with SubMenu(width=100.0):
                                add_text(content="Additional Profiles >", size=SIZE)
                                with MenuItem():
                                    add_text(content="Profile 3", size=SIZE)
                                    add_text(content="Profile 4", size=SIZE)
                                    add_text(content="Profile 5", size=SIZE)
                with MenuItem():
                    add_separator(width=200.0, height=2.0)
                with MenuItem():
                    menu_row(None, "Open File...", "Ctrl+O", None )
                with MenuItem():
                    menu_row(None, "Open Folder...", "Ctrl+K Crtl+O", None)
                with MenuItem():
                    menu_row(None, "Open Workspace from File...", None, None)
                    with SubMenu(width=240.0):
                        with MenuItem():
                            sub_menu_row("Open Recent")
                            add_text(content="~/document/iced_menu", size=SIZE)
                with MenuItem():
                    add_separator(width=200.0, height=2.0)
                with MenuItem():
                    menu_row(None, "Save", "Ctrl+S", None)
                with MenuItem():
                    menu_row(None, "Save as...", "Ctrl+Shift+S", None)
                with MenuItem():
                    menu_row(None, "Save All", None, None)
                with MenuItem():
                    add_separator(width=200.0, height=2.0)
                    with SubMenu(width=240.0):
                        sub_menu_row("Share")
                        with MenuItem():
                            add_text(content="Copy vscode.dec Link", size=SIZE)
                        with MenuItem():
                            add_separator(width=200.0, height=2.0)
                        with MenuItem():
                            add_text(content="Export Profile (default)", size=SIZE)

                with MenuItem():
                    add_separator(width=200.0, height=2.0)
                with MenuItem():
                    menu_row(state.get("is_checked"), "Auto Save", None, None)

                    with SubMenu(width=300.0):
                        sub_menu_row("Preferences")
                        with MenuItem():
                            menu_row(None, "Profiles", None, None)
                        with MenuItem():
                            menu_row(None, "Settings", "Ctrl+,", None)
                        with MenuItem():
                            menu_row(None, "Extensions", "Ctrl+Shift+X,", None)
                        with MenuItem():
                            menu_row(None, "Keyboard Shorcuts", "Ctrl+K Ctrl+S,", None)
                with MenuItem():
                    add_separator(width=200.0, height=2.0)
                with MenuItem():
                    menu_row(None, "Revert File", None, None)
                with MenuItem():
                    menu_row(None, "Close Editor", "Ctrl+W", None)
                with MenuItem():
                    menu_row(None, "Close Folder", "Ctrl+k F", None)
                with MenuItem():
                    menu_row(None, "Close Window", "Ctrl+F4", None)
                with MenuItem():
                    add_separator(width=200.0, height=2.0)
                with MenuItem():
                    menu_row(None, "Exit", "Ctrl+Q", None)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
