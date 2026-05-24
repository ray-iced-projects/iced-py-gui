#!/usr/bin/env python3
"""
Table demo
"""

import csv
import os

from icedpygui import (
    Window,
    Container,
    start_session,
    Table,
    TableHeader,
    TableBody,
    TableFooter,
    add_pick_list,
    add_text)


cwd = os.getcwd()
FILE_PATH = f"{cwd}/python_examples/resources/table_data/best-selling-books.csv"

header = []
book_lines = []

with open(FILE_PATH, 'r', encoding='utf-8', newline='') as file:
    reader = csv.reader(file)
    header.extend(next(reader))
    for row in reader:
        book_lines.append(row)


column_widths = [200.0] * len(header)
width = sum(column_widths)

footers = ["footer", "footer", "footer", "footer"]

pick_list_options = ["Sort Ascending", "Sort Descending"]

def sort_books(_pick_id: int, idx: int):
    """Sort a column"""
    print(idx)


# Add the window
with Window(
        title="Table Demo",
        size=(1000, 600),
        center=True):

    # Add the container for centering the table
    with Container(fill=True, align_center=True):

        # The table is added.
        with Table(
            row_height=30.0,
            col_widths=column_widths,
            sash_size=6,):

            with TableHeader():
                for h1 in header:
                    add_text(content=h1, align_center=True, fill=True)

            with TableHeader():
                for index in range(len(header)):
                    add_pick_list(
                        options=pick_list_options,
                        placeholder="Sort",
                        on_select=sort_books,
                        user_data=index)

            with TableBody():
                for row in book_lines:
                    for cell in row:
                        add_text(content=cell, align_center=True, fill=True, size=12)

            with TableFooter():
                for f in footers:
                    add_text(content=f, align_center=True, fill=True, size=12)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
