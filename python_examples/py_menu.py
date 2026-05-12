"""
Simple Menu
"""
from icedpygui import (
    Window,
    Container,
    Row,
    start_session,
    MenuItem,
    add_button,
    ButtonStyleStd)

state = {"menu_id": 0}

def on_press(_id, name: str):
    """Button callback"""
    print(f"selected: {name}")



# Add a window
with Window(title="Menu", center=True, size=[600, 600]):

    with Container(padding=[20.0], fill=True):
        with Row():
            # First item of the MenuBarItem is the bar item followed by the dropdown items
            with MenuItem(label="File", width=150.0, spacing=5.0):

                # dropdown items
                add_button(label="New",
                        on_press=on_press,
                        user_data="New")

                add_button(label="Open",
                        on_press=on_press,
                        user_data="Open")

                add_button(label="Save",
                        on_press=on_press,
                        user_data="Save")

            with MenuItem(label="Edit", width=75.0, spacing=5.0, padding=[5.0]):

                # dropdown items
                add_button(label="Cut",
                        on_press=on_press,
                        user_data="Cut")

                add_button(label="Copy",
                        on_press=on_press,
                        user_data="Copy")

                add_button(label="Paste",
                        on_press=on_press,
                        user_data="Paste")

            with MenuItem(label="Help"):

                # dropdown items
                add_button(label="About",
                        on_press=on_press,
                        user_data="About")


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
