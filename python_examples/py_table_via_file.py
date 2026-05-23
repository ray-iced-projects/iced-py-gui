#!/usr/bin/env python3
"""
Table demo
"""
import os
from icedpygui import (
    Window,
    Container,
    start_session,
    add_table_basic)

column_widths = [100.0] * 7

cwd = os.getcwd()
FILE_PATH = f"{cwd}/python_examples/resources/google.csv"



def to_str(data: list[list[float]]) -> list[list[str]]:
    """Convert float data to str"""
    return [[str(cell) for cell in row] for row in data]


# Add the window
with Window(
        title="Table Demo",
        size=(800, 600),
        center=True):

    # Add the container for centering the table
    with Container(fill=True, align_center=True):

        add_table_basic(
                file_path=FILE_PATH,
                col_widths=column_widths,
                row_height=30.0,
                scrollable_height=300.0)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
