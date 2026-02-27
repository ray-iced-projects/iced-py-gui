from imports import *

"""
Minimal example demonstrating the new module-level function API.

This example shows how to use individual functions instead of the IPG class.
"""

from icedpygui import (
    add_window, 
    add_button, 
    start_session,
    IpgWindowTheme,
    IpgAlignmentX,
    IpgAlignmentY,
)


def on_button_press(btn_id):
    """Callback when button is pressed."""
    print(f"Button {btn_id} was pressed!")


def main():
    # Create a window - this replaces add_window()
    window_id = add_window(
        window_id="main",
        title="Minimal Example",
    size=(400.0, 300.0),
        theme=IpgWindowTheme.Dark,
        exit_on_close=True)
    print(f"Created window with id: {window_id}")

    # Add a button directly to the window
    # Note: In the full version, you'd add containers first
    btn_id = add_button(
        parent_id="main",
        label="Click Me!",
        on_press=on_button_press,
        text_align_x=IpgAlignmentX.Center,
        text_align_y=IpgAlignmentY.Center,
    )
    print(f"Created button with id: {btn_id}")

    # Start the GUI event loop - this replaces start_session()
    start_session()


if __name__ == "__main__":
    main()
