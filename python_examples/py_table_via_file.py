#!/usr/bin/env python3
"""
Table demo
"""

import os
import sys
from pathlib import Path
from icedpygui import (
    Window,
    Container,
    start_session,
    add_table_basic)

column_widths = [100.0] * 7

def to_str(data: list[list[float]]) -> list[list[str]]:
    """Convert float data to str"""
    return [[str(cell) for cell in row] for row in data]

cwd = os.getcwd()
FILE_PATH = f"{cwd}/python_examples/resources/table_data/google.csv"
body = []

fp = Path(FILE_PATH)
if not fp.exists():
    print(f"File path does not eist {fp}")
    sys.exit()
else:
    # Load the CSV file
    with open(fp, 'r', encoding='utf-8') as file:
        lines = file.readlines()
        # Parse CSV data
        for line in lines:
            row = line.strip().split(',')
            body.append(row)

header = body.pop(0)

# Add the window
with Window(
        title="Table Demo",
        size=(800, 600),
        center=True):

    # Add the container for centering the table
    with Container(fill=True, align_center=True):
        add_table_basic(
            header = header,
            body=body,
            column_widths=column_widths,
            scrollable_height=300.0)


# Required to be the last widget sent to Iced,  If you start the program
# and nothing happens, it might mean you forgot to add this command.
start_session()
