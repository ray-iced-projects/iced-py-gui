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
    add_icon,
    add_pick_list,
    add_space,
    add_text,
    Arrow)


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

genres = ["None", "Biography", "Children's Lit", "Christian Lit", "Dark Comedy",
"Detective", "Dystopian", "Erotica", "Fantasy", "Fiction",
"Historical Fiction", "History", "Horror", "Mystery/Crime",
"Romance", "Science Fiction", "Science", "Self-Help", "Social Science",
"Thriller/Suspense", "Travel", "True Crime", "Young Adult"]

icon_open = add_icon(arrow=Arrow.ArrowBarLeft)
icon_closed = add_icon(arrow=Arrow.ArrowLeft)

def sort_books(_pick_id: int, data: str):
    """Sort a column"""
    print(data)


def filter_genre(_pick_id: int, genre: str):
    """Filter by Genre"""
    print(genre)


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
                    add_text(content=h1, align_center=True, fill=True, size=14)

                for index in range(len(header)):
                    add_pick_list(
                        options=pick_list_options,
                        placeholder="Sort",
                        handle_dynamic_closed_icon_id=icon_closed,
                        handle_dynamic_open_icon_id=icon_open,
                        on_select=sort_books)

            with TableHeader():
                for index in range(len(header)):
                    if index == len(header)-1:
                        add_pick_list(
                            options=genres,
                            placeholder="Filter",
                            on_select=filter_genre)
                    else:
                        add_space(width=0)


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
