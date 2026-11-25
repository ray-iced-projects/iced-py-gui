#!/usr/bin/env python3
"""
Table demo
"""

from icedpygui import (
    Window,
    Container,
    start_session,
    add_table_basic,
    add_text_format)

column_widths = [100.0] * 4
width = sum(column_widths)

header = ["zero", "one", "two", "three"]
# The table basic only takes a str value but
# these can easily be converted
# using something like the to_str() below.
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
footer = ["f", "f", "f", "f"]


def to_str(data: list[list[float]]) -> list[list[str]]:
    """Convert float data to str"""
    return [[str(cell) for cell in row] for row in data]

# fill has to be true if any alignment is used
text_format = add_text_format(align_center=True, fill=True, size=14.0)

# Add the window
with Window(
        title="Table Basic Demo",
        size=(700, 600),
        center=True):

    # Add the container for centering the table
    with Container(fill=True, align_center=True):

        # The table is added.
        add_table_basic(
            header=header,
            header_text_format_id=text_format,
            body=to_str(body),
            footer=footer,
            footer_text_format_id=text_format,
            column_widths=column_widths,
            scrollable_height=300.0,
            )


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
