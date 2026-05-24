#!/usr/bin/env python3
"""
Table demo
"""

import random

from icedpygui import (
    Window,
    Container,
    start_session,
    Table,
    TableHeader,
    TableBody,
    TableFooter,
    add_button,
    add_button_style,
    add_text)

column_widths = [100.0] * 4
width = sum(column_widths)

headers = ["one", "two", "three", "four"]
body = [[random.randint(0, 100) for _ in range(4)] for _ in range(11)]
footers = ["footer", "footer", "footer", "footer"]

btn_style = add_button_style(text_center=True, border_radius=[5])

def to_str(data: list[list[float]]) -> list[list[str]]:
    """Convert float data to str"""
    return [[str(cell) for cell in row] for row in data]

def sort_column(_btn_id: int, idx: int):
    """Sort a column"""
    print(idx)


# Add the window
with Window(
        title="Table Demo",
        size=(700, 600),
        center=True):

    # Add the container for centering the table
    with Container(fill=True, align_center=True):

        # The table is added.
        with Table(
            row_height=30.0,
            col_widths=column_widths,
            sash_size=6,):

            with TableHeader():
                for h1 in headers:
                    add_text(content=h1, align_center=True, width_fill=True)

            with TableHeader():
                for index in range(len(headers)):
                    # container used for alignment
                        add_button(
                            label="Sort",
                            padding=[5],
                            width=column_widths[index]-6,
                            on_press=sort_column,
                            user_data=index,
                            style_id=btn_style)

            with TableBody():
                for row in to_str(body):
                    for cell in row:
                        add_text(content=cell, align_center=True, width_fill=True)
            with TableFooter():
                for f in footers:
                    add_text(content=f, align_center=True, width_fill=True)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
