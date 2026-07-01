#!/usr/bin/env python3
"""
Date Picker use demo
"""

from icedpygui import (Window, Column, Container, start_session,
    DatePicker, add_button, update_widget, add_text, TextParam)


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

            # The date picker size can be scaled from > 1.0.
            with DatePicker(on_submit=date_selected):
                add_button(label="Calendar")

            # Text widget id needed for callback.
            selected_date_id = add_text(content="No selection")


# Required to be the last widget sent to Iced. If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
