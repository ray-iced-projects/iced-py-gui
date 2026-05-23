#!/usr/bin/env python3
"""
Table demo
"""

from icedpygui import (
    Window,
    Container,
    start_session,
    add_table_basic)

column_widths = [100.0] * 4
width = sum(column_widths)

headers = ["str", "one", "two", "three"]
body = [
    [0.0, 1.0, 2.0, 3.0],
    [0.0, 2.0, 4.0, 6.0],
    [0.0, 3.0, 6.0, 9.0],
    [0.0, 4.0, 8.0, 12.0],
    [0.0, 5.0, 10.0, 15.0],
    [0.0, 6.0, 12.0, 18.0],
    [0.0, 7.0, 14.0, 21.0],
    [0.0, 8.0, 16.0, 24.0],
    [0.0, 9.0, 18.0, 27.0],
    [0.0, 10.0, 20.0, 30.0],
    [0.0, 11.0, 22.0, 33.0],
]
footers = ["f", "f", "f", "f"]


def to_str(data: list[list[float]]) -> list[list[str]]:
    """Convert float data to str"""
    return [[str(cell) for cell in row] for row in data]


# Add the window
with Window(
        title="Table Demo",
        size=(700, 600),
        center=True):

    # Add the container for centering the table
    with Container(fill=True, align_center=True):

        # The table is added.
        add_table_basic(
            headers=headers,
            body=to_str(body),
            footers=footers,
            col_widths=column_widths,
            row_height=30.0)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
