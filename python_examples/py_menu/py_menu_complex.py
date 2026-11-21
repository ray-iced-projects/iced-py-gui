"""
Complet Menu
"""
from icedpygui import (
    Window,
    # Color,
    Container,
    # add_container_style,
    # ContainerStyleStd,
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

# for testing the container style used in the MenuBar
# When you change the style of the dropdown, which is a container
# you need to use the container styling widgets.
# You will also need to change the style of the text used in the MenuButton
# in order to see the text if the text color doesn't fit since
# the text color is based on the window theme and not the container theme.

# style_id = add_container_style(bkg_color=Color.ALICE_BLUE)


# The menu item is construct to be a row of widgets in this case.
# This is a more complex situation to show what can be done with the menu.
# The most complex here is the alignment of the text so that the
# label is aligned left and the keys are aligned right followed by an arrow
# if there is a sub menu.  The first item in the row is just a
# checkbox with no label.  This is a bit different than in the vscode
# menu since it that case, its just a reflection of the settings and not
# an actionable checkbox.
def menu_row(
    label: str,
    label_width: float,
    keys: str,
    arrow: bool,
) -> int:
    """Generates the menu row"""
    size_ = 13.0

    with Row(spacing=5, align_center=True, width_fill=True):
        # sapacing for the checkbox that is added later to keep alignment
        add_space(width=20)

        # The MenuButton is special since it is a container that accepts
        # a single widget like row, column or container.  In doing this,
        # as done below, the whole row is a button that when pressed does the callback.
        # You can also use this outside of the menu if you want.
        # If you menu is simple, you can just use a normal button.
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

# a simple sub menu row
def for_sub_menu_row(label: str, width: float):
    """Generates a sub menu row"""
    with Row(align_center=True, width=width):
        add_space(width=25)
        add_text(content=label, size=13, width_fill=True)
        add_text(content=">", width=25, size=14, align_center_right=True)


# Add a window
with Window(title="Menu", center=True, size=[600, 600]) as wnd_id:

    with Container(fill=True):
        SIZE = 13.0
        LABEL_SIZE = 13.0
        MAIN_WIDTH_1 = 275

        with Menu(padding=[10]):
            with MenuBar(
                label="File",
                label_size=LABEL_SIZE,
                width=50.0,
                dropdown_width=MAIN_WIDTH_1,
                spacing=5,
                gap=10,
                # These are commented out to show you that the dropdown is a container
                # and needs the container styling, if used.  Uncomment one and the
                # style_id above to see the effect.
                # container_style_id=style_id,
                # container_style_std=ContainerStyleStd.Secondary,
                ):
                with MenuItem():
                    menu_row("New Text File", 200, "Ctrl+N", None)
                with MenuItem():
                    menu_row("New File...", 200, "Ctrl+Alt+Super+N", None)
                with MenuItem():
                    menu_row("New Window", 200, "Ctrl+Shift+N", None)
                # Important: The MenuItem before the SubMenu content will be the label
                # that when pressed opens the submenu.  Therefore, the submenu must
                # be in the MenuItem and therefore must be within the 'with' statement.
                # if you see an error then it should indicate your issue.
                # To help with the error message, you can use the str_id="Some text"
                # in the MenuItem, SubMenu, MenuBar which will be printed when the error occurs.
                # When you are first creating a menu, it might be useful to use the
                # str_id in each so that the error can be tracked better when it occurs.
                with MenuItem():
                    # No MenuButton used for the labels before a SubMenu
                    with Row(spacing=5, align_center=True, width_fill=True):
                        # The sub_menu_row just supplies a widget, no MenuButton()
                        # Otherwise if if was pressed, the menu would close.
                        for_sub_menu_row("New Window with Profile", MAIN_WIDTH_1)
                    with SubMenu(width=200):
                        with MenuItem():
                            menu_row("Profile 1", 125, None, None)
                        with MenuItem():
                            menu_row("Profile 2", 125, None, None)
                        with MenuItem():
                            for_sub_menu_row("Additional Profiles", 200)
                            with SubMenu(width=100):
                                with MenuItem():
                                    menu_row("Profile 3", 100, None, None)
                                with MenuItem():
                                    menu_row("Profile 4", 100, None, None)
                                with MenuItem():
                                    menu_row("Profile 5", 100, None, None)
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_1)
                with MenuItem():
                    menu_row("Open File...", 200, "Ctrl+O", None)
                with MenuItem():
                    menu_row("Open Folder...", 200, "Ctrl+K Ctrl+O", None)
                with MenuItem():
                    menu_row("Open Workspace from File...", 200, None, None)
                with MenuItem():
                    for_sub_menu_row("Open Recent", MAIN_WIDTH_1)
                    with SubMenu(width=200):
                        with MenuItem():
                            menu_row("~/document/iced_menu", 200, None, None)
                with MenuItem():
                    add_separator(width=275)
                with MenuItem():
                    menu_row("Save", 200, "Ctrl+S", None)
                with MenuItem():
                    menu_row("Save as...", 200, "Ctrl+Shift+S", None)
                with MenuItem():
                    menu_row("Save All", 200, None, None)
                with MenuItem():
                    add_separator(width=275)
                with MenuItem():
                    for_sub_menu_row("Share", MAIN_WIDTH_1)
                    with SubMenu(width=240):
                        with MenuItem():
                            menu_row("Copy vscode link", 240, None, None)
                        with MenuItem():
                            add_separator(width=240)
                        with MenuItem():
                            menu_row("Export Profile", 240, None, None)
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_1)
                with MenuItem():
                    add_checkbox(label="Auto Save", on_toggle=set_auto_save,
                                 width=200, is_checked=state["is_checked"],
                                 text_size=SIZE)
                with MenuItem():
                    for_sub_menu_row("Preferences", MAIN_WIDTH_1)
                    with SubMenu(width=125):
                        with MenuItem():
                            menu_row("Profiles", 200, None, None )
                        with MenuItem():
                            menu_row("Settings", 200, "Ctrl+,", None )
                        with MenuItem():
                            menu_row("Extensions", 200, "Ctrl+Shift+X,", None )
                        with MenuItem():
                            menu_row("Keyboard Shorcuts", 200, "Ctrl+K Ctrl+S,", None )
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_1)
                with MenuItem():
                    menu_row("Revert File", 175, None, None )
                with MenuItem():
                    menu_row("Close Editor", 175, "Ctrl+W", None )
                with MenuItem():
                    menu_row("Close Folder", 175, "Ctrl+k F", None )
                with MenuItem():
                    menu_row("Close Window", 175, "Ctrl+F4", None )
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_1)
                with MenuItem():
                    menu_row("Exit", 125, "Ctrl+Q", None )

            MAIN_WIDTH_2 = 300
            with MenuBar(
                label="Edit",
                label_size=LABEL_SIZE,
                width=50.0,
                dropdown_width=MAIN_WIDTH_2,
                spacing=10,
                ):
                with MenuItem():
                    menu_row("Undo", 125, "Ctrl+Z", None)
                with MenuItem():
                    menu_row("Redo", 125, "Ctrl+Y", None)
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_2)
                with MenuItem():
                    menu_row("Cut", 125, "Ctrl+X", None)
                with MenuItem():
                    menu_row("Copy", 125, "Ctrl+C", None)
                with MenuItem():
                    menu_row("Paste", 125, "Ctrl+V", None)
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_2)
                with MenuItem():
                    menu_row("Find", 125, "Ctrl+F", None)
                with MenuItem():
                    menu_row("Replace", 125, "Ctrl+H", None)
                with MenuItem():
                    add_separator(width=MAIN_WIDTH_2)
                with MenuItem():
                    menu_row("Find in Files", 150, "Ctrl+Shift+F", None)
                with MenuItem():
                    menu_row("Replace in Files", 150, "Ctrl+Shift+H", None)
                with MenuItem():
                    menu_row("Toggle Line Comment", 150, "Ctrl+/", None)
                with MenuItem():
                    menu_row("Toggle Block Comment", 175, "Ctrl+Shift+A", None)
                with MenuItem():
                    menu_row("Emmet: Expand Abreviation", 200, "Tab", None)

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
                    with Row(align_center=True):
                        add_separator(width=75)
                        add_text(content="Label", size=SIZE,
                                 width=50, align_center=True)
                        add_separator(width=75)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
