"""
Sub-menu test: MenuSubItem inside a MenuBarItem
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
    add_text,
    ButtonStyleStd)


def on_press(_id, name: str):
    print(f"selected: {name}")


with Window(title="SubMenu Test", center=True, size=[600, 400]):

    with Container(padding=[20.0], fill=True):
        with Column(spacing=20):
            with Menu(spacing=20.0):

                with MenuBarItem(width=75, spacing=5.0, offset=3.0):

                    add_text(content="File")  # bar item

                    add_button(label="New", style_std=ButtonStyleStd.Text,
                               on_press=on_press, user_data="New")

                    # Sub-menu: first child is trigger, rest are child items
                    with MenuSubItem(width=130, spacing=5.0, offset=2.0):
                        add_button(label="Open Recent >", style_std=ButtonStyleStd.Text,
                                   on_press=on_press, user_data="OpenRecent")
                        add_button(label="project1.py", style_std=ButtonStyleStd.Text,
                                   on_press=on_press, user_data="project1")
                        add_button(label="project2.py", style_std=ButtonStyleStd.Text,
                                   on_press=on_press, user_data="project2")

                    add_button(label="Save", style_std=ButtonStyleStd.Text,
                               on_press=on_press, user_data="Save")

                with MenuBarItem(width=75.0, spacing=5.0, offset=3.0):

                    add_text(content="Edit")

                    add_button(label="Cut", style_std=ButtonStyleStd.Text,
                               on_press=on_press, user_data="Cut")
                    add_button(label="Copy", style_std=ButtonStyleStd.Text,
                               on_press=on_press, user_data="Copy")
                    add_button(label="Paste", style_std=ButtonStyleStd.Text,
                               on_press=on_press, user_data="Paste")


start_session()
