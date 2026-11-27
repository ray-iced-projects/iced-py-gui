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
    add_text_format,
    TextParam,
    PickListParam,
    Arrow,
    update_widget)

header = []
book_lines = []

cwd = os.getcwd()
FILE_PATH = f"{cwd}/python_examples/resources/table_data/best-selling-books.csv"

try:
    with open(FILE_PATH, "r", encoding='utf-8', newline='') as file:
        reader = csv.reader(file)
        header.extend(next(reader))
        for row in reader:
            book_lines.append(row)
except FileNotFoundError:
    print(f"The file does not exist using {FILE_PATH}.")

# Title,Author(s),Originallanguage,First published,Sales(10^6),Genre
column_widths = [300.0, 200.0, 100.0, 75.0, 100.0, 150.0]
width = sum(column_widths)

footers = ["footer"] * 6

pick_list_options = ["Ascending", "Descending"]

genres = ["None", "Biography", "Children's Lit", "Christian Lit", "Dark Comedy",
"Detective", "Dystopian", "Erotica", "Fantasy", "Fiction",
"Historical Fiction", "History", "Horror", "Mystery/Crime",
"Romance", "Science Fiction", "Science", "Self-Help", "Social Science",
"Thriller/Suspense", "Travel", "True Crime", "Young Adult"]

sort_dict = {}
body_text_ids = []
state = {"filtered_genre": None}

icon_open = add_icon(arrow=Arrow.ArrowBarLeft)
icon_closed = add_icon(arrow=Arrow.ArrowLeft)

def sort_books(pick_id: int, sort_direction: str):
    """Sort a column"""
    key = next(k for k, v in sort_dict.items() if v == pick_id)
    col_index = header.index(key)
    reverse = sort_direction == "Descending"
    book_lines.sort(key=lambda row: row[col_index].lower(), reverse=reverse)
    for (_i, _row) in enumerate(book_lines):
        for (_j, _cell) in enumerate(_row):
            update_widget(body_text_ids[_i * len(header) + _j], TextParam.Content, _cell)
    ids = sort_dict.values()
    for _id in ids:
        if _id != pick_id:
            update_widget(_id, PickListParam.Selected, None)
    if state["filtered_genre"] is None:
        return
    filter_genre(0, state["filtered_genre"])


def filter_genre(_pick_id: int, genre: str):
    """Filter by Genre — shows only rows matching the selected genre (or all if 'None')."""
    genre_col = header.index("Genre")
    filtered = book_lines if genre == "None" else [
        row for row in book_lines if row[genre_col] == genre
    ]
    if genre == "None":
        genre = None
    state["filtered_genre"] = genre
    num_cols = len(header)
    num_rows = len(body_text_ids) // num_cols
    for _i in range(num_rows):
        if _i < len(filtered):
            for _j, _cell in enumerate(filtered[_i]):
                update_widget(body_text_ids[_i * num_cols + _j], TextParam.Content, _cell)
        else:
            for _j in range(num_cols):
                update_widget(body_text_ids[_i * num_cols + _j], TextParam.Content, "")

text_center_align = add_text_format(align_center=True, fill=True, size=14)
text_left_align = add_text_format(align_center_left=True, fill=True, size=14)

# Add the window
with Window(
        title="Best Selling Books",
        size=(1000, 600),
        center=True):

    # Add the container for centering the table
    with Container(fill=True, align_center=True):

        # The table is added.
        with Table(
            col_widths=column_widths,
            sash_size=6,
            ):

            with TableHeader(row_height=50.0):
                for h1 in header:
                    if "Sales" in h1:
                        h = h1.split(" ")
                        add_text(content=f"{h[0]}\n{h[1]}", format_id=text_center_align)
                    else:
                        add_text(content=h1, format_id=text_center_align)

            with TableHeader():
                for h2 in header:
                    pl_id = add_pick_list(
                        options=pick_list_options,
                        placeholder="Sort",
                        text_size=12.0,
                        width_fill=True,
                        on_select=sort_books)

                    sort_dict[h2] = pl_id

            with TableHeader():
                for index in range(len(header)):
                    if index == len(header)-1:
                        add_pick_list(
                            options=genres,
                            placeholder="Filter",
                            text_size=12.0,
                            on_select=filter_genre)
                    else:
                        add_space(width=0)

            with TableBody(body_height=400.0):
                for row in book_lines:
                    for idx, cell in enumerate(row):
                        if idx > 1:
                            body_text_ids.append(
                                add_text(content=cell, format_id=text_center_align))
                        else:
                            body_text_ids.append(
                                add_text(content=cell, format_id=text_left_align))

            with TableFooter():
                for f in footers:
                    add_text(content=f, format_id=text_center_align)

# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
