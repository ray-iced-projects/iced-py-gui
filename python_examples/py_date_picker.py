#!/usr/bin/env python3
"""
Date Picker use demo
"""

from icedpygui import (Window, Column, Container, start_session,
    add_date_picker, update_widget, add_text, TextParam)

# Callback for the date picker. The id is the date_picker id, so you have to get
# the id of whatever widget you want to update using a class or, for small projects,
# a global variable. Dataclass is not supported at this time, but you can use a class
# as seen in other demo files.
def date_selected(_dp_id: int, date: str):
    """Date Picker Callback"""
    update_widget(selected_date_id, TextParam.Content, f"You submitted {date}")


# Add a window first
with Window(
    title="Date Picker Demo",
    size=(400, 500),
    center=True):

    # Add the container to center both x and y. Holds only one widget.
    with Container(fill=True, align_center=True):

        # Add a column to hold more than one widget and put this into the container.
        with Column(align_center=True, spacing=10.0):

            # Add info text
            add_text(
                content="Press the calendar button to access the calendar. " +
                "Select a date, then press submit. You can change the format " +
                "through the dropdown list."
            )

            # The date picker size can be scaled from > 1.0. Anything less than 1 will
            # give an error and is not readable anyway.
            add_date_picker(
                size_factor=1.2,
                on_submit=date_selected
            )

            # Text widget id needed for callback.
            selected_date_id = add_text(content="No selection")


# Required to be the last widget sent to Iced. If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
