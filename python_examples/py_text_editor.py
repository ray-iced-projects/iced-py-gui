#!/usr/bin/env python3
"""
Text Editor demo
"""

import os
from icedpygui import (Window, Container, start_session,
    add_text_editor)

cwd = os.getcwd()
FILE_PATH = f"{cwd}/python_examples/py_text_editor.py"

try:
    with open(FILE_PATH, "r", encoding='utf-8') as file:
        content = file.read()
except FileNotFoundError:
    print(f"The file does not exist using {FILE_PATH}.")

with Window(title="Text Editor", center=True):
    with Container(fill=True, align_center=True):
        add_text_editor(content=content, fill=True)

start_session()
