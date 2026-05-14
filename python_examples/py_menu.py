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
    add_separator,
    add_text,
    ButtonStyleStd)


def on_press(_id, name: str):
    """Button callback"""
    print(f"selected: {name}")

btn_style = add_button_style(text_center_left=True)

# Add a window
with Window(title="Menu", center=True, size=[600, 600]):

    with Container(padding=[20.0], fill=True):
        with Column(spacing=20):
            with Menu(spacing=20.0):

                # First item of the MenuBarItem is the bar item followed by the dropdown items
                with MenuBarItem(width=125, spacing=5.0, offset=3.0):

                    add_text(content="File") # bar item
                    # dropdown items
                    add_button(
                        label="New",
                        width_fill=True,
                        style_std=ButtonStyleStd.Text,
                        on_press=on_press,
                        user_data="New")
                    add_separator(
                        dot=True,
                        dot_radius=3.0,
                        dot_count=8,
                        spacing=10.0,)

                    #  Submenu
                    with MenuSubItem(width=130, spacing=5.0, offset=2.0):

                        add_text(content="Open Recent >", width_fill=True,)  # trigger (shown in dropdown)
                        add_button(
                            label="project1.py",
                            width_fill=True,
                            style_std=ButtonStyleStd.Text)
                        add_button(
                            label="project2.py",
                            width_fill=True,
                            style_std=ButtonStyleStd.Text)

                    add_separator(
                        dot=True,
                        dot_radius=3.0,
                        dot_count=8,
                        spacing=10.0,)

                    add_button(
                        label="Open",
                        width_fill=True,
                        style_std=ButtonStyleStd.Text,
                        on_press=on_press,
                        user_data="Open")

                    add_button(
                        label="Save",
                        width_fill=True,
                        style_std=ButtonStyleStd.Text,
                        on_press=on_press,
                        user_data="Save")

                with MenuBarItem(width=50.0, spacing=5.0, offset=3.0):

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

                with MenuBarItem(width=75.0, offset=3.0):

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
